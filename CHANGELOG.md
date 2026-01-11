# Changelog

## v0.8.0

### New Features

- **Page zoom**: Added keyboard shortcuts for page zoom. cmd+= zooms in, cmd+-
  zooms out, and cmd+0 resets to 100%. Works in all webview modes.

### Documentation

- Updated TODO.md and webview.md to reflect completed features and document
  cross-origin download limitations.

## v0.7.0

### New Features

- **Camera/microphone permissions**: Websites can now request camera and
  microphone access. A native permission dialog appears with Allow/Don't Allow
  options.
- **HTTP Basic Auth**: Password-protected websites now show a login dialog
  instead of failing silently.
- **Crash recovery**: When WebKit crashes, a dialog offers to reload or close
  the webview instead of showing a blank screen.

### Internal

- **Media test page**: Added `/test-media` route to the website for testing
  camera/microphone permissions.

## v0.6.0

### New Features

- **Download support**: Added file download support for webviews. Same-origin
  downloads with the `download` attribute, blob URL downloads (JavaScript-
  generated files), and non-displayable MIME types all trigger the native macOS
  save panel.

### Internal

- **Download test page**: Added `/test-download` route to the website for
  testing download functionality.

## v0.5.0

### New Features

- **File uploads**: Added support for file uploads in webviews. Single file,
  multiple files, and directory uploads all work via the native macOS file
  picker.

### Website

- **Version badge**: The website header now shows the current release version.

## v0.4.0

### New Features

- **JavaScript dialogs**: Added support for `alert()`, `confirm()`, and
  `prompt()` dialogs in webviews. Dialogs show the origin hostname for security,
  and include a checkbox to suppress further dialogs from the same page.

## v0.3.0

### New Features

- **Dynamic tab titles for webviews**: When browsing with `web`, the tab title
  now shows the web page's title (e.g., "Google" instead of "web google.com").
  Title automatically reverts to terminal title when webview is closed.

### Website

- **Favicon**: Added favicon to termsurf.com

## v0.2.0

### New Features

- **termsurf.com website**: Launched project website with commit log, built with
  TanStack Router and Bun SSR, deployed to Fly.io
- **Default homepage**: Browser now opens termsurf.com by default when no URL is
  specified
- **Expandable commits**: Website commit log entries expand to show full commit
  messages with GitHub links
- **GitPoet skill**: New Claude skill for writing poetic commit messages

### Improvements

- **Web file command**: Added `web file` command for opening local files in the
  browser pane

## v0.1.7

### Improvements

- **URL input click to highlight**: Click on the URL input box to highlight all
  text for easy replacement
- **ctrl+c to exit browse mode**: Press ctrl+c to exit browse mode and return to
  control mode

### Internal

- **Skills directory restructure**: Reorganized `.claude/skills/` to use proper
  folder structure with `SKILL.md` files

## v0.1.6

### Upstream Merge

- Merged 113 commits from upstream Ghostty, bringing in numerous bug fixes and
  improvements

### New Features

- **Use Selection for Find**: New menu item (Edit > Find > Use Selection for
  Find) to search for selected text
- **Jump to Selection**: New menu item (Edit > Find > Jump to Selection) to
  scroll to the current selection

### Fixes

- **Window drag bug (#10110)**: Fixed pane grab handles incorrectly triggering
  window drag instead of pane drag
- **Search focus race condition**: Fixed intermittent issue where search field
  wouldn't receive focus
- **Bell indicator in title override**: Bell indicator now correctly appears
  when using a title override
- **Memory leak**: Fixed memory leak when pruning scrollback with non-standard
  pages

### Improvements

- **Key binding handling**: Improved integration with system menu for key
  bindings
- **SplitTree API**: Refactored to use more idiomatic Swift naming conventions
- **Repository moved**: Now at github.com/termsurf/termsurf
- **Bundle identifiers**: Fixed to use com.termsurf.* namespace

## v0.1.5

### Fixes

- **Google.com and other sites displaying incorrectly**: Fixed websites serving
  mobile/simplified layouts to WKWebView. Root cause: WKWebView doesn't send the
  `Upgrade-Insecure-Requests` HTTP header that Safari sends. We now inject this
  header on all HTTP/HTTPS requests. See [docs/webview.md](docs/webview.md).
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
