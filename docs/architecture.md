# TermSurf Architecture

This document explains the architectural decisions behind TermSurf, including
why we chose Ghostty as our foundation and how we integrate browser panes.

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

### WKWebView (MVP)

For the MVP, we use Apple's native WKWebView:

**Why WKWebView for MVP?**
- **Zero dependencies**: Built into macOS, no additional frameworks
- **Native Swift integration**: Seamless API, no C marshalling
- **Profile isolation**: `WKWebsiteDataStore(forIdentifier:)` on macOS 14+
- **Console capture**: JS injection for console.log/error interception
- **DevTools**: Safari Web Inspector available

**Trade-offs accepted**:
- WebKit only (not Chromium)
- No Chrome DevTools (Safari Web Inspector instead)
- Console capture requires JS injection (not native callback)

### Future: Multi-Engine Support

TermSurf is designed to support multiple browser engines via a common protocol:

- **Safari/WebKit (WKWebView)** - Current MVP implementation
- **Chromium (CEF)** - Deferred (see [docs/cef.md](cef.md) for prior work)
- **Firefox/Gecko** - Longer-term goal

This will allow `termsurf open --browser chromium` or `--browser gecko` for
cross-browser testing.

## CLI-App Communication

### The Problem

TermSurf needs a way for CLI tools (`termsurf open`, etc.) to communicate with
the running TermSurf app to control browser panes.

### Solution: Unix Domain Sockets

We use Unix domain sockets for CLI-to-app communication:

```
┌─────────────────────────────────────────────────────────────┐
│ TermSurf App                                                │
│                                                             │
│  ┌──────────────┐         ┌─────────────────┐              │
│  │ SocketServer │◄────────│ CommandHandler  │              │
│  └──────┬───────┘         └────────┬────────┘              │
│         │                          │                        │
│  ┌──────▼───────────────────────────────────────────────┐  │
│  │ Terminal Pane (shell with env vars)                  │  │
│  │   TERMSURF_SOCKET=/tmp/termsurf-12345.sock           │  │
│  │   TERMSURF_PANE_ID=pane-abc-123                      │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │ Unix Socket (JSON)
                    ┌─────────┴─────────┐
                    │ termsurf CLI      │
                    └───────────────────┘
```

### Why Unix Sockets Over OSC Escape Sequences?

We considered OSC escape sequences (like iTerm2 and Kitty use) but chose Unix
domain sockets for these reasons:

| Aspect | OSC Escape Sequences | Unix Domain Sockets |
|--------|---------------------|---------------------|
| **libghostty changes** | Required (fork) | **None** |
| **Bidirectional** | No | **Yes** |
| **Protocol** | String parsing | **Structured JSON** |
| **Robustness** | Broken by pipes | **Always works** |
| **Blocking** | Not possible | **By default** |

**Key advantages:**

1. **No libghostty modification** - All code lives in `termsurf-macos/`
2. **Bidirectional** - CLI can receive responses and events
3. **Robust** - Works regardless of stdout redirection or piping
4. **Structured** - JSON protocol avoids escaping issues

### Protocol

```json
// Request (CLI → App)
{"id": "1", "action": "open", "paneId": "abc-123", "data": {"url": "https://..."}}

// Response (App → CLI)
{"id": "1", "status": "ok", "data": {"webviewId": "wv-456"}}

// Event (App → CLI, when webview closes)
{"id": "1", "event": "closed", "data": {"exitCode": 0}}
```

### Environment Variables

When TermSurf spawns a shell, it sets:
- `TERMSURF_SOCKET` - Path to the Unix domain socket
- `TERMSURF_PANE_ID` - Unique identifier for this pane

These are inherited by all child processes, allowing the CLI tool to discover
the socket path and identify which pane it's running in.

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
    case browser(WebViewOverlay)  // WKWebView-based
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

### WKWebView Implementation

WKWebView doesn't expose native console access, so we use JavaScript injection:

```javascript
// Injected at document start
['log', 'warn', 'error', 'info', 'debug'].forEach(level => {
    const original = console[level];
    console[level] = function(...args) {
        const message = args.map(a =>
            typeof a === 'object' ? JSON.stringify(a) : String(a)
        ).join(' ');
        window.webkit.messageHandlers.console.postMessage({level, message});
        original.apply(console, args);
    };
});
```

Swift receives messages via `WKScriptMessageHandler`:

```swift
func userContentController(_ controller: WKUserContentController,
                          didReceive message: WKScriptMessage) {
    guard let dict = message.body as? [String: Any],
          let level = dict["level"] as? String,
          let msg = dict["message"] as? String else { return }

    // Route to PTY based on level
    if ["error", "warn"].contains(level) {
        writeToPTY(stderr: "[\\(level)] \\(msg)\\n")
    } else {
        writeToPTY(stdout: "[\\(level)] \\(msg)\\n")
    }
}
```

## Cross-Platform Strategy

### Phase 1: macOS MVP

Focus on macOS first:
- Fork the Swift app (`termsurf-macos/`)
- Add WKWebView browser pane support
- Implement profile isolation via `WKWebsiteDataStore`

### Phase 2: Linux

Apply same patterns to Ghostty's GTK app:
- Create `termsurf-linux/` as fork of GTK app
- Use WebKitGTK for browser panes (similar API to WKWebView)
- Share architectural patterns, adapt for GTK

## File Structure

```
termsurf/
├── src/                          # libghostty (Zig) - shared core (UNMODIFIED)
├── src/termsurf-cli/             # CLI tool (new)
│   └── main.zig                  # Socket client, command parsing
├── macos/                        # Original Ghostty macOS app
├── termsurf-macos/               # TermSurf macOS app
│   ├── Sources/
│   │   ├── App/                  # App delegate, main entry
│   │   ├── Features/
│   │   │   ├── Splits/           # SplitTree (extend for browser panes)
│   │   │   ├── Terminal/         # Terminal views
│   │   │   ├── Socket/           # Unix domain socket server (new)
│   │   │   │   ├── SocketServer.swift
│   │   │   │   ├── SocketConnection.swift
│   │   │   │   ├── TermsurfProtocol.swift
│   │   │   │   └── CommandHandler.swift
│   │   │   └── WebView/          # WebView overlay, manager (new)
│   │   │       ├── WebViewOverlay.swift
│   │   │       ├── WebViewManager.swift
│   │   │       └── ConsoleCapture.swift
│   │   └── Ghostty/              # Ghostty integration
│   └── WebViewKit/               # WKWebView wrapper (prototype)
├── docs/                         # Documentation
│   ├── architecture.md           # This file
│   └── cef.md                    # CEF reference (deferred approach)
└── TODO.md                       # Active task checklist
```

**Key point:** libghostty (`src/`) remains completely unmodified. All TermSurf
functionality is implemented in the Swift app and CLI tool.
