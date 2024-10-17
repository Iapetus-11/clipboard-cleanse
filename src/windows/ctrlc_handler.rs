use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, WPARAM},
    System::Console::{SetConsoleCtrlHandler, CTRL_C_EVENT},
    UI::WindowsAndMessaging::{PostMessageW, WM_QUIT},
};

static mut CTRLC_COUNTER: i32 = -1;
static mut G_HWND: Option<HWND> = None;

extern "system" fn console_handler(signal: u32) -> BOOL {
    match signal {
        CTRL_C_EVENT => {
            unsafe {
                PostMessageW(G_HWND.unwrap(), WM_QUIT, WPARAM(0), LPARAM(0)).unwrap();

                // If graceful shutdown isn't working, provide an escape hatch
                CTRLC_COUNTER += 1;
                CTRLC_COUNTER < 1
            }
        }
        _ => false,
    }
    .into()
}

pub fn setup_ctrlc_handler(hwnd: HWND) -> windows::core::Result<()> {
    unsafe {
        G_HWND = Some(hwnd);
        SetConsoleCtrlHandler(Some(console_handler), true)
    }
}
