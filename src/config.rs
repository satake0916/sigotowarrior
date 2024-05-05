use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    pub data: String,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("sigotorrior").expect("XDG is not used");
        let data_dir = xdg_dirs.get_data_home();
        Self {
            data: data_dir
                .into_os_string()
                .into_string()
                .expect("XDG_DATA_HOME is not set"),
        }
    }
}
