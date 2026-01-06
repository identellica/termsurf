# Changelog

## v0.1.1

### Fixes

- **cmd+r refresh**: Press cmd+r to refresh the current webview
- **target="_blank" links**: Links that request a new window now navigate in the
  current webview instead of being silently ignored

## v0.1.0

Initial release of TermSurf, a terminal emulator with integrated browser panes.

### Features

- **CLI-invoked browser**: `web open <url>` opens a webview overlay that blocks
  like `vim` or `less`
- **Console bridging**: `console.log()` routes to stdout, `console.error()` to
  stderr
- **Three-mode keyboard**: Control mode (terminal keybindings), Browse mode
  (browser focus), Insert mode (edit URL)
- **Profile isolation**: `--profile <name>` for separate sessions (cookies,
  localStorage, etc.)
- **Incognito mode**: `--incognito` for ephemeral sessions
- **JavaScript API**: `--js-api` enables `window.termsurf.exit(code)` for
  programmatic control
- **Webview stacking**: Multiple concurrent webviews per pane with stack
  indicator
- **Bookmarks**: cmd+b to bookmark current page
- **Safari Web Inspector**: cmd+alt+i to open developer tools
