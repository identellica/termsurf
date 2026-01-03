import Cocoa
import os

private let logger = Logger(subsystem: "com.termsurf", category: "WebViewContainer")

/// Container view that holds a WebViewOverlay and FooterView with mode-based focus switching.
///
/// Two modes:
/// - Footer mode: Footer is focused, all terminal keybindings work (ctrl+c, ctrl+h/j/k/l, etc.)
/// - Webview mode: Webview is focused, browser has full control, only Esc escapes
///
/// Visual indicator: Footer is dimmed when webview is focused.
class WebViewContainer: NSView {
    /// The webview ID
    let webviewId: String

    /// The webview overlay (WKWebView wrapper)
    let webViewOverlay: WebViewOverlay

    /// The footer bar
    let footerView: FooterView

    /// Called when the webview should close
    var onClose: ((String) -> Void)?

    /// Height of the footer bar
    private let footerHeight: CGFloat = 24

    /// Current focus mode
    enum FocusMode {
        case footer
        case webview
    }

    private(set) var focusMode: FocusMode = .webview {
        didSet {
            if oldValue != focusMode {
                updateFocusVisuals()
            }
        }
    }

    /// Public getter for SurfaceView to check if we're in footer mode
    var isFooterMode: Bool { focusMode == .footer }

    // MARK: - Initialization

    init(url: URL, webviewId: String, profile: String? = nil) {
        self.webviewId = webviewId
        self.webViewOverlay = WebViewOverlay(url: url, webviewId: webviewId, profile: profile)
        self.footerView = FooterView()
        super.init(frame: .zero)

        setupSubviews()
        setupCallbacks()

        // Ensure initial visual state is correct
        updateFocusVisuals()

        logger.info("WebViewContainer \(webviewId) created")
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    deinit {
        logger.info("WebViewContainer \(self.webviewId) deallocated")
    }

    // MARK: - View Lifecycle

    override func viewDidMoveToWindow() {
        super.viewDidMoveToWindow()
        logger.info("viewDidMoveToWindow called, window: \(String(describing: self.window))")

        if window != nil {
            // When we're added to a window, ensure webview has focus
            DispatchQueue.main.async { [weak self] in
                guard let self = self else { return }
                logger.info("viewDidMoveToWindow async block executing for \(self.webviewId)")
                self.focusWebView()
            }
        }
    }

    // MARK: - Setup

    private func setupSubviews() {
        // Footer at bottom
        addSubview(footerView)

        // WebView fills rest (above footer)
        addSubview(webViewOverlay)
    }

    override func layout() {
        super.layout()

        // Footer at bottom
        footerView.frame = NSRect(
            x: 0,
            y: 0,
            width: bounds.width,
            height: footerHeight
        )

        // WebView fills rest (above footer)
        webViewOverlay.frame = NSRect(
            x: 0,
            y: footerHeight,
            width: bounds.width,
            height: bounds.height - footerHeight
        )
    }

    private func setupCallbacks() {
        // WebView: Esc -> focus footer (switch to terminal mode)
        webViewOverlay.onEscapePressed = { [weak self] in
            self?.focusFooter()
        }

        // WebView: Navigation finished -> re-establish proper focus state
        webViewOverlay.onNavigationFinished = { [weak self] in
            guard let self = self else { return }
            if self.focusMode == .webview {
                // Re-focus web content after navigation so cursor is in the right place
                self.webViewOverlay.focusWebContent()
            } else {
                // Re-blur web content after navigation (page may have auto-focused an element)
                self.webViewOverlay.blurWebContent()
            }
        }
    }

    // MARK: - First Responder

    override var acceptsFirstResponder: Bool { true }

    override func becomeFirstResponder() -> Bool {
        // When the container is asked to become first responder (e.g., from pane navigation),
        // redirect focus to the appropriate view based on current mode
        logger.debug("WebViewContainer asked to become first responder, redirecting to \(String(describing: self.focusMode))")

        if focusMode == .webview {
            return webViewOverlay.webView.becomeFirstResponder()
        } else {
            // In footer mode, parent SurfaceView should be first responder
            return superview?.becomeFirstResponder() ?? false
        }
    }

    // MARK: - Focus Management

    /// Focus the webview (browser mode)
    func focusWebView() {
        logger.info("focusWebView called for \(self.webviewId)")
        logger.info("  - window: \(String(describing: self.window))")
        logger.info("  - webViewOverlay.webView: \(String(describing: self.webViewOverlay.webView))")

        focusMode = .webview
        let success = window?.makeFirstResponder(webViewOverlay.webView) ?? false
        logger.info("  - makeFirstResponder result: \(success)")
        logger.info("  - actual firstResponder after: \(String(describing: self.window?.firstResponder))")

        // Also focus web content so cursor appears in input fields
        webViewOverlay.focusWebContent()
    }

    /// Focus the footer (terminal mode)
    /// Makes parent SurfaceView the first responder so ghostty keybindings work
    func focusFooter() {
        logger.info("focusFooter called for \(self.webviewId)")
        logger.info("  - superview: \(String(describing: self.superview))")
        logger.info("  - window: \(String(describing: self.window))")

        focusMode = .footer
        if let surfaceView = superview {
            let success = window?.makeFirstResponder(surfaceView) ?? false
            logger.info("  - makeFirstResponder result: \(success)")
            logger.info("  - actual firstResponder after: \(String(describing: self.window?.firstResponder))")
        } else {
            logger.warning("  - superview is nil!")
        }
        // Blur web content so keystrokes don't go to webview
        webViewOverlay.blurWebContent()
    }

    /// Sync internal state to footer mode without changing first responder.
    /// Called when SurfaceView detects it's receiving keys but our state is out of sync
    /// (e.g., after pane switching).
    func syncToFooterMode() {
        logger.info("syncToFooterMode called for \(self.webviewId)")
        focusMode = .footer
        // Blur web content to ensure keystrokes don't leak to webview
        webViewOverlay.blurWebContent()
    }

    private func updateFocusVisuals() {
        let isWebviewFocused = (focusMode == .webview)
        footerView.updateText(isWebviewFocused: isWebviewFocused)
        logger.debug("Focus mode: \(String(describing: self.focusMode))")
    }
}
