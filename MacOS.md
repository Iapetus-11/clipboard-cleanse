# Clipboard Cleanse on MacOS

## Installation
1. Download the latest release for MacOS from the [releases page](https://github.com/Iapetus-11/clipboard-cleanse/releases/latest)
    - Make sure to download only the zip file that with "MacOS" in the name
2. Open up your terminal and run the following commands:
```
cd ~/Downloads
unzip -o "Clipboard.Cleanse.MacOS.zip"
mv "Clipboard Cleanse.app" "/Applications/Clipboard Cleanse.app"
xattr -d com.apple.quarantine "/Applications/Clipboard Cleanse.app"
```

## Building from Source
- Ensure you have Rust installed https://rustup.rs/
- Open your terminal and clone the repository with `git clone https://github.com/Iapetus-11/clipboard-cleanse.git`
- Navigate to the project's root: `cd clipboard-cleanse`

### Development
- Simply use `cargo run`

### Bundling
- Run `./bundling/macos/bundle.sh` to build the app in release mode and bundle it into a MacOS .app
- To copy the app to the `/Applications` folder, use the `--copy-to-applications` flag
- To sign the app using `codesign`, set the `CODESIGN_IDENTIFIER` envvar