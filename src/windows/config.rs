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
pub struct Config {
    #[serde(skip_serializing, default)]
    pub config_path: String,

    #[serde(default = "config_default_log_level")]
    pub log_level: String,

    #[serde(default = "config_default_log_file")]
    pub log_file: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_path: "".into(),
            log_level: "INFO".into(),
            log_file: None,
        }
    }
}
