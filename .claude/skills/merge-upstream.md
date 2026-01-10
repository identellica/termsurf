---
name: merge-upstream
description: "Merge upstream Ghostty changes into TermSurf"
---

# Merge Upstream

Read and follow the process documented in `docs/merge-upstream.md`.

## Steps

1. **Read the documentation** - Read `docs/merge-upstream.md` to understand the full process and conflict resolution strategies.

2. **Pre-merge checklist** - Ensure working tree is clean, all changes committed, and current build passes.

3. **Fetch and review upstream**
   - `git fetch upstream`
   - Check how many commits behind: `git rev-list --count HEAD..upstream/main`
   - Review what changed in areas we've modified

4. **Merge upstream**
   - `git merge upstream/main -m "Merge upstream Ghostty (N commits)"`
   - Resolve any conflicts using the guide in the documentation

5. **Fix build errors** - API changes in libghostty may require updates to termsurf-macos Swift code. Fix any compilation errors.

6. **Review and port macos/ changes** - Check what changed in `macos/Sources/` and port relevant changes to `termsurf-macos/Sources/`. See the "Review macOS App Changes" section in the documentation.

7. **Verify and commit**
   - Run `zig build` and `xcodebuild` to verify everything compiles
   - Commit any additional fixes needed after the merge
