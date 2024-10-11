use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
use crate::windows::{get_home_directory, Config as WindowsConfig};

#[cfg(target_os = "macos")]
use crate::macos::{get_home_directory, Config as MacOSConfig};

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

    #[serde(default = "MacOSConfig::default")]
    pub macos: MacOSConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_path: "".into(),
            log_level: "INFO".into(),
            log_file: None,

            #[cfg(target_os = "macos")]
            macos: MacOSConfig::default(),
        }
    }
}

fn get_config_file_path() -> PathBuf {
    let mut home_dir = get_home_directory();

    home_dir.extend([".config", "clipboard_cleanse", "config.toml"]);

    home_dir
}

fn write_config(config_path: &PathBuf, config: &Config) {
    let serialized_config = &toml::to_string_pretty(&config)
        .unwrap_or_else(|_| panic!("Serialization of config {:#?} to succeed", config));

    let mut file = File::create(config_path)
        .unwrap_or_else(|_| panic!("Config file creation at {:#?} to succeed", config_path));

    file.write_all(serialized_config.as_bytes())
        .expect("Writing default config to succeed");
}

/// Ensures the config exists as the specified path, returning true if it had to be created from the defaults.
fn ensure_config_exists(config_path: &PathBuf) -> bool {
    if !config_path.try_exists().unwrap() {
        if !config_path.parent().unwrap().exists() {
            create_dir_all(config_path.parent().unwrap()).expect("Creating config dir to succeed");
        }

        write_config(config_path, &Config::default());

        true
    } else {
        false
    }
}

pub fn load_and_ensure_config() -> Config {
    let config_path = get_config_file_path();

    let created_new_config = ensure_config_exists(&config_path);

    let mut config = if created_new_config {
        Config::default()
    } else {
        let mut config_file_data = Vec::<u8>::new();
        let mut config_file = File::open(&config_path)
            .unwrap_or_else(|_| panic!("Opening config file at {:#?} to succeed", config_path));
        config_file
            .read_to_end(&mut config_file_data)
            .expect("Reading config file to succeed");

        toml::from_str(&String::from_utf8(config_file_data).expect("Valid utf8"))
            .expect("Config file to be valid")
    };

    write_config(&config_path, &config);

    config.config_path = config_path.to_str().unwrap().to_string();

    config
}
