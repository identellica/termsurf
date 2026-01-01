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

- **Build:** `cd termsurf-macos && xcodebuild -project Ghostty.xcodeproj -scheme Ghostty -configuration Debug build`
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
- Xcode project: `termsurf-macos/Ghostty.xcodeproj`
- Documentation: `termsurf-macos/docs/`
  - `ARCHITECTURE.md` - Technical decisions and design
  - `ROADMAP.md` - Development phases and milestones

## libghostty-vt

- Build: `zig build lib-vt`
- Build Wasm Module: `zig build lib-vt -Dtarget=wasm32-freestanding`
- Test: `zig build test-lib-vt`
- Test filter: `zig build test-lib-vt -Dtest-filter=<test name>`
- When working on libghostty-vt, do not build the full app.
- For C only changes, don't run the Zig tests. Build all the examples.

## Key Files for TermSurf Development

When implementing webview pane support, focus on these files in `termsurf-macos/`:

1. **SplitTree.swift** (`Sources/Features/Splits/`) - Pane layout tree, extend for webview nodes
2. **TerminalSplitTreeView.swift** - Renders panes, add webview rendering
3. **BaseTerminalController.swift** - Handle `termsurf open` command
4. **New: WebviewPane.swift** - WKWebView wrapper (to be created)

## Icon Generation

TermSurf uses two icons: a production icon and a debug icon (shown in DEBUG builds to distinguish dev from release).

- **Source icons:**
  - Production: `termsurf-macos/icon-source/termsurf-icon.png`
  - Debug: `termsurf-macos/icon-source/termsurf-debug-icon.png`
- **Update icons:** `cd termsurf-macos && ./scripts/generate-icons.sh`
- **Generated assets:**
  - `termsurf-macos/Assets.xcassets/AppIcon.appiconset/` (production, multiple sizes)
  - `termsurf-macos/Assets.xcassets/TermSurfDebugIcon.imageset/` (debug)

Note: Source icons should be at least 1024x1024 pixels for best quality.

## Build System Notes

- `zig build` creates `GhosttyKit.xcframework` in both `macos/` and `termsurf-macos/`
- Both Xcode projects reference their local xcframework
- Modified files: `build.zig`, `src/build/GhosttyXCFramework.zig`
