# Keybindings Architecture

## libghostty Keybindings

libghostty (the Zig core) owns the keybinding system for terminal operations:

1. **Config parsing** - Keybindings defined in `~/.config/ghostty/config` (e.g.,
   `keybind = ctrl+t=new_tab`)
2. **Action dispatch** - When a key is pressed in a terminal surface, libghostty
   matches it against bindings and fires an action
3. **App runtime handles actions** - Swift receives action callbacks (e.g.,
   `GHOSTTY_ACTION_NEW_TAB`) and implements the behavior

Key files:

- `src/config/Config.zig` - Keybinding config parsing
- `src/input/Binding.zig` - Trigger-to-action mapping
- `src/apprt/action.zig` - Action enum (quit, new_tab, goto_split, etc.)
- `termsurf-macos/Sources/Ghostty/Ghostty.App.swift` - Action handlers

This system assumes keyboard input flows through a terminal surface, which
passes events to libghostty.

## TermSurf Webview Keybindings

Webviews introduce a problem: when WKWebView is focused, keyboard events go to
the browser, not libghostty. We handle this with a **modal approach**:

### Two Modes

1. **Footer mode** (terminal keybindings work)
   - SurfaceView is the first responder
   - All ghostty keybindings work naturally (pane navigation, splits, etc.)
   - Enter switches to webview mode
   - ctrl+c closes the webview
   - FooterView displays: "Enter to browse, ctrl+c to close"

2. **Webview mode** (browser has full control)
   - WKWebView is the first responder
   - All keys go to the browser
   - Esc (intercepted via injected JS) switches to footer mode
   - FooterView displays: "Esc to exit browser"

### Implementation

**Footer mode** keybindings are handled in `SurfaceView_AppKit.swift`:

- At the start of `keyDown()`, check if a WebViewContainer subview exists
- If so, intercept Enter and ctrl+c before passing to libghostty
- All other keys flow through to libghostty normally

**Webview mode** keybindings are handled via JavaScript injection in
`WebViewOverlay.swift`:

- Only Esc is intercepted, sent via `postMessage` to Swift
- Swift calls `onEscapePressed` callback → `focusFooter()`

**FooterView** (`FooterView.swift`) is visual-only:

- Displays mode-specific hint text
- No keyboard handling

### Focus State Synchronization

When switching between panes, ghostty makes the target pane's SurfaceView the
first responder. If returning to a pane with a webview, WebViewContainer's
internal `focusMode` may be stale (still set to `.webview` from before).

This is handled in `SurfaceView_AppKit.swift`:

```swift
if let container = subviews.first(where: { $0 is WebViewContainer }) ... {
    // If SurfaceView is receiving keys but container thinks it's in webview mode,
    // sync the state
    if !container.isFooterMode {
        container.syncToFooterMode()
    }
    // Then handle Enter/ctrl+c
}
```

`syncToFooterMode()` updates the internal state and footer text without changing
the first responder (since SurfaceView already has focus).

### Why Keep SurfaceView as First Responder?

The key insight: keeping SurfaceView as first responder in footer mode means
**all ghostty keybindings work automatically**. Events flow through SurfaceView
→ libghostty → action dispatch, just like a normal terminal pane.

Previous attempts to make FooterView the first responder required forwarding key
events back to SurfaceView, which broke due to focus guards in the responder
chain.

### Current Hardcoded Bindings

| Context | Key    | Action        |
| ------- | ------ | ------------- |
| Footer  | Enter  | Focus webview |
| Footer  | ctrl+c | Close webview |
| Webview | Esc    | Focus footer  |

These are not configurable via ghostty config. This may change in the future if
we add TermSurf-specific configuration.
