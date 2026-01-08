import Cocoa
import os

private let logger = Logger(subsystem: "com.termsurf", category: "ControlBar")

/// A visual control bar for the webview container.
/// Displays URL on the left (truncated with ellipsis) and mode hint on the right.
/// Supports insert mode for URL editing.
class ControlBar: NSView, NSTextFieldDelegate {
  /// Stack indicator label (left side, shows "(1/2)" etc. when stacked)
  private let stackLabel: NSTextField

  /// The URL text field (after stack label, truncates with ellipsis, editable in insert mode)
  private let urlField: NSTextField

  /// The mode hint label (right side, fixed width)
  private let modeLabel: NSTextField

  /// The actual current URL (stored separately so we can restore on cancel)
  private var actualURL: URL?

  /// Whether we're currently in insert mode
  private(set) var isInsertMode: Bool = false

  /// Current stack position (1-indexed)
  private var stackPosition: Int = 1

  /// Total number of webviews in the stack
  private var stackTotal: Int = 1

  /// Callback when URL is submitted (Enter pressed in insert mode)
  var onURLSubmitted: ((String) -> Void)?

  /// Callback when insert mode is cancelled (Esc pressed in insert mode)
  var onInsertCancelled: (() -> Void)?

  // MARK: - Initialization

  override init(frame: NSRect) {
    stackLabel = NSTextField(labelWithString: "")
    urlField = NSTextField(string: "")
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
    layer?.backgroundColor = NSColor.controlBackgroundColor.cgColor

    // Stack label styling (left side, shows stack position when multiple webviews)
    stackLabel.textColor = NSColor.labelColor
    stackLabel.font = .systemFont(ofSize: 12, weight: .medium)
    stackLabel.isBezeled = false
    stackLabel.drawsBackground = false
    stackLabel.isEditable = false
    stackLabel.isSelectable = false
    stackLabel.isHidden = true  // Hidden by default (only shown when stacked)
    addSubview(stackLabel)

    // URL field styling (after stack label, truncates, system font, native bezel)
    urlField.textColor = NSColor.textColor
    urlField.font = .systemFont(ofSize: 12)
    urlField.isBezeled = true
    urlField.bezelStyle = .roundedBezel
    urlField.drawsBackground = false
    urlField.isEditable = false
    urlField.isSelectable = false
    urlField.lineBreakMode = .byTruncatingTail
    urlField.cell?.truncatesLastVisibleLine = true
    urlField.focusRingType = .default
    urlField.delegate = self
    addSubview(urlField)

    // Mode label styling (right side, fixed width)
    modeLabel.textColor = NSColor.secondaryLabelColor
    modeLabel.font = .systemFont(ofSize: 12)
    modeLabel.isBezeled = false
    modeLabel.drawsBackground = false
    modeLabel.isEditable = false
    modeLabel.isSelectable = false
    modeLabel.alignment = .right
    addSubview(modeLabel)

    // Set initial text for control mode
    updateModeText(mode: .control)
  }

  // MARK: - Mode enum for text updates

  enum Mode {
    case control
    case browse
    case insert
  }

  // MARK: - Text Updates

  /// Update the mode hint text based on current mode
  func updateModeText(mode: Mode) {
    switch mode {
    case .control:
      modeLabel.stringValue = "i to edit, enter to browse, ctrl+c to close"
    case .browse:
      modeLabel.stringValue = "Esc to control"
    case .insert:
      modeLabel.stringValue = "Enter to go, Esc to cancel"
    }
    needsLayout = true
  }

  /// Update the displayed URL (called from WebViewOverlay when URL changes)
  func updateURL(_ url: URL?) {
    actualURL = url
    if !isInsertMode {
      urlField.stringValue = url?.absoluteString ?? ""
    }
    needsLayout = true
  }

  /// Update the stack indicator (called when webviews are added/removed from the pane)
  func updateStackInfo(position: Int, total: Int) {
    self.stackPosition = position
    self.stackTotal = total

    if total > 1 {
      stackLabel.stringValue = "(\(position)/\(total))"
      stackLabel.isHidden = false
    } else {
      stackLabel.isHidden = true
    }
    needsLayout = true
  }

  // MARK: - Temporary Messages

  /// Timer for restoring URL after temporary message
  private var messageTimer: Timer?

  /// Show a temporary message in the URL field (auto-clears after ~2 seconds)
  func showTemporaryMessage(_ message: String) {
    // Cancel any existing timer
    messageTimer?.invalidate()

    // Store the message
    let previousText = urlField.stringValue
    urlField.stringValue = message

    // Restore after 2 seconds
    messageTimer = Timer.scheduledTimer(withTimeInterval: 2.0, repeats: false) { [weak self] _ in
      guard let self = self, !self.isInsertMode else { return }
      // Restore actual URL
      self.urlField.stringValue = self.actualURL?.absoluteString ?? previousText
    }
  }

  // MARK: - Insert Mode

  /// Enter insert mode - make URL field editable and select all
  func enterInsertMode() {
    logger.info("Entering insert mode")
    isInsertMode = true

    // Make field editable
    urlField.isEditable = true
    urlField.isSelectable = true
    urlField.drawsBackground = true
    urlField.backgroundColor = NSColor.textBackgroundColor

    // Become first responder and select all text
    if let window = window {
      window.makeFirstResponder(urlField)
      urlField.selectText(nil)
    }

    updateModeText(mode: .insert)
  }

  /// Exit insert mode - restore to non-editable state
  func exitInsertMode(restoreURL: Bool) {
    logger.info("Exiting insert mode, restoreURL: \(restoreURL)")
    isInsertMode = false

    // Restore URL if cancelled
    if restoreURL {
      urlField.stringValue = actualURL?.absoluteString ?? ""
    }

    // Make field non-editable
    urlField.isEditable = false
    urlField.isSelectable = false
    urlField.drawsBackground = false

    updateModeText(mode: .control)
  }

  // MARK: - NSTextFieldDelegate

  func control(_ control: NSControl, textView: NSTextView, doCommandBy commandSelector: Selector)
    -> Bool
  {
    if commandSelector == #selector(insertNewline(_:)) {
      // Enter pressed - submit URL
      logger.info("Enter pressed in insert mode, submitting URL")
      let urlString = urlField.stringValue
      exitInsertMode(restoreURL: false)
      onURLSubmitted?(urlString)
      return true
    } else if commandSelector == #selector(cancelOperation(_:)) {
      // Esc pressed - cancel
      logger.info("Esc pressed in insert mode, cancelling")
      exitInsertMode(restoreURL: true)
      onInsertCancelled?()
      return true
    }
    return false
  }

  // MARK: - Layout

  override func layout() {
    super.layout()

    let padding: CGFloat = 8
    let spacing: CGFloat = 12
    let stackSpacing: CGFloat = 6
    let verticalCenter = bounds.height / 2

    // URL field height determines its own vertical positioning
    let urlFieldHeight = urlField.intrinsicContentSize.height
    let urlFieldY = verticalCenter - urlFieldHeight / 2

    // Mode label: centered vertically relative to control bar center
    let modeLabelWidth = modeLabel.intrinsicContentSize.width + 4
    let modeLabelHeight = modeLabel.intrinsicContentSize.height
    modeLabel.frame = NSRect(
      x: bounds.width - padding - modeLabelWidth,
      y: verticalCenter - modeLabelHeight / 2,
      width: modeLabelWidth,
      height: modeLabelHeight
    )

    // Stack label: centered vertically relative to control bar center
    var urlFieldX = padding
    if !stackLabel.isHidden {
      let stackLabelWidth = stackLabel.intrinsicContentSize.width + 4
      let stackLabelHeight = stackLabel.intrinsicContentSize.height
      stackLabel.frame = NSRect(
        x: padding,
        y: verticalCenter - stackLabelHeight / 2,
        width: stackLabelWidth,
        height: stackLabelHeight
      )
      urlFieldX = padding + stackLabelWidth + stackSpacing
    }

    // URL field: fills remaining space, centered vertically
    let urlFieldWidth = bounds.width - urlFieldX - modeLabelWidth - spacing - padding
    urlField.frame = NSRect(
      x: urlFieldX,
      y: urlFieldY,
      width: max(0, urlFieldWidth),
      height: urlFieldHeight
    )
  }
}
