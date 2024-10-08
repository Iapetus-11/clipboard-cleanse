use std::{
    process,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use objc2_foundation::MainThreadMarker;

use crate::{
    config::load_and_ensure_config,
    macos::{
        app_delegate::AppDelegate,
        appkit::{NSApplication, NSButtonMethods, NSMenu, NSMenuItem, NSStatusBar},
    },
    sanitization::sanitize,
};

use super::{
    appkit::{NSData, NSImage, NSImageResizingMode, NSPasteboard, NSSize},
    Config,
};

fn poll_and_sanitize_clipboard(config: Arc<RwLock<Config>>) {
    thread::spawn(move || {
        let mut last_change_count = -1_isize;

        let pasteboard = NSPasteboard::get_general_pasteboard();

        loop {
            let sleep_duration = {
                let config = config.read().unwrap();
                Duration::from_millis(config.poll_interval_ms)
            };

            thread::sleep(sleep_duration);

            let change_count = pasteboard.get_change_count();
            if last_change_count == change_count {
                continue;
            }
            last_change_count = change_count;

            let contents = pasteboard.get_text();
            if contents.is_none() {
                continue;
            }
            let contents = contents.unwrap();

            let sanitized_contents = sanitize(&contents);

            if contents != sanitized_contents {
                pasteboard.set_text(&sanitized_contents);
                last_change_count += 1;
            }
        }
    });
}

pub fn main(config: Config) {
    let config = Arc::new(RwLock::new(config));

    let mtm = MainThreadMarker::new().unwrap();
    let app = NSApplication::get_shared();

    app.set_delegate(&AppDelegate::new(mtm, {
        let config = config.clone();
        Box::new(move || {
            poll_and_sanitize_clipboard(config.clone());
        })
    }));

    let system_status_bar = NSStatusBar::get_system_status_bar();

    let status_item = system_status_bar.new_status_item(16.0_f64);

    let status_item_button = status_item.get_button();

    let status_item_button_image = NSImage::init_with_data(&NSData::init_with_bytes(
        include_bytes!("../../resources/logo.png"),
    ));
    status_item_button_image.set_resizing_mode(NSImageResizingMode::Stretch);
    status_item_button_image.set_size(&NSSize::new(16.0, 16.0));

    status_item_button.set_image(&status_item_button_image);

    let status_item_menu = NSMenu::init(&mtm, "Clipboard Cleanse");

    status_item_menu.set_auto_enables_items(true);
    status_item_menu.add_item(&NSMenuItem::init_section_header("Clipboard Cleanse"));
    status_item_menu.add_item(&NSMenuItem::init_with_action(
        &mtm,
        "Open Config File",
        {
            let config_path = config.read().unwrap().config_path.clone();
            Box::new(move || {
                process::Command::new("open")
                    .args([&config_path])
                    .output()
                    .expect("Expected successful opening of config file");
            })
        },
        ",",
    ));
    status_item_menu.add_item(&NSMenuItem::init_with_action(
        &mtm,
        "Reload Config",
        {
            let config = config.clone();
            Box::new(move || {
                let mut config = config.write().unwrap();
                *config = load_and_ensure_config();
            })
        },
        "r",
    ));
    status_item_menu.add_item(&NSMenuItem::init_with_action(
        &mtm,
        "Quit",
        Box::new(|| NSApplication::get_shared().terminate()),
        "q",
    ));

    status_item.set_menu(&status_item_menu);

    app.run();
}
