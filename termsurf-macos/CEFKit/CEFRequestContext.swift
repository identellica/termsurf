// CEFRequestContext.swift
// CEFKit - Swift bindings for CEF
//
// Request context for browser profile isolation.
// Each context with a different cache path has separate cookies, localStorage, etc.

import Foundation
import CEF

// MARK: - CEFRequestContext

/// A request context provides isolated storage for browsers.
/// Create contexts with different cache paths for separate browser profiles.
///
/// Usage:
/// ```swift
/// // Create an isolated profile
/// let workProfile = CEFRequestContext.create(cachePath: "~/.termsurf/profiles/work")
/// let browser = try CEFBrowser.create(url: "https://example.com", requestContext: workProfile)
///
/// // Use global (shared) context
/// let globalContext = CEFRequestContext.global
/// ```
public final class CEFRequestContext {
    private var cefContext: CEFBase<cef_request_context_t>?

    private init(cefContext: CEFBase<cef_request_context_t>) {
        self.cefContext = cefContext
    }

    // MARK: - Factory Methods

    /// Get the global (default) request context.
    /// All browsers using this context share the same cache and cookies.
    public static var global: CEFRequestContext? {
        guard let ptr = cef_request_context_get_global_context() else {
            return nil
        }
        guard let base = CEFBase(ptr) else {
            return nil
        }
        return CEFRequestContext(cefContext: base)
    }

    /// Create a new request context with the specified settings.
    /// - Parameter settings: Context settings including cache path
    /// - Returns: New request context, or nil if creation failed
    public static func create(settings: CEFRequestContextSettings) -> CEFRequestContext? {
        var cefSettings = settings.toCEF()
        guard let ptr = cef_request_context_create_context(&cefSettings, nil) else {
            return nil
        }
        guard let base = CEFBase(ptr) else {
            return nil
        }
        return CEFRequestContext(cefContext: base)
    }

    /// Create a new request context with the specified cache path.
    /// Convenience method for creating isolated profiles.
    /// - Parameter cachePath: Path to store profile data (cookies, localStorage, etc.)
    /// - Returns: New request context, or nil if creation failed
    public static func create(cachePath: String) -> CEFRequestContext? {
        let settings = CEFRequestContextSettings(cachePath: cachePath)
        return create(settings: settings)
    }

    // MARK: - Properties

    /// Get the cache path for this context.
    /// Returns nil for incognito (in-memory) contexts.
    public var cachePath: String? {
        guard let ctx = cefContext?.cefObject,
              let getCachePath = ctx.pointee.get_cache_path else {
            return nil
        }
        guard let cefStr = getCachePath(ctx) else {
            return nil
        }
        defer { cef_string_userfree_utf16_free(cefStr) }
        return cefStringToSwift(cefStr.pointee)
    }

    /// Whether this is the global context.
    public var isGlobal: Bool {
        guard let ctx = cefContext?.cefObject,
              let isGlobal = ctx.pointee.is_global else {
            return false
        }
        return isGlobal(ctx) != 0
    }

    // MARK: - Internal

    /// Get the underlying CEF pointer for use when creating browsers.
    var cefContextPtr: UnsafeMutablePointer<cef_request_context_t>? {
        return cefContext?.cefObject
    }
}
