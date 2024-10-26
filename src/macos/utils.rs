use std::ffi::{c_char, CStr};

use objc2::msg_send;
use objc2_foundation::{NSString, NSUTF8StringEncoding};

pub fn nsstring_to_string(nsstring: *mut NSString) -> Option<String> {
    unsafe {
        let string_size: usize = {
            let mut s = msg_send![nsstring, lengthOfBytesUsingEncoding: NSUTF8StringEncoding];
            s += 1;
            s
        };

        let mut chars = vec![0_u8; string_size];
        let chars_ptr = chars.as_mut_ptr();

        let successful_copy: bool = msg_send![nsstring, getCString:chars_ptr as *mut c_char maxLength:string_size encoding:NSUTF8StringEncoding];

        if successful_copy {
            let result = Some(
                CStr::from_ptr(chars_ptr as *const i8)
                    .to_string_lossy()
                    .into_owned(),
            );

            result
        } else {
            None
        }
    }
}
