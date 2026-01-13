---
name: merge-upstream
description: "Merge upstream changes from Ghostty, WezTerm, or cef-rs into TermSurf"
arguments:
  - name: repo
    description: "Which upstream to merge: ghostty, wezterm, or cef-rs"
    required: true
---

# Merge Upstream

Merge changes from one of our upstream repositories into TermSurf.

## Usage

```
/merge-upstream <repo>
```

Where `<repo>` is one of:
- `ghostty` - Merge from ghostty-org/ghostty into ts1/
- `wezterm` - Merge from wez/wezterm into ts2/ and root
- `cef-rs` - Merge from tauri-apps/cef-rs into cef-rs/

## Upstream Repositories

| Repo | Directory | Remote | Upstream URL | Branch |
|------|-----------|--------|--------------|--------|
| Ghostty | `ts1/` | `upstream` | github.com/ghostty-org/ghostty | main |
| WezTerm | `ts2/` + root | `wezterm-upstream` | github.com/wez/wezterm | main |
| cef-rs | `cef-rs/` | `cef-rs-upstream` | github.com/tauri-apps/cef-rs | dev |

## Steps

1. **Read the documentation** - Read `docs/merge-upstream.md` for the full process, especially the repo-specific conflict resolution guides.

2. **Pre-merge checklist**
   - Ensure working tree is clean (`git status`)
   - All changes committed
   - Note current HEAD: `git rev-parse HEAD`

3. **Fetch and review upstream**
   ```bash
   # For ghostty:
   git fetch upstream
   git rev-list --count HEAD..upstream/main -- ts1/

   # For wezterm:
   git fetch wezterm-upstream
   git rev-list --count HEAD..wezterm-upstream/main

   # For cef-rs:
   git fetch cef-rs-upstream
   git rev-list --count HEAD..cef-rs-upstream/dev -- cef-rs/
   ```

4. **Merge upstream**
   ```bash
   # For ghostty (uses subtree merge):
   git merge -X subtree=ts1 upstream/main -m "Merge upstream Ghostty"

   # For wezterm (uses subtree merge):
   git merge -X subtree=ts2 wezterm-upstream/main -m "Merge upstream WezTerm"

   # For cef-rs (uses subtree merge):
   git merge -X subtree=cef-rs cef-rs-upstream/dev -m "Merge upstream cef-rs"
   ```

5. **Resolve conflicts** - Use the repo-specific guide in `docs/merge-upstream.md`.

6. **Fix build errors** - API changes may require updates to our code.

7. **Verify and test**
   - For Ghostty: `cd ts1 && zig build && ./scripts/build-debug.sh`
   - For WezTerm: `cargo build` (from root)
   - For cef-rs: `cd cef-rs && cargo build --example osr`

8. **Commit** any additional fixes needed after the merge.
