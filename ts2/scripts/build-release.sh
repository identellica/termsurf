#!/bin/bash
# Build WezTerm with CEF in Release mode
#
# Usage:
#   ./scripts/build-release.sh [--clean] [--open]
#
# Flags:
#   --clean  Clear build caches and do a fresh build
#   --open   Open the app after building

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
CEF_RS_DIR="$(dirname "$REPO_DIR")/cef-rs"
CEF_OSR_APP="$CEF_RS_DIR/cef-osr.app"
BUNDLE_DIR="$REPO_DIR/target/release/WezTerm.app"

# Parse flags
CLEAN=false
OPEN=false
for arg in "$@"; do
    case $arg in
        --clean) CLEAN=true ;;
        --open) OPEN=true ;;
    esac
done

# Check prerequisites
if [[ ! -d "$CEF_OSR_APP" ]]; then
    echo "ERROR: cef-osr.app not found at $CEF_OSR_APP"
    echo "Build it first:"
    echo "  cd $CEF_RS_DIR && cargo build -p cef-osr && cargo run -p bundle-cef-app -- cef-osr -o cef-osr.app"
    exit 1
fi

# Clean if requested
if [ "$CLEAN" = true ]; then
    echo "=== Cleaning build caches ==="
    rm -rf "$REPO_DIR/target/release"
    echo "Cleared target/release"
fi

echo "=== Building WezTerm with CEF (Release) ==="

# 1. Build release binaries
echo "Building release binaries..."
cd "$REPO_DIR"
cargo build -p wezterm-gui --features cef --release

# 2. Remove existing bundle and copy template
echo "Creating bundle from template..."
rm -rf "$BUNDLE_DIR"
cp -R "$REPO_DIR/assets/macos/WezTerm.app" "$BUNDLE_DIR"

# 3. Create directories
mkdir -p "$BUNDLE_DIR/Contents/MacOS"
mkdir -p "$BUNDLE_DIR/Contents/Frameworks"

# 4. Move ANGLE dylibs if present
if [[ -f "$BUNDLE_DIR/libEGL.dylib" ]]; then
    mv "$BUNDLE_DIR/libEGL.dylib" "$BUNDLE_DIR/Contents/Frameworks/"
    mv "$BUNDLE_DIR/libGLESv1_CM.dylib" "$BUNDLE_DIR/Contents/Frameworks/"
    mv "$BUNDLE_DIR/libGLESv2.dylib" "$BUNDLE_DIR/Contents/Frameworks/"
fi

# 5. Copy main executable
echo "Copying main executable..."
cp "$REPO_DIR/target/release/wezterm-gui" "$BUNDLE_DIR/Contents/MacOS/"

# 6. Copy CEF framework
echo "Copying CEF framework (~200MB, this takes a moment)..."
cp -R "$CEF_OSR_APP/Contents/Frameworks/Chromium Embedded Framework.framework" "$BUNDLE_DIR/Contents/Frameworks/"

# 7. Create helper bundles
echo "Creating helper bundles..."
CEF_OSR_FRAMEWORKS="$CEF_OSR_APP/Contents/Frameworks"
for suffix in "Helper" "Helper (GPU)" "Helper (Renderer)" "Helper (Plugin)" "Helper (Alerts)"; do
    SRC_BUNDLE="${CEF_OSR_FRAMEWORKS}/cef-osr ${suffix}.app"
    DEST_BUNDLE="$BUNDLE_DIR/Contents/Frameworks/WezTerm ${suffix}.app"

    cp -R "${SRC_BUNDLE}" "${DEST_BUNDLE}"
    mv "${DEST_BUNDLE}/Contents/MacOS/cef-osr ${suffix}" "${DEST_BUNDLE}/Contents/MacOS/WezTerm ${suffix}"
    cp "$REPO_DIR/target/release/wezterm-cef-helper" "${DEST_BUNDLE}/Contents/MacOS/WezTerm ${suffix}"
    sed -i '' 's/cef-osr/WezTerm/g' "${DEST_BUNDLE}/Contents/Info.plist"
    sed -i '' 's/apps.tauri.cef-rs.WezTerm/com.github.wez.wezterm.helper/g' "${DEST_BUNDLE}/Contents/Info.plist"

    echo "  Created: WezTerm ${suffix}.app"
done

# 8. Update Info.plist
echo "Updating Info.plist..."
python3 << 'PYTHON_SCRIPT'
import plistlib

plist_path = "target/release/WezTerm.app/Contents/Info.plist"

with open(plist_path, 'rb') as f:
    plist = plistlib.load(f)

if 'LSEnvironment' not in plist:
    plist['LSEnvironment'] = {}
if 'MallocNanoZone' not in plist['LSEnvironment']:
    plist['LSEnvironment']['MallocNanoZone'] = '0'

with open(plist_path, 'wb') as f:
    plistlib.dump(plist, f)

print("  Added MallocNanoZone=0 to LSEnvironment")
PYTHON_SCRIPT

# 9. Sign the bundle
echo "Signing bundle..."
codesign --sign - --force --deep "$BUNDLE_DIR"

echo ""
echo "=== Release Build Complete ==="
echo "App location: $BUNDLE_DIR"
echo ""
echo "To run:"
echo "  $BUNDLE_DIR/Contents/MacOS/wezterm-gui"

# Open if requested
if [ "$OPEN" = true ]; then
    echo ""
    echo "Opening WezTerm..."
    open "$BUNDLE_DIR"
fi
