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
    static func injectEnvVars(into config: inout Ghostty.SurfaceConfiguration) {
        guard let socketPath = SocketServer.shared.socketPath else {
            logger.warning("Socket server not running, cannot inject env vars")
            return
        }

        let paneId = generatePaneId()

        config.environmentVariables["TERMSURF_SOCKET"] = socketPath
        config.environmentVariables["TERMSURF_PANE_ID"] = paneId

        logger.info("Injected env vars: TERMSURF_SOCKET=\(socketPath) TERMSURF_PANE_ID=\(paneId)")
    }
}
