# TermSurf

A terminal emulator with integrated webview support, built as a fork of
[Ghostty](https://github.com/ghostty-org/ghostty).

## Vision

TermSurf is a terminal designed for web developers. It combines a high-quality
native terminal (via libghostty) with the ability to open browsers directly
inside terminal panes. This enables workflows like:

- Run `web open https://localhost:3000` to preview your dev server in a pane
- View documentation alongside your terminal
- Debug web applications with terminal and browser side-by-side
- Script the terminal using TypeScript instead of Lua

**The ultimate goal:** Test your web app in _any_ browser engine, right from
your terminal. Starting with WebKit (Safari) for the MVP, we plan to add
Chromium and Firefox/Gecko support, making TermSurf the ultimate browser for web
developers.

## Key Features

**Implemented:**
- **Browser Panes**: Open browsers as first-class panes via `web open`
  - Safari/WebKit (via WKWebView) - current engine
- **Browser Profiles**: Isolated sessions with `--profile` flag (separate
  cookies, localStorage)
- **Bookmarks**: Save and recall URLs with `web bookmark add/get/list/delete`
- **Unified Focus Management**: Navigate between terminal and browser panes with
  vim-style keybindings (ctrl+h/j/k/l)
- **Console Bridging**: `console.log` → stdout, `console.error` → stderr
- **JavaScript API**: Optional `--js-api` flag enables `window.termsurf.exit(code)`
  for programmatic control

**Planned:**
- **Multi-Engine Support**: Chromium (via CEF) and Firefox/Gecko
- **TypeScript Configuration**: Configure the terminal using TypeScript instead
  of config files

```bash
# Example usage
web open google.com                          # Open in WebKit
web open --profile work google.com           # With isolated profile
web open --js-api localhost:3000             # Enable JS API for automation

# Bookmarks
web bookmark add --name google --url https://google.com
web bookmark list
web open google                              # Open by bookmark name
```

## Architecture

This project is structured as a fork of Ghostty with TermSurf code in the
`termsurf-macos/` directory:

```
termsurf/                    # Root (Ghostty fork)
├── src/                     # libghostty (Zig) - shared core
│   └── cli/web.zig          # CLI web command (termsurf +web)
├── macos/                   # Original Ghostty macOS app
├── docs/                    # Documentation
│   ├── architecture.md      # Technical decisions
│   ├── console.md           # Console bridging & JS API
│   ├── keybindings.md       # Keyboard shortcuts
│   └── cef.md               # CEF integration (deferred)
├── TODO.md                  # Active task checklist
├── termsurf-macos/          # TermSurf macOS app (our code)
│   ├── Sources/             # Swift source
│   │   └── Features/
│   │       ├── WebView/     # WebView implementation
│   │       └── Socket/      # CLI-app communication
│   └── TermSurf.xcodeproj   # Xcode project
└── ...                      # Other Ghostty/libghostty files
```

### Why This Structure?

By forking Ghostty and placing our modifications in a separate folder:

1. **Upstream compatibility**: Can merge upstream Ghostty changes
2. **Side-by-side comparison**: Can always compare against working Ghostty
3. **Clear separation**: TermSurf-specific code is isolated in `termsurf-macos/`

### Browser Engine Strategy

TermSurf is designed to support multiple browser engines. Our API abstracts over
engine-specific details so you can test your app in any browser.

**Current: Safari/WebKit (MVP)**

We start with WKWebView (Apple's WebKit) because:

- Native Swift integration with zero external dependencies
- Console capture via JavaScript injection (console.log → stdout, console.error
  → stderr)
- Safari Web Inspector for debugging
- Session isolation via WKWebsiteDataStore
- Fast to implement and reliable

**Planned: Chromium (via CEF)**

CEF integration is deferred due to Swift-to-C struct marshalling challenges. See
`docs/cef.md` for documentation. When implemented, CEF will provide:

- Full Chrome DevTools
- Cross-platform API (macOS, Linux, Windows)
- Native profile and console capture support

**Planned: Firefox/Gecko**

Gecko is harder to embed (no official desktop embedding API), but we plan to
fork and create one. GeckoView exists for Android, proving it's possible. This
is a longer-term goal.

**The Architecture**

Each engine will implement a common `BrowserEngine` protocol:

- Create browser with profile/cache path
- Navigate, reload, go back/forward
- Capture console messages
- Return embeddable native view

This allows future engines to be added without changing the rest of TermSurf.

## Building

### Prerequisites

- macOS 13+
- Xcode 15+
- Zig (see
  [Ghostty's build instructions](https://ghostty.org/docs/install/build))

### Build libghostty

Build the shared library from the repo root:

```bash
zig build
```

This builds `GhosttyKit.xcframework` for both `macos/` and `termsurf-macos/`.

### Build TermSurf macOS App

```bash
cd termsurf-macos
xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Debug build
```

Or open `termsurf-macos/TermSurf.xcodeproj` in Xcode and build from there.

### Build for Release

To build both libghostty and the Swift app in release mode:

```bash
./scripts/build-release.sh         # Build only
./scripts/build-release.sh --open  # Build and open the app
```

### Updating the App Icon

To update the app icon, place your source image in
`termsurf-macos/icon-source/termsurf-icon.png`, then run:

```bash
./scripts/generate-icons.sh
```

Then rebuild the app.

## Development Status

**Current Phase**: Core features implemented (Phases 3A-3F complete)

See:

- [TODO.md](TODO.md) - Active checklist of tasks to launch
- [Architecture](docs/architecture.md) - Technical decisions and design rationale
- [Console Bridging](docs/console.md) - Console output and JavaScript API
- [Keybindings](docs/keybindings.md) - Keyboard shortcuts for webview modes
- [CEF Integration](docs/cef.md) - CEF attempt documentation (deferred)

## Acknowledgments

TermSurf is built on top of [Ghostty](https://github.com/ghostty-org/ghostty),
an excellent terminal emulator by Mitchell Hashimoto. We use libghostty for
terminal emulation and rendering, extending it with webview integration for web
developers.

## License

This project is a fork of Ghostty and maintains the MIT license. See
[LICENSE](LICENSE) for details. TermSurf is a trademark of Identellica LLC. The
code is MIT-licensed; the name and logo are not.
