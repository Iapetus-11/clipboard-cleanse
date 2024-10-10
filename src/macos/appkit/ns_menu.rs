use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{
    class, declare_class, extern_class, msg_send, msg_send_id, mutability, sel, ClassType,
    DeclaredClass,
};
use objc2_foundation::{MainThreadMarker, NSString};

extern_class!(
    #[derive(Debug)]
    pub struct NSMenu;

    unsafe impl ClassType for NSMenu {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
    }
);

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
    #[allow(clippy::type_complexity)]
    action: Option<Box<dyn Fn(&NSMenuItem)>>,
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
                action(self);
            }
        }
    }
);

#[allow(dead_code)]
impl NSMenuItem {
    pub fn init_with_action(
        mtm: &MainThreadMarker,
        title: &str,
        action: Box<dyn Fn(&Self)>,
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

    pub fn set_title(&self, title: &str) {
        unsafe { msg_send![self, setTitle:&*NSString::from_str(title)] }
    }

    pub fn get_enabled(&self) -> bool {
        unsafe { msg_send![self, enabled] }
    }

    pub fn set_enabled(&self, is_enabled: bool) {
        unsafe { msg_send![self, setEnabled:is_enabled] }
    }

    pub fn get_badge(&self) -> Retained<NSMenuItemBadge> {
        unsafe { msg_send_id![self, badge] }
    }

    pub fn set_badge(&self, badge: &NSMenuItemBadge) {
        unsafe { msg_send![self, setBadge:badge] }
    }
}

extern_class!(
    #[derive(Debug)]
    pub struct NSMenuItemBadge;

    unsafe impl ClassType for NSMenuItemBadge {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
    }
);

#[allow(dead_code)]
impl NSMenuItemBadge {
    pub fn init_with_count(mtm: &MainThreadMarker, count: isize) -> Retained<Self> {
        let this = mtm.alloc::<Self>();
        unsafe { msg_send_id![this, initWithCount:count] }
    }

    pub fn init_with_string(mtm: &MainThreadMarker, string: &str) -> Retained<Self> {
        let this = mtm.alloc::<Self>();
        unsafe { msg_send_id![this, initWithString:&*NSString::from_str(string)] }
    }

    pub fn get_count(&self) -> isize {
        unsafe { msg_send![self, itemCount] }
    }

    pub fn set_count(&self, count: isize) {
        unsafe { msg_send![self, setItemCount:count] }
    }

    pub fn get_string(&self) -> String {
        let string: Retained<NSString> = unsafe { msg_send_id![self, stringValue] };

        string.to_string()
    }

    pub fn set_string(&self, string: &str) {
        unsafe { msg_send![self, setStringValue:&*NSString::from_str(string)] }
    }
}
