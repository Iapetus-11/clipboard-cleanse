use crate::log;
use crate::Config;

use windows::Win32::UI::WindowsAndMessaging::{DestroyWindow, WM_LBUTTONDOWN};
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        DefWindowProcW, DispatchMessageW, PeekMessageW, MSG, PM_REMOVE, WM_CLIPBOARDUPDATE, WM_QUIT,
    },
};

use super::clipboard_listener::destroy_clipboard_listener;
use super::clipboard_listener::handle_clipboard_changed;
use super::clipboard_listener::setup_clipboard_listener;
use super::ctrlc_handler::setup_ctrlc_handler;
use super::system_tray::setup_system_tray;
use super::window::init_window;
use super::wm_user::WM_USER;

pub fn destroy(hwnd: HWND) -> windows::core::Result<()> {
    destroy_clipboard_listener(hwnd)?;

    unsafe { DestroyWindow(hwnd) }
}

pub fn process_win32_events_forever(hwnd: HWND) -> windows::core::Result<()> {
    log!(Debug, "Running event loop...");

    let mut msg: MSG = MSG::default();
    while msg.message != WM_QUIT {
        unsafe {
            if PeekMessageW(&mut msg, hwnd, 0, 0, PM_REMOVE).into() {
                DispatchMessageW(&msg);
            }
        }
    }

    destroy(hwnd)
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        msg if msg == WM_CLIPBOARDUPDATE => handle_clipboard_changed(hwnd),
        msg if msg == WM_USER::SHELLICON as u32 => {
            log!(Debug, "SHELLICON event detected!");

            if lparam.0 as u32 == WM_LBUTTONDOWN {
                log!(Debug, "Shell icon was clicked?");
            }

            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

pub fn main(_config: Config) {
    let hwnd = init_window(wnd_proc).unwrap();

    setup_ctrlc_handler(hwnd).unwrap();

    setup_clipboard_listener(hwnd).unwrap();

    setup_system_tray(hwnd);

    process_win32_events_forever(hwnd).unwrap();
}
