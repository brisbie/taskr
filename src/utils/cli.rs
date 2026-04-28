use clap::Parser;

#[derive(Parser)]
#[command(name = "taskr")]
#[command(about = "A CLI Task Manager", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub add: Option<String>,

    #[arg(short, long)]
    pub priority: Option<i32>,

    #[arg(short, long)]
    pub project: Option<i32>,

    #[arg(short, long)]
    pub note: Option<String>,

    #[arg(short, long)]
    pub due: Option<String>,

    #[arg(short, long)]
    pub list: bool,

    #[arg(short, long)]
    pub done: Option<i32>,

    #[arg(short, long)]
    pub delete: Option<i32>,

    #[arg(short = 'n', long)]
    pub project_name: Option<String>,

    #[arg(short, long)]
    pub report: bool,

    #[arg(long)]
    pub archive: Option<i32>, // New flag for transaction demo
}
