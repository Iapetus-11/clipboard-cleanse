use objc2::{extern_class, msg_send, msg_send_id, mutability, rc::Retained, ClassType};
use objc2_foundation::NSObject;

use super::NSImage;

extern_class!(
    #[derive(Debug)]
    pub struct NSButton;

    unsafe impl ClassType for NSButton {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
    }
);

impl NSButtonMethods for NSButton {}

impl NSButton {}

#[allow(dead_code)]
pub trait NSButtonMethods: ClassType {
    fn get_image(&self) -> Retained<NSImage> {
        unsafe { msg_send_id![self, image] }
    }

    fn set_image(&self, image: &NSImage) {
        unsafe { msg_send![self, setImage:image] }
    }
}
