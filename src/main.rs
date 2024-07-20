use std::{fs, path::PathBuf};

use chrono::NaiveDate;
use clap::{Parser, Subcommand, ValueEnum};
use config::MyConfig;
use serde::{Deserialize, Serialize};
use strum::Display;

mod active_params;
mod command;
mod config;
mod date;
mod display;
mod error;
mod file;
mod task;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct AppArg {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add sigo
    Add {
        description: String,

        /// Priority(H/M/L)
        #[arg(value_enum, short, long)]
        priority: Option<Priority>,

        /// Waiting
        #[arg(short, long)]
        waiting: bool,

        /// Due date
        #[arg(short, long, value_parser = date::validate_date_str)]
        due: Option<NaiveDate>,
    },

    /// Modify sigo
    Modify {
        id: u32,

        /// Priority(H/M/L)
        #[arg(value_enum, short, long)]
        priority: Option<Priority>,

        /// Due date
        #[arg(short, long, value_parser = date::validate_date_str)]
        due: Option<NaiveDate>,
    },

    /// Done sigo
    Done { id: u32 },

    /// Change sigo waiting
    Wait {
        id: u32,

        /// Description text
        #[arg(short, long)]
        text: Option<String>,
    },

    /// Change sigo ready
    Back {
        id: u32,

        /// Description text
        #[arg(short, long)]
        text: Option<String>,
    },

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

#[derive(
    Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Display, Serialize, Deserialize,
)]
enum Priority {
    H,
    M,
    L,
}

fn main() {
    // load config.ini
    let xdg_dirs = xdg::BaseDirectories::with_prefix("sigotowarrior").expect("XDG is not used");
    let config_path = xdg_dirs.get_config_file("config.ini");
    let cfg = confy::load_path::<MyConfig>(&config_path).expect("cannot load config.ini");

    // if task dir doesnot exist, create dir
    let sigo_path = PathBuf::from(&cfg.data);
    if !sigo_path.is_dir() {
        let _ = fs::create_dir(sigo_path);
    }

    // Parse args and Run command
    let cli = AppArg::parse();
    match command::run(&cfg, cli) {
        Ok(output) => {
            println!("{}", output.display(&cfg));
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
