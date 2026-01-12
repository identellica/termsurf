#!/bin/bash
# Generate app icon assets from source images
# Usage: ./scripts/generate-icons.sh
#
# This script generates all icon sizes for the macOS app from source images:
# - Production icon: termsurf-macos/icon-source/termsurf-icon.png
# - Debug icon: termsurf-macos/icon-source/termsurf-debug-icon.png

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
PROJECT_DIR="$REPO_ROOT/termsurf-macos"

PROD_SOURCE="$PROJECT_DIR/icon-source/termsurf-icon.png"
DEBUG_SOURCE="$PROJECT_DIR/icon-source/termsurf-debug-icon.png"
APPICONSET="$PROJECT_DIR/Assets.xcassets/AppIcon.appiconset"
DEBUG_IMAGESET="$PROJECT_DIR/Assets.xcassets/TermSurfDebugIcon.imageset"

# Check source files exist
if [ ! -f "$PROD_SOURCE" ]; then
    echo "Error: Production icon not found: $PROD_SOURCE"
    exit 1
fi

if [ ! -f "$DEBUG_SOURCE" ]; then
    echo "Error: Debug icon not found: $DEBUG_SOURCE"
    exit 1
fi

# Generate production icon sizes
echo "Generating production icon sizes..."
mkdir -p "$APPICONSET"

for size in 16 32 64 128 256 512 1024; do
    echo "  Creating icon-${size}.png"
    sips -z $size $size "$PROD_SOURCE" --out "$APPICONSET/icon-${size}.png" 2>/dev/null
done

# Copy debug icon
echo "Copying debug icon..."
mkdir -p "$DEBUG_IMAGESET"
cp "$DEBUG_SOURCE" "$DEBUG_IMAGESET/termsurf-debug-icon.png"

echo ""
echo "Done! Rebuild the app to see the new icons."
echo ""
echo "Note: For best quality, source icons should be at least 1024x1024 pixels."
