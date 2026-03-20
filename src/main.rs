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
    sqlx::query!(
        r#"
        INSERT INTO tasks (title, priority, due_date)
        VALUES (?, ?, ?)
        "#,
        title,
        priority,
        due_date
    )
    .execute(pool)
    .await
    .expect("Failed to insert task");

    println!("Task added: {} (priority: {})", title, priority);
}

// Struct to represent a row in the table
async fn list_tasks(pool: &MySqlPool) {
    let tasks = sqlx::query!(
        r#"
        SELECT id, title, priority, status, due_date
        FROM tasks
        ORDER BY id ASC
        "#
    )
    .fetch_all(pool)
    .await
    .expect("Failed to fetch tasks");

    // Column widths
    let id_w = 4;
    let name_w = 20;
    let prio_w = 10;
    let status_w = 10;
    let due_w = 12;

    // Header
    println!(
        "{:<id_w$} {:<name_w$} {:<prio_w$} {:<status_w$} {:<due_w$}",
        "ID",
        "Name",
        "Priority",
        "Status",
        "Due",
        id_w = id_w,
        name_w = name_w,
        prio_w = prio_w,
        status_w = status_w,
        due_w = due_w,
    );

    println!("{}", "-".repeat(id_w + name_w + prio_w + status_w + due_w + 4));
    //Loop to fetch tasks
    for task in tasks {
        let due = match task.due_date {
            Some(date) => date.to_string(),
            None => "N/A".to_string(),
        };

        let priority = match task.priority {
            1 => "1".bright_green(),
            2 => "2".green(),
            3 => "3".yellow(),
            4 => "4".truecolor(255, 165, 0),
            5 => "5".red(),
            _ => task.priority.to_string().normal(),
        };

        let name = task.title.bold();

        println!(
            "{:<id_w$} {:<name_w$} {:<prio_w$} {:<status_w$} {:<due_w$}",
            task.id,
            name,
            priority,
            task.status,
            due,
            id_w = id_w,
            name_w = name_w,
            prio_w = prio_w,
            status_w = status_w,
            due_w = due_w,
        );
    }
}
