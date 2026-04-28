use sqlx::MySqlPool;
use anyhow::Result;

pub async fn create_task_atomic(
    pool: &MySqlPool,
    project_id: i32,
    title: &str,
    priority: i32,
    note: &str,
) -> Result<()> {
    // Prepared Statement used via query! macro
    sqlx::query!(
        "INSERT INTO tasks (project_id, title, priority, description, status) 
         VALUES (?, ?, ?, ?, 'pending')",
        project_id,
        title,
        priority,
        note
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn mark_task_complete(pool: &MySqlPool, id: i32) -> Result<()> {
    sqlx::query!(
        "UPDATE tasks SET status = 'Done' WHERE id = ?",
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn archive_project_atomic(pool: &MySqlPool, project_id: i32) -> Result<()> {
    // START TRANSACTION [cite: 58, 65]
    let mut tx = pool.begin().await?;

    // Step 1: Update all tasks in project to 'Done'
    // This prevents a data anomaly where an archived project has active tasks 
    sqlx::query!(
        "UPDATE tasks SET status = 'Done' WHERE project_id = ?",
        project_id
    )
    .execute(&mut *tx)
    .await?;

    // Step 2: Update the project status itself
    // Now that the 'status' column exists, this will compile successfully
    sqlx::query!(
        "UPDATE projects SET status = 'Archived' WHERE id = ?",
        project_id
    )
    .execute(&mut *tx)
    .await?;

    // COMMIT the transaction
    tx.commit().await?;
    
    Ok(())
}
