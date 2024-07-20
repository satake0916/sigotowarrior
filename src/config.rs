use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    pub data: String,
    pub mode: Mode,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("sigotowarrior").expect("XDG is not used");
        let data_dir = xdg_dirs.get_data_home();
        Self {
            data: data_dir
                .into_os_string()
                .into_string()
                .expect("XDG_DATA_HOME is not set"),
            mode: Mode::Simple,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Mode {
    Minimum,
    Simple,
}
