use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        CreateWindowExW, DestroyWindow, RegisterClassW, CW_USEDEFAULT, WINDOW_EX_STYLE, WNDCLASSW,
        WS_BORDER, WS_OVERLAPPEDWINDOW,
    },
};

use crate::{log, windows::win_utils::str_to_pcwstr};

pub type WindowProc = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;

pub fn init_window(wnd_proc: WindowProc) -> windows::core::Result<HWND> {
    log!(Debug, "Initializing window...");

    let wnd_class_name = str_to_pcwstr("ClipboardCleanseWindow");

    let hwnd = unsafe {
        let h_instance: HINSTANCE = GetModuleHandleW(None)?.into();

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: h_instance,
            lpszClassName: wnd_class_name,
            ..Default::default()
        };

        RegisterClassW(&wnd_class);

        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            wnd_class_name,
            str_to_pcwstr("Clipboard Cleanse"),
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

    log!(Debug, "Window initialized");

    Ok(hwnd)
}

pub fn destroy_window(hwnd: HWND) -> windows::core::Result<()> {
    unsafe { DestroyWindow(hwnd) }
}
