# TermSurf 2.0: CEF + Zig Architecture (SUPERSEDED)

> **This approach has been superseded.** We explored integrating CEF directly
> into Ghostty's Zig codebase, but ultimately chose a different path: **WezTerm
> + cef-rs** (pure Rust). The WezTerm approach is simpler (single language),
> already cross-platform, and cef-rs provides working Rust bindings with
> validated OSR support.
>
> **See [termsurf2-wezterm-analysis.md](termsurf2-wezterm-analysis.md) for the
> current architecture.**

---

*The content below is preserved for historical reference.*

---

This document outlines the vision and architecture for TermSurf 2.0, which integrates Chromium Embedded Framework (CEF) directly into the Zig codebase rather than using Swift/WKWebView.

## Overview

**TermSurf 1.0** (current) uses Apple's WKWebView for browser panes. This works but has limitations:
- Visited links require private API workarounds
- Limited to macOS/iOS
- Some OAuth/iframe navigation issues
- No Chrome DevTools (Safari Web Inspector only)
- Behavior varies by macOS version

**TermSurf 2.0** will integrate CEF at the Zig level, providing:
- Full Chromium browser capabilities
- Chrome DevTools support
- Consistent behavior across platforms
- Cross-platform potential (Linux, Windows)
- Same rendering architecture as the terminal

## Key Insight: How Ghostty Rendering Works

The critical discovery enabling this architecture: **Ghostty's terminal rendering is done entirely in Zig, not Swift.**

```
┌─────────────────────────────────────────────────────────────┐
│                     Swift (thin shell)                       │
│  - Creates NSWindow/NSView                                   │
│  - Handles macOS UI (menus, dialogs, tabs)                  │
│  - Forwards input events to Zig                             │
│  - Passes NSView pointer to libghostty                      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ (NSView pointer)
┌─────────────────────────────────────────────────────────────┐
│                     Zig (libghostty)                        │
│  - Creates CALayer (IOSurfaceLayer) on the NSView           │
│  - Owns the Metal rendering pipeline                        │
│  - Renders terminal content to IOSurface                    │
│  - CALayer displays the IOSurface                           │
│  - Calls Objective-C APIs directly via `objc` package       │
└─────────────────────────────────────────────────────────────┘
```

Swift is essentially a **container** - it provides windows and native widgets, but Zig handles all GPU rendering. This means CEF can be integrated at the Zig level using the same pattern.

## Why Zig + CEF Works (When Swift + CEF Failed)

### The Swift Problem

CEF has a C API, but integrating it with Swift failed due to struct marshalling issues:
- Swift class memory layout doesn't match what CEF expects
- The CEF C-to-C++ wrapper validates struct sizes and rejects Swift-created structs
- See `docs/cef.md` for detailed documentation of the Swift integration challenges

### The Zig Solution

Zig doesn't have these problems:
- **Direct C interop**: Zig can import C headers and call C functions with zero overhead
- **Exact memory layout control**: Zig structs have predictable, C-compatible layouts
- **No marshalling**: When Zig creates a `cef_app_t` struct, it's exactly what CEF expects
- **Proven pattern**: Zig already successfully calls Objective-C APIs (Metal, CoreGraphics, CALayer)

## Proposed Architecture

### TermSurf 2.0 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Swift (thin shell)                       │
│  - Creates NSWindow/NSView for terminal                     │
│  - Creates NSWindow/NSView for browser                      │
│  - Handles macOS UI (menus, dialogs, tabs)                  │
│  - Forwards events to Zig                                   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ (NSView pointers)
┌─────────────────────────────────────────────────────────────┐
│                     Zig (libghostty + CEF)                  │
│                                                             │
│  Terminal Surface:          Browser Surface:                │
│  ┌─────────────────┐       ┌─────────────────┐             │
│  │ Metal renderer  │       │ CEF C API       │             │
│  │ Font rasterizer │       │ Off-screen mode │             │
│  │ Terminal state  │       │ Browser state   │             │
│  └────────┬────────┘       └────────┬────────┘             │
│           │                         │                       │
│           ▼                         ▼                       │
│  ┌─────────────────────────────────────────────┐           │
│  │         IOSurface → CALayer                 │           │
│  │    (same rendering pattern for both!)       │           │
│  └─────────────────────────────────────────────┘           │
└─────────────────────────────────────────────────────────────┘
```

### Key Components

1. **CEF Zig Bindings** (`src/browser/`)
   - Import CEF C headers
   - Implement CEF handlers (`cef_app_t`, `cef_client_t`, `cef_render_handler_t`)
   - Manage browser lifecycle

2. **Off-Screen Rendering (OSR)**
   - CEF renders to pixel buffers via `OnPaint()` callback
   - Zig copies pixels to IOSurface
   - Same CALayer display as terminal

3. **libghostty API Extension**
   - Add `ghostty_browser_new()` alongside `ghostty_surface_new()`
   - Expose browser control functions (navigate, back, forward, etc.)
   - Console message callbacks

4. **Swift Integration**
   - Minimal changes to existing Swift code
   - Create browser views same way as terminal views
   - Pass NSView pointer to Zig, Zig handles the rest

## CEF Integration Details

### Off-Screen Rendering Mode

CEF's OSR mode is ideal for this architecture:

```zig
// Simplified concept
const RenderHandler = struct {
    // CEF calls this when it has new pixels to display
    fn onPaint(
        self: *RenderHandler,
        browser: *cef_browser_t,
        type: PaintElementType,
        dirty_rects: []const cef_rect_t,
        buffer: [*]const u8,  // BGRA pixel data
        width: c_int,
        height: c_int,
    ) void {
        // Copy pixels to IOSurface
        self.iosurface.lock();
        @memcpy(self.iosurface.getBaseAddress(), buffer, width * height * 4);
        self.iosurface.unlock();

        // Trigger CALayer redraw
        self.layer.setNeedsDisplay();
    }
};
```

### CEF C API Handlers

Zig implements these CEF handler structs:

| Handler | Purpose |
|---------|---------|
| `cef_app_t` | Application lifecycle |
| `cef_client_t` | Browser event routing |
| `cef_life_span_handler_t` | Browser creation/destruction |
| `cef_render_handler_t` | Off-screen rendering callbacks |
| `cef_display_handler_t` | Console messages, title changes |
| `cef_request_handler_t` | Navigation, downloads |

### Reference Counting

CEF uses reference counting. Zig implementation:

```zig
fn cef_add_ref(base: *cef_base_ref_counted_t) callconv(.C) void {
    const self = @fieldParentPtr(MyCefHandler, "base", base);
    _ = @atomicRmw(usize, &self.ref_count, .Add, 1, .seq_cst);
}

fn cef_release(base: *cef_base_ref_counted_t) callconv(.C) c_int {
    const self = @fieldParentPtr(MyCefHandler, "base", base);
    const prev = @atomicRmw(usize, &self.ref_count, .Sub, 1, .seq_cst);
    if (prev == 1) {
        self.destroy();
        return 1;
    }
    return 0;
}
```

## Cross-Platform Potential

### TermSurf 2.0 (macOS)
- CEF integrated at Zig level
- Swift provides macOS windowing
- Replaces WKWebView entirely

### TermSurf 3.0 (Linux, Windows)
- Same Zig + CEF core
- GTK frontend on Linux (like Ghostty)
- Win32/WinRT frontend on Windows
- Shared browser code across all platforms

The terminal (libghostty) already works cross-platform. Adding CEF at the Zig level means the browser would too.

## Implementation Roadmap

### Phase 1: Proof of Concept
- [ ] Download CEF binary distribution
- [ ] Create minimal Zig CEF bindings
- [ ] Initialize CEF from Zig
- [ ] Create a browser with OSR enabled
- [ ] Render to an IOSurface
- [ ] Display via CALayer in a test window

### Phase 2: libghostty Integration
- [ ] Add browser surface type to libghostty
- [ ] Implement `ghostty_browser_*` C API
- [ ] Handle input events (keyboard, mouse)
- [ ] Console message routing
- [ ] Profile/cookie isolation

### Phase 3: Swift Integration
- [ ] Create BrowserView in Swift (mirrors SurfaceView)
- [ ] Integrate with existing tab/split infrastructure
- [ ] Port `web` CLI command to use CEF browser

### Phase 4: Feature Parity
- [ ] DevTools support
- [ ] Downloads
- [ ] Find in page
- [ ] Zoom controls
- [ ] Bookmarks (port from WKWebView)

### Phase 5: Cross-Platform (TermSurf 3.0)
- [ ] GTK frontend for Linux
- [ ] Test on Linux
- [ ] Windows frontend (if desired)

## Open Questions

1. **CEF message loop integration**: How to integrate CEF's message loop with Ghostty's event loop?
   - Option A: `cef_do_message_loop_work()` called from main loop
   - Option B: Separate CEF thread (complexity)

2. **Multi-process architecture**: CEF uses helper processes for GPU, renderer, etc.
   - Need to bundle and configure helper apps
   - May need build system changes

3. **Binary size**: CEF adds ~150MB to the app bundle
   - Acceptable for a browser-focused app
   - Consider optional download for users who don't need browser?

4. **macOS code signing**: CEF framework and helpers need proper signing
   - Already solved for TermSurf 1.0 helper app pattern

5. **GTK + CEF on macOS**: The GTK build failed due to Zig linker bugs
   - For macOS, continue using Swift frontend
   - GTK + CEF for Linux only

## Comparison: WKWebView vs CEF

| Feature | WKWebView (1.0) | CEF (2.0) |
|---------|-----------------|-----------|
| Visited links | Private API workaround | Works natively |
| DevTools | Safari Web Inspector | Full Chrome DevTools |
| Cross-platform | macOS/iOS only | Linux, Windows, macOS |
| Binary size | 0 (system framework) | ~150MB |
| Consistency | Varies by OS version | Same everywhere |
| OAuth/iframes | Some issues | Full support |
| Control | Limited APIs | Full browser control |
| Integration | Swift-only | Zig (portable) |

## References

- [CEF Project](https://bitbucket.org/chromiumembedded/cef) - Official repository
- [CEF Builds](https://cef-builds.spotifycdn.com/index.html) - Binary distributions
- [cefcapi](https://github.com/cztomczak/cefcapi) - C API usage example
- [CEF Wiki](https://bitbucket.org/chromiumembedded/cef/wiki/Home) - Documentation
- `docs/cef.md` - CEF C API reference and Swift integration history
- `src/renderer/Metal.zig` - How Ghostty does Metal rendering from Zig
- `src/renderer/metal/IOSurfaceLayer.zig` - IOSurface → CALayer pattern
