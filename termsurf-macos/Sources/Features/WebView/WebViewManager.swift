import Cocoa
import WebKit
import os

private let logger = Logger(subsystem: "com.termsurf", category: "WebViewManager")

/// Manages webview containers and their association with terminal panes.
class WebViewManager {
    static let shared = WebViewManager()

    /// Mapping of pane ID to surface view (weak references)
    private var paneRegistry = [String: WeakSurfaceRef]()

    /// Mapping of webview ID to container
    private var containers = [String: WebViewContainer]()

    /// Mapping of webview ID to pane ID (for focus restoration on close)
    private var webviewToPaneId = [String: String]()

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

    /// Create a webview container on a pane.
    /// Returns the webview ID immediately. The actual container creation happens asynchronously.
    /// Returns nil if the pane doesn't exist.
    func createWebView(
        url: URL,
        paneId: String,
        profile: String? = nil
    ) -> String? {
        // Check if pane exists first
        lock.lock()
        let paneExists = paneRegistry[paneId]?.surface != nil
        lock.unlock()

        guard paneExists else {
            logger.error("Cannot create webview: pane \(paneId) not found")
            return nil
        }

        // Generate unique webview ID synchronously
        let webviewId = "wv-\(UUID().uuidString.prefix(8))"

        // Dispatch the actual UI work asynchronously - don't block!
        // This allows the response to be sent immediately while the container
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

            // Create the container on main thread
            let container = WebViewContainer(url: url, webviewId: webviewId, profile: profile)
            container.onClose = { [weak self] id in
                self?.closeWebView(id: id)
            }

            // Add container to surface
            container.frame = currentSurface.bounds
            container.autoresizingMask = [.width, .height]
            currentSurface.addSubview(container)

            // Focus footer by default (terminal mode)
            container.focusFooter()

            self.lock.lock()
            self.containers[webviewId] = container
            self.webviewToPaneId[webviewId] = paneId
            self.lock.unlock()

            logger.info("Created webview \(webviewId) on pane \(paneId) with URL: \(url.absoluteString)")
        }

        // Return ID immediately - the container will be created asynchronously
        logger.info("Queued webview \(webviewId) for creation on pane \(paneId)")
        return webviewId
    }

    /// Close a webview by ID.
    func closeWebView(id: String) {
        lock.lock()
        let container = containers.removeValue(forKey: id)
        let paneId = webviewToPaneId.removeValue(forKey: id)
        let surface = paneId.flatMap { paneRegistry[$0]?.surface }
        lock.unlock()

        guard let container = container else {
            logger.warning("Cannot close webview: \(id) not found")
            return
        }

        DispatchQueue.main.async {
            container.removeFromSuperview()

            // Restore focus to the terminal surface
            if let surface = surface {
                surface.window?.makeFirstResponder(surface)
                logger.info("Restored focus to terminal for pane \(paneId ?? "unknown")")
            }
        }

        logger.info("Closed webview: \(id)")
    }

    /// Look up a webview container by ID.
    func lookupContainer(id: String) -> WebViewContainer? {
        lock.lock()
        defer { lock.unlock() }
        return containers[id]
    }

    /// Close all webviews associated with a pane.
    func closeWebViewsForPane(paneId: String) {
        lock.lock()
        // Find webviews associated with this pane
        let webviewIds = webviewToPaneId.filter { $0.value == paneId }.map { $0.key }
        lock.unlock()

        for id in webviewIds {
            closeWebView(id: id)
        }
    }

    /// Hide a webview (for ctrl+z backgrounding).
    func hideWebView(id: String) {
        lock.lock()
        let container = containers[id]
        lock.unlock()

        DispatchQueue.main.async {
            container?.isHidden = true
        }
    }

    /// Show a webview (for fg foregrounding).
    func showWebView(id: String) {
        lock.lock()
        let container = containers[id]
        lock.unlock()

        DispatchQueue.main.async {
            container?.isHidden = false
            container?.focusFooter()
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
