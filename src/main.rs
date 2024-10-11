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
    let was_invalid_log_level = log_level.is_err();
    let logger = logger::Logger::new(
        log_level.unwrap_or(logger::LogLevel::Info),
        config.log_file.clone(),
    );

    if was_invalid_log_level {
        logger.error(&format!(
            "Invalid log level configured: {:#?}",
            config.log_level
        ));
    }

    logger.debug(&format!("Loaded config: {:#?}", config));

    #[cfg(target_os = "macos")]
    macos::main(config, logger);

    #[cfg(target_os = "windows")]
    windows::main(config, logger);
}
