# Agent Development Guide

A file for [guiding coding agents](https://agents.md/).

## Project Overview

TermSurf is a terminal emulator with webview support. The project is transitioning from TermSurf 1.x (Ghostty-based, macOS-only) to TermSurf 2.0 (WezTerm-based, cross-platform with CEF browser).

**Current structure:**
- `ts1/` - TermSurf 1.x (Ghostty fork + WKWebView)
- `ts2/` + root - TermSurf 2.0 (WezTerm fork)
- `cef-rs/` - CEF Rust bindings (Chromium Embedded Framework)

## TermSurf 1.x (ts1/)

### Commands

#### libghostty (Zig core)

- **Build:** `cd ts1 && zig build`
- **Test (Zig):** `cd ts1 && zig build test`
- **Test filter (Zig)**: `cd ts1 && zig build test -Dtest-filter=<test name>`
- **Formatting (Zig)**: `cd ts1 && zig fmt .`
- **Formatting (other)**: `prettier -w .`

#### TermSurf macOS App

- **Build (Debug):** `cd ts1 && ./scripts/build-debug.sh` → `ts1/build/debug/TermSurf.app`
- **Build (Release):** `cd ts1 && ./scripts/build-release.sh` → `ts1/build/release/TermSurf.app`
- **Build & Open:** Add `--open` flag to either script
- **Clean Build:** Add `--clean` flag to either script
- **Run:** Build with scripts above, or use Xcode, or `cd ts1 && zig build run` for original Ghostty

### Directory Structure

- Shared Zig core: `ts1/src/`
- C API headers: `ts1/include/`
- Original Ghostty macOS app: `ts1/macos/`
- **TermSurf macOS app: `ts1/termsurf-macos/`**
- GTK (Linux and FreeBSD) app: `ts1/src/apprt/gtk`

### TermSurf-specific files

- Swift sources: `ts1/termsurf-macos/Sources/`
- CLI web command: `ts1/src/cli/web.zig`
- Xcode project: `ts1/termsurf-macos/TermSurf.xcodeproj`
- **TODO.md: `TODO.md`** - Active checklist of tasks to launch (keep up to date!)
- Documentation: `docs/`
  - `docs/architecture.md` - Technical decisions and design rationale
  - `docs/bookmarks.md` - Bookmarks implementation plan and checklist
  - `docs/build.md` - Build instructions and troubleshooting
  - `docs/console.md` - Console bridging and JavaScript API (`--js-api`)
  - `docs/keybindings.md` - Webview keyboard shortcuts and modes
  - `docs/libghostty.md` - Changes to libghostty (tracking for upstream PRs)
  - `docs/merge-upstream.md` - How to merge changes from upstream repos (Ghostty, WezTerm, cef-rs)
  - `docs/release.md` - Release procedure and versioning
  - `docs/target-blank.md` - target="_blank" link handling
  - `docs/webview.md` - WebView implementation, API checklist, and workarounds
  - `docs/ctrl-z.md` - ctrl+z/fg analysis (deferred, documented for future reference)
  - `docs/cef.md` - CEF integration attempt (deferred, documented for future reference)
  - `docs/termsurf2.md` - TermSurf 2.0 planning (CEF + Zig approach, superseded)
  - `docs/termsurf2-wezterm-analysis.md` - TermSurf 2.0 analysis (WezTerm + cef-rs)

### libghostty-vt

- Build: `cd ts1 && zig build lib-vt`
- Build Wasm Module: `cd ts1 && zig build lib-vt -Dtarget=wasm32-freestanding`
- Test: `cd ts1 && zig build test-lib-vt`
- Test filter: `cd ts1 && zig build test-lib-vt -Dtest-filter=<test name>`
- When working on libghostty-vt, do not build the full app.
- For C only changes, don't run the Zig tests. Build all the examples.

### Browser Integration (TermSurf 1.x)

TermSurf 1.x uses WKWebView (Apple's WebKit) for browser panes, providing:
- Native Swift integration (no external dependencies)
- Console message capture (stdout/stderr bridging via socket to CLI)
- Safari Web Inspector for debugging (cmd+alt+i in browse mode)
- Session isolation via WKWebsiteDataStore
- Optional JavaScript API (`--js-api` flag) for programmatic control

**Key locations:**
- `ts1/termsurf-macos/Sources/Features/WebView/` - WebView implementation
- `ts1/termsurf-macos/Sources/Features/Socket/` - CLI-app socket communication
- `ts1/src/cli/web.zig` - CLI web command (integrated into termsurf binary)
- `docs/console.md` - Console bridging and JS API documentation
- `docs/webview.md` - WebView implementation and API checklist

### Key Files for TermSurf 1.x Development

**WebView implementation** (`ts1/termsurf-macos/Sources/Features/WebView/`):
- `WebViewOverlay.swift` - WKWebView wrapper with console capture and JS injection
- `WebViewContainer.swift` - Container with control bar, mode management
- `WebViewManager.swift` - Tracks webviews, routes console events
- `ControlBar.swift` - URL bar and mode indicator

**Socket communication** (`ts1/termsurf-macos/Sources/Features/Socket/`):
- `SocketServer.swift` - Unix domain socket server
- `SocketConnection.swift` - Client connection handling
- `CommandHandler.swift` - Request routing (open, close, etc.)
- `TermsurfProtocol.swift` - JSON protocol definitions
- `TermsurfEnvironment.swift` - Injects TERMSURF_SOCKET and TERMSURF_PANE_ID env vars

**Terminal integration** (`ts1/termsurf-macos/Sources/Ghostty/Surface View/`):
- `SurfaceView_AppKit.swift` - Keyboard handling for webview modes

### Icon Generation

TermSurf uses two icons: a production icon and a debug icon (shown in DEBUG builds to distinguish dev from release).

- **Source icons:**
  - Production: `ts1/termsurf-macos/icon-source/termsurf-icon.png`
  - Debug: `ts1/termsurf-macos/icon-source/termsurf-debug-icon.png`
- **Update icons:** `cd ts1 && ./scripts/generate-icons.sh`
- **Generated assets:**
  - `ts1/termsurf-macos/Assets.xcassets/AppIcon.appiconset/` (production, multiple sizes)
  - `ts1/termsurf-macos/Assets.xcassets/TermSurfDebugIcon.imageset/` (debug)

Note: Source icons should be at least 1024x1024 pixels for best quality.

### Build System Notes

- `cd ts1 && zig build` creates `GhosttyKit.xcframework` in both `ts1/macos/` and `ts1/termsurf-macos/`
- Both Xcode projects reference their local xcframework
- Modified files: `ts1/build.zig`, `ts1/src/build/GhosttyXCFramework.zig`

## TermSurf 2.0 (Planned)

TermSurf 2.0 will be based on WezTerm + cef-rs for cross-platform support with full browser capabilities.

See `docs/termsurf2-wezterm-analysis.md` for the detailed architecture analysis and implementation plan.

### Key differences from 1.x:
- **Language:** Rust (single language) vs Zig + Swift + Objective-C
- **Platforms:** Linux, Windows, macOS vs macOS-only
- **Browser:** CEF (Chromium) vs WKWebView (limited API)
- **Terminal:** WezTerm vs Ghostty

## cef-rs

CEF (Chromium Embedded Framework) Rust bindings for browser integration in TermSurf 2.0.

### Commands

- **Build:** `cd cef-rs && cargo build`
- **Build OSR example:** `cd cef-rs && cargo build --example osr`
- **Run OSR example:** `cd cef-rs && ./target/debug/examples/osr`
- **Bundle CEF app (macOS):** `cd cef-rs && cargo run -p bundle-cef-app -- ./target/debug/examples/osr`

### Key Files

- `cef-rs/cef/` - Main CEF wrapper crate
- `cef-rs/cef/src/osr_texture_import/` - Off-screen rendering texture import (IOSurface on macOS)
- `cef-rs/examples/osr/` - Off-screen rendering example with wgpu
- `cef-rs/sys/` - Low-level CEF C API bindings
- `cef-rs/update-bindings/` - Tool to regenerate bindings from CEF headers

### Notes

- CEF binaries are downloaded automatically by the build system
- macOS apps must be bundled with `bundle-cef-app` to include CEF framework
- We have a fix for IOSurface texture import in `cef/src/osr_texture_import/iosurface.rs`
