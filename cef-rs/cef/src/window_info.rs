use crate::{sys::cef_window_handle_t, *};
#[cfg(target_os = "windows")]
use windows_sys::Win32::UI::WindowsAndMessaging::*;

impl WindowInfo {
    /// Create the browser as a child window.
    pub fn set_as_child(self, parent: cef_window_handle_t, bounds: &Rect) -> Self {
        Self {
            #[cfg(target_os = "windows")]
            style: WS_CHILD | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | WS_TABSTOP | WS_VISIBLE,
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            parent_window: parent,
            #[cfg(target_os = "macos")]
            parent_view: parent,
            bounds: bounds.clone(),
            #[cfg(target_os = "macos")]
            hidden: 0,
            ..self
        }
    }

    /// Create the browser as a popup window.
    #[cfg(target_os = "windows")]
    pub fn set_as_popup(self, parent: cef_window_handle_t, title: &str) -> Self {
        Self {
            window_name: CefString::from(title),
            parent_window: parent,
            style: WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | WS_VISIBLE,
            bounds: Rect {
                x: CW_USEDEFAULT,
                y: CW_USEDEFAULT,
                width: CW_USEDEFAULT,
                height: CW_USEDEFAULT,
            },
            ..self
        }
    }

    /// Create the browser using windowless (off-screen) rendering. No window
    /// will be created for the browser and all rendering will occur via the
    /// CefRenderHandler interface. The |parent| value will be used to identify
    /// monitor info and to act as the parent window for dialogs, context menus,
    /// etc. If |parent| is not provided then the main screen monitor will be used
    /// and some functionality that requires a parent window may not function
    /// correctly. In order to create windowless browsers the
    /// CefSettings.windowless_rendering_enabled value must be set to true.
    /// Transparent painting is enabled by default but can be disabled by setting
    /// CefBrowserSettings.background_color to an opaque value.
    pub fn set_as_windowless(self, parent: cef_window_handle_t) -> Self {
        Self {
            windowless_rendering_enabled: 1,
            #[cfg(any(target_os = "linux", target_os = "windows"))]
            parent_window: parent,
            #[cfg(target_os = "macos")]
            parent_view: parent,
            runtime_style: RuntimeStyle::ALLOY,
            ..self
        }
    }
}
