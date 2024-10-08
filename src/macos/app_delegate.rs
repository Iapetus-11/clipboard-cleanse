use objc2::{declare_class, msg_send_id, mutability, rc::Retained, ClassType, DeclaredClass};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol};

pub struct AppDelegateIvars {
    on_launched: Box<dyn Fn()>,
}

declare_class!(
    #[derive(Debug)]
    pub struct AppDelegate;

    unsafe impl ClassType for AppDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "AppDelegate";
    }

    impl DeclaredClass for AppDelegate {
        type Ivars = AppDelegateIvars;
    }

    unsafe impl AppDelegate {
        #[method(applicationDidFinishLaunching:)]
        fn did_finish_launching(&self, _notification: &NSNotification) {
            (self.ivars().on_launched)();
        }
    }

    unsafe impl NSObjectProtocol for AppDelegate {}
);

impl AppDelegate {
    pub fn new(mtm: MainThreadMarker, on_launched: Box<dyn Fn()>) -> Retained<Self> {
        let this = MainThreadMarker::alloc::<AppDelegate>(mtm);
        let this = this.set_ivars(AppDelegateIvars { on_launched });

        unsafe { msg_send_id![super(this), init] }
    }
}
