# TermSurf Browser Pane Integration Plan

This document tracks the integration of browser panes into TermSurf, enabling
web content alongside terminal sessions.

## Decision: WKWebView for MVP

After extensive work on CEF (Chromium Embedded Framework) integration, we
pivoted to Apple's WKWebView for the MVP due to fundamental Swift-to-C
marshalling issues with CEF's C API.

### Why WKWebView?

| Factor | WKWebView | CEF |
|--------|-----------|-----|
| Setup time | 15 minutes | Hours (still failing) |
| Dependencies | None (built-in) | 250MB framework |
| Console capture | Works via JS injection | Native callback (if it worked) |
| Swift integration | Native, seamless | Complex C struct marshalling |
| DevTools | Safari Web Inspector | Chrome DevTools |
| Rendering engine | WebKit | Blink (Chrome) |

**Bottom line:** WKWebView gives us a working browser pane MVP immediately.
CEF remains an option for the future if Chrome DevTools become essential.

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

## Phase 3: TermSurf Integration

Integrate WebViewKit into the TermSurf app and connect it to the pane system.

### Setup

- [ ] Create `termsurf-macos/WebViewKit/` directory (extract from WebViewTest)
- [ ] Add WebViewKit sources to TermSurf Xcode project
- [ ] Verify builds with main TermSurf target

### Pane System Integration

- [ ] Extend SplitTree for browser panes:
  - [ ] Add `PaneContent.browser(WebBrowserView)` case (or similar)
  - [ ] Create `WebBrowserView` wrapper that hosts WKWebView
  - [ ] Handle focus routing between terminal and browser panes
  - [ ] Handle pane resizing
- [ ] Implement pane lifecycle:
  - [ ] Create browser pane
  - [ ] Close browser pane (return to terminal or close split)
  - [ ] Switch focus between panes

### Command Integration

- [ ] Implement `termsurf open` command:
  - [ ] Parse: `termsurf open [--profile NAME] URL`
  - [ ] Create browser pane with URL
  - [ ] Handle invalid URLs gracefully
- [ ] Implement browser controls:
  - [ ] Keyboard shortcut to close browser (e.g., Ctrl+W or Escape)
  - [ ] Navigation: back, forward, reload
  - [ ] URL display in status bar or title

### Console Output Bridging

- [ ] Connect console capture to terminal output:
  - [ ] Route captured console.log to associated terminal's stdout
  - [ ] Route captured console.error to associated terminal's stderr
  - [ ] Handle case where browser pane has no associated terminal
- [ ] Consider prefixing output (e.g., `[browser] message`)

## Phase 4: Polish & Features

### Profile/Session Isolation

WKWebView works differently than CEF for profile isolation:

- [ ] Research `WKWebsiteDataStore` for session isolation
  - [ ] `WKWebsiteDataStore.default()` - shared cookies
  - [ ] `WKWebsiteDataStore.nonPersistent()` - ephemeral (incognito)
  - [ ] Custom persistent store with specific directory?
- [ ] Implement profile support if feasible:
  - [ ] Default profile
  - [ ] Named profiles with separate cookies/localStorage
  - [ ] Ephemeral/incognito mode

### Developer Tools

- [ ] Enable Safari Web Inspector for WKWebView:
  - [ ] Set `webView.configuration.preferences.setValue(true, forKey: "developerExtrasEnabled")`
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

### Files to Clean Up

When ready to remove CEF code:

```
termsurf-macos/
├── CEFKit/                    # Delete entire directory
├── CEFTest/                   # Delete entire directory
├── Frameworks/cef/            # Delete (250MB framework)
└── WebViewTest.xcodeproj/     # Can delete after extracting WebViewKit
```

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
- [CEF Builds](https://cef-builds.spotifycdn.com/index.html) - Binary distributions
- [CEF Wiki](https://bitbucket.org/chromiumembedded/cef/wiki/Home) - General guide

---

## Notes

- WebViewTest app is the working prototype - use as reference for integration
- Console capture uses JS injection; native console isn't accessible
- WKWebView works best when app is properly signed (some features restricted otherwise)
- Safari Web Inspector requires "Develop" menu enabled in Safari preferences
