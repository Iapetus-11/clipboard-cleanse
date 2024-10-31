use objc2::{extern_class, msg_send, msg_send_id, mutability, rc::Retained, ClassType};
use objc2_foundation::NSObject;

use super::{NSData, NSSize};

extern_class!(
    #[derive(Debug)]
    pub struct NSImage;

    unsafe impl ClassType for NSImage {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

#[allow(dead_code)]
pub enum NSImageResizingMode {
    Tile = 0,
    Stretch = 1,
}

impl NSImage {
    pub fn init_with_data(data: &NSData) -> Retained<Self> {
        unsafe { msg_send_id![Self::alloc(), initWithData:data] }
    }

    pub fn set_resizing_mode(&self, mode: NSImageResizingMode) {
        unsafe { msg_send![self, setResizingMode:(mode as i64)] }
    }

    pub fn set_size(&self, size: &NSSize) {
        unsafe { msg_send![self, setSize:*size] }
    }
}
