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
    /// Add sigo
    Add { description: String },

    /// Done sigo
    Done { id: u32 },

    /// Change sigo waiting
    Wait { id: u32 },

    /// Change sigo ready
    Back { id: u32 },

    /// Annotate existing sigo
    Annotate {
        id: u32,

        /// Annotation text
        #[arg(short, long)]
        text: String,
    },

    /// List ready sigos
    List,

    /// List waiting sigos
    Waiting,
}

fn main() {
    // load config.ini
    let xdg_dirs = xdg::BaseDirectories::with_prefix("sigotorrior").expect("XDG is not used");
    let config_path = xdg_dirs.get_config_file("config.ini");
    let cfg = confy::load_path::<MyConfig>(&config_path).expect("cannot load config.ini");

    // if task dir doesnot exist, create dir
    let sigo_path = PathBuf::from(&cfg.data);
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
        Command::Annotate { id, text } => {
            let task = match Task::get_by_id(&cfg, id) {
                Ok(task) => task,
                Err(err) => {
                    eprintln!("{}", err);
                    return;
                }
            };
            match task.annotate(&cfg, &text) {
                Ok(()) => println!("Annotated sigo {} with '{}'", id, text),
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
