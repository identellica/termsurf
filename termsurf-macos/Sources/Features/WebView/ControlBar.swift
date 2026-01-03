import Cocoa
import os

private let logger = Logger(subsystem: "com.termsurf", category: "ControlBar")

/// A visual control bar for the webview container.
/// Displays URL on the left (truncated with ellipsis) and mode hint on the right.
/// Keyboard handling is done by SurfaceView.
class ControlBar: NSView {
    /// The URL label (left side, truncates with ellipsis)
    private let urlLabel: NSTextField

    /// The mode hint label (right side, fixed width)
    private let modeLabel: NSTextField

    // MARK: - Initialization

    override init(frame: NSRect) {
        urlLabel = NSTextField(labelWithString: "")
        modeLabel = NSTextField(labelWithString: "")
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

        // URL label styling (left side, truncates, monospace font)
        urlLabel.textColor = NSColor(white: 0.7, alpha: 1.0)
        urlLabel.font = .monospacedSystemFont(ofSize: 11, weight: .regular)
        urlLabel.isBezeled = false
        urlLabel.drawsBackground = false
        urlLabel.isEditable = false
        urlLabel.isSelectable = false
        urlLabel.lineBreakMode = .byTruncatingTail
        urlLabel.cell?.truncatesLastVisibleLine = true
        addSubview(urlLabel)

        // Mode label styling (right side, fixed width)
        modeLabel.textColor = NSColor(white: 0.7, alpha: 1.0)
        modeLabel.font = .systemFont(ofSize: 11)
        modeLabel.isBezeled = false
        modeLabel.drawsBackground = false
        modeLabel.isEditable = false
        modeLabel.isSelectable = false
        modeLabel.alignment = .right
        addSubview(modeLabel)

        // Set initial text for control mode
        updateText(isBrowseMode: false)
    }

    // MARK: - Text Updates

    /// Update the mode hint text based on current focus mode
    func updateText(isBrowseMode: Bool) {
        if isBrowseMode {
            modeLabel.stringValue = "Esc to exit"
        } else {
            modeLabel.stringValue = "Enter to browse, ctrl+c to close"
        }
        needsLayout = true
    }

    /// Update the displayed URL
    func updateURL(_ url: URL?) {
        urlLabel.stringValue = url?.absoluteString ?? ""
        needsLayout = true
    }

    override func layout() {
        super.layout()

        let padding: CGFloat = 8
        let spacing: CGFloat = 12
        let labelHeight = max(urlLabel.intrinsicContentSize.height, modeLabel.intrinsicContentSize.height)

        // Mode label: fixed width based on content, positioned at right
        // Add small buffer to intrinsicContentSize for proper text rendering
        let modeLabelWidth = modeLabel.intrinsicContentSize.width + 4
        modeLabel.frame = NSRect(
            x: bounds.width - padding - modeLabelWidth,
            y: (bounds.height - labelHeight) / 2,
            width: modeLabelWidth,
            height: labelHeight
        )

        // URL label: fills remaining space on left
        let urlLabelWidth = bounds.width - padding - modeLabelWidth - spacing - padding
        urlLabel.frame = NSRect(
            x: padding,
            y: (bounds.height - labelHeight) / 2,
            width: max(0, urlLabelWidth),
            height: labelHeight
        )
    }
}
