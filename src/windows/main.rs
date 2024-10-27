use core::error;
use std::env::current_exe;
use std::error::Error;
use std::{fs, process};

use crate::log;
use crate::windows::get_home_directory;
use crate::windows::system_tray::destroy_system_tray_item;
use crate::Config;

use windows::Win32::UI::Shell::NOTIFYICONDATAW;
use windows::Win32::UI::WindowsAndMessaging::{
    PostMessageW, HMENU, WM_COMMAND, WM_LBUTTONDOWN, WM_RBUTTONDOWN,
};
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
use super::shell_link::create_shortcut;
use super::system_tray::setup_system_tray_item;
use super::window::{destroy_window, init_window};
use super::wm_command::WmCommand;
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

fn process_wm_command(app: App, cmd: WmCommand) -> Result<(), Box<dyn Error>> {
    match cmd {
        WmCommand::MenuToggleAutoStart => {
            let startup_file_path = {
                let mut path = get_home_directory();

                path.extend([
                    "AppData",
                    "Roaming",
                    "Microsoft",
                    "Windows",
                    "Start Menu",
                    "Programs",
                    "Startup",
                    // "Desktop",
                    "Clipboard Cleanse.lnk",
                ]);

                path
            };

            if fs::exists(&startup_file_path)? {
                log!(
                    Debug,
                    "Removing shortcut from startup folder ({})...",
                    startup_file_path.to_string_lossy()
                );

                fs::remove_file(&startup_file_path)?;

                log!(Debug, "Removed shortcut from startup folder")
            } else {
                log!(
                    Debug,
                    "Creating shortcut in startup folder ({})...",
                    startup_file_path.to_string_lossy()
                );

                let exe_path = current_exe()?;
                let exe_path = exe_path.to_str().unwrap().strip_suffix(".exe").unwrap();

                create_shortcut(exe_path, startup_file_path.to_str().unwrap())?;

                log!(Debug, "Created shortcut in startup folder");
            }
        }
        WmCommand::MenuOpenConfigFile => {
            process::Command::new("cmd")
                .args(["/c", "start", &app.config.config_path])
                .output()
                .unwrap();
        }
        WmCommand::MenuQuit => unsafe {
            PostMessageW(app.hwnd, WM_QUIT, WPARAM(0), LPARAM(0)).unwrap();
        },
    }

    Ok(())
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        msg if msg == WM_CLIPBOARDUPDATE => handle_clipboard_changed(hwnd),
        msg if msg == WmUser::ShellIcon as u32 => {
            let lparam = lparam.0 as u32;

            if lparam == WM_LBUTTONDOWN || lparam == WM_RBUTTONDOWN {
                let app = unsafe { APP.clone().unwrap() };
                show_menu_and_handle_action(hwnd, app.menu).unwrap();
            }

            LRESULT(0)
        }
        msg if msg == WM_COMMAND => {
            let wm_cmd = WmCommand::try_from(wparam.0);

            if let Ok(wm_cmd) = wm_cmd {
                let app = unsafe { APP.clone().unwrap() };

                match process_wm_command(app, wm_cmd.clone()) {
                    Ok(_) => {}
                    Err(err) => {
                        log!(
                            Error,
                            "Failed to process {wm_cmd:?} ({wparam:?}, {lparam:?}): {err:?}"
                        );
                    }
                }
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

    Ok(App {
        hwnd,
        nid,
        menu,
        config,
    })
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
