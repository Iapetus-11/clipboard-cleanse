use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        System::{DataExchange::AddClipboardFormatListener, LibraryLoader::GetModuleHandleW},
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, PeekMessageW, PostQuitMessage,
            RegisterClassW, CW_USEDEFAULT, MSG, PM_REMOVE, WINDOW_EX_STYLE, WM_CLIPBOARDUPDATE,
            WM_DESTROY, WM_QUIT, WNDCLASSW, WS_BORDER, WS_OVERLAPPEDWINDOW,
        },
    },
};

use crate::logger::Logger;
use crate::Config;

use super::Clipboard;

pub fn process_win32_events_forever() {
    let mut msg = MSG::default();
    while msg.message != WM_QUIT {
        unsafe {
            if PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {
                DispatchMessageW(&msg);
            }
        }
    }
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        msg if msg == WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        msg if msg == WM_CLIPBOARDUPDATE => {
            let mut clipboard = Clipboard::new(hwnd);

            println!("clipboard: {:?}", clipboard.get_string());

            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn setup_clipboard_listener(hwnd: HWND, logger: &Logger) {
    logger.debug("Registering clipboard listener...");

    unsafe {
        AddClipboardFormatListener(hwnd).unwrap();
        // https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-clipboardupdate
    }

    logger.debug("Clipboard listener registered!");
}

fn init_window(logger: &Logger) -> HWND {
    logger.debug("Initializing window...");

    let wnd_class_name = "ClipboardCleanseWindow"
        .encode_utf16()
        .collect::<Vec<u16>>();

    let hwnd = unsafe {
        let h_instance: HINSTANCE = GetModuleHandleW(None).unwrap().into();

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: h_instance,
            lpszClassName: PCWSTR::from_raw(wnd_class_name.as_ptr()),
            ..Default::default()
        };

        RegisterClassW(&wnd_class);

        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            PCWSTR(wnd_class_name.as_ptr()),
            PCWSTR(
                "Clipboard Cleanse"
                    .encode_utf16()
                    .collect::<Vec<u16>>()
                    .as_ptr(),
            ),
            WS_BORDER | WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            h_instance,
            None,
        )
        .unwrap()
    };

    logger.debug("Window initialized!");

    hwnd
}

pub fn main(_config: Config, logger: Logger) {
    logger.debug("Initializing window...");
    let hwnd = init_window(&logger);
    logger.debug("Window initialized!");

    setup_clipboard_listener(hwnd, &logger);

    logger.debug("Running event loop...");
    process_win32_events_forever();
}
