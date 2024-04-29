use clap::{Parser, Subcommand};
use task::Task;
mod task;
mod repository;

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
                Command::add_task(&description);

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
    // REVIEW: It is a little strange that add_task is a method of Command.
    fn add_task(task: &String) {
        let home = std::env::var("HOME").unwrap();
        // Create Task
        let task: Task = task::Task::Ready(Task::new(task));
        // Insert DB
        let path = home + "/sigo.txt";
        println!("{}", &path);
        repository::insert_task(task, path.into());
    }
}
