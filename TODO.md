# TermSurf CEF Integration Plan

This document tracks the integration of Chromium Embedded Framework (CEF) into TermSurf, enabling browser panes within the terminal.

## Why CEF?

- **Consistent cross-platform API** - Same C API on macOS, Linux, Windows
- **Full Chrome DevTools** - Essential for web developers
- **Profile support** - Different cache paths = isolated sessions (cookies, localStorage)
- **Console message capture** - Native `OnConsoleMessage` callback for stdout/stderr bridging
- **Binary size is acceptable** - ~150-200MB, but provides full browser capabilities

## Phase 1: Setup & Foundation ✓

Download CEF and understand its structure before writing any code.

- [x] Download latest stable CEF macOS arm64 build from cef-builds.spotifycdn.com
- [x] Extract to `termsurf-macos/Frameworks/cef/` (v143.0.13, Chromium 143.0.7499.170)
- [x] Document the directory structure (headers, framework, resources)
- [x] Read key C API headers we need:
  - [x] `include/capi/cef_app_capi.h` - initialization
  - [x] `include/capi/cef_browser_capi.h` - browser control
  - [x] `include/capi/cef_client_capi.h` - client handlers
  - [x] `include/capi/cef_display_handler_capi.h` - console messages
  - [x] `include/capi/cef_life_span_handler_capi.h` - browser lifecycle
  - [x] `include/capi/cef_request_context_capi.h` - profiles
- [x] Document the exact C function signatures we need to wrap

### CEF Directory Structure

```
termsurf-macos/Frameworks/cef/
├── include/
│   ├── capi/                    # C API headers (what we use)
│   │   ├── cef_app_capi.h
│   │   ├── cef_browser_capi.h
│   │   ├── cef_client_capi.h
│   │   ├── cef_display_handler_capi.h
│   │   ├── cef_life_span_handler_capi.h
│   │   └── cef_request_context_capi.h
│   └── internal/
│       ├── cef_types.h          # Settings structures
│       └── cef_types_mac.h      # macOS-specific types (NSView*, etc.)
├── Release/
│   └── Chromium Embedded Framework.framework
├── Resources/                   # Locale files, pak resources
└── libcef_dll_wrapper/          # C++ wrapper (not needed for C API)
```

### Required C Function Signatures

**Initialization (cef_app_capi.h)**
```c
// Initialize CEF - call once at app startup
int cef_initialize(
    const cef_main_args_t* args,           // argc/argv
    const cef_settings_t* settings,        // Global settings
    cef_app_t* application,                // App callbacks (optional)
    void* windows_sandbox_info             // NULL on macOS
);

// Shutdown CEF - call once at app exit
void cef_shutdown(void);

// Process one iteration of CEF message loop - for run loop integration
void cef_do_message_loop_work(void);

// Run CEF message loop (blocks) - alternative to do_message_loop_work
void cef_run_message_loop(void);
```

**Browser Creation (cef_browser_capi.h)**
```c
// Create browser asynchronously (recommended)
int cef_browser_host_create_browser(
    const cef_window_info_t* windowInfo,       // Parent view, bounds
    cef_client_t* client,                      // Event handlers
    const cef_string_t* url,                   // Initial URL
    const cef_browser_settings_t* settings,    // Browser settings
    cef_dictionary_value_t* extra_info,        // NULL usually
    cef_request_context_t* request_context     // Profile/cache context
);

// Create browser synchronously (UI thread only)
cef_browser_t* cef_browser_host_create_browser_sync(...);  // Same params
```

**Browser Control (cef_browser_t struct)**
```c
// Navigation
void (*go_back)(cef_browser_t* self);
void (*go_forward)(cef_browser_t* self);
void (*reload)(cef_browser_t* self);
void (*stop_load)(cef_browser_t* self);
int (*can_go_back)(cef_browser_t* self);
int (*can_go_forward)(cef_browser_t* self);
int (*is_loading)(cef_browser_t* self);

// Get host for additional control
cef_browser_host_t* (*get_host)(cef_browser_t* self);

// Frame access (for URL loading)
cef_frame_t* (*get_main_frame)(cef_browser_t* self);
```

**Browser Host (cef_browser_host_t struct)**
```c
// Close browser
void (*close_browser)(cef_browser_host_t* self, int force_close);

// Focus
void (*set_focus)(cef_browser_host_t* self, int focus);

// DevTools
void (*show_dev_tools)(cef_browser_host_t* self, ...);
void (*close_dev_tools)(cef_browser_host_t* self);
int (*has_dev_tools)(cef_browser_host_t* self);

// Get window handle (NSView* on macOS)
cef_window_handle_t (*get_window_handle)(cef_browser_host_t* self);
```

**Client Interface (cef_client_t struct)**
```c
// Return handlers for various events
cef_display_handler_t* (*get_display_handler)(cef_client_t* self);
cef_life_span_handler_t* (*get_life_span_handler)(cef_client_t* self);
// ... other handlers (focus, keyboard, etc.)
```

**Console Messages (cef_display_handler_t struct)**
```c
// Called for console.log/warn/error/etc.
int (*on_console_message)(
    cef_display_handler_t* self,
    cef_browser_t* browser,
    cef_log_severity_t level,      // LOGSEVERITY_INFO, _WARNING, _ERROR, etc.
    const cef_string_t* message,   // Console message text
    const cef_string_t* source,    // Source file URL
    int line                       // Line number
);

// Log severity levels for routing to stdout/stderr:
// LOGSEVERITY_DEBUG, LOGSEVERITY_INFO    -> stdout
// LOGSEVERITY_WARNING, LOGSEVERITY_ERROR -> stderr
```

**Browser Lifecycle (cef_life_span_handler_t struct)**
```c
// Called after browser created - safe to use browser now
void (*on_after_created)(cef_life_span_handler_t* self, cef_browser_t* browser);

// Called when browser is about to close
int (*do_close)(cef_life_span_handler_t* self, cef_browser_t* browser);

// Called just before browser destroyed - release references
void (*on_before_close)(cef_life_span_handler_t* self, cef_browser_t* browser);
```

**Profile/Request Context (cef_request_context_capi.h)**
```c
// Get global context (shared cache)
cef_request_context_t* cef_request_context_get_global_context(void);

// Create isolated context with custom cache path
cef_request_context_t* cef_request_context_create_context(
    const cef_request_context_settings_t* settings,  // Contains cache_path
    cef_request_context_handler_t* handler           // NULL usually
);

// Settings struct for profile isolation
typedef struct _cef_request_context_settings_t {
    size_t size;
    cef_string_t cache_path;              // Profile directory path
    int persist_session_cookies;          // Save session cookies
    cef_string_t accept_language_list;    // Optional
    cef_string_t cookieable_schemes_list; // Optional
} cef_request_context_settings_t;
```

**Window Info for macOS (cef_types_mac.h)**
```c
typedef struct _cef_window_info_t {
    size_t size;
    cef_string_t window_name;
    cef_rect_t bounds;                     // Initial bounds
    int hidden;                            // Start hidden?
    cef_window_handle_t parent_view;       // NSView* parent
    int windowless_rendering_enabled;      // Off-screen rendering
    cef_window_handle_t view;              // Output: created NSView*
    cef_runtime_style_t runtime_style;     // Chrome or Alloy style
} cef_window_info_t;

// Handle types (all void* on macOS, actually NSView*)
typedef void* cef_window_handle_t;  // NSView*
```

**String Conversion**
```c
// CEF uses UTF-16 strings internally
typedef struct _cef_string_utf16_t {
    char16_t* str;
    size_t length;
    void (*dtor)(char16_t* str);  // Destructor
} cef_string_utf16_t;

// cef_string_t is typedef'd to cef_string_utf16_t
typedef cef_string_utf16_t cef_string_t;

// Conversion functions
int cef_string_utf8_to_utf16(const char* src, size_t src_len, cef_string_utf16_t* out);
int cef_string_utf16_to_utf8(const char16_t* src, size_t src_len, cef_string_utf8_t* out);
void cef_string_userfree_utf16_free(cef_string_userfree_utf16_t str);
```

**Reference Counting (cef_base_capi.h)**
```c
// All CEF objects inherit from this
typedef struct _cef_base_ref_counted_t {
    size_t size;
    void (*add_ref)(cef_base_ref_counted_t* self);
    int (*release)(cef_base_ref_counted_t* self);  // Returns 1 if deleted
    int (*has_one_ref)(cef_base_ref_counted_t* self);
    int (*has_at_least_one_ref)(cef_base_ref_counted_t* self);
} cef_base_ref_counted_t;
```

## Phase 2: Minimal Swift Bindings

Create a lean Swift wrapper (CEFKit) that exposes only what TermSurf needs.

### Structure
```
termsurf-macos/CEFKit/
├── Modules/CEF/
│   ├── module.modulemap
│   └── CEF.h
├── Core/
│   ├── CEFBase.swift
│   ├── CEFString.swift
│   └── CEFCallback.swift
├── CEFApp.swift
├── CEFBrowser.swift
├── CEFClient.swift
├── CEFDisplayHandler.swift
├── CEFRequestContext.swift
└── CEFSettings.swift
```

### Tasks
- [ ] Create module.modulemap to import CEF C headers
- [ ] Create umbrella header with only needed headers
- [ ] Verify Swift can see CEF types
- [ ] Implement `CEFString.swift` - Swift String ↔ cef_string_t conversion
- [ ] Implement `CEFBase.swift` - Reference counting wrapper
- [ ] Implement `CEFCallback.swift` - Swift callback marshalling pattern
- [ ] Implement `CEFApp.swift`:
  - [ ] `CEFApp.initialize(settings:)`
  - [ ] `CEFApp.shutdown()`
  - [ ] `CEFApp.doMessageLoopWork()`
- [ ] Implement `CEFBrowser.swift`:
  - [ ] `CEFBrowser.create(url:profile:client:)`
  - [ ] `browser.loadURL(_:)`
  - [ ] `browser.goBack()`, `goForward()`, `reload()`
  - [ ] `browser.close()`
  - [ ] `browser.view` - returns NSView
- [ ] Implement `CEFDisplayHandler.swift`:
  - [ ] Protocol with `onConsoleMessage(level:message:source:line:)`
  - [ ] C callback that marshals to Swift
- [ ] Implement `CEFRequestContext.swift`:
  - [ ] `CEFRequestContext.create(cachePath:)` for profile isolation

## Phase 3: Standalone Prototype

Test CEF integration in isolation before integrating with Ghostty.

- [ ] Create simple test macOS app (outside Ghostty)
- [ ] Display CEF browser in a window
- [ ] Add URL text field for navigation
- [ ] Test console message capture:
  - [ ] Load page with `console.log()`, `console.error()`
  - [ ] Verify messages arrive in Swift callback
  - [ ] Implement JSON.stringify workaround for multiple arguments
- [ ] Test profile isolation:
  - [ ] Create two browsers with different cache paths
  - [ ] Verify separate cookies/localStorage
- [ ] Test run loop integration:
  - [ ] Try `cef_do_message_loop_work()` with timer
  - [ ] Verify browser + app both responsive
  - [ ] Test alternative: `multi_threaded_message_loop = true`

## Phase 4: TermSurf Integration

Integrate CEFKit into the TermSurf app and connect it to the pane system.

- [ ] Add CEFKit sources to termsurf-macos Xcode project
- [ ] Link Chromium Embedded Framework
- [ ] Configure framework search paths
- [ ] Handle code signing for CEF framework
- [ ] Integrate with AppKit run loop:
  - [ ] Add timer/dispatch source for `cef_do_message_loop_work()`
  - [ ] Ensure terminal rendering unaffected
- [ ] Extend SplitTree for browser panes:
  - [ ] Add `PaneContent.browser(CEFBrowserView)` case
  - [ ] Create `CEFBrowserView` wrapper
  - [ ] Handle focus routing
- [ ] Implement `termsurf open` command:
  - [ ] Parse: `termsurf open [--profile NAME] URL`
  - [ ] Create browser with appropriate profile
  - [ ] Replace/split current pane with browser
- [ ] Implement console output bridging:
  - [ ] Route `console.log` → stdout
  - [ ] Route `console.error` → stderr
  - [ ] Inject JS for JSON.stringify workaround
- [ ] Implement browser controls:
  - [ ] Ctrl+C to close browser (return to terminal)
  - [ ] Navigation shortcuts
  - [ ] URL display

## Phase 5: Polish & Documentation

Final polish and documentation updates.

- [ ] Profile management:
  - [ ] Default: `~/.termsurf/profiles/default/`
  - [ ] Named: `~/.termsurf/profiles/{name}/`
- [ ] DevTools support:
  - [ ] Command: `termsurf devtools`
  - [ ] Keyboard shortcut (Cmd+Option+I)
- [ ] Update documentation:
  - [ ] ARCHITECTURE.md - CEF integration details
  - [ ] ROADMAP.md - mark completed milestones
  - [ ] Document profile system
  - [ ] Document console bridging
- [ ] Binary distribution:
  - [ ] Bundle CEF framework with app
  - [ ] Document build process

## Resources

- [CEF Builds](https://cef-builds.spotifycdn.com/index.html) - Official binary distributions
- [CEF C API Docs](https://cef-builds.spotifycdn.com/docs/stable.html) - API documentation
- [CEF Wiki](https://bitbucket.org/chromiumembedded/cef/wiki/Home) - General usage guide
- [CEF.swift](../CEF.swift/) - Reference implementation (outdated but informative)

## Notes

- CEF.swift (cloned to repo root) is for reference only - we're building our own minimal wrapper
- Console message callback only receives first argument; use JS injection to JSON.stringify all args
- CEF takes over message loop by default; use `cef_do_message_loop_work()` to integrate with existing loop
