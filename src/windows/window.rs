use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, RegisterClassW, CW_USEDEFAULT, WINDOW_EX_STYLE, WNDCLASSW, WS_BORDER,
            WS_OVERLAPPEDWINDOW,
        },
    },
};

use crate::log;

pub type WindowProc = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;

pub fn init_window(wnd_proc: WindowProc) -> windows::core::Result<HWND> {
    log!(Debug, "Initializing window...");

    let wnd_class_name = "ClipboardCleanseWindow"
        .encode_utf16()
        .collect::<Vec<u16>>();

    let hwnd = unsafe {
        let h_instance: HINSTANCE = GetModuleHandleW(None)?.into();

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: h_instance,
            lpszClassName: PCWSTR(wnd_class_name.as_ptr()),
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
        )?
    };

    log!(Debug, "Window initialized!");

    Ok(hwnd)
}
