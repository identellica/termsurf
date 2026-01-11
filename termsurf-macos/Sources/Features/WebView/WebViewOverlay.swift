import Cocoa
import WebKit
import os

private let logger = Logger(subsystem: "com.termsurf", category: "WebViewOverlay")

/// A view that displays a WKWebView overlay on top of a terminal pane.
/// Includes console capture that routes console.log/error to the underlying terminal.
class WebViewOverlay: NSView, WKScriptMessageHandler, WKNavigationDelegate, WKUIDelegate {
  /// The webview ID
  let webviewId: String

  /// The WKWebView instance
  private(set) var webView: WKWebView!

  /// Callback when webview should close (called from container, not JS anymore)
  var onClose: ((String) -> Void)?

  /// Callback when JS API exit() is called (with exit code)
  var onExit: ((Int) -> Void)?

  /// Callback for console output (defaults to stdout/stderr)
  var onConsoleOutput: ((ConsoleLevel, String) -> Void)?

  /// Callback when URL changes (navigation started or finished)
  var onURLChanged: ((URL?) -> Void)?

  /// Callback when title changes
  var onTitleChanged: ((String?) -> Void)?

  enum ConsoleLevel: String {
    case log, info, warn, error
  }

  /// KVO observation for URL changes
  private var urlObservation: NSKeyValueObservation?

  /// KVO observation for title changes
  private var titleObservation: NSKeyValueObservation?

  /// Whether the JS API is enabled
  private let jsApiEnabled: Bool

  /// The profile name (for bookmarking)
  let profileName: String?

  /// Whether to suppress JavaScript dialogs (user opted to prevent additional dialogs)
  private var suppressDialogs = false

  // MARK: - Initialization

  init(
    url: URL, webviewId: String, profile: String? = nil, incognito: Bool = false,
    jsApi: Bool = false
  ) {
    self.webviewId = webviewId
    self.jsApiEnabled = jsApi
    self.profileName = profile
    super.init(frame: .zero)

    setupWebView(profile: profile, incognito: incognito)
    loadURL(url)
  }

  required init?(coder: NSCoder) {
    fatalError("init(coder:) has not been implemented")
  }

  deinit {
    urlObservation?.invalidate()
    titleObservation?.invalidate()
    logger.info("WebViewOverlay \(self.webviewId) deallocated")
  }

  // MARK: - Setup

  private func setupWebView(profile: String?, incognito: Bool) {
    let config = WKWebViewConfiguration()
    let contentController = WKUserContentController()

    // Console capture script
    let consoleScript = """
      (function() {
          const originalLog = console.log;
          const originalError = console.error;
          const originalWarn = console.warn;
          const originalInfo = console.info;

          function formatArg(arg) {
              try {
                  return typeof arg === 'object' ? JSON.stringify(arg) : String(arg);
              } catch {
                  return String(arg);
              }
          }

          console.log = function(...args) {
              window.webkit.messageHandlers.consoleLog.postMessage({
                  level: 'log',
                  message: args.map(formatArg).join(' ')
              });
              originalLog.apply(console, args);
          };

          console.error = function(...args) {
              window.webkit.messageHandlers.consoleLog.postMessage({
                  level: 'error',
                  message: args.map(formatArg).join(' ')
              });
              originalError.apply(console, args);
          };

          console.warn = function(...args) {
              window.webkit.messageHandlers.consoleLog.postMessage({
                  level: 'warn',
                  message: args.map(formatArg).join(' ')
              });
              originalWarn.apply(console, args);
          };

          console.info = function(...args) {
              window.webkit.messageHandlers.consoleLog.postMessage({
                  level: 'info',
                  message: args.map(formatArg).join(' ')
              });
              originalInfo.apply(console, args);
          };

          // Capture uncaught errors
          window.onerror = function(message, source, lineno, colno, error) {
              window.webkit.messageHandlers.consoleLog.postMessage({
                  level: 'error',
                  message: 'Uncaught: ' + message + ' at ' + source + ':' + lineno + ':' + colno
              });
          };

          window.onunhandledrejection = function(event) {
              window.webkit.messageHandlers.consoleLog.postMessage({
                  level: 'error',
                  message: 'Unhandled Promise Rejection: ' + event.reason
              });
          };
      })();
      """

    // Optional JS API (window.termsurf) - only injected when --js-api flag is used
    let jsApiScript = """
      window.termsurf = {
          webviewId: '\(webviewId)',
          exit: function(code) {
              var exitCode = typeof code === 'number' ? Math.floor(code) : 0;
              exitCode = Math.max(0, Math.min(255, exitCode));
              window.webkit.messageHandlers.termsurf.postMessage({
                  action: 'exit',
                  code: exitCode
              });
          }
      };
      """

    let consoleUserScript = WKUserScript(
      source: consoleScript,
      injectionTime: .atDocumentStart,
      forMainFrameOnly: false
    )

    contentController.addUserScript(consoleUserScript)

    // Only inject JS API if enabled via --js-api flag
    if jsApiEnabled {
      let jsApiUserScript = WKUserScript(
        source: jsApiScript,
        injectionTime: .atDocumentStart,
        forMainFrameOnly: true
      )
      contentController.addUserScript(jsApiUserScript)
      logger.info("JS API enabled for webview \(self.webviewId)")
    }

    // Register message handlers
    contentController.add(self, name: "consoleLog")
    contentController.add(self, name: "termsurf")

    config.userContentController = contentController

    // Configure data store for session isolation
    // Incognito takes precedence over profile
    if incognito {
      config.websiteDataStore = .nonPersistent()
      logger.info("Using incognito mode (non-persistent data store)")
    } else if let profile = profile {
      if #available(macOS 14.0, *) {
        // Get UUID from ProfileManager (ensures profile JSON file exists)
        let profileUUID = ProfileManager.shared.uuidForProfile(name: profile)
        config.websiteDataStore = WKWebsiteDataStore(forIdentifier: profileUUID)

        // Ensure the profile JSON file exists (for reverse UUID -> name lookup)
        ProfileManager.shared.ensureProfileExists(name: profile)

        logger.info("Using profile '\(profile)' with data store UUID: \(profileUUID)")
      } else {
        logger.warning("Profile isolation requires macOS 14+, using default data store")
      }
    }

    // Enable developer extras (Safari Web Inspector)
    config.preferences.setValue(true, forKey: "developerExtrasEnabled")

    // Create webview
    webView = WKWebView(frame: bounds, configuration: config)
    webView.navigationDelegate = self
    webView.uiDelegate = self
    webView.autoresizingMask = [.width, .height]

    // Set Safari User-Agent to avoid mobile/simplified layouts
    webView.customUserAgent =
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.2 Safari/605.1.15"

    // Make background semi-transparent while loading
    webView.setValue(false, forKey: "drawsBackground")

    // Observe URL changes (catches all navigation including SPA pushState)
    urlObservation = webView.observe(\.url, options: [.new]) { [weak self] webView, _ in
      self?.onURLChanged?(webView.url)
    }

    // Observe title changes
    titleObservation = webView.observe(\.title, options: [.new]) { [weak self] webView, _ in
      self?.onTitleChanged?(webView.title)
    }

    addSubview(webView)
    logger.info("WebViewOverlay \(self.webviewId) created")
  }

  private func loadURL(_ url: URL) {
    logger.info("Loading URL: \(url.absoluteString)")
    webView.load(URLRequest(url: url))
  }

  /// Navigate to a new URL (public method for external navigation requests)
  func navigate(to url: URL) {
    logger.info("Navigating to: \(url.absoluteString)")
    webView.load(URLRequest(url: url))
  }

  // MARK: - WKScriptMessageHandler

  func userContentController(
    _ userContentController: WKUserContentController, didReceive message: WKScriptMessage
  ) {
    if message.name == "consoleLog" {
      handleConsoleMessage(message)
    } else if message.name == "termsurf" {
      handleTermsurfMessage(message)
    }
  }

  private func handleConsoleMessage(_ message: WKScriptMessage) {
    guard let body = message.body as? [String: Any],
      let levelStr = body["level"] as? String,
      let msg = body["message"] as? String
    else {
      return
    }

    let level = ConsoleLevel(rawValue: levelStr) ?? .log

    if let handler = onConsoleOutput {
      handler(level, msg)
    } else {
      // Default: write to stdout/stderr
      let output = "[\(level.rawValue)] \(msg)\n"
      if level == .error {
        fputs(output, stderr)
        fflush(stderr)
      } else {
        fputs(output, stdout)
        fflush(stdout)
      }
    }
  }

  private func handleTermsurfMessage(_ message: WKScriptMessage) {
    logger.info("handleTermsurfMessage called")
    logger.info("  - message.body: \(String(describing: message.body))")

    guard let body = message.body as? [String: Any],
      let action = body["action"] as? String
    else {
      logger.warning("  - Failed to parse message body")
      return
    }

    logger.info("  - action: \(action)")

    switch action {
    case "exit":
      guard jsApiEnabled else {
        logger.warning("Exit action received but JS API is not enabled")
        return
      }
      let exitCode = (body["code"] as? Int) ?? 0
      logger.info("Webview \(self.webviewId) requested exit with code \(exitCode)")
      onExit?(exitCode)

    default:
      logger.warning("Unknown termsurf action: \(action)")
    }
  }

  // MARK: - WKNavigationDelegate

  /// Intercept navigation requests to add the Upgrade-Insecure-Requests header.
  /// This header is sent by Safari but not by WKWebView by default. Some sites
  /// (e.g., Google) use its absence to detect embedded webviews and serve
  /// simplified/mobile layouts.
  func webView(
    _ webView: WKWebView,
    decidePolicyFor navigationAction: WKNavigationAction,
    decisionHandler: @escaping (WKNavigationActionPolicy) -> Void
  ) {
    // Only modify HTTP/HTTPS requests
    guard let url = navigationAction.request.url,
      (url.scheme == "http" || url.scheme == "https")
    else {
      decisionHandler(.allow)
      return
    }

    // If header is already present, allow the request
    if navigationAction.request.value(forHTTPHeaderField: "Upgrade-Insecure-Requests") != nil {
      decisionHandler(.allow)
      return
    }

    // Cancel this request and reload with the header added
    decisionHandler(.cancel)

    var modifiedRequest = navigationAction.request
    modifiedRequest.setValue("1", forHTTPHeaderField: "Upgrade-Insecure-Requests")
    webView.load(modifiedRequest)
  }

  func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
    logger.debug(
      "Navigation started for \(self.webviewId): \(webView.url?.absoluteString ?? "unknown")")
    onURLChanged?(webView.url)
    // Reset dialog suppression on new navigation
    suppressDialogs = false
  }

  /// Callback when navigation finishes (for re-establishing focus)
  var onNavigationFinished: (() -> Void)?

  func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
    logger.info(
      "Navigation finished for \(self.webviewId): \(webView.url?.absoluteString ?? "unknown")")

    // Make background opaque once loaded
    webView.setValue(true, forKey: "drawsBackground")

    // Notify URL change (may differ from provisional due to redirects)
    onURLChanged?(webView.url)

    // Notify container that navigation finished (to re-establish focus)
    onNavigationFinished?()
  }

  // MARK: - Web Content Focus

  /// Blur the web content (unfocus any focused element)
  func blurWebContent() {
    webView.evaluateJavaScript("document.activeElement?.blur(); window.focus();") { _, error in
      if let error = error {
        logger.debug("Blur JS error (ignorable): \(error.localizedDescription)")
      }
    }
  }

  /// Focus the web content (focus the document body)
  func focusWebContent() {
    // First make sure the webview window is key, then focus the document
    webView.evaluateJavaScript("document.body.focus(); window.focus();") { _, error in
      if let error = error {
        logger.debug("Focus JS error (ignorable): \(error.localizedDescription)")
      }
    }
  }

  func webView(_ webView: WKWebView, didFail navigation: WKNavigation!, withError error: Error) {
    logger.error("Navigation failed for \(self.webviewId): \(error.localizedDescription)")
  }

  func webView(
    _ webView: WKWebView, didFailProvisionalNavigation navigation: WKNavigation!,
    withError error: Error
  ) {
    logger.error(
      "Provisional navigation failed for \(self.webviewId): \(error.localizedDescription)")
  }

  // MARK: - WKUIDelegate

  func webView(
    _ webView: WKWebView,
    createWebViewWith configuration: WKWebViewConfiguration,
    for navigationAction: WKNavigationAction,
    windowFeatures: WKWindowFeatures
  ) -> WKWebView? {
    // Handle target="_blank" links by loading in the same webview.
    // This is a simple solution that works for most cases.
    // Future: open in new tab via `web open <url>` command.
    if navigationAction.targetFrame == nil {
      webView.load(navigationAction.request)
    }
    return nil
  }

  // MARK: - JavaScript Dialogs

  /// Helper to get origin string for dialog titles
  private func originString(from frame: WKFrameInfo) -> String {
    let host = frame.securityOrigin.host
    return host.isEmpty ? "This page" : host
  }

  /// Helper to create and configure suppression checkbox
  private func createSuppressionCheckbox() -> NSButton {
    let checkbox = NSButton(checkboxWithTitle: "Prevent this page from creating additional dialogs", target: nil, action: nil)
    checkbox.frame = NSRect(x: 0, y: 0, width: 400, height: 18)
    checkbox.state = .off
    return checkbox
  }

  func webView(
    _ webView: WKWebView,
    runJavaScriptAlertPanelWithMessage message: String,
    initiatedByFrame frame: WKFrameInfo,
    completionHandler: @escaping () -> Void
  ) {
    // If dialogs are suppressed, auto-dismiss
    if suppressDialogs {
      completionHandler()
      return
    }

    let alert = NSAlert()
    alert.messageText = "\(originString(from: frame)) says:"
    alert.informativeText = message
    alert.addButton(withTitle: "OK")

    let checkbox = createSuppressionCheckbox()
    alert.accessoryView = checkbox

    alert.runModal()

    if checkbox.state == .on {
      suppressDialogs = true
    }

    completionHandler()
  }

  func webView(
    _ webView: WKWebView,
    runJavaScriptConfirmPanelWithMessage message: String,
    initiatedByFrame frame: WKFrameInfo,
    completionHandler: @escaping (Bool) -> Void
  ) {
    // If dialogs are suppressed, auto-dismiss with false
    if suppressDialogs {
      completionHandler(false)
      return
    }

    let alert = NSAlert()
    alert.messageText = "\(originString(from: frame)) says:"
    alert.informativeText = message
    alert.addButton(withTitle: "OK")
    alert.addButton(withTitle: "Cancel")

    let checkbox = createSuppressionCheckbox()
    alert.accessoryView = checkbox

    let response = alert.runModal()

    if checkbox.state == .on {
      suppressDialogs = true
    }

    completionHandler(response == .alertFirstButtonReturn)
  }

  func webView(
    _ webView: WKWebView,
    runJavaScriptTextInputPanelWithPrompt prompt: String,
    defaultText: String?,
    initiatedByFrame frame: WKFrameInfo,
    completionHandler: @escaping (String?) -> Void
  ) {
    // If dialogs are suppressed, auto-dismiss with nil
    if suppressDialogs {
      completionHandler(nil)
      return
    }

    let alert = NSAlert()
    alert.messageText = "\(originString(from: frame)) says:"
    alert.informativeText = prompt
    alert.addButton(withTitle: "OK")
    alert.addButton(withTitle: "Cancel")

    // Create container view with explicit frame-based layout
    let containerView = NSView(frame: NSRect(x: 0, y: 0, width: 400, height: 50))

    let textField = NSTextField(frame: NSRect(x: 0, y: 26, width: 400, height: 24))
    textField.stringValue = defaultText ?? ""

    let checkbox = NSButton(checkboxWithTitle: "Prevent this page from creating additional dialogs", target: nil, action: nil)
    checkbox.frame = NSRect(x: 0, y: 0, width: 400, height: 18)
    checkbox.state = .off

    containerView.addSubview(textField)
    containerView.addSubview(checkbox)

    alert.accessoryView = containerView

    // Make the text field first responder
    alert.window.initialFirstResponder = textField

    let response = alert.runModal()

    if checkbox.state == .on {
      suppressDialogs = true
    }

    completionHandler(response == .alertFirstButtonReturn ? textField.stringValue : nil)
  }

  // MARK: - File Upload

  func webView(
    _ webView: WKWebView,
    runOpenPanelWith parameters: WKOpenPanelParameters,
    initiatedByFrame frame: WKFrameInfo,
    completionHandler: @escaping ([URL]?) -> Void
  ) {
    let panel = NSOpenPanel()
    panel.allowsMultipleSelection = parameters.allowsMultipleSelection
    panel.canChooseDirectories = parameters.allowsDirectories
    panel.begin { response in
      completionHandler(response == .OK ? panel.urls : nil)
    }
  }

  // MARK: - Focus Handling

  override var acceptsFirstResponder: Bool { true }

  override func becomeFirstResponder() -> Bool {
    webView.becomeFirstResponder()
  }
}
