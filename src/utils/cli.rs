use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub list: bool,

    #[arg(short, long)]
    pub add: Option<String>,

    #[arg(short, long)]
    pub project: Option<String>,

    #[arg(short, long)]
    pub id: Option<i32>,

    #[arg(short, long)]
    pub subtask: Option<String>,

    /// Status for subtask: 'pending' (yellow) or 'completed' (green)
    #[arg(long)]
    pub substatus: Option<String>,

    #[arg(short, long)]
    pub comment: Option<String>,

    #[arg(short, long)]
    pub tag: Option<String>,

    #[arg(short = 'A', long)]
    pub archive_project: Option<i32>,

    #[arg(short, long)]
    pub delete: Option<i32>,

    #[arg(short = 'C', long)]
    pub complete: Option<i32>,
}
