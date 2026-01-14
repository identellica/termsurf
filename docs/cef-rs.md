# cef-rs Modifications

This document tracks TermSurf-specific modifications to the cef-rs library (CEF Rust bindings).

## Overview

cef-rs was imported into the TermSurf monorepo at `cef-rs/` and modified to fix critical bugs and add features needed for browser pane integration. The OSR (Off-Screen Rendering) example serves as our validation testbed.

## Validation Status

| Feature | Status | Commit |
|---------|--------|--------|
| IOSurface texture import (macOS) | Working | `d8b58edea` |
| Purple flash fix | Working | `e6f8a2e4c` |
| Input handling | Working | `88ab04355` |
| Multi-browser instances | Working | `40f2a55cc` |
| Context menu suppression | Working | `25def7592` |
| Resize handling | Working | — |
| Fullscreen | Broken | winit issue, defer to WezTerm |

## Commits

### 1. Initial Import (`5075cc44c`)

Moved cef-rs files into `cef-rs/` folder for TermSurf integration.

---

### 2. Fix macOS IOSurface Texture Import Crash (`d8b58edea`)

**File:** `cef-rs/cef/src/osr_texture_import/iosurface.rs`

**Problem:** The original code used `std::mem::transmute` to cast raw pointers to Metal API types, causing crashes at memory address 0x1f00000080.

**Root cause:** Transmuting raw device/descriptor pointers to `&metal::NSObject` references was incorrect. The Metal-rs crate expects properly typed references that implement the `Message` trait for Objective-C message sending.

**Fix:** Replace unsafe transmutes with proper typed references via the objc crate:

```rust
// Before (crashed):
let texture: metal::Texture = std::mem::transmute(objc::msg_send![
    std::mem::transmute::<_, &metal::NSObject>(raw_device),
    newTextureWithDescriptor:std::mem::transmute::<_, &metal::NSObject>(metal_desc.as_ptr())
    iosurface:self.handle
    plane:0usize
]);

// After (working):
let device_ref: &metal::DeviceRef = raw_device;
let desc_ref: &metal::TextureDescriptorRef = metal_desc.as_ref();
let texture: metal::Texture = objc::msg_send![
    device_ref,
    newTextureWithDescriptor:desc_ref
    iosurface:self.handle
    plane:0usize
];
```

**Additional fixes:**
- Added IOSurface validation via C functions (`IOSurfaceGetWidth`, `IOSurfaceGetHeight`)
- Removed `metal_desc.set_storage_mode()` call - Metal determines storage mode from the IOSurface itself
- Added dimension mismatch warnings

---

### 3. Fix Purple Flash on Startup (`e6f8a2e4c`)

**Files:** `cef-rs/examples/osr/src/main.rs`, `cef-rs/cef/src/osr_texture_import/iosurface.rs`

**Problem:** Uninitialized GPU memory displayed as purple/magenta color before CEF rendered its first frame.

**Fix:** Clear the render pass to black before any CEF content:

```rust
ops: wgpu::Operations {
    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
    store: wgpu::StoreOp::Store,
}
```

---

### 4. Add Input Handling (`88ab04355`)

**File:** `cef-rs/examples/osr/src/main.rs`

**Problem:** The OSR example had no input handling - browsers were non-interactive.

**Added:**
- Mouse move, click, drag events
- Mouse wheel scrolling
- Keyboard input with proper key codes
- Modifier key tracking (Shift, Ctrl, Alt, Cmd)
- Mouse button state tracking

**Key implementation details:**

1. **CEF event flags** for modifier/button state:
   ```rust
   const EVENTFLAG_SHIFT_DOWN: u32 = 1 << 1;
   const EVENTFLAG_CONTROL_DOWN: u32 = 1 << 2;
   const EVENTFLAG_ALT_DOWN: u32 = 1 << 3;
   const EVENTFLAG_LEFT_MOUSE_BUTTON: u32 = 1 << 4;
   const EVENTFLAG_MIDDLE_MOUSE_BUTTON: u32 = 1 << 5;
   const EVENTFLAG_RIGHT_MOUSE_BUTTON: u32 = 1 << 6;
   const EVENTFLAG_COMMAND_DOWN: u32 = 1 << 7;
   ```

2. **Platform-specific native key codes:**
   - macOS: Uses native key codes (0x00-0x7E range)
   - Windows/Linux: Uses Windows Virtual Key codes (0x08-0x5A range)

3. **Text input:** Sends `CHAR` events after `KEYDOWN` for actual character input

---

### 5. Window Config Cleanup (`c4bbf909d`)

**File:** `cef-rs/examples/osr/src/main.rs`

**Changes:**
- Added descriptive window titles: `format!("CEF Browser - {}", url)`
- Set explicit default size: `LogicalSize::new(800.0, 600.0)`
- Documented fullscreen limitation (winit issue, not cef-rs)

---

### 6. Add Multi-Browser Instance Support (`40f2a55cc`)

**Files:** `cef-rs/examples/osr/src/main.rs`, `cef-rs/examples/osr/src/webrender.rs`

**Problem:** Original code used a global `thread_local!` texture holder, meaning only one browser could render at a time.

**Solution:** Per-browser texture storage with HashMap-based window management.

**Architecture:**

```rust
/// Per-browser instance state
struct BrowserInstance {
    state: State,                    // wgpu rendering state
    browser: cef::Browser,           // CEF browser handle
    size: Rc<RefCell<LogicalSize>>,  // Shared with RenderHandler
    texture_holder: TextureHolder,   // Per-instance texture storage
    cursor_pos: (f64, f64),          // Mouse position for this window
    closing: bool,
}

/// Application manages multiple browser windows
struct App {
    instances: HashMap<WindowId, BrowserInstance>,
    key_modifiers: u32,   // Shared modifier state
    mouse_buttons: u32,   // Shared button state
    urls_to_open: Vec<&'static str>,
}
```

**Key changes to `webrender.rs`:**

```rust
/// Return type includes per-browser texture holder
pub struct RenderHandlerParts {
    pub handler: OsrRenderHandler,
    pub size: Rc<RefCell<LogicalSize<f32>>>,
    pub texture_holder: Rc<RefCell<Option<wgpu::BindGroup>>>,
}

/// Each RenderHandler stores to its own texture_holder
impl OsrRenderHandler {
    // In on_accelerated_paint:
    *self.handler.texture_holder.borrow_mut() = Some(bind_group);
}
```

**Event routing:** All window events are routed by `WindowId` to the correct browser instance.

---

### 7. Suppress Context Menu to Prevent Crash (`25def7592`)

**File:** `cef-rs/examples/osr/src/webrender.rs`

**Problem:** Right-clicking triggered CEF to display a native context menu, which called `NSApplication.isHandlingSendEvent` - a method that winit's NSApplication subclass doesn't implement, causing a crash.

**Fix:** Implement `ContextMenuHandler` that clears the menu model before display:

```rust
#[derive(Clone)]
pub struct OsrContextMenuHandler {}

wrap_context_menu_handler! {
    pub(crate) struct ContextMenuHandlerBuilder {
        handler: OsrContextMenuHandler,
    }

    impl ContextMenuHandler {
        fn on_before_context_menu(
            &self,
            _browser: Option<&mut Browser>,
            _frame: Option<&mut Frame>,
            _params: Option<&mut ContextMenuParams>,
            model: Option<&mut MenuModel>,
        ) {
            // Clear the menu model to suppress the context menu
            if let Some(model) = model {
                model.clear();
            }
        }
    }
}
```

**Integration:** Added `context_menu_handler()` to the `Client` implementation:

```rust
wrap_client! {
    impl Client {
        fn render_handler(&self) -> Option<RenderHandler> { ... }
        fn context_menu_handler(&self) -> Option<ContextMenuHandler> {
            Some(self.context_menu_handler.clone())
        }
    }
}
```

---

## Files Modified

| File | Lines Changed | Purpose |
|------|---------------|---------|
| `cef/src/osr_texture_import/iosurface.rs` | ~66 | Metal API type fix, IOSurface validation |
| `examples/osr/src/main.rs` | ~600 | Input handling, multi-browser, window config |
| `examples/osr/src/webrender.rs` | ~80 | Per-browser texture storage, context menu handler |

## Files NOT Modified

- `cef-rs/sys/` - Low-level CEF C API bindings
- `cef-rs/cef/src/` - Core library (except iosurface.rs)
- `cef-rs/examples/cefsimple/` - Other examples
- Build system, CI, documentation

## Known Issues

### Fullscreen Broken

Fullscreen mode crashes due to a winit issue with NSApplication event handling. This is deferred to WezTerm integration, which uses its own windowing system.

### Clippy Warnings

The objc crate generates `unexpected_cfgs` warnings for `cargo-clippy` feature checks. Suppressed with `#![allow(unexpected_cfgs)]` in iosurface.rs.

## Running the Validation Example

```bash
cd cef-rs
cargo build -p cef-osr
cargo run -p bundle-cef-app -- cef-osr -o cef-osr.app
./cef-osr.app/Contents/MacOS/cef-osr
```

The example opens two browser windows (github.com and google.com) to validate multi-browser support. Test by:
- Interacting with each window independently (typing, clicking, scrolling)
- Closing one window and verifying others continue working
- Resizing windows

## Next Steps

These modifications validate that cef-rs is ready for WezTerm integration. The patterns established here (per-browser texture storage, input routing, context menu suppression) should transfer directly to the BrowserPane implementation in ts2.

See `docs/termsurf2-wezterm-analysis.md` for the integration roadmap.
