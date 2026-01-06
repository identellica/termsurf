# Build Guide

## Clean Build

To ensure a completely fresh build with no cached artifacts (both Zig and Swift):

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

| Cache | Location | Contents |
|-------|----------|----------|
| Zig build | `zig-out`, `zig-cache`, `.zig-cache` | Compiled Zig objects, libghostty |
| DerivedData | `~/Library/Developer/Xcode/DerivedData/TermSurf-*` | Compiled Swift, linked app bundle |
| SPM artifacts | `~/Library/Caches/org.swift.swiftpm/artifacts/` | Downloaded binary dependencies (Sparkle) |
| SPM packages | `termsurf-macos/.build` | Resolved package versions |
