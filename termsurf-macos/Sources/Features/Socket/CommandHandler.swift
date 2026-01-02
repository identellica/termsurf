import Foundation
import os

private let logger = Logger(subsystem: "com.termsurf", category: "CommandHandler")

/// Handles incoming requests from CLI clients and routes them to appropriate handlers.
class CommandHandler {
    static let shared = CommandHandler()

    private init() {}

    /// Handle a request and return a response
    func handle(_ request: TermsurfRequest) -> TermsurfResponse {
        logger.info("Handling request: action=\(request.action) id=\(request.id)")

        switch request.action {
        case "ping":
            return handlePing(request)

        case "open":
            return handleOpen(request)

        case "close":
            return handleClose(request)

        case "show":
            return handleShow(request)

        case "hide":
            return handleHide(request)

        default:
            logger.warning("Unknown action: \(request.action)")
            return .error(id: request.id, message: "Unknown action: \(request.action)")
        }
    }

    // MARK: - Handlers

    private func handlePing(_ request: TermsurfRequest) -> TermsurfResponse {
        logger.info("Ping received")
        return .ok(id: request.id, data: ["pong": .bool(true)])
    }

    private func handleOpen(_ request: TermsurfRequest) -> TermsurfResponse {
        guard let url = request.getString("url") else {
            return .error(id: request.id, message: "Missing 'url' in data")
        }

        guard let paneId = request.paneId else {
            return .error(id: request.id, message: "Missing paneId")
        }

        logger.info("Open webview: url=\(url) paneId=\(paneId)")

        // TODO: Phase 3C - Implement WebViewManager.open()
        // For now, just acknowledge the request
        let webviewId = "wv-\(UUID().uuidString.prefix(8))"

        return .ok(id: request.id, data: [
            "webviewId": .string(webviewId),
            "message": .string("WebView opening not yet implemented")
        ])
    }

    private func handleClose(_ request: TermsurfRequest) -> TermsurfResponse {
        let webviewId = request.getString("webviewId")

        logger.info("Close webview: webviewId=\(webviewId ?? "nil")")

        // TODO: Phase 3C - Implement WebViewManager.close()
        return .ok(id: request.id, data: [
            "message": .string("WebView closing not yet implemented")
        ])
    }

    private func handleShow(_ request: TermsurfRequest) -> TermsurfResponse {
        guard let webviewId = request.getString("webviewId") else {
            return .error(id: request.id, message: "Missing 'webviewId' in data")
        }

        logger.info("Show webview: webviewId=\(webviewId)")

        // TODO: Phase 3F - Implement WebViewManager.show()
        return .ok(id: request.id, data: [
            "message": .string("WebView show not yet implemented")
        ])
    }

    private func handleHide(_ request: TermsurfRequest) -> TermsurfResponse {
        guard let webviewId = request.getString("webviewId") else {
            return .error(id: request.id, message: "Missing 'webviewId' in data")
        }

        logger.info("Hide webview: webviewId=\(webviewId)")

        // TODO: Phase 3F - Implement WebViewManager.hide()
        return .ok(id: request.id, data: [
            "message": .string("WebView hide not yet implemented")
        ])
    }
}
