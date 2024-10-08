use objc2::{class, extern_class, msg_send_id, mutability, rc::Retained, ClassType};
use objc2_foundation::{NSObject, NSString};

extern_class!(
    #[derive(Debug)]
    pub struct NSURL;

    unsafe impl ClassType for NSURL {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

#[allow(dead_code)]
impl NSURL {
    pub fn create_from_string(value: &str) -> Option<Retained<Self>> {
        unsafe { msg_send_id![class!(NSURL), URLWithString:&*NSString::from_str(value)] }
    }

    pub fn create_from_string_relative_to(
        value: &str,
        relative_to: &NSURL,
    ) -> Option<Retained<Self>> {
        unsafe {
            msg_send_id![class!(NSURL), URLWithString:&*NSString::from_str(value), relativeToURL:relative_to]
        }
    }

    pub fn get_absolute_string(&self) -> String {
        let ns_str: Retained<NSString> = unsafe { msg_send_id![self, absoluteString] };

        ns_str.to_string()
    }
}
