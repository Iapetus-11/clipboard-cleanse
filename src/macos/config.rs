use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::appkit::NSFileManager;

pub fn get_home_directory() -> PathBuf {
    let home_dir_url = NSFileManager::get_default_manager().get_home_directory_for_current_user();
    PathBuf::from(
        home_dir_url
            .get_absolute_string()
            .strip_prefix("file://")
            .unwrap(),
    )
}

fn config_default_poll_interval_ms() -> u64 {
    500_u64
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

    #[serde(default = "config_default_poll_interval_ms")]
    pub poll_interval_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_path: "".into(),
            log_level: "INFO".into(),
            log_file: None,
            poll_interval_ms: 500,
        }
    }
}
