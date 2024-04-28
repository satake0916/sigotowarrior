use clap::{Parser, Subcommand};

#[derive(Parser)]
struct AppArg {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Fue { task: String },
}

fn main() {
    let cli = AppArg::parse();
    if let Some(command) = cli.command {
        match command {
            Command::Fue { task } => {
                // Add Task

                // Print
                println!("Add {}", task)
            }
        }
    } else {
        // When no subcommand, list tasks
        // List Tasks

        // Print
        println!("Task1, Task2, Task3...")
    }
}
