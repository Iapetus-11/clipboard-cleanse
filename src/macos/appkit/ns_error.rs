use objc2::{extern_class, mutability, ClassType};
use objc2_foundation::NSObject;

extern_class!(
    #[derive(Debug)]
    pub struct NSError;

    unsafe impl ClassType for NSError {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

impl NSError {}
