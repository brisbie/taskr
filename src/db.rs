use sqlx::mysql::MySqlPool; // MySQL connection pool from sqlx
use std::env;               // Used to access environment variables

// Establishes and returns a connection pool to the MySQL database
pub async fn connect_db() -> MySqlPool {
    // Load environment variables from a .env file (if it exists)
    dotenvy::dotenv().ok();

    // Retrieve the database URL from environment variables
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create and return a connection pool using the database URL
    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}
