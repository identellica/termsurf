# Build Guide

## Build Commands

TermSurf requires building two components: libghostty (Zig) and the macOS app
(Swift).

### Debug Build

```bash
# 1. Build libghostty (debug)
zig build

# 2. Build TermSurf.app (debug)
cd termsurf-macos && xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Debug build
```

### Release Build

```bash
# 1. Build libghostty (release)
zig build -Doptimize=ReleaseFast

# 2. Build TermSurf.app (release)
cd termsurf-macos && xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Release build
```

Or use the convenience script:

```bash
./scripts/build-release.sh         # Build release
./scripts/build-release.sh --clean # Clean build
./scripts/build-release.sh --open  # Build and open app
```

## Clean Build

To ensure a completely fresh build with no cached artifacts (both Zig and
Swift):

```bash
# 1. Clear SPM cache for Sparkle dependency
rm -rf ~/Library/Caches/org.swift.swiftpm/artifacts/*Sparkle*

# 2. Clear Xcode DerivedData
rm -rf ~/Library/Developer/Xcode/DerivedData/TermSurf-*

# 3. Clear Zig build cache
rm -rf zig-out zig-cache .zig-cache

# 4. Clear SPM package resolution in project
rm -rf termsurf-macos/.build
rm -rf termsurf-macos/Package.resolved
```

Then build:

```bash
./scripts/build-release.sh --open
```

## Nuclear Option

If you still have dependency issues, clear the entire SPM cache:

```bash
rm -rf ~/Library/Caches/org.swift.swiftpm
```

## What Each Cache Contains

| Cache         | Location                                           | Contents                                 |
| ------------- | -------------------------------------------------- | ---------------------------------------- |
| Zig build     | `zig-out`, `zig-cache`, `.zig-cache`               | Compiled Zig objects, libghostty         |
| DerivedData   | `~/Library/Developer/Xcode/DerivedData/TermSurf-*` | Compiled Swift, linked app bundle        |
| SPM artifacts | `~/Library/Caches/org.swift.swiftpm/artifacts/`    | Downloaded binary dependencies (Sparkle) |
| SPM packages  | `termsurf-macos/.build`                            | Resolved package versions                |

## CLI Access

The TermSurf app binary doubles as a CLI tool. You can run commands like
`termsurf +help`, `termsurf +list-fonts`, etc.

### Binary Locations

After building, the executable is located at:

| Build Type | Location                                                                                                       |
| ---------- | -------------------------------------------------------------------------------------------------------------- |
| Debug      | `~/Library/Developer/Xcode/DerivedData/TermSurf-*/Build/Products/Debug/TermSurf.app/Contents/MacOS/termsurf`   |
| Release    | `~/Library/Developer/Xcode/DerivedData/TermSurf-*/Build/Products/Release/TermSurf.app/Contents/MacOS/termsurf` |

### Shell Alias

For convenience, add an alias to your shell config (`~/.zshrc` or `~/.bashrc`):

```bash
# For development (Debug build)
alias termsurf-dev='~/Library/Developer/Xcode/DerivedData/TermSurf-*/Build/Products/Debug/TermSurf.app/Contents/MacOS/termsurf'

# For release testing
alias termsurf-release='~/Library/Developer/Xcode/DerivedData/TermSurf-*/Build/Products/Release/TermSurf.app/Contents/MacOS/termsurf'

# Or if installed to /Applications
alias termsurf='/Applications/TermSurf.app/Contents/MacOS/termsurf'
```

### Example Commands

```bash
termsurf +help           # Show available CLI actions
termsurf +version        # Show version info
termsurf +list-fonts     # List available fonts
termsurf +list-themes    # Browse themes interactively
termsurf +show-config    # Show current configuration
```
