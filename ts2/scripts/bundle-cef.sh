#!/bin/bash
# Bundle WezTerm with CEF support for macOS
# This script creates a signed WezTerm.app bundle with CEF framework and helper apps

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
CEF_RS_DIR="$(dirname "$REPO_DIR")/cef-rs"

# Configuration
BUNDLE_DIR="$REPO_DIR/target/release/WezTerm.app"
CEF_OSR_APP="$CEF_RS_DIR/cef-osr.app"

echo "=== Building WezTerm with CEF support ==="

# Check prerequisites
if [[ ! -d "$CEF_OSR_APP" ]]; then
    echo "ERROR: cef-osr.app not found at $CEF_OSR_APP"
    echo "Build it first:"
    echo "  cd $CEF_RS_DIR && cargo build -p cef-osr && cargo run -p bundle-cef-app -- cef-osr -o cef-osr.app"
    exit 1
fi

# 1. Build release binaries
echo "Building release binaries..."
cd "$REPO_DIR"
cargo build -p wezterm-gui --features cef --release

# 2. Remove existing bundle and copy template
echo "Creating bundle from template..."
rm -rf "$BUNDLE_DIR"
cp -R "$REPO_DIR/assets/macos/WezTerm.app" "$BUNDLE_DIR"

# 3. Create missing directories
mkdir -p "$BUNDLE_DIR/Contents/MacOS"
mkdir -p "$BUNDLE_DIR/Contents/Frameworks"

# 4. Move ANGLE dylibs from bundle root to Frameworks (fixes code signing)
if [[ -f "$BUNDLE_DIR/libEGL.dylib" ]]; then
    mv "$BUNDLE_DIR/libEGL.dylib" "$BUNDLE_DIR/Contents/Frameworks/"
    mv "$BUNDLE_DIR/libGLESv1_CM.dylib" "$BUNDLE_DIR/Contents/Frameworks/"
    mv "$BUNDLE_DIR/libGLESv2.dylib" "$BUNDLE_DIR/Contents/Frameworks/"
fi

# 5. Copy main executable
echo "Copying main executable..."
cp "$REPO_DIR/target/release/wezterm-gui" "$BUNDLE_DIR/Contents/MacOS/"

# 6. Copy CEF framework from cef-osr
echo "Copying CEF framework (~200MB, this takes a moment)..."
cp -R "$CEF_OSR_APP/Contents/Frameworks/Chromium Embedded Framework.framework" "$BUNDLE_DIR/Contents/Frameworks/"

# 7. Create helper bundles by copying from cef-osr and modifying
echo "Creating helper bundles..."
CEF_OSR_FRAMEWORKS="$CEF_OSR_APP/Contents/Frameworks"
for suffix in "Helper" "Helper (GPU)" "Helper (Renderer)" "Helper (Plugin)" "Helper (Alerts)"; do
    SRC_BUNDLE="${CEF_OSR_FRAMEWORKS}/cef-osr ${suffix}.app"
    DEST_BUNDLE="$BUNDLE_DIR/Contents/Frameworks/WezTerm ${suffix}.app"

    # Copy entire helper bundle structure from cef-osr
    cp -R "${SRC_BUNDLE}" "${DEST_BUNDLE}"

    # Rename the executable
    mv "${DEST_BUNDLE}/Contents/MacOS/cef-osr ${suffix}" "${DEST_BUNDLE}/Contents/MacOS/WezTerm ${suffix}"

    # Replace with our helper binary
    cp "$REPO_DIR/target/release/wezterm-cef-helper" "${DEST_BUNDLE}/Contents/MacOS/WezTerm ${suffix}"

    # Update Info.plist: replace "cef-osr" with "WezTerm" and update bundle identifier
    sed -i '' 's/cef-osr/WezTerm/g' "${DEST_BUNDLE}/Contents/Info.plist"
    sed -i '' 's/apps.tauri.cef-rs.WezTerm/com.github.wez.wezterm.helper/g' "${DEST_BUNDLE}/Contents/Info.plist"

    echo "  Created: WezTerm ${suffix}.app"
done

# 8. Add MallocNanoZone to main app Info.plist (required for CEF on macOS)
# Use Python for reliable plist modification instead of sed
echo "Updating Info.plist..."
python3 << 'PYTHON_SCRIPT'
import plistlib
import sys

plist_path = sys.argv[1] if len(sys.argv) > 1 else "target/release/WezTerm.app/Contents/Info.plist"

with open(plist_path, 'rb') as f:
    plist = plistlib.load(f)

# Add LSEnvironment with MallocNanoZone=0 if not present
if 'LSEnvironment' not in plist:
    plist['LSEnvironment'] = {}
if 'MallocNanoZone' not in plist['LSEnvironment']:
    plist['LSEnvironment']['MallocNanoZone'] = '0'

with open(plist_path, 'wb') as f:
    plistlib.dump(plist, f)

print("  Added MallocNanoZone=0 to LSEnvironment")
PYTHON_SCRIPT

# 9. Sign the bundle (required for macOS to allow execution)
echo "Signing bundle..."
codesign --force --deep --sign - "$BUNDLE_DIR"

echo ""
echo "=== Bundle created successfully ==="
echo "Location: $BUNDLE_DIR"
echo ""
echo "To run:"
echo "  $BUNDLE_DIR/Contents/MacOS/wezterm-gui"
