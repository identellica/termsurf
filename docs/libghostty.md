# libghostty Changes

This document tracks modifications made to libghostty (the Zig core in `src/`)
for TermSurf. These changes are designed to be upstream-friendly and will be
submitted as PRs to
[ghostty-org/ghostty](https://github.com/ghostty-org/ghostty) after MVP.

## Upstream Strategy

1. **Keep changes minimal and additive** - No breaking changes to existing APIs
2. **Follow existing patterns** - Match Ghostty's code style and conventions
3. **Document rationale** - Explain why each change benefits the ecosystem
4. **Submit after MVP** - Gather all changes, then follow Ghostty's contribution
   guidelines to start a discussion before submitting PRs

## Changes

### 1. Custom Config Directory Support

**Files modified:**

- `include/ghostty.h` - Added C API declaration
- `src/config/Config.zig` - Added `loadFiles` method
- `src/config/CApi.zig` - Added C API wrapper
- `src/os/macos.zig` - Added `appSupportDirWithBundleId` helper

**New C API function:**

```c
void ghostty_config_load_files(ghostty_config_t, const char* app_name, const char* bundle_id);
```

**Behavior:**

- Loads config from `~/.config/{app_name}/` (XDG, all platforms)
- On macOS, falls back to `~/Library/Application Support/{bundle_id}/`
- Looks for both `config` and `config.ghostty` files (same as Ghostty)
- Creates template config in XDG location if no config found
- XDG is preferred over Application Support (opposite of `load_default_files`)

**Why this change:**

Applications embedding libghostty need their own config namespace. Currently,
`ghostty_config_load_default_files()` hardcodes paths to `~/.config/ghostty/`
and `com.mitchellh.ghostty`. This forces embedders to either:

- Share config with Ghostty (confusing for users)
- Fork libghostty permanently (maintenance burden)
- Use environment variables or other workarounds

The new `ghostty_config_load_files()` function solves this by accepting the app
name and bundle ID as parameters, allowing each embedding application to have
its own config directory while reusing all of libghostty's config loading logic.

**Precedent:**

Ghostty already supports custom config paths via the `--config-file` CLI flag
(see
[Discussion #9434](https://github.com/ghostty-org/ghostty/discussions/9434)).
This change extends that capability to the C API for embedded use cases.

**Backwards compatibility:**

This is purely additive:

- `ghostty_config_load_default_files()` is unchanged
- Existing applications continue to work without modification
- New applications can opt into custom config directories

---

### 2. CLI Branding (TermSurf-Specific)

**Note:** Unlike other changes in this document, these modifications are
TermSurf-specific branding and are NOT intended for upstream submission.

**Files modified:**

- `src/cli/help.zig` - Changed usage text and app name references
- `src/cli/version.zig` - Changed version banner from "Ghostty" to "TermSurf"
- `src/cli/list_themes.zig` - Changed theme preview title

**Changes:**

- `Usage: ghostty` → `Usage: termsurf`
- `Ghostty terminal emulator` → `TermSurf terminal emulator`
- `ghostty -e top` → `termsurf -e top`
- `open -na Ghostty.app` → `open -na TermSurf.app`
- `Ghostty {version}` → `TermSurf {version}`
- `👻 Ghostty Theme Preview 👻` → `🏄 TermSurf Theme Preview 🏄`

**Why this change:**

TermSurf is a distinct product with its own branding. Users running CLI
commands should see "TermSurf" rather than "Ghostty" to avoid confusion.

**Upstream compatibility:**

These are string-only changes in isolated locations. When merging upstream
updates, these files may have conflicts but they will be trivial to resolve
(just keep the TermSurf strings).

---

## Future Changes

(This section will be updated as we make additional modifications to libghostty)

---

## Submitting Upstream

When ready to submit these changes:

1. Review
   [Ghostty's contribution guidelines](https://github.com/ghostty-org/ghostty)
2. Start a GitHub Discussion explaining the use case (embedding libghostty)
3. Reference this document for technical details
4. Submit PR(s) after discussion reaches consensus
