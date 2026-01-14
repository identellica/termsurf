# CEF Integration Guide

This document covers the Chromium Embedded Framework (CEF) integration for
TermSurf browser panes.

## Current Status

**Status:** CEF integration validated via cef-rs. WezTerm integration pending.

### TermSurf 1.x (Stable)

Uses Apple's WKWebView for browser panes. This works but has limitations:
- Incomplete browser API (no visited links, limited cookie control, etc.)
- macOS only - no path to cross-platform
- No Chrome DevTools (only Safari Web Inspector)

### TermSurf 2.0 (In Development)

Integrates CEF via **cef-rs** (Rust bindings) into a WezTerm fork:
- Full Chromium browser capabilities
- Cross-platform (macOS, Linux, Windows)
- Chrome DevTools support
- Rust has predictable C-compatible memory layouts (unlike Swift)

See [termsurf2-wezterm-analysis.md](termsurf2-wezterm-analysis.md) for the
architecture analysis.

### cef-rs Validation Results

The cef-rs integration has been validated in `cef-rs/examples/osr/`:

| Feature | Status | Notes |
|---------|--------|-------|
| IOSurface texture import (macOS) | Working | Fixed Metal API types |
| Input handling (keyboard, mouse, scroll) | Working | Full event routing |
| Multiple browser instances | Working | Per-instance textures |
| Resize handling | Working | Browser resizes with window |
| Context menu (right-click) | Suppressed | Prevents winit crash |
| Fullscreen | Broken | winit issue, defer to WezTerm |

**Key validation:** Multiple CEF browser instances run successfully in a single
process with independent texture storage and event routing. This is critical
for WezTerm integration where browser panes coexist with terminal panes.

See [cef-rs.md](cef-rs.md) for detailed documentation of our modifications.

## CEF Resources

- [CEF Builds](https://cef-builds.spotifycdn.com/index.html) - Official binary
  distributions
- [CEF Wiki](https://bitbucket.org/chromiumembedded/cef/wiki/Home) - General
  usage guide
- [cef-rs](https://github.com/tauri-apps/cef-rs) - Rust bindings (our fork at
  `cef-rs/`)

---

# Historical Approaches

The content below documents approaches we explored before settling on cef-rs.
These are preserved for reference.

## Zig Integration (Superseded)

We originally planned to integrate CEF directly into Ghostty's Zig codebase.
The rationale:

- Zig has predictable C-compatible memory layouts
- Ghostty already calls Metal/Objective-C from Zig
- Would keep browser logic in the portable core

This approach was superseded by **WezTerm + cef-rs** because:
- WezTerm is already pure Rust (single language)
- cef-rs already handles all CEF struct marshalling
- Both WezTerm and cef-rs use wgpu for GPU rendering

See [termsurf2.md](termsurf2.md) for the original Zig-based architecture plan.

## Swift Integration (Abandoned)

We attempted to integrate CEF directly with Swift but hit fundamental issues
with struct marshalling. CEF's C API wrapper validates struct layouts, and
Swift's class memory model doesn't produce the expected layouts.

### The Core Problem

When passing a `cef_app_t` struct to `cef_initialize()`, CEF's validation failed:

```
[FATAL:cef/libcef_dll/ctocpp/app_ctocpp.cc:118] CefApp_0_CToCpp called with invalid version -1
```

CEF reads `base.size` from the struct pointer and validates it matches the
expected size (80 bytes for `cef_app_t`). Swift's memory layout for classes
doesn't match what CEF expects.

### Approaches Tried

1. **Direct struct allocation** - Failed validation
2. **CEF.swift marshaller pattern** - Embedded C struct as first property of
   Swift class at offset 16. Still failed with modern Swift.
3. **Global @convention(c) functions** - Avoided closure capture issues but
   didn't fix validation.

### Why Rust Works

Rust structs have `#[repr(C)]` which guarantees C-compatible memory layout.
cef-rs uses this to create CEF structs that pass validation. This is why we
moved to cef-rs instead of continuing to fight Swift's memory model.

### References

- [CEF Forum: CefApp_0_CToCpp invalid version](https://magpcss.org/ceforum/viewtopic.php?f=6&t=19114)
- [CEF.swift](https://github.com/aspect-apps/aspect/tree/main/aspect-platform/aspect-platform-cef/aspect-platform-cef-swift) - Historical Swift bindings (circa 2016, no longer maintained)
