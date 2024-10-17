use std::ffi::c_void;

use windows::Win32::{
    Foundation::{HGLOBAL, HWND},
    System::{
        DataExchange::{CloseClipboard, GetClipboardData, OpenClipboard},
        Memory::{GlobalLock, GlobalSize, GlobalUnlock},
    },
};

#[derive(Debug)]
pub struct Clipboard {
    hwnd: HWND,
}

impl Clipboard {
    pub fn new(hwnd: HWND) -> Self {
        Self { hwnd }
    }

    fn open(&mut self) -> Result<(), windows_result::Error> {
        unsafe { OpenClipboard(self.hwnd) }
    }

    fn close(&mut self) -> Result<(), windows_result::Error> {
        unsafe { CloseClipboard() }
    }

    pub fn get_string(&mut self) -> Result<Option<String>, windows_result::Error> {
        self.open()?;

        let clipboard_data = {
            const CF_UNICODETEXT: u32 = 13;
            unsafe { GetClipboardData(CF_UNICODETEXT) }
        }?
        .0;

        if clipboard_data.is_null() {
            return Ok(None);
        }

        let clipboard_data = HGLOBAL(clipboard_data);

        let clipboard_data = unsafe {
            let clipboard_data_size = GlobalSize(clipboard_data);

            let mut string_data = vec![0_u16; clipboard_data_size / size_of::<u16>()];

            let locked_clipboard_data = GlobalLock(clipboard_data);
            if locked_clipboard_data.is_null() {
                return Ok(None);
            }

            locked_clipboard_data.copy_to(
                string_data.as_mut_ptr().cast::<c_void>(),
                clipboard_data_size,
            );

            GlobalUnlock(clipboard_data)?;

            string_data
        };

        self.close()?;

        Ok(Some(String::from_utf16_lossy(&clipboard_data)))
    }
}
