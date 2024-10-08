use objc2::{class, extern_class, msg_send_id, mutability, rc::Retained, ClassType};
use objc2_foundation::NSObject;

use super::NSURL;

extern_class!(
    #[derive(Debug)]
    pub struct NSFileManager;

    unsafe impl ClassType for NSFileManager {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

impl NSFileManager {
    pub fn get_default_manager() -> Retained<Self> {
        unsafe { msg_send_id![class!(NSFileManager), defaultManager] }
    }

    pub fn get_home_directory_for_current_user(&self) -> Retained<NSURL> {
        unsafe { msg_send_id![self, homeDirectoryForCurrentUser] }
    }
}
