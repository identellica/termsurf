use cef::*;
use objc2::{Message, rc::Retained};
use objc2_app_kit::{NSView, NSWindow};
use objc2_foundation::NSString;

fn window_from_browser(browser: Option<&mut Browser>) -> Option<Retained<NSWindow>> {
    let view_ptr = browser?.host()?.window_handle().cast::<NSView>();
    let view = unsafe { view_ptr.as_ref()? };
    let view = view.retain();
    view.window()
}

pub fn platform_title_change(browser: Option<&mut Browser>, title: Option<&CefString>) {
    let Some(window) = window_from_browser(browser) else {
        return;
    };

    let title = title.map(CefString::to_string).unwrap_or_default();
    let title = NSString::from_str(&title);
    window.setTitle(&title);
}

pub fn platform_show_window(browser: Option<&mut Browser>) {
    let Some(window) = window_from_browser(browser) else {
        return;
    };

    window.makeKeyAndOrderFront(Some(&window));
}
