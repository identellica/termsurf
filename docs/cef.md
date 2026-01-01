# CEF Integration Guide

This document covers the Chromium Embedded Framework (CEF) integration for TermSurf browser panes.

## Overview

TermSurf uses CEF to embed Chromium browsers as first-class panes within the terminal. This enables:
- Full Chrome DevTools support
- Isolated browser profiles (separate cookies/localStorage)
- Console message capture (stdout/stderr bridging)
- Consistent cross-platform API

## Installation

CEF binary distribution is located at `termsurf-macos/Frameworks/cef/`.

**Current version:** v143.0.13 (Chromium 143.0.7499.170, macOS ARM64)

**Download source:** https://cef-builds.spotifycdn.com/index.html

## Directory Structure

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

## C API Reference

TermSurf uses the CEF C API (`include/capi/`) rather than the C++ API for easier Swift interop.

### Initialization

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

### Browser Creation

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

### Browser Control

```c
// cef_browser_t struct methods

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

### Browser Host

```c
// cef_browser_host_t struct methods

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

### Client Interface

```c
// cef_client_t struct methods

// Return handlers for various events
cef_display_handler_t* (*get_display_handler)(cef_client_t* self);
cef_life_span_handler_t* (*get_life_span_handler)(cef_client_t* self);
// ... other handlers (focus, keyboard, etc.)
```

### Console Messages

```c
// cef_display_handler_t struct method

// Called for console.log/warn/error/etc.
int (*on_console_message)(
    cef_display_handler_t* self,
    cef_browser_t* browser,
    cef_log_severity_t level,      // LOGSEVERITY_INFO, _WARNING, _ERROR, etc.
    const cef_string_t* message,   // Console message text
    const cef_string_t* source,    // Source file URL
    int line                       // Line number
);
```

**Log severity routing:**
- `LOGSEVERITY_DEBUG`, `LOGSEVERITY_INFO` → stdout
- `LOGSEVERITY_WARNING`, `LOGSEVERITY_ERROR` → stderr

**Note:** The callback only receives the first argument passed to `console.log()`. To capture multiple arguments, inject JavaScript that wraps console methods to JSON.stringify all arguments.

### Browser Lifecycle

```c
// cef_life_span_handler_t struct methods

// Called after browser created - safe to use browser now
void (*on_after_created)(cef_life_span_handler_t* self, cef_browser_t* browser);

// Called when browser is about to close
int (*do_close)(cef_life_span_handler_t* self, cef_browser_t* browser);

// Called just before browser destroyed - release references
void (*on_before_close)(cef_life_span_handler_t* self, cef_browser_t* browser);
```

### Profile/Request Context

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

**Profile paths:**
- Default: `~/.termsurf/profiles/default/`
- Named: `~/.termsurf/profiles/{name}/`

### Window Info (macOS)

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

### String Conversion

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

### Reference Counting

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

## Run Loop Integration

CEF needs to process its message loop. Two options:

1. **Integrated loop** (recommended for TermSurf):
   ```swift
   // Call periodically from main run loop (e.g., via Timer or DispatchSource)
   cef_do_message_loop_work()
   ```

2. **Multi-threaded loop** (Windows/Linux only):
   ```c
   settings.multi_threaded_message_loop = 1;
   ```

## Resources

- [CEF Builds](https://cef-builds.spotifycdn.com/index.html) - Official binary distributions
- [CEF C API Docs](https://cef-builds.spotifycdn.com/docs/stable.html) - API documentation
- [CEF Wiki](https://bitbucket.org/chromiumembedded/cef/wiki/Home) - General usage guide
