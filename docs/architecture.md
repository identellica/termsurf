# TermSurf Architecture

This document explains the architectural decisions behind TermSurf, including why we chose to fork Ghostty and use CEF for browser integration.

## Requirements

TermSurf has two primary requirements:

1. **Browser as a pane**: Display web content in terminal panes, not as separate windows
2. **Multi-engine support**: Test web apps in Chromium, Safari/WebKit, and Firefox/Gecko

## Terminal Emulator Comparison

We evaluated three terminal emulators:

### Ghostty

- **Language**: Zig (libghostty) + Swift (macOS app) + GTK (Linux)
- **Rendering**: Platform-native views (NSView on macOS)
- **Pane Management**: Host application manages layout via callbacks
- **Architecture**: libghostty is a library; apps embed it

**Key Insight**: The macOS app uses native NSViews for each terminal surface. Adding a browser view is natural - it's just another native view managed by the same SplitTree.

### WezTerm

- **Language**: Pure Rust
- **Rendering**: Custom GPU renderer (wgpu)
- **Pane Management**: Internal binary tree with `Pane` trait
- **Architecture**: Monolithic application

**Key Insight**: WezTerm renders everything to a single GPU surface. Adding browser views requires either:
- Overlay approach (native window positioned over pane region - focus issues)
- Texture compositing (CEF offscreen rendering - complex)

### Alacritty

- **Language**: Rust
- **Rendering**: OpenGL/wgpu
- **Pane Management**: None (single terminal per window)

Not suitable - would require building pane management from scratch.

## Why Ghostty?

| Criteria | Ghostty | WezTerm |
|----------|---------|---------|
| Browser integration | Trivial (native view sibling) | Hard (overlay or texture) |
| Time to MVP | Weeks | Months |
| Focus management | Native (AppKit handles it) | Custom (must route to browser) |
| Cross-platform | Separate apps per platform | Single codebase |

**Ghostty wins on browser integration** - the fundamental requirement. WezTerm's custom GPU rendering makes browser integration significantly harder.

## Browser Integration Approach

### CEF (Chosen)

We use the Chromium Embedded Framework (CEF) for browser panes:

**Why CEF over system webviews (WKWebView)?**
- **Profile isolation**: Different cache paths = separate cookies/localStorage per profile
- **Console capture**: Native `on_console_message` callback with log level for stdout/stderr routing
- **DevTools**: Full Chrome DevTools built-in
- **Cross-platform consistency**: Same C API on macOS, Linux, Windows
- **Multi-engine future**: CEF provides the architecture pattern for adding WebKit/Gecko later

**Trade-offs**:
- Binary size: ~150-200MB (acceptable for web developer tooling)
- Complexity: C API requires Swift wrapper (CEFKit)

See [docs/cef.md](cef.md) for C API reference and integration details.

### Future: Multi-Engine Support

TermSurf is designed to support multiple browser engines via a common `BrowserEngine` protocol:

- **Chromium (CEF)** - Current implementation
- **Safari/WebKit** - Planned (WKWebView wrapper with same interface)
- **Firefox/Gecko** - Planned (longer-term, requires custom embedding work)

This allows `termsurf open --browser webkit` or `--browser gecko` for cross-browser testing.

## SplitTree Architecture

Ghostty's macOS app uses a binary tree for pane layout:

```swift
// Ghostty's SplitTree (Sources/Features/Splits/SplitTree.swift)
indirect enum Node: Codable {
    case leaf(view: ViewType)  // ViewType = terminal surface
    case split(Split)
}

struct Split {
    let direction: Direction  // horizontal or vertical
    let ratio: Double
    let left: Node
    let right: Node
}
```

### TermSurf Extension

We extend this to support multiple pane types:

```swift
// TermSurf modification
enum PaneContent {
    case terminal(TerminalSurfaceView)
    case browser(CEFBrowserView)
}

indirect enum Node: Codable {
    case leaf(pane: PaneContent)
    case split(Split)
}
```

This allows:
- Same SplitTree logic for layout
- Same focus navigation (ctrl+h/j/k/l)
- Terminal and browser panes are peers

## Console Bridging

When a browser pane is active, JavaScript console output should appear in the terminal's stdout/stderr.

### CEF Implementation

CEF provides native console capture via `on_console_message` callback:

```c
int (*on_console_message)(
    cef_display_handler_t* self,
    cef_browser_t* browser,
    cef_log_severity_t level,   // Route based on this
    const cef_string_t* message,
    const cef_string_t* source,
    int line
);
```

Routing:
- `LOGSEVERITY_DEBUG`, `LOGSEVERITY_INFO` → stdout
- `LOGSEVERITY_WARNING`, `LOGSEVERITY_ERROR` → stderr

**Note**: The callback only receives the first argument. For multiple arguments, inject JavaScript to wrap console methods and JSON.stringify all args before logging.

## Cross-Platform Strategy

### Phase 1: macOS MVP

Focus on macOS first:
- Fork the Swift app (`termsurf-macos/`)
- Add CEF browser pane support
- Implement profile isolation

### Phase 2: Linux

Apply same patterns to Ghostty's GTK app:
- Create `termsurf-linux/` as fork of GTK app
- Use same CEF integration (CEF supports Linux)
- Share CEFKit concepts, adapt for GTK

## File Structure

```
termsurf/
├── src/                          # libghostty (Zig) - shared core
├── macos/                        # Original Ghostty macOS app
├── termsurf-macos/               # TermSurf macOS app
│   ├── Sources/
│   │   ├── App/                  # App delegate, main entry
│   │   ├── Features/
│   │   │   ├── Splits/           # SplitTree (extend for browser panes)
│   │   │   └── Terminal/         # Terminal views
│   │   └── Ghostty/              # Ghostty integration
│   ├── Frameworks/
│   │   └── cef/                  # CEF binary distribution
│   └── CEFKit/                   # Swift bindings for CEF (to be created)
├── docs/                         # Documentation
│   ├── architecture.md           # This file
│   └── cef.md                    # CEF C API reference
└── TODO.md                       # Active task checklist
```
