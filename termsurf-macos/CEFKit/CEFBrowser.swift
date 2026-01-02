// CEFBrowser.swift
// CEFKit - Swift bindings for CEF
//
// Browser control for creating and managing browser instances.

import Foundation
import CEF

#if canImport(AppKit)
import AppKit
#endif

// MARK: - CEFBrowser

/// A browser instance that can display web content.
///
/// Usage:
/// ```swift
/// // Create a browser delegate
/// class BrowserDelegate: CEFClientDelegate {
///     func onConsoleMessage(browser: CEFBrowser?, level: CEFLogSeverity, message: String, source: String, line: Int) -> Bool {
///         if level.isError {
///             fputs("\(message)\n", stderr)
///         } else {
///             print(message)
///         }
///         return true
///     }
/// }
///
/// // Create client and browser
/// let delegate = BrowserDelegate()
/// let client = CEFClient(delegate: delegate)
///
/// var windowInfo = CEFWindowInfo()
/// windowInfo.parentView = parentNSView
/// windowInfo.bounds = CGRect(x: 0, y: 0, width: 800, height: 600)
///
/// let browser = try CEFBrowser.create(
///     url: "https://example.com",
///     windowInfo: windowInfo,
///     client: client
/// )
///
/// // Navigate
/// browser?.loadURL("https://google.com")
///
/// // Navigation controls
/// browser?.goBack()
/// browser?.goForward()
/// browser?.reload()
/// ```
public final class CEFBrowser {
    private var cefBrowser: CEFBase<cef_browser_t>?

    // Keep a reference to the client to prevent premature deallocation
    private var client: CEFClient?

    /// Initialize with an existing CEF browser pointer.
    /// Used internally when receiving browser from callbacks.
    init?(cefBrowser: UnsafeMutablePointer<cef_browser_t>?) {
        guard let ptr = cefBrowser else { return nil }
        // Add ref since we're keeping a reference
        self.cefBrowser = CEFBase(ptr, addRef: true)
    }

    /// Initialize with owned CEF browser pointer.
    private init?(ownedBrowser: UnsafeMutablePointer<cef_browser_t>?, client: CEFClient?) {
        guard let ptr = ownedBrowser else { return nil }
        self.cefBrowser = CEFBase(ptr)
        self.client = client
    }

    // MARK: - Factory Methods

    /// Create a new browser synchronously.
    /// Must be called on the UI thread.
    ///
    /// - Parameters:
    ///   - url: Initial URL to load
    ///   - windowInfo: Window/view configuration
    ///   - client: Client to handle browser events
    ///   - settings: Browser settings (optional)
    ///   - requestContext: Request context for profile isolation (optional, uses global if nil)
    /// - Returns: New browser instance
    /// - Throws: CEFError if browser creation fails
    public static func create(
        url: String,
        windowInfo: CEFWindowInfo,
        client: CEFClient,
        settings: CEFBrowserSettings = CEFBrowserSettings(),
        requestContext: CEFRequestContext? = nil
    ) throws -> CEFBrowser {
        guard CEFApp.isInitialized else {
            throw CEFError.notInitialized
        }

        var cefWindowInfo = windowInfo.toCEF()
        var cefSettings = settings.toCEF()

        let browserPtr = withCefString(url) { urlPtr in
            cef_browser_host_create_browser_sync(
                &cefWindowInfo,
                client.cefClientPtr,
                urlPtr,
                &cefSettings,
                nil,  // extra_info
                requestContext?.cefContextPtr
            )
        }

        guard let browser = CEFBrowser(ownedBrowser: browserPtr, client: client) else {
            throw CEFError.browserCreationFailed
        }

        return browser
    }

    /// Create a new browser asynchronously.
    /// The browser will be available in the onAfterCreated callback.
    ///
    /// - Parameters:
    ///   - url: Initial URL to load
    ///   - windowInfo: Window/view configuration
    ///   - client: Client to handle browser events
    ///   - settings: Browser settings (optional)
    ///   - requestContext: Request context for profile isolation (optional)
    /// - Returns: true if browser creation was initiated successfully
    public static func createAsync(
        url: String,
        windowInfo: CEFWindowInfo,
        client: CEFClient,
        settings: CEFBrowserSettings = CEFBrowserSettings(),
        requestContext: CEFRequestContext? = nil
    ) -> Bool {
        guard CEFApp.isInitialized else { return false }

        var cefWindowInfo = windowInfo.toCEF()
        var cefSettings = settings.toCEF()

        let result = withCefString(url) { urlPtr in
            cef_browser_host_create_browser(
                &cefWindowInfo,
                client.cefClientPtr,
                urlPtr,
                &cefSettings,
                nil,
                requestContext?.cefContextPtr
            )
        }

        return result != 0
    }

    // MARK: - Navigation

    /// Load a URL in the browser.
    /// - Parameter url: URL to load
    public func loadURL(_ url: String) {
        guard let browser = cefBrowser?.cefObject,
              let getMainFrame = browser.pointee.get_main_frame,
              let frame = getMainFrame(browser) else {
            return
        }

        // Load URL on the main frame
        withCefString(url) { urlPtr in
            frame.pointee.load_url?(frame, urlPtr)
        }

        // Release our reference to the frame
        frame.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1) { base in
            _ = base.pointee.release?(base)
        }
    }

    /// Navigate back in history.
    public func goBack() {
        guard let browser = cefBrowser?.cefObject else { return }
        browser.pointee.go_back?(browser)
    }

    /// Navigate forward in history.
    public func goForward() {
        guard let browser = cefBrowser?.cefObject else { return }
        browser.pointee.go_forward?(browser)
    }

    /// Reload the current page.
    public func reload() {
        guard let browser = cefBrowser?.cefObject else { return }
        browser.pointee.reload?(browser)
    }

    /// Reload the current page, ignoring cache.
    public func reloadIgnoreCache() {
        guard let browser = cefBrowser?.cefObject else { return }
        browser.pointee.reload_ignore_cache?(browser)
    }

    /// Stop loading the current page.
    public func stopLoad() {
        guard let browser = cefBrowser?.cefObject else { return }
        browser.pointee.stop_load?(browser)
    }

    // MARK: - State

    /// Whether the browser can navigate back.
    public var canGoBack: Bool {
        guard let browser = cefBrowser?.cefObject,
              let canGoBack = browser.pointee.can_go_back else {
            return false
        }
        return canGoBack(browser) != 0
    }

    /// Whether the browser can navigate forward.
    public var canGoForward: Bool {
        guard let browser = cefBrowser?.cefObject,
              let canGoForward = browser.pointee.can_go_forward else {
            return false
        }
        return canGoForward(browser) != 0
    }

    /// Whether the browser is currently loading.
    public var isLoading: Bool {
        guard let browser = cefBrowser?.cefObject,
              let isLoading = browser.pointee.is_loading else {
            return false
        }
        return isLoading(browser) != 0
    }

    // MARK: - Browser Host

    /// Get the browser host for additional control.
    private var host: UnsafeMutablePointer<cef_browser_host_t>? {
        guard let browser = cefBrowser?.cefObject,
              let getHost = browser.pointee.get_host else {
            return nil
        }
        return getHost(browser)
    }

    /// Close the browser.
    /// - Parameter force: If true, close immediately without firing unload events
    public func close(force: Bool = false) {
        guard let host = host else { return }
        host.pointee.close_browser?(host, force ? 1 : 0)

        // Release our reference to the host
        host.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1) { base in
            _ = base.pointee.release?(base)
        }
    }

    /// Set focus on the browser.
    /// - Parameter focus: true to focus, false to unfocus
    public func setFocus(_ focus: Bool) {
        guard let host = host else { return }
        host.pointee.set_focus?(host, focus ? 1 : 0)

        host.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1) { base in
            _ = base.pointee.release?(base)
        }
    }

    #if canImport(AppKit)
    /// Get the NSView for this browser (macOS only).
    /// Use this to add the browser view to your view hierarchy.
    public var view: NSView? {
        guard let host = host,
              let getWindowHandle = host.pointee.get_window_handle else {
            return nil
        }

        let handle = getWindowHandle(host)

        // Release our reference to the host
        host.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1) { base in
            _ = base.pointee.release?(base)
        }

        guard let handle = handle else { return nil }

        // Cast the window handle to NSView (it's actually NSView* on macOS)
        return Unmanaged<NSView>.fromOpaque(handle).takeUnretainedValue()
    }
    #endif

    // MARK: - DevTools

    /// Show the developer tools window.
    public func showDevTools() {
        guard let host = host else { return }

        var windowInfo = cef_window_info_t()
        windowInfo.size = MemoryLayout<cef_window_info_t>.size

        var settings = cef_browser_settings_t()
        settings.size = MemoryLayout<cef_browser_settings_t>.size

        host.pointee.show_dev_tools?(host, &windowInfo, nil, &settings, nil)

        host.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1) { base in
            _ = base.pointee.release?(base)
        }
    }

    /// Close the developer tools window.
    public func closeDevTools() {
        guard let host = host else { return }
        host.pointee.close_dev_tools?(host)

        host.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1) { base in
            _ = base.pointee.release?(base)
        }
    }

    /// Whether DevTools is currently open.
    public var hasDevTools: Bool {
        guard let host = host,
              let hasDevTools = host.pointee.has_dev_tools else {
            return false
        }
        let result = hasDevTools(host) != 0

        host.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1) { base in
            _ = base.pointee.release?(base)
        }

        return result
    }
}
