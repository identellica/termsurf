# TermSurf

A terminal emulator with integrated webview support, built as a fork of [Ghostty](https://github.com/ghostty-org/ghostty).

## Vision

TermSurf is a terminal designed for web developers. It combines a high-quality native terminal (via libghostty) with the ability to open browsers directly inside terminal panes. This enables workflows like:

- Run `termsurf open https://localhost:3000` to preview your dev server in a pane
- View documentation alongside your terminal
- Debug web applications with terminal and browser side-by-side
- Script the terminal using TypeScript instead of Lua

**The ultimate goal:** Test your web app in *any* browser engine, right from your terminal. Starting with Chromium, we plan to add Safari/WebKit and Firefox/Gecko support, making TermSurf the ultimate browser for web developers.

## Key Features (Planned)

- **Multi-Engine Browser Panes**: Open browsers as first-class panes with `--browser` flag
  - Chromium (via CEF) - available now
  - Safari/WebKit - planned
  - Firefox/Gecko - planned
- **Browser Profiles**: Isolated sessions with `--profile` flag (separate cookies, localStorage)
- **Unified Focus Management**: Navigate between terminal and browser panes with vim-style keybindings (ctrl+h/j/k/l)
- **Console Bridging**: `console.log` → stdout, `console.error` → stderr
- **TypeScript Configuration**: Configure the terminal using TypeScript instead of config files

```bash
# Example usage
termsurf open google.com                        # Chromium (default)
termsurf open --browser chromium google.com     # Chromium (explicit)
termsurf open --browser webkit google.com       # Safari/WebKit (planned)
termsurf open --browser gecko google.com        # Firefox/Gecko (planned)
termsurf open --profile work --browser chromium google.com  # With profile
```

## Architecture

This project is structured as a fork of Ghostty with TermSurf code in the `termsurf-macos/` directory:

```
termsurf/                    # Root (Ghostty fork)
├── src/                     # libghostty (Zig) - shared core
├── macos/                   # Original Ghostty macOS app
├── docs/                    # Documentation
│   ├── architecture.md      # Technical decisions
│   └── cef.md               # CEF C API reference
├── TODO.md                  # Active task checklist
├── termsurf-macos/          # TermSurf macOS app (our code)
│   ├── Sources/             # Swift source
│   ├── Frameworks/cef/      # CEF binary distribution
│   └── Ghostty.xcodeproj    # Xcode project
└── ...                      # Other Ghostty/libghostty files
```

### Why This Structure?

By forking Ghostty and placing our modifications in a separate folder:

1. **Upstream compatibility**: Can merge upstream Ghostty changes
2. **Side-by-side comparison**: Can always compare against working Ghostty
3. **Clear separation**: TermSurf-specific code is isolated in `termsurf-macos/`

### Browser Engine Strategy

TermSurf is designed to support multiple browser engines. Our API abstracts over engine-specific details so you can test your app in any browser.

**Current: Chromium (via CEF)**

We start with the Chromium Embedded Framework because:
- Consistent cross-platform API (macOS, Linux, Windows)
- Full Chrome DevTools
- Native profile and console capture support
- Well-established, used by Electron, Spotify, Steam, etc.

**Planned: Safari/WebKit**

WebKit is already designed for embedding (WKWebView, WebKitGTK). We'll create a unified wrapper providing the same API as our CEF integration. This is our next priority after Chromium.

**Planned: Firefox/Gecko**

Gecko is harder to embed (no official desktop embedding API), but we plan to fork and create one. GeckoView exists for Android, proving it's possible. This is a longer-term goal.

**The Architecture**

Each engine will implement a common `BrowserEngine` protocol:
- Create browser with profile/cache path
- Navigate, reload, go back/forward
- Capture console messages
- Return embeddable native view

This allows future engines to be added without changing the rest of TermSurf.

Binary size increases ~150-200MB per engine, which is acceptable for web developers who need accurate cross-browser testing.

## Building

### Prerequisites

- macOS 13+
- Xcode 15+
- Zig (see [Ghostty's build instructions](https://ghostty.org/docs/install/build))

### Download CEF

TermSurf requires the Chromium Embedded Framework (CEF) for browser panes. Run the setup script to download it (~250MB):

```bash
./scripts/setup-cef.sh
```

Or download manually from [cef-builds.spotifycdn.com](https://cef-builds.spotifycdn.com/index.html):
- Select **macOS ARM64** (or x64 for Intel)
- Download the **Standard Distribution** for the latest stable version
- Extract to `termsurf-macos/Frameworks/cef/`

### Build libghostty

Build the shared library from the repo root:

```bash
zig build
```

This builds `GhosttyKit.xcframework` for both `macos/` and `termsurf-macos/`.

### Build TermSurf macOS App

```bash
cd termsurf-macos
xcodebuild -project Ghostty.xcodeproj -scheme Ghostty -configuration Debug build
```

Or open `termsurf-macos/Ghostty.xcodeproj` in Xcode and build from there.

### Updating the App Icon

To update the app icon, place your source image in `termsurf-macos/icon-source/termsurf-icon.png`, then run:

```bash
cd termsurf-macos
./scripts/generate-icons.sh
```

Then rebuild the app.

## Development Status

**Current Phase**: Foundation (CEF Integration)

See:
- [TODO.md](TODO.md) - Active checklist of tasks to launch
- [Architecture](docs/architecture.md) - Technical decisions and design rationale
- [CEF Integration](docs/cef.md) - Browser engine C API reference

## Acknowledgments

TermSurf is built on top of [Ghostty](https://github.com/ghostty-org/ghostty), an excellent terminal emulator by Mitchell Hashimoto. We use libghostty for terminal emulation and rendering, extending it with webview integration for web developers.

## License

This project is a fork of Ghostty and maintains the MIT license. See [LICENSE](LICENSE) for details.
