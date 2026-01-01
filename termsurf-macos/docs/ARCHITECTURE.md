# TermSurf Architecture

This document explains the architectural decisions behind TermSurf, including why we chose to fork Ghostty rather than alternatives like WezTerm.

## Requirements

TermSurf has two primary requirements:

1. **Webview as a pane**: Display web content in terminal panes, not as separate windows
2. **TypeScript configuration**: Replace Lua scripting with TypeScript for web developer familiarity

## Terminal Emulator Comparison

We evaluated three terminal emulators:

### Ghostty

- **Language**: Zig (libghostty) + Swift (macOS app) + GTK (Linux)
- **Rendering**: Platform-native views (NSView on macOS)
- **Pane Management**: Host application manages layout via callbacks
- **Architecture**: libghostty is a library; apps embed it

**Key Insight**: The macOS app uses native NSViews for each terminal surface. Adding a WKWebView is natural - it's just another native view managed by the same SplitTree.

### WezTerm

- **Language**: Pure Rust
- **Rendering**: Custom GPU renderer (wgpu)
- **Pane Management**: Internal binary tree with `Pane` trait
- **Architecture**: Monolithic application

**Key Insight**: WezTerm renders everything to a single GPU surface. Adding webviews requires either:
- Overlay approach (native window positioned over pane region - focus issues)
- Texture compositing (CEF offscreen rendering - adds 150MB+, complex)

### Alacritty

- **Language**: Rust
- **Rendering**: OpenGL/wgpu
- **Pane Management**: None (single terminal per window)

Not suitable - would require building pane management from scratch.

## Why Ghostty?

| Criteria | Ghostty | WezTerm |
|----------|---------|---------|
| Webview integration | Trivial (native view sibling) | Hard (overlay or texture) |
| Time to MVP | ~2-4 weeks | ~6-10 weeks |
| Focus management | Native (AppKit handles it) | Custom (must route to webview) |
| TypeScript config | Medium (new system) | Hard (replace 16 Lua modules) |
| Cross-platform | Separate apps per platform | Single codebase |

**Ghostty wins on webview integration** - the fundamental blocker. WezTerm's custom GPU rendering makes webview integration significantly harder.

## Webview Integration Approach

### System Webviews (Chosen)

Use platform-native webviews as child views:
- **macOS**: WKWebView
- **Linux**: WebKitGTK
- **Windows**: WebView2

**Pros**:
- Zero binary size increase
- Native performance
- Standard focus/input handling
- No compositor complexity

### Alternatives Considered

**CEF (Chromium Embedded Framework)**:
- Full Chromium browser
- Supports offscreen rendering to texture
- Adds ~150-200MB to binary
- Would be needed if we couldn't use native views

**Ultralight**:
- Lightweight HTML renderer (~30MB)
- Designed for game UI embedding
- Not a full browser (limited JS, no extensions)

## SplitTree Architecture

Ghostty's macOS app uses a binary tree for pane layout:

```swift
// Current Ghostty (macos/Sources/Features/Splits/SplitTree.swift)
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
    case webview(WKWebView)
}

indirect enum Node: Codable {
    case leaf(pane: PaneContent)
    case split(Split)
}
```

This allows:
- Same SplitTree logic for layout
- Same focus navigation (ctrl+h/j/k/l)
- Terminal and webview panes are peers

## TypeScript Configuration

### Current State

Ghostty uses its own config file format (not Lua, simpler than WezTerm).

### Planned Approach

Embed a JavaScript engine for TypeScript evaluation:

**macOS**: JavaScriptCore (already available in system frameworks)
**Linux**: Consider libdeno (deno_core) or direct V8

The config API would allow:
```typescript
// termsurf.config.ts
export default {
  font: { family: "JetBrains Mono", size: 14 },
  keybindings: {
    "ctrl+t": "new_tab",
    "ctrl+shift+o": () => termsurf.open("https://localhost:3000")
  }
}
```

## Console.log Bridging

When a webview is active, JavaScript `console.log` output should appear in the terminal's stdout.

Implementation:
1. Inject JavaScript into WKWebView to intercept console methods
2. Forward messages via `WKScriptMessageHandler`
3. Write to the PTY or a dedicated output stream

```javascript
// Injected into webview
const originalConsole = { ...console };
['log', 'warn', 'error', 'info'].forEach(method => {
  console[method] = (...args) => {
    window.webkit.messageHandlers.console.postMessage({
      method,
      args: args.map(a => String(a))
    });
    originalConsole[method](...args);
  };
});
```

## Cross-Platform Strategy

### Phase 1: macOS MVP

Focus on macOS first:
- Fork the Swift app (this folder: `termsurf-macos/`)
- Add webview pane support
- Implement TypeScript config

### Phase 2: Linux

Apply same patterns to Ghostty's GTK app:
- Create `termsurf-linux/` as fork of GTK app
- Use WebKitGTK for webviews
- Share concepts, not code (different UI frameworks)

### Long-Term

Once stable, consider:
- Extracting libghostty as a dependency rather than source fork
- Possibly unifying macOS/Linux into single codebase using cross-platform Rust/Zig GUI

## File Structure

```
termsurf/
├── src/                          # libghostty (Zig) - DO NOT MODIFY
├── macos/                        # Original Ghostty macOS app - DO NOT MODIFY
├── termsurf-macos/               # Our macOS app fork
│   ├── Sources/
│   │   ├── App/                  # App delegate, main entry
│   │   ├── Features/
│   │   │   ├── Splits/           # SplitTree (MODIFY for webview support)
│   │   │   ├── Terminal/         # Terminal views
│   │   │   └── Webview/          # NEW: Webview pane support
│   │   └── Ghostty/              # Ghostty integration
│   ├── docs/                     # This documentation
│   └── README.md
└── ...                           # Other Ghostty files - DO NOT MODIFY
```

## Key Files to Modify

1. **SplitTree.swift** (`Sources/Features/Splits/`) - Add webview node type
2. **TerminalSplitTreeView.swift** - Render webview panes
3. **BaseTerminalController.swift** - Handle `termsurf open` command
4. **New: WebviewPane.swift** - WKWebView wrapper with console bridging
