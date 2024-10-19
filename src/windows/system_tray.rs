use std::sync::LazyLock;

use windows::Win32::{
    Foundation::{HINSTANCE, HWND},
    UI::{
        Shell::{
            Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD, NIM_DELETE,
            NOTIFYICONDATAW,
        },
        WindowsAndMessaging::{LoadIconW, IDI_QUESTION},
    },
};

use crate::{log, windows::wm_user::WmUser};

use super::win_utils::str_to_u16_nul_term_array;

static SYS_TRAY_UID: LazyLock<u32> = LazyLock::new(rand::random::<u32>);

pub fn setup_system_tray(hwnd: HWND) -> Result<NOTIFYICONDATAW, String> {
    let nid = NOTIFYICONDATAW {
        cbSize: size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: *SYS_TRAY_UID,
        uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP | NIF_SHOWTIP,
        uCallbackMessage: WmUser::ShellIcon as u32,
        hIcon: unsafe { LoadIconW(HINSTANCE::default(), IDI_QUESTION).unwrap() },
        szTip: str_to_u16_nul_term_array::<128>("Clipboard Cleanse").unwrap(),
        ..Default::default()
    };

    let success = unsafe { Shell_NotifyIconW(NIM_ADD, &nid) }.as_bool();

    match success {
        true => {
            log!(Debug, "System tray setup");

            Ok(nid)
        }
        false => {
            log!(Error, "Failed to setup system tray :/");

            Err("Failed to setup system tray (Shell_NotifyIconW returned FALSE)".into())
        }
    }
}

pub fn destroy_system_tray(nid: &NOTIFYICONDATAW) {
    let success = unsafe { Shell_NotifyIconW(NIM_DELETE, nid) }.as_bool();

    match success {
        true => log!(Debug, "System tray destroyed"),
        false => log!(Error, "Failed to destroy system tray :/"),
    }
}
