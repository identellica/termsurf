#!/bin/bash
# Build TermSurf in Release mode (both zig and Swift)

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
MACOS_DIR="$ROOT_DIR/termsurf-macos"

echo "=== Building libghostty (Release) ==="
cd "$ROOT_DIR"
zig build -Doptimize=ReleaseFast

echo ""
echo "=== Building TermSurf.app (Release) ==="
cd "$MACOS_DIR"
xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Release build | tail -5

# Get the build products directory
APP_PATH=$(xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Release -showBuildSettings 2>/dev/null | grep "BUILT_PRODUCTS_DIR" | head -1 | awk '{print $3}')/TermSurf.app

echo ""
echo "=== Build Complete ==="
echo "App location: $APP_PATH"
echo ""
echo "To open the app, run:"
echo "  open \"$APP_PATH\""

# If --open flag is passed, open the app
if [[ "$1" == "--open" ]]; then
    echo ""
    echo "Opening TermSurf..."
    open "$APP_PATH"
fi
