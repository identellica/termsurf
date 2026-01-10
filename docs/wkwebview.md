# WKWebView Implementation Details

This document covers WKWebView-specific implementation details, workarounds, and
gotchas in TermSurf's browser pane implementation.

For high-level architecture decisions (why WKWebView, comparison with CEF, etc.),
see [architecture.md](architecture.md).

## Header Injection: Upgrade-Insecure-Requests

### The Problem

Some websites (notably Google) serve different HTML to WKWebView than to Safari,
even with an identical User-Agent string. This results in:

- Simplified/mobile-style layouts (e.g., "Sign in" button on wrong side)
- Missing features or degraded UI
- Wrong color scheme (light mode instead of respecting system dark mode)

### Root Cause

WKWebView doesn't send the `Upgrade-Insecure-Requests: 1` HTTP header that
Safari sends by default. Some sites use this header's absence as a signal to
detect embedded webviews and serve different content.

The `Upgrade-Insecure-Requests` header is part of the
[W3C Upgrade Insecure Requests specification](https://www.w3.org/TR/upgrade-insecure-requests/).
It signals that the client prefers HTTPS and can handle secure connections.

### Why No Built-in Fix?

Apple's `WKWebViewConfiguration` has no property to enable this header.
The closest option, `upgradeKnownHostsToHTTPS`, does something different—it
automatically converts HTTP URLs to HTTPS but doesn't send the header.

This is a known limitation. See [Open Radar rdar://50057283](https://openradar.appspot.com/50057283):
"WKWebView does not support custom headers on outgoing requests."

### Our Solution

We intercept navigation requests via `WKNavigationDelegate` and inject the header:

```swift
func webView(
  _ webView: WKWebView,
  decidePolicyFor navigationAction: WKNavigationAction,
  decisionHandler: @escaping (WKNavigationActionPolicy) -> Void
) {
  guard let url = navigationAction.request.url,
    (url.scheme == "http" || url.scheme == "https")
  else {
    decisionHandler(.allow)
    return
  }

  // If header already present, allow
  if navigationAction.request.value(forHTTPHeaderField: "Upgrade-Insecure-Requests") != nil {
    decisionHandler(.allow)
    return
  }

  // Cancel and reload with header
  decisionHandler(.cancel)
  var modifiedRequest = navigationAction.request
  modifiedRequest.setValue("1", forHTTPHeaderField: "Upgrade-Insecure-Requests")
  webView.load(modifiedRequest)
}
```

**Location:** `WebViewOverlay.swift`, in the `WKNavigationDelegate` section.

### Limitations

This approach only works for top-level navigation requests. It does **not** cover:

- XHR/fetch requests from JavaScript
- Subresource requests (images, scripts, etc.)

For most sites, fixing the initial page load is sufficient since server-side
detection typically happens on the main document request.

## User-Agent String

We set a Safari User-Agent to avoid mobile/simplified layouts:

```swift
webView.customUserAgent =
  "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.2 Safari/605.1.15"
```

This matches what Safari sends and prevents sites from detecting "embedded
webview" patterns in the default WKWebView User-Agent.

**Note:** The version number (18.2) should be updated periodically to match
current Safari versions, though this is not critical for functionality.

## Other WKWebView Behaviors

### Console Capture

WKWebView has no native API for capturing console output. We inject JavaScript
at document start to override `console.log`, `console.error`, etc., and route
messages through `WKScriptMessageHandler`. See [console.md](console.md).

### Developer Tools

Safari Web Inspector works with WKWebView when `developerExtrasEnabled` is set:

```swift
config.preferences.setValue(true, forKey: "developerExtrasEnabled")
```

Access via cmd+alt+i in browse mode.

### Session Isolation

WKWebView supports session isolation via `WKWebsiteDataStore`:

- **Incognito:** `WKWebsiteDataStore.nonPersistent()` - no data persisted
- **Profiles:** `WKWebsiteDataStore(forIdentifier: UUID)` (macOS 14+) - isolated storage per profile

### target="_blank" Links

WKWebView doesn't handle `target="_blank"` links by default—they're silently
ignored. We implement `WKUIDelegate.webView(_:createWebViewWith:for:windowFeatures:)`
to load these in the same webview. See [target-blank.md](target-blank.md).

## References

- [WKWebViewConfiguration Documentation](https://developer.apple.com/documentation/webkit/wkwebviewconfiguration)
- [Open Radar: Custom Headers](https://openradar.appspot.com/50057283)
- [W3C Upgrade Insecure Requests](https://www.w3.org/TR/upgrade-insecure-requests/)
- [Hacking with Swift: WKWebView Guide](https://www.hackingwithswift.com/articles/112/the-ultimate-guide-to-wkwebview)
