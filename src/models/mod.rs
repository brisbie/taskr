use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub project_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub priority: i32,
    pub status: String,
    pub due_date: Option<NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
}
