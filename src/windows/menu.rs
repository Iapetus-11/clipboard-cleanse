use windows::Win32::{
    Foundation::{GetLastError, HWND},
    UI::WindowsAndMessaging::{
        CreatePopupMenu, InsertMenuW, TrackPopupMenu, HMENU, MF_BYPOSITION, MF_STRING,
        TPM_BOTTOMALIGN, TPM_LEFTALIGN, TPM_LEFTBUTTON,
    },
};
use windows_result::HRESULT;

use crate::{log, windows::system_tray::get_system_tray_item_rect};

use super::win_utils::str_to_pcwstr;

pub enum MenuCommand {
    ToggleAutoStart = 1,
    OpenConfigFile = 2,
    Quit = 3,
}

impl TryFrom<usize> for MenuCommand {
    type Error = usize;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MenuCommand::ToggleAutoStart),
            2 => Ok(MenuCommand::OpenConfigFile),
            3 => Ok(MenuCommand::Quit),
            _ => Err(value)
        }
    }
}

pub fn setup_menu() -> windows::core::Result<HMENU> {
    unsafe {
        let menu = CreatePopupMenu()?;

        InsertMenuW(
            menu,
            0xFFFFFFFF,
            MF_BYPOSITION | MF_STRING,
            MenuCommand::ToggleAutoStart as usize,
            str_to_pcwstr("Toggle Auto-Start"),
        )?;

        InsertMenuW(
            menu,
            0xFFFFFFFF,
            MF_BYPOSITION | MF_STRING,
            MenuCommand::OpenConfigFile as usize,
            str_to_pcwstr("Open Config File"),
        )?;

        InsertMenuW(
            menu,
            0xFFFFFFFF,
            MF_BYPOSITION | MF_STRING,
            MenuCommand::Quit as usize,
            str_to_pcwstr("Quit"),
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
            TPM_LEFTALIGN | TPM_LEFTBUTTON | TPM_BOTTOMALIGN,
            rect.left,
            rect.top,
            0,
            hwnd,
            None,
        )
    };

    if success == false {
        let error = windows::core::Error::from(unsafe { GetLastError() });

        // Popup menu already active
        if error.code() == HRESULT(0x800705A6_u32 as i32) {
            return Ok(());
        }

        log!(Error, "Failed to open system tray icon popup menu :/");
        return Err(error);
    }

    log!(Debug, "Opened system tray icon popup menu");

    Ok(())
}
