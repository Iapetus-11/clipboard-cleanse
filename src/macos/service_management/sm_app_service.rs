use objc2::{class, extern_class, msg_send, msg_send_id, mutability, rc::Retained, ClassType};
use objc2_foundation::NSObject;

use crate::macos::appkit::NSError;

extern_class!(
    #[derive(Debug)]
    pub struct SMAppService;

    unsafe impl ClassType for SMAppService {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

#[derive(Debug, PartialEq)]
pub enum SMAppServiceStatus {
    NotRegistered = 0,
    Enabled = 1,
    RequiresApproval = 2,
    ErrorAndNotFound = 3,
}

impl TryFrom<isize> for SMAppServiceStatus {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotRegistered),
            1 => Ok(Self::Enabled),
            2 => Ok(Self::RequiresApproval),
            3 => Ok(Self::ErrorAndNotFound),
            value => Err(format!("Expected value between 0-3, bot got {value}")),
        }
    }
}

impl SMAppService {
    pub fn open_system_settings_to_login_items() {
        unsafe { msg_send![class!(SMAppService), openSystemSettingsLoginItems] }
    }

    pub fn get_main_app_service() -> Retained<Self> {
        unsafe { msg_send_id![class!(SMAppService), mainAppService] }
    }

    pub fn get_status(&self) -> Result<SMAppServiceStatus, String> {
        let value: isize = unsafe { msg_send![self, status] };
        SMAppServiceStatus::try_from(value)
    }

    pub fn register_and_return_error(&self) -> (bool, Option<Retained<NSError>>) {
        let mut ns_error: Option<Retained<NSError>> = None;

        let success: bool = unsafe { msg_send![self, registerAndReturnError:&mut ns_error] };

        (success, ns_error)
    }

    pub fn unregister_and_return_error(&self) -> (bool, Option<Retained<NSError>>) {
        let mut ns_error: Option<Retained<NSError>> = None;

        let success: bool = unsafe { msg_send![self, unregisterAndReturnError:&mut ns_error] };

        (success, ns_error)
    }
}
