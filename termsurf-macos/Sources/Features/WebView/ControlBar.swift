import Cocoa
import os

private let logger = Logger(subsystem: "com.termsurf", category: "ControlBar")

/// A visual control bar for the webview container.
/// Displays mode-specific hint text. Keyboard handling is done by SurfaceView.
class ControlBar: NSView {
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

        // Set initial text for control mode
        updateText(isBrowseMode: false)
    }

    // MARK: - Text Updates

    /// Update the label text based on current focus mode
    func updateText(isBrowseMode: Bool) {
        if isBrowseMode {
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
}
