// CEFClient.swift
// CEFKit - Swift bindings for CEF
//
// Client interface that provides handlers for browser events.

import Foundation
import CEF

// MARK: - Client Protocol

/// Combined protocol for all browser event handlers.
/// Implement the specific delegate protocols you need.
public protocol CEFClientDelegate: AnyObject,
                                   CEFDisplayHandlerDelegate,
                                   CEFLifeSpanHandlerDelegate {
}

// Default implementations for all methods
public extension CEFClientDelegate {
    // Display handler defaults
    func onConsoleMessage(browser: CEFBrowser?, level: CEFLogSeverity, message: String, source: String, line: Int) -> Bool { false }
    func onTitleChange(browser: CEFBrowser?, title: String) {}
    func onAddressChange(browser: CEFBrowser?, url: String) {}

    // Life span handler defaults
    func onAfterCreated(browser: CEFBrowser?) {}
    func doClose(browser: CEFBrowser?) -> Bool { false }
    func onBeforeClose(browser: CEFBrowser?) {}
}

// MARK: - CEFClient

/// Client that provides handlers for browser events.
/// Create a client with a delegate to receive browser callbacks.
///
/// Usage:
/// ```swift
/// class MyBrowserDelegate: CEFClientDelegate {
///     func onConsoleMessage(...) -> Bool {
///         print("Console: \(message)")
///         return false
///     }
/// }
///
/// let delegate = MyBrowserDelegate()
/// let client = CEFClient(delegate: delegate)
/// let browser = try CEFBrowser.create(url: "https://example.com", client: client)
/// ```
public final class CEFClient {
    private weak var delegate: CEFClientDelegate?
    private var displayHandler: CEFDisplayHandler?
    private var lifeSpanHandler: CEFLifeSpanHandler?
    private var cefClient: UnsafeMutablePointer<cef_client_t>?

    /// Create a new client with the specified delegate.
    /// - Parameter delegate: Object that will receive browser callbacks
    public init(delegate: CEFClientDelegate) {
        self.delegate = delegate
        self.displayHandler = CEFDisplayHandler(delegate: delegate)
        self.lifeSpanHandler = CEFLifeSpanHandler(delegate: delegate)
        self.cefClient = createClient()
    }

    deinit {
        // Client is ref-counted
    }

    /// Get the underlying CEF client pointer.
    /// Used internally when creating browsers.
    var cefClientPtr: UnsafeMutablePointer<cef_client_t>? {
        return cefClient
    }

    private func createClient() -> UnsafeMutablePointer<cef_client_t> {
        let ptr = allocateHandler(cef_client_t.self)

        setupBaseRefCounted(ptr)

        let context = CEFHandlerContext(self)
        setHandlerContext(context, on: ptr)

        // Return display handler
        ptr.pointee.get_display_handler = { clientPtr in
            guard let client: CEFClient = getHandler(from: clientPtr),
                  let handler = client.displayHandler?.handler else {
                return nil
            }
            // Add ref before returning
            handler.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1) { base in
                base.pointee.add_ref?(base)
            }
            return handler
        }

        // Return life span handler
        ptr.pointee.get_life_span_handler = { clientPtr in
            guard let client: CEFClient = getHandler(from: clientPtr),
                  let handler = client.lifeSpanHandler?.handler else {
                return nil
            }
            handler.withMemoryRebound(to: cef_base_ref_counted_t.self, capacity: 1) { base in
                base.pointee.add_ref?(base)
            }
            return handler
        }

        // Other handlers return nil (use defaults)
        ptr.pointee.get_audio_handler = { _ in nil }
        ptr.pointee.get_command_handler = { _ in nil }
        ptr.pointee.get_context_menu_handler = { _ in nil }
        ptr.pointee.get_dialog_handler = { _ in nil }
        ptr.pointee.get_download_handler = { _ in nil }
        ptr.pointee.get_drag_handler = { _ in nil }
        ptr.pointee.get_find_handler = { _ in nil }
        ptr.pointee.get_focus_handler = { _ in nil }
        ptr.pointee.get_frame_handler = { _ in nil }
        ptr.pointee.get_permission_handler = { _ in nil }
        ptr.pointee.get_jsdialog_handler = { _ in nil }
        ptr.pointee.get_keyboard_handler = { _ in nil }
        ptr.pointee.get_load_handler = { _ in nil }
        ptr.pointee.get_print_handler = { _ in nil }
        ptr.pointee.get_render_handler = { _ in nil }
        ptr.pointee.get_request_handler = { _ in nil }

        return ptr
    }
}
