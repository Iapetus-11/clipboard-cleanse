use core::error;
use std::process;

use crate::log;
use crate::windows::get_home_directory;
use crate::windows::menu::MenuCommand;
use crate::windows::system_tray::destroy_system_tray_item;
use crate::Config;

use windows::Win32::UI::Shell::NOTIFYICONDATAW;
use windows::Win32::UI::WindowsAndMessaging::{PostMessageW, HMENU, WM_COMMAND, WM_LBUTTONDOWN};
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
use super::menu::{setup_menu, show_menu_and_handle_action};
use super::system_tray::setup_system_tray_item;
use super::window::{destroy_window, init_window};
use super::wm_user::WmUser;

#[derive(Clone)]
struct App {
    config: Config,
    hwnd: HWND,
    nid: NOTIFYICONDATAW,
    menu: HMENU,
}

static mut APP: Option<App> = None;

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
            if lparam.0 as u32 == WM_LBUTTONDOWN {
                let app = unsafe { APP.clone().unwrap() };
                show_menu_and_handle_action(hwnd, app.menu).unwrap();
            }

            LRESULT(0)
        }
        msg if msg == WM_COMMAND => {
            let app = unsafe { APP.clone().unwrap() };

            match MenuCommand::try_from(wparam.0) {
                Ok(MenuCommand::ToggleAutoStart) => {
                    // C:\Users\miloi\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup

                    let mut startup_dir = get_home_directory();
                    startup_dir.extend(["AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup"]);

                    todo!("Handle toggle auto start!");
                }
                Ok(MenuCommand::OpenConfigFile) => {
                    process::Command::new("cmd")
                        .args(["/c", "start", &app.config.config_path])
                        .output()
                        .unwrap();
                }
                Ok(MenuCommand::Quit) => unsafe {
                    PostMessageW(hwnd, WM_QUIT, WPARAM(0), LPARAM(0)).unwrap();
                }
                _ => {},
            }

            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn setup(config: Config) -> Result<App, Box<dyn error::Error>> {
    let hwnd = init_window(wnd_proc)?;

    setup_ctrlc_handler(hwnd)?;
    setup_clipboard_listener(hwnd)?;
    let nid = setup_system_tray_item(hwnd)?;
    let menu = setup_menu()?;

    Ok(App { hwnd, nid, menu, config })
}

fn destroy(app: &App) -> Result<(), Box<dyn error::Error>> {
    destroy_clipboard_listener(app.hwnd)?;
    destroy_system_tray_item(&app.nid);
    destroy_window(app.hwnd)?;
    // TODO destroy menu

    log!(Debug, "Goodbye!");

    Ok(())
}

pub fn main(config: Config) {
    let app = setup(config).unwrap();
    unsafe {
        APP = Some(app.clone());
    }

    process_win32_events_forever(app.hwnd);

    destroy(&app).unwrap();
}
