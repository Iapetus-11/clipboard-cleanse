use std::{sync::Arc, thread, time::Duration};

use objc2_foundation::MainThreadMarker;

use crate::{
    logger::Logger,
    macos::{app_delegate::AppDelegate, appkit::NSApplication, ui},
    sanitization::sanitize,
};

use super::{appkit::NSPasteboard, Config};

fn poll_and_sanitize_clipboard(config: Arc<Config>, logger: Arc<Logger>) {
    thread::spawn(move || {
        let sleep_duration = Duration::from_millis(config.poll_interval_ms);

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

                logger.info("Sanitized copied text!");
            }
        }
    });
}

pub fn main(config: Config, logger: Logger) {
    let logger = Arc::new(logger);

    logger.debug("Initializing app...");

    let config = Arc::new(config);

    let mtm = MainThreadMarker::new().unwrap();
    let app = NSApplication::get_shared();

    app.set_delegate(&AppDelegate::new(mtm, {
        let logger = logger.clone();
        let config = config.clone();

        Box::new(move || {
            logger.info("Application launched!");
            poll_and_sanitize_clipboard(config.clone(), logger.clone());
        })
    }));

    let status_bar_item = ui::setup_status_bar_item();
    let _status_bar_item_menu =
        ui::setup_menu(&mtm, &status_bar_item, config.clone(), logger.clone());

    logger.debug("Running app...");

    app.run();
}
