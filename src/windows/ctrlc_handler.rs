use std::sync::atomic::{self, AtomicI32};

use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, WPARAM},
    System::Console::{SetConsoleCtrlHandler, CTRL_C_EVENT},
    UI::WindowsAndMessaging::{PostMessageW, WM_QUIT},
};

use crate::log;

static CTRLC_COUNTER: AtomicI32 = AtomicI32::new(0);
static mut GLOBAL_HWND: Option<HWND> = None;

extern "system" fn console_handler(signal: u32) -> BOOL {
    if unsafe { GLOBAL_HWND.is_none() } {
        return false.into();
    }

    let hwnd = unsafe { GLOBAL_HWND.unwrap() };

    match signal {
        CTRL_C_EVENT => {
            let attempt_graceful = CTRLC_COUNTER.fetch_add(1, atomic::Ordering::Relaxed) < 1;

            if attempt_graceful {
                log!(Debug, "CTRL+C intercepted, attempting graceful shutdown (use CTRL+C again to terminate)...");
            }

            match attempt_graceful {
                true => unsafe {
                    PostMessageW(hwnd, WM_QUIT, WPARAM(0), LPARAM(0)).unwrap();
                },
                false => {},
            }

            attempt_graceful
        }
        _ => false,
    }
    .into()
}

pub fn setup_ctrlc_handler(hwnd: HWND) -> windows::core::Result<()> {
    unsafe {
        GLOBAL_HWND = Some(hwnd);
        SetConsoleCtrlHandler(Some(console_handler), true)
    }
}
