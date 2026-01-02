// CEF.h - Umbrella header for CEF C API
// Only includes the headers needed for TermSurf

#ifndef CEF_UMBRELLA_H
#define CEF_UMBRELLA_H

// Base types and reference counting
#include "include/capi/cef_base_capi.h"

// Initialization and message loop
#include "include/capi/cef_app_capi.h"

// Browser creation and control
#include "include/capi/cef_browser_capi.h"

// Client interface (returns handlers)
#include "include/capi/cef_client_capi.h"

// Display handler (console messages, title changes)
#include "include/capi/cef_display_handler_capi.h"

// Life span handler (browser lifecycle)
#include "include/capi/cef_life_span_handler_capi.h"

// Request context (profile isolation)
#include "include/capi/cef_request_context_capi.h"

// Frame for URL loading
#include "include/capi/cef_frame_capi.h"

// Types (settings, window info, etc.)
#include "include/internal/cef_types.h"

// macOS-specific types (NSView handle)
#if defined(__APPLE__)
#include "include/internal/cef_types_mac.h"
#endif

// String utilities
#include "include/internal/cef_string.h"
#include "include/internal/cef_string_types.h"

#endif // CEF_UMBRELLA_H
