# CEF Integration Guide

This document covers the Chromium Embedded Framework (CEF) integration for TermSurf browser panes.

## Current Status

**Status:** ⚠️ Deferred - Using WKWebView for MVP

CEF integration was attempted but encountered fundamental issues with Swift-to-C struct marshalling. The CEF C API's validation layer rejects structs created from Swift due to memory layout incompatibilities. See [Implementation Challenges](#implementation-challenges) below for details.

**Decision:** For the MVP, TermSurf uses Apple's WKWebView instead, which provides:
- Native Swift integration (no marshalling issues)
- Console capture via `WKScriptMessageHandler`
- Zero external dependencies
- Simpler implementation (~200 lines vs ~1000+ for CEF)

CEF remains a future option if Chrome DevTools or Blink-specific features become necessary.

---

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

---

## Implementation Challenges

This section documents the technical challenges encountered while attempting to integrate CEF with Swift, preserved for future reference.

### The Core Problem

When passing a `cef_app_t` struct to `cef_initialize()`, CEF's C-to-C++ wrapper validates the struct before use. This validation consistently failed with:

```
[FATAL:cef/libcef_dll/ctocpp/app_ctocpp.cc:118] CefApp_0_CToCpp called with invalid version -1
```

Or alternatively:

```
[FATAL:cef/libcef_dll/ctocpp/ctocpp_ref_counted.h:124] Cannot wrap struct with invalid base.size value (got 7598814392448188417, expected 80) at API version -1
```

### Why This Happens

CEF's C API is a wrapper around C++ classes. When you pass a C struct to CEF:

1. CEF reads the `base.size` field from the struct pointer
2. It validates that `size` matches the expected struct size (80 bytes for `cef_app_t`)
3. It checks callback function pointers are valid
4. If validation fails, it reports "invalid version -1"

The problem is that Swift's memory layout for classes doesn't match what CEF expects when we try to embed the C struct inside a Swift class.

### What We Tried

#### Attempt 1: Direct Struct Allocation

```swift
let appStructPtr = UnsafeMutablePointer<cef_app_t>.allocate(capacity: 1)
appStructPtr.initialize(to: cef_app_t())
appStructPtr.pointee.base.size = MemoryLayout<cef_app_t>.stride  // 80
appStructPtr.pointee.base.add_ref = { _ in }  // Closures
// ... assign other callbacks
cef_initialize(&mainArgs, &settings, appStructPtr, nil)
```

**Result:** Failed with "invalid version -1"

**Why:** The struct is allocated separately from any Swift object, so when CEF's validation reads memory at the struct pointer, it finds the correct size but something else fails in the validation chain.

#### Attempt 2: Hardcoded Sizes

We verified the expected sizes by examining CEF headers:
- `cef_app_t`: 80 bytes
- `cef_browser_process_handler_t`: 96 bytes

Hardcoding these values didn't help.

#### Attempt 3: Using stride vs size

CEF.swift (a working older implementation) uses `strideof()` (now `MemoryLayout<T>.stride`) instead of `sizeof()`. We tried both - neither worked.

#### Attempt 4: Global @convention(c) Functions

Swift closures that capture context cannot be converted to C function pointers. We moved all callbacks to global functions:

```swift
private let cefApp_addRef: @convention(c) (UnsafeMutablePointer<cef_base_ref_counted_t>?) -> Void = { _ in }
```

**Result:** Still failed with the same error.

#### Attempt 5: CEF.swift Marshaller Pattern

The [CEF.swift](https://github.com/aspect-apps/aspect/tree/main/aspect-platform/aspect-platform-cef/aspect-platform-cef-swift) project used a clever pattern: embed the C struct as the **first stored property** of a Swift class, placing it at a known offset (16 bytes) from the class pointer.

```swift
open class CEFMarshaller<TStruct> {
    // MUST be first property - at offset 16 from class pointer
    public var cefStruct: TStruct

    func toCEF() -> UnsafeMutablePointer<TStruct> {
        let selfPtr = Unmanaged.passUnretained(self).toOpaque()
        return selfPtr.advanced(by: 16).assumingMemoryBound(to: TStruct.self)
    }
}
```

We implemented this pattern with:
- `CEFMarshaller<TStruct>` base class
- `CEFAppHandler: CEFMarshaller<cef_app_t>`
- `CEFBrowserProcessHandler: CEFMarshaller<cef_browser_process_handler_t>`
- Global `@convention(c)` callbacks that recover the Swift object via pointer arithmetic

**Result:** Still failed. Even with `cefStruct.base.size` correctly set to 80, CEF rejected the struct.

**Debugging output showed:**
```
[CEFAppHandler] cef_app_t stride: 80
[CEFAppHandler] cefStruct.base.size = 80
[CEFAppHandler] size at offset 16: 80
```

The size was correct, but CEF still rejected it. This suggests either:
1. Modern Swift class layout differs from when CEF.swift worked
2. Additional validation beyond `base.size` is failing
3. The callback function pointers aren't being read correctly

### CEF.swift Analysis

The [CEF.swift](https://github.com/aspect-apps/aspect/tree/main/aspect-platform/aspect-platform-cef/aspect-platform-cef-swift) project (circa 2016-2017) successfully integrated CEF with Swift using these key techniques:

#### The Marshaller Pattern

```swift
class CEFMarshaller<TClass, TStruct> {
    var cefStruct: TStruct    // First property at offset 16
    var swiftObj: TClass      // Reference to handler

    static let kOffset = 16   // Swift class header size

    // Get C pointer from Swift object
    static func pass(obj: TClass) -> UnsafeMutablePointer<TStruct> {
        let marshaller = CEFMarshaller(obj: obj)
        return UnsafeMutablePointer(
            Unmanaged.passUnretained(marshaller).toOpaque().advanced(by: kOffset)
        )
    }

    // Recover Swift object from C pointer
    static func get(_ ptr: UnsafeMutablePointer<TStruct>) -> TClass? {
        let raw = UnsafeMutableRawPointer(ptr).advanced(by: -kOffset)
        let marshaller = Unmanaged<CEFMarshaller>.fromOpaque(raw).takeUnretainedValue()
        return marshaller.swiftObj
    }
}
```

#### Callback Marshalling

Each CEF struct type has an extension that assigns callbacks:

```swift
extension cef_app_t: CEFCallbackMarshalling {
    mutating func marshalCallbacks() {
        get_browser_process_handler = { ptr in
            guard let app = CEFAppMarshaller.get(ptr) else { return nil }
            return app.browserProcessHandler?.toCEF()
        }
        // ... other callbacks
    }
}
```

#### Why It May Have Stopped Working

1. **Swift ABI changes:** Swift's class memory layout may have changed since 2016
2. **CEF version differences:** Newer CEF versions may have stricter validation
3. **Compiler optimizations:** Modern Swift may reorder properties or add padding differently

### Files Created

The following files were created in `termsurf-macos/CEFKit/`:

```
CEFKit/
├── CEFApp.swift                 # Main initialization API
├── CEFSettings.swift            # Settings structs
├── CEFBrowser.swift             # Browser wrapper
├── CEFClient.swift              # Client handler
├── CEFLifeSpanHandler.swift     # Lifecycle callbacks
├── CEFDisplayHandler.swift      # Console message handling
├── CEFRequestContext.swift      # Profile/cache context
├── Core/
│   ├── CEFString.swift          # String conversion utilities
│   ├── CEFCallback.swift        # Callback helpers
│   ├── CEFBase.swift            # Base protocols
│   └── CEFMarshaller.swift      # Marshaller pattern implementation
└── Handlers/
    ├── CEFAppHandler.swift      # cef_app_t handler
    └── CEFBrowserProcessHandler.swift  # Browser process handler
```

**State when abandoned:** The marshaller infrastructure was complete, but `cef_initialize()` still rejected the app handler struct.

### Helper Apps

CEF requires helper apps for its multi-process architecture. We successfully configured:

```
CEFTest.app/Contents/Frameworks/
└── Chromium Embedded Framework.framework/
    └── Helpers/
        ├── CEFTest Helper.app                    (bundle ID: com.termsurf.ceftest.helper)
        ├── CEFTest Helper (GPU).app              (bundle ID: com.termsurf.ceftest.helper.GPU)
        ├── CEFTest Helper (Renderer).app         (bundle ID: com.termsurf.ceftest.helper.Renderer)
        ├── CEFTest Helper (Plugin).app           (bundle ID: com.termsurf.ceftest.helper.Plugin)
        └── CEFTest Helper (Alerts).app           (bundle ID: com.termsurf.ceftest.helper.Alerts)
```

The helper app bundle IDs must follow the pattern `{main_bundle_id}.helper.{type}`.

### Recommendations for Future Work

If CEF integration is revisited:

1. **Try the C++ API instead of C API**
   - Use a bridging header to expose C++ classes to Swift
   - This avoids the struct marshalling problem entirely
   - The C++ wrapper handles reference counting internally

2. **Test with different CEF versions**
   - CEF 133.x may have specific bugs
   - Try an older version that's known to work with CEF.swift

3. **Investigate the cefcapi project**
   - [cefcapi](https://github.com/aspect-apps/aspect/tree/main/aspect-platform/aspect-platform-cef/aspect-platform-cef-capi) shows C API usage patterns
   - May have insights on struct initialization

4. **Check Swift class layout**
   - Use memory debugging to verify actual offset of first property
   - The 16-byte assumption may no longer hold

5. **Consider Objective-C bridge**
   - Write CEF integration in Objective-C
   - Expose a clean Swift API on top
   - Objective-C has more predictable memory layout

### References

- [CEF Forum: CefApp_0_CToCpp invalid version](https://magpcss.org/ceforum/viewtopic.php?f=6&t=19114) - Discussion of this exact error
- [CEF.swift](https://github.com/aspect-apps/aspect/tree/main/aspect-platform/aspect-platform-cef/aspect-platform-cef-swift) - Working (older) Swift bindings
- [cefcapi](https://github.com/aspect-apps/aspect/tree/main/aspect-platform/aspect-platform-cef/aspect-platform-cef-capi) - C API usage example
