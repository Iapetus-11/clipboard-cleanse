use core::error;

use crate::log;
use crate::windows::system_tray::destroy_system_tray;
use crate::Config;

use windows::Win32::UI::Shell::NOTIFYICONDATAW;
use windows::Win32::UI::WindowsAndMessaging::WM_LBUTTONDOWN;
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        DefWindowProcW, DispatchMessageW, PeekMessageW, MSG, PM_REMOVE, WM_CLIPBOARDUPDATE, WM_QUIT,
    },
};

use super::clipboard_listener::{
    destroy_clipboard_listener, handle_clipboard_changed, setup_clipboard_listener,
};
use super::ctrlc_handler::setup_ctrlc_handler;
use super::system_tray::setup_system_tray;
use super::window::{destroy_window, init_window};
use super::wm_user::WmUser;

struct App {
    hwnd: HWND,
    nid: NOTIFYICONDATAW,
}

pub fn process_win32_events_forever(hwnd: HWND) {
    log!(Debug, "Running event loop...");

    let mut msg: MSG = MSG::default();
    while msg.message != WM_QUIT {
        unsafe {
            if PeekMessageW(&mut msg, hwnd, 0, 0, PM_REMOVE).into() {
                DispatchMessageW(&msg);
            }
        }
    }
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        msg if msg == WM_CLIPBOARDUPDATE => handle_clipboard_changed(hwnd),
        msg if msg == WmUser::ShellIcon as u32 => {
            log!(Debug, "SHELLICON event detected!");

            if lparam.0 as u32 == WM_LBUTTONDOWN {
                log!(Debug, "Shell icon was clicked?");
            }

            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn setup() -> Result<App, Box<dyn error::Error>> {
    let hwnd = init_window(wnd_proc)?;

    setup_ctrlc_handler(hwnd)?;
    setup_clipboard_listener(hwnd)?;
    let nid = setup_system_tray(hwnd)?;

    Ok(App { hwnd, nid })
}

fn destroy(app: &App) -> Result<(), Box<dyn error::Error>> {
    destroy_clipboard_listener(app.hwnd)?;
    destroy_system_tray(&app.nid);
    destroy_window(app.hwnd)?;

    log!(Debug, "Goodbye!");

    Ok(())
}

pub fn main(_config: Config) {
    let app = setup().unwrap();

    process_win32_events_forever(app.hwnd);

    destroy(&app).unwrap();
}
