use windows::Win32::{
    Foundation::{GetLastError, HWND},
    UI::WindowsAndMessaging::{
        CreatePopupMenu, InsertMenuW, SendMessageW, TrackPopupMenu, HMENU, MF_BYPOSITION,
        MF_STRING, TPM_BOTTOMALIGN, TPM_HORNEGANIMATION, TPM_LEFTALIGN, TPM_RIGHTBUTTON,
        WM_CANCELMODE,
    },
};
use windows_result::HRESULT;

use crate::{log, windows::system_tray::get_system_tray_item_rect};

use super::{win_utils::str_as_pcwstr, wm_command::WmCommand};

pub fn setup_menu() -> windows::core::Result<HMENU> {
    unsafe {
        let menu = CreatePopupMenu()?;

        InsertMenuW(
            menu,
            0xFFFFFFFF,
            MF_BYPOSITION | MF_STRING,
            WmCommand::MenuToggleAutoStart.into(),
            str_as_pcwstr("Toggle Auto-Start").value,
        )?;

        InsertMenuW(
            menu,
            0xFFFFFFFF,
            MF_BYPOSITION | MF_STRING,
            WmCommand::MenuOpenConfigFile.into(),
            str_as_pcwstr("Open Config File").value,
        )?;

        InsertMenuW(
            menu,
            0xFFFFFFFF,
            MF_BYPOSITION | MF_STRING,
            WmCommand::MenuQuit.into(),
            str_as_pcwstr("Quit").value,
        )?;

        Ok(menu)
    }
}

pub fn show_menu_and_handle_action(hwnd: HWND, menu: HMENU) -> windows::core::Result<()> {
    log!(Debug, "Showing system tray icon menu...");

    let rect = get_system_tray_item_rect(hwnd);

    if rect.is_err() {
        log!(
            Error,
            "Failed to get rect of system tray item to show menu :/"
        );
        return Err(rect.unwrap_err());
    }
    let rect = rect.unwrap();

    let success = unsafe {
        TrackPopupMenu(
            menu,
            TPM_LEFTALIGN | TPM_RIGHTBUTTON | TPM_BOTTOMALIGN | TPM_HORNEGANIMATION,
            rect.left,
            rect.top,
            None,
            hwnd,
            None,
        )
    };

    if success == false {
        let error = windows::core::Error::from(unsafe { GetLastError() });

        // Popup menu already active
        if error.code() == HRESULT(0x800705A6_u32 as i32) {
            log!(Debug, "Closing system tray menu...");

            unsafe { SendMessageW(hwnd, WM_CANCELMODE, None, None) };

            log!(Debug, "Requested system menu to be closed...");

            return Ok(());
        }

        log!(Error, "Failed to open system tray icon popup menu :/");
        return Err(error);
    }

    log!(Debug, "System tray menu closed");

    Ok(())
}
