use std::{
    fmt::{Debug, Display},
    fs::{File, OpenOptions},
    io::{self, Write},
    sync::{LazyLock, RwLock},
};

static LOGGER: LazyLock<RwLock<Logger>> = LazyLock::new(|| RwLock::new(Logger::default()));

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl TryFrom<&str> for LogLevel {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, <LogLevel as TryFrom<&str>>::Error> {
        match value.to_ascii_uppercase().as_str() {
            "DEBUG" => Ok(Self::Debug),
            "INFO" => Ok(Self::Info),
            "WARNING" => Ok(Self::Warning),
            "ERROR" => Ok(Self::Error),
            _ => Err(format!(
                "Expected one of DEBUG, INFO, WARNING, or ERROR, but got {:#?} instead",
                value
            )),
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Debug => "DEBUG",
                Self::Info => "INFO",
                Self::Warning => "WARNING",
                Self::Error => "ERROR",
            }
        )
    }
}

#[derive(Debug)]
pub struct Logger {
    threshold: LogLevel,
    file: Option<File>,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            threshold: LogLevel::Debug,
            file: None,
        }
    }
}

pub fn set_threshold(threshold: LogLevel) {
    let mut logger = LOGGER.write().unwrap();
    logger.threshold = threshold;
}

pub fn set_file(file_path: Option<&str>) {
    let file = file_path.map(|fp| {
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(fp)
            .unwrap_or_else(|err| panic!("Failed to open log file {fp} due to {err}"))
    });

    let mut logger = LOGGER.write().unwrap();
    logger.file = file;
}

pub fn log(level: LogLevel, text: &str) {
    let skip_log = {
        let logger = LOGGER.read().unwrap();
        level < logger.threshold
    };

    if skip_log {
        return;
    }

    let mut logger = LOGGER.write().unwrap();

    let formatted = format!("{level}: {text}\n");
    io::stdout().write_all(formatted.as_bytes()).unwrap();
    io::stdout().flush().unwrap();

    if let Some(file) = &mut logger.file {
        file.write_all(formatted.as_bytes()).unwrap();
        file.flush().unwrap();
    }
}

#[macro_export]
macro_rules! log {
    ($log_level:ident, $($arg:tt)*) => {
        {
            use $crate::logger::{log, LogLevel};
            log(LogLevel::$log_level, &format!($($arg)*));
        }
    };
}
