// Import internal modules for CLI parsing and database connection
mod cli;
mod db;

// Import external crates
use clap::Parser;              // For parsing command-line arguments
use sqlx::MySqlPool;           // MySQL connection pool for database access

#[tokio::main]
async fn main() {
    // Entry point of the application
    println!("Taskr CLI starting...");

    // Establish a connection pool to the database
    let pool = db::connect_db().await;

    // Parse command-line arguments into the CLI struct
    let cli = cli::Cli::parse();

    // Determine which command was provided and execute accordingly
    if let Some(title) = cli.add {
        // If the user used --add, insert a new task
        add_task(&pool, &title).await;
    } else if cli.list {
        // If the user used --list, display all tasks
        list_tasks(&pool).await;
    } else {
        // If no valid command was provided, show help message
        println!("No command provided. Try --help");
    }
}

// Adds a new task to the database
async fn add_task(pool: &MySqlPool, title: &str) {
    // Execute an INSERT query to add the task with a default priority
    sqlx::query!(
        r#"
        INSERT INTO tasks (title, priority)
        VALUES (?, ?)
        "#,
        title,
        1
    )
    .execute(pool)
    .await
    .expect("Failed to insert task"); // Panic if query fails

    // Confirm task was added
    println!("Task added: {}", title);
}

// Retrieves and prints all tasks from the database
async fn list_tasks(pool: &MySqlPool) {
    // Query all tasks, ordered by most recent first
    let tasks = sqlx::query!(
        r#"
        SELECT id, title, priority, status
        FROM tasks
        ORDER BY id DESC
        "#
    )
    .fetch_all(pool)
    .await
    .expect("Failed to fetch tasks"); // Panic if query fails

    // Print each task in a readable format
    for task in tasks {
        println!(
            "[{}] {} (priority: {}, status: {})",
            task.id, task.title, task.priority, task.status
        );
    }
}
