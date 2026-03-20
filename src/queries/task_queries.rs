use sqlx::MySqlPool; // MySQL connection pool type from sqlx

// Creates a new task in the database
pub async fn create_task(pool: &MySqlPool, title: &str) {
    // Execute an INSERT query to add a new task with the given title
    // The priority is currently hardcoded to 1
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
    .expect("Failed to insert task"); // Panic if the database operation fails
}
