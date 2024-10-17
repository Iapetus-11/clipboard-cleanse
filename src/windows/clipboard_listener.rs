use windows::Win32::{
    Foundation::{HWND, LRESULT},
    System::DataExchange::{AddClipboardFormatListener, RemoveClipboardFormatListener},
};

use crate::{log, windows::clipboard::Clipboard};

pub fn handle_clipboard_changed(hwnd: HWND) -> LRESULT {
    let mut clipboard = Clipboard::new(hwnd);

    // TODO: For some reason, reading from the text clipboard breaks pasting other types of content (Ex: images)

    log!(
        Debug,
        "clipboard: {:#?}",
        clipboard.get_string().unwrap_or(None).unwrap_or("".into())
    );

    LRESULT(0)
}

pub fn setup_clipboard_listener(hwnd: HWND) -> windows::core::Result<()> {
    log!(Debug, "Registering clipboard listener...");

    unsafe {
        AddClipboardFormatListener(hwnd)?;
    }

    log!(Debug, "Clipboard listener registered!");

    Ok(())
}

pub fn destroy_clipboard_listener(hwnd: HWND) -> windows::core::Result<()> {
    log!(Debug, "Unregistering clipboard listener...");

    unsafe {
        RemoveClipboardFormatListener(hwnd)?;
    }

    log!(Debug, "Clipboard listener unregistered!");

    Ok(())
}
