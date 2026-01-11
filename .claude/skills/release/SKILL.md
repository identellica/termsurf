---
name: release
description: "Create a new TermSurf release"
---

# Release

Read and follow the process documented in `docs/release.md`.

## Steps

1. **Read the documentation** - Read `docs/release.md` to understand the full
   release process.

2. **Review changes since last release**
   - Get current version: `git describe --tags --abbrev=0`
   - List commits since last release:
     `git log --oneline $(git describe --tags --abbrev=0)..HEAD`
   - Determine new version number (MAJOR.MINOR.PATCH)
   - Always increment the minor version number, not major or patch, unless
     explicitly requested otherwise.

3. **Update version numbers** - Update version in two places:
   - `build.zig.zon` - the `.version` field
   - `termsurf-macos/TermSurf.xcodeproj/project.pbxproj` - `MARKETING_VERSION`

4. **Update CHANGELOG.md** - Add a new section for the release version
   summarizing the changes.

5. **Commit version bump**
   - `git add build.zig.zon CHANGELOG.md termsurf-macos/TermSurf.xcodeproj/project.pbxproj`
   - `git commit -m "Bump version to X.Y.Z"`

6. **Verify builds**
   - `zig build`
   - `cd termsurf-macos && xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Release build`

7. **Tag and push**
   - `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
   - `git push origin main --tags`

8. **Deploy website**
   - `cd website && bun run build:commits`
   - `bun run deploy`
