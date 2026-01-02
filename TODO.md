# TermSurf Browser Pane Integration Plan

This document tracks the integration of browser panes into TermSurf, enabling
web content alongside terminal sessions.

## Decision: WKWebView for MVP

After extensive work on CEF (Chromium Embedded Framework) integration, we
pivoted to Apple's WKWebView for the MVP due to fundamental Swift-to-C
marshalling issues with CEF's C API.

### Why WKWebView?

| Factor            | WKWebView              | CEF                            |
| ----------------- | ---------------------- | ------------------------------ |
| Setup time        | 15 minutes             | Hours (still failing)          |
| Dependencies      | None (built-in)        | 250MB framework                |
| Console capture   | Works via JS injection | Native callback (if it worked) |
| Swift integration | Native, seamless       | Complex C struct marshalling   |
| DevTools          | Safari Web Inspector   | Chrome DevTools                |
| Rendering engine  | WebKit                 | Blink (Chrome)                 |

**Bottom line:** WKWebView gives us a working browser pane MVP immediately. CEF
remains an option for the future if Chrome DevTools become essential.

### Trade-offs Accepted

- **No Chrome DevTools** - Safari Web Inspector is available instead
- **WebKit only** - Some sites may render differently than Chrome
- **Less customization** - WKWebView is more opaque than CEF

See [docs/cef.md](docs/cef.md) for detailed documentation of the CEF attempt.

---

## Phase 1: WebViewKit Foundation ✓

Create a minimal Swift wrapper for WKWebView with console capture.

### Structure

```
termsurf-macos/WebViewKit/
├── WebViewManager.swift      # Initialization, configuration
├── WebViewController.swift   # WKWebView + console capture
└── ConsoleCapture.swift      # JS injection for console.log/error
```

### Tasks

- [x] Create WKWebView with proper configuration
- [x] Inject JavaScript to intercept console.log, console.warn, console.error
- [x] Implement `WKScriptMessageHandler` to receive console messages
- [x] Route console.log → stdout
- [x] Route console.error → stderr
- [x] Handle object serialization (JSON.stringify)
- [x] Capture uncaught errors via window.onerror

## Phase 2: Standalone Prototype ✓

Test WKWebView integration in isolation before integrating with TermSurf.

- [x] Create WebViewTest macOS app
- [x] Display WKWebView in a window
- [x] Load external URL (google.com)
- [x] Test console message capture:
  - [x] Verify console.log appears on stdout
  - [x] Verify console.error appears on stderr
  - [x] Verify objects are JSON-serialized
- [x] Verify navigation callbacks work

**Result:** WebViewTest app working! Located at `termsurf-macos/WebViewTest/`

## Architecture Decision: OSC Escape Sequences

The webview integration uses OSC (Operating System Command) escape sequences for
communication between the CLI tool and the TermSurf app. This approach:

- Uses the existing PTY connection (no separate IPC mechanism)
- Follows precedent from iTerm2, Kitty, and other modern terminals
- Keeps the CLI tool simple (just writes escape sequences)
- Allows console output to flow naturally to the terminal

**Protocol:**

```
CLI → App:  \x1b]termsurf;open;https://google.com\x07
CLI → App:  \x1b]termsurf;show;wv-123\x07
App → PTY:  Console output written directly to PTY master
App → PTY:  0x03 (ctrl+c) or 0x1a (ctrl+z) relayed to shell
```

**Webview as overlay:** The webview renders on top of the terminal pane (which
continues to exist underneath). This allows:

- `ctrl+c` to close webview and signal the CLI tool
- `ctrl+z` to hide webview and background the CLI (shell job control)
- `fg` to restore the webview (CLI sends show command on SIGCONT)
- Console output to accumulate in the terminal underneath

---

## Phase 3: TermSurf Integration

Integrate webview support into TermSurf via OSC escape sequences and a CLI tool.

### Phase 3A: OSC Handler Foundation

**Goal:** Add custom OSC handler `\x1b]termsurf;...\x07` bridging Zig → Swift.

**Files to modify:**

- `src/terminal/osc.zig` - Add `termsurf` command type
- `src/terminal/stream.zig` - Add dispatch in `oscDispatch()`
- `src/apprt/action.zig` - Add action type
- `include/ghostty.h` - Add C API types
- `termsurf-macos/Sources/Ghostty/Ghostty.App.swift` - Handle callback

**Tasks:**

- [ ] Define `TermsurfCommand` struct in `osc.zig`:
  - [ ] Actions: `ping`, `open`, `close`, `show`, `hide`
  - [ ] Data field for URL or webview ID
- [ ] Add OSC parsing for `termsurf;...` format
- [ ] Add dispatch in `stream.zig` `oscDispatch()` function
- [ ] Add `termsurf_command` action type in `action.zig`
- [ ] Add C struct `ghostty_action_termsurf_s` in `ghostty.h`
- [ ] Handle `GHOSTTY_ACTION_TERMSURF_COMMAND` in Swift

**Test:**

```bash
# In TermSurf terminal:
printf '\x1b]termsurf;ping\x07'
# Expected: "TermSurf: received ping" in Xcode console
```

### Phase 3B: CLI Tool Foundation

**Goal:** Create `termsurf` CLI tool in Zig that sends OSC commands.

**Files to create:**

- `src/termsurf-cli/main.zig` - CLI entry point
- Update `build.zig` - Add build target

**Tasks:**

- [ ] Create `src/termsurf-cli/` directory
- [ ] Implement argument parsing:
  - [ ] `termsurf ping` - Test connectivity
  - [ ] `termsurf open [--profile NAME] URL` - Open webview
- [ ] Write OSC escape sequences to stdout
- [ ] Add `termsurf-cli` build target to `build.zig`
- [ ] Prepend `https://` to URLs without scheme

**Test:**

```bash
zig build termsurf-cli
./zig-out/bin/termsurf ping
# Expected: Same log as Phase 3A

./zig-out/bin/termsurf open google.com
# Expected: Log showing "termsurf open: https://google.com"
```

### Phase 3C: Webview Overlay

**Goal:** Show WKWebView overlay on terminal pane when receiving `open` command.

**Files to create/modify:**

- `termsurf-macos/Sources/Features/WebView/WebViewOverlay.swift` (new)
- `termsurf-macos/Sources/Features/WebView/WebViewManager.swift` (new)
- Terminal view files to support overlay

**Tasks:**

- [ ] Create `WebViewOverlay` class (extract/adapt from WebViewTest):
  - [ ] WKWebView with console capture JS injection
  - [ ] Navigation delegate for load events
- [ ] Create `WebViewManager` singleton to track active webviews
- [ ] Implement overlay display on terminal pane:
  - [ ] Add webview as subview on top of terminal
  - [ ] Handle pane resizing
- [ ] Handle `open` action in Swift callback:
  - [ ] Create WebViewOverlay with URL
  - [ ] Add to correct pane
  - [ ] Give webview focus

**Test:**

```bash
./zig-out/bin/termsurf open google.com
# Expected: Google.com appears overlaid on terminal
# Can scroll and click in webview
```

### Phase 3D: Console Output Bridging

**Goal:** Route webview console.log/error to the terminal's PTY.

**Tasks:**

- [ ] Get PTY file descriptor for the pane hosting the webview
- [ ] Modify console message handler to write to PTY instead of stdout:
  - [ ] Format: `[log] message\n`, `[error] message\n`, etc.
- [ ] Handle high-frequency console output (buffering if needed)

**Test:**

```bash
./zig-out/bin/termsurf open https://example.com
# Open Safari Web Inspector, run: console.log("Hello")
# Close webview manually
# Expected: Terminal shows "[log] Hello"
```

### Phase 3E: Close via ctrl+c

**Goal:** Intercept ctrl+c in webview, close webview, relay signal to terminal.

**Tasks:**

- [ ] Add JS key interception for ctrl+c:
  ```javascript
  document.addEventListener('keydown', (e) => {
      if (e.ctrlKey && e.key === 'c') {
          e.preventDefault();
          window.webkit.messageHandlers.termsurf.postMessage({action: 'close'});
      }
  }, true);
  ```
- [ ] Register `termsurf` message handler in WKUserContentController
- [ ] Handle close message in Swift:
  - [ ] Remove webview overlay
  - [ ] Write `0x03` (ctrl+c byte) to PTY master
  - [ ] Return focus to terminal
- [ ] CLI tool exits naturally on SIGINT (default behavior)

**Test:**

```bash
./zig-out/bin/termsurf open google.com
# Webview appears
# Press ctrl+c
# Expected: Webview closes, terminal prompt returns
```

### Phase 3F: Background/Foreground (ctrl+z / fg)

**Goal:** ctrl+z hides webview and backgrounds CLI, fg restores both.

**Tasks:**

- [ ] Add JS key interception for ctrl+z
- [ ] Handle background message in Swift:
  - [ ] Hide webview (set `isHidden = true`, don't destroy)
  - [ ] Write `0x1a` (ctrl+z byte) to PTY master
  - [ ] Return focus to terminal
- [ ] CLI tool: Add SIGCONT signal handler:
  - [ ] On SIGCONT, write `\x1b]termsurf;show;{webview_id}\x07`
- [ ] Handle `show` action in Swift:
  - [ ] Find webview by ID
  - [ ] Set `isHidden = false`
  - [ ] Give webview focus
- [ ] CLI tool: Track webview ID received from app

**Test:**

```bash
./zig-out/bin/termsurf open google.com
# Press ctrl+z
# Expected: Webview hides, shell shows "[1]+ Stopped ..."

fg
# Expected: Webview reappears with same page
```

### Phase 3G: Multi-webview Tracking

**Goal:** Support multiple webviews across panes, each with correct association.

**Tasks:**

- [ ] CLI generates unique webview ID on open
- [ ] Include ID in OSC commands: `termsurf;open;{id};{url}`
- [ ] Determine source pane from PTY (map PTY fd → pane)
- [ ] WebViewManager tracks:
  - [ ] Webview ID → WebViewOverlay instance
  - [ ] Webview ID → Pane association
- [ ] Ensure ctrl+c/z only affects focused webview
- [ ] Ensure pane switching (cmd+[, cmd+]) works with webviews

**Test:**

```bash
# Open two panes side by side
# Left pane: ./zig-out/bin/termsurf open google.com
# Right pane: ./zig-out/bin/termsurf open github.com
# Expected: Each pane has its own webview
# ctrl+c in left only closes left webview
# cmd+[ and cmd+] switch between them
```

### Phase 3 Summary

| Phase | Goal                    | Test                              | Success Criteria           |
| ----- | ----------------------- | --------------------------------- | -------------------------- |
| 3A    | OSC bridge              | `printf '\x1b]termsurf;ping\x07'` | Log in Xcode               |
| 3B    | CLI tool                | `termsurf ping`                   | Same log via CLI           |
| 3C    | Webview overlay         | `termsurf open google.com`        | Webview appears            |
| 3D    | Console bridging        | console.log in webview            | Output in terminal         |
| 3E    | ctrl+c close            | Press ctrl+c                      | Webview closes, CLI exits  |
| 3F    | ctrl+z / fg             | ctrl+z then fg                    | Hide/restore works         |
| 3G    | Multi-webview           | Open in two panes                 | Independent operation      |

### Key Files Reference

**Zig core (libghostty):**

- `src/terminal/osc.zig` - OSC command definitions
- `src/terminal/stream.zig` - Stream handler dispatch
- `src/apprt/action.zig` - Action definitions
- `include/ghostty.h` - C API types

**Zig CLI:**

- `src/termsurf-cli/main.zig` (new)

**Swift app:**

- `termsurf-macos/Sources/Ghostty/Ghostty.App.swift` - Action callback
- `termsurf-macos/Sources/Features/WebView/WebViewOverlay.swift` (new)
- `termsurf-macos/Sources/Features/WebView/WebViewManager.swift` (new)

## Phase 4: Polish & Features

### Profile/Session Isolation

WKWebView supports named profiles via `WKWebsiteDataStore(forIdentifier:)` (macOS 14+).
Each profile gets completely isolated: cookies, localStorage, IndexedDB, cache, service workers.

**Requires:** macOS 14+ (Sonoma) - acceptable for target audience (web developers)

```swift
// Example implementation
func createWebView(profileName: String?) -> WKWebView {
    let config = WKWebViewConfiguration()

    if let name = profileName {
        // Deterministic UUID from profile name
        let identifier = UUID(name.utf8)  // simplified
        config.websiteDataStore = WKWebsiteDataStore(forIdentifier: identifier)
    } else {
        config.websiteDataStore = .default()
    }

    return WKWebView(frame: .zero, configuration: config)
}
```

- [ ] Implement profile support:
  - [ ] Map profile names to deterministic UUIDs
  - [ ] Store profile name → UUID mapping (or use hash-based generation)
  - [ ] Default profile uses `.default()` data store
  - [ ] Named profiles use `WKWebsiteDataStore(forIdentifier:)`
  - [ ] Support `--profile NAME` flag in `termsurf open` command
  - [ ] Consider `--incognito` flag using `.nonPersistent()` for ephemeral sessions
- [ ] Profile management:
  - [ ] List existing profiles
  - [ ] Delete profile data (`WKWebsiteDataStore.remove(forIdentifier:)`)

### Developer Tools

- [ ] Enable Safari Web Inspector for WKWebView:
  - [ ] Set
        `webView.configuration.preferences.setValue(true, forKey: "developerExtrasEnabled")`
  - [ ] Document how to access (Develop menu in Safari)
- [ ] Consider command: `termsurf devtools` to open inspector

### Additional Features

- [ ] User agent customization
- [ ] JavaScript injection API for automation
- [ ] Download handling
- [ ] Permission prompts (camera, microphone, location)

### Documentation

- [ ] Update ARCHITECTURE.md with browser pane details
- [ ] Document console bridging behavior
- [ ] Document profile system (if implemented)
- [ ] Document keyboard shortcuts
- [ ] Add usage examples to README

---

## CEF Integration (Deferred)

CEF integration is deferred due to Swift-to-C marshalling issues. The work is
preserved for potential future use.

### What Was Completed

- [x] CEF framework downloaded and configured (v143.0.13)
- [x] Module map created for Swift import
- [x] Helper apps configured with correct bundle IDs
- [x] CEFKit wrapper structure created
- [x] Marshaller pattern implemented (based on CEF.swift)

### What Failed

The `cef_initialize()` function rejects our `cef_app_t` struct with:

```
CefApp_0_CToCpp called with invalid version -1
```

Despite correct struct sizes and callback assignments, CEF's validation fails.
See [docs/cef.md](docs/cef.md) for detailed analysis.

### Cleanup Completed ✓

All CEF code has been removed:

- [x] `termsurf-macos/CEFKit/` - Deleted
- [x] `termsurf-macos/CEFTest/` - Deleted
- [x] `termsurf-macos/Frameworks/cef/` - Deleted (~1.2GB)
- [x] `CEF.swift/` - Deleted (reference project)
- [x] `scripts/setup-cef.sh` - Deleted
- [x] Xcode project references - Removed

### Future CEF Options

If CEF is revisited:

1. Try C++ API with Objective-C++ bridging header
2. Test older CEF versions
3. Write integration layer in Objective-C
4. Check if Swift ABI changes affected the marshaller pattern

---

## Resources

### WKWebView

- [WKWebView Documentation](https://developer.apple.com/documentation/webkit/wkwebview)
- [WKScriptMessageHandler](https://developer.apple.com/documentation/webkit/wkscriptmessagehandler)
- [WKUserContentController](https://developer.apple.com/documentation/webkit/wkusercontentcontroller)
- [WKWebsiteDataStore](https://developer.apple.com/documentation/webkit/wkwebsitedatastore)

### CEF (Deferred)

- [CEF Integration Guide](docs/cef.md) - Our detailed documentation
- [CEF Builds](https://cef-builds.spotifycdn.com/index.html) - Binary
  distributions
- [CEF Wiki](https://bitbucket.org/chromiumembedded/cef/wiki/Home) - General
  guide

---

## Notes

- WebViewTest app is the working prototype - use as reference for integration
- Console capture uses JS injection; native console isn't accessible
- WKWebView works best when app is properly signed (some features restricted
  otherwise)
- Safari Web Inspector requires "Develop" menu enabled in Safari preferences
