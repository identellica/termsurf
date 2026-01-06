# Release Procedure

This document describes how to make a new release of TermSurf.

## Prerequisites

- All changes committed to `main` branch
- CHANGELOG.md updated with new version section
- Working Xcode installation

## Steps

### 1. Verify Build

Build the app to ensure it compiles without errors:

```bash
cd termsurf-macos && xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Release build
```

### 2. Tag the Release

Create an annotated tag for the new version:

```bash
git tag -a v0.1.1 -m "Release v0.1.1"
```

### 3. Push to GitHub

Push the main branch and the new tag:

```bash
git push origin main
git push origin v0.1.1
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
