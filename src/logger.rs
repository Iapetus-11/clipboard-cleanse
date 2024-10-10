use std::{
    fmt::{Debug, Display},
    fs::{File, OpenOptions}, io::{self, Write}, sync::Mutex,
};

#[derive(Debug, PartialEq, PartialOrd)]
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
            match self {
                &Self::Debug => "DEBUG",
                &Self::Info => "INFO",
                &Self::Warning => "WARNING",
                &Self::Error => "ERROR",
            }
        )
    }
}

#[derive(Debug)]
pub struct Logger {
    threshold: LogLevel,
    file: Option<Mutex<File>>,
}

impl Logger {
    pub fn new(threshold: LogLevel, file_path: Option<String>) -> Self {
        let file = file_path.map(|fp| {
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&fp)
                .expect(&format!("Failed to open log file {fp}"))
        });

        Logger { threshold, file: file.map(Mutex::new) }
    }

    pub fn log(&self, level: LogLevel, text: &str) {
        if level >= self.threshold {
            let formatted = format!("{level}: {text}\n");
            io::stdout().write_all(formatted.as_bytes()).unwrap();
            io::stdout().flush().unwrap();

            if let Some(file) = &self.file {
                let mut file = file.lock().unwrap();
                file.write_all(formatted.as_bytes()).unwrap();
                file.flush().unwrap();
            }
        }
    }

    pub fn debug(&self, text: &str) {
        self.log(LogLevel::Debug, text);
    }

    pub fn info(&self, text: &str) {
        self.log(LogLevel::Info, text);
    }

    pub fn warning(&self, text: &str) {
        self.log(LogLevel::Warning, text);
    }

    pub fn error(&self, text: &str) {
        self.log(LogLevel::Error, text);
    }
}
