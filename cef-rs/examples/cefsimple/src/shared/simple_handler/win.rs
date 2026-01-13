use cef::*;
use std::iter;
use windows_sys::Win32::{Foundation::HWND, UI::WindowsAndMessaging::*};

fn window_from_browser(browser: Option<&mut Browser>) -> Option<HWND> {
    let window = browser?.host()?.window_handle().0;
    Some(window.cast())
}

pub fn platform_title_change(browser: Option<&mut Browser>, title: Option<&CefString>) {
    let Some(window) = window_from_browser(browser) else {
        return;
    };

    let title = title.map(CefString::to_string).unwrap_or_default();
    let title: Vec<_> = title.encode_utf16().chain(iter::once(0)).collect();
    unsafe { SetWindowTextW(window, title.as_ptr()) };
}
