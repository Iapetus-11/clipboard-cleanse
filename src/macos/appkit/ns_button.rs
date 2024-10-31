use objc2::{extern_class, msg_send, mutability, ClassType};
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

pub trait NSButtonMethods: ClassType {
    fn set_image(&self, image: &NSImage) {
        unsafe { msg_send![self, setImage:image] }
    }
}
