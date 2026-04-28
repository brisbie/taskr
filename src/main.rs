mod models;
mod queries;
mod services;
mod utils;

use crate::utils::cli::Cli;
use crate::utils::db;
use anyhow::Result;
use clap::Parser;
use colored::*;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = db::connect_db().await;
    let cli = Cli::parse();

    if let Some(title) = &cli.add {
        let priority = cli.priority.unwrap_or(3);
        let project_id = cli.project.unwrap_or(1);
        let note = cli.note.clone().unwrap_or_default();

        services::task_service::create_task_atomic(&pool, project_id, title, priority, &note).await?;
        println!("{} Task added successfully.", "✔".green());

    } else if let Some(id) = cli.done {
        services::task_service::mark_task_complete(&pool, id).await?;
        println!("{} Task marked as done.", "✔".green());

    } else if let Some(id) = cli.delete {
        queries::task_queries::delete_task(&pool, id).await?;
        println!("{} Task removed.", "✔".green());

    } else if let Some(project_id) = cli.archive {
        // Demonstration of Transaction Logic 
        match services::task_service::archive_project_atomic(&pool, project_id).await {
            Ok(_) => println!("{} Project and tasks archived successfully.", "✔".green()),
            Err(e) => eprintln!("{} Transaction Failed (Rolled Back): {}", "✘".red(), e),
        }

    } else if cli.list {
        services::report_service::display_task_list(&pool, cli.project_name.clone()).await?;

    } else if cli.report {
        services::report_service::get_overdue_report(&pool).await?;
    }

    Ok(())
}
