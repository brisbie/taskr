use sqlx::{MySql, Pool, MySqlPool};
use std::env;
use dotenvy::dotenv;

pub async fn connect_db() -> Pool<MySql> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}
