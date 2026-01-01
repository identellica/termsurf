# TermSurf

A terminal emulator with integrated webview support, built as a fork of [Ghostty](https://github.com/ghostty-org/ghostty).

## Vision

TermSurf is a terminal designed for web developers. It combines a high-quality native terminal (via libghostty) with the ability to open webviews directly inside terminal panes. This enables workflows like:

- Run `termsurf open https://localhost:3000` to preview your dev server in a pane
- View documentation alongside your terminal
- Debug web applications with terminal and browser side-by-side
- Script the terminal using TypeScript instead of Lua

## Key Features (Planned)

- **Webview Panes**: Open web content as first-class panes alongside terminals
- **Unified Focus Management**: Navigate between terminal and webview panes with vim-style keybindings (ctrl+h/j/k/l)
- **Console Bridging**: `console.log` output from webviews appears in stdout
- **TypeScript Configuration**: Configure the terminal using TypeScript instead of config files

## Architecture

This project is structured as a fork of Ghostty with TermSurf code in the `termsurf-macos/` directory:

```
termsurf/                    # Root (Ghostty fork)
├── src/                     # libghostty (Zig) - shared core
├── macos/                   # Original Ghostty macOS app
├── termsurf-macos/          # TermSurf macOS app (our code)
│   ├── Sources/             # Swift source
│   ├── docs/                # TermSurf documentation
│   │   ├── ARCHITECTURE.md  # Technical decisions
│   │   └── ROADMAP.md       # Development phases
│   └── Ghostty.xcodeproj    # Xcode project
└── ...                      # Other Ghostty/libghostty files
```

### Why This Structure?

By forking Ghostty and placing our modifications in a separate folder:

1. **Upstream compatibility**: Can merge upstream Ghostty changes
2. **Side-by-side comparison**: Can always compare against working Ghostty
3. **Clear separation**: TermSurf-specific code is isolated in `termsurf-macos/`

## Building

### Prerequisites

- macOS 13+
- Xcode 15+
- Zig (see [Ghostty's build instructions](https://ghostty.org/docs/install/build))

### Build libghostty

First, build the shared library from the repo root:

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

## Development Status

**Current Phase**: Foundation

See the [Roadmap](termsurf-macos/docs/ROADMAP.md) for development phases and the [Architecture](termsurf-macos/docs/ARCHITECTURE.md) document for technical decisions.

## Acknowledgments

TermSurf is built on top of [Ghostty](https://github.com/ghostty-org/ghostty), an excellent terminal emulator by Mitchell Hashimoto. We use libghostty for terminal emulation and rendering, extending it with webview integration for web developers.

## License

This project is a fork of Ghostty and maintains the MIT license. See [LICENSE](LICENSE) for details.
