import Cocoa
import os

private let logger = Logger(subsystem: "com.termsurf", category: "FooterView")

/// A footer bar for the webview container that handles keyboard input when focused.
/// Displays "TermSurf Browser" text and handles Enter (focus webview) and ctrl+c (close).
class FooterView: NSView {
    /// Called when Enter is pressed (to focus the webview)
    var onEnterPressed: (() -> Void)?

    /// Called when ctrl+c is pressed (to close the webview)
    var onCloseRequested: (() -> Void)?

    /// The text label
    private let label: NSTextField

    // MARK: - Initialization

    override init(frame: NSRect) {
        label = NSTextField(labelWithString: "TermSurf Browser")
        super.init(frame: frame)
        setupUI()
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    // MARK: - Setup

    private func setupUI() {
        // Background
        wantsLayer = true
        layer?.backgroundColor = NSColor(white: 0.15, alpha: 1.0).cgColor

        // Label styling
        label.textColor = NSColor(white: 0.7, alpha: 1.0)
        label.font = .systemFont(ofSize: 11)
        label.isBezeled = false
        label.drawsBackground = false
        label.isEditable = false
        label.isSelectable = false
        addSubview(label)

        // Set initial text for footer mode
        updateText(isWebviewFocused: false)
    }

    // MARK: - Text Updates

    /// Update the label text based on current focus mode
    func updateText(isWebviewFocused: Bool) {
        if isWebviewFocused {
            label.stringValue = "Esc to exit browser"
        } else {
            label.stringValue = "Enter to browse, ctrl+c to close"
        }
        needsLayout = true
    }

    override func layout() {
        super.layout()

        // Position label with left padding, vertically centered, full available width
        let padding: CGFloat = 8
        let labelHeight = label.intrinsicContentSize.height
        label.frame = NSRect(
            x: padding,
            y: (bounds.height - labelHeight) / 2,
            width: bounds.width - (padding * 2),
            height: labelHeight
        )
    }

    // MARK: - First Responder

    override var acceptsFirstResponder: Bool { true }

    override func becomeFirstResponder() -> Bool {
        let result = super.becomeFirstResponder()
        if result {
            logger.debug("FooterView became first responder")
        }
        return result
    }

    override func resignFirstResponder() -> Bool {
        let result = super.resignFirstResponder()
        if result {
            logger.debug("FooterView resigned first responder")
        }
        return result
    }

    // MARK: - Keyboard Handling

    override func keyDown(with event: NSEvent) {
        let chars = event.charactersIgnoringModifiers ?? ""

        // Enter key - focus webview
        if chars == "\r" {
            logger.info("Enter pressed - focusing webview")
            onEnterPressed?()
            return
        }

        // Ctrl+C - close webview
        if event.modifierFlags.contains(.control) && chars == "c" {
            logger.info("Ctrl+C pressed - closing webview")
            onCloseRequested?()
            return
        }

        // Pass unhandled keys up the responder chain
        // This allows ctrl+h/j/k/l to reach the menu system for pane navigation
        super.keyDown(with: event)
    }

    // Suppress the beep for unhandled keys that we pass through
    override func performKeyEquivalent(with event: NSEvent) -> Bool {
        // Let the responder chain handle key equivalents
        return false
    }
}
