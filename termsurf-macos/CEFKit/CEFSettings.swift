// CEFSettings.swift
// CEFKit - Swift bindings for CEF
//
// Settings structures for CEF initialization and browser creation.

import Foundation
import CEF

// MARK: - CEF Application Settings

/// Settings for CEF initialization.
/// These are passed to CEFApp.initialize().
public struct CEFSettings {
    /// Path to the CEF framework directory (macOS).
    /// If empty, CEF looks in Contents/Frameworks/Chromium Embedded Framework.framework
    public var frameworkDirPath: String = ""

    /// Root directory for cache data.
    /// All profile cache paths must be under this directory.
    public var rootCachePath: String = ""

    /// Default cache path for the global browser context.
    /// If empty, browsers use "incognito mode" (in-memory cache).
    public var cachePath: String = ""

    /// User agent string. If empty, uses default Chrome user agent.
    public var userAgent: String = ""

    /// Locale (e.g., "en-US"). If empty, uses system default.
    public var locale: String = ""

    /// Log file path. If empty, uses default location.
    public var logFile: String = ""

    /// Log severity level.
    public var logSeverity: LogSeverity = .default

    /// Disable sandbox for sub-processes.
    public var noSandbox: Bool = true

    /// Enable windowless (off-screen) rendering.
    public var windowlessRenderingEnabled: Bool = false

    public init() {}

    /// Log severity levels
    public enum LogSeverity: Int32 {
        case `default` = 0  // LOGSEVERITY_DEFAULT
        case verbose = 1    // LOGSEVERITY_VERBOSE
        case info = 2       // LOGSEVERITY_INFO
        case warning = 3    // LOGSEVERITY_WARNING
        case error = 4      // LOGSEVERITY_ERROR
        case fatal = 5      // LOGSEVERITY_FATAL
        case disable = 99   // LOGSEVERITY_DISABLE
    }

    /// Convert to CEF settings struct
    func toCEF() -> cef_settings_t {
        var settings = cef_settings_t()
        settings.size = MemoryLayout<cef_settings_t>.size

        settings.no_sandbox = noSandbox ? 1 : 0
        settings.windowless_rendering_enabled = windowlessRenderingEnabled ? 1 : 0
        settings.log_severity = cef_log_severity_t(rawValue: UInt32(logSeverity.rawValue))

        if !frameworkDirPath.isEmpty {
            setCefString(frameworkDirPath, to: &settings.framework_dir_path)
        }
        if !rootCachePath.isEmpty {
            setCefString(rootCachePath, to: &settings.root_cache_path)
        }
        if !cachePath.isEmpty {
            setCefString(cachePath, to: &settings.cache_path)
        }
        if !userAgent.isEmpty {
            setCefString(userAgent, to: &settings.user_agent)
        }
        if !locale.isEmpty {
            setCefString(locale, to: &settings.locale)
        }
        if !logFile.isEmpty {
            setCefString(logFile, to: &settings.log_file)
        }

        return settings
    }
}

// MARK: - Browser Settings

/// Settings for browser creation.
public struct CEFBrowserSettings {
    /// Frame rate for windowless rendering (1-60 fps).
    public var windowlessFrameRate: Int = 30

    public init() {}

    /// Convert to CEF browser settings struct
    func toCEF() -> cef_browser_settings_t {
        var settings = cef_browser_settings_t()
        settings.size = MemoryLayout<cef_browser_settings_t>.size
        settings.windowless_frame_rate = Int32(windowlessFrameRate)
        return settings
    }
}

// MARK: - Request Context Settings (Profiles)

/// Settings for creating a request context (profile).
/// Use different cache paths for isolated browser profiles.
public struct CEFRequestContextSettings {
    /// Cache path for this profile.
    /// Each unique path creates an isolated browser profile with separate cookies/localStorage.
    public var cachePath: String = ""

    /// Persist session cookies (cookies without expiry).
    public var persistSessionCookies: Bool = true

    public init() {}

    public init(cachePath: String) {
        self.cachePath = cachePath
    }

    /// Convert to CEF request context settings struct
    func toCEF() -> cef_request_context_settings_t {
        var settings = cef_request_context_settings_t()
        settings.size = MemoryLayout<cef_request_context_settings_t>.size
        settings.persist_session_cookies = persistSessionCookies ? 1 : 0

        if !cachePath.isEmpty {
            setCefString(cachePath, to: &settings.cache_path)
        }

        return settings
    }
}

// MARK: - Window Info

/// Window information for browser creation on macOS.
public struct CEFWindowInfo {
    /// Parent view (NSView) for the browser.
    public var parentView: UnsafeMutableRawPointer?

    /// Initial bounds for the browser view.
    public var bounds: CGRect = .zero

    /// Whether to create the view initially hidden.
    public var hidden: Bool = false

    /// Enable windowless (off-screen) rendering.
    public var windowlessRenderingEnabled: Bool = false

    public init() {}

    public init(parentView: UnsafeMutableRawPointer?, bounds: CGRect) {
        self.parentView = parentView
        self.bounds = bounds
    }

    /// Convert to CEF window info struct
    func toCEF() -> cef_window_info_t {
        var info = cef_window_info_t()
        info.size = MemoryLayout<cef_window_info_t>.size

        info.parent_view = parentView
        info.bounds.x = Int32(bounds.origin.x)
        info.bounds.y = Int32(bounds.origin.y)
        info.bounds.width = Int32(bounds.size.width)
        info.bounds.height = Int32(bounds.size.height)
        info.hidden = hidden ? 1 : 0
        info.windowless_rendering_enabled = windowlessRenderingEnabled ? 1 : 0

        return info
    }
}
