# TermSurf 2.0 Architecture Analysis: WezTerm + cef-rs

This document compares the current TermSurf architecture (Ghostty + WKWebView)
with a proposed alternative (WezTerm + cef-rs) for building a cross-platform
terminal-browser.

## Executive Summary

**Recommendation: Proceed with WezTerm + cef-rs for TermSurf 2.0**

The WezTerm + cef-rs approach offers significant advantages:

- Single language (Rust) vs three (Zig + Swift + Objective-C)
- True cross-platform (Linux, Windows, macOS) vs macOS-only
- Full browser API (CEF) vs limited (WKWebView)
- Both projects use wgpu for GPU rendering, enabling clean integration
- cef-rs already has working OSR (Off-Screen Rendering) with hardware
  acceleration

## Current Architecture: Ghostty + WKWebView

### Stack

```
┌─────────────────────────────────────────┐
│           Swift UI Layer                │  termsurf-macos/ (~33k lines)
│   (WebViewOverlay, ControlBar, etc.)    │
├─────────────────────────────────────────┤
│           WKWebView (WebKit)            │  Apple's WebKit framework
├─────────────────────────────────────────┤
│         libghostty (Zig)                │  src/ (~213k lines)
│   Terminal emulation, GPU rendering     │
├─────────────────────────────────────────┤
│      Metal (macOS GPU)                  │
└─────────────────────────────────────────┘
```

### Strengths

- Working product (TermSurf 1.0 released)
- Ghostty is high-quality terminal emulator
- WKWebView is lightweight and native

### Weaknesses

- **macOS only** - No path to Linux/Windows without rewrite
- **Limited browser API** - WKWebView lacks:
  - Proper visited link handling
  - Full cookie/storage control
  - Extension support
  - DevTools API (only Safari Web Inspector)
  - Robust download handling
  - Many other browser features
- **Three languages** - Zig + Swift + Objective-C increases complexity
- **No upstream path** - TermSurf changes unlikely to merge into Ghostty

## Proposed Architecture: WezTerm + cef-rs

### Stack

```
┌─────────────────────────────────────────┐
│           Rust Application              │
│   (TermSurf-specific UI, integration)   │
├─────────────────────────────────────────┤
│     WezTerm Core        │   CEF (cef-rs)│  Both render to wgpu textures
│  Terminal emulation     │   Browser     │
│     wgpu rendering      │   OSR mode    │
├─────────────────────────────────────────┤
│              wgpu (unified)             │  WebGPU abstraction
├─────────────────────────────────────────┤
│   Metal │ Vulkan │ DX12 │ OpenGL       │  Platform GPU APIs
└─────────────────────────────────────────┘
```

### Key Insight: Shared GPU Path

Both WezTerm and cef-rs use **wgpu** for GPU rendering:

- WezTerm: `wgpu = "25.0.2"` for terminal rendering
- cef-rs: `wgpu = "28"` for CEF texture import

CEF's accelerated OSR mode renders to shared textures:

- **macOS**: IOSurface → Metal → wgpu
- **Linux**: DMA-BUF → Vulkan → wgpu
- **Windows**: D3D11 → Vulkan/DX12 → wgpu

This means we can composite terminal and browser content in a unified GPU
pipeline.

## Detailed Analysis

### WezTerm Architecture

**Codebase Stats**

- 452 Rust files, ~410k lines
- Mature, feature-rich terminal emulator
- Active development, good community

**Key Components**

| Component       | Purpose                           | Files                            |
| --------------- | --------------------------------- | -------------------------------- |
| `wezterm-gui/`  | Main GUI application              | termwindow, rendering            |
| `mux/`          | Multiplexer (tabs, panes, splits) | pane.rs (~29k), tab.rs (~85k)    |
| `window/`       | Cross-platform windowing          | macos/, windows/, x11/, wayland/ |
| `termwiz/`      | Terminal emulation library        | VT parsing, cell representation  |
| `wezterm-font/` | Font rendering                    | HarfBuzz, FreeType integration   |

**Rendering Pipeline**

1. Terminal content → glyph atlas → wgpu textures
2. WebGPU shader (`shader.wgsl`) composites glyphs
3. Platform backend (Metal/Vulkan/DX12/OpenGL) presents

**Pane System** The `Pane` trait (`mux/src/pane.rs:167`) is terminal-oriented
but extensible:

```rust
pub trait Pane: Downcast + Send + Sync {
    fn pane_id(&self) -> PaneId;
    fn get_cursor_position(&self) -> StableCursorPosition;
    fn get_lines(&self, lines: Range<StableRowIndex>) -> (StableRowIndex, Vec<Line>);
    fn resize(&self, size: TerminalSize) -> anyhow::Result<()>;
    fn key_down(&self, key: KeyCode, mods: KeyModifiers) -> anyhow::Result<()>;
    fn mouse_event(&self, event: MouseEvent) -> anyhow::Result<()>;
    // ... many more terminal-specific methods
}
```

A browser pane would need a different rendering path, not terminal lines.

**Platform Support**

| Platform | Windowing    | GPU            |
| -------- | ------------ | -------------- |
| macOS    | Cocoa        | Metal          |
| Linux    | X11, Wayland | OpenGL, Vulkan |
| Windows  | Win32        | DX12, OpenGL   |

### cef-rs Architecture

**Codebase Stats**

- CEF version: 143.7.0 (recent Chromium)
- Full CEF API bindings (~2.3MB per platform)
- Active Tauri project maintenance

**Key Components**

| Component             | Purpose                                  |
| --------------------- | ---------------------------------------- |
| `cef/`                | High-level Rust API                      |
| `sys/`                | Low-level FFI bindings                   |
| `osr_texture_import/` | GPU texture sharing                      |
| `examples/osr/`       | Working hardware-accelerated OSR example |

**OSR (Off-Screen Rendering) Pipeline**

```rust
// From examples/osr/src/webrender.rs
fn on_accelerated_paint(
    &self,
    _browser: Option<&mut Browser>,
    type_: PaintElementType,
    _dirty_rects: Option<&[Rect]>,
    info: Option<&AcceleratedPaintInfo>,
) {
    let shared_handle = SharedTextureHandle::new(info);
    let texture = shared_handle.import_texture(&device)?;
    // texture is now a wgpu::Texture ready for rendering
}
```

**Platform-Specific Texture Import**

| Platform | Mechanism            | File           |
| -------- | -------------------- | -------------- |
| macOS    | IOSurface → Metal    | `iosurface.rs` |
| Linux    | DMA-BUF → Vulkan     | `dmabuf.rs`    |
| Windows  | D3D11 shared texture | `d3d11.rs`     |

**CEF Multi-Process Model** CEF uses multiple processes (browser, renderer, GPU,
etc.):

```rust
// Main process check
let is_browser_process = cmd.has_switch(Some(&"type".into())) != 1;
let ret = execute_process(Some(args), Some(&mut app), sandbox_info);
if is_browser_process {
    // Initialize CEF, create browser windows
} else {
    // Subprocess exits after execute_process
}
```

**Browser API Coverage** CEF provides full Chromium API including:

- Navigation, history, cookies, storage
- JavaScript execution and message passing
- DevTools protocol
- Extensions (limited)
- Downloads, uploads, permissions
- Certificate handling
- Print preview
- All HTML5 features

## Integration Strategy

### Phase 1: Fork WezTerm

- Fork WezTerm as TermSurf base
- Remove/disable features not needed (SSH multiplexing, etc.)
- Understand rendering pipeline

### Phase 2: Add CEF Integration

- Add cef-rs dependency
- Create `BrowserPane` type (not implementing terminal `Pane` trait)
- Implement CEF OSR handlers
- Import CEF textures into wgpu pipeline

### Phase 3: Unified Compositor

- Modify WezTerm's render pass to support mixed pane types
- Terminal panes: existing glyph rendering
- Browser panes: CEF texture blit
- Handle pane splitting between types

### Phase 4: CLI Integration

- Implement `web` command similar to TermSurf 1.0
- Console bridging (stdout/stderr routing)
- JavaScript API (`window.termsurf.exit()`)

### Phase 5: Polish

- Profile isolation
- Bookmarks
- DevTools integration
- Platform-specific packaging

## Code Changes Required

### WezTerm Modifications

**New pane type** (`src/browser_pane.rs`):

```rust
pub struct BrowserPane {
    id: PaneId,
    browser: cef::Browser,
    texture: RefCell<Option<wgpu::Texture>>,
    size: RefCell<Size>,
}

impl BrowserPane {
    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView) {
        // Blit CEF texture to render target
    }
}
```

**Render pipeline modification** (`wezterm-gui/src/termwindow/render/`):

```rust
// In render loop
match pane {
    PaneType::Terminal(term_pane) => {
        // Existing terminal rendering
        self.render_terminal_pane(term_pane, ...);
    }
    PaneType::Browser(browser_pane) => {
        // New browser rendering
        browser_pane.render(encoder, target);
    }
}
```

**Event routing**:

```rust
// In input handling
if let Some(browser_pane) = self.get_active_browser_pane() {
    // Route keyboard/mouse to CEF
    browser_pane.browser.host().send_key_event(...);
    browser_pane.browser.host().send_mouse_event(...);
}
```

### CEF Setup

**Initialization** (in main or startup):

```rust
fn init_cef() {
    let args = cef::args::Args::new();
    let settings = cef::Settings {
        windowless_rendering_enabled: true,
        external_message_pump: true, // Integrate with WezTerm event loop
        ..Default::default()
    };
    cef::initialize(Some(args.as_main_args()), Some(&settings), ...);
}
```

**Message pump integration**:

```rust
// In WezTerm's main event loop
loop {
    // Process WezTerm events
    process_wezterm_events();

    // Pump CEF messages
    cef::do_message_loop_work();

    // Render frame
    render();
}
```

## Risk Assessment

### Technical Risks

| Risk                       | Likelihood | Impact | Mitigation                                |
| -------------------------- | ---------- | ------ | ----------------------------------------- |
| wgpu version mismatch      | Medium     | Medium | Align versions, test early                |
| CEF message pump conflicts | Medium     | High   | Study WezTerm event loop, prototype early |
| Performance overhead       | Low        | Medium | CEF OSR is hardware-accelerated           |
| CEF binary size (~100MB)   | Certain    | Low    | Accept as tradeoff for full browser       |
| Cross-platform CEF quirks  | Medium     | Medium | Test on all platforms early               |

### Project Risks

| Risk                         | Likelihood | Impact | Mitigation                        |
| ---------------------------- | ---------- | ------ | --------------------------------- |
| Large codebase to understand | Certain    | Medium | Start with minimal changes        |
| Upstream WezTerm changes     | Medium     | Low    | Periodic merge, maintain fork     |
| cef-rs API changes           | Low        | Medium | Pin versions, contribute upstream |

## Comparison Summary

| Aspect                 | Ghostty + WKWebView | WezTerm + cef-rs      |
| ---------------------- | ------------------- | --------------------- |
| **Languages**          | Zig + Swift + ObjC  | Rust                  |
| **Platforms**          | macOS only          | Linux, Windows, macOS |
| **Browser API**        | Limited             | Full Chromium         |
| **Terminal quality**   | Excellent           | Excellent             |
| **GPU rendering**      | Metal only          | wgpu (all backends)   |
| **Codebase size**      | ~246k lines         | ~410k lines           |
| **Binary size**        | ~20MB               | ~150MB+ (with CEF)    |
| **Community**          | Ghostty growing     | WezTerm established   |
| **Maintenance**        | Two upstreams       | Two upstreams         |
| **Integration effort** | Done (1.0)          | Significant           |

## Recommendation

**Proceed with WezTerm + cef-rs** because:

1. **Cross-platform is critical** - A terminal-browser should work everywhere
   developers work
2. **Full browser API** - CEF eliminates the frustrating limitations of
   WKWebView
3. **Single language** - Rust throughout simplifies development and contribution
4. **Proven integration path** - cef-rs OSR example demonstrates the exact
   pattern needed
5. **Shared GPU architecture** - Both use wgpu, enabling clean compositor design

The main tradeoffs are:

- Larger binary size (CEF adds ~100MB)
- Significant initial development effort
- Abandoning TermSurf 1.0 codebase

Given the goal of a "primary terminal AND primary browser", the CEF approach is
the only realistic path to feature parity with standalone browsers.

## Next Steps

1. Set up WezTerm build environment
2. Build and run cef-rs OSR example on macOS
3. Prototype: Add CEF texture to WezTerm render pipeline
4. Create `BrowserPane` abstraction
5. Integrate `web` CLI command
6. Test on Linux (critical for cross-platform validation)

## References

- WezTerm: https://github.com/wez/wezterm
- cef-rs: https://github.com/tauri-apps/cef-rs
- CEF Documentation: https://bitbucket.org/chromiumembedded/cef/wiki/Home
- wgpu: https://wgpu.rs/
