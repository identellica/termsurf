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

    private(set) var focusMode: FocusMode = .footer {
        didSet {
            if oldValue != focusMode {
                updateFocusVisuals()
            }
        }
    }

    // MARK: - Initialization

    init(url: URL, webviewId: String, profile: String? = nil) {
        self.webviewId = webviewId
        self.webViewOverlay = WebViewOverlay(url: url, webviewId: webviewId, profile: profile)
        self.footerView = FooterView()
        super.init(frame: .zero)

        setupSubviews()
        setupCallbacks()
        updateFocusVisuals()

        logger.info("WebViewContainer \(webviewId) created")
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    deinit {
        logger.info("WebViewContainer \(self.webviewId) deallocated")
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
        // Footer: Enter -> focus webview
        footerView.onEnterPressed = { [weak self] in
            self?.focusWebView()
        }

        // Footer: Ctrl+C -> close
        footerView.onCloseRequested = { [weak self] in
            guard let self = self else { return }
            logger.info("Close requested for webview \(self.webviewId)")
            self.onClose?(self.webviewId)
        }

        // WebView: Esc -> focus footer
        webViewOverlay.onEscapePressed = { [weak self] in
            self?.focusFooter()
        }
    }

    // MARK: - Focus Management

    /// Focus the webview (browser mode)
    func focusWebView() {
        logger.info("Focusing webview for \(self.webviewId)")
        focusMode = .webview
        window?.makeFirstResponder(webViewOverlay.webView)
    }

    /// Focus the footer (terminal mode)
    func focusFooter() {
        logger.info("Focusing footer for \(self.webviewId)")
        focusMode = .footer
        window?.makeFirstResponder(footerView)
    }

    private func updateFocusVisuals() {
        // Dim footer when webview is focused, full opacity when footer is focused
        // Webview always stays at full opacity (user needs to see content)
        let footerAlpha: CGFloat = (focusMode == .footer) ? 1.0 : 0.5
        footerView.alphaValue = footerAlpha

        logger.debug("Focus mode: \(String(describing: self.focusMode)), footer alpha: \(footerAlpha)")
    }
}
