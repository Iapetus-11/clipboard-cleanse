use std::sync::LazyLock;

use windows::Win32::{
    Foundation::{HINSTANCE, HWND},
    UI::{
        Shell::{
            Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD,
            NOTIFYICONDATAW,
        },
        WindowsAndMessaging::{LoadIconW, IDI_QUESTION},
    },
};

use crate::windows::wm_user::WM_USER;

use super::win_utils::str_to_u16_nul_term_array;

static SYS_TRAY_UID: LazyLock<u32> = LazyLock::new(rand::random::<u32>);

pub fn setup_system_tray(hwnd: HWND) {
    let nid = NOTIFYICONDATAW {
        cbSize: size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: *SYS_TRAY_UID,
        uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP | NIF_SHOWTIP,
        uCallbackMessage: WM_USER::SHELLICON as u32,
        hIcon: unsafe { LoadIconW(HINSTANCE::default(), IDI_QUESTION).unwrap() },
        szTip: str_to_u16_nul_term_array::<128>("Clipboard Cleanse").unwrap(),
        ..Default::default()
    };

    unsafe { Shell_NotifyIconW(NIM_ADD, &nid) }.unwrap();
}
