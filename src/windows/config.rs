use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use windows::Win32::{
    Foundation::{HANDLE, HWND},
    UI::Shell::{SHGetFolderPathW, CSIDL_PROFILE},
};

pub fn get_home_directory() -> PathBuf {
    PathBuf::from(unsafe {
        let mut out = [0_u16; 260];
        SHGetFolderPathW(
            HWND::default(),
            CSIDL_PROFILE as i32,
            HANDLE::default(),
            0,
            &mut out,
        )
        .unwrap();

        String::from_utf16_lossy(
            &out.into_iter()
                .take_while(|d| d != &0)
                .collect::<Vec<u16>>(),
        )
    })
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {}
