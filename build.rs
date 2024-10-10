fn main() {
    // Link libraries/frameworks required for MacOS clipboard stuff
    if cfg!(target_os = "macos") {
        println!("cargo::rustc-link-arg=-lobjc");
        println!("cargo::rustc-link-arg=-framework");
        println!("cargo::rustc-link-arg=AppKit");
        println!("cargo::rustc-link-lib=framework=ServiceManagement");
    }
}
