use std::{env, fs, path::PathBuf, process, sync::LazyLock};

static BUILD_PROFILE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env::var("OUT_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
});

fn main() {
    // Link libraries/frameworks required for MacOS clipboard stuff
    if cfg!(target_os = "macos") {
        println!("cargo::rustc-link-arg=-lobjc");
        println!("cargo::rustc-link-arg=-framework");
        println!("cargo::rustc-link-arg=AppKit");
        println!("cargo::rustc-link-lib=framework=ServiceManagement");
    }

    // Compile & link resource file for Windows
    if cfg!(target_os = "windows") {
        println!("cargo::rerun-if-changed=bundling\\windows\\resources.rc");
        println!("cargo::rerun-if-changed=resources\\icon.ico");

        let build_profile_bundle_path = BUILD_PROFILE_DIR.join("bundle");
        fs::create_dir_all(&build_profile_bundle_path).unwrap();

        let res_file_path = build_profile_bundle_path.join("resources.res");
        let res_file_path = res_file_path.as_os_str().to_string_lossy();

        // TODO: Rerun if resources.rc or icon.ico changes

        let windres_cmd = process::Command::new("windres")
            .args([
                "bundling\\windows\\resources.rc",
                "-O",
                "coff",
                "-o",
                &res_file_path,
            ])
            .output()
            .unwrap();

        if !windres_cmd.status.success() {
            panic!(
                "Failed to build resources.rc:\n{}\n{}",
                String::from_utf8_lossy(&windres_cmd.stdout),
                String::from_utf8_lossy(&windres_cmd.stderr)
            )
        }

        println!("cargo::rustc-link-arg={res_file_path}");
    }
}
