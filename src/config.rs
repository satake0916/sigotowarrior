use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    pub home: String,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap();
        let mut home = PathBuf::from(&home);
        home.push(".sigo");
        Self {
            home: home
                .into_os_string()
                .into_string()
                .expect("HOME is not UTF-8"),
        }
    }
}
