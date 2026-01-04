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

### Three Modes

1. **Control mode** (terminal keybindings work)
   - SurfaceView is the first responder
   - All ghostty keybindings work naturally (pane navigation, splits, etc.)
   - Enter switches to browse mode
   - i switches to insert mode (edit URL)
   - ctrl+c closes the webview
   - ControlBar displays: "i to edit, enter to browse, ctrl+c to close"

2. **Browse mode** (browser has full control)
   - WKWebView is the first responder
   - All keys go to the browser
   - Esc (intercepted via injected JS) switches to control mode
   - ControlBar displays: "Esc to exit browser"

3. **Insert mode** (edit URL)
   - URL text field is the first responder
   - Normal text editing controls work (arrow keys, selection, etc.)
   - URL is selected by default when entering insert mode
   - Enter navigates to the URL and switches to browse mode
   - Esc cancels editing, restores original URL, switches to control mode
   - ControlBar displays: "Enter to go, Esc to cancel"

### Implementation

**Control mode** keybindings are handled in `SurfaceView_AppKit.swift`:

- At the start of `keyDown()`, check if a WebViewContainer subview exists
- If so, intercept Enter, i, and ctrl+c before passing to libghostty
- All other keys flow through to libghostty normally

**Browse mode** keybindings are handled via JavaScript injection in
`WebViewOverlay.swift`:

- Only Esc is intercepted, sent via `postMessage` to Swift
- Swift calls `onEscapePressed` callback → `focusControlBar()`

**Insert mode** keybindings are handled in `ControlBar.swift`:

- ControlBar implements `NSTextFieldDelegate`
- `control(_:textView:doCommandBy:)` intercepts Enter and Esc
- Enter triggers `onURLSubmitted` callback with the edited URL string
- Esc triggers `onInsertCancelled` callback and restores the original URL
- WebViewContainer wires these callbacks to navigate and switch modes

**ControlBar** (`ControlBar.swift`):

- Displays URL on the left (monospace font, truncates with ellipsis)
- Displays mode-specific hint text on the right
- In insert mode, URL field becomes editable with text selected

### Focus State Synchronization

When switching between panes, ghostty makes the target pane's SurfaceView the
first responder. If returning to a pane with a webview, WebViewContainer's
internal `focusMode` may be stale (still set to `.browse` from before).

This is handled in `SurfaceView_AppKit.swift`:

```swift
if let container = subviews.first(where: { $0 is WebViewContainer }) ... {
    // If SurfaceView is receiving keys but container thinks it's in browse mode,
    // sync the state
    if !container.isControlMode {
        container.syncToControlMode()
    }
    // Then handle Enter/i/ctrl+c
}
```

`syncToControlMode()` updates the internal state and control bar text without
changing the first responder (since SurfaceView already has focus).

### Why Keep SurfaceView as First Responder?

The key insight: keeping SurfaceView as first responder in control mode means
**all ghostty keybindings work automatically**. Events flow through SurfaceView
→ libghostty → action dispatch, just like a normal terminal pane.

Previous attempts to make a separate view the first responder required
forwarding key events back to SurfaceView, which broke due to focus guards in
the responder chain.

### URL Normalization

When a URL is submitted from insert mode, it is normalized before navigation:

- URLs without a scheme (e.g., `example.com`) get `https://` prepended
- URLs with `http://`, `https://`, or `file://` are used as-is

### Current Hardcoded Bindings

| Context | Key       | Action                            |
| ------- | --------- | --------------------------------- |
| Control | Enter     | Switch to browse                  |
| Control | i         | Switch to insert (edit URL)       |
| Control | ctrl+c    | Close webview                     |
| Browse  | Esc       | Switch to control                 |
| Browse  | cmd+c     | Copy (via menu action)            |
| Browse  | cmd+x     | Cut (via menu action)             |
| Browse  | cmd+v     | Paste (via menu action)           |
| Browse  | cmd+alt+i | Open Safari Web Inspector         |
| Insert  | Enter     | Navigate to URL, switch to browse |
| Insert  | Esc       | Cancel edit, switch to control    |

These are not configurable via ghostty config. This may change in the future if
we add TermSurf-specific configuration.

## AppKit Keyboard Event Types

AppKit has two completely separate code paths for keyboard events:

1. **`keyDown`** - Regular key events (letters, arrows, shift+arrow, escape, etc.)
2. **`performKeyEquivalent`** - Command-key events (cmd+c, cmd+v, cmd+a, etc.)

Understanding this distinction is critical for handling keyboard input when both
terminal and webview need to coexist.

### keyDown Flow

```
User presses key (e.g., shift+arrow)
    ↓
First responder receives keyDown
    ↓
If not handled, bubbles up responder chain
```

### performKeyEquivalent Flow

```
User presses cmd+key (e.g., cmd+c)
    ↓
First responder receives performKeyEquivalent
    ↓
If returns true → event consumed
If returns false → bubbles up, eventually becomes menu action
```

## SurfaceView Key Handling Implementation

When a webview is visible, SurfaceView must decide which keys to handle itself
(for terminal) vs which to let the webview handle.

### Regular Keys (keyDown)

In `SurfaceView_AppKit.swift`, `keyDown` checks for webview presence:

```swift
if let container = subviews.last(where: { $0 is WebViewContainer }) ... {
    if container.isControlMode {
        // Handle control mode special keys: Enter, i, ctrl+c
        // ...
    }
    // Webview visible - return early, let first responder (webview) handle
    return
}
// No webview - send to terminal via libghostty
```

This works because WKWebView correctly handles regular key events when it's the
first responder.

### Command Keys (performKeyEquivalent)

Command keys are trickier due to a **WKWebView quirk**:

> WKWebView's `performKeyEquivalent` claims cmd+c/x/v (returns `true`) but
> doesn't actually execute the copy/cut operation. However, WKWebView's `copy:`
> action method works correctly when triggered via the Edit menu.

The workaround is to intercept cmd+c/x/v and convert them to menu actions:

```swift
override func performKeyEquivalent(with event: NSEvent) -> Bool {
    if let container = subviews.last(where: { $0 is WebViewContainer }) ... {
        if hasCmd && !hasOpt {
            switch char {
            case "c":
                NSApp.sendAction(#selector(NSText.copy(_:)), to: nil, from: self)
                return true
            case "x":
                NSApp.sendAction(#selector(NSText.cut(_:)), to: nil, from: self)
                return true
            case "v":
                NSApp.sendAction(#selector(NSText.paste(_:)), to: nil, from: self)
                return true
            default:
                break
            }
        }
        // Other cmd+keys: return false to let system handle
        return false
    }
    // No webview - send to terminal
}
```

### Menu Item Validation

To ensure the menu action reaches WKWebView (not SurfaceView), we also return
`false` from `validateMenuItem` for copy/cut/paste when a webview is visible:

```swift
func validateMenuItem(_ item: NSMenuItem) -> Bool {
    if subviews.contains(where: { $0 is WebViewContainer }) {
        switch item.action {
        case #selector(copy(_:)), #selector(cut(_:)), #selector(paste(_:)):
            return false  // Don't claim these - let webview handle
        default:
            break
        }
    }
    // ... rest of validation
}
```

This tells AppKit "SurfaceView can't handle copy/cut/paste right now" so the
action continues down the responder chain to WKWebView.

## Pattern for Future Keybindings

When adding new keybindings that need to work in webviews:

1. **Regular keys** - Let first responder handle by returning early from `keyDown`
2. **Command keys that WKWebView handles correctly** - Return `false` from
   `performKeyEquivalent` to let them flow normally
3. **Command keys that WKWebView breaks** - Intercept in `performKeyEquivalent`
   and convert to `NSApp.sendAction` to trigger the menu action directly
