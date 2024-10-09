# -----------------------------------------------------------------------------
# Initial Setup

set -e

PROJECT_ROOT=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )/../..
cd "$PROJECT_ROOT"

BUNDLE_DIR="$PROJECT_ROOT/target/release/bundle/osx/Clipboard Cleanse.app/Contents"

# -----------------------------------------------------------------------------
# Build Cargo Project

cargo build --release


# -----------------------------------------------------------------------------
# Change Directory to Bundle Folder

mkdir -p "$BUNDLE_DIR"
cd "$BUNDLE_DIR"


# -----------------------------------------------------------------------------
# Copy & Template-Fill Info.plist

cp "$PROJECT_ROOT/bundling/macos/Info.plist" ./Info.plist.template

export PLIST_APP_VERSION=$(grep -m 1 'version' "$PROJECT_ROOT/Cargo.toml" | sed 's/version = "\(.*\)"/\1/')
export PLIST_APP_BUNDLE_VERSION=$(date +"%Y%m%d.%H%M%S")

envsubst < Info.plist.template > Info.plist
rm Info.plist.template


# -----------------------------------------------------------------------------
# Generate App Icons Variants & Icon Set

mkdir -p Resources
cd Resources

mkdir icon.iconset

sips -Z 64 "$PROJECT_ROOT/resources/icon.png" --out icon.iconset/icon_64x64.png
sips -Z 128 "$PROJECT_ROOT/resources/icon.png" --out icon.iconset/icon_128x128.png
sips -Z 256 "$PROJECT_ROOT/resources/icon.png" --out icon.iconset/icon_256x256.png
sips -Z 512 "$PROJECT_ROOT/resources/icon.png" --out icon.iconset/icon_512x512.png

iconutil --convert icns icon.iconset

rm -r icon.iconset


# -----------------------------------------------------------------------------
# Copy Executable

cd "$BUNDLE_DIR"

mkdir -p MacOS
cd MacOS

cp "$PROJECT_ROOT/target/release/clipboard_cleanse" .


# -----------------------------------------------------------------------------
# Done!

open "$PROJECT_ROOT/target/release/bundle/osx/"