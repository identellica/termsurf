# WKWebView API Audit

This document tracks WKWebView delegate methods and configuration options,
comparing what TermSurf currently implements vs what's available in the full
API.

**Sources:**

- [WKNavigationDelegate](https://developer.apple.com/documentation/webkit/wknavigationdelegate)
- [WKUIDelegate](https://developer.apple.com/documentation/webkit/wkuidelegate)
- [WKDownloadDelegate](https://developer.apple.com/documentation/webkit/wkdownloaddelegate)
- [WKWebViewConfiguration](https://developer.apple.com/documentation/webkit/wkwebviewconfiguration)
- [WebKit GitHub Headers](https://github.com/WebKit/webkit/tree/main/Source/WebKit/UIProcess/API/Cocoa)

---

## Current Implementation Status

### WKNavigationDelegate

| Method                                              | Implemented | Priority | Notes                                     |
| --------------------------------------------------- | ----------- | -------- | ----------------------------------------- |
| `decidePolicyFor:navigationAction:`                 | Yes         | -        | Used for Upgrade-Insecure-Requests header |
| `decidePolicyFor:navigationResponse:`               | No          | Low      | Could inspect response headers            |
| `didStartProvisionalNavigation:`                    | Yes         | -        | URL change notification                   |
| `didReceiveServerRedirectForProvisionalNavigation:` | No          | Low      | Redirect tracking                         |
| `didCommit:`                                        | No          | Low      | Content arriving                          |
| `didFinish:`                                        | Yes         | -        | Navigation complete, focus handling       |
| `didFail:withError:`                                | Yes         | -        | Error logging                             |
| `didFailProvisionalNavigation:withError:`           | Yes         | -        | Error logging                             |
| `didReceiveAuthenticationChallenge:`                | No          | **High** | HTTP Basic Auth, client certs             |
| `webContentProcessDidTerminate:`                    | No          | **High** | Crash recovery                            |
| `shouldAllowDeprecatedTLS:`                         | No          | Medium   | TLS 1.0/1.1 warning                       |
| `navigationAction:didBecomeDownload:`               | No          | **High** | Download handling                         |
| `navigationResponse:didBecomeDownload:`             | No          | **High** | Download handling                         |

### WKUIDelegate

| Method                                         | Implemented | Priority | Notes                    |
| ---------------------------------------------- | ----------- | -------- | ------------------------ |
| `createWebViewWith:for:windowFeatures:`        | Yes         | -        | target="_blank" handling |
| `webViewDidClose:`                             | No          | Medium   | window.close() handling  |
| `runJavaScriptAlertPanelWithMessage:`          | No          | **High** | alert() dialogs          |
| `runJavaScriptConfirmPanelWithMessage:`        | No          | **High** | confirm() dialogs        |
| `runJavaScriptTextInputPanelWithPrompt:`       | No          | **High** | prompt() dialogs         |
| `runOpenPanelWithParameters:`                  | No          | **High** | **File uploads**         |
| `requestMediaCapturePermissionForOrigin:`      | No          | **High** | Camera/mic access        |
| `requestDeviceOrientationAndMotionPermission:` | No          | Low      | Gyroscope access         |
| `contextMenuConfigurationForElement:`          | No          | Low      | macOS uses native menus  |
| `showLockdownModeFirstUseMessage:`             | No          | Low      | Lockdown Mode warning    |

### WKDownloadDelegate

| Method                                        | Implemented | Priority | Notes                          |
| --------------------------------------------- | ----------- | -------- | ------------------------------ |
| `download:decideDestinationUsing:`            | No          | **High** | Required for downloads         |
| `download:willPerformHTTPRedirection:`        | No          | Medium   | Redirect during download       |
| `download:didReceiveAuthenticationChallenge:` | No          | Medium   | Auth during download           |
| `downloadDidFinish:`                          | No          | **High** | Download complete notification |
| `download:didFailWithError:resumeData:`       | No          | **High** | Download failure handling      |

### WKWebViewConfiguration

| Property                                   | Set     | Priority | Notes                   |
| ------------------------------------------ | ------- | -------- | ----------------------- |
| `processPool`                              | Default | Low      | Process sharing         |
| `preferences`                              | Yes     | -        | developerExtrasEnabled  |
| `userContentController`                    | Yes     | -        | Console capture, JS API |
| `websiteDataStore`                         | Yes     | -        | Profile isolation       |
| `suppressesIncrementalRendering`           | No      | Low      | Wait for full load      |
| `applicationNameForUserAgent`              | No      | Low      | Append to UA            |
| `allowsAirPlayForMediaPlayback`            | Default | Low      | AirPlay                 |
| `upgradeKnownHostsToHTTPS`                 | No      | Low      | Auto HTTPS upgrade      |
| `mediaTypesRequiringUserActionForPlayback` | Default | Medium   | Autoplay policy         |
| `defaultWebpagePreferences`                | No      | Low      | Per-page settings       |
| `limitsNavigationsToAppBoundDomains`       | No      | Low      | Domain restriction      |
| `allowsInlinePredictions`                  | Default | Low      | Text predictions        |

### WKWebView Properties

| Property                              | Used    | Priority | Notes                       |
| ------------------------------------- | ------- | -------- | --------------------------- |
| `customUserAgent`                     | Yes     | -        | Safari UA string            |
| `allowsBackForwardNavigationGestures` | No      | Medium   | Swipe navigation            |
| `allowsLinkPreview`                   | Default | Low      | Force Touch preview         |
| `isInspectable`                       | No      | Medium   | Web Inspector (macOS 13.3+) |
| `pageZoom`                            | No      | Medium   | User zoom control           |
| `underPageBackgroundColor`            | No      | Low      | Bounce background           |

---

## Implementation Plan

### Phase 1: Critical Missing Features (High Priority)

These are features users will definitely encounter and expect to work:

1. **JavaScript Dialogs** - `alert()`, `confirm()`, `prompt()`
   - Without these, many sites break silently
   - Implementation: Show native NSAlert/NSPanel

2. **File Uploads** - `<input type="file">`
   - Many sites need this (forms, cloud storage, etc.)
   - Implementation: NSOpenPanel file picker

3. **Downloads**
   - Click-to-download links currently do nothing
   - Implementation: WKDownloadDelegate + save dialog

4. **Authentication Challenges**
   - HTTP Basic Auth, NTLM, client certificates
   - Implementation: NSAlert with username/password fields

5. **Process Crash Recovery**
   - If WebContent process crashes, webview goes blank
   - Implementation: Detect crash, offer reload

6. **Media Permissions**
   - Camera/microphone access requests
   - Implementation: Show permission dialog, remember choice

### Phase 2: Important Improvements (Medium Priority)

1. **window.close() Handling**
   - Pages that call `window.close()` should close the webview

2. **Back/Forward Gestures**
   - Two-finger swipe for history navigation

3. **TLS Deprecation Warnings**
   - Warn when connecting to sites with old TLS

4. **Page Zoom**
   - cmd+/- for zoom control

5. **Web Inspector Toggle**
   - Set `isInspectable = true` for macOS 13.3+

### Phase 3: Nice to Have (Low Priority)

1. Download redirect handling
2. Device orientation permissions
3. Context menu customization
4. Suppress incremental rendering option
5. AirPlay configuration
6. Link preview customization

---

## Implementation Notes

### JavaScript Dialogs

```swift
func webView(_ webView: WKWebView,
             runJavaScriptAlertPanelWithMessage message: String,
             initiatedByFrame frame: WKFrameInfo,
             completionHandler: @escaping () -> Void) {
    let alert = NSAlert()
    alert.messageText = message
    alert.addButton(withTitle: "OK")
    alert.runModal()
    completionHandler()
}
```

### File Upload

```swift
func webView(_ webView: WKWebView,
             runOpenPanelWith parameters: WKOpenPanelParameters,
             initiatedByFrame frame: WKFrameInfo,
             completionHandler: @escaping ([URL]?) -> Void) {
    let panel = NSOpenPanel()
    panel.allowsMultipleSelection = parameters.allowsMultipleSelection
    panel.canChooseDirectories = parameters.allowsDirectories
    panel.begin { response in
        completionHandler(response == .OK ? panel.urls : nil)
    }
}
```

### Downloads

```swift
// In WKNavigationDelegate
func webView(_ webView: WKWebView,
             navigationResponse: WKNavigationResponse,
             didBecomeDownload download: WKDownload) {
    download.delegate = self
}

// WKDownloadDelegate
func download(_ download: WKDownload,
              decideDestinationUsing response: URLResponse,
              suggestedFilename: String,
              completionHandler: @escaping (URL?) -> Void) {
    let panel = NSSavePanel()
    panel.nameFieldStringValue = suggestedFilename
    panel.begin { response in
        completionHandler(response == .OK ? panel.url : nil)
    }
}
```

---

## References

- [The Ultimate Guide to WKWebView](https://www.hackingwithswift.com/articles/112/the-ultimate-guide-to-wkwebview)
- [WebKit Source Headers](https://github.com/WebKit/webkit/tree/main/Source/WebKit/UIProcess/API/Cocoa)
- [WKWebView improvements in iOS 15](https://nemecek.be/blog/111/wkwebview-improvements-in-ios-15)
