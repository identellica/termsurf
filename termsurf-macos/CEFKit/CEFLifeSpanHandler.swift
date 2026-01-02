// CEFLifeSpanHandler.swift
// CEFKit - Swift bindings for CEF
//
// Life span handler for browser lifecycle events.

import Foundation
import CEF

// MARK: - Life Span Handler Protocol

/// Protocol for handling browser lifecycle events.
public protocol CEFLifeSpanHandlerDelegate: AnyObject {
    /// Called after a new browser is created.
    /// - Parameter browser: The newly created browser
    func onAfterCreated(browser: CEFBrowser?)

    /// Called when a browser is about to close.
    /// Return true to cancel the close, false to allow it.
    /// - Parameter browser: The browser being closed
    /// - Returns: true to cancel close, false to allow
    func doClose(browser: CEFBrowser?) -> Bool

    /// Called just before a browser is destroyed.
    /// Release all references to the browser.
    /// - Parameter browser: The browser being destroyed
    func onBeforeClose(browser: CEFBrowser?)
}

// Default implementations
public extension CEFLifeSpanHandlerDelegate {
    func onAfterCreated(browser: CEFBrowser?) {}
    func doClose(browser: CEFBrowser?) -> Bool { false }
    func onBeforeClose(browser: CEFBrowser?) {}
}

// MARK: - Life Span Handler Implementation

/// Internal class that wraps a CEFLifeSpanHandlerDelegate.
final class CEFLifeSpanHandler {
    weak var delegate: CEFLifeSpanHandlerDelegate?
    private var cefHandler: UnsafeMutablePointer<cef_life_span_handler_t>?

    init(delegate: CEFLifeSpanHandlerDelegate) {
        self.delegate = delegate
        self.cefHandler = createHandler()
    }

    var handler: UnsafeMutablePointer<cef_life_span_handler_t>? {
        return cefHandler
    }

    private func createHandler() -> UnsafeMutablePointer<cef_life_span_handler_t> {
        let ptr = allocateHandler(cef_life_span_handler_t.self)

        setupBaseRefCounted(ptr)

        let context = CEFHandlerContext(self)
        setHandlerContext(context, on: ptr)

        ptr.pointee.on_after_created = { handlerPtr, browserPtr in
            guard let handler: CEFLifeSpanHandler = getHandler(from: handlerPtr),
                  let delegate = handler.delegate else {
                return
            }

            let browser = browserPtr.flatMap { CEFBrowser(cefBrowser: $0) }
            delegate.onAfterCreated(browser: browser)
        }

        ptr.pointee.do_close = { handlerPtr, browserPtr in
            guard let handler: CEFLifeSpanHandler = getHandler(from: handlerPtr),
                  let delegate = handler.delegate else {
                return 0
            }

            let browser = browserPtr.flatMap { CEFBrowser(cefBrowser: $0) }
            return delegate.doClose(browser: browser) ? 1 : 0
        }

        ptr.pointee.on_before_close = { handlerPtr, browserPtr in
            guard let handler: CEFLifeSpanHandler = getHandler(from: handlerPtr),
                  let delegate = handler.delegate else {
                return
            }

            let browser = browserPtr.flatMap { CEFBrowser(cefBrowser: $0) }
            delegate.onBeforeClose(browser: browser)
        }

        return ptr
    }
}
