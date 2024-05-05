use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use config::MyConfig;
use task::{ReadyTask, Task, WaitingTask};

use crate::utils::tasks_to_string;
mod config;
mod error;
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
    Back { id: u32 },

    Taiki,
    Waiting,
}

fn main() {
    // load .sigorc
    let home = std::env::var("HOME").expect("HOME is not set");
    let mut home = PathBuf::from(&home);
    home.push(".sigorc");
    let cfg = confy::load_path::<MyConfig>(&home).expect("cannot load .sigorc");

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
                let new_task = match ReadyTask::new(&cfg, &description) {
                    Ok(task) => task,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };
                let id = new_task.id;
                match ReadyTask::add_task(&cfg, new_task) {
                    Ok(()) => println!("Created sigo {}", id),
                    Err(err) => println!("{}", err),
                }
            }
            Command::Yari { id } | Command::Done { id } => {
                let task = match Task::get_by_id(&cfg, id) {
                    Ok(task) => task,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };
                match task.complete(&cfg) {
                    Ok(task) => println!("Completed sigo {} '{}'", task.id, task.description),
                    Err(err) => println!("{}", err),
                }
            }
            Command::Machi { id } | Command::Wait { id } => {
                let task = match Task::get_by_id(&cfg, id) {
                    Ok(task) => task,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };
                match task {
                    Task::Ready(task) => match task.wait(&cfg) {
                        Ok(()) => println!("Waiting sigo {} '{}'", task.id, task.description),
                        Err(err) => println!("{}", err),
                    },
                    Task::Waiting(task) => {
                        println!("Already waiting task {}", task.id)
                    }
                    Task::Completed(task) => {
                        println!("Already completed task {}", task.id)
                    }
                }
            }
            Command::Modo { id } | Command::Back { id } => {
                let task = match Task::get_by_id(&cfg, id) {
                    Ok(task) => task,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };
                match task {
                    Task::Ready(task) => {
                        println!("Already ready task {}", task.id);
                    }
                    Task::Waiting(task) => match task.back(&cfg) {
                        Ok(()) => println!("Returning sigo {} '{}'", task.id, task.description),
                        Err(err) => println!("{}", err),
                    },
                    Task::Completed(task) => {
                        println!("Already completed task {}", task.id);
                    }
                }
            }
            Command::Taiki | Command::Waiting => {
                let tasks = match WaitingTask::read_tasks(&cfg) {
                    Ok(tasks) => tasks,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };
                println!("{}", tasks_to_string(tasks));
            }
        }
    } else {
        match ReadyTask::read_tasks(&cfg) {
            Ok(tasks) => println!("{}", tasks_to_string(tasks)),
            Err(err) => println!("{}", err),
        }
    }
}
