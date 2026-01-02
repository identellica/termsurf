// AppDelegate.swift
// WebViewTest - WKWebView test app

import Cocoa

class AppDelegate: NSObject, NSApplicationDelegate {
    var window: NSWindow!
    var webViewController: WebViewController!

    func applicationDidFinishLaunching(_ notification: Notification) {
        NSLog("[WebViewTest] applicationDidFinishLaunching starting")

        // Set activation policy - required for window to appear without a nib
        NSApp.setActivationPolicy(.regular)

        // Create window
        NSLog("[WebViewTest] Creating window...")
        window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 1024, height: 768),
            styleMask: [.titled, .closable, .miniaturizable, .resizable],
            backing: .buffered,
            defer: false
        )
        window.title = "WebViewTest"
        window.center()

        // Create web view controller
        webViewController = WebViewController()
        window.contentViewController = webViewController

        // Show window - be very explicit about activation
        window.makeKeyAndOrderFront(nil)
        window.orderFrontRegardless()
        NSApp.activate(ignoringOtherApps: true)

        NSLog("[WebViewTest] Window created and shown")
        NSLog("[WebViewTest] Window is visible: \(window.isVisible)")
        NSLog("[WebViewTest] Window frame: \(window.frame)")

        // Load a test page
        webViewController.loadURL("https://www.google.com")
    }

    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }
}
