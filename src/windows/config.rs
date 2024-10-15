use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub fn get_home_directory() -> PathBuf {
    PathBuf::from(r"C:\Users\miloi")
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {}
