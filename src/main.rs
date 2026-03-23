mod cli;
mod db;

use clap::Parser;
use sqlx::MySqlPool;
use colored::*;

#[tokio::main]
async fn main() {

    // Connect to the database
    let pool = db::connect_db().await;

    // Parse CLI arguments
    let cli = cli::Cli::parse();

    // Handle commands
    if let Some(title) = cli.add {
        // Get optional values, with defaults if not provided
        let priority = cli.priority.unwrap_or(1).clamp(1, 5);
        let due_date = cli.due;

        add_task(&pool, &title, priority, due_date).await;
    } else if cli.list {
        list_tasks(&pool).await;
    } else {
        println!("No command provided. Try --help");
    }
}

// Add a task with priority and optional due date
async fn add_task(
    pool: &MySqlPool,
    title: &str,
    priority: i32,
    due_date: Option<String>,
) {
    // Start transaction
    let mut tx = pool.begin().await.expect("Failed to start transaction");

    // STEP 1: Insert task
    let task_result = sqlx::query!(
        r#"
        INSERT INTO tasks (title, priority, due_date)
        VALUES (?, ?, ?)
        "#,
        title,
        priority,
        due_date
    )
    .execute(&mut *tx)
    .await;

    let task_id = match task_result {
        Ok(res) => res.last_insert_id(),
        Err(e) => {
            tx.rollback().await.expect("Rollback failed");
            eprintln!("Task insert failed: {}", e);
            return;
        }
    };

    // STEP 2: Insert log entry
    let log_result = sqlx::query!(
        r#"
        INSERT INTO task_logs (task_id, message)
        VALUES (?, ?)
        "#,
        task_id,
        "Task created"
    )
    .execute(&mut *tx)
    .await;

    match log_result {
        Ok(_) => {
            // SUCCESS → commit everything
            tx.commit().await.expect("Commit failed");
            println!("Task added successfully: {}", title);
        }
        Err(e) => {
            // FAILURE → rollback everything (including task insert)
            tx.rollback().await.expect("Rollback failed");
            eprintln!("Log insert failed, transaction rolled back: {}", e);
        }
    }
}

pub async fn list_tasks(pool: &sqlx::MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = sqlx::query!("SELECT id, title, priority, status, due_date FROM tasks")
        .fetch_all(pool)
        .await?;

    if tasks.is_empty() {
        println!("No tasks found. Use 'taskr --add' to create one.");
        return Ok(());
    }

    println!(
        "\n{:<4} {:<30} {:<10} {:<10} {:<12}",
        "ID", "Name", "Priority", "Status", "Due"
    );
    println!("{}", "-".repeat(70));

    for task in tasks {
        let id = task.id;
        let title = task.title;
        let priority_val = task.priority;
        let status = task.status;
        
        use colored::*; 

        let priority_display = match priority_val {
            5 => priority_val.to_string().red().bold(),
            4 => priority_val.to_string().bright_red(),
            3 => priority_val.to_string().yellow(),
            2 => priority_val.to_string().green(),
            1 => priority_val.to_string().bright_green(),
            _ => priority_val.to_string().white(),
        };

        let due = task.due_date
            .map(|d| d.to_string())
            .unwrap_or_else(|| "None".to_string());

        println!(
            "{:<4} {:<30} {:<20} {:<10} {:<12}",
            id, 
            title, 
            format!("{:<10}", priority_display), 
            status, 
            due
        );
    }
    
    println!();
    Ok(())
}
