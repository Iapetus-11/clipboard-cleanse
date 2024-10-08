use objc2::rc::Retained;
use objc2::{class, extern_class, msg_send_id, ClassType};
use objc2::{msg_send, mutability};
use objc2_foundation::NSObject;

use super::ns_button::NSButtonMethods;
use super::{NSButton, NSMenu};

extern_class!(
    #[derive(Debug)]
    pub struct NSStatusBar;

    unsafe impl ClassType for NSStatusBar {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
    }
);

impl NSStatusBar {
    pub fn get_system_status_bar() -> Retained<Self> {
        unsafe { msg_send_id![class!(NSStatusBar), systemStatusBar] }
    }

    pub fn new_status_item(&self, length: f64) -> Retained<NSStatusItem> {
        unsafe { msg_send_id![self, statusItemWithLength:length] }
    }
}

extern_class!(
    #[derive(Debug)]
    pub struct NSStatusItem;

    unsafe impl ClassType for NSStatusItem {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
    }
);

impl NSStatusItem {
    pub fn get_button(&self) -> Retained<NSStatusBarButton> {
        unsafe { msg_send_id![self, button] }
    }

    pub fn set_menu(&self, menu: &NSMenu) {
        unsafe { msg_send![self, setMenu:menu] }
    }
}

extern_class!(
    #[derive(Debug)]
    pub struct NSStatusBarButton;

    unsafe impl ClassType for NSStatusBarButton {
        type Super = NSButton;
        type Mutability = mutability::MainThreadOnly;
    }
);

impl NSButtonMethods for NSStatusBarButton {}

impl NSStatusBarButton {}
