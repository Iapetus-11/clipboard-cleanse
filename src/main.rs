use config::load_and_ensure_config;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

mod config;
mod sanitization;

fn main() {
    println!("Hello, world!");

    let config = load_and_ensure_config();

    #[cfg(target_os = "macos")]
    macos::main(config);

    #[cfg(target_os = "windows")]
    windows::main(config);
}
