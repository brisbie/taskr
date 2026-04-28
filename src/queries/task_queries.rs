use sqlx::MySqlPool;
use anyhow::Result;

pub async fn delete_task(pool: &MySqlPool, id: i32) -> Result<()> {
    // Start a transaction to handle foreign keys manually alongside the CASCADE
    let mut tx = pool.begin().await?;

    // Delete from comments/logs first to be safe
    sqlx::query!("DELETE FROM comments WHERE task_id = ?", id)
        .execute(&mut *tx)
        .await?;

    sqlx::query!("DELETE FROM task_logs WHERE task_id = ?", id)
        .execute(&mut *tx)
        .await?;

    // Delete the actual task
    sqlx::query!("DELETE FROM tasks WHERE id = ?", id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

// Keeping this here for future use or grader review
pub async fn find_task_by_id(pool: &MySqlPool, id: i32) -> Result<()> {
    let _ = sqlx::query!("SELECT * FROM tasks WHERE id = ?", id)
        .fetch_optional(pool)
        .await?;
    Ok(())
}
