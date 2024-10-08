use objc2::{class, extern_class, msg_send, msg_send_id, mutability, rc::Retained, ClassType};
use objc2_foundation::{NSInteger, NSObject, NSString};

#[allow(dead_code)]
pub enum NSPasteboardType {
    Url,
    CollaborationMetadata,
    Color,
    FileContents,
    FileURL,
    FindPanelSearchOptions,
    Font,
    Html,
    MultipleTextSelection,
    Pdf,
    Png,
    Rtf,
    Rtfd,
    Ruler,
    Sound,
    String,
    TabularText,
    TextFinderOptions,
    Tiff,
}

pub fn pasteboard_type_to_string(pbt: &NSPasteboardType) -> String {
    match pbt {
        NSPasteboardType::Url => "public.url",
        NSPasteboardType::CollaborationMetadata => todo!(),
        NSPasteboardType::Color => "com.apple.cocoa.pasteboard.color",
        NSPasteboardType::FileContents => todo!(),
        NSPasteboardType::FileURL => "public.file-url",
        NSPasteboardType::FindPanelSearchOptions => {
            "com.apple.cocoa.pasteboard.find-panel-search-options"
        }
        NSPasteboardType::Font => "com.apple.cocoa.pasteboard.character-formatting",
        NSPasteboardType::Html => "public.html",
        NSPasteboardType::MultipleTextSelection => {
            "com.apple.cocoa.pasteboard.multiple-text-selection"
        }
        NSPasteboardType::Pdf => "com.adobe.pdf",
        NSPasteboardType::Png => "public.png",
        NSPasteboardType::Rtf => "public.rtf",
        NSPasteboardType::Rtfd => "com.apple.flat-rtfd",
        NSPasteboardType::Ruler => "com.apple.cocoa.pasteboard.paragraph-formatting",
        NSPasteboardType::Sound => "com.apple.cocoa.pasteboard.sound",
        NSPasteboardType::String => "public.utf8-plain-text",
        NSPasteboardType::TabularText => "public.utf8-tab-separated-values-text",
        NSPasteboardType::TextFinderOptions => {
            "com.apple.cocoa.pasteboard.find-panel-search-options"
        }
        NSPasteboardType::Tiff => "public.tiff",
    }
    .into()
}

extern_class!(
    #[derive(Debug)]
    pub struct NSPasteboard;

    unsafe impl ClassType for NSPasteboard {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

#[allow(dead_code)]
impl NSPasteboard {
    pub fn get_general_pasteboard() -> Retained<Self> {
        unsafe { msg_send_id![class!(NSPasteboard), generalPasteboard] }
    }

    pub fn get_change_count(&self) -> isize {
        unsafe { msg_send![self, changeCount] }
    }

    pub fn get_text(&self) -> Option<String> {
        let contents: Option<Retained<NSString>> = unsafe {
            msg_send_id![self, stringForType: &*NSString::from_str(&pasteboard_type_to_string(&NSPasteboardType::String))]
        };

        contents.map(|contents| contents.to_string())
    }

    pub fn set_text(&self, contents: &str) {
        unsafe {
            let _: NSInteger = msg_send![self, clearContents];

            let _: bool = msg_send![
                self,
                setString:&*NSString::from_str(contents),
                forType:&*NSString::from_str(&pasteboard_type_to_string(&NSPasteboardType::String)),
            ];
        }
    }
}
