import Foundation
import os

private let logger = Logger(subsystem: "com.termsurf", category: "CommandHandler")

/// Handles incoming requests from CLI clients and routes them to appropriate handlers.
class CommandHandler {
  static let shared = CommandHandler()

  private init() {}

  /// Handle a request and return a response
  /// - Parameters:
  ///   - request: The incoming request
  ///   - connection: The socket connection (optional, used for streaming events)
  func handle(_ request: TermsurfRequest, connection: SocketConnection? = nil) -> TermsurfResponse {
    logger.info("Handling request: action=\(request.action) id=\(request.id)")

    switch request.action {
    case "ping":
      return handlePing(request)

    case "open":
      return handleOpen(request, connection: connection)

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

  /// Default homepage when no URL is provided
  private static let defaultHomepage = "https://hallucipedia.com"

  private func handleOpen(_ request: TermsurfRequest, connection: SocketConnection? = nil)
    -> TermsurfResponse
  {
    guard let paneId = request.paneId else {
      return .error(id: request.id, message: "Missing paneId")
    }

    // Get URL string, using default homepage if not provided
    let rawUrlString = request.getString("url")
    let normalizedUrlString = normalizeUrl(rawUrlString)

    guard let url = URL(string: normalizedUrlString) else {
      return .error(id: request.id, message: "Invalid URL: \(normalizedUrlString)")
    }

    let profile = request.getString("profile")
    let incognito = request.getBool("incognito") ?? false
    let jsApi = request.getBool("jsApi") ?? false

    logger.info(
      "Open webview: url=\(normalizedUrlString) paneId=\(paneId) profile=\(profile ?? "default") incognito=\(incognito) jsApi=\(jsApi)"
    )

    // Create webview - WebViewManager handles main thread dispatch
    // Pass connection for console event streaming
    let webviewId = WebViewManager.shared.createWebView(
      url: url,
      paneId: paneId,
      profile: profile,
      incognito: incognito,
      jsApi: jsApi,
      connection: connection,
      requestId: request.id
    )

    if let webviewId = webviewId {
      return .ok(
        id: request.id,
        data: [
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
      return .ok(
        id: request.id,
        data: [
          "closed": .string(webviewId)
        ])
    } else {
      // TODO: Close all webviews for this pane
      return .ok(
        id: request.id,
        data: [
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

    return .ok(
      id: request.id,
      data: [
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

    return .ok(
      id: request.id,
      data: [
        "hidden": .string(webviewId)
      ])
  }

  // MARK: - Helpers

  /// Normalize a URL string: use default if nil, prepend https:// if no scheme
  private func normalizeUrl(_ urlString: String?) -> String {
    guard let urlString = urlString, !urlString.isEmpty else {
      return Self.defaultHomepage
    }

    // If already has a scheme, return as-is
    if urlString.hasPrefix("http://") || urlString.hasPrefix("https://")
      || urlString.hasPrefix("file://")
    {
      return urlString
    }

    // Prepend https://
    return "https://\(urlString)"
  }
}
