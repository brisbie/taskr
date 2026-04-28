use sqlx::{MySql, Pool};
use crate::models::Task;
use anyhow::Result;

pub async fn insert_task(
    pool: &Pool<MySql>, 
    title: &str, 
    priority: i32, 
    project_id: i32
) -> Result<u64> {
    let res = sqlx::query!(
        "INSERT INTO tasks (title, priority, project_id) VALUES (?, ?, ?)",
        title, priority, project_id
    )
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn find_all_tasks(pool: &Pool<MySql>) -> Result<Vec<Task>> {
    let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(pool)
        .await?;
    Ok(tasks)
}
