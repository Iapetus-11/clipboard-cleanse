use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub fn get_home_directory() -> PathBuf {
    PathBuf::from(r"C:\Users\miloi")
}

fn config_default_log_level() -> String {
    "INFO".into()
}

fn config_default_log_file() -> Option<String> {
    None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}
