use std::ffi::c_void;

use windows::Win32::{
    Foundation::{HANDLE, HGLOBAL, HWND},
    System::{
        DataExchange::{
            CloseClipboard, EmptyClipboard, GetClipboardData, IsClipboardFormatAvailable,
            OpenClipboard, SetClipboardData,
        },
        Memory::{GlobalLock, GlobalSize, GlobalUnlock},
    },
};

use crate::log;

// TODO: Make proper enum
const CF_UNICODETEXT: u32 = 13;

#[derive(Debug)]
pub struct Clipboard {
    hwnd: HWND,
}

impl Clipboard {
    pub fn new(hwnd: HWND) -> Self {
        Self { hwnd }
    }

    fn open(&mut self) -> Result<(), windows_result::Error> {
        let result = unsafe { OpenClipboard(self.hwnd) };

        match result.is_ok() {
            true => log!(Debug, "Opened clipboard"),
            false => log!(Error, "Failed to open clipboard"),
        }

        result
    }

    fn close(&mut self) -> Result<(), windows_result::Error> {
        let result = unsafe { CloseClipboard() };

        match result.is_ok() {
            true => log!(Debug, "Closed clipboard"),
            false => log!(Error, "Failed to close clipboard"),
        }

        result
    }

    fn with_clipboard<T>(
        &mut self,
        execute: Box<dyn Fn() -> windows::core::Result<T>>,
    ) -> windows::core::Result<T> {
        self.open()?;

        let result = execute()?;

        self.close()?;

        Ok(result)
    }

    pub fn get_text(&mut self) -> windows::core::Result<Option<String>> {
        let clipboard_data = self.with_clipboard(Box::new(|| {
            if unsafe { IsClipboardFormatAvailable(CF_UNICODETEXT) }.is_err() {
                log!(Debug, "Clipboard for CF_UNICODETEXT is not available");
                return Ok(None);
            }

            let clipboard_data = unsafe { GetClipboardData(CF_UNICODETEXT) }?.0;

            if clipboard_data.is_null() {
                log!(Debug, "Clipboard data is null");
                return Ok(None);
            }

            let clipboard_data = HGLOBAL(clipboard_data);

            let clipboard_data = unsafe {
                let clipboard_data_size = GlobalSize(clipboard_data);

                let mut string_data = vec![0_u16; clipboard_data_size / size_of::<u16>()];

                let locked_clipboard_data = GlobalLock(clipboard_data);
                if locked_clipboard_data.is_null() {
                    log!(Debug, "Clipboard data is null");
                    return Ok(None);
                }

                locked_clipboard_data.copy_to(
                    string_data.as_mut_ptr().cast::<c_void>(),
                    clipboard_data_size,
                );

                GlobalUnlock(clipboard_data)?;

                string_data
            };

            Ok(Some(clipboard_data))
        }))?;

        Ok(clipboard_data.map(|cpd| String::from_utf16_lossy(&cpd)))
    }

    pub fn set_text(&mut self, text: String) -> windows::core::Result<()> {
        self.with_clipboard(Box::new(move || {
            unsafe {
                EmptyClipboard()?;
                SetClipboardData(
                    CF_UNICODETEXT,
                    HANDLE(
                        text.clone()
                            .encode_utf16()
                            .collect::<Box<[u16]>>()
                            .as_mut_ptr()
                            .cast::<c_void>(),
                    ),
                )?;
            }

            Ok(())
        }))
    }
}
