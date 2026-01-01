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
- **TypeScript Configuration**: Configure the terminal using TypeScript instead of Lua

## Architecture

This project is structured as a fork of Ghostty with our code isolated in the `termsurf-macos/` directory:

```
termsurf/                    # Root (Ghostty fork)
├── src/                     # libghostty (Zig) - unchanged
├── macos/                   # Original Ghostty macOS app - unchanged
├── termsurf-macos/          # Our fork of the macOS app (this folder)
│   ├── Sources/             # Swift source (will be modified)
│   ├── docs/                # Our documentation
│   └── README.md            # This file
└── ...                      # Other Ghostty files - unchanged
```

### Why This Structure?

By keeping all Ghostty files unchanged and placing our modifications in a separate folder:

1. **Easy upstream sync**: `git fetch upstream && git merge upstream/main` works without conflicts
2. **Side-by-side comparison**: Can always compare our code against working Ghostty
3. **Clear separation**: Everything in `termsurf-macos/` is our code

## Building

### Prerequisites

- macOS 13+
- Xcode 15+
- Zig (see Ghostty's build instructions)

### Build libghostty

First, build the shared library:

```bash
# From the repo root
zig build -Doptimize=ReleaseFast
```

### Build TermSurf macOS App

```bash
cd termsurf-macos
xcodebuild -project Ghostty.xcodeproj -scheme Ghostty -configuration Debug
```

Or open `termsurf-macos/Ghostty.xcodeproj` in Xcode and build from there.

## Development Status

**Current Phase**: Initial Setup

See [docs/ROADMAP.md](docs/ROADMAP.md) for the development plan.

## Documentation

- [Architecture & Research](docs/ARCHITECTURE.md) - Why we chose Ghostty, technical analysis
- [Roadmap](docs/ROADMAP.md) - Development phases and MVP plan

## Contributing

This project is in early development. Check the roadmap for current priorities.

## License

This project is a fork of Ghostty and maintains the same license. See [LICENSE](../LICENSE) for details.
