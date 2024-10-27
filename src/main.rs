#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::load_and_ensure_config;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

mod config;
mod logger;
mod sanitization;

pub use config::Config;

fn main() {
    let config = load_and_ensure_config();

    let log_level = logger::LogLevel::try_from(config.log_level.as_str());

    if let Ok(log_level) = log_level {
        logger::set_threshold(log_level);
    } else {
        log!(Error, "Invalid log level configured: {log_level:#?}");
    }

    logger::set_file(match &config.log_file {
        Some(lfp) => Some(lfp.as_str()),
        None => None,
    });

    log!(Info, "Loaded config: {config:#?}");

    #[cfg(target_os = "macos")]
    macos::main(config);

    #[cfg(target_os = "windows")]
    windows::main(config);
}
