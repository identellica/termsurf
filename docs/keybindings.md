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
   - FooterView is the first responder
   - Pane navigation (ctrl+h/j/k/l) works via responder chain → menu system →
     ghostty actions
   - ctrl+c closes the webview
   - Enter switches to webview mode

2. **Webview mode** (browser has full control)
   - WKWebView is the first responder
   - All keys go to the browser
   - Esc (intercepted via injected JS) switches to footer mode

### Implementation

Footer keybindings are handled in `FooterView.swift`:

- Enter, ctrl+c are intercepted in `keyDown()`
- Other keys pass through `super.keyDown()` to reach the responder chain

Webview keybindings are handled via JavaScript injection in
`WebViewOverlay.swift`:

- Only Esc is intercepted, sent via `postMessage` to Swift

### Why Not Use libghostty?

libghostty's keybinding system requires events to flow through a terminal
surface. When the webview is focused, this path doesn't exist. Rather than fight
the architecture, we accept that webview mode is a separate input context with
its own minimal bindings.

### Current Hardcoded Bindings

| Context | Key    | Action        |
| ------- | ------ | ------------- |
| Footer  | Enter  | Focus webview |
| Footer  | ctrl+c | Close webview |
| Webview | Esc    | Focus footer  |

These are not configurable via ghostty config. This may change in the future if
we add TermSurf-specific configuration.
