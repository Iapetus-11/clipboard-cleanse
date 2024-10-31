use objc2::{extern_class, msg_send_id, mutability, rc::Retained, ClassType};
use objc2_foundation::{NSObject, NSString};

extern_class!(
    #[derive(Debug)]
    pub struct NSURL;

    unsafe impl ClassType for NSURL {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

impl NSURL {
    pub fn get_absolute_string(&self) -> String {
        let ns_str: Retained<NSString> = unsafe { msg_send_id![self, absoluteString] };

        ns_str.to_string()
    }
}
