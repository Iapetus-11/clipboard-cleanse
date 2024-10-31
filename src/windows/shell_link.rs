use windows::{
    core::Interface,
    Win32::{
        System::Com::{
            CoCreateInstance, CoInitialize, CoUninitialize, IPersistFile, CLSCTX_INPROC_SERVER,
        },
        UI::Shell::{IShellLinkW, ShellLink},
    },
};

use crate::windows::win_utils::str_as_pcwstr;

pub fn create_shortcut(target: &str, output: &str) -> windows::core::Result<()> {
    unsafe {
        let com_init_result = CoInitialize(None);
        if com_init_result.is_err() {
            return Err(com_init_result.into());
        }

        let shell_link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;

        shell_link.SetPath(str_as_pcwstr(target).value)?;

        let persist_file: IPersistFile = shell_link.cast()?;

        persist_file.Save(str_as_pcwstr(output).value, true)?;

        CoUninitialize();
    }

    Ok(())
}
