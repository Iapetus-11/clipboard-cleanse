use std::{cell::Cell, sync::Arc};

use objc2::runtime::AnyObject;
use objc2::{
    class, declare_class, extern_class, msg_send_id, mutability, rc::autoreleasepool, rc::Retained,
    sel, ClassType, DeclaredClass,
};
use objc2_foundation::{MainThreadMarker, NSObject};

use crate::{
    log,
    macos::{app_delegate::AppDelegate, appkit::NSApplication, ui},
    sanitization::sanitize,
};

use super::appkit::NSPasteboard;
use crate::Config;

extern_class!(
    #[derive(Debug)]
    struct NSTimer;

    unsafe impl ClassType for NSTimer {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
    }
);

pub struct ClipboardPollerIvars {
    config: Arc<Config>,
    last_change_count: Cell<isize>,
}

declare_class!(
    #[derive(Debug)]
    struct ClipboardPoller;

    unsafe impl ClassType for ClipboardPoller {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "ClipboardPoller";
    }

    impl DeclaredClass for ClipboardPoller {
        type Ivars = ClipboardPollerIvars;
    }

    unsafe impl ClipboardPoller {
        #[method(pollClipboard:)]
        fn poll_clipboard(&self, _timer: &NSTimer) {
            autoreleasepool(|_| {
                let pasteboard = NSPasteboard::get_general_pasteboard();

                let change_count = pasteboard.get_change_count();
                if self.ivars().last_change_count.get() == change_count {
                    return;
                }
                self.ivars().last_change_count.set(change_count);

                let Some(contents) = pasteboard.get_text() else {
                    return;
                };

                let sanitized_contents = sanitize(&contents);

                if contents != sanitized_contents {
                    pasteboard.set_text(&sanitized_contents);
                    self.ivars()
                        .last_change_count
                        .set(self.ivars().last_change_count.get() + 1);

                    log!(Info, "Sanitized copied text!");
                }
            });
        }
    }
);

impl ClipboardPoller {
    pub fn new(mtm: MainThreadMarker, config: Arc<Config>) -> Retained<Self> {
        let this = MainThreadMarker::alloc::<ClipboardPoller>(mtm);
        let this = this.set_ivars(ClipboardPollerIvars {
            config,
            last_change_count: Cell::new(-1),
        });

        unsafe { msg_send_id![super(this), init] }
    }

    pub fn schedule(&self) -> Retained<NSTimer> {
        let interval_secs = self.ivars().config.macos.poll_interval_ms as f64 / 1000.0;

        unsafe {
            msg_send_id![
                class!(NSTimer),
                scheduledTimerWithTimeInterval: interval_secs,
                target: self,
                selector: sel!(pollClipboard:),
                userInfo: std::ptr::null_mut::<AnyObject>(),
                repeats: true,
            ]
        }
    }
}

pub fn main(config: Config) {
    log!(Debug, "Initializing app...");

    let config = Arc::new(config);

    let mtm = MainThreadMarker::new().unwrap();
    let app = NSApplication::get_shared();

    let delegate = AppDelegate::new(
        mtm,
        Box::new(move || {
            log!(Info, "Application launched!");
        }),
    );
    app.set_delegate(&delegate);

    let poller = ClipboardPoller::new(mtm, config.clone());
    let _poll_timer = poller.schedule();

    let status_bar_item = ui::setup_status_bar_item();
    let _status_bar_item_menu = ui::setup_menu(&mtm, &status_bar_item, config.clone());

    log!(Debug, "Running app...");

    app.run();
}
