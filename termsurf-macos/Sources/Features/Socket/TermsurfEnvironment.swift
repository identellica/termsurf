import Foundation
import os

private let logger = Logger(subsystem: "com.termsurf", category: "TermsurfEnvironment")

/// Helper for injecting TermSurf environment variables into terminal surfaces.
enum TermsurfEnvironment {
    /// Generate a unique pane ID
    static func generatePaneId() -> String {
        "pane-\(UUID().uuidString.prefix(8))"
    }

    /// Inject TermSurf environment variables into a SurfaceConfiguration.
    /// Call this before creating a SurfaceView with the config.
    /// Returns the generated pane ID (needed for registering the surface later).
    @discardableResult
    static func injectEnvVars(into config: inout Ghostty.SurfaceConfiguration) -> String? {
        guard let socketPath = SocketServer.shared.socketPath else {
            logger.warning("Socket server not running, cannot inject env vars")
            return nil
        }

        let paneId = generatePaneId()

        config.environmentVariables["TERMSURF_SOCKET"] = socketPath
        config.environmentVariables["TERMSURF_PANE_ID"] = paneId

        logger.info("Injected env vars: TERMSURF_SOCKET=\(socketPath) TERMSURF_PANE_ID=\(paneId)")
        return paneId
    }

    /// Register a surface view with the WebViewManager.
    /// Call this after creating a SurfaceView to enable webview overlays on it.
    static func registerSurface(_ surface: Ghostty.SurfaceView, paneId: String) {
        WebViewManager.shared.registerPane(id: paneId, surface: surface)
        logger.info("Registered surface with pane ID: \(paneId)")
    }
}
