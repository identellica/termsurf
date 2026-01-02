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
        guard let urlString = request.getString("url") else {
            return .error(id: request.id, message: "Missing 'url' in data")
        }

        guard let paneId = request.paneId else {
            return .error(id: request.id, message: "Missing paneId")
        }

        guard let url = URL(string: urlString) else {
            return .error(id: request.id, message: "Invalid URL: \(urlString)")
        }

        let profile = request.getString("profile")

        logger.info("Open webview: url=\(urlString) paneId=\(paneId) profile=\(profile ?? "default")")

        // Create webview - WebViewManager handles main thread dispatch
        let webviewId = WebViewManager.shared.createWebViewSync(
            url: url,
            paneId: paneId,
            profile: profile
        )

        if let webviewId = webviewId {
            return .ok(id: request.id, data: [
                "webviewId": .string(webviewId)
            ])
        } else {
            return .error(id: request.id, message: "Failed to create webview - pane not found: \(paneId)")
        }
    }

    private func handleClose(_ request: TermsurfRequest) -> TermsurfResponse {
        let webviewId = request.getString("webviewId")

        logger.info("Close webview: webviewId=\(webviewId ?? "nil")")

        if let webviewId = webviewId {
            // Close specific webview
            DispatchQueue.main.sync {
                WebViewManager.shared.closeWebView(id: webviewId)
            }
            return .ok(id: request.id, data: [
                "closed": .string(webviewId)
            ])
        } else {
            // TODO: Close all webviews for this pane
            return .ok(id: request.id, data: [
                "message": .string("Closing all webviews for pane not yet implemented")
            ])
        }
    }

    private func handleShow(_ request: TermsurfRequest) -> TermsurfResponse {
        guard let webviewId = request.getString("webviewId") else {
            return .error(id: request.id, message: "Missing 'webviewId' in data")
        }

        logger.info("Show webview: webviewId=\(webviewId)")

        DispatchQueue.main.sync {
            WebViewManager.shared.showWebView(id: webviewId)
        }

        return .ok(id: request.id, data: [
            "shown": .string(webviewId)
        ])
    }

    private func handleHide(_ request: TermsurfRequest) -> TermsurfResponse {
        guard let webviewId = request.getString("webviewId") else {
            return .error(id: request.id, message: "Missing 'webviewId' in data")
        }

        logger.info("Hide webview: webviewId=\(webviewId)")

        DispatchQueue.main.sync {
            WebViewManager.shared.hideWebView(id: webviewId)
        }

        return .ok(id: request.id, data: [
            "hidden": .string(webviewId)
        ])
    }
}
