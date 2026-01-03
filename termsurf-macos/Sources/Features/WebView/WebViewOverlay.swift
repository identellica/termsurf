import Cocoa
import WebKit
import os

private let logger = Logger(subsystem: "com.termsurf", category: "WebViewOverlay")

/// A view that displays a WKWebView overlay on top of a terminal pane.
/// Includes console capture that routes console.log/error to the underlying terminal.
class WebViewOverlay: NSView, WKScriptMessageHandler, WKNavigationDelegate {
    /// The webview ID
    let webviewId: String

    /// The WKWebView instance
    private(set) var webView: WKWebView!

    /// Callback when webview should close (called from container, not JS anymore)
    var onClose: ((String) -> Void)?

    /// Callback when Esc is pressed (to switch to control mode)
    var onEscapePressed: (() -> Void)?

    /// Callback for console output (defaults to stdout/stderr)
    var onConsoleOutput: ((ConsoleLevel, String) -> Void)?

    /// Callback when URL changes (navigation started or finished)
    var onURLChanged: ((URL?) -> Void)?

    enum ConsoleLevel: String {
        case log, info, warn, error
    }

    // MARK: - Initialization

    init(url: URL, webviewId: String, profile: String? = nil) {
        self.webviewId = webviewId
        super.init(frame: .zero)

        setupWebView(profile: profile)
        loadURL(url)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    deinit {
        logger.info("WebViewOverlay \(self.webviewId) deallocated")
    }

    // MARK: - Setup

    private func setupWebView(profile: String?) {
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

        // Keyboard interception for Esc only (to switch back to control mode)
        // All other keys go to the browser - ctrl+c, ctrl+z, etc. are handled by
        // SurfaceView when in control mode
        let keyboardScript = """
        document.addEventListener('keydown', function(e) {
            if (e.key === 'Escape') {
                e.preventDefault();
                window.webkit.messageHandlers.termsurf.postMessage({action: 'escape'});
            }
        }, true);
        """

        let consoleUserScript = WKUserScript(
            source: consoleScript,
            injectionTime: .atDocumentStart,
            forMainFrameOnly: false
        )
        let keyboardUserScript = WKUserScript(
            source: keyboardScript,
            injectionTime: .atDocumentStart,
            forMainFrameOnly: true
        )

        contentController.addUserScript(consoleUserScript)
        contentController.addUserScript(keyboardUserScript)

        // Register message handlers
        contentController.add(self, name: "consoleLog")
        contentController.add(self, name: "termsurf")

        config.userContentController = contentController

        // Configure data store for profile isolation (macOS 14+)
        if let profile = profile {
            if #available(macOS 14.0, *) {
                // Create deterministic UUID from profile name using a hash
                let hash = profile.hash
                let hashStr = String(format: "%08x%08x%04x%04x%012x",
                                     UInt32(truncatingIfNeeded: hash),
                                     UInt32(truncatingIfNeeded: hash >> 32),
                                     UInt16(truncatingIfNeeded: hash >> 48),
                                     UInt16(truncatingIfNeeded: hash >> 56),
                                     UInt64(truncatingIfNeeded: hash))
                let uuidStr = "\(hashStr.prefix(8))-\(hashStr.dropFirst(8).prefix(4))-\(hashStr.dropFirst(12).prefix(4))-\(hashStr.dropFirst(16).prefix(4))-\(hashStr.dropFirst(20).prefix(12))"
                let profileUUID = UUID(uuidString: uuidStr) ?? UUID()
                config.websiteDataStore = WKWebsiteDataStore(forIdentifier: profileUUID)
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
        webView.autoresizingMask = [.width, .height]

        // Make background semi-transparent while loading
        webView.setValue(false, forKey: "drawsBackground")

        addSubview(webView)
        logger.info("WebViewOverlay \(self.webviewId) created")
    }

    private func loadURL(_ url: URL) {
        logger.info("Loading URL: \(url.absoluteString)")
        webView.load(URLRequest(url: url))
    }

    // MARK: - WKScriptMessageHandler

    func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
        if message.name == "consoleLog" {
            handleConsoleMessage(message)
        } else if message.name == "termsurf" {
            handleTermsurfMessage(message)
        }
    }

    private func handleConsoleMessage(_ message: WKScriptMessage) {
        guard let body = message.body as? [String: Any],
              let levelStr = body["level"] as? String,
              let msg = body["message"] as? String else {
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
              let action = body["action"] as? String else {
            logger.warning("  - Failed to parse message body")
            return
        }

        logger.info("  - action: \(action)")

        switch action {
        case "escape":
            logger.info("Webview \(self.webviewId) requested escape (switch to control mode)")
            logger.info("  - onEscapePressed callback exists: \(self.onEscapePressed != nil)")
            onEscapePressed?()
            logger.info("  - onEscapePressed callback invoked")

        default:
            logger.warning("Unknown termsurf action: \(action)")
        }
    }

    // MARK: - WKNavigationDelegate

    func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
        logger.debug("Navigation started for \(self.webviewId): \(webView.url?.absoluteString ?? "unknown")")
        onURLChanged?(webView.url)
    }

    /// Callback when navigation finishes (for re-establishing focus)
    var onNavigationFinished: (() -> Void)?

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        logger.info("Navigation finished for \(self.webviewId): \(webView.url?.absoluteString ?? "unknown")")

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

    func webView(_ webView: WKWebView, didFailProvisionalNavigation navigation: WKNavigation!, withError error: Error) {
        logger.error("Provisional navigation failed for \(self.webviewId): \(error.localizedDescription)")
    }

    // MARK: - Focus Handling

    override var acceptsFirstResponder: Bool { true }

    override func becomeFirstResponder() -> Bool {
        webView.becomeFirstResponder()
    }
}
