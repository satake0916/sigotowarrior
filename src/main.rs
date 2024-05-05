use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use config::MyConfig;
use task::{ReadyTask, Task, WaitingTask};

use crate::utils::tasks_to_string;
mod config;
mod task;
mod utils;

#[derive(Parser)]
struct AppArg {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Fue { description: String },
    Add { description: String },

    Yari { id: u32 },
    Done { id: u32 },

    Machi { id: u32 },
    Wait { id: u32 },

    Modo { id: u32 },
    Return { id: u32 },

    Taiki,
    Waiting,
}

fn main() {
    // load .sigorc
    let home = std::env::var("HOME").unwrap();
    let mut home = PathBuf::from(&home);
    home.push(".sigorc");
    let cfg = confy::load_path::<MyConfig>(&home).unwrap();

    // if task dir doesnot exist, create dir
    let sigo_path = PathBuf::from(&cfg.home);
    if !sigo_path.is_dir() {
        let _ = fs::create_dir(sigo_path);
    }

    // Parse args
    let cli = AppArg::parse();
    if let Some(command) = cli.command {
        match command {
            Command::Fue { description } | Command::Add { description } => {
                let new_task = ReadyTask::new(&cfg, &description);
                let id = new_task.id;
                ReadyTask::add_task(&cfg, new_task);
                println!("Created sigo {}", id);
            }
            Command::Yari { id } | Command::Done { id } => {
                let task = Task::get_by_id(&cfg, id).unwrap();
                task.complete(&cfg);
                println!("Completed sigo {} '{}'", task.id(), task.description());
            }
            Command::Machi { id } | Command::Wait { id } => {
                let task = Task::get_by_id(&cfg, id).unwrap();
                match task {
                    Task::Ready(task) => {
                        task.wait(&cfg);
                        println!("Waiting sigo {} '{}'", task.id, task.description)
                    }
                    _ => {
                        // Exception
                    }
                }
            }
            Command::Modo { id } | Command::Return { id } => {
                let task = Task::get_by_id(&cfg, id).unwrap();
                match task {
                    Task::Waiting(task) => {
                        task.back(&cfg);
                        println!("Returning sigo {} '{}'", task.id, task.description)
                    }
                    _ => {
                        // Exception
                    }
                }
            }
            Command::Taiki | Command::Waiting => {
                let tasks = WaitingTask::read_tasks(&cfg).unwrap();
                println!("{}", tasks_to_string(tasks));
            }
        }
    } else {
        let tasks = ReadyTask::read_tasks(&cfg).unwrap();
        println!("{}", tasks_to_string(tasks));
    }
}
