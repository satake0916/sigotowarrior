use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::MyConfig;
use task::{ReadyTask, Task};
mod config;
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
    let home = std::env::var("HOME").unwrap();
    let mut home = PathBuf::from(&home);
    home.push(".sigorc");
    let cfg = confy::load_path::<MyConfig>(&home).unwrap();

    let cli = AppArg::parse();
    if let Some(command) = cli.command {
        match command {
            Command::Fue { description } => {
                Command::add_task(&cfg, &description);
                println!("Add {}", description)
            }
        }
    } else {
        let tasks = Command::read_all_ready_tasks(&cfg);
        // Print
        tasks.iter().for_each(|t| println!("{:?}", t));
    }
}

impl Command {
    // REVIEW: It is a little strange that add_task is a method of Command.
    fn add_task(cfg: &MyConfig, description: &String) {
        let task = Task::new(description);
        repository::insert_task(cfg, task);
    }

    fn read_all_ready_tasks(cfg: &MyConfig) -> Vec<ReadyTask> {
        repository::read_ready_tasks(cfg).unwrap()
    }
}
