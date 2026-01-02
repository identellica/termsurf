import Cocoa
import WebKit
import os

private let logger = Logger(subsystem: "com.termsurf", category: "WebViewManager")

/// Manages webview overlays and their association with terminal panes.
class WebViewManager {
    static let shared = WebViewManager()

    /// Mapping of pane ID to surface view (weak references)
    private var paneRegistry = [String: WeakSurfaceRef]()

    /// Mapping of webview ID to overlay
    private var webviews = [String: WebViewOverlay]()

    /// Lock for thread-safe access
    private let lock = NSLock()

    private init() {}

    // MARK: - Pane Registration

    /// Register a surface view with its pane ID.
    /// Call this when injecting environment variables into a new surface.
    func registerPane(id: String, surface: Ghostty.SurfaceView) {
        lock.lock()
        defer { lock.unlock() }

        paneRegistry[id] = WeakSurfaceRef(surface)
        logger.info("Registered pane: \(id)")
    }

    /// Unregister a pane (called when surface is destroyed).
    func unregisterPane(id: String) {
        lock.lock()
        defer { lock.unlock() }

        paneRegistry.removeValue(forKey: id)
        logger.info("Unregistered pane: \(id)")
    }

    /// Look up a surface by pane ID.
    func lookupPane(id: String) -> Ghostty.SurfaceView? {
        lock.lock()
        defer { lock.unlock() }

        return paneRegistry[id]?.surface
    }

    // MARK: - Webview Management

    /// Create a webview overlay on a pane.
    /// Returns the webview ID immediately. The actual webview creation happens asynchronously.
    /// Returns nil if the pane doesn't exist.
    func createWebView(
        url: URL,
        paneId: String,
        profile: String? = nil
    ) -> String? {
        // Check if pane exists first
        lock.lock()
        let surface = paneRegistry[paneId]?.surface
        lock.unlock()

        guard let surface = surface else {
            logger.error("Cannot create webview: pane \(paneId) not found")
            return nil
        }

        // Generate unique webview ID synchronously
        let webviewId = "wv-\(UUID().uuidString.prefix(8))"

        // Dispatch the actual UI work asynchronously - don't block!
        // This allows the response to be sent immediately while the webview
        // is created in the next run loop iteration.
        DispatchQueue.main.async { [weak self] in
            guard let self = self else { return }

            // Re-check surface validity (it could have been destroyed)
            self.lock.lock()
            guard let currentSurface = self.paneRegistry[paneId]?.surface else {
                self.lock.unlock()
                logger.warning("Surface for pane \(paneId) no longer exists")
                return
            }
            self.lock.unlock()

            // Create the overlay on main thread
            let overlay = WebViewOverlay(url: url, webviewId: webviewId, profile: profile)
            overlay.onClose = { [weak self] id in
                self?.closeWebView(id: id)
            }

            // Add overlay to surface
            overlay.frame = currentSurface.bounds
            overlay.autoresizingMask = [.width, .height]
            currentSurface.addSubview(overlay)

            // Make webview the first responder
            overlay.window?.makeFirstResponder(overlay.webView)

            self.lock.lock()
            self.webviews[webviewId] = overlay
            self.lock.unlock()

            logger.info("Created webview \(webviewId) on pane \(paneId) with URL: \(url.absoluteString)")
        }

        // Return ID immediately - the webview will be created asynchronously
        logger.info("Queued webview \(webviewId) for creation on pane \(paneId)")
        return webviewId
    }

    /// Close a webview by ID.
    func closeWebView(id: String) {
        lock.lock()
        let overlay = webviews.removeValue(forKey: id)
        lock.unlock()

        guard let overlay = overlay else {
            logger.warning("Cannot close webview: \(id) not found")
            return
        }

        DispatchQueue.main.async {
            overlay.removeFromSuperview()
        }

        logger.info("Closed webview: \(id)")
    }

    /// Look up a webview by ID.
    func lookupWebView(id: String) -> WebViewOverlay? {
        lock.lock()
        defer { lock.unlock() }
        return webviews[id]
    }

    /// Close all webviews associated with a pane.
    func closeWebViewsForPane(paneId: String) {
        lock.lock()
        // Find webviews associated with this pane
        // For now, we don't track pane → webview associations
        // This will be added if needed
        lock.unlock()
    }

    /// Hide a webview (for ctrl+z backgrounding).
    func hideWebView(id: String) {
        lock.lock()
        let overlay = webviews[id]
        lock.unlock()

        DispatchQueue.main.async {
            overlay?.isHidden = true
        }
    }

    /// Show a webview (for fg foregrounding).
    func showWebView(id: String) {
        lock.lock()
        let overlay = webviews[id]
        lock.unlock()

        DispatchQueue.main.async {
            overlay?.isHidden = false
            overlay?.window?.makeFirstResponder(overlay?.webView)
        }
    }

    // MARK: - Cleanup

    /// Clean up any stale weak references.
    func cleanupStaleReferences() {
        lock.lock()
        defer { lock.unlock() }

        paneRegistry = paneRegistry.filter { $0.value.surface != nil }
    }
}

// MARK: - Helper Types

/// Weak reference wrapper for SurfaceView to avoid retain cycles.
private class WeakSurfaceRef {
    weak var surface: Ghostty.SurfaceView?

    init(_ surface: Ghostty.SurfaceView) {
        self.surface = surface
    }
}
