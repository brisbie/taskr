use sqlx::MySqlPool;
use colored::*;

pub async fn display_task_list(pool: &MySqlPool, project_filter: Option<String>) -> anyhow::Result<()> {
    // JOIN Query Implementation 
    let rows = sqlx::query!(
        r#"
        SELECT t.id, t.title, t.priority, t.status, t.due_date, t.description, p.name as project_name
        FROM tasks t
        LEFT JOIN projects p ON t.project_id = p.id
        WHERE (? IS NULL OR p.name = ?)
        ORDER BY t.id ASC
        "#,
        project_filter,
        project_filter
    )
    .fetch_all(pool)
    .await?;

    println!("\n{:<4} {:<30} {:<10} {:<10} {:<12}", "ID", "Name", "Priority", "Status", "Due Date");
    println!("{}", "-".repeat(75));

    for row in rows {
        let date_str = row.due_date.map(|d| d.to_string()).unwrap_or_else(|| "N/A".to_string());
        let status_display = if row.status == "Done" { row.status.green() } else { row.status.yellow() };

        println!(
            "{:<4} {:<30} {:<10} {:<10} {:<12}",
            row.id, row.title.bold(), row.priority, status_display, date_str
        );

        if let Some(note) = row.description {
            if !note.trim().is_empty() {
                println!("     {} {}", "└─ Note:".blue(), note.italic().white());
            }
        }
    }
    println!();
    Ok(())
}

pub async fn get_overdue_report(pool: &MySqlPool) -> anyhow::Result<()> {
    println!("{}", "--- Overdue Tasks Report ---".red().bold());
    let rows = sqlx::query!(
        "SELECT id, title, due_date FROM tasks WHERE due_date < CURDATE() AND status != 'Done'"
    )
    .fetch_all(pool)
    .await?;

    if rows.is_empty() {
        println!("No overdue tasks found.");
    } else {
        for row in rows {
            println!("[ID: {}] {} (Due: {:?})", row.id, row.title.red(), row.due_date);
        }
    }
    Ok(())
}
