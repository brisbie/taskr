// Import the Parser trait from clap to enable command-line argument parsing
use clap::Parser;

// Derive the Parser trait to automatically generate CLI parsing logic
#[derive(Parser)]
// Set the name of the command-line application
#[command(name = "taskr")]
pub struct Cli {
    // Optional argument: --add <String>
    // If provided, this will contain the title of the task to add
    #[arg(long)]
    pub add: Option<String>,

    // Flag argument: --list
    // If present, this will be true and triggers listing all tasks
    #[arg(long)]
    pub list: bool,
}
