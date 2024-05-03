use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::MyConfig;
use task::{ReadyTask, Task};
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
                ReadyTask::create_task(&cfg, &description);
                println!("Add {}", description);
            }
            Command::Yari { id } => {
                let task = Task::get_by_id(&cfg, id).unwrap();
                task.complete(&cfg);
                println!("Done {:?}", task);
            }
        }
    } else {
        let tasks = ReadyTask::read_tasks(&cfg);
        tasks.iter().for_each(|t| println!("{:?}", t));
    }
}

