use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        System::{DataExchange::AddClipboardFormatListener, LibraryLoader::GetModuleHandleW},
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, PeekMessageW, PostQuitMessage,
            RegisterClassW, CW_USEDEFAULT, MSG, PM_REMOVE, WINDOW_EX_STYLE, WM_DESTROY, WM_QUIT,
            WNDCLASSW, WS_BORDER, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
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
                println!("{:#?}", msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

fn setup_clipboard_listener(hwnd: HWND, callback: Box<dyn Fn()>) {
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
            WS_BORDER | WS_VISIBLE | WS_OVERLAPPEDWINDOW,
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

    setup_clipboard_listener(hwnd, Box::new(move || logger.debug("COPY!")));
    process_win32_events_forever();
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        msg if msg == WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}
