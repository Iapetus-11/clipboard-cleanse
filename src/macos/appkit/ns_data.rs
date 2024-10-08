use std::ffi::c_void;

use objc2::{extern_class, msg_send_id, mutability, rc::Retained, ClassType};
use objc2_foundation::NSObject;

extern_class!(
    #[derive(Debug)]
    pub struct NSData;

    unsafe impl ClassType for NSData {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

impl NSData {
    pub fn init_with_bytes<const N: usize>(data: &[u8; N]) -> Retained<Self> {
        let data = data.as_ptr() as *mut c_void;
        unsafe { msg_send_id![Self::alloc(), initWithBytes:data, length:N] }
    }
}
