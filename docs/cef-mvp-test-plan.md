# CEF MVP Test Plan

This document outlines a comprehensive testing strategy for the CEF browser integration in TermSurf 2.0.

## Overview

The implementation spans 8 phases with code changes across multiple files. Given the complexity of CEF integration (multi-process architecture, GPU texture sharing, input handling), there are many potential failure points.

### Potential Failure Categories

1. **Build/Compilation** - Feature flags, missing imports, type mismatches
2. **CEF Process** - Subprocess spawning, framework loading, IPC
3. **Rendering** - Texture creation, GPU sharing, display
4. **Input** - Key code conversion, modifier handling, event routing
5. **Lifecycle** - Browser creation, close, cleanup
6. **Integration** - WezTerm pane system, overlays, notifications

---

# Epic 1: Automated Testing

All tests in this epic can run without human intervention. They verify correctness at the code level before we attempt manual testing.

## Phase 1.1: Build Verification

**Objective:** Ensure the code compiles without errors or warnings.

### Tests

```bash
# Test 1.1.1: Build without CEF feature (baseline)
cargo build --release

# Test 1.1.2: Build with CEF feature
cargo build --release --features cef

# Test 1.1.3: Check for warnings
cargo build --release --features cef 2>&1 | grep -E "^warning:"

# Test 1.1.4: Run clippy for static analysis
cargo clippy --features cef -- -D warnings
```

### Expected Results
- All builds succeed with exit code 0
- No warnings (or only known/acceptable warnings)
- No clippy errors

### Debug Section

**If build fails:**
1. Check the error message for the specific file and line
2. Common issues:
   - Missing imports: Add `use` statements
   - Type mismatches: Check CEF API types vs WezTerm types
   - Feature flag issues: Ensure `#[cfg(feature = "cef")]` is correct
3. Read the failing file and surrounding context
4. Check if the issue is in our code or a dependency

**If warnings appear:**
1. Unused variables: Either use them or prefix with `_`
2. Dead code: Remove or gate with `#[cfg(feature = "cef")]`
3. Deprecated APIs: Update to current API

---

## Phase 1.2: Unit Tests for Keyboard Conversion

**Objective:** Verify keyboard code conversion functions work correctly.

### Setup

Create a test module in `wezterm-gui/src/cef/mod.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ::window::KeyCode;
    use ::window::Modifiers;

    #[test]
    fn test_keycode_to_windows_vk_letters() {
        // Lowercase letters should map to uppercase VK codes
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('a')), 0x41);
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('z')), 0x5A);
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('A')), 0x41);
    }

    #[test]
    fn test_keycode_to_windows_vk_numbers() {
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('0')), 0x30);
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('9')), 0x39);
    }

    #[test]
    fn test_keycode_to_windows_vk_special() {
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('\r')), 0x0D); // Enter
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('\n')), 0x0D); // Enter
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('\t')), 0x09); // Tab
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('\u{08}')), 0x08); // Backspace
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('\u{7f}')), 0x2E); // Delete
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char('\u{1b}')), 0x1B); // Escape
        assert_eq!(keycode_to_windows_vk(&KeyCode::Char(' ')), 0x20); // Space
    }

    #[test]
    fn test_keycode_to_windows_vk_arrows() {
        assert_eq!(keycode_to_windows_vk(&KeyCode::LeftArrow), 0x25);
        assert_eq!(keycode_to_windows_vk(&KeyCode::UpArrow), 0x26);
        assert_eq!(keycode_to_windows_vk(&KeyCode::RightArrow), 0x27);
        assert_eq!(keycode_to_windows_vk(&KeyCode::DownArrow), 0x28);
    }

    #[test]
    fn test_keycode_to_windows_vk_function_keys() {
        assert_eq!(keycode_to_windows_vk(&KeyCode::Function(1)), 0x70);  // F1
        assert_eq!(keycode_to_windows_vk(&KeyCode::Function(12)), 0x7B); // F12
    }

    #[test]
    fn test_modifiers_to_cef_flags() {
        assert_eq!(modifiers_to_cef_flags(Modifiers::NONE), 0);
        assert_eq!(modifiers_to_cef_flags(Modifiers::SHIFT), EVENTFLAG_SHIFT_DOWN);
        assert_eq!(modifiers_to_cef_flags(Modifiers::CTRL), EVENTFLAG_CONTROL_DOWN);
        assert_eq!(modifiers_to_cef_flags(Modifiers::ALT), EVENTFLAG_ALT_DOWN);
        assert_eq!(modifiers_to_cef_flags(Modifiers::SUPER), EVENTFLAG_COMMAND_DOWN);

        // Combined modifiers
        let ctrl_shift = Modifiers::CTRL | Modifiers::SHIFT;
        assert_eq!(
            modifiers_to_cef_flags(ctrl_shift),
            EVENTFLAG_CONTROL_DOWN | EVENTFLAG_SHIFT_DOWN
        );
    }

    #[test]
    fn test_keycode_to_native_special() {
        // macOS native key codes
        assert_eq!(keycode_to_native(&KeyCode::Char('\r')), 0x24); // kVK_Return
        assert_eq!(keycode_to_native(&KeyCode::Char('\t')), 0x30); // kVK_Tab
        assert_eq!(keycode_to_native(&KeyCode::Char('\u{1b}')), 0x35); // kVK_Escape
        assert_eq!(keycode_to_native(&KeyCode::Char(' ')), 0x31); // kVK_Space
    }
}
```

### Tests

```bash
# Test 1.2.1: Run unit tests
cargo test --features cef -p wezterm-gui cef::tests

# Test 1.2.2: Run with verbose output
cargo test --features cef -p wezterm-gui cef::tests -- --nocapture
```

### Expected Results
- All tests pass
- Key code mappings are correct for letters, numbers, special keys, arrows, function keys
- Modifier flags combine correctly

### Debug Section

**If tests fail:**
1. Check which specific assertion failed
2. For VK codes: Reference https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
3. For macOS native codes: Reference Carbon HIToolbox/Events.h
4. Compare with cef-rs osr example implementation
5. Check if WezTerm's KeyCode enum has changed

**Common issues:**
- Off-by-one in function key mapping (F1 = 0x70, not 0x71)
- Case sensitivity in letter handling
- Character code points (Escape is `\u{1b}`, not `\x1b` in pattern matching)

---

## Phase 1.3: Integration Tests - Notification System

**Objective:** Verify the WebClosed notification flows through the system correctly.

### Setup

Create test in `mux/src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_closed_notification_serializable() {
        // Verify WebClosed can be created and matched
        let notif = MuxNotification::WebClosed { pane_id: 42 };
        match notif {
            MuxNotification::WebClosed { pane_id } => {
                assert_eq!(pane_id, 42);
            }
            _ => panic!("Wrong notification type"),
        }
    }
}
```

Create test in `codec/src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_closed_pdu_roundtrip() {
        let original = WebClosed { pane_id: 123 };

        // Serialize
        let encoded = rmp_serde::to_vec(&original).unwrap();

        // Deserialize
        let decoded: WebClosed = rmp_serde::from_slice(&encoded).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_web_closed_pdu_id() {
        // Verify the PDU ID is assigned and doesn't conflict
        // This is a compile-time check mostly
        let pdu = Pdu::WebClosed(WebClosed { pane_id: 1 });
        match pdu {
            Pdu::WebClosed(wc) => assert_eq!(wc.pane_id, 1),
            _ => panic!("Wrong PDU type"),
        }
    }
}
```

### Tests

```bash
# Test 1.3.1: Run mux tests
cargo test --features cef -p mux

# Test 1.3.2: Run codec tests
cargo test --features cef -p codec
```

### Expected Results
- WebClosed notification can be created and pattern matched
- WebClosed PDU serializes and deserializes correctly
- No PDU ID conflicts

### Debug Section

**If serialization fails:**
1. Check derive macros on WebClosed struct
2. Verify all fields are serializable
3. Check for version mismatches in serde/rmp-serde

**If PDU ID conflicts:**
1. Check `pdu!` macro in codec/src/lib.rs
2. Ensure ID 65 isn't used by another PDU
3. Look for duplicate IDs in the macro invocation

---

## Phase 1.4: Feature Flag Verification

**Objective:** Ensure CEF code is properly gated and doesn't break non-CEF builds.

### Tests

```bash
# Test 1.4.1: Build without CEF feature
cargo build --release
# Should succeed without any CEF code being compiled

# Test 1.4.2: Run all tests without CEF
cargo test
# Should pass without CEF-specific tests

# Test 1.4.3: Check that CEF imports are gated
grep -rn "use cef" wezterm-gui/src/ | grep -v "#\[cfg"
# Should return nothing or only properly gated uses

# Test 1.4.4: Check that browser_states usage is gated
grep -rn "browser_states" wezterm-gui/src/ | head -20
# Verify all uses are within #[cfg(feature = "cef")] blocks
```

### Expected Results
- Non-CEF build works identically to before our changes
- No CEF code leaks into non-CEF builds
- All CEF-specific code is properly feature-gated

### Debug Section

**If non-CEF build fails:**
1. Find the error location
2. Add `#[cfg(feature = "cef")]` attribute
3. For struct fields, use `#[cfg(feature = "cef")]` on the field
4. For match arms, use `#[cfg(feature = "cef")]` on the arm

**If CEF imports leak:**
1. Wrap import with `#[cfg(feature = "cef")]`
2. Consider reorganizing code into a separate module

---

## Phase 1.5: Memory Safety Analysis

**Objective:** Detect potential memory issues before runtime.

### Tests

```bash
# Test 1.5.1: Check for potential memory leaks with clippy
cargo clippy --features cef -- -W clippy::mem_forget

# Test 1.5.2: Check for unsafe code
grep -rn "unsafe" wezterm-gui/src/cef/ wezterm-gui/src/termwindow/

# Test 1.5.3: Check for unwrap() calls that could panic
grep -rn "\.unwrap()" wezterm-gui/src/cef/
grep -rn "\.expect(" wezterm-gui/src/cef/

# Test 1.5.4: Check for RefCell borrow patterns
grep -rn "\.borrow(" wezterm-gui/src/cef/
grep -rn "\.borrow_mut(" wezterm-gui/src/cef/
```

### Expected Results
- No unsafe code in our additions (unless absolutely necessary)
- unwrap()/expect() only used where failure is truly impossible
- RefCell borrows are short-lived and don't overlap

### Debug Section

**If unsafe code found:**
1. Document why it's necessary
2. Consider safe alternatives
3. If keeping, add `// SAFETY:` comment

**If unwrap() found:**
1. Replace with `if let Some()` or `match`
2. Or use `.ok()?` for fallible returns
3. Or use `.unwrap_or_default()` where appropriate

**If RefCell issues:**
1. Look for nested borrows (borrow inside borrow)
2. Check for borrow across await points
3. Consider restructuring to avoid

---

## Phase 1.6: Automated Runtime Test (Headless)

**Objective:** Test CEF initialization without a display.

### Setup

This requires a test binary that initializes CEF and immediately exits.

Create `wezterm-gui/tests/cef_init_test.rs`:

```rust
#[cfg(feature = "cef")]
#[test]
fn test_cef_can_initialize_types() {
    // Test that CEF types can be instantiated
    // This doesn't require a running CEF process

    use wezterm_gui::cef::{keycode_to_windows_vk, modifiers_to_cef_flags};
    use window::KeyCode;
    use window::Modifiers;

    // Basic sanity checks
    let vk = keycode_to_windows_vk(&KeyCode::Char('a'));
    assert!(vk > 0);

    let flags = modifiers_to_cef_flags(Modifiers::CTRL);
    assert!(flags > 0);
}
```

### Tests

```bash
# Test 1.6.1: Run integration test
cargo test --features cef -p wezterm-gui --test cef_init_test

# Test 1.6.2: Check CEF binary exists after build
ls -la target/release/wezterm-gui
# Should show executable

# Test 1.6.3: Run with --help to ensure basic startup works
./target/release/wezterm --help
```

### Expected Results
- Tests pass
- Binary is created
- Help text displays (proves basic initialization works)

### Debug Section

**If initialization fails:**
1. Check CEF framework path
2. Verify CEF subprocess helper is built
3. Check environment variables
4. Review CEF log output

---

# Epic 2: Manual Testing

These tests require human observation and interaction. They verify the actual user experience.

## Phase 2.1: Application Launch

**Objective:** Verify the application starts correctly with CEF support.

### Prerequisites
- Built the app with `cargo build --release --features cef`
- CEF framework is bundled (or available in expected location)

### Tests

| ID | Test | Expected Result |
|----|------|-----------------|
| 2.1.1 | Launch wezterm normally | Terminal opens, no errors in console |
| 2.1.2 | Check logs for CEF initialization | Should see "CEF initialized" or similar |
| 2.1.3 | Open a normal terminal tab | Works as before, no regressions |
| 2.1.4 | Type in terminal | Input works normally |

### Test Commands

```bash
# Launch with verbose logging
WEZTERM_LOG=info ./target/release/wezterm

# Check for CEF-related log messages
WEZTERM_LOG=debug ./target/release/wezterm 2>&1 | grep -i cef
```

### Debug Section

**If app crashes on launch:**
1. Run with `RUST_BACKTRACE=1` for stack trace
2. Check if CEF subprocess is the issue (add logging to main.rs subprocess check)
3. Verify CEF framework path
4. Check macOS security permissions (Gatekeeper)

**If CEF doesn't initialize:**
1. Check CEF_FRAMEWORK_DIR environment variable
2. Verify framework exists at expected path
3. Look for CEF error messages in stderr
4. Try running cef-osr example to verify CEF works independently

**If terminal is broken:**
1. Build without CEF feature to verify baseline
2. Check for accidental changes outside CEF blocks
3. Review recent commits for unintended changes

---

## Phase 2.2: Browser Creation (web-open)

**Objective:** Verify the web-open command creates a browser pane.

### Tests

| ID | Test | Expected Result |
|----|------|-----------------|
| 2.2.1 | Run `web-open https://example.com` | Browser pane appears |
| 2.2.2 | Check pane shows content | Should see Example Domain page |
| 2.2.3 | Run web-open in new tab | New tab with browser |
| 2.2.4 | Run multiple web-open commands | Multiple browser panes work |
| 2.2.5 | Open HTTPS site | SSL works |
| 2.2.6 | Open HTTP site | Redirect/content works |

### Test Commands

```bash
# From within wezterm
wezterm cli web-open https://example.com
wezterm cli web-open https://www.google.com
wezterm cli web-open https://github.com
```

### Debug Section

**If nothing appears:**
1. Check logs for browser creation message
2. Verify BrowserState is being created
3. Check if texture is being created
4. Add debug logging to `handle_web_open` or equivalent

**If black screen:**
1. Check if OnPaint is being called (add logging)
2. Verify texture format matches wgpu expectations
3. Check IOSurface creation on macOS
4. Compare with cef-osr example

**If content wrong/garbled:**
1. Check texture dimensions vs pane dimensions
2. Verify pixel format (BGRA vs RGBA)
3. Check for coordinate system issues (flipped Y)

**If crashes:**
1. Check CEF process communication
2. Verify browser host is valid
3. Check for null pointer access
4. Review RefCell borrow patterns

---

## Phase 2.3: Browser Rendering

**Objective:** Verify the browser renders correctly and updates.

### Tests

| ID | Test | Expected Result |
|----|------|-----------------|
| 2.3.1 | Static page renders | Content visible, styled correctly |
| 2.3.2 | Scroll page | Content scrolls smoothly |
| 2.3.3 | Animated content | Animations play |
| 2.3.4 | Video playback | Video plays (if CEF supports) |
| 2.3.5 | CSS transitions | Smooth transitions |
| 2.3.6 | Canvas/WebGL | Graphics render |

### Test URLs

```bash
# Static content
wezterm cli web-open https://example.com

# Scrollable content
wezterm cli web-open https://en.wikipedia.org/wiki/Terminal_emulator

# Animation
wezterm cli web-open https://css-tricks.com/almanac/properties/a/animation/

# Video (test page)
wezterm cli web-open https://www.w3schools.com/html/html5_video.asp
```

### Debug Section

**If rendering stutters:**
1. Check frame rate (add FPS counter)
2. Verify vsync settings
3. Check if OnPaint is called too frequently/infrequently
4. Profile GPU usage

**If colors wrong:**
1. Check color space conversion
2. Verify BGRA vs RGBA handling
3. Check if premultiplied alpha is handled correctly

**If content clipped:**
1. Verify browser size matches pane size
2. Check viewport settings
3. Verify texture coordinates in shader

---

## Phase 2.4: Resize Handling

**Objective:** Verify browser resizes correctly with the pane.

### Tests

| ID | Test | Expected Result |
|----|------|-----------------|
| 2.4.1 | Resize window | Browser content resizes |
| 2.4.2 | Split pane horizontally | Both panes correct size |
| 2.4.3 | Split pane vertically | Both panes correct size |
| 2.4.4 | Maximize window | Browser fills space |
| 2.4.5 | Resize rapidly | No crashes or artifacts |
| 2.4.6 | Very small size | Handles gracefully |
| 2.4.7 | Very large size | Handles gracefully |

### Debug Section

**If resize doesn't work:**
1. Check if resize notification reaches browser
2. Verify WasResized() is being called
3. Check if new texture is created
4. Log size changes

**If resize causes crash:**
1. Check for race conditions
2. Verify old texture is properly released
3. Check for division by zero (zero-size pane)
4. Add guards for minimum size

**If content stretched/squished:**
1. Check aspect ratio handling
2. Verify browser viewport matches texture size
3. Check for off-by-one errors in dimensions

---

## Phase 2.5: Keyboard Input

**Objective:** Verify all keyboard input works correctly.

### Tests

| ID | Test | Expected Result |
|----|------|-----------------|
| 2.5.1 | Type letters a-z | Letters appear in text field |
| 2.5.2 | Type SHIFT+letters | Uppercase letters |
| 2.5.3 | Type numbers 0-9 | Numbers appear |
| 2.5.4 | Type symbols (!@#$) | Symbols appear |
| 2.5.5 | Press Enter | Form submits / newline |
| 2.5.6 | Press Tab | Focus moves |
| 2.5.7 | Press Backspace | Character deleted |
| 2.5.8 | Press Delete | Character deleted forward |
| 2.5.9 | Press Escape | Modal closes / action cancelled |
| 2.5.10 | Arrow keys | Cursor/selection moves |
| 2.5.11 | Home/End | Cursor jumps |
| 2.5.12 | Page Up/Down | Page scrolls |
| 2.5.13 | Cmd+A (select all) | Text selected |
| 2.5.14 | Cmd+C (copy) | Text copied |
| 2.5.15 | Cmd+V (paste) | Text pasted |
| 2.5.16 | Cmd+Z (undo) | Action undone |
| 2.5.17 | F1-F12 keys | Function key actions |

### Test URL

```bash
# Use a text input test page
wezterm cli web-open https://www.google.com
# Type in search box

# Or use a dedicated input tester
wezterm cli web-open https://keyboardtester.co
```

### Debug Section

**If keys don't register:**
1. Add logging to send_key_event
2. Check if key event reaches CEF
3. Verify key down/up events both sent
4. Check if CHAR event is sent for printable keys

**If wrong characters appear:**
1. Check keycode_to_windows_vk mapping
2. Verify modifier handling
3. Check character vs keycode in CHAR event
4. Compare with cef-osr example

**If modifiers don't work:**
1. Check modifiers_to_cef_flags
2. Verify modifier state is passed
3. Check for modifier key up/down events
4. Test modifier combinations

**If special keys fail:**
1. Check VK code for that specific key
2. Verify native key code mapping
3. Check if key is being consumed elsewhere

---

## Phase 2.6: Mouse Input (if implemented)

**Objective:** Verify mouse input works correctly.

> Note: Mouse input may not be fully implemented in MVP. Skip if not applicable.

### Tests

| ID | Test | Expected Result |
|----|------|-----------------|
| 2.6.1 | Click link | Navigates to link |
| 2.6.2 | Click button | Button activates |
| 2.6.3 | Click text field | Field focuses |
| 2.6.4 | Right-click | Context menu (or suppressed) |
| 2.6.5 | Double-click word | Word selected |
| 2.6.6 | Drag to select text | Text selected |
| 2.6.7 | Scroll wheel | Page scrolls |
| 2.6.8 | Hover | Hover styles apply |

### Debug Section

**If clicks don't work:**
1. Check mouse event coordinates
2. Verify coordinate transformation (screen to browser)
3. Check if click events reach CEF
4. Log mouse down/up events

**If coordinates wrong:**
1. Check pane offset calculation
2. Verify DPI scaling
3. Check for inverted Y axis

---

## Phase 2.7: Browser Close (Ctrl+C)

**Objective:** Verify Ctrl+C closes the browser and returns to terminal.

### Tests

| ID | Test | Expected Result |
|----|------|-----------------|
| 2.7.1 | Press Ctrl+C in browser | Browser closes |
| 2.7.2 | Pane returns to terminal | Terminal prompt visible |
| 2.7.3 | Can type in terminal | Input works |
| 2.7.4 | No memory leak | Memory stable after close |
| 2.7.5 | Close multiple browsers | All close correctly |
| 2.7.6 | Ctrl+C during page load | Cancels and closes |

### Test Sequence

```bash
# 1. Open browser
wezterm cli web-open https://example.com

# 2. Wait for page to load

# 3. Press Ctrl+C

# 4. Verify terminal returns

# 5. Type 'echo "test"' to verify terminal works
```

### Debug Section

**If Ctrl+C not detected:**
1. Check key_event_impl intercept logic
2. Verify modifiers.contains(CTRL) check
3. Check KeyCode::Char('c') matching
4. Add logging before the check

**If browser doesn't close:**
1. Check close_browser_for_pane is called
2. Verify browser.host() returns valid host
3. Check close_browser(1) call
4. Look for CEF errors

**If pane doesn't return:**
1. Check WebClosed notification flow
2. Verify client receives notification
3. Check pane replacement logic

**If memory leaks:**
1. Use Activity Monitor to watch memory
2. Check if browser_states entry is removed
3. Verify CEF ref counting cleanup
4. Check for retained textures

---

## Phase 2.8: Overlay Behavior

**Objective:** Verify browser hides correctly when overlays appear.

### Tests

| ID | Test | Expected Result |
|----|------|-----------------|
| 2.8.1 | Open launcher (Cmd+Shift+L or equivalent) | Browser hidden, launcher visible |
| 2.8.2 | Close launcher | Browser visible again |
| 2.8.3 | Open search (Cmd+F) | Browser hidden, search visible |
| 2.8.4 | Close search | Browser visible again |
| 2.8.5 | Switch tabs while overlay open | Overlay stays, correct behavior |
| 2.8.6 | Rapid overlay toggle | No crashes or artifacts |

### Debug Section

**If browser shows through overlay:**
1. Check render order in paint_pane
2. Verify overlay pane_id differs from browser pane_id
3. Check if browser_states lookup is failing to not-match
4. Add logging to render path

**If browser doesn't return after overlay:**
1. Check if browser_states entry persists
2. Verify pane_id mapping is correct
3. Check overlay close handling

**If crashes during overlay:**
1. Check for concurrent access issues
2. Verify RefCell borrows don't overlap
3. Check pane lifecycle events

---

## Phase 2.9: Edge Cases and Stress Tests

**Objective:** Find crashes and edge cases.

### Tests

| ID | Test | Expected Result |
|----|------|-----------------|
| 2.9.1 | Open 10 browser panes | All work |
| 2.9.2 | Close all quickly | No crashes |
| 2.9.3 | Open same URL twice | Both work independently |
| 2.9.4 | Very long URL | Handles or errors gracefully |
| 2.9.5 | Invalid URL | Error message, no crash |
| 2.9.6 | Offline network | Error page or message |
| 2.9.7 | HTTPS cert error | Handles appropriately |
| 2.9.8 | Page with alert() | Alert handled or suppressed |
| 2.9.9 | Page with confirm() | Dialog handled or suppressed |
| 2.9.10 | Page with print() | Print handled or suppressed |
| 2.9.11 | Page opens popup | Popup handled or blocked |
| 2.9.12 | JavaScript infinite loop | Doesn't freeze WezTerm |
| 2.9.13 | Very large page (high memory) | Handles gracefully |

### Debug Section

**For any crash:**
1. Run with `RUST_BACKTRACE=1`
2. Check last log message before crash
3. Try to reproduce with minimal steps
4. Check if crash is in Rust or CEF subprocess

**For hangs:**
1. Check if main thread is blocked
2. Look for deadlocks in RefCell patterns
3. Check CEF message loop
4. Try sending SIGINT and check response

---

## Phase 2.10: Regression Testing

**Objective:** Ensure existing WezTerm functionality still works.

### Tests

| ID | Test | Expected Result |
|----|------|-----------------|
| 2.10.1 | Open new terminal | Works |
| 2.10.2 | Split panes | Works |
| 2.10.3 | Switch between panes | Works |
| 2.10.4 | Run shell commands | Works |
| 2.10.5 | Use vim/nano | Works |
| 2.10.6 | Use tmux | Works |
| 2.10.7 | SSH connection | Works |
| 2.10.8 | Copy/paste in terminal | Works |
| 2.10.9 | Scrollback | Works |
| 2.10.10 | Font rendering | Unchanged |
| 2.10.11 | Configuration changes | Applied correctly |
| 2.10.12 | Tabs | Work |
| 2.10.13 | Window management | Works |

### Debug Section

**For any regression:**
1. Compare with non-CEF build
2. Check if change is in shared code path
3. Review commits for unintended changes
4. Git bisect if necessary

---

# Execution Checklist

## Before Testing

- [ ] Build completed successfully with CEF feature
- [ ] All automated tests pass
- [ ] Test environment prepared
- [ ] Logging enabled for debugging

## Epic 1: Automated

- [ ] Phase 1.1: Build Verification
- [ ] Phase 1.2: Unit Tests for Keyboard Conversion
- [ ] Phase 1.3: Integration Tests - Notification System
- [ ] Phase 1.4: Feature Flag Verification
- [ ] Phase 1.5: Memory Safety Analysis
- [ ] Phase 1.6: Automated Runtime Test

## Epic 2: Manual

- [ ] Phase 2.1: Application Launch
- [ ] Phase 2.2: Browser Creation
- [ ] Phase 2.3: Browser Rendering
- [ ] Phase 2.4: Resize Handling
- [ ] Phase 2.5: Keyboard Input
- [ ] Phase 2.6: Mouse Input (if applicable)
- [ ] Phase 2.7: Browser Close
- [ ] Phase 2.8: Overlay Behavior
- [ ] Phase 2.9: Edge Cases
- [ ] Phase 2.10: Regression Testing

## After Testing

- [ ] All critical issues fixed
- [ ] Test results documented
- [ ] Known issues logged
- [ ] Ready for next phase
