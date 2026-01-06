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

  /// Mapping of webview ID to socket connection (weak to avoid retain cycles)
  private var webviewConnections = [String: WeakConnectionRef]()

  /// Mapping of webview ID to request ID (for sending events)
  private var webviewRequestIds = [String: String]()

  /// Ordered list of webview IDs per pane (index 0 = bottom, last = top)
  /// Used for stacking multiple webviews on the same pane
  private var paneStacks = [String: [String]]()

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
  /// - Parameters:
  ///   - url: The URL to load
  ///   - paneId: The pane to attach the webview to
  ///   - profile: Optional browser profile for session isolation
  ///   - incognito: Whether to use ephemeral session (no data persisted)
  ///   - jsApi: Whether to enable the window.termsurf JavaScript API
  ///   - connection: Optional socket connection for sending console events
  ///   - requestId: Request ID for event correlation (required if connection is provided)
  func createWebView(
    url: URL,
    paneId: String,
    profile: String? = nil,
    incognito: Bool = false,
    jsApi: Bool = false,
    connection: SocketConnection? = nil,
    requestId: String? = nil
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

    // Store connection reference if provided (before async to ensure it's captured)
    if let connection = connection, let requestId = requestId {
      lock.lock()
      webviewConnections[webviewId] = WeakConnectionRef(connection)
      webviewRequestIds[webviewId] = requestId
      // Subscribe connection to events for this request
      connection.subscribeToEvents(requestId: requestId)
      lock.unlock()
    }

    // Add to pane stack and calculate position (before async to ensure order)
    lock.lock()
    if paneStacks[paneId] == nil {
      paneStacks[paneId] = []
    }
    paneStacks[paneId]!.append(webviewId)
    let stackPosition = paneStacks[paneId]!.count
    let stackTotal = stackPosition  // At creation time, this is the newest
    lock.unlock()

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
        // Remove from stack since we couldn't create it
        if var stack = self.paneStacks[paneId] {
          stack.removeAll { $0 == webviewId }
          self.paneStacks[paneId] = stack.isEmpty ? nil : stack
        }
        return
      }
      self.lock.unlock()

      // Create the container on main thread with stack info
      let container = WebViewContainer(
        url: url, webviewId: webviewId, profile: profile, incognito: incognito,
        jsApi: jsApi, stackPosition: stackPosition, stackTotal: stackTotal)
      container.onClose = { [weak self] id, exitCode in
        self?.closeWebView(id: id, exitCode: exitCode)
      }

      // Wire up console output to send events via socket
      container.onConsoleOutput = { [weak self] level, message in
        self?.sendConsoleEvent(webviewId: webviewId, level: level, message: message)
      }

      // Add container to surface
      container.frame = currentSurface.bounds
      container.autoresizingMask = [.width, .height]
      currentSurface.addSubview(container)

      // Start in browse mode so user can interact with the page immediately
      container.focusBrowser()

      self.lock.lock()
      self.containers[webviewId] = container
      self.webviewToPaneId[webviewId] = paneId

      // Update stack totals for all webviews on this pane
      if let stack = self.paneStacks[paneId] {
        let newTotal = stack.count
        for (index, existingId) in stack.enumerated() {
          if let existingContainer = self.containers[existingId] {
            existingContainer.updateStackInfo(position: index + 1, total: newTotal)
          }
        }
      }
      self.lock.unlock()

      logger.info(
        "Created webview \(webviewId) on pane \(paneId) with URL: \(url.absoluteString) [stack: \(stackPosition)/\(stackTotal)]"
      )
    }

    // Return ID immediately - the container will be created asynchronously
    logger.info("Queued webview \(webviewId) for creation on pane \(paneId)")
    return webviewId
  }

  /// Close a webview by ID.
  /// - Parameters:
  ///   - id: The webview ID to close
  ///   - exitCode: Exit code to send to CLI (default 0)
  func closeWebView(id: String, exitCode: Int = 0) {
    lock.lock()
    let container = containers.removeValue(forKey: id)
    let paneId = webviewToPaneId.removeValue(forKey: id)
    let surface = paneId.flatMap { paneRegistry[$0]?.surface }
    let connectionRef = webviewConnections.removeValue(forKey: id)
    let requestId = webviewRequestIds.removeValue(forKey: id)

    // Remove from pane stack and update remaining webviews
    var remainingContainers: [(WebViewContainer, Int, Int)] = []
    if let paneId = paneId, var stack = paneStacks[paneId] {
      stack.removeAll { $0 == id }
      if stack.isEmpty {
        paneStacks.removeValue(forKey: paneId)
      } else {
        paneStacks[paneId] = stack
        // Collect remaining containers to update (while holding lock)
        let newTotal = stack.count
        for (index, webviewId) in stack.enumerated() {
          if let existingContainer = containers[webviewId] {
            remainingContainers.append((existingContainer, index + 1, newTotal))
          }
        }
      }
    }
    lock.unlock()

    guard let container = container else {
      logger.warning("Cannot close webview: \(id) not found")
      return
    }

    // Send closed event to CLI if connection exists
    if let connection = connectionRef?.connection, let requestId = requestId {
      let event = TermsurfEvent(
        id: requestId, event: "closed",
        data: [
          "webviewId": .string(id),
          "exitCode": .int(exitCode)
        ])
      connection.sendEvent(event)
      logger.info("Sent closed event to CLI for webview \(id) with exit code \(exitCode)")
    }

    DispatchQueue.main.async {
      container.removeFromSuperview()

      // Update stack info for remaining webviews on this pane
      for (existingContainer, position, total) in remainingContainers {
        existingContainer.updateStackInfo(position: position, total: total)
      }

      // Restore focus to the terminal surface (only if no webviews remain on this pane)
      if remainingContainers.isEmpty, let surface = surface {
        surface.window?.makeFirstResponder(surface)
        logger.info("Restored focus to terminal for pane \(paneId ?? "unknown")")
      }
    }

    logger.info("Closed webview: \(id)")
  }

  /// Send a console event to the CLI for a webview
  private func sendConsoleEvent(
    webviewId: String, level: WebViewOverlay.ConsoleLevel, message: String
  ) {
    lock.lock()
    let connectionRef = webviewConnections[webviewId]
    let requestId = webviewRequestIds[webviewId]
    lock.unlock()

    guard let connection = connectionRef?.connection, let requestId = requestId else {
      // No connection, fall back to stdout/stderr (handled by WebViewOverlay default)
      return
    }

    let event = TermsurfEvent(
      id: requestId, event: "console",
      data: [
        "level": .string(level.rawValue),
        "message": .string(message)
      ])
    connection.sendEvent(event)
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
      container?.focusControlBar()
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

/// Weak reference wrapper for SocketConnection to avoid retain cycles.
private class WeakConnectionRef {
  weak var connection: SocketConnection?

  init(_ connection: SocketConnection) {
    self.connection = connection
  }
}
