# Merging Upstream (Ghostty)

This document describes how to merge changes from the upstream Ghostty repository
into TermSurf while preserving our modifications.

## Overview

TermSurf is a fork of [Ghostty](https://github.com/ghostty-org/ghostty). We track
upstream in a remote called `upstream` and periodically merge to get bug fixes,
performance improvements, and new features.

Our modifications fall into two categories:

1. **Upstream-friendly changes** - Additive APIs that could be submitted as PRs
   to Ghostty (e.g., custom config directory support)

2. **TermSurf-specific changes** - Branding and features unique to TermSurf
   (e.g., web CLI command, surfer emoji)

See [libghostty.md](libghostty.md) for detailed documentation of our changes.

## Modified Files Inventory

### Upstream-Friendly (Low Conflict Risk)

These are additive changes that don't modify existing Ghostty code paths:

| File | Change | Notes |
|------|--------|-------|
| `include/ghostty.h` | Added `ghostty_config_load_files` declaration | End of file |
| `src/config/Config.zig` | Added `loadFiles` method | New public method |
| `src/config/CApi.zig` | Added C API wrapper | New function |
| `src/os/macos.zig` | Added `appSupportDirWithBundleId` helper | New function |

### TermSurf-Specific (Branding)

Simple string replacements, easy to re-apply if conflicts occur:

| File | Change |
|------|--------|
| `src/cli/help.zig` | "ghostty" → "termsurf", app name references |
| `src/cli/version.zig` | "Ghostty" → "TermSurf" in version banner |
| `src/cli/list_themes.zig` | Ghost emoji → surfer emoji in preview title |

### TermSurf-Specific (Functional)

These modify existing Ghostty code and have higher conflict risk:

| File | Change | Conflict Risk |
|------|--------|---------------|
| `src/cli/ghostty.zig` | Added `web` action, `detectMultiCall` | **High** |
| `src/cli/action.zig` | Multi-call binary detection via `argv[0]` | **High** |
| `src/cli/web.zig` | **New file** (no conflict) | None |

### Build System

| File | Change |
|------|--------|
| `build.zig` | XCFramework output to both `macos/` and `termsurf-macos/` |
| `src/build/GhosttyXCFramework.zig` | Dual output paths |

## Pre-Merge Checklist

Before starting a merge:

- [ ] Working tree is clean (`git status` shows no changes)
- [ ] All local changes are committed
- [ ] Note current HEAD: `git rev-parse HEAD`
- [ ] Ensure tests pass: `zig build test`
- [ ] Ensure app builds: `cd termsurf-macos && xcodebuild`

## Fetch and Review

### 1. Fetch Upstream

```bash
git fetch upstream
```

### 2. See What Changed

```bash
# Summary of new commits
git log --oneline HEAD..upstream/main | head -50

# Count of commits
git rev-list --count HEAD..upstream/main

# Files changed in areas we care about
git diff --stat HEAD..upstream/main -- src/cli/
git diff --stat HEAD..upstream/main -- src/config/
git diff --stat HEAD..upstream/main -- include/
```

### 3. Check for Conflicts

Preview which of our modified files have upstream changes:

```bash
# CLI files (highest risk)
git diff HEAD..upstream/main -- src/cli/ghostty.zig
git diff HEAD..upstream/main -- src/cli/action.zig

# Config API files
git diff HEAD..upstream/main -- src/config/Config.zig
git diff HEAD..upstream/main -- src/config/CApi.zig

# Branding files
git diff HEAD..upstream/main -- src/cli/help.zig
git diff HEAD..upstream/main -- src/cli/version.zig
```

If a file shows no diff, it hasn't changed upstream and will merge cleanly.

## Merge Strategy

### Recommended: Merge Commit

```bash
git merge upstream/main -m "Merge upstream Ghostty"
```

**Pros:**
- Preserves full history
- Easy to see what came from upstream vs our changes
- Can be reverted cleanly

**Cons:**
- Creates merge commits in history

### Alternative: Rebase

```bash
git rebase upstream/main
```

**Pros:**
- Linear history
- Our commits stay on top

**Cons:**
- Rewrites history (problematic if already pushed)
- Each of our commits may need conflict resolution

**Recommendation:** Use merge commits for routine updates. Use rebase only for
major restructuring when you want a clean history.

## Conflict Resolution Guide

### src/cli/ghostty.zig

This file defines the CLI entry point. We added:
- Import for `web.zig`
- `detectMultiCall` function
- `web` case in the action switch

**Resolution strategy:**
1. Keep all upstream changes to existing code
2. Re-add our `web` import at the top
3. Re-add `detectMultiCall` function (search for it in our version)
4. Re-add `.web` case in the action switch statement

```zig
// Our additions to look for:
const web = @import("web.zig");

fn detectMultiCall(argv0: []const u8) ?Action.Tag { ... }

// In the switch statement:
.web => web.run(alloc),
```

### src/cli/action.zig

We added multi-call binary detection. Look for our changes to the `init` function
that checks `argv[0]` for "web".

**Resolution strategy:**
1. Accept upstream changes
2. Re-add our multi-call detection logic in `init`

### src/cli/help.zig, version.zig, list_themes.zig

Simple branding changes. If conflicts occur:

**Resolution strategy:**
1. Accept upstream changes (they may have added new text)
2. Re-apply our branding substitutions:
   - "ghostty" → "termsurf"
   - "Ghostty" → "TermSurf"
   - Ghost emoji → surfer emoji

### src/config/Config.zig, CApi.zig

We added new methods/functions. These are additive and unlikely to conflict.

**Resolution strategy:**
1. Accept upstream changes
2. Verify our added functions are still present
3. If removed by conflict, re-add them

### include/ghostty.h

We added a single function declaration at the end.

**Resolution strategy:**
1. Accept upstream changes
2. Ensure our declaration is still at the end:
   ```c
   void ghostty_config_load_files(ghostty_config_t, const char*, const char*);
   ```

### build.zig, GhosttyXCFramework.zig

We modified build output paths.

**Resolution strategy:**
1. Accept upstream changes
2. Re-add our dual output path logic

## Performing the Merge

### Step-by-Step

```bash
# 1. Ensure clean state
git status  # Should be clean

# 2. Fetch latest
git fetch upstream

# 3. Start merge
git merge upstream/main

# 4. If conflicts, resolve each file
git status  # Shows conflicted files
# Edit each file, then:
git add <resolved-file>

# 5. Complete merge
git commit  # If needed (merge may auto-commit if no conflicts)

# 6. Verify build
zig build
zig build test

# 7. Build macOS app
cd termsurf-macos && xcodebuild -scheme TermSurf -configuration Debug build

# 8. Smoke test
# - Open terminal, verify basic functionality
# - Run `web open https://example.com`, verify browser pane works
# - Check About window shows correct version
```

## Post-Merge Testing

- [ ] `zig build` succeeds
- [ ] `zig build test` passes
- [ ] `xcodebuild` succeeds
- [ ] App launches
- [ ] Terminal pane works (typing, colors, scrolling)
- [ ] `termsurf +web open https://example.com` works
- [ ] `web google.com` works (multi-call binary)
- [ ] Profiles work (`--profile test`)
- [ ] About window shows correct info
- [ ] Keyboard shortcuts work (cmd+t, cmd+w, etc.)

## Rollback

If the merge goes wrong:

```bash
# Before committing the merge:
git merge --abort

# After committing the merge:
git reset --hard ORIG_HEAD

# If already pushed (careful!):
git revert -m 1 <merge-commit-hash>
```

## Merge Frequency

**Recommended:** Merge upstream monthly, or more frequently if:
- A security fix is released
- A bug affecting TermSurf is fixed
- A feature we want is added

**Before major releases:** Always merge upstream to get latest fixes.

## Submitting Upstream PRs

After TermSurf MVP, consider submitting our upstream-friendly changes:

1. **Custom config directory API** - Useful for any app embedding libghostty
2. **Any bug fixes** we make to libghostty

See the "Submitting Upstream" section in [libghostty.md](libghostty.md) for the
process.
