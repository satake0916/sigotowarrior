use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::MyConfig;
use task::{ReadyTask, Task, WaitingTask};
mod config;
mod task;

#[derive(Parser)]
struct AppArg {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Fue { description: String },
    Yari { id: u32 },
    Machi { id: u32 },
    Modo { id: u32 },
    Taiki,
}

fn main() {
    // load settings
    let home = std::env::var("HOME").unwrap();
    let mut home = PathBuf::from(&home);
    home.push(".sigorc");
    let cfg = confy::load_path::<MyConfig>(&home).unwrap();

    // Parse args
    let cli = AppArg::parse();
    if let Some(command) = cli.command {
        match command {
            Command::Fue { description } => {
                let new_task = ReadyTask::new(&cfg, &description);
                ReadyTask::add_task(&cfg, new_task);
                println!("Add {}", description);
            }
            Command::Yari { id } => {
                let task = Task::get_by_id(&cfg, id).unwrap();
                task.complete(&cfg);
                println!("Done {:?}", task);
            }
            Command::Machi { id } => {
                let task = Task::get_by_id(&cfg, id).unwrap();
                match task {
                    Task::Ready(task) => {
                        task.wait(&cfg);
                        println!("Wait {:?}", task)
                    }
                    _ => {
                        // Exception
                    }
                }
            }
            Command::Modo { id } => {
                let task = Task::get_by_id(&cfg, id).unwrap();
                match task {
                    Task::Waiting(task) => {
                        task.get_ball(&cfg);
                        println!("Myball {:?}", task)
                    }
                    _ => {
                        // Exception
                    }
                }
            }
            Command::Taiki => {
                let tasks = WaitingTask::read_tasks(&cfg);
                tasks.iter().for_each(|t| println!("{:?}", t));
            }
        }
    } else {
        let tasks = ReadyTask::read_tasks(&cfg);
        tasks.iter().for_each(|t| println!("{:?}", t));
    }
}
