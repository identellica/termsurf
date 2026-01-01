#!/bin/bash
#
# Downloads and extracts CEF (Chromium Embedded Framework) for TermSurf
#
# Usage: ./scripts/setup-cef.sh
#

set -e

# CEF version info
CEF_VERSION="143.0.13"
CEF_HASH="g30cb3bd"
CHROMIUM_VERSION="143.0.7499.170"
PLATFORM="macosarm64"

# Construct the download URL (+ characters are URL encoded as %2B)
CEF_FULL_VERSION="${CEF_VERSION}%2B${CEF_HASH}%2Bchromium-${CHROMIUM_VERSION}"
CEF_FILENAME="cef_binary_${CEF_VERSION}+${CEF_HASH}+chromium-${CHROMIUM_VERSION}_${PLATFORM}"
CEF_URL="https://cef-builds.spotifycdn.com/${CEF_FILENAME/+/%2B}.tar.bz2"
CEF_URL="${CEF_URL/+/%2B}"

# Paths
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FRAMEWORKS_DIR="$REPO_ROOT/termsurf-macos/Frameworks"
CEF_DIR="$FRAMEWORKS_DIR/cef"

echo "TermSurf CEF Setup"
echo "=================="
echo "CEF Version: $CEF_VERSION (Chromium $CHROMIUM_VERSION)"
echo ""

# Check if already downloaded
if [ -d "$CEF_DIR" ] && [ -f "$CEF_DIR/README.txt" ]; then
    echo "CEF already installed at: $CEF_DIR"
    echo "To reinstall, remove the directory and run this script again."
    exit 0
fi

# Create Frameworks directory
mkdir -p "$FRAMEWORKS_DIR"

# Download
TEMP_DIR=$(mktemp -d)
ARCHIVE="$TEMP_DIR/cef.tar.bz2"

echo "Downloading CEF (~250MB)..."
echo "URL: $CEF_URL"
echo ""

if command -v curl &> /dev/null; then
    curl -L -o "$ARCHIVE" "$CEF_URL"
elif command -v wget &> /dev/null; then
    wget -O "$ARCHIVE" "$CEF_URL"
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

echo ""
echo "Extracting..."
tar -xjf "$ARCHIVE" -C "$TEMP_DIR"

# Find the extracted directory (it has a versioned name)
EXTRACTED_DIR=$(find "$TEMP_DIR" -maxdepth 1 -type d -name "cef_binary_*" | head -1)

if [ -z "$EXTRACTED_DIR" ]; then
    echo "Error: Could not find extracted CEF directory"
    rm -rf "$TEMP_DIR"
    exit 1
fi

# Move to final location
mv "$EXTRACTED_DIR" "$CEF_DIR"

# Cleanup
rm -rf "$TEMP_DIR"

echo ""
echo "CEF installed successfully!"
echo "Location: $CEF_DIR"
echo ""
echo "You can now build TermSurf."
