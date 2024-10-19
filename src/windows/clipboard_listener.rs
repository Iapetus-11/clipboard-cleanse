use windows::Win32::{
    Foundation::{HWND, LRESULT},
    System::DataExchange::{AddClipboardFormatListener, RemoveClipboardFormatListener},
};

use crate::{log, sanitization::sanitize, windows::clipboard::Clipboard};

pub fn handle_clipboard_changed(hwnd: HWND) -> LRESULT {
    let mut clipboard = Clipboard::new(hwnd);

    if let Ok(Some(contents)) = clipboard.get_text() {
        let sanitized_contents = sanitize(&contents);

        if contents == sanitized_contents {
            return LRESULT(0);
        }

        let result = clipboard.set_text(sanitized_contents);

        match result {
            Ok(_) => log!(Info, "Sanitized copied text!"),
            Err(err) => log!(
                Error,
                "Failed to set clipboard with sanitized text due to: {err}"
            ),
        }
    }

    LRESULT(0)
}

pub fn setup_clipboard_listener(hwnd: HWND) -> windows::core::Result<()> {
    let result = unsafe { AddClipboardFormatListener(hwnd) };

    match result.is_ok() {
        true => log!(Debug, "Registered clipboard listener"),
        false => log!(Error, "Failed to register clipboard listener :/"),
    };

    result
}

pub fn destroy_clipboard_listener(hwnd: HWND) -> windows::core::Result<()> {
    let result = unsafe { RemoveClipboardFormatListener(hwnd) };

    match result.is_ok() {
        true => log!(Debug, "Removed clipboard listener"),
        false => log!(Error, "Failed to remove clipboard listener :/"),
    };

    result
}
