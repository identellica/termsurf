// CEFDisplayHandler.swift
// CEFKit - Swift bindings for CEF
//
// Display handler for browser display state events, including console messages.

import Foundation
import CEF

// MARK: - Display Handler Protocol

/// Protocol for handling browser display events.
/// Implement this to receive console messages and other display-related callbacks.
public protocol CEFDisplayHandlerDelegate: AnyObject {
    /// Called when a console message is received.
    /// - Parameters:
    ///   - browser: The browser that generated the message
    ///   - level: Log severity level
    ///   - message: The console message text
    ///   - source: Source file URL
    ///   - line: Line number in the source file
    /// - Returns: true to suppress the message from being output to the console
    func onConsoleMessage(
        browser: CEFBrowser?,
        level: CEFLogSeverity,
        message: String,
        source: String,
        line: Int
    ) -> Bool

    /// Called when the page title changes.
    /// - Parameters:
    ///   - browser: The browser whose title changed
    ///   - title: The new page title
    func onTitleChange(browser: CEFBrowser?, title: String)

    /// Called when the page URL changes.
    /// - Parameters:
    ///   - browser: The browser whose URL changed
    ///   - url: The new URL
    func onAddressChange(browser: CEFBrowser?, url: String)
}

// Default implementations (all optional)
public extension CEFDisplayHandlerDelegate {
    func onConsoleMessage(browser: CEFBrowser?, level: CEFLogSeverity, message: String, source: String, line: Int) -> Bool { false }
    func onTitleChange(browser: CEFBrowser?, title: String) {}
    func onAddressChange(browser: CEFBrowser?, url: String) {}
}

// MARK: - Log Severity

/// Console message log severity levels
public enum CEFLogSeverity: Int32 {
    case `default` = 0
    case verbose = 1
    case info = 2
    case warning = 3
    case error = 4
    case fatal = 5

    /// Alias for verbose (CEF uses debug = 1, same as verbose)
    public static let debug = CEFLogSeverity.verbose

    /// Whether this message should go to stderr (warning/error/fatal)
    public var isError: Bool {
        return self.rawValue >= CEFLogSeverity.warning.rawValue
    }

    init(cef: cef_log_severity_t) {
        self = CEFLogSeverity(rawValue: Int32(cef.rawValue)) ?? .default
    }
}

// MARK: - Display Handler Implementation

/// Internal class that wraps a CEFDisplayHandlerDelegate and creates the C handler struct.
final class CEFDisplayHandler {
    weak var delegate: CEFDisplayHandlerDelegate?
    private var cefHandler: UnsafeMutablePointer<cef_display_handler_t>?

    init(delegate: CEFDisplayHandlerDelegate) {
        self.delegate = delegate
        self.cefHandler = createHandler()
    }

    deinit {
        // Handler is ref-counted, will be freed when ref count reaches 0
    }

    /// Get the CEF handler pointer to pass to cef_client_t
    var handler: UnsafeMutablePointer<cef_display_handler_t>? {
        return cefHandler
    }

    private func createHandler() -> UnsafeMutablePointer<cef_display_handler_t> {
        let ptr = allocateHandler(cef_display_handler_t.self)

        // Set up base ref counting
        setupBaseRefCounted(ptr)

        // Store context
        let context = CEFHandlerContext(self)
        setHandlerContext(context, on: ptr)

        // Set up callbacks
        ptr.pointee.on_console_message = { handlerPtr, browserPtr, level, message, source, line in
            guard let handler: CEFDisplayHandler = getHandler(from: handlerPtr),
                  let delegate = handler.delegate else {
                return 0
            }

            let browser = browserPtr.flatMap { CEFBrowser(cefBrowser: $0) }
            let result = delegate.onConsoleMessage(
                browser: browser,
                level: CEFLogSeverity(cef: level),
                message: cefStringToSwift(message),
                source: cefStringToSwift(source),
                line: Int(line)
            )
            return result ? 1 : 0
        }

        ptr.pointee.on_title_change = { handlerPtr, browserPtr, title in
            guard let handler: CEFDisplayHandler = getHandler(from: handlerPtr),
                  let delegate = handler.delegate else {
                return
            }

            let browser = browserPtr.flatMap { CEFBrowser(cefBrowser: $0) }
            delegate.onTitleChange(browser: browser, title: cefStringToSwift(title))
        }

        ptr.pointee.on_address_change = { handlerPtr, browserPtr, framePtr, url in
            guard let handler: CEFDisplayHandler = getHandler(from: handlerPtr),
                  let delegate = handler.delegate else {
                return
            }

            let browser = browserPtr.flatMap { CEFBrowser(cefBrowser: $0) }
            delegate.onAddressChange(browser: browser, url: cefStringToSwift(url))
        }

        return ptr
    }
}
