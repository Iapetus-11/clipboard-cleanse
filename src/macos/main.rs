use std::{sync::Arc, thread, time::Duration};

use objc2_foundation::MainThreadMarker;

use crate::{
    log,
    macos::{app_delegate::AppDelegate, appkit::NSApplication, ui},
    sanitization::sanitize,
};

use super::appkit::NSPasteboard;
use crate::Config;

fn poll_and_sanitize_clipboard(config: Arc<Config>) {
    thread::spawn(move || {
        let sleep_duration = Duration::from_millis(config.macos.poll_interval_ms);

        let mut last_change_count = -1_isize;

        let pasteboard = NSPasteboard::get_general_pasteboard();

        loop {
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

                log!(Info, "Sanitized copied text!");
            }
        }
    });
}

pub fn main(config: Config) {
    log!(Debug, "Initializing app...");

    let config = Arc::new(config);

    let mtm = MainThreadMarker::new().unwrap();
    let app = NSApplication::get_shared();

    app.set_delegate(&AppDelegate::new(mtm, {
        let config = config.clone();

        Box::new(move || {
            log!(Info, "Application launched!");
            poll_and_sanitize_clipboard(config.clone());
        })
    }));

    let status_bar_item = ui::setup_status_bar_item();
    let _status_bar_item_menu = ui::setup_menu(&mtm, &status_bar_item, config.clone());

    log!(Debug, "Running app...");

    app.run();
}
