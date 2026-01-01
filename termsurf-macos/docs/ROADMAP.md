# TermSurf Development Roadmap

## Phase 1: Foundation (Current)

### 1.1 Project Setup
- [x] Fork Ghostty repository
- [x] Create `termsurf-macos/` directory structure
- [x] Document architecture decisions
- [x] Verify original `macos/` app builds
- [x] Verify `termsurf-macos/` app builds
- [ ] Rename app bundle (Ghostty → TermSurf)

### 1.2 Basic Webview Integration
- [ ] Create `WebviewPaneView.swift` wrapper around WKWebView
- [ ] Extend `SplitTree` node type to support webview panes
- [ ] Implement basic webview rendering in split view
- [ ] Add keyboard focus routing to webviews

**Milestone**: Can manually create a split with terminal + webview side by side

## Phase 2: Command Integration

### 2.1 `termsurf open` Command
- [ ] Define escape sequence or command format for opening URLs
- [ ] Parse URL from terminal input
- [ ] Replace current pane with webview (or split)
- [ ] Handle navigation (back, forward, refresh)

### 2.2 Webview-Terminal Interaction
- [ ] Implement console.log → stdout bridging
- [ ] Handle webview close (return to terminal)
- [ ] Pass environment variables to webview (for dev tools integration)

**Milestone**: Can type `termsurf open https://google.com` and see it in a pane

## Phase 3: Navigation & Polish

### 3.1 Focus Management
- [ ] Vim-style navigation works across terminal and webview panes
- [ ] Visual indicator for focused pane type
- [ ] Tab key behavior in webviews

### 3.2 Webview Features
- [ ] URL bar / title display
- [ ] Loading indicator
- [ ] Error handling (failed loads)
- [ ] DevTools integration (optional)

### 3.3 Configuration
- [ ] Add webview-specific config options
- [ ] Default search engine for URL-like inputs
- [ ] Keybindings for webview actions

**Milestone**: Fully functional webview panes with polished UX

## Phase 4: TypeScript Configuration

### 4.1 JavaScript Engine Integration
- [ ] Embed JavaScriptCore for config evaluation
- [ ] Define TypeScript config schema
- [ ] Implement config hot-reloading

### 4.2 Config API
- [ ] Expose terminal configuration options
- [ ] Expose keybinding configuration
- [ ] Expose `termsurf.open()` API for scripting

**Milestone**: Can configure terminal entirely via TypeScript

## Phase 5: Cross-Platform (Future)

### 5.1 Linux Support
- [ ] Create `termsurf-linux/` as fork of GTK app
- [ ] Implement WebKitGTK webview panes
- [ ] Port TypeScript config to Linux

### 5.2 Windows Support (Stretch)
- [ ] Evaluate Windows terminal options
- [ ] WebView2 integration

## MVP Definition

The **Minimum Viable Product** is complete when:

1. TermSurf macOS app builds and runs
2. Can open webview in a pane via command
3. Can navigate between terminal and webview panes
4. Console.log output appears in terminal
5. Basic documentation exists

This corresponds to completing **Phases 1-2**.

## Technical Debt to Address

- [ ] Rename Ghostty references in termsurf-macos to TermSurf
- [ ] Update Xcode project settings (bundle ID, app name)
- [ ] Add TermSurf-specific app icon
- [ ] Proper error handling throughout

## Testing Strategy

### Manual Testing
- Build and run on macOS
- Test split operations with webviews
- Test focus navigation
- Test console.log bridging

### Automated Testing (Future)
- Unit tests for SplitTree modifications
- UI tests for webview integration

## Dependencies

### Required
- Xcode 15+
- macOS 13+
- Zig (for building libghostty)

### Optional
- Node.js (for TypeScript type definitions)

## Resources

- [Ghostty Documentation](https://ghostty.org/docs)
- [WKWebView Documentation](https://developer.apple.com/documentation/webkit/wkwebview)
- [JavaScriptCore Documentation](https://developer.apple.com/documentation/javascriptcore)
