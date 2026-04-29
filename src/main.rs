use anyhow::Result;
use colored::*;
use sqlx::MySqlPool;
use std::env;
use dotenvy::dotenv;

mod utils;
use utils::cli::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = MySqlPool::connect(&database_url).await?;

    // --- DELETE & COMPLETE ---
    if let Some(task_id) = args.delete {
        sqlx::query!("DELETE FROM tasks WHERE id = ?", task_id).execute(&pool).await?;
        println!("{} Task {} removed.", "✔".green(), task_id);
    }
    if let Some(task_id) = args.complete {
        sqlx::query!("UPDATE tasks SET status = 'completed' WHERE id = ?", task_id).execute(&pool).await?;
        println!("{} Task {} completed.", "✔".green(), task_id);
    }

    // --- ADD TASK ---
    if let Some(title) = args.add.as_ref() {
        let mut target_project_id = 1; 
        if let Some(proj_input) = args.project.as_ref() {
            let res = sqlx::query!("SELECT id FROM projects WHERE name = ?", proj_input).fetch_optional(&pool).await?;
            if let Some(r) = res { target_project_id = r.id; }
            else {
                sqlx::query!("INSERT INTO projects (name) VALUES (?)", proj_input).execute(&pool).await?;
                target_project_id = sqlx::query!("SELECT id FROM projects WHERE name = ?", proj_input).fetch_one(&pool).await?.id;
            }
        }
        let task_res = sqlx::query!("INSERT INTO tasks (title, project_id, priority, status) VALUES (?, ?, 3, 'pending')", title, target_project_id).execute(&pool).await?;
        let new_id = task_res.last_insert_id() as i32;
        if let Some(c) = args.comment.as_ref() {
            sqlx::query!("INSERT INTO comments (task_id, content) VALUES (?, ?)", new_id, c).execute(&pool).await?;
        }
        println!("{} Task created (ID: {}).", "✔".green(), new_id);
    }

    // --- SUBTASKS, COMMENTS, TAGS ---
    if let (Some(task_id), Some(sub)) = (args.id, args.subtask.as_ref()) {
        let status = args.substatus.as_deref().unwrap_or("pending");
        sqlx::query!("INSERT INTO subtasks (task_id, title, status) VALUES (?, ?, ?)", task_id, sub, status).execute(&pool).await?;
        println!("{} Subtask added.", "✔".green());
    }
    if let (Some(task_id), Some(c)) = (args.id, args.comment.as_ref()) {
        if args.add.is_none() {
            sqlx::query!("INSERT INTO comments (task_id, content) VALUES (?, ?)", task_id, c).execute(&pool).await?;
            println!("{} Comment added.", "✔".green());
        }
    }
    if let (Some(task_id), Some(t)) = (args.id, args.tag.as_ref()) {
        sqlx::query!("INSERT IGNORE INTO tags (name) VALUES (?)", t).execute(&pool).await?;
        let tag_id = sqlx::query!("SELECT id FROM tags WHERE name = ?", t).fetch_one(&pool).await?.id;
        sqlx::query!("INSERT INTO task_tags (task_id, tag_id) VALUES (?, ?)", task_id, tag_id).execute(&pool).await?;
        println!("{} Tag linked.", "✔".green());
    }

    // --- LIST VIEW ---
    if args.list {
        let rows = sqlx::query!(
            r#"
            SELECT t.id, t.title, t.status, p.name as project_name, 
                   GROUP_CONCAT(DISTINCT tg.name) as tags,
                   GROUP_CONCAT(DISTINCT CONCAT(s.title, ':', s.status) SEPARATOR '||') as subtasks,
                   (SELECT content FROM comments WHERE task_id = t.id ORDER BY created_at DESC LIMIT 1) as last_comment
            FROM tasks t
            LEFT JOIN projects p ON t.project_id = p.id
            LEFT JOIN task_tags tt ON t.id = tt.task_id
            LEFT JOIN tags tg ON tt.tag_id = tg.id
            LEFT JOIN subtasks s ON t.id = s.task_id
            GROUP BY t.id
            "#
        ).fetch_all(&pool).await?;

        println!("\n{:<4} {:<30} {:<15} {:<12} {:<20}", "ID".dimmed(), "Title".bold(), "Project".bold(), "Status".bold(), "Tags".bold());
        println!("{}", "-".repeat(85).dimmed());

        for row in rows {
            let status_color = if row.status == "completed" { row.status.green() } else { row.status.yellow() };
            println!("{:<4} {:<30} {:<15} {:<12} {:<20}", row.id.to_string().cyan(), row.title.bold(), row.project_name.unwrap_or_else(|| "N/A".to_string()).magenta(), status_color, row.tags.unwrap_or_default().blue());

            // 1. Note displays first with branch symbol
            if let Some(comment) = row.last_comment {
                println!("     {} {}: {}", "└─".blue(), "Note".blue().bold(), comment.blue().italic());
            }

            // 2. Subtasks nested under Note with arrows - Unified Yellow
            if let Some(sub_str) = row.subtasks {
                for item in sub_str.split("||") {
                    let parts: Vec<&str> = item.split(':').collect();
                    if parts.len() == 2 {
                        let sub_title = parts[0];
                        // Force subtask title to yellow regardless of status
                        println!("         {} {}", "➜".dimmed(), sub_title.yellow());
                    }
                }
            }
        }
    }
    Ok(())
}
