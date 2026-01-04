import Cocoa
import os

private let logger = Logger(subsystem: "com.termsurf", category: "WebViewContainer")

/// Container view that holds a WebViewOverlay and ControlBar with mode-based focus switching.
///
/// Three modes:
/// - Control mode: SurfaceView is focused, all terminal keybindings work (ctrl+c, ctrl+h/j/k/l, etc.)
/// - Browse mode: Webview is focused, browser has full control, only Esc escapes
/// - Insert mode: URL field is focused, can edit URL, Enter navigates, Esc cancels
class WebViewContainer: NSView {
    /// The webview ID
    let webviewId: String

    /// The webview overlay (WKWebView wrapper)
    let webViewOverlay: WebViewOverlay

    /// The control bar
    let controlBar: ControlBar

    /// Called when the webview should close (webviewId, exitCode)
    var onClose: ((String, Int) -> Void)?

    /// Called when console output is received (level, message)
    var onConsoleOutput: ((WebViewOverlay.ConsoleLevel, String) -> Void)? {
        didSet {
            webViewOverlay.onConsoleOutput = onConsoleOutput
        }
    }

    /// Height of the control bar
    private let controlBarHeight: CGFloat = 24

    /// Track whether initial focus setup has been done
    private var didInitialFocus = false

    /// Captured superview before removal (for cursor rect invalidation)
    private weak var previousSuperview: NSView?

    /// Stack position (1-indexed, 1 = bottom of stack)
    private(set) var stackPosition: Int = 1

    /// Total number of webviews in the stack
    private(set) var stackTotal: Int = 1

    /// Current focus mode
    enum FocusMode {
        case control
        case browse
        case insert
    }

    private(set) var focusMode: FocusMode = .browse {
        didSet {
            if oldValue != focusMode {
                updateFocusVisuals()
            }
        }
    }

    /// Public getter for SurfaceView to check if we're in control mode
    var isControlMode: Bool { focusMode == .control }

    /// Public getter to check if we're in insert mode
    var isInsertMode: Bool { focusMode == .insert }

    // MARK: - Initialization

    init(url: URL, webviewId: String, profile: String? = nil, incognito: Bool = false,
         jsApi: Bool = false, stackPosition: Int = 1, stackTotal: Int = 1) {
        self.webviewId = webviewId
        self.stackPosition = stackPosition
        self.stackTotal = stackTotal
        self.webViewOverlay = WebViewOverlay(url: url, webviewId: webviewId, profile: profile, incognito: incognito, jsApi: jsApi)
        self.controlBar = ControlBar()
        super.init(frame: .zero)

        setupSubviews()
        setupCallbacks()

        // Ensure initial visual state is correct
        updateFocusVisuals()

        // Set initial stack info on control bar
        controlBar.updateStackInfo(position: stackPosition, total: stackTotal)

        logger.info("WebViewContainer \(webviewId) created with jsApi=\(jsApi) [stack: \(stackPosition)/\(stackTotal)]")
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    deinit {
        logger.info("WebViewContainer \(self.webviewId) deallocated")
    }

    // MARK: - View Lifecycle

    override func viewWillMove(toSuperview newSuperview: NSView?) {
        // Capture current superview before change (for cursor invalidation on removal)
        previousSuperview = superview
        super.viewWillMove(toSuperview: newSuperview)
    }

    override func viewDidMoveToSuperview() {
        super.viewDidMoveToSuperview()

        // Invalidate cursor rects on superview so it can update based on our presence
        if let newSuperview = superview {
            // Added to a superview - invalidate its cursor rects
            newSuperview.window?.invalidateCursorRects(for: newSuperview)
        } else if let oldSuperview = previousSuperview {
            // Removed from superview - invalidate old superview's cursor rects
            oldSuperview.window?.invalidateCursorRects(for: oldSuperview)
        }
        previousSuperview = nil
    }

    override func viewDidMoveToWindow() {
        super.viewDidMoveToWindow()
        logger.info("viewDidMoveToWindow called, window: \(String(describing: self.window))")

        // Only do initial focus setup once (not when view hierarchy changes due to splits)
        if window != nil && !didInitialFocus {
            didInitialFocus = true
            // When we're added to a window, ensure webview has focus (start in browse mode)
            DispatchQueue.main.async { [weak self] in
                guard let self = self else { return }
                logger.info("viewDidMoveToWindow async block executing for \(self.webviewId)")
                self.focusBrowser()
            }
        }
    }

    // MARK: - Setup

    private func setupSubviews() {
        // Control bar at bottom
        addSubview(controlBar)

        // WebView fills rest (above control bar)
        addSubview(webViewOverlay)
    }

    override func layout() {
        super.layout()

        // Control bar at bottom
        controlBar.frame = NSRect(
            x: 0,
            y: 0,
            width: bounds.width,
            height: controlBarHeight
        )

        // WebView fills rest (above control bar)
        webViewOverlay.frame = NSRect(
            x: 0,
            y: controlBarHeight,
            width: bounds.width,
            height: bounds.height - controlBarHeight
        )
    }

    private func setupCallbacks() {
        // WebView: Esc -> switch to control mode
        webViewOverlay.onEscapePressed = { [weak self] in
            self?.focusControlBar()
        }

        // WebView: URL changed -> update control bar
        webViewOverlay.onURLChanged = { [weak self] url in
            self?.controlBar.updateURL(url)
        }

        // WebView: Navigation finished -> re-establish proper focus state
        webViewOverlay.onNavigationFinished = { [weak self] in
            guard let self = self else { return }
            if self.focusMode == .browse {
                // Re-focus web content after navigation so cursor is in the right place
                self.webViewOverlay.focusWebContent()
            } else {
                // Re-blur web content after navigation (page may have auto-focused an element)
                self.webViewOverlay.blurWebContent()
            }
        }

        // ControlBar: URL submitted -> navigate and switch to browse mode
        controlBar.onURLSubmitted = { [weak self] urlString in
            guard let self = self else { return }
            logger.info("URL submitted: \(urlString)")

            // Normalize URL (add https:// if no scheme)
            let normalizedURL = self.normalizeURL(urlString)
            if let url = URL(string: normalizedURL) {
                self.webViewOverlay.navigate(to: url)
                self.focusBrowser()
            } else {
                logger.warning("Invalid URL: \(urlString)")
                self.focusControlBar()
            }
        }

        // ControlBar: Insert cancelled -> switch back to control mode
        controlBar.onInsertCancelled = { [weak self] in
            self?.focusControlBar()
        }

        // WebView: JS API exit() called -> close webview with exit code
        webViewOverlay.onExit = { [weak self] exitCode in
            guard let self = self else { return }
            logger.info("JS API exit(\(exitCode)) called for \(self.webviewId)")
            self.onClose?(self.webviewId, exitCode)
        }
    }

    // MARK: - First Responder

    override var acceptsFirstResponder: Bool { true }

    override func becomeFirstResponder() -> Bool {
        // When the container is asked to become first responder (e.g., from pane navigation),
        // redirect focus to the appropriate view based on current mode
        logger.debug("WebViewContainer asked to become first responder, redirecting to \(String(describing: self.focusMode))")

        switch focusMode {
        case .browse:
            return webViewOverlay.webView.becomeFirstResponder()
        case .control:
            // In control mode, parent SurfaceView should be first responder
            return superview?.becomeFirstResponder() ?? false
        case .insert:
            // In insert mode, URL field should keep focus
            return true
        }
    }

    // MARK: - Focus Management

    /// Focus the browser (browse mode)
    func focusBrowser() {
        logger.info("focusBrowser called for \(self.webviewId)")
        logger.info("  - window: \(String(describing: self.window))")
        logger.info("  - webViewOverlay.webView: \(String(describing: self.webViewOverlay.webView))")

        focusMode = .browse
        let success = window?.makeFirstResponder(webViewOverlay.webView) ?? false
        logger.info("  - makeFirstResponder result: \(success)")
        logger.info("  - actual firstResponder after: \(String(describing: self.window?.firstResponder))")

        // Also focus web content so cursor appears in input fields
        webViewOverlay.focusWebContent()
    }

    /// Focus the control bar (control mode)
    /// Makes parent SurfaceView the first responder so ghostty keybindings work
    func focusControlBar() {
        logger.info("focusControlBar called for \(self.webviewId)")
        logger.info("  - superview: \(String(describing: self.superview))")
        logger.info("  - window: \(String(describing: self.window))")

        focusMode = .control
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

    /// Focus the URL field (insert mode)
    func focusURLField() {
        logger.info("focusURLField called for \(self.webviewId)")
        focusMode = .insert
        controlBar.enterInsertMode()
        // Blur web content so keystrokes don't go to webview
        webViewOverlay.blurWebContent()
    }

    /// Sync internal state to control mode without changing first responder.
    /// Called when SurfaceView detects it's receiving keys but our state is out of sync
    /// (e.g., after pane switching).
    func syncToControlMode() {
        logger.info("syncToControlMode called for \(self.webviewId)")
        focusMode = .control
        // Blur web content to ensure keystrokes don't leak to webview
        webViewOverlay.blurWebContent()
    }

    private func updateFocusVisuals() {
        let mode: ControlBar.Mode
        switch focusMode {
        case .control:
            mode = .control
        case .browse:
            mode = .browse
        case .insert:
            mode = .insert
        }
        controlBar.updateModeText(mode: mode)
        logger.debug("Focus mode: \(String(describing: self.focusMode))")
    }

    // MARK: - Stack Management

    /// Update the stack position and total for this webview.
    /// Called by WebViewManager when webviews are added/removed from the pane.
    func updateStackInfo(position: Int, total: Int) {
        self.stackPosition = position
        self.stackTotal = total
        controlBar.updateStackInfo(position: position, total: total)
        logger.debug("WebViewContainer \(self.webviewId) stack updated: \(position)/\(total)")
    }

    // MARK: - Helpers

    /// Normalize a URL string: prepend https:// if no scheme
    private func normalizeURL(_ urlString: String) -> String {
        let trimmed = urlString.trimmingCharacters(in: .whitespacesAndNewlines)
        if trimmed.isEmpty {
            return trimmed
        }

        // If already has a scheme, return as-is
        if trimmed.hasPrefix("http://") ||
           trimmed.hasPrefix("https://") ||
           trimmed.hasPrefix("file://") {
            return trimmed
        }

        // Prepend https://
        return "https://\(trimmed)"
    }
}
