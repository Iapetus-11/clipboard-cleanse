# Clipboard Cleanse on MacOS

## Installation
1. Download the latest release for MacOS from the [releases page](https://github.com/Iapetus-11/clipboard-cleanse/releases/latest)
    - Make sure to download only the zip file that with "MacOS" in the name
2. Open up your terminal and run the following commands:
```
cd ~/Downloads
unzip -o "Clipboard.Cleanse.MacOS.zip"
mv "Clipboard Cleanse.app" "/Applications/Clipboard Cleanse.app"
```
3. If MacOS says the application is invalid, you can run the following command to remove it from the quarantine:
```
xattr -l "Applications/Clipboard Cleanse.app"
```

## Building from Source
- Open your terminal and navigate to the project's root
- Run `./bundling/macos/bundle.sh`
  - This will build the app in release mode and bundle it into a MacOS .app
- To copy the app to the `/Applications` folder, add the `--copy-to-applications` flag
- To sign the app using `codesign`, set the `CODESIGN_IDENTIFIER` envvar