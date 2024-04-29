use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    pub home: String,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap();
        // let mut home = PathBuf::from(&home);
        // home.push(".sigo");
        // std::fs::create_dir(&home);
        // let mut ready_file = PathBuf::from(&home);
        // ready_file.push("ready_tasks");
        // std::fs::File::create_new(&ready_file).unwrap();
        Self { home: home }
    }
}
