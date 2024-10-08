use objc2::Encode;

// This is a hack to get around not being able to call certain functions to construct an NSSize,
// should be structurally identical to what's actually in ObjectiveC and therefore compatible
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NSSize {
    pub width: f64,
    pub height: f64,
}

impl NSSize {
    pub fn new(width: f64, height: f64) -> Self {
        NSSize { width, height }
    }
}

unsafe impl Encode for NSSize {
    const ENCODING: objc2::Encoding = objc2::Encoding::Struct(
        "CGSize",
        &[objc2::Encoding::Double, objc2::Encoding::Double],
    );
}
