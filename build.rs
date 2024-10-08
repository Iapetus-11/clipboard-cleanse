fn main() {
    // Link libraries required for MacOS clipboard stuff
    if cfg!(target_os = "macos") {
        println!("cargo::rustc-link-arg=-lobjc");
        println!("cargo::rustc-link-arg=-framework");
        println!("cargo::rustc-link-arg=AppKit");
    }
}
