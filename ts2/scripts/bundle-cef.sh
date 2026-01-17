#!/bin/bash
# Bundle wezterm-gui with CEF framework into a macOS .app bundle.
# This is required for CEF to find its resources at runtime.
#
# Usage:
#   ./scripts/bundle-cef.sh [profile]
#
# Examples:
#   ./scripts/bundle-cef.sh          # Uses 'release' profile
#   ./scripts/bundle-cef.sh debug    # Uses 'debug' profile

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
TARGET_DIR="${TARGET_DIR:-$REPO_DIR/target}"
PROFILE="${1:-release}"
APP_DIR="$TARGET_DIR/$PROFILE/WezTerm.app"

# Check that wezterm-gui was built
if [ ! -f "$TARGET_DIR/$PROFILE/wezterm-gui" ]; then
    echo "Error: wezterm-gui not found at $TARGET_DIR/$PROFILE/wezterm-gui"
    echo "Build first with: cargo build --${PROFILE} --features cef -p wezterm-gui"
    exit 1
fi

# Find CEF framework in build output
CEF_FRAMEWORK=$(find "$TARGET_DIR/$PROFILE/build" -path "*/cef-dll-sys-*/out/*/Chromium Embedded Framework.framework" -type d 2>/dev/null | head -1)

if [ -z "$CEF_FRAMEWORK" ]; then
    echo "Error: CEF framework not found in build output."
    echo "Build with CEF enabled: cargo build --${PROFILE} --features cef -p wezterm-gui"
    exit 1
fi

echo "Found CEF framework: $CEF_FRAMEWORK"

# Create bundle structure
echo "Creating app bundle at $APP_DIR"
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Frameworks"
mkdir -p "$APP_DIR/Contents/Resources"

# Copy executable
echo "Copying wezterm-gui executable"
cp "$TARGET_DIR/$PROFILE/wezterm-gui" "$APP_DIR/Contents/MacOS/"

# Copy Info.plist
echo "Copying Info.plist"
cp "$REPO_DIR/assets/macos/WezTerm.app/Contents/Info.plist" "$APP_DIR/Contents/"

# Copy icon if it exists
if [ -f "$REPO_DIR/assets/macos/WezTerm.app/Contents/Resources/terminal.icns" ]; then
    cp "$REPO_DIR/assets/macos/WezTerm.app/Contents/Resources/terminal.icns" "$APP_DIR/Contents/Resources/"
fi

# Symlink CEF framework (symlink for dev speed, deploy.sh does full copy for release)
echo "Symlinking CEF framework"
ln -s "$CEF_FRAMEWORK" "$APP_DIR/Contents/Frameworks/Chromium Embedded Framework.framework"

echo ""
echo "Bundle created successfully: $APP_DIR"
echo ""
echo "Run with:"
echo "  $APP_DIR/Contents/MacOS/wezterm-gui"
