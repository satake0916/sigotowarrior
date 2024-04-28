use clap::{Parser, Subcommand};
use task::Task;
mod task;

#[derive(Parser)]
struct AppArg {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Fue { description: String },
}

fn main() {
    let cli = AppArg::parse();
    if let Some(command) = cli.command {
        match command {
            Command::Fue { description } => {
                // Add Task

                // Print
                println!("Add {}", description)
            }
        }
    } else {
        // When no subcommand, list tasks
        // List Tasks

        // Print
        println!("Task1, Task2, Task3...")
    }
}

impl Command {
    fn add_task (task: &String) {
        // Create Task
        let task = Task::new(task);
        // Insert DB

    }
}