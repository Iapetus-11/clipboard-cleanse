use std::thread;

use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::{HGLOBAL, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        System::{
            DataExchange::{
                AddClipboardFormatListener, CloseClipboard, GetClipboardData, OpenClipboard,
            },
            LibraryLoader::GetModuleHandleW,
            Memory::{GlobalAlloc, GlobalLock, GMEM_MOVEABLE},
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, PeekMessageW, PostQuitMessage,
            RegisterClassW, CW_USEDEFAULT, MSG, PM_REMOVE, WINDOW_EX_STYLE, WM_CLIPBOARDUPDATE,
            WM_DESTROY, WM_QUIT, WNDCLASSW, WS_BORDER, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

use crate::logger::Logger;
use crate::Config;

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
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        msg if msg == WM_CLIPBOARDUPDATE => {
            println!("Clipboard was updated! {wparam:#?} {lparam:#?}");
            unsafe {
                OpenClipboard(hwnd).unwrap();
            }
            let clipboard_data = unsafe {
                let clipboard_data = GetClipboardData(13).unwrap(); // CF_UNICODETEXT
                let x = GlobalAlloc(GMEM_MOVEABLE, 1000);
                let clipboard_data = GlobalLock(x);
            };
            println!("clipboard data: {clipboard_data:#?}");
            unsafe {
                CloseClipboard().unwrap();
            }
            // println!("monkas: {:#?}", clipboard_data);

            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn setup_clipboard_listener(hwnd: HWND) {
    unsafe {
        AddClipboardFormatListener(hwnd).unwrap();
        // https://learn.microsoft.com/en-us/windows/win32/dataxchg/wm-clipboardupdate
    }
}

fn init_window() -> HWND {
    unsafe {
        let class_name = "ClipboardCleanseWindow"
            .encode_utf16()
            .collect::<Vec<u16>>();

        let h_instance: HINSTANCE = GetModuleHandleW(None).unwrap().into();
        println!("30");

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: h_instance,
            lpszClassName: PCWSTR::from_raw(class_name.as_ptr()),
            ..Default::default()
        };
        println!("36: {:#?}", wnd_class);

        RegisterClassW(&wnd_class);
        println!("39");

        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            PCWSTR::from_raw(class_name.as_ptr()),
            PCWSTR::from_raw("My Window".encode_utf16().collect::<Vec<u16>>().as_ptr()),
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
    }
}

pub fn main(_config: Config, logger: Logger) {
    let hwnd = init_window();
    logger.debug(&format!("hwnd: {:#?}", hwnd));

    // unsafe {
    //     OpenClipboard(hwnd).unwrap();
    // }
    // println!("after open clipboard");

    setup_clipboard_listener(hwnd);
    process_win32_events_forever();
}
