use objc2::{
    class, extern_class, msg_send, msg_send_id, mutability, rc::Retained, runtime::ProtocolObject,
    ClassType, Message,
};
use objc2_foundation::{NSObject, NSObjectProtocol};

extern_class!(
    #[derive(Debug)]
    pub struct NSApplication;

    unsafe impl ClassType for NSApplication {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
    }
);

impl NSApplication {
    pub fn get_shared() -> Retained<Self> {
        unsafe { msg_send_id![class!(NSApplication), sharedApplication] }
    }

    pub fn set_delegate<T>(&self, delegate: &Retained<T>)
    where
        T: ?Sized + Message + NSObjectProtocol,
    {
        let delegate: &ProtocolObject<dyn NSObjectProtocol> = ProtocolObject::from_ref(&**delegate);

        unsafe { msg_send![self, setDelegate:delegate] }
    }

    pub fn run(&self) {
        unsafe { msg_send![self, run] }
    }

    pub fn terminate(&self) {
        unsafe { msg_send![self, terminate:self] }
    }
}
