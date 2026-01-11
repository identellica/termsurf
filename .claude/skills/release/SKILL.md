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

5. **Commit version bump** with a poetic message
   - `git add build.zig.zon CHANGELOG.md termsurf-macos/TermSurf.xcodeproj/project.pbxproj`
   - Write a short poem celebrating the release (see examples below)
   - Commit with the poetic message

## Poetic Release Messages

Releases are milestones worth celebrating! Write a poem (4-8 lines) that captures the spirit of the release.

### Format
```
Release X.Y.Z

[Poem, 4-8 lines]

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>
```

### Examples

**Limerick style:**
```
Release 0.4.0

A terminal wanted to browse,
So we gave it a web-viewing house.
With tabs and a bar,
It's come quite far—
Now surf and type, take your vows!
```

**Ballad style:**
```
Release 0.5.0

The changelog grows with features anew,
Bugs were squashed, the codebase true.
From terminal depths to browser heights,
We ship this version with delight.
Another step upon the way,
TermSurf improves again today!
```

**Celebratory style:**
```
Release 1.0.0

From zero to one, the journey's begun,
A terminal browser, second to none!
Through countless commits and late-night fights,
We've reached this peak of surfing heights.
```

6. **Verify builds**
   - `zig build`
   - `cd termsurf-macos && xcodebuild -project TermSurf.xcodeproj -scheme TermSurf -configuration Release build`

7. **Tag and push**
   - `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
   - `git push origin main --tags`

8. **Deploy website**
   - `cd website && bun run build:data`
   - `bun run deploy`
