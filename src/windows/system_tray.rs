use std::{error::Error, sync::LazyLock};

use windows::{
    core::{GUID, PCWSTR},
    Win32::{
        Foundation::{HINSTANCE, HWND, RECT},
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Shell::{
                Shell_NotifyIconGetRect, Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP,
                NIM_ADD, NIM_DELETE, NOTIFYICONDATAW, NOTIFYICONIDENTIFIER,
            },
            WindowsAndMessaging::LoadIconW,
        },
    },
};

use crate::{log, windows::wm_user::WmUser};

use super::{resources::IDI_ICON, win_utils::str_to_u16_nul_term_array};

static NOTIFY_ICON_UID: LazyLock<u32> = LazyLock::new(rand::random::<u32>);

pub fn setup_system_tray_item(hwnd: HWND) -> Result<NOTIFYICONDATAW, Box<dyn Error>> {
    let h_instance: HINSTANCE = unsafe { GetModuleHandleW(None) }?.into();

    let icon = unsafe { LoadIconW(h_instance, PCWSTR(IDI_ICON as *mut u16)) }?;

    let nid = NOTIFYICONDATAW {
        cbSize: size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: *NOTIFY_ICON_UID,
        uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
        uCallbackMessage: WmUser::ShellIcon as u32,
        hIcon: icon,
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

pub fn destroy_system_tray_item(nid: &NOTIFYICONDATAW) {
    let success = unsafe { Shell_NotifyIconW(NIM_DELETE, nid) }.as_bool();

    match success {
        true => log!(Debug, "System tray destroyed"),
        false => log!(Error, "Failed to destroy system tray :/"),
    }
}

pub fn get_system_tray_item_rect(hwnd: HWND) -> windows::core::Result<RECT> {
    let identifier = NOTIFYICONIDENTIFIER {
        cbSize: size_of::<NOTIFYICONIDENTIFIER>() as u32,
        hWnd: hwnd,
        uID: *NOTIFY_ICON_UID,
        guidItem: GUID::zeroed(),
    };

    unsafe { Shell_NotifyIconGetRect(&identifier) }
}
