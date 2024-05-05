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
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { description: String },
    Done { id: u32 },
    Wait { id: u32 },
    Back { id: u32 },
    Annotate { id: u32, annotation: String },
    List,
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
    match cli.command {
        Command::Add { description } => {
            let new_task = match ReadyTask::new(&cfg, &description) {
                Ok(task) => task,
                Err(err) => {
                    eprintln!("{}", err);
                    return;
                }
            };
            match ReadyTask::add_task(&cfg, new_task) {
                Ok(task) => println!("Created sigo {}", task.id),
                Err(err) => eprintln!("{}", err),
            }
        }
        Command::Done { id } => {
            let task = match Task::get_by_id(&cfg, id) {
                Ok(task) => task,
                Err(err) => {
                    eprintln!("{}", err);
                    return;
                }
            };
            match task.complete(&cfg) {
                Ok(task) => println!("Completed sigo '{}'", task.description),
                Err(err) => eprintln!("{}", err),
            }
        }
        Command::Wait { id } => {
            let task = match Task::get_by_id(&cfg, id) {
                Ok(task) => task,
                Err(err) => {
                    eprintln!("{}", err);
                    return;
                }
            };
            match task {
                Task::Ready(task) => match task.wait(&cfg) {
                    Ok(task) => {
                        println!("Waiting sigo {} '{}'", task.id, task.get_main_description())
                    }
                    Err(err) => eprintln!("{}", err),
                },
                Task::Waiting(task) => {
                    println!("Already waiting task {}", task.id)
                }
                Task::Completed(_task) => {
                    panic!("get_by_id function doesnot return completed task {}", id);
                }
            }
        }
        Command::Back { id } => {
            let task = match Task::get_by_id(&cfg, id) {
                Ok(task) => task,
                Err(err) => {
                    eprintln!("{}", err);
                    return;
                }
            };
            match task {
                Task::Ready(task) => {
                    println!("Already ready sigo {}", task.id);
                }
                Task::Waiting(task) => match task.back(&cfg) {
                    Ok(task) => println!(
                        "Returning sigo {} '{}'",
                        task.id,
                        task.get_main_description()
                    ),
                    Err(err) => eprintln!("{}", err),
                },
                Task::Completed(_task) => {
                    panic!("get_by_id function doesnot return completed task {}", id);
                }
            }
        }
        Command::Annotate { id, annotation } => {
            let task = match Task::get_by_id(&cfg, id) {
                Ok(task) => task,
                Err(err) => {
                    eprintln!("{}", err);
                    return;
                }
            };
            match task.annotate(&cfg, &annotation) {
                Ok(()) => println!("Annotated sigo {} with '{}'", id, annotation),
                Err(err) => eprintln!("{}", err),
            }
        }
        Command::List => match ReadyTask::read_tasks(&cfg) {
            Ok(tasks) => println!("{}", tasks_to_string(tasks)),
            Err(err) => eprintln!("{}", err),
        },
        Command::Waiting => {
            let tasks = match WaitingTask::read_tasks(&cfg) {
                Ok(tasks) => tasks,
                Err(err) => {
                    eprintln!("{}", err);
                    return;
                }
            };
            println!("{}", tasks_to_string(tasks));
        }
    }
}
