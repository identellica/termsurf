# Changelog

## v0.1.5

### Fixes

- **Google.com and other sites displaying incorrectly**: Fixed websites serving
  mobile/simplified layouts to WKWebView. Root cause: WKWebView doesn't send the
  `Upgrade-Insecure-Requests` HTTP header that Safari sends. We now inject this
  header on all HTTP/HTTPS requests. See [docs/wkwebview.md](docs/wkwebview.md).
- **User-Agent**: Set Safari User-Agent string to avoid being detected as an
  embedded webview.

## v0.1.4

### Fixes

- **web symlink arguments**: Fixed `web <url>` not passing URL to the browser
  (e.g., `web google.com` incorrectly opened the default homepage instead of
  google.com). The `web` symlink now correctly forwards all arguments.

## v0.1.3

### Improvements

- **CLI binary renamed**: The CLI binary is now `termsurf` instead of `ghostty`,
  matching the app name
- **Integrated web command**: The `web` CLI tool is now integrated into the main
  binary as `termsurf +web` (e.g., `termsurf +web open https://example.com`)
- **Multi-call binary**: A `web` symlink is included for convenience—you can run
  `web open <url>` directly instead of `termsurf +web open <url>`
- **Surfer branding**: Changed ghost emoji (👻) to surfer emoji (🏄) throughout
  the app

## v0.1.2

### Fixes

- **cmd+c/v/x in insert mode**: Copy, paste, and cut now work in the URL field
  when editing
- **cmd+z/Z in insert mode**: Undo and redo now work in the URL field when
  editing

### Improvements

- **Native control bar styling**: The webview control bar now uses native macOS
  colors and widgets that respect light/dark mode

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
