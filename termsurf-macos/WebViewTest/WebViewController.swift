// WebViewController.swift
// WebViewTest - WKWebView with console capture

import Cocoa
import WebKit

class WebViewController: NSViewController, WKScriptMessageHandler, WKNavigationDelegate {
    var webView: WKWebView!

    override func loadView() {
        // Create a container view with a reasonable default size
        let containerView = NSView(frame: NSRect(x: 0, y: 0, width: 1024, height: 768))
        self.view = containerView

        // Create the WKWebView configuration
        let config = WKWebViewConfiguration()
        let contentController = WKUserContentController()

        // Inject script to capture console.log and console.error
        let consoleScript = """
        (function() {
            const originalLog = console.log;
            const originalError = console.error;
            const originalWarn = console.warn;
            const originalInfo = console.info;

            console.log = function(...args) {
                window.webkit.messageHandlers.consoleLog.postMessage({
                    level: 'log',
                    message: args.map(arg => {
                        try { return typeof arg === 'object' ? JSON.stringify(arg) : String(arg); }
                        catch { return String(arg); }
                    }).join(' ')
                });
                originalLog.apply(console, args);
            };

            console.error = function(...args) {
                window.webkit.messageHandlers.consoleError.postMessage({
                    level: 'error',
                    message: args.map(arg => {
                        try { return typeof arg === 'object' ? JSON.stringify(arg) : String(arg); }
                        catch { return String(arg); }
                    }).join(' ')
                });
                originalError.apply(console, args);
            };

            console.warn = function(...args) {
                window.webkit.messageHandlers.consoleLog.postMessage({
                    level: 'warn',
                    message: args.map(arg => {
                        try { return typeof arg === 'object' ? JSON.stringify(arg) : String(arg); }
                        catch { return String(arg); }
                    }).join(' ')
                });
                originalWarn.apply(console, args);
            };

            console.info = function(...args) {
                window.webkit.messageHandlers.consoleLog.postMessage({
                    level: 'info',
                    message: args.map(arg => {
                        try { return typeof arg === 'object' ? JSON.stringify(arg) : String(arg); }
                        catch { return String(arg); }
                    }).join(' ')
                });
                originalInfo.apply(console, args);
            };

            // Also capture uncaught errors
            window.onerror = function(message, source, lineno, colno, error) {
                window.webkit.messageHandlers.consoleError.postMessage({
                    level: 'error',
                    message: 'Uncaught: ' + message + ' at ' + source + ':' + lineno + ':' + colno
                });
            };

            window.onunhandledrejection = function(event) {
                window.webkit.messageHandlers.consoleError.postMessage({
                    level: 'error',
                    message: 'Unhandled Promise Rejection: ' + event.reason
                });
            };
        })();
        """

        let userScript = WKUserScript(
            source: consoleScript,
            injectionTime: .atDocumentStart,
            forMainFrameOnly: false
        )
        contentController.addUserScript(userScript)

        // Register message handlers
        contentController.add(self, name: "consoleLog")
        contentController.add(self, name: "consoleError")

        config.userContentController = contentController

        // Create the web view
        webView = WKWebView(frame: view.bounds, configuration: config)
        webView.navigationDelegate = self
        webView.autoresizingMask = [.width, .height]

        view.addSubview(webView)
        NSLog("[WebViewTest] WKWebView created")
    }

    func loadURL(_ urlString: String) {
        guard let url = URL(string: urlString) else {
            NSLog("[WebViewTest] Invalid URL: \(urlString)")
            return
        }
        NSLog("[WebViewTest] Loading URL: \(urlString)")
        webView.load(URLRequest(url: url))
    }

    // MARK: - WKScriptMessageHandler

    func userContentController(_ userContentController: WKUserContentController, didReceive message: WKScriptMessage) {
        guard let body = message.body as? [String: Any],
              let level = body["level"] as? String,
              let msg = body["message"] as? String else {
            NSLog("[WebViewTest] Failed to parse message: \(message.body)")
            return
        }

        if message.name == "consoleLog" {
            // stdout - use fputs to avoid buffering issues
            fputs("[\(level)] \(msg)\n", stdout)
            fflush(stdout)
        } else if message.name == "consoleError" {
            // stderr
            fputs("[error] \(msg)\n", stderr)
            fflush(stderr)
        }
    }

    // MARK: - WKNavigationDelegate

    func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
        NSLog("[WebViewTest] Navigation started")
    }

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        NSLog("[WebViewTest] Navigation finished: \(webView.url?.absoluteString ?? "unknown")")

        // Test console capture by executing JavaScript
        let testScript = """
        console.log('Hello from console.log!');
        console.warn('This is a warning');
        console.error('This is an error message');
        console.log('Object test:', {foo: 'bar', num: 42});
        """
        webView.evaluateJavaScript(testScript) { result, error in
            if let error = error {
                NSLog("[WebViewTest] JS execution error: \(error)")
            }
        }
    }

    func webView(_ webView: WKWebView, didFail navigation: WKNavigation!, withError error: Error) {
        NSLog("[WebViewTest] Navigation failed: \(error.localizedDescription)")
    }
}
