use clap::{Parser, Subcommand};
use task::{ReadyTask, Task};
mod repository;
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
                Command::add_task(&description);

                // Print
                println!("Add {}", description)
            }
        }
    } else {
        // List Tasks
        let tasks = Command::list_ready_task();
        // Print
        tasks.iter().for_each(|t| println!("{:?}", t));
    }
}

impl Command {
    // REVIEW: It is a little strange that add_task is a method of Command.
    fn add_task(task: &String) {
        let home = std::env::var("HOME").unwrap();
        // Create Task
        let task = Task::new(task);
        // Insert DB
        let path = home + "/sigo.txt";
        println!("{}", &path);
        repository::insert_task(task, path.into());
    }

    fn list_ready_task() -> Vec<ReadyTask> {
        let home = std::env::var("HOME").unwrap();
        let path = home + "/sigo.txt";
        repository::read_ready_tasks(&path.into()).unwrap()
    }
}
