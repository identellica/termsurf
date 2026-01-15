# TermSurf 2.0 TODO

## Completed

### cef-rs Validation

- [x] Import cef-rs into monorepo
- [x] Fix IOSurface texture import (macOS)
- [x] Fix purple flash on startup
- [x] Add input handling (keyboard, mouse, scroll)
- [x] Add multi-browser instance support
- [x] Suppress context menu (winit crash workaround)
- [x] Event-driven rendering (performance)

### WezTerm Foundation

- [x] Fork WezTerm as ts2/
- [x] Add `web-open` CLI command (PDU plumbing)

## In Progress

### CEF Integration

- [ ] Add cef-rs dependency to WezTerm
- [ ] Initialize CEF at startup (message pump integration)
- [ ] Create `BrowserPane` struct
- [ ] Render CEF texture in wgpu pipeline
- [ ] Wire `web-open` to create actual browser pane
