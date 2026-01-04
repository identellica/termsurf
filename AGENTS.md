# Agent Development Guide

A file for [guiding coding agents](https://agents.md/).

## Project Overview

TermSurf is a terminal emulator with webview support, built as a fork of Ghostty. The TermSurf-specific code lives in `termsurf-macos/` while the shared terminal core (libghostty) is in `src/`.

## Commands

### libghostty (Zig core)

- **Build:** `zig build`
- **Test (Zig):** `zig build test`
- **Test filter (Zig)**: `zig build test -Dtest-filter=<test name>`
- **Formatting (Zig)**: `zig fmt .`
- **Formatting (other)**: `prettier -w .`

### TermSurf macOS App

- **Build:** `cd termsurf-macos && xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Debug build`
- **Run:** Build in Xcode and run, or use `zig build run` for the original Ghostty app
- **Clean:** `cd termsurf-macos && xcodebuild clean`

## Directory Structure

- Shared Zig core: `src/`
- C API headers: `include/`
- Original Ghostty macOS app: `macos/`
- **TermSurf macOS app: `termsurf-macos/`**
- GTK (Linux and FreeBSD) app: `src/apprt/gtk`

### TermSurf-specific files

- Swift sources: `termsurf-macos/Sources/`
- CLI tool: `src/termsurf-cli/main.zig`
- Xcode project: `termsurf-macos/TermSurf.xcodeproj`
- **TODO.md: `TODO.md`** - Active checklist of tasks to launch (keep up to date!)
- Documentation: `docs/`
  - `docs/architecture.md` - Technical decisions and design rationale
  - `docs/console.md` - Console bridging and JavaScript API (`--js-api`)
  - `docs/keybindings.md` - Webview keyboard shortcuts and modes
  - `docs/ctrl-z.md` - ctrl+z/fg analysis (deferred, documented for future reference)
  - `docs/cef.md` - CEF integration attempt (deferred, documented for future reference)

## libghostty-vt

- Build: `zig build lib-vt`
- Build Wasm Module: `zig build lib-vt -Dtarget=wasm32-freestanding`
- Test: `zig build test-lib-vt`
- Test filter: `zig build test-lib-vt -Dtest-filter=<test name>`
- When working on libghostty-vt, do not build the full app.
- For C only changes, don't run the Zig tests. Build all the examples.

## Browser Integration

TermSurf uses WKWebView (Apple's WebKit) for browser panes, providing:
- Native Swift integration (no external dependencies)
- Console message capture (stdout/stderr bridging via socket to CLI)
- Safari Web Inspector for debugging (cmd+alt+i in browse mode)
- Session isolation via WKWebsiteDataStore
- Optional JavaScript API (`--js-api` flag) for programmatic control

CEF (Chromium) integration is deferred due to Swift-to-C marshalling challenges. See `docs/cef.md` for details.

**Key locations:**
- `termsurf-macos/Sources/Features/WebView/` - WebView implementation
- `termsurf-macos/Sources/Features/Socket/` - CLI-app socket communication
- `src/termsurf-cli/main.zig` - CLI tool
- `docs/console.md` - Console bridging and JS API documentation

## Key Files for TermSurf Development

**WebView implementation** (`termsurf-macos/Sources/Features/WebView/`):
- `WebViewOverlay.swift` - WKWebView wrapper with console capture and JS injection
- `WebViewContainer.swift` - Container with control bar, mode management
- `WebViewManager.swift` - Tracks webviews, routes console events
- `ControlBar.swift` - URL bar and mode indicator

**Socket communication** (`termsurf-macos/Sources/Features/Socket/`):
- `SocketServer.swift` - Unix domain socket server
- `SocketConnection.swift` - Client connection handling
- `CommandHandler.swift` - Request routing (open, close, etc.)
- `TermsurfProtocol.swift` - JSON protocol definitions

**Terminal integration**:
- `SurfaceView_AppKit.swift` - Keyboard handling for webview modes
- `TermsurfEnvironment.swift` - Injects TERMSURF_SOCKET and TERMSURF_PANE_ID env vars

## Icon Generation

TermSurf uses two icons: a production icon and a debug icon (shown in DEBUG builds to distinguish dev from release).

- **Source icons:**
  - Production: `termsurf-macos/icon-source/termsurf-icon.png`
  - Debug: `termsurf-macos/icon-source/termsurf-debug-icon.png`
- **Update icons:** `./scripts/generate-icons.sh`
- **Generated assets:**
  - `termsurf-macos/Assets.xcassets/AppIcon.appiconset/` (production, multiple sizes)
  - `termsurf-macos/Assets.xcassets/TermSurfDebugIcon.imageset/` (debug)

Note: Source icons should be at least 1024x1024 pixels for best quality.

## Build System Notes

- `zig build` creates `GhosttyKit.xcframework` in both `macos/` and `termsurf-macos/`
- Both Xcode projects reference their local xcframework
- Modified files: `build.zig`, `src/build/GhosttyXCFramework.zig`
