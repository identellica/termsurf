# TermSurf CEF Integration Plan

This document tracks the integration of Chromium Embedded Framework (CEF) into
TermSurf, enabling browser panes within the terminal.

## Why CEF?

- **Consistent cross-platform API** - Same C API on macOS, Linux, Windows
- **Full Chrome DevTools** - Essential for web developers
- **Profile support** - Different cache paths = isolated sessions (cookies,
  localStorage)
- **Console message capture** - Native `OnConsoleMessage` callback for
  stdout/stderr bridging
- **Binary size is acceptable** - ~150-200MB, but provides full browser
  capabilities

## Phase 1: Setup & Foundation ✓

Download CEF and understand its structure before writing any code.

- [x] Download latest stable CEF macOS arm64 build from
      cef-builds.spotifycdn.com
- [x] Extract to `termsurf-macos/Frameworks/cef/` (v143.0.13, Chromium
      143.0.7499.170)
- [x] Document the directory structure (headers, framework, resources)
- [x] Read key C API headers we need:
  - [x] `include/capi/cef_app_capi.h` - initialization
  - [x] `include/capi/cef_browser_capi.h` - browser control
  - [x] `include/capi/cef_client_capi.h` - client handlers
  - [x] `include/capi/cef_display_handler_capi.h` - console messages
  - [x] `include/capi/cef_life_span_handler_capi.h` - browser lifecycle
  - [x] `include/capi/cef_request_context_capi.h` - profiles
- [x] Document the exact C function signatures we need to wrap (see
      [docs/cef.md](docs/cef.md))

## Phase 2: Minimal Swift Bindings ✓

Create a lean Swift wrapper (CEFKit) that exposes only what TermSurf needs.

### Structure

```
termsurf-macos/CEFKit/
├── Modules/CEF/
│   ├── module.modulemap
│   └── CEF.h
├── Core/
│   ├── CEFBase.swift
│   ├── CEFString.swift
│   └── CEFCallback.swift
├── CEFApp.swift
├── CEFBrowser.swift
├── CEFClient.swift
├── CEFDisplayHandler.swift
├── CEFLifeSpanHandler.swift
├── CEFRequestContext.swift
└── CEFSettings.swift
```

### Tasks

- [x] Create module.modulemap to import CEF C headers
- [x] Create umbrella header with only needed headers
- [x] Verify Swift can see CEF types
- [x] Implement `CEFString.swift` - Swift String ↔ cef_string_t conversion
- [x] Implement `CEFBase.swift` - Reference counting wrapper
- [x] Implement `CEFCallback.swift` - Swift callback marshalling pattern
- [x] Implement `CEFApp.swift`:
  - [x] `CEFApp.initialize(settings:)`
  - [x] `CEFApp.shutdown()`
  - [x] `CEFApp.doMessageLoopWork()`
- [x] Implement `CEFBrowser.swift`:
  - [x] `CEFBrowser.create(url:profile:client:)`
  - [x] `browser.loadURL(_:)`
  - [x] `browser.goBack()`, `goForward()`, `reload()`
  - [x] `browser.close()`
  - [x] `browser.view` - returns NSView
- [x] Implement `CEFDisplayHandler.swift`:
  - [x] Protocol with `onConsoleMessage(level:message:source:line:)`
  - [x] C callback that marshals to Swift
- [x] Implement `CEFRequestContext.swift`:
  - [x] `CEFRequestContext.create(cachePath:)` for profile isolation

## Phase 3: Standalone Prototype

Test CEF integration in isolation before integrating with Ghostty.

- [ ] Create simple test macOS app (outside Ghostty)
- [ ] Display CEF browser in a window
- [ ] Add URL text field for navigation
- [ ] Test console message capture:
  - [ ] Load page with `console.log()`, `console.error()`
  - [ ] Verify messages arrive in Swift callback
  - [ ] Implement JSON.stringify workaround for multiple arguments
- [ ] Test profile isolation:
  - [ ] Create two browsers with different cache paths
  - [ ] Verify separate cookies/localStorage
- [ ] Test run loop integration:
  - [ ] Try `cef_do_message_loop_work()` with timer
  - [ ] Verify browser + app both responsive
  - [ ] Test alternative: `multi_threaded_message_loop = true`

## Phase 4: TermSurf Integration

Integrate CEFKit into the TermSurf app and connect it to the pane system.

- [ ] Add CEFKit sources to termsurf-macos Xcode project
- [ ] Link Chromium Embedded Framework
- [ ] Configure framework search paths
- [ ] Handle code signing for CEF framework
- [ ] Integrate with AppKit run loop:
  - [ ] Add timer/dispatch source for `cef_do_message_loop_work()`
  - [ ] Ensure terminal rendering unaffected
- [ ] Extend SplitTree for browser panes:
  - [ ] Add `PaneContent.browser(CEFBrowserView)` case
  - [ ] Create `CEFBrowserView` wrapper
  - [ ] Handle focus routing
- [ ] Implement `termsurf open` command:
  - [ ] Parse: `termsurf open [--profile NAME] URL`
  - [ ] Create browser with appropriate profile
  - [ ] Replace/split current pane with browser
- [ ] Implement console output bridging:
  - [ ] Route `console.log` → stdout
  - [ ] Route `console.error` → stderr
  - [ ] Inject JS for JSON.stringify workaround
- [ ] Implement browser controls:
  - [ ] Ctrl+C to close browser (return to terminal)
  - [ ] Navigation shortcuts
  - [ ] URL display

## Phase 5: Polish & Documentation

Final polish and documentation updates.

- [ ] Profile management:
  - [ ] Default: `~/.termsurf/profiles/default/`
  - [ ] Named: `~/.termsurf/profiles/{name}/`
- [ ] DevTools support:
  - [ ] Command: `termsurf devtools`
  - [ ] Keyboard shortcut (Cmd+Option+I)
- [ ] Update documentation:
  - [ ] ARCHITECTURE.md - CEF integration details
  - [ ] ROADMAP.md - mark completed milestones
  - [ ] Document profile system
  - [ ] Document console bridging
- [ ] Binary distribution:
  - [ ] Bundle CEF framework with app
  - [ ] Document build process

## Resources

- [CEF Integration Guide](docs/cef.md) - C API reference and directory structure
- [CEF Builds](https://cef-builds.spotifycdn.com/index.html) - Official binary
  distributions
- [CEF C API Docs](https://cef-builds.spotifycdn.com/docs/stable.html) - API
  documentation
- [CEF Wiki](https://bitbucket.org/chromiumembedded/cef/wiki/Home) - General
  usage guide
- [CEF.swift](CEF.swift/) - Reference implementation (outdated but informative)

## Notes

- CEF.swift (cloned to repo root) is for reference only - we're building our own
  minimal wrapper
- Console message callback only receives first argument; use JS injection to
  JSON.stringify all args
- CEF takes over message loop by default; use `cef_do_message_loop_work()` to
  integrate with existing loop
