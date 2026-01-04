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

    case "bookmark":
      return handleBookmark(request)

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

  // MARK: - Bookmark Handlers

  private func handleBookmark(_ request: TermsurfRequest) -> TermsurfResponse {
    guard let subaction = request.subaction else {
      return .error(id: request.id, message: "Missing subaction for bookmark")
    }

    switch subaction {
    case "add":
      return handleBookmarkAdd(request)
    case "get":
      return handleBookmarkGet(request)
    case "list":
      return handleBookmarkList(request)
    case "update":
      return handleBookmarkUpdate(request)
    case "delete":
      return handleBookmarkDelete(request)
    default:
      return .error(id: request.id, message: "Unknown bookmark subaction: \(subaction)")
    }
  }

  private func handleBookmarkAdd(_ request: TermsurfRequest) -> TermsurfResponse {
    let profile = request.getString("profile") ?? "default"

    guard let name = request.getString("name") else {
      return .error(id: request.id, message: "Missing 'name' in data")
    }
    guard let url = request.getString("url") else {
      return .error(id: request.id, message: "Missing 'url' in data")
    }

    let title = request.getString("title") ?? name

    do {
      try ProfileManager.shared.addBookmark(profile: profile, name: name, title: title, url: url)
      return .ok(id: request.id)
    } catch let error as BookmarkError {
      return .error(id: request.id, message: error.localizedDescription)
    } catch {
      return .error(id: request.id, message: "Failed to add bookmark: \(error.localizedDescription)")
    }
  }

  private func handleBookmarkGet(_ request: TermsurfRequest) -> TermsurfResponse {
    let profile = request.getString("profile") ?? "default"

    guard let name = request.getString("name") else {
      return .error(id: request.id, message: "Missing 'name' in data")
    }

    if let bookmark = ProfileManager.shared.getBookmark(profile: profile, name: name) {
      return .ok(
        id: request.id,
        data: [
          "title": .string(bookmark.title),
          "url": .string(bookmark.url)
        ])
    } else {
      return .error(id: request.id, message: "Bookmark '\(name)' not found")
    }
  }

  private func handleBookmarkList(_ request: TermsurfRequest) -> TermsurfResponse {
    let profile = request.getString("profile") ?? "default"

    let bookmarks = ProfileManager.shared.listBookmarks(profile: profile)

    // Convert bookmarks to nested dictionary format
    var bookmarksDict: [String: AnyCodableValue] = [:]
    for (name, bookmark) in bookmarks {
      bookmarksDict[name] = .dictionary([
        "title": .string(bookmark.title),
        "url": .string(bookmark.url)
      ])
    }

    return .ok(
      id: request.id,
      data: [
        "bookmarks": .dictionary(bookmarksDict)
      ])
  }

  private func handleBookmarkUpdate(_ request: TermsurfRequest) -> TermsurfResponse {
    let profile = request.getString("profile") ?? "default"

    guard let name = request.getString("name") else {
      return .error(id: request.id, message: "Missing 'name' in data")
    }

    let title = request.getString("title")
    let url = request.getString("url")

    if title == nil && url == nil {
      return .error(id: request.id, message: "Must provide 'title' or 'url' to update")
    }

    do {
      try ProfileManager.shared.updateBookmark(profile: profile, name: name, title: title, url: url)
      return .ok(id: request.id)
    } catch let error as BookmarkError {
      return .error(id: request.id, message: error.localizedDescription)
    } catch {
      return .error(
        id: request.id, message: "Failed to update bookmark: \(error.localizedDescription)")
    }
  }

  private func handleBookmarkDelete(_ request: TermsurfRequest) -> TermsurfResponse {
    let profile = request.getString("profile") ?? "default"

    guard let name = request.getString("name") else {
      return .error(id: request.id, message: "Missing 'name' in data")
    }

    do {
      try ProfileManager.shared.deleteBookmark(profile: profile, name: name)
      return .ok(id: request.id)
    } catch let error as BookmarkError {
      return .error(id: request.id, message: error.localizedDescription)
    } catch {
      return .error(
        id: request.id, message: "Failed to delete bookmark: \(error.localizedDescription)")
    }
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
