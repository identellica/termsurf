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

## Architecture Decision: Unix Domain Sockets

The webview integration uses Unix domain sockets for communication between the
CLI tool and the TermSurf app. This approach was chosen over OSC escape
sequences after careful analysis.

### Why Unix Sockets Over Escape Sequences?

| Aspect                 | OSC Escape Sequences | Unix Domain Sockets    |
| ---------------------- | -------------------- | ---------------------- |
| **libghostty changes** | Required (fork)      | **None**               |
| **Bidirectional**      | No                   | **Yes**                |
| **Protocol**           | String parsing       | **Structured JSON**    |
| **Robustness**         | Broken by pipes      | **Always works**       |
| **Error handling**     | Silent failures      | **Explicit responses** |
| **`--wait` support**   | Not possible         | **Supported**          |

**Key advantages of sockets:**

1. **No libghostty modification** - All code lives in `termsurf-macos/`
2. **Bidirectional communication** - CLI can receive responses and events
3. **Structured protocol** - JSON avoids escaping issues, easy to extend
4. **Robust** - Works regardless of stdout redirection or piping
5. **`--wait` flag** - CLI can block until webview closes

### Protocol

```
┌─────────────────────────────────────────────────────────────┐
│ TermSurf App                                                │
│                                                             │
│  ┌──────────────┐         ┌─────────────────┐              │
│  │ SocketServer │◄────────│ CommandHandler  │              │
│  │ (listener)   │         └────────┬────────┘              │
│  └──────┬───────┘                  │                        │
│         │                   ┌──────▼───────┐               │
│         │                   │ WebViewMgr   │               │
│         │                   └──────────────┘               │
│  ┌──────▼───────────────────────────────────────────────┐  │
│  │ Terminal Pane (shell with env vars)                  │  │
│  │   TERMSURF_SOCKET=/tmp/termsurf-12345.sock           │  │
│  │   TERMSURF_PANE_ID=pane-abc-123                      │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │ Unix Socket (JSON)
                              │
                    ┌─────────┴─────────┐
                    │ termsurf CLI      │
                    │ (reads env vars,  │
                    │  sends commands)  │
                    └───────────────────┘
```

**Message format:**

```json
// Request (CLI → App)
{"id": "1", "action": "open", "paneId": "abc-123", "data": {"url": "https://..."}}

// Response (App → CLI)
{"id": "1", "status": "ok", "data": {"webviewId": "wv-456"}}

// Event (App → CLI, for --wait)
{"id": "1", "event": "closed", "data": {"exitCode": 0}}
```

**Webview as overlay:** The webview renders on top of the terminal pane (which
continues to exist underneath). This allows:

- `ctrl+c` to close webview and notify waiting CLI
- `ctrl+z` to hide webview and background the CLI (shell job control)
- `fg` to restore the webview (CLI sends show command on SIGCONT)
- Console output to accumulate in the terminal underneath

---

## Phase 3: TermSurf Integration

Integrate webview support into TermSurf via Unix domain sockets and a CLI tool.

**No libghostty changes required!** All code lives in `termsurf-macos/` and the
CLI tool.

### Phase 3A: Socket Server Foundation ✓

**Goal:** Create Unix socket server, set env vars on shell spawn, verify
connectivity.

**New files in `termsurf-macos/Sources/`:**

```
Features/Socket/
├── SocketServer.swift       # Unix domain socket listener
├── SocketConnection.swift   # Handle individual client connections
├── TermsurfProtocol.swift   # JSON message types (Codable structs)
├── TermsurfEnvironment.swift # Inject env vars into surface configs
└── CommandHandler.swift     # Route commands to handlers
```

**Tasks:**

- [x] Create `SocketServer` class:
  - [x] Create socket at `/tmp/termsurf-{pid}.sock`
  - [x] Listen for connections using `Darwin.socket()`, `bind()`, `listen()`
  - [x] Accept connections on background queue
  - [x] Clean up socket on app termination

- [x] Create `TermsurfProtocol.swift` with Codable message types:
  ```swift
  struct Request: Codable {
      let id: String
      let action: String    // "ping", "open", "close", "show", "hide"
      let paneId: String?
      let data: [String: AnyCodable]?
  }

  struct Response: Codable {
      let id: String
      let status: String    // "ok", "error"
      let data: [String: AnyCodable]?
      let error: String?
  }

  struct Event: Codable {
      let id: String
      let event: String     // "closed", "backgrounded"
      let data: [String: AnyCodable]?
  }
  ```

- [x] Create `CommandHandler` with `ping` handler:
  ```swift
  func handle(_ request: Request) -> Response
  ```

- [x] Modify shell spawning to set environment variables:
  - [x] Find where shell is spawned (surface configuration)
  - [x] Add `TERMSURF_SOCKET` with socket path
  - [x] Add `TERMSURF_PANE_ID` with unique pane identifier

**Test:** ✓

```bash
# In TermSurf terminal:
echo $TERMSURF_SOCKET
# Expected: /tmp/termsurf-12345.sock

echo $TERMSURF_PANE_ID
# Expected: pane-abc-123

# Test with netcat:
echo '{"id":"1","action":"ping"}' | nc -U $TERMSURF_SOCKET
# Expected: {"id":"1","status":"ok","data":{"pong":true}}
```

### Phase 3B: CLI Tool Foundation ✓

**Goal:** Create `termsurf` CLI tool that communicates via Unix socket.

**Files created:**

- `src/termsurf-cli/main.zig` - CLI entry point
- `build.zig` - Added `termsurf-cli` build target

**Tasks:**

- [x] Create `src/termsurf-cli/` directory
- [x] Implement socket client:
  - [x] Read `TERMSURF_SOCKET` and `TERMSURF_PANE_ID` env vars
  - [x] Connect to Unix socket
  - [x] Send JSON request, receive JSON response
- [x] Implement subcommands:
  - [x] `termsurf ping` - Test connectivity
  - [x] `termsurf open [--wait] [--profile NAME] URL` - Open webview
  - [x] `termsurf close [WEBVIEW_ID]` - Close webview
- [x] Error handling:
  - [x] `TERMSURF_SOCKET` not set → "Not running inside TermSurf"
  - [x] Socket connection failed → "TermSurf not running"
  - [x] Display error messages from response
- [x] Add `termsurf-cli` build target to `build.zig`
- [x] Prepend `https://` to URLs without scheme

**Test:** ✓

```bash
zig build termsurf-cli
./zig-out/bin/termsurf ping
# Expected: pong

./zig-out/bin/termsurf open google.com
# Expected: Opened webview wv-123

# Outside TermSurf:
./zig-out/bin/termsurf ping
# Expected: Error: Not running inside TermSurf (TERMSURF_SOCKET not set)
```

### Phase 3C: Webview Overlay ✓

**Goal:** Handle `open` command from socket, display WKWebView overlay.

**Files created:**

```
Features/WebView/
├── WebViewOverlay.swift     # WKWebView with console capture
└── WebViewManager.swift     # Track active webviews by ID + pane registry
```

**Tasks:**

- [x] Add `open` handler to `CommandHandler`:
  - [x] Parse URL and options from request
  - [x] Find target pane by `paneId`
  - [x] Create `WebViewOverlay` with URL
  - [x] Add overlay to pane
  - [x] Return `webviewId` in response

- [x] Create `WebViewManager` singleton:
  - [x] Track webviews by ID
  - [x] Track pane ID → SurfaceView associations
  - [x] Generate unique webview IDs

- [x] Create `WebViewOverlay` (adapted from WebViewTest):
  - [x] WKWebView with configuration
  - [x] Console capture JS injection
  - [x] Navigation delegate for load events
  - [x] Keyboard interception for ctrl+c/ctrl+z
  - [x] Profile isolation support (macOS 14+)
  - [x] Safari Web Inspector enabled

- [x] Handle `close` command:
  - [x] Find webview by ID
  - [x] Remove from pane

- [x] Modified pane registration:
  - [x] `TermsurfEnvironment.injectEnvVars()` now returns pane ID
  - [x] `TermsurfEnvironment.registerSurface()` registers with WebViewManager
  - [x] Updated `BaseTerminalController` and `QuickTerminalController`

**Test:**

```bash
termsurf open google.com
# Expected: Google.com appears overlaid on terminal
# Response: {"id":"1","status":"ok","data":{"webviewId":"wv-123"}}
```

### Phase 3D: Footer-Based Mode Switching + ctrl+c

**Goal:** Add footer-based UI with mode switching. ctrl+c closes webview when
footer is focused.

**Architecture:**

```
WebViewContainer (NSView)
├── WebViewOverlay (fills most of container)
└── FooterView (bottom strip, ~24px)
```

- Two modes: footer focused (terminal mode) vs webview focused (browser mode)
- Footer focused by default when webview opens
- In footer mode: all terminal keybindings work naturally (ctrl+c, ctrl+h/j/k/l)
- In webview mode: browser has full control, only Esc escapes

**Tasks:**

- [ ] Create `WebViewContainer.swift`:
  - [ ] Contains WebViewOverlay (top) + FooterView (bottom)
  - [ ] Tracks current focus mode (footer vs webview)
  - [ ] Manages focus transitions between footer and webview

- [ ] Create `FooterView.swift`:
  - [ ] Simple NSView with "TermSurf Browser" label (placeholder for future URL bar)
  - [ ] Accepts first responder
  - [ ] keyDown handles Enter → focus webview
  - [ ] keyDown handles ctrl+c → close container (via callback)

- [ ] Update `WebViewOverlay.swift`:
  - [ ] Remove ctrl+c and ctrl+z JS interception (no longer needed)
  - [ ] Add Esc JS interception → notify container to focus footer
  - [ ] Add callback `onEscapePressed` for mode switching

- [ ] Update `WebViewManager.swift`:
  - [ ] Create WebViewContainer instead of raw WebViewOverlay
  - [ ] Track webviewId → paneId for focus restoration
  - [ ] closeWebView() removes container and restores focus to terminal

- [ ] Add visual mode indicator:
  - [ ] Footer dimmed (alphaValue ~0.5) when webview is focused
  - [ ] Footer normal (alphaValue 1.0) when footer is focused
  - [ ] Webview always stays at full opacity (user needs to see content)

**Test:**

```bash
termsurf open google.com
# Footer shows "TermSurf Browser", footer is focused (not dimmed)
# Press Enter → webview is focused, footer dims
# Press Esc → footer is focused again, footer undims
# Press ctrl+c → webview closes, terminal has focus
# Can also press ctrl+h/j/k/l to navigate panes (when footer focused)
```

### Phase 3E: Verify Split Pane Navigation

**Goal:** Verify that pane navigation works with the footer-based approach.

**Background:** With the footer-based approach, when footer is focused, all
terminal keybindings should flow through the normal responder chain. This phase
verifies that pane navigation "just works" without additional code.

**Tasks:**

- [ ] Test ctrl+h/j/k/l navigation between terminal and webview panes
- [ ] Verify focus moves correctly in both directions
- [ ] If issues found, debug responder chain

**Test:**

```bash
# Open split pane (cmd+d), open webview in left pane
termsurf open google.com
# Footer should be focused by default
# Press ctrl+l to move to right pane (terminal)
# Press ctrl+h to move back to left pane (webview footer)
# Expected: Focus moves correctly between panes
```

**Note:** If this works without changes, mark complete. Future work will ensure
all user-configured keybindings work.

### Phase 3F: Console Output Bridging

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

### Phase 3G: Background/Foreground (ctrl+z / fg)

**Goal:** ctrl+z hides webview and backgrounds CLI, fg restores both.

**Tasks:**

- [ ] Add JS key interception for ctrl+z
- [ ] Handle background message in Swift:
  - [ ] Hide webview (set `isHidden = true`, don't destroy)
  - [ ] Write `0x1a` (ctrl+z byte) to PTY master
  - [ ] Send `{"event":"backgrounded"}` to waiting CLI
  - [ ] Return focus to terminal
- [ ] CLI tool: Add SIGCONT signal handler:
  - [ ] On SIGCONT, send `{"action":"show","data":{"webviewId":"..."}}` via
        socket
- [ ] Handle `show` command in Swift:
  - [ ] Find webview by ID
  - [ ] Set `isHidden = false`
  - [ ] Give webview focus
  - [ ] Return success response
- [ ] CLI tool: Track webview ID from initial `open` response

**Test:**

```bash
termsurf open google.com --wait
# Press ctrl+z
# Expected: Webview hides, shell shows "[1]+ Stopped ..."

fg
# Expected: Webview reappears with same page
```

### Phase 3H: Multi-webview Tracking

**Goal:** Support multiple webviews across panes, each with correct association.

**Tasks:**

- [ ] Each pane gets unique `TERMSURF_PANE_ID` env var
- [ ] CLI includes `paneId` in all requests (from env var)
- [ ] `WebViewManager` tracks:
  - [ ] Webview ID → WebViewOverlay instance
  - [ ] Webview ID → Pane association
  - [ ] Pane ID → Active webview (for keyboard routing)
- [ ] Ensure ctrl+c/z only affects webview in focused pane
- [ ] Ensure pane navigation keybindings work with webviews

**Test:**

```bash
# Open two panes side by side
# Left pane: termsurf open google.com
# Right pane: termsurf open github.com
# Expected: Each pane has its own webview
# ctrl+c in left only closes left webview
# Pane navigation keybindings switch between them
```

### Phase 3 Summary

| Phase | Goal                  | Test                                                          | Success Criteria          | Status |
| ----- | --------------------- | ------------------------------------------------------------- | ------------------------- | ------ |
| 3A    | Socket server         | `echo '{"id":"1","action":"ping"}' \| nc -U $TERMSURF_SOCKET` | JSON response             | ✓      |
| 3B    | CLI tool              | `termsurf ping`                                               | "pong" output             | ✓      |
| 3C    | Webview overlay       | `termsurf open google.com`                                    | Webview appears           | ✓      |
| 3D    | Footer + ctrl+c       | Enter/Esc mode switch, ctrl+c close                           | Mode switching works      |        |
| 3E    | Split pane navigation | ctrl+h/j/k/l between panes                                    | Focus moves correctly     |        |
| 3F    | Console bridging      | console.log in webview                                        | Output in terminal        |        |
| 3G    | ctrl+z / fg           | ctrl+z then fg                                                | Hide/restore works        |        |
| 3H    | Multi-webview         | Open in two panes                                             | Independent operation     |        |

### Key Files Reference

**No libghostty changes required!**

**Zig CLI:**

- `src/termsurf-cli/main.zig` (new) - Socket client, command parsing

**Swift app (termsurf-macos):**

```
Sources/Features/Socket/
├── SocketServer.swift           # Unix domain socket listener
├── SocketConnection.swift       # Handle client connections
├── TermsurfProtocol.swift       # JSON message types
└── CommandHandler.swift         # Route commands to handlers

Sources/Features/WebView/
├── WebViewContainer.swift       # Container with footer + webview, mode switching
├── FooterView.swift             # Footer bar (placeholder for URL bar)
├── WebViewOverlay.swift         # WKWebView with console capture
├── WebViewManager.swift         # Track webviews by ID
└── ConsoleCapture.swift         # JS injection
```

- Shell spawning code - Set `TERMSURF_SOCKET` and `TERMSURF_PANE_ID` env vars

## Phase 4: Polish & Features

### Profile/Session Isolation

WKWebView supports named profiles via `WKWebsiteDataStore(forIdentifier:)`
(macOS 14+). Each profile gets completely isolated: cookies, localStorage,
IndexedDB, cache, service workers.

**Requires:** macOS 14+ (Sonoma) - acceptable for target audience (web
developers)

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
  - [ ] Consider `--incognito` flag using `.nonPersistent()` for ephemeral
        sessions
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
