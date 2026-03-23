// Import the Parser trait from clap to enable command-line argument parsing
use clap::Parser;

// Derive the Parser trait to automatically generate CLI parsing logic
#[derive(Parser)]
// Set the name of the command-line application
#[command(name = "taskr")]
pub struct Cli {
    #[arg(long)]
    pub add: Option<String>, //task name 

    #[arg(long)]
    pub priority: Option<i32>, //priority 1 - 5

    #[arg(long)]
    pub due: Option<String>, //due date
    
    #[arg(long)]
    pub list: bool,         //list tasks

    #[arg(long)]
    pub done: Option<i32>,  //mark task as done using its ID
}