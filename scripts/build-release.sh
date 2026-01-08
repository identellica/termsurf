#!/bin/bash
# Build TermSurf in Release mode (both zig and Swift)
#
# Usage:
#   ./scripts/build-release.sh [--clean] [--open]
#
# Flags:
#   --clean  Clear all caches and do a fresh build
#   --open   Open the app after building

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
MACOS_DIR="$ROOT_DIR/termsurf-macos"

# Parse flags
CLEAN=false
OPEN=false
for arg in "$@"; do
    case $arg in
        --clean) CLEAN=true ;;
        --open) OPEN=true ;;
    esac
done

# Clean caches if requested
if [ "$CLEAN" = true ]; then
    echo "=== Cleaning build caches ==="
    rm -rf "$ROOT_DIR/zig-out" "$ROOT_DIR/zig-cache" "$ROOT_DIR/.zig-cache"
    echo "Cleared Zig cache"
    rm -rf ~/Library/Developer/Xcode/DerivedData/TermSurf-*
    echo "Cleared Xcode DerivedData"
fi

echo "=== Building libghostty (Release) ==="
cd "$ROOT_DIR"
zig build -Doptimize=ReleaseFast

echo ""
echo "=== Building TermSurf.app (Release) ==="
cd "$MACOS_DIR"
if [ "$CLEAN" = true ]; then
    xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Release clean build | tail -5
else
    xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Release build | tail -5
fi

# Get the build products directory
APP_PATH=$(xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Release -showBuildSettings 2>/dev/null | grep "BUILT_PRODUCTS_DIR" | head -1 | awk '{print $3}')/TermSurf.app

echo ""
echo "=== Build Complete ==="
echo "App location: $APP_PATH"
echo ""
echo "To open the app, run:"
echo "  open \"$APP_PATH\""

# If --open flag is passed, open the app
if [ "$OPEN" = true ]; then
    echo ""
    echo "Opening TermSurf..."
    open "$APP_PATH"
fi
