# Release Procedure

This document describes how to make a new release of TermSurf.

## Prerequisites

- All changes committed to `main` branch
- Working Xcode installation

## Steps

### 1. Update Version in build.zig.zon

Edit `build.zig.zon` and update the version field to the new version:

```zig
.version = "X.Y.Z",
```

**Important:** The version here must match the git tag you'll create (without the `v` prefix). The build system enforces this—if they don't match, `zig build` will fail with "tagged releases must be in vX.Y.Z format matching build.zig".

### 2. Update CHANGELOG.md

Add a new section for the release version with a summary of changes.

### 3. Commit Version Bump

```bash
git add build.zig.zon CHANGELOG.md
git commit -m "Bump version to X.Y.Z"
```

### 4. Verify Build

Build the app to ensure it compiles without errors:

```bash
zig build
cd termsurf-macos && xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Release build
```

### 5. Tag the Release

Create an annotated tag for the new version:

```bash
git tag -a vX.Y.Z -m "Release vX.Y.Z"
```

### 6. Push to GitHub

Push the main branch and the new tag:

```bash
git push origin main
git push origin vX.Y.Z
```

Or push all tags at once:

```bash
git push origin main --tags
```

## Version Numbering

We use semantic versioning (MAJOR.MINOR.PATCH):

- **PATCH** (0.0.x): Bug fixes, small improvements
- **MINOR** (0.x.0): New features, backward compatible
- **MAJOR** (x.0.0): Breaking changes

## Future

When we publish builds (e.g., Homebrew, GitHub Releases with binaries), this
document will be expanded with:

- Building signed release binaries
- Creating GitHub Release with release notes
- Publishing to package managers
