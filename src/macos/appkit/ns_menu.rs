use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{
    class, declare_class, extern_class, msg_send, msg_send_id, mutability, sel, ClassType,
    DeclaredClass,
};
use objc2_foundation::{MainThreadMarker, NSCoding, NSCopying, NSObjectProtocol, NSString};

extern_class!(
    #[derive(Debug)]
    pub struct NSMenu;

    unsafe impl ClassType for NSMenu {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for NSMenu {}
unsafe impl NSCopying for NSMenu {}
unsafe impl NSCoding for NSMenu {}

impl NSMenu {
    pub fn init(mtm: &MainThreadMarker, title: &str) -> Retained<Self> {
        let ns_menu = mtm.alloc::<Self>();

        unsafe { msg_send_id![ns_menu, initWithTitle:&*NSString::from_str(title)] }
    }

    pub fn add_item(&self, item: &NSMenuItem) {
        unsafe { msg_send![self, addItem:item] }
    }

    pub fn set_auto_enables_items(&self, value: bool) {
        unsafe { msg_send![self, setAutoenablesItems:value] }
    }
}

extern_class!(
    #[derive(Debug)]
    pub struct NSMenuItemRaw;

    unsafe impl ClassType for NSMenuItemRaw {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "NSMenuItem";
    }
);

pub struct NSMenuItemIvars {
    action: Option<Box<dyn Fn()>>,
}

declare_class!(
    #[derive(Debug)]
    pub struct NSMenuItem;

    unsafe impl ClassType for NSMenuItem {
        type Super = NSMenuItemRaw;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "NSMenuItemWithCallback";
    }

    impl DeclaredClass for NSMenuItem {
        type Ivars = NSMenuItemIvars;
    }

    unsafe impl NSMenuItem {
        #[method(callback)]
        fn callback(&self) {
            if let Some(action) = &self.ivars().action {
                action();
            }
        }
    }

    unsafe impl NSObjectProtocol for NSMenuItem {}
    unsafe impl NSCopying for NSMenuItem {}
    unsafe impl NSCoding for NSMenuItem {}
);

impl NSMenuItem {
    pub fn init_with_action(
        mtm: &MainThreadMarker,
        title: &str,
        action: Box<dyn Fn()>,
        key: &str,
    ) -> Retained<Self> {
        let this = mtm.alloc::<NSMenuItem>().set_ivars(NSMenuItemIvars {
            action: Some(action),
        });

        let this: Retained<NSMenuItem> = unsafe {
            msg_send_id![
                super(this),
                initWithTitle:&*NSString::from_str(title),
                action:sel!(callback),
                keyEquivalent:&*NSString::from_str(key),
            ]
        };

        unsafe {
            let _: () = msg_send![&*this, setTarget:&*this];
        }

        this
    }

    pub fn init_section_header(title: &str) -> Retained<Self> {
        unsafe {
            msg_send_id![class!(NSMenuItem), sectionHeaderWithTitle:&*NSString::from_str(title)]
        }
    }
}
