#![allow(
    dead_code,
    improper_ctypes_definitions,
    non_camel_case_types,
    unused_variables
)]
use crate::rc::{ConvertParam, ConvertReturnValue, Rc, RcImpl, RefGuard, WrapParamRef};
use cef_dll_sys::*;

/// Perform the conversion between CEF and Rust types in field initializers.
fn init_array_field<T, U, const N: usize>(mut value: [U; N]) -> [T; N]
where
    T: Sized,
    U: Sized + Into<T>,
{
    std::array::from_fn(move |i| {
        let mut elem = unsafe { std::mem::zeroed() };
        std::mem::swap(&mut value[i], &mut elem);
        elem.into()
    })
}

/// See [cef_string_wide_t] for more documentation.
pub type CefStringUserfreeWide = *mut CefStringWide;

/// See [cef_string_utf8_t] for more documentation.
pub type CefStringUserfreeUtf8 = *mut CefStringUtf8;

/// See [cef_string_utf16_t] for more documentation.
pub type CefStringUserfreeUtf16 = *mut CefStringUtf16;

/// See [char16_t] for more documentation.
pub type Char = char16_t;

/// See [cef_string_userfree_utf16_t] for more documentation.
pub type CefStringUserfree = *mut CefStringUtf16;

/// See [cef_string_utf16_t] for more documentation.
pub type CefString = CefStringUtf16;

/// See [HCURSOR] for more documentation.
pub type CursorHandle = HCURSOR;

/// See [MSG] for more documentation.
pub type EventHandle = *mut MSG;

/// See [HWND] for more documentation.
pub type WindowHandle = HWND;

/// See [HANDLE] for more documentation.
pub type SharedTextureHandle = HANDLE;

/// See [u32] for more documentation.
pub type Color = u32;

/// See [_cef_string_wide_t] for more documentation.
pub use crate::string::CefStringWide;

/// See [_cef_string_utf8_t] for more documentation.
pub use crate::string::CefStringUtf8;

/// See [_cef_string_utf16_t] for more documentation.
pub use crate::string::CefStringUtf16;

/// See [_cef_string_list_t] for more documentation.
pub use crate::string::CefStringList;

/// See [_cef_string_map_t] for more documentation.
pub use crate::string::CefStringMap;

/// See [_cef_string_multimap_t] for more documentation.
pub use crate::string::CefStringMultimap;

/// See [_cef_basetime_t] for more documentation.
#[derive(Clone)]
pub struct Basetime {
    pub val: i64,
}
impl From<_cef_basetime_t> for Basetime {
    fn from(value: _cef_basetime_t) -> Self {
        Self {
            val: value.val.into(),
        }
    }
}
impl Into<_cef_basetime_t> for Basetime {
    fn into(self) -> _cef_basetime_t {
        _cef_basetime_t {
            val: self.val.into(),
        }
    }
}
impl Default for Basetime {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_time_t] for more documentation.
#[derive(Clone)]
pub struct Time {
    pub year: ::std::os::raw::c_int,
    pub month: ::std::os::raw::c_int,
    pub day_of_week: ::std::os::raw::c_int,
    pub day_of_month: ::std::os::raw::c_int,
    pub hour: ::std::os::raw::c_int,
    pub minute: ::std::os::raw::c_int,
    pub second: ::std::os::raw::c_int,
    pub millisecond: ::std::os::raw::c_int,
}
impl From<_cef_time_t> for Time {
    fn from(value: _cef_time_t) -> Self {
        Self {
            year: value.year.into(),
            month: value.month.into(),
            day_of_week: value.day_of_week.into(),
            day_of_month: value.day_of_month.into(),
            hour: value.hour.into(),
            minute: value.minute.into(),
            second: value.second.into(),
            millisecond: value.millisecond.into(),
        }
    }
}
impl Into<_cef_time_t> for Time {
    fn into(self) -> _cef_time_t {
        _cef_time_t {
            year: self.year.into(),
            month: self.month.into(),
            day_of_week: self.day_of_week.into(),
            day_of_month: self.day_of_month.into(),
            hour: self.hour.into(),
            minute: self.minute.into(),
            second: self.second.into(),
            millisecond: self.millisecond.into(),
        }
    }
}
impl Default for Time {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_point_t] for more documentation.
#[derive(Clone)]
pub struct Point {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
impl From<_cef_point_t> for Point {
    fn from(value: _cef_point_t) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
        }
    }
}
impl Into<_cef_point_t> for Point {
    fn into(self) -> _cef_point_t {
        _cef_point_t {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}
impl Default for Point {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_rect_t] for more documentation.
#[derive(Clone)]
pub struct Rect {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}
impl From<_cef_rect_t> for Rect {
    fn from(value: _cef_rect_t) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}
impl Into<_cef_rect_t> for Rect {
    fn into(self) -> _cef_rect_t {
        _cef_rect_t {
            x: self.x.into(),
            y: self.y.into(),
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}
impl Default for Rect {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_size_t] for more documentation.
#[derive(Clone)]
pub struct Size {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}
impl From<_cef_size_t> for Size {
    fn from(value: _cef_size_t) -> Self {
        Self {
            width: value.width.into(),
            height: value.height.into(),
        }
    }
}
impl Into<_cef_size_t> for Size {
    fn into(self) -> _cef_size_t {
        _cef_size_t {
            width: self.width.into(),
            height: self.height.into(),
        }
    }
}
impl Default for Size {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_insets_t] for more documentation.
#[derive(Clone)]
pub struct Insets {
    pub top: ::std::os::raw::c_int,
    pub left: ::std::os::raw::c_int,
    pub bottom: ::std::os::raw::c_int,
    pub right: ::std::os::raw::c_int,
}
impl From<_cef_insets_t> for Insets {
    fn from(value: _cef_insets_t) -> Self {
        Self {
            top: value.top.into(),
            left: value.left.into(),
            bottom: value.bottom.into(),
            right: value.right.into(),
        }
    }
}
impl Into<_cef_insets_t> for Insets {
    fn into(self) -> _cef_insets_t {
        _cef_insets_t {
            top: self.top.into(),
            left: self.left.into(),
            bottom: self.bottom.into(),
            right: self.right.into(),
        }
    }
}
impl Default for Insets {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_accelerated_paint_info_common_t] for more documentation.
#[derive(Clone)]
pub struct AcceleratedPaintInfoCommon {
    pub size: usize,
    pub timestamp: u64,
    pub coded_size: Size,
    pub visible_rect: Rect,
    pub content_rect: Rect,
    pub source_size: Size,
    pub capture_update_rect: Rect,
    pub region_capture_rect: Rect,
    pub capture_counter: u64,
    pub has_capture_update_rect: u8,
    pub has_region_capture_rect: u8,
    pub has_source_size: u8,
    pub has_capture_counter: u8,
}
impl From<_cef_accelerated_paint_info_common_t> for AcceleratedPaintInfoCommon {
    fn from(value: _cef_accelerated_paint_info_common_t) -> Self {
        Self {
            size: value.size.into(),
            timestamp: value.timestamp.into(),
            coded_size: value.coded_size.into(),
            visible_rect: value.visible_rect.into(),
            content_rect: value.content_rect.into(),
            source_size: value.source_size.into(),
            capture_update_rect: value.capture_update_rect.into(),
            region_capture_rect: value.region_capture_rect.into(),
            capture_counter: value.capture_counter.into(),
            has_capture_update_rect: value.has_capture_update_rect.into(),
            has_region_capture_rect: value.has_region_capture_rect.into(),
            has_source_size: value.has_source_size.into(),
            has_capture_counter: value.has_capture_counter.into(),
        }
    }
}
impl Into<_cef_accelerated_paint_info_common_t> for AcceleratedPaintInfoCommon {
    fn into(self) -> _cef_accelerated_paint_info_common_t {
        _cef_accelerated_paint_info_common_t {
            size: self.size.into(),
            timestamp: self.timestamp.into(),
            coded_size: self.coded_size.into(),
            visible_rect: self.visible_rect.into(),
            content_rect: self.content_rect.into(),
            source_size: self.source_size.into(),
            capture_update_rect: self.capture_update_rect.into(),
            region_capture_rect: self.region_capture_rect.into(),
            capture_counter: self.capture_counter.into(),
            has_capture_update_rect: self.has_capture_update_rect.into(),
            has_region_capture_rect: self.has_region_capture_rect.into(),
            has_source_size: self.has_source_size.into(),
            has_capture_counter: self.has_capture_counter.into(),
        }
    }
}
impl Default for AcceleratedPaintInfoCommon {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_accelerated_paint_info_common_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_main_args_t] for more documentation.
#[derive(Clone)]
pub struct MainArgs {
    pub instance: HINSTANCE,
}
impl From<_cef_main_args_t> for MainArgs {
    fn from(value: _cef_main_args_t) -> Self {
        Self {
            instance: value.instance.into(),
        }
    }
}
impl Into<_cef_main_args_t> for MainArgs {
    fn into(self) -> _cef_main_args_t {
        _cef_main_args_t {
            instance: self.instance.into(),
        }
    }
}
impl Default for MainArgs {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_window_info_t] for more documentation.
#[derive(Clone)]
pub struct WindowInfo {
    pub size: usize,
    pub ex_style: DWORD,
    pub window_name: CefStringUtf16,
    pub style: DWORD,
    pub bounds: Rect,
    pub parent_window: HWND,
    pub menu: HMENU,
    pub windowless_rendering_enabled: ::std::os::raw::c_int,
    pub shared_texture_enabled: ::std::os::raw::c_int,
    pub external_begin_frame_enabled: ::std::os::raw::c_int,
    pub window: HWND,
    pub runtime_style: RuntimeStyle,
}
impl From<_cef_window_info_t> for WindowInfo {
    fn from(value: _cef_window_info_t) -> Self {
        Self {
            size: value.size.into(),
            ex_style: value.ex_style.into(),
            window_name: value.window_name.into(),
            style: value.style.into(),
            bounds: value.bounds.into(),
            parent_window: value.parent_window.into(),
            menu: value.menu.into(),
            windowless_rendering_enabled: value.windowless_rendering_enabled.into(),
            shared_texture_enabled: value.shared_texture_enabled.into(),
            external_begin_frame_enabled: value.external_begin_frame_enabled.into(),
            window: value.window.into(),
            runtime_style: value.runtime_style.into(),
        }
    }
}
impl Into<_cef_window_info_t> for WindowInfo {
    fn into(self) -> _cef_window_info_t {
        _cef_window_info_t {
            size: self.size.into(),
            ex_style: self.ex_style.into(),
            window_name: self.window_name.into(),
            style: self.style.into(),
            bounds: self.bounds.into(),
            parent_window: self.parent_window.into(),
            menu: self.menu.into(),
            windowless_rendering_enabled: self.windowless_rendering_enabled.into(),
            shared_texture_enabled: self.shared_texture_enabled.into(),
            external_begin_frame_enabled: self.external_begin_frame_enabled.into(),
            window: self.window.into(),
            runtime_style: self.runtime_style.into(),
        }
    }
}
impl Default for WindowInfo {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_window_info_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_accelerated_paint_info_t] for more documentation.
#[derive(Clone)]
pub struct AcceleratedPaintInfo {
    pub size: usize,
    pub shared_texture_handle: HANDLE,
    pub format: ColorType,
    pub extra: AcceleratedPaintInfoCommon,
}
impl From<_cef_accelerated_paint_info_t> for AcceleratedPaintInfo {
    fn from(value: _cef_accelerated_paint_info_t) -> Self {
        Self {
            size: value.size.into(),
            shared_texture_handle: value.shared_texture_handle.into(),
            format: value.format.into(),
            extra: value.extra.into(),
        }
    }
}
impl Into<_cef_accelerated_paint_info_t> for AcceleratedPaintInfo {
    fn into(self) -> _cef_accelerated_paint_info_t {
        _cef_accelerated_paint_info_t {
            size: self.size.into(),
            shared_texture_handle: self.shared_texture_handle.into(),
            format: self.format.into(),
            extra: self.extra.into(),
        }
    }
}
impl Default for AcceleratedPaintInfo {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_accelerated_paint_info_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_settings_t] for more documentation.
#[derive(Clone)]
pub struct Settings {
    pub size: usize,
    pub no_sandbox: ::std::os::raw::c_int,
    pub browser_subprocess_path: CefStringUtf16,
    pub framework_dir_path: CefStringUtf16,
    pub main_bundle_path: CefStringUtf16,
    pub multi_threaded_message_loop: ::std::os::raw::c_int,
    pub external_message_pump: ::std::os::raw::c_int,
    pub windowless_rendering_enabled: ::std::os::raw::c_int,
    pub command_line_args_disabled: ::std::os::raw::c_int,
    pub cache_path: CefStringUtf16,
    pub root_cache_path: CefStringUtf16,
    pub persist_session_cookies: ::std::os::raw::c_int,
    pub user_agent: CefStringUtf16,
    pub user_agent_product: CefStringUtf16,
    pub locale: CefStringUtf16,
    pub log_file: CefStringUtf16,
    pub log_severity: LogSeverity,
    pub log_items: LogItems,
    pub javascript_flags: CefStringUtf16,
    pub resources_dir_path: CefStringUtf16,
    pub locales_dir_path: CefStringUtf16,
    pub remote_debugging_port: ::std::os::raw::c_int,
    pub uncaught_exception_stack_size: ::std::os::raw::c_int,
    pub background_color: u32,
    pub accept_language_list: CefStringUtf16,
    pub cookieable_schemes_list: CefStringUtf16,
    pub cookieable_schemes_exclude_defaults: ::std::os::raw::c_int,
    pub chrome_policy_id: CefStringUtf16,
    pub chrome_app_icon_id: ::std::os::raw::c_int,
    pub disable_signal_handlers: ::std::os::raw::c_int,
}
impl From<_cef_settings_t> for Settings {
    fn from(value: _cef_settings_t) -> Self {
        Self {
            size: value.size.into(),
            no_sandbox: value.no_sandbox.into(),
            browser_subprocess_path: value.browser_subprocess_path.into(),
            framework_dir_path: value.framework_dir_path.into(),
            main_bundle_path: value.main_bundle_path.into(),
            multi_threaded_message_loop: value.multi_threaded_message_loop.into(),
            external_message_pump: value.external_message_pump.into(),
            windowless_rendering_enabled: value.windowless_rendering_enabled.into(),
            command_line_args_disabled: value.command_line_args_disabled.into(),
            cache_path: value.cache_path.into(),
            root_cache_path: value.root_cache_path.into(),
            persist_session_cookies: value.persist_session_cookies.into(),
            user_agent: value.user_agent.into(),
            user_agent_product: value.user_agent_product.into(),
            locale: value.locale.into(),
            log_file: value.log_file.into(),
            log_severity: value.log_severity.into(),
            log_items: value.log_items.into(),
            javascript_flags: value.javascript_flags.into(),
            resources_dir_path: value.resources_dir_path.into(),
            locales_dir_path: value.locales_dir_path.into(),
            remote_debugging_port: value.remote_debugging_port.into(),
            uncaught_exception_stack_size: value.uncaught_exception_stack_size.into(),
            background_color: value.background_color.into(),
            accept_language_list: value.accept_language_list.into(),
            cookieable_schemes_list: value.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: value.cookieable_schemes_exclude_defaults.into(),
            chrome_policy_id: value.chrome_policy_id.into(),
            chrome_app_icon_id: value.chrome_app_icon_id.into(),
            disable_signal_handlers: value.disable_signal_handlers.into(),
        }
    }
}
impl Into<_cef_settings_t> for Settings {
    fn into(self) -> _cef_settings_t {
        _cef_settings_t {
            size: self.size.into(),
            no_sandbox: self.no_sandbox.into(),
            browser_subprocess_path: self.browser_subprocess_path.into(),
            framework_dir_path: self.framework_dir_path.into(),
            main_bundle_path: self.main_bundle_path.into(),
            multi_threaded_message_loop: self.multi_threaded_message_loop.into(),
            external_message_pump: self.external_message_pump.into(),
            windowless_rendering_enabled: self.windowless_rendering_enabled.into(),
            command_line_args_disabled: self.command_line_args_disabled.into(),
            cache_path: self.cache_path.into(),
            root_cache_path: self.root_cache_path.into(),
            persist_session_cookies: self.persist_session_cookies.into(),
            user_agent: self.user_agent.into(),
            user_agent_product: self.user_agent_product.into(),
            locale: self.locale.into(),
            log_file: self.log_file.into(),
            log_severity: self.log_severity.into(),
            log_items: self.log_items.into(),
            javascript_flags: self.javascript_flags.into(),
            resources_dir_path: self.resources_dir_path.into(),
            locales_dir_path: self.locales_dir_path.into(),
            remote_debugging_port: self.remote_debugging_port.into(),
            uncaught_exception_stack_size: self.uncaught_exception_stack_size.into(),
            background_color: self.background_color.into(),
            accept_language_list: self.accept_language_list.into(),
            cookieable_schemes_list: self.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: self.cookieable_schemes_exclude_defaults.into(),
            chrome_policy_id: self.chrome_policy_id.into(),
            chrome_app_icon_id: self.chrome_app_icon_id.into(),
            disable_signal_handlers: self.disable_signal_handlers.into(),
        }
    }
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_settings_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_request_context_settings_t] for more documentation.
#[derive(Clone)]
pub struct RequestContextSettings {
    pub size: usize,
    pub cache_path: CefStringUtf16,
    pub persist_session_cookies: ::std::os::raw::c_int,
    pub accept_language_list: CefStringUtf16,
    pub cookieable_schemes_list: CefStringUtf16,
    pub cookieable_schemes_exclude_defaults: ::std::os::raw::c_int,
}
impl From<_cef_request_context_settings_t> for RequestContextSettings {
    fn from(value: _cef_request_context_settings_t) -> Self {
        Self {
            size: value.size.into(),
            cache_path: value.cache_path.into(),
            persist_session_cookies: value.persist_session_cookies.into(),
            accept_language_list: value.accept_language_list.into(),
            cookieable_schemes_list: value.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: value.cookieable_schemes_exclude_defaults.into(),
        }
    }
}
impl Into<_cef_request_context_settings_t> for RequestContextSettings {
    fn into(self) -> _cef_request_context_settings_t {
        _cef_request_context_settings_t {
            size: self.size.into(),
            cache_path: self.cache_path.into(),
            persist_session_cookies: self.persist_session_cookies.into(),
            accept_language_list: self.accept_language_list.into(),
            cookieable_schemes_list: self.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: self.cookieable_schemes_exclude_defaults.into(),
        }
    }
}
impl Default for RequestContextSettings {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_request_context_settings_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_browser_settings_t] for more documentation.
#[derive(Clone)]
pub struct BrowserSettings {
    pub size: usize,
    pub windowless_frame_rate: ::std::os::raw::c_int,
    pub standard_font_family: CefStringUtf16,
    pub fixed_font_family: CefStringUtf16,
    pub serif_font_family: CefStringUtf16,
    pub sans_serif_font_family: CefStringUtf16,
    pub cursive_font_family: CefStringUtf16,
    pub fantasy_font_family: CefStringUtf16,
    pub default_font_size: ::std::os::raw::c_int,
    pub default_fixed_font_size: ::std::os::raw::c_int,
    pub minimum_font_size: ::std::os::raw::c_int,
    pub minimum_logical_font_size: ::std::os::raw::c_int,
    pub default_encoding: CefStringUtf16,
    pub remote_fonts: State,
    pub javascript: State,
    pub javascript_close_windows: State,
    pub javascript_access_clipboard: State,
    pub javascript_dom_paste: State,
    pub image_loading: State,
    pub image_shrink_standalone_to_fit: State,
    pub text_area_resize: State,
    pub tab_to_links: State,
    pub local_storage: State,
    pub databases: State,
    pub webgl: State,
    pub background_color: u32,
    pub chrome_status_bubble: State,
    pub chrome_zoom_bubble: State,
}
impl From<_cef_browser_settings_t> for BrowserSettings {
    fn from(value: _cef_browser_settings_t) -> Self {
        Self {
            size: value.size.into(),
            windowless_frame_rate: value.windowless_frame_rate.into(),
            standard_font_family: value.standard_font_family.into(),
            fixed_font_family: value.fixed_font_family.into(),
            serif_font_family: value.serif_font_family.into(),
            sans_serif_font_family: value.sans_serif_font_family.into(),
            cursive_font_family: value.cursive_font_family.into(),
            fantasy_font_family: value.fantasy_font_family.into(),
            default_font_size: value.default_font_size.into(),
            default_fixed_font_size: value.default_fixed_font_size.into(),
            minimum_font_size: value.minimum_font_size.into(),
            minimum_logical_font_size: value.minimum_logical_font_size.into(),
            default_encoding: value.default_encoding.into(),
            remote_fonts: value.remote_fonts.into(),
            javascript: value.javascript.into(),
            javascript_close_windows: value.javascript_close_windows.into(),
            javascript_access_clipboard: value.javascript_access_clipboard.into(),
            javascript_dom_paste: value.javascript_dom_paste.into(),
            image_loading: value.image_loading.into(),
            image_shrink_standalone_to_fit: value.image_shrink_standalone_to_fit.into(),
            text_area_resize: value.text_area_resize.into(),
            tab_to_links: value.tab_to_links.into(),
            local_storage: value.local_storage.into(),
            databases: value.databases.into(),
            webgl: value.webgl.into(),
            background_color: value.background_color.into(),
            chrome_status_bubble: value.chrome_status_bubble.into(),
            chrome_zoom_bubble: value.chrome_zoom_bubble.into(),
        }
    }
}
impl Into<_cef_browser_settings_t> for BrowserSettings {
    fn into(self) -> _cef_browser_settings_t {
        _cef_browser_settings_t {
            size: self.size.into(),
            windowless_frame_rate: self.windowless_frame_rate.into(),
            standard_font_family: self.standard_font_family.into(),
            fixed_font_family: self.fixed_font_family.into(),
            serif_font_family: self.serif_font_family.into(),
            sans_serif_font_family: self.sans_serif_font_family.into(),
            cursive_font_family: self.cursive_font_family.into(),
            fantasy_font_family: self.fantasy_font_family.into(),
            default_font_size: self.default_font_size.into(),
            default_fixed_font_size: self.default_fixed_font_size.into(),
            minimum_font_size: self.minimum_font_size.into(),
            minimum_logical_font_size: self.minimum_logical_font_size.into(),
            default_encoding: self.default_encoding.into(),
            remote_fonts: self.remote_fonts.into(),
            javascript: self.javascript.into(),
            javascript_close_windows: self.javascript_close_windows.into(),
            javascript_access_clipboard: self.javascript_access_clipboard.into(),
            javascript_dom_paste: self.javascript_dom_paste.into(),
            image_loading: self.image_loading.into(),
            image_shrink_standalone_to_fit: self.image_shrink_standalone_to_fit.into(),
            text_area_resize: self.text_area_resize.into(),
            tab_to_links: self.tab_to_links.into(),
            local_storage: self.local_storage.into(),
            databases: self.databases.into(),
            webgl: self.webgl.into(),
            background_color: self.background_color.into(),
            chrome_status_bubble: self.chrome_status_bubble.into(),
            chrome_zoom_bubble: self.chrome_zoom_bubble.into(),
        }
    }
}
impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_browser_settings_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_urlparts_t] for more documentation.
#[derive(Clone)]
pub struct Urlparts {
    pub size: usize,
    pub spec: CefStringUtf16,
    pub scheme: CefStringUtf16,
    pub username: CefStringUtf16,
    pub password: CefStringUtf16,
    pub host: CefStringUtf16,
    pub port: CefStringUtf16,
    pub origin: CefStringUtf16,
    pub path: CefStringUtf16,
    pub query: CefStringUtf16,
    pub fragment: CefStringUtf16,
}
impl From<_cef_urlparts_t> for Urlparts {
    fn from(value: _cef_urlparts_t) -> Self {
        Self {
            size: value.size.into(),
            spec: value.spec.into(),
            scheme: value.scheme.into(),
            username: value.username.into(),
            password: value.password.into(),
            host: value.host.into(),
            port: value.port.into(),
            origin: value.origin.into(),
            path: value.path.into(),
            query: value.query.into(),
            fragment: value.fragment.into(),
        }
    }
}
impl Into<_cef_urlparts_t> for Urlparts {
    fn into(self) -> _cef_urlparts_t {
        _cef_urlparts_t {
            size: self.size.into(),
            spec: self.spec.into(),
            scheme: self.scheme.into(),
            username: self.username.into(),
            password: self.password.into(),
            host: self.host.into(),
            port: self.port.into(),
            origin: self.origin.into(),
            path: self.path.into(),
            query: self.query.into(),
            fragment: self.fragment.into(),
        }
    }
}
impl Default for Urlparts {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_urlparts_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_cookie_t] for more documentation.
#[derive(Clone)]
pub struct Cookie {
    pub size: usize,
    pub name: CefStringUtf16,
    pub value: CefStringUtf16,
    pub domain: CefStringUtf16,
    pub path: CefStringUtf16,
    pub secure: ::std::os::raw::c_int,
    pub httponly: ::std::os::raw::c_int,
    pub creation: Basetime,
    pub last_access: Basetime,
    pub has_expires: ::std::os::raw::c_int,
    pub expires: Basetime,
    pub same_site: CookieSameSite,
    pub priority: CookiePriority,
}
impl From<_cef_cookie_t> for Cookie {
    fn from(value: _cef_cookie_t) -> Self {
        Self {
            size: value.size.into(),
            name: value.name.into(),
            value: value.value.into(),
            domain: value.domain.into(),
            path: value.path.into(),
            secure: value.secure.into(),
            httponly: value.httponly.into(),
            creation: value.creation.into(),
            last_access: value.last_access.into(),
            has_expires: value.has_expires.into(),
            expires: value.expires.into(),
            same_site: value.same_site.into(),
            priority: value.priority.into(),
        }
    }
}
impl Into<_cef_cookie_t> for Cookie {
    fn into(self) -> _cef_cookie_t {
        _cef_cookie_t {
            size: self.size.into(),
            name: self.name.into(),
            value: self.value.into(),
            domain: self.domain.into(),
            path: self.path.into(),
            secure: self.secure.into(),
            httponly: self.httponly.into(),
            creation: self.creation.into(),
            last_access: self.last_access.into(),
            has_expires: self.has_expires.into(),
            expires: self.expires.into(),
            same_site: self.same_site.into(),
            priority: self.priority.into(),
        }
    }
}
impl Default for Cookie {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_cookie_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_draggable_region_t] for more documentation.
#[derive(Clone)]
pub struct DraggableRegion {
    pub bounds: Rect,
    pub draggable: ::std::os::raw::c_int,
}
impl From<_cef_draggable_region_t> for DraggableRegion {
    fn from(value: _cef_draggable_region_t) -> Self {
        Self {
            bounds: value.bounds.into(),
            draggable: value.draggable.into(),
        }
    }
}
impl Into<_cef_draggable_region_t> for DraggableRegion {
    fn into(self) -> _cef_draggable_region_t {
        _cef_draggable_region_t {
            bounds: self.bounds.into(),
            draggable: self.draggable.into(),
        }
    }
}
impl Default for DraggableRegion {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_screen_info_t] for more documentation.
#[derive(Clone)]
pub struct ScreenInfo {
    pub size: usize,
    pub device_scale_factor: f32,
    pub depth: ::std::os::raw::c_int,
    pub depth_per_component: ::std::os::raw::c_int,
    pub is_monochrome: ::std::os::raw::c_int,
    pub rect: Rect,
    pub available_rect: Rect,
}
impl From<_cef_screen_info_t> for ScreenInfo {
    fn from(value: _cef_screen_info_t) -> Self {
        Self {
            size: value.size.into(),
            device_scale_factor: value.device_scale_factor.into(),
            depth: value.depth.into(),
            depth_per_component: value.depth_per_component.into(),
            is_monochrome: value.is_monochrome.into(),
            rect: value.rect.into(),
            available_rect: value.available_rect.into(),
        }
    }
}
impl Into<_cef_screen_info_t> for ScreenInfo {
    fn into(self) -> _cef_screen_info_t {
        _cef_screen_info_t {
            size: self.size.into(),
            device_scale_factor: self.device_scale_factor.into(),
            depth: self.depth.into(),
            depth_per_component: self.depth_per_component.into(),
            is_monochrome: self.is_monochrome.into(),
            rect: self.rect.into(),
            available_rect: self.available_rect.into(),
        }
    }
}
impl Default for ScreenInfo {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_screen_info_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_linux_window_properties_t] for more documentation.
#[derive(Clone)]
pub struct LinuxWindowProperties {
    pub size: usize,
    pub wayland_app_id: CefStringUtf16,
    pub wm_class_class: CefStringUtf16,
    pub wm_class_name: CefStringUtf16,
    pub wm_role_name: CefStringUtf16,
}
impl From<_cef_linux_window_properties_t> for LinuxWindowProperties {
    fn from(value: _cef_linux_window_properties_t) -> Self {
        Self {
            size: value.size.into(),
            wayland_app_id: value.wayland_app_id.into(),
            wm_class_class: value.wm_class_class.into(),
            wm_class_name: value.wm_class_name.into(),
            wm_role_name: value.wm_role_name.into(),
        }
    }
}
impl Into<_cef_linux_window_properties_t> for LinuxWindowProperties {
    fn into(self) -> _cef_linux_window_properties_t {
        _cef_linux_window_properties_t {
            size: self.size.into(),
            wayland_app_id: self.wayland_app_id.into(),
            wm_class_class: self.wm_class_class.into(),
            wm_class_name: self.wm_class_name.into(),
            wm_role_name: self.wm_role_name.into(),
        }
    }
}
impl Default for LinuxWindowProperties {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_linux_window_properties_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_mouse_event_t] for more documentation.
#[derive(Clone)]
pub struct MouseEvent {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub modifiers: u32,
}
impl From<_cef_mouse_event_t> for MouseEvent {
    fn from(value: _cef_mouse_event_t) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
            modifiers: value.modifiers.into(),
        }
    }
}
impl Into<_cef_mouse_event_t> for MouseEvent {
    fn into(self) -> _cef_mouse_event_t {
        _cef_mouse_event_t {
            x: self.x.into(),
            y: self.y.into(),
            modifiers: self.modifiers.into(),
        }
    }
}
impl Default for MouseEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_touch_event_t] for more documentation.
#[derive(Clone)]
pub struct TouchEvent {
    pub id: ::std::os::raw::c_int,
    pub x: f32,
    pub y: f32,
    pub radius_x: f32,
    pub radius_y: f32,
    pub rotation_angle: f32,
    pub pressure: f32,
    pub type_: TouchEventType,
    pub modifiers: u32,
    pub pointer_type: PointerType,
}
impl From<_cef_touch_event_t> for TouchEvent {
    fn from(value: _cef_touch_event_t) -> Self {
        Self {
            id: value.id.into(),
            x: value.x.into(),
            y: value.y.into(),
            radius_x: value.radius_x.into(),
            radius_y: value.radius_y.into(),
            rotation_angle: value.rotation_angle.into(),
            pressure: value.pressure.into(),
            type_: value.type_.into(),
            modifiers: value.modifiers.into(),
            pointer_type: value.pointer_type.into(),
        }
    }
}
impl Into<_cef_touch_event_t> for TouchEvent {
    fn into(self) -> _cef_touch_event_t {
        _cef_touch_event_t {
            id: self.id.into(),
            x: self.x.into(),
            y: self.y.into(),
            radius_x: self.radius_x.into(),
            radius_y: self.radius_y.into(),
            rotation_angle: self.rotation_angle.into(),
            pressure: self.pressure.into(),
            type_: self.type_.into(),
            modifiers: self.modifiers.into(),
            pointer_type: self.pointer_type.into(),
        }
    }
}
impl Default for TouchEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_key_event_t] for more documentation.
#[derive(Clone)]
pub struct KeyEvent {
    pub size: usize,
    pub type_: KeyEventType,
    pub modifiers: u32,
    pub windows_key_code: ::std::os::raw::c_int,
    pub native_key_code: ::std::os::raw::c_int,
    pub is_system_key: ::std::os::raw::c_int,
    pub character: char16_t,
    pub unmodified_character: char16_t,
    pub focus_on_editable_field: ::std::os::raw::c_int,
}
impl From<_cef_key_event_t> for KeyEvent {
    fn from(value: _cef_key_event_t) -> Self {
        Self {
            size: value.size.into(),
            type_: value.type_.into(),
            modifiers: value.modifiers.into(),
            windows_key_code: value.windows_key_code.into(),
            native_key_code: value.native_key_code.into(),
            is_system_key: value.is_system_key.into(),
            character: value.character.into(),
            unmodified_character: value.unmodified_character.into(),
            focus_on_editable_field: value.focus_on_editable_field.into(),
        }
    }
}
impl Into<_cef_key_event_t> for KeyEvent {
    fn into(self) -> _cef_key_event_t {
        _cef_key_event_t {
            size: self.size.into(),
            type_: self.type_.into(),
            modifiers: self.modifiers.into(),
            windows_key_code: self.windows_key_code.into(),
            native_key_code: self.native_key_code.into(),
            is_system_key: self.is_system_key.into(),
            character: self.character.into(),
            unmodified_character: self.unmodified_character.into(),
            focus_on_editable_field: self.focus_on_editable_field.into(),
        }
    }
}
impl Default for KeyEvent {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_key_event_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_popup_features_t] for more documentation.
#[derive(Clone)]
pub struct PopupFeatures {
    pub size: usize,
    pub x: ::std::os::raw::c_int,
    pub x_set: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub y_set: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub width_set: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub height_set: ::std::os::raw::c_int,
    pub is_popup: ::std::os::raw::c_int,
}
impl From<_cef_popup_features_t> for PopupFeatures {
    fn from(value: _cef_popup_features_t) -> Self {
        Self {
            size: value.size.into(),
            x: value.x.into(),
            x_set: value.xSet.into(),
            y: value.y.into(),
            y_set: value.ySet.into(),
            width: value.width.into(),
            width_set: value.widthSet.into(),
            height: value.height.into(),
            height_set: value.heightSet.into(),
            is_popup: value.isPopup.into(),
        }
    }
}
impl Into<_cef_popup_features_t> for PopupFeatures {
    fn into(self) -> _cef_popup_features_t {
        _cef_popup_features_t {
            size: self.size.into(),
            x: self.x.into(),
            xSet: self.x_set.into(),
            y: self.y.into(),
            ySet: self.y_set.into(),
            width: self.width.into(),
            widthSet: self.width_set.into(),
            height: self.height.into(),
            heightSet: self.height_set.into(),
            isPopup: self.is_popup.into(),
        }
    }
}
impl Default for PopupFeatures {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_popup_features_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_cursor_info_t] for more documentation.
#[derive(Clone)]
pub struct CursorInfo {
    pub hotspot: Point,
    pub image_scale_factor: f32,
    pub buffer: *mut ::std::os::raw::c_void,
    pub size: Size,
}
impl From<_cef_cursor_info_t> for CursorInfo {
    fn from(value: _cef_cursor_info_t) -> Self {
        Self {
            hotspot: value.hotspot.into(),
            image_scale_factor: value.image_scale_factor.into(),
            buffer: value.buffer.into(),
            size: value.size.into(),
        }
    }
}
impl Into<_cef_cursor_info_t> for CursorInfo {
    fn into(self) -> _cef_cursor_info_t {
        _cef_cursor_info_t {
            hotspot: self.hotspot.into(),
            image_scale_factor: self.image_scale_factor.into(),
            buffer: self.buffer.into(),
            size: self.size.into(),
        }
    }
}
impl Default for CursorInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_pdf_print_settings_t] for more documentation.
#[derive(Clone)]
pub struct PdfPrintSettings {
    pub size: usize,
    pub landscape: ::std::os::raw::c_int,
    pub print_background: ::std::os::raw::c_int,
    pub scale: f64,
    pub paper_width: f64,
    pub paper_height: f64,
    pub prefer_css_page_size: ::std::os::raw::c_int,
    pub margin_type: PdfPrintMarginType,
    pub margin_top: f64,
    pub margin_right: f64,
    pub margin_bottom: f64,
    pub margin_left: f64,
    pub page_ranges: CefStringUtf16,
    pub display_header_footer: ::std::os::raw::c_int,
    pub header_template: CefStringUtf16,
    pub footer_template: CefStringUtf16,
    pub generate_tagged_pdf: ::std::os::raw::c_int,
    pub generate_document_outline: ::std::os::raw::c_int,
}
impl From<_cef_pdf_print_settings_t> for PdfPrintSettings {
    fn from(value: _cef_pdf_print_settings_t) -> Self {
        Self {
            size: value.size.into(),
            landscape: value.landscape.into(),
            print_background: value.print_background.into(),
            scale: value.scale.into(),
            paper_width: value.paper_width.into(),
            paper_height: value.paper_height.into(),
            prefer_css_page_size: value.prefer_css_page_size.into(),
            margin_type: value.margin_type.into(),
            margin_top: value.margin_top.into(),
            margin_right: value.margin_right.into(),
            margin_bottom: value.margin_bottom.into(),
            margin_left: value.margin_left.into(),
            page_ranges: value.page_ranges.into(),
            display_header_footer: value.display_header_footer.into(),
            header_template: value.header_template.into(),
            footer_template: value.footer_template.into(),
            generate_tagged_pdf: value.generate_tagged_pdf.into(),
            generate_document_outline: value.generate_document_outline.into(),
        }
    }
}
impl Into<_cef_pdf_print_settings_t> for PdfPrintSettings {
    fn into(self) -> _cef_pdf_print_settings_t {
        _cef_pdf_print_settings_t {
            size: self.size.into(),
            landscape: self.landscape.into(),
            print_background: self.print_background.into(),
            scale: self.scale.into(),
            paper_width: self.paper_width.into(),
            paper_height: self.paper_height.into(),
            prefer_css_page_size: self.prefer_css_page_size.into(),
            margin_type: self.margin_type.into(),
            margin_top: self.margin_top.into(),
            margin_right: self.margin_right.into(),
            margin_bottom: self.margin_bottom.into(),
            margin_left: self.margin_left.into(),
            page_ranges: self.page_ranges.into(),
            display_header_footer: self.display_header_footer.into(),
            header_template: self.header_template.into(),
            footer_template: self.footer_template.into(),
            generate_tagged_pdf: self.generate_tagged_pdf.into(),
            generate_document_outline: self.generate_document_outline.into(),
        }
    }
}
impl Default for PdfPrintSettings {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_pdf_print_settings_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_box_layout_settings_t] for more documentation.
#[derive(Clone)]
pub struct BoxLayoutSettings {
    pub size: usize,
    pub horizontal: ::std::os::raw::c_int,
    pub inside_border_horizontal_spacing: ::std::os::raw::c_int,
    pub inside_border_vertical_spacing: ::std::os::raw::c_int,
    pub inside_border_insets: Insets,
    pub between_child_spacing: ::std::os::raw::c_int,
    pub main_axis_alignment: AxisAlignment,
    pub cross_axis_alignment: AxisAlignment,
    pub minimum_cross_axis_size: ::std::os::raw::c_int,
    pub default_flex: ::std::os::raw::c_int,
}
impl From<_cef_box_layout_settings_t> for BoxLayoutSettings {
    fn from(value: _cef_box_layout_settings_t) -> Self {
        Self {
            size: value.size.into(),
            horizontal: value.horizontal.into(),
            inside_border_horizontal_spacing: value.inside_border_horizontal_spacing.into(),
            inside_border_vertical_spacing: value.inside_border_vertical_spacing.into(),
            inside_border_insets: value.inside_border_insets.into(),
            between_child_spacing: value.between_child_spacing.into(),
            main_axis_alignment: value.main_axis_alignment.into(),
            cross_axis_alignment: value.cross_axis_alignment.into(),
            minimum_cross_axis_size: value.minimum_cross_axis_size.into(),
            default_flex: value.default_flex.into(),
        }
    }
}
impl Into<_cef_box_layout_settings_t> for BoxLayoutSettings {
    fn into(self) -> _cef_box_layout_settings_t {
        _cef_box_layout_settings_t {
            size: self.size.into(),
            horizontal: self.horizontal.into(),
            inside_border_horizontal_spacing: self.inside_border_horizontal_spacing.into(),
            inside_border_vertical_spacing: self.inside_border_vertical_spacing.into(),
            inside_border_insets: self.inside_border_insets.into(),
            between_child_spacing: self.between_child_spacing.into(),
            main_axis_alignment: self.main_axis_alignment.into(),
            cross_axis_alignment: self.cross_axis_alignment.into(),
            minimum_cross_axis_size: self.minimum_cross_axis_size.into(),
            default_flex: self.default_flex.into(),
        }
    }
}
impl Default for BoxLayoutSettings {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_box_layout_settings_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_range_t] for more documentation.
#[derive(Clone)]
pub struct Range {
    pub from: u32,
    pub to: u32,
}
impl From<_cef_range_t> for Range {
    fn from(value: _cef_range_t) -> Self {
        Self {
            from: value.from.into(),
            to: value.to.into(),
        }
    }
}
impl Into<_cef_range_t> for Range {
    fn into(self) -> _cef_range_t {
        _cef_range_t {
            from: self.from.into(),
            to: self.to.into(),
        }
    }
}
impl Default for Range {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_composition_underline_t] for more documentation.
#[derive(Clone)]
pub struct CompositionUnderline {
    pub size: usize,
    pub range: Range,
    pub color: u32,
    pub background_color: u32,
    pub thick: ::std::os::raw::c_int,
    pub style: CompositionUnderlineStyle,
}
impl From<_cef_composition_underline_t> for CompositionUnderline {
    fn from(value: _cef_composition_underline_t) -> Self {
        Self {
            size: value.size.into(),
            range: value.range.into(),
            color: value.color.into(),
            background_color: value.background_color.into(),
            thick: value.thick.into(),
            style: value.style.into(),
        }
    }
}
impl Into<_cef_composition_underline_t> for CompositionUnderline {
    fn into(self) -> _cef_composition_underline_t {
        _cef_composition_underline_t {
            size: self.size.into(),
            range: self.range.into(),
            color: self.color.into(),
            background_color: self.background_color.into(),
            thick: self.thick.into(),
            style: self.style.into(),
        }
    }
}
impl Default for CompositionUnderline {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_composition_underline_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_audio_parameters_t] for more documentation.
#[derive(Clone)]
pub struct AudioParameters {
    pub size: usize,
    pub channel_layout: ChannelLayout,
    pub sample_rate: ::std::os::raw::c_int,
    pub frames_per_buffer: ::std::os::raw::c_int,
}
impl From<_cef_audio_parameters_t> for AudioParameters {
    fn from(value: _cef_audio_parameters_t) -> Self {
        Self {
            size: value.size.into(),
            channel_layout: value.channel_layout.into(),
            sample_rate: value.sample_rate.into(),
            frames_per_buffer: value.frames_per_buffer.into(),
        }
    }
}
impl Into<_cef_audio_parameters_t> for AudioParameters {
    fn into(self) -> _cef_audio_parameters_t {
        _cef_audio_parameters_t {
            size: self.size.into(),
            channel_layout: self.channel_layout.into(),
            sample_rate: self.sample_rate.into(),
            frames_per_buffer: self.frames_per_buffer.into(),
        }
    }
}
impl Default for AudioParameters {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_audio_parameters_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_media_sink_device_info_t] for more documentation.
#[derive(Clone)]
pub struct MediaSinkDeviceInfo {
    pub size: usize,
    pub ip_address: CefStringUtf16,
    pub port: ::std::os::raw::c_int,
    pub model_name: CefStringUtf16,
}
impl From<_cef_media_sink_device_info_t> for MediaSinkDeviceInfo {
    fn from(value: _cef_media_sink_device_info_t) -> Self {
        Self {
            size: value.size.into(),
            ip_address: value.ip_address.into(),
            port: value.port.into(),
            model_name: value.model_name.into(),
        }
    }
}
impl Into<_cef_media_sink_device_info_t> for MediaSinkDeviceInfo {
    fn into(self) -> _cef_media_sink_device_info_t {
        _cef_media_sink_device_info_t {
            size: self.size.into(),
            ip_address: self.ip_address.into(),
            port: self.port.into(),
            model_name: self.model_name.into(),
        }
    }
}
impl Default for MediaSinkDeviceInfo {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_media_sink_device_info_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_touch_handle_state_t] for more documentation.
#[derive(Clone)]
pub struct TouchHandleState {
    pub size: usize,
    pub touch_handle_id: ::std::os::raw::c_int,
    pub flags: u32,
    pub enabled: ::std::os::raw::c_int,
    pub orientation: HorizontalAlignment,
    pub mirror_vertical: ::std::os::raw::c_int,
    pub mirror_horizontal: ::std::os::raw::c_int,
    pub origin: Point,
    pub alpha: f32,
}
impl From<_cef_touch_handle_state_t> for TouchHandleState {
    fn from(value: _cef_touch_handle_state_t) -> Self {
        Self {
            size: value.size.into(),
            touch_handle_id: value.touch_handle_id.into(),
            flags: value.flags.into(),
            enabled: value.enabled.into(),
            orientation: value.orientation.into(),
            mirror_vertical: value.mirror_vertical.into(),
            mirror_horizontal: value.mirror_horizontal.into(),
            origin: value.origin.into(),
            alpha: value.alpha.into(),
        }
    }
}
impl Into<_cef_touch_handle_state_t> for TouchHandleState {
    fn into(self) -> _cef_touch_handle_state_t {
        _cef_touch_handle_state_t {
            size: self.size.into(),
            touch_handle_id: self.touch_handle_id.into(),
            flags: self.flags.into(),
            enabled: self.enabled.into(),
            orientation: self.orientation.into(),
            mirror_vertical: self.mirror_vertical.into(),
            mirror_horizontal: self.mirror_horizontal.into(),
            origin: self.origin.into(),
            alpha: self.alpha.into(),
        }
    }
}
impl Default for TouchHandleState {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_touch_handle_state_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_task_info_t] for more documentation.
#[derive(Clone)]
pub struct TaskInfo {
    pub size: usize,
    pub id: i64,
    pub type_: TaskType,
    pub is_killable: ::std::os::raw::c_int,
    pub title: CefStringUtf16,
    pub cpu_usage: f64,
    pub number_of_processors: ::std::os::raw::c_int,
    pub memory: i64,
    pub gpu_memory: i64,
    pub is_gpu_memory_inflated: ::std::os::raw::c_int,
}
impl From<_cef_task_info_t> for TaskInfo {
    fn from(value: _cef_task_info_t) -> Self {
        Self {
            size: value.size.into(),
            id: value.id.into(),
            type_: value.type_.into(),
            is_killable: value.is_killable.into(),
            title: value.title.into(),
            cpu_usage: value.cpu_usage.into(),
            number_of_processors: value.number_of_processors.into(),
            memory: value.memory.into(),
            gpu_memory: value.gpu_memory.into(),
            is_gpu_memory_inflated: value.is_gpu_memory_inflated.into(),
        }
    }
}
impl Into<_cef_task_info_t> for TaskInfo {
    fn into(self) -> _cef_task_info_t {
        _cef_task_info_t {
            size: self.size.into(),
            id: self.id.into(),
            type_: self.type_.into(),
            is_killable: self.is_killable.into(),
            title: self.title.into(),
            cpu_usage: self.cpu_usage.into(),
            number_of_processors: self.number_of_processors.into(),
            memory: self.memory.into(),
            gpu_memory: self.gpu_memory.into(),
            is_gpu_memory_inflated: self.is_gpu_memory_inflated.into(),
        }
    }
}
impl Default for TaskInfo {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_task_info_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [_cef_base_ref_counted_t] for more documentation.
#[derive(Clone)]
pub struct BaseRefCounted(RefGuard<_cef_base_ref_counted_t>);
impl BaseRefCounted {
    fn get_raw(&self) -> *mut _cef_base_ref_counted_t {
        unsafe { RefGuard::as_raw(&self.0) }
    }
}
impl Rc for BaseRefCounted {
    fn as_base(&self) -> &_cef_base_ref_counted_t {
        self.0.as_base()
    }
}
impl ConvertParam<*mut _cef_base_ref_counted_t> for &BaseRefCounted {
    fn as_raw(self) -> *mut _cef_base_ref_counted_t {
        self.get_raw()
    }
}
impl ConvertParam<*mut _cef_base_ref_counted_t> for &mut BaseRefCounted {
    fn as_raw(self) -> *mut _cef_base_ref_counted_t {
        self.get_raw()
    }
}
impl ConvertReturnValue<BaseRefCounted> for *mut _cef_base_ref_counted_t {
    fn as_wrapper(self) -> BaseRefCounted {
        BaseRefCounted(unsafe { RefGuard::from_raw(self) })
    }
}
impl Into<*mut _cef_base_ref_counted_t> for BaseRefCounted {
    fn into(self) -> *mut _cef_base_ref_counted_t {
        let object = self.get_raw();
        std::mem::forget(self);
        object
    }
}
impl Default for BaseRefCounted {
    fn default() -> Self {
        Self(unsafe { RefGuard::from_raw(std::ptr::null_mut()) })
    }
}

/// See [_cef_base_scoped_t] for more documentation.
#[derive(Clone, Copy)]
pub struct BaseScoped(*mut _cef_base_scoped_t);
impl BaseScoped {
    fn get_raw(&self) -> *mut _cef_base_scoped_t {
        self.0
    }
}
impl ConvertParam<*mut _cef_base_scoped_t> for &BaseScoped {
    fn as_raw(self) -> *mut _cef_base_scoped_t {
        self.get_raw()
    }
}
impl ConvertParam<*mut _cef_base_scoped_t> for &mut BaseScoped {
    fn as_raw(self) -> *mut _cef_base_scoped_t {
        self.get_raw()
    }
}
impl ConvertReturnValue<BaseScoped> for *mut _cef_base_scoped_t {
    fn as_wrapper(self) -> BaseScoped {
        BaseScoped(self)
    }
}
impl Into<*mut _cef_base_scoped_t> for BaseScoped {
    fn into(self) -> *mut _cef_base_scoped_t {
        self.get_raw()
    }
}
impl Default for BaseScoped {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

/// See [_cef_dev_tools_message_observer_t] for more documentation.
#[derive(Clone)]
pub struct DevToolsMessageObserver {
    pub base: BaseRefCounted,
    pub on_dev_tools_message: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dev_tools_message_observer_t,
            browser: *mut _cef_browser_t,
            message: *const ::std::os::raw::c_void,
            message_size: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_dev_tools_method_result: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dev_tools_message_observer_t,
            browser: *mut _cef_browser_t,
            message_id: ::std::os::raw::c_int,
            success: ::std::os::raw::c_int,
            result: *const ::std::os::raw::c_void,
            result_size: usize,
        ),
    >,
    pub on_dev_tools_event: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dev_tools_message_observer_t,
            browser: *mut _cef_browser_t,
            method: *const cef_string_t,
            params: *const ::std::os::raw::c_void,
            params_size: usize,
        ),
    >,
    pub on_dev_tools_agent_attached: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dev_tools_message_observer_t,
            browser: *mut _cef_browser_t,
        ),
    >,
    pub on_dev_tools_agent_detached: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dev_tools_message_observer_t,
            browser: *mut _cef_browser_t,
        ),
    >,
}
impl From<_cef_dev_tools_message_observer_t> for DevToolsMessageObserver {
    fn from(value: _cef_dev_tools_message_observer_t) -> Self {
        Self {
            base: value.base.into(),
            on_dev_tools_message: value.on_dev_tools_message.into(),
            on_dev_tools_method_result: value.on_dev_tools_method_result.into(),
            on_dev_tools_event: value.on_dev_tools_event.into(),
            on_dev_tools_agent_attached: value.on_dev_tools_agent_attached.into(),
            on_dev_tools_agent_detached: value.on_dev_tools_agent_detached.into(),
        }
    }
}
impl Into<_cef_dev_tools_message_observer_t> for DevToolsMessageObserver {
    fn into(self) -> _cef_dev_tools_message_observer_t {
        _cef_dev_tools_message_observer_t {
            base: self.base.into(),
            on_dev_tools_message: self.on_dev_tools_message.into(),
            on_dev_tools_method_result: self.on_dev_tools_method_result.into(),
            on_dev_tools_event: self.on_dev_tools_event.into(),
            on_dev_tools_agent_attached: self.on_dev_tools_agent_attached.into(),
            on_dev_tools_agent_detached: self.on_dev_tools_agent_detached.into(),
        }
    }
}
impl Default for DevToolsMessageObserver {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_value_t] for more documentation.
#[derive(Clone)]
pub struct Value {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_owned: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_value_t,
            that: *mut _cef_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_equal: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_value_t,
            that: *mut _cef_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub copy: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> *mut _cef_value_t,
    >,
    pub get_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> cef_value_type_t,
    >,
    pub get_bool: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> ::std::os::raw::c_int,
    >,
    pub get_int: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> ::std::os::raw::c_int,
    >,
    pub get_double:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> f64>,
    pub get_string: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> cef_string_userfree_t,
    >,
    pub get_binary: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> *mut _cef_binary_value_t,
    >,
    pub get_dictionary: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> *mut _cef_dictionary_value_t,
    >,
    pub get_list: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> *mut _cef_list_value_t,
    >,
    pub set_null: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t) -> ::std::os::raw::c_int,
    >,
    pub set_bool: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_value_t,
            value: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_int: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_value_t,
            value: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_double: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_value_t, value: f64) -> ::std::os::raw::c_int,
    >,
    pub set_string: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_value_t,
            value: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_binary: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_value_t,
            value: *mut _cef_binary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_dictionary: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_value_t,
            value: *mut _cef_dictionary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_list: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_value_t,
            value: *mut _cef_list_value_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_value_t> for Value {
    fn from(value: _cef_value_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            is_owned: value.is_owned.into(),
            is_read_only: value.is_read_only.into(),
            is_same: value.is_same.into(),
            is_equal: value.is_equal.into(),
            copy: value.copy.into(),
            get_type: value.get_type.into(),
            get_bool: value.get_bool.into(),
            get_int: value.get_int.into(),
            get_double: value.get_double.into(),
            get_string: value.get_string.into(),
            get_binary: value.get_binary.into(),
            get_dictionary: value.get_dictionary.into(),
            get_list: value.get_list.into(),
            set_null: value.set_null.into(),
            set_bool: value.set_bool.into(),
            set_int: value.set_int.into(),
            set_double: value.set_double.into(),
            set_string: value.set_string.into(),
            set_binary: value.set_binary.into(),
            set_dictionary: value.set_dictionary.into(),
            set_list: value.set_list.into(),
        }
    }
}
impl Into<_cef_value_t> for Value {
    fn into(self) -> _cef_value_t {
        _cef_value_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            is_owned: self.is_owned.into(),
            is_read_only: self.is_read_only.into(),
            is_same: self.is_same.into(),
            is_equal: self.is_equal.into(),
            copy: self.copy.into(),
            get_type: self.get_type.into(),
            get_bool: self.get_bool.into(),
            get_int: self.get_int.into(),
            get_double: self.get_double.into(),
            get_string: self.get_string.into(),
            get_binary: self.get_binary.into(),
            get_dictionary: self.get_dictionary.into(),
            get_list: self.get_list.into(),
            set_null: self.set_null.into(),
            set_bool: self.set_bool.into(),
            set_int: self.set_int.into(),
            set_double: self.set_double.into(),
            set_string: self.set_string.into(),
            set_binary: self.set_binary.into(),
            set_dictionary: self.set_dictionary.into(),
            set_list: self.set_list.into(),
        }
    }
}
impl Default for Value {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_binary_value_t] for more documentation.
#[derive(Clone)]
pub struct BinaryValue {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_binary_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_owned: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_binary_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_binary_value_t,
            that: *mut _cef_binary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_equal: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_binary_value_t,
            that: *mut _cef_binary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub copy: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_binary_value_t) -> *mut _cef_binary_value_t,
    >,
    pub get_raw_data: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_binary_value_t,
        ) -> *const ::std::os::raw::c_void,
    >,
    pub get_size:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_binary_value_t) -> usize>,
    pub get_data: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_binary_value_t,
            buffer: *mut ::std::os::raw::c_void,
            buffer_size: usize,
            data_offset: usize,
        ) -> usize,
    >,
}
impl From<_cef_binary_value_t> for BinaryValue {
    fn from(value: _cef_binary_value_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            is_owned: value.is_owned.into(),
            is_same: value.is_same.into(),
            is_equal: value.is_equal.into(),
            copy: value.copy.into(),
            get_raw_data: value.get_raw_data.into(),
            get_size: value.get_size.into(),
            get_data: value.get_data.into(),
        }
    }
}
impl Into<_cef_binary_value_t> for BinaryValue {
    fn into(self) -> _cef_binary_value_t {
        _cef_binary_value_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            is_owned: self.is_owned.into(),
            is_same: self.is_same.into(),
            is_equal: self.is_equal.into(),
            copy: self.copy.into(),
            get_raw_data: self.get_raw_data.into(),
            get_size: self.get_size.into(),
            get_data: self.get_data.into(),
        }
    }
}
impl Default for BinaryValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_dictionary_value_t] for more documentation.
#[derive(Clone)]
pub struct DictionaryValue {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_dictionary_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_owned: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_dictionary_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_dictionary_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            that: *mut _cef_dictionary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_equal: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            that: *mut _cef_dictionary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub copy: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            exclude_empty_children: ::std::os::raw::c_int,
        ) -> *mut _cef_dictionary_value_t,
    >,
    pub get_size: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_dictionary_value_t) -> usize,
    >,
    pub clear: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_dictionary_value_t) -> ::std::os::raw::c_int,
    >,
    pub has_key: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_keys: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            keys: cef_string_list_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub remove: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_type: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> cef_value_type_t,
    >,
    pub get_value: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> *mut _cef_value_t,
    >,
    pub get_bool: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_int: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_double: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> f64,
    >,
    pub get_string: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> cef_string_userfree_t,
    >,
    pub get_binary: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> *mut _cef_binary_value_t,
    >,
    pub get_dictionary: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> *mut _cef_dictionary_value_t,
    >,
    pub get_list: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> *mut _cef_list_value_t,
    >,
    pub set_value: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
            value: *mut _cef_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_null: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_bool: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
            value: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_int: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
            value: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_double: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
            value: f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_string: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
            value: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_binary: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
            value: *mut _cef_binary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_dictionary: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
            value: *mut _cef_dictionary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_list: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dictionary_value_t,
            key: *const cef_string_t,
            value: *mut _cef_list_value_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_dictionary_value_t> for DictionaryValue {
    fn from(value: _cef_dictionary_value_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            is_owned: value.is_owned.into(),
            is_read_only: value.is_read_only.into(),
            is_same: value.is_same.into(),
            is_equal: value.is_equal.into(),
            copy: value.copy.into(),
            get_size: value.get_size.into(),
            clear: value.clear.into(),
            has_key: value.has_key.into(),
            get_keys: value.get_keys.into(),
            remove: value.remove.into(),
            get_type: value.get_type.into(),
            get_value: value.get_value.into(),
            get_bool: value.get_bool.into(),
            get_int: value.get_int.into(),
            get_double: value.get_double.into(),
            get_string: value.get_string.into(),
            get_binary: value.get_binary.into(),
            get_dictionary: value.get_dictionary.into(),
            get_list: value.get_list.into(),
            set_value: value.set_value.into(),
            set_null: value.set_null.into(),
            set_bool: value.set_bool.into(),
            set_int: value.set_int.into(),
            set_double: value.set_double.into(),
            set_string: value.set_string.into(),
            set_binary: value.set_binary.into(),
            set_dictionary: value.set_dictionary.into(),
            set_list: value.set_list.into(),
        }
    }
}
impl Into<_cef_dictionary_value_t> for DictionaryValue {
    fn into(self) -> _cef_dictionary_value_t {
        _cef_dictionary_value_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            is_owned: self.is_owned.into(),
            is_read_only: self.is_read_only.into(),
            is_same: self.is_same.into(),
            is_equal: self.is_equal.into(),
            copy: self.copy.into(),
            get_size: self.get_size.into(),
            clear: self.clear.into(),
            has_key: self.has_key.into(),
            get_keys: self.get_keys.into(),
            remove: self.remove.into(),
            get_type: self.get_type.into(),
            get_value: self.get_value.into(),
            get_bool: self.get_bool.into(),
            get_int: self.get_int.into(),
            get_double: self.get_double.into(),
            get_string: self.get_string.into(),
            get_binary: self.get_binary.into(),
            get_dictionary: self.get_dictionary.into(),
            get_list: self.get_list.into(),
            set_value: self.set_value.into(),
            set_null: self.set_null.into(),
            set_bool: self.set_bool.into(),
            set_int: self.set_int.into(),
            set_double: self.set_double.into(),
            set_string: self.set_string.into(),
            set_binary: self.set_binary.into(),
            set_dictionary: self.set_dictionary.into(),
            set_list: self.set_list.into(),
        }
    }
}
impl Default for DictionaryValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_list_value_t] for more documentation.
#[derive(Clone)]
pub struct ListValue {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_list_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_owned: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_list_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_list_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            that: *mut _cef_list_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_equal: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            that: *mut _cef_list_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub copy: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_list_value_t) -> *mut _cef_list_value_t,
    >,
    pub set_size: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            size: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_size:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_list_value_t) -> usize>,
    pub clear: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_list_value_t) -> ::std::os::raw::c_int,
    >,
    pub remove: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_list_value_t, index: usize) -> cef_value_type_t,
    >,
    pub get_value: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
        ) -> *mut _cef_value_t,
    >,
    pub get_bool: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_int: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_double: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_list_value_t, index: usize) -> f64,
    >,
    pub get_string: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
        ) -> cef_string_userfree_t,
    >,
    pub get_binary: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
        ) -> *mut _cef_binary_value_t,
    >,
    pub get_dictionary: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
        ) -> *mut _cef_dictionary_value_t,
    >,
    pub get_list: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
        ) -> *mut _cef_list_value_t,
    >,
    pub set_value: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
            value: *mut _cef_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_null: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_bool: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
            value: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_int: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
            value: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_double: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
            value: f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_string: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
            value: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_binary: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
            value: *mut _cef_binary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_dictionary: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
            value: *mut _cef_dictionary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_list: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_list_value_t,
            index: usize,
            value: *mut _cef_list_value_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_list_value_t> for ListValue {
    fn from(value: _cef_list_value_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            is_owned: value.is_owned.into(),
            is_read_only: value.is_read_only.into(),
            is_same: value.is_same.into(),
            is_equal: value.is_equal.into(),
            copy: value.copy.into(),
            set_size: value.set_size.into(),
            get_size: value.get_size.into(),
            clear: value.clear.into(),
            remove: value.remove.into(),
            get_type: value.get_type.into(),
            get_value: value.get_value.into(),
            get_bool: value.get_bool.into(),
            get_int: value.get_int.into(),
            get_double: value.get_double.into(),
            get_string: value.get_string.into(),
            get_binary: value.get_binary.into(),
            get_dictionary: value.get_dictionary.into(),
            get_list: value.get_list.into(),
            set_value: value.set_value.into(),
            set_null: value.set_null.into(),
            set_bool: value.set_bool.into(),
            set_int: value.set_int.into(),
            set_double: value.set_double.into(),
            set_string: value.set_string.into(),
            set_binary: value.set_binary.into(),
            set_dictionary: value.set_dictionary.into(),
            set_list: value.set_list.into(),
        }
    }
}
impl Into<_cef_list_value_t> for ListValue {
    fn into(self) -> _cef_list_value_t {
        _cef_list_value_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            is_owned: self.is_owned.into(),
            is_read_only: self.is_read_only.into(),
            is_same: self.is_same.into(),
            is_equal: self.is_equal.into(),
            copy: self.copy.into(),
            set_size: self.set_size.into(),
            get_size: self.get_size.into(),
            clear: self.clear.into(),
            remove: self.remove.into(),
            get_type: self.get_type.into(),
            get_value: self.get_value.into(),
            get_bool: self.get_bool.into(),
            get_int: self.get_int.into(),
            get_double: self.get_double.into(),
            get_string: self.get_string.into(),
            get_binary: self.get_binary.into(),
            get_dictionary: self.get_dictionary.into(),
            get_list: self.get_list.into(),
            set_value: self.set_value.into(),
            set_null: self.set_null.into(),
            set_bool: self.set_bool.into(),
            set_int: self.set_int.into(),
            set_double: self.set_double.into(),
            set_string: self.set_string.into(),
            set_binary: self.set_binary.into(),
            set_dictionary: self.set_dictionary.into(),
            set_list: self.set_list.into(),
        }
    }
}
impl Default for ListValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_image_t] for more documentation.
#[derive(Clone)]
pub struct Image {
    pub base: BaseRefCounted,
    pub is_empty: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_image_t) -> ::std::os::raw::c_int,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_image_t,
            that: *mut _cef_image_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub add_bitmap: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_image_t,
            scale_factor: f32,
            pixel_width: ::std::os::raw::c_int,
            pixel_height: ::std::os::raw::c_int,
            color_type: cef_color_type_t,
            alpha_type: cef_alpha_type_t,
            pixel_data: *const ::std::os::raw::c_void,
            pixel_data_size: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub add_png: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_image_t,
            scale_factor: f32,
            png_data: *const ::std::os::raw::c_void,
            png_data_size: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub add_jpeg: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_image_t,
            scale_factor: f32,
            jpeg_data: *const ::std::os::raw::c_void,
            jpeg_data_size: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_width:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_image_t) -> usize>,
    pub get_height:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_image_t) -> usize>,
    pub has_representation: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_image_t,
            scale_factor: f32,
        ) -> ::std::os::raw::c_int,
    >,
    pub remove_representation: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_image_t,
            scale_factor: f32,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_representation_info: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_image_t,
            scale_factor: f32,
            actual_scale_factor: *mut f32,
            pixel_width: *mut ::std::os::raw::c_int,
            pixel_height: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_as_bitmap: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_image_t,
            scale_factor: f32,
            color_type: cef_color_type_t,
            alpha_type: cef_alpha_type_t,
            pixel_width: *mut ::std::os::raw::c_int,
            pixel_height: *mut ::std::os::raw::c_int,
        ) -> *mut _cef_binary_value_t,
    >,
    pub get_as_png: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_image_t,
            scale_factor: f32,
            with_transparency: ::std::os::raw::c_int,
            pixel_width: *mut ::std::os::raw::c_int,
            pixel_height: *mut ::std::os::raw::c_int,
        ) -> *mut _cef_binary_value_t,
    >,
    pub get_as_jpeg: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_image_t,
            scale_factor: f32,
            quality: ::std::os::raw::c_int,
            pixel_width: *mut ::std::os::raw::c_int,
            pixel_height: *mut ::std::os::raw::c_int,
        ) -> *mut _cef_binary_value_t,
    >,
}
impl From<_cef_image_t> for Image {
    fn from(value: _cef_image_t) -> Self {
        Self {
            base: value.base.into(),
            is_empty: value.is_empty.into(),
            is_same: value.is_same.into(),
            add_bitmap: value.add_bitmap.into(),
            add_png: value.add_png.into(),
            add_jpeg: value.add_jpeg.into(),
            get_width: value.get_width.into(),
            get_height: value.get_height.into(),
            has_representation: value.has_representation.into(),
            remove_representation: value.remove_representation.into(),
            get_representation_info: value.get_representation_info.into(),
            get_as_bitmap: value.get_as_bitmap.into(),
            get_as_png: value.get_as_png.into(),
            get_as_jpeg: value.get_as_jpeg.into(),
        }
    }
}
impl Into<_cef_image_t> for Image {
    fn into(self) -> _cef_image_t {
        _cef_image_t {
            base: self.base.into(),
            is_empty: self.is_empty.into(),
            is_same: self.is_same.into(),
            add_bitmap: self.add_bitmap.into(),
            add_png: self.add_png.into(),
            add_jpeg: self.add_jpeg.into(),
            get_width: self.get_width.into(),
            get_height: self.get_height.into(),
            has_representation: self.has_representation.into(),
            remove_representation: self.remove_representation.into(),
            get_representation_info: self.get_representation_info.into(),
            get_as_bitmap: self.get_as_bitmap.into(),
            get_as_png: self.get_as_png.into(),
            get_as_jpeg: self.get_as_jpeg.into(),
        }
    }
}
impl Default for Image {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_read_handler_t] for more documentation.
#[derive(Clone)]
pub struct ReadHandler {
    pub base: BaseRefCounted,
    pub read: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_read_handler_t,
            ptr: *mut ::std::os::raw::c_void,
            size: usize,
            n: usize,
        ) -> usize,
    >,
    pub seek: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_read_handler_t,
            offset: i64,
            whence: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tell:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_read_handler_t) -> i64>,
    pub eof: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_read_handler_t) -> ::std::os::raw::c_int,
    >,
    pub may_block: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_read_handler_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_read_handler_t> for ReadHandler {
    fn from(value: _cef_read_handler_t) -> Self {
        Self {
            base: value.base.into(),
            read: value.read.into(),
            seek: value.seek.into(),
            tell: value.tell.into(),
            eof: value.eof.into(),
            may_block: value.may_block.into(),
        }
    }
}
impl Into<_cef_read_handler_t> for ReadHandler {
    fn into(self) -> _cef_read_handler_t {
        _cef_read_handler_t {
            base: self.base.into(),
            read: self.read.into(),
            seek: self.seek.into(),
            tell: self.tell.into(),
            eof: self.eof.into(),
            may_block: self.may_block.into(),
        }
    }
}
impl Default for ReadHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_stream_reader_t] for more documentation.
#[derive(Clone)]
pub struct StreamReader {
    pub base: BaseRefCounted,
    pub read: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_stream_reader_t,
            ptr: *mut ::std::os::raw::c_void,
            size: usize,
            n: usize,
        ) -> usize,
    >,
    pub seek: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_stream_reader_t,
            offset: i64,
            whence: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tell:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_stream_reader_t) -> i64>,
    pub eof: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_stream_reader_t) -> ::std::os::raw::c_int,
    >,
    pub may_block: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_stream_reader_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_stream_reader_t> for StreamReader {
    fn from(value: _cef_stream_reader_t) -> Self {
        Self {
            base: value.base.into(),
            read: value.read.into(),
            seek: value.seek.into(),
            tell: value.tell.into(),
            eof: value.eof.into(),
            may_block: value.may_block.into(),
        }
    }
}
impl Into<_cef_stream_reader_t> for StreamReader {
    fn into(self) -> _cef_stream_reader_t {
        _cef_stream_reader_t {
            base: self.base.into(),
            read: self.read.into(),
            seek: self.seek.into(),
            tell: self.tell.into(),
            eof: self.eof.into(),
            may_block: self.may_block.into(),
        }
    }
}
impl Default for StreamReader {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_write_handler_t] for more documentation.
#[derive(Clone)]
pub struct WriteHandler {
    pub base: BaseRefCounted,
    pub write: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_write_handler_t,
            ptr: *const ::std::os::raw::c_void,
            size: usize,
            n: usize,
        ) -> usize,
    >,
    pub seek: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_write_handler_t,
            offset: i64,
            whence: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tell:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_write_handler_t) -> i64>,
    pub flush: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_write_handler_t) -> ::std::os::raw::c_int,
    >,
    pub may_block: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_write_handler_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_write_handler_t> for WriteHandler {
    fn from(value: _cef_write_handler_t) -> Self {
        Self {
            base: value.base.into(),
            write: value.write.into(),
            seek: value.seek.into(),
            tell: value.tell.into(),
            flush: value.flush.into(),
            may_block: value.may_block.into(),
        }
    }
}
impl Into<_cef_write_handler_t> for WriteHandler {
    fn into(self) -> _cef_write_handler_t {
        _cef_write_handler_t {
            base: self.base.into(),
            write: self.write.into(),
            seek: self.seek.into(),
            tell: self.tell.into(),
            flush: self.flush.into(),
            may_block: self.may_block.into(),
        }
    }
}
impl Default for WriteHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_stream_writer_t] for more documentation.
#[derive(Clone)]
pub struct StreamWriter {
    pub base: BaseRefCounted,
    pub write: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_stream_writer_t,
            ptr: *const ::std::os::raw::c_void,
            size: usize,
            n: usize,
        ) -> usize,
    >,
    pub seek: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_stream_writer_t,
            offset: i64,
            whence: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tell:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_stream_writer_t) -> i64>,
    pub flush: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_stream_writer_t) -> ::std::os::raw::c_int,
    >,
    pub may_block: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_stream_writer_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_stream_writer_t> for StreamWriter {
    fn from(value: _cef_stream_writer_t) -> Self {
        Self {
            base: value.base.into(),
            write: value.write.into(),
            seek: value.seek.into(),
            tell: value.tell.into(),
            flush: value.flush.into(),
            may_block: value.may_block.into(),
        }
    }
}
impl Into<_cef_stream_writer_t> for StreamWriter {
    fn into(self) -> _cef_stream_writer_t {
        _cef_stream_writer_t {
            base: self.base.into(),
            write: self.write.into(),
            seek: self.seek.into(),
            tell: self.tell.into(),
            flush: self.flush.into(),
            may_block: self.may_block.into(),
        }
    }
}
impl Default for StreamWriter {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_drag_data_t] for more documentation.
#[derive(Clone)]
pub struct DragData {
    pub base: BaseRefCounted,
    pub clone: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> *mut _cef_drag_data_t,
    >,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> ::std::os::raw::c_int,
    >,
    pub is_link: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> ::std::os::raw::c_int,
    >,
    pub is_fragment: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> ::std::os::raw::c_int,
    >,
    pub is_file: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> ::std::os::raw::c_int,
    >,
    pub get_link_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> cef_string_userfree_t,
    >,
    pub get_link_title: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> cef_string_userfree_t,
    >,
    pub get_link_metadata: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> cef_string_userfree_t,
    >,
    pub get_fragment_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> cef_string_userfree_t,
    >,
    pub get_fragment_html: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> cef_string_userfree_t,
    >,
    pub get_fragment_base_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> cef_string_userfree_t,
    >,
    pub get_file_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> cef_string_userfree_t,
    >,
    pub get_file_contents: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_drag_data_t,
            writer: *mut _cef_stream_writer_t,
        ) -> usize,
    >,
    pub get_file_names: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_drag_data_t,
            names: cef_string_list_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_file_paths: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_drag_data_t,
            paths: cef_string_list_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_link_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t, url: *const cef_string_t),
    >,
    pub set_link_title: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t, title: *const cef_string_t),
    >,
    pub set_link_metadata: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t, data: *const cef_string_t),
    >,
    pub set_fragment_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t, text: *const cef_string_t),
    >,
    pub set_fragment_html: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t, html: *const cef_string_t),
    >,
    pub set_fragment_base_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t, base_url: *const cef_string_t),
    >,
    pub reset_file_contents:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t)>,
    pub add_file: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_drag_data_t,
            path: *const cef_string_t,
            display_name: *const cef_string_t,
        ),
    >,
    pub clear_filenames:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t)>,
    pub get_image: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> *mut _cef_image_t,
    >,
    pub get_image_hotspot: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> cef_point_t,
    >,
    pub has_image: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_drag_data_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_drag_data_t> for DragData {
    fn from(value: _cef_drag_data_t) -> Self {
        Self {
            base: value.base.into(),
            clone: value.clone.into(),
            is_read_only: value.is_read_only.into(),
            is_link: value.is_link.into(),
            is_fragment: value.is_fragment.into(),
            is_file: value.is_file.into(),
            get_link_url: value.get_link_url.into(),
            get_link_title: value.get_link_title.into(),
            get_link_metadata: value.get_link_metadata.into(),
            get_fragment_text: value.get_fragment_text.into(),
            get_fragment_html: value.get_fragment_html.into(),
            get_fragment_base_url: value.get_fragment_base_url.into(),
            get_file_name: value.get_file_name.into(),
            get_file_contents: value.get_file_contents.into(),
            get_file_names: value.get_file_names.into(),
            get_file_paths: value.get_file_paths.into(),
            set_link_url: value.set_link_url.into(),
            set_link_title: value.set_link_title.into(),
            set_link_metadata: value.set_link_metadata.into(),
            set_fragment_text: value.set_fragment_text.into(),
            set_fragment_html: value.set_fragment_html.into(),
            set_fragment_base_url: value.set_fragment_base_url.into(),
            reset_file_contents: value.reset_file_contents.into(),
            add_file: value.add_file.into(),
            clear_filenames: value.clear_filenames.into(),
            get_image: value.get_image.into(),
            get_image_hotspot: value.get_image_hotspot.into(),
            has_image: value.has_image.into(),
        }
    }
}
impl Into<_cef_drag_data_t> for DragData {
    fn into(self) -> _cef_drag_data_t {
        _cef_drag_data_t {
            base: self.base.into(),
            clone: self.clone.into(),
            is_read_only: self.is_read_only.into(),
            is_link: self.is_link.into(),
            is_fragment: self.is_fragment.into(),
            is_file: self.is_file.into(),
            get_link_url: self.get_link_url.into(),
            get_link_title: self.get_link_title.into(),
            get_link_metadata: self.get_link_metadata.into(),
            get_fragment_text: self.get_fragment_text.into(),
            get_fragment_html: self.get_fragment_html.into(),
            get_fragment_base_url: self.get_fragment_base_url.into(),
            get_file_name: self.get_file_name.into(),
            get_file_contents: self.get_file_contents.into(),
            get_file_names: self.get_file_names.into(),
            get_file_paths: self.get_file_paths.into(),
            set_link_url: self.set_link_url.into(),
            set_link_title: self.set_link_title.into(),
            set_link_metadata: self.set_link_metadata.into(),
            set_fragment_text: self.set_fragment_text.into(),
            set_fragment_html: self.set_fragment_html.into(),
            set_fragment_base_url: self.set_fragment_base_url.into(),
            reset_file_contents: self.reset_file_contents.into(),
            add_file: self.add_file.into(),
            clear_filenames: self.clear_filenames.into(),
            get_image: self.get_image.into(),
            get_image_hotspot: self.get_image_hotspot.into(),
            has_image: self.has_image.into(),
        }
    }
}
impl Default for DragData {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_domvisitor_t] for more documentation.
#[derive(Clone)]
pub struct Domvisitor {
    pub base: BaseRefCounted,
    pub visit: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_domvisitor_t,
            document: *mut _cef_domdocument_t,
        ),
    >,
}
impl From<_cef_domvisitor_t> for Domvisitor {
    fn from(value: _cef_domvisitor_t) -> Self {
        Self {
            base: value.base.into(),
            visit: value.visit.into(),
        }
    }
}
impl Into<_cef_domvisitor_t> for Domvisitor {
    fn into(self) -> _cef_domvisitor_t {
        _cef_domvisitor_t {
            base: self.base.into(),
            visit: self.visit.into(),
        }
    }
}
impl Default for Domvisitor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_domdocument_t] for more documentation.
#[derive(Clone)]
pub struct Domdocument {
    pub base: BaseRefCounted,
    pub get_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> cef_dom_document_type_t,
    >,
    pub get_document: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> *mut _cef_domnode_t,
    >,
    pub get_body: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> *mut _cef_domnode_t,
    >,
    pub get_head: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> *mut _cef_domnode_t,
    >,
    pub get_title: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> cef_string_userfree_t,
    >,
    pub get_element_by_id: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_domdocument_t,
            id: *const cef_string_t,
        ) -> *mut _cef_domnode_t,
    >,
    pub get_focused_node: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> *mut _cef_domnode_t,
    >,
    pub has_selection: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> ::std::os::raw::c_int,
    >,
    pub get_selection_start_offset: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> ::std::os::raw::c_int,
    >,
    pub get_selection_end_offset: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> ::std::os::raw::c_int,
    >,
    pub get_selection_as_markup: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> cef_string_userfree_t,
    >,
    pub get_selection_as_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> cef_string_userfree_t,
    >,
    pub get_base_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domdocument_t) -> cef_string_userfree_t,
    >,
    pub get_complete_url: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_domdocument_t,
            partialURL: *const cef_string_t,
        ) -> cef_string_userfree_t,
    >,
}
impl From<_cef_domdocument_t> for Domdocument {
    fn from(value: _cef_domdocument_t) -> Self {
        Self {
            base: value.base.into(),
            get_type: value.get_type.into(),
            get_document: value.get_document.into(),
            get_body: value.get_body.into(),
            get_head: value.get_head.into(),
            get_title: value.get_title.into(),
            get_element_by_id: value.get_element_by_id.into(),
            get_focused_node: value.get_focused_node.into(),
            has_selection: value.has_selection.into(),
            get_selection_start_offset: value.get_selection_start_offset.into(),
            get_selection_end_offset: value.get_selection_end_offset.into(),
            get_selection_as_markup: value.get_selection_as_markup.into(),
            get_selection_as_text: value.get_selection_as_text.into(),
            get_base_url: value.get_base_url.into(),
            get_complete_url: value.get_complete_url.into(),
        }
    }
}
impl Into<_cef_domdocument_t> for Domdocument {
    fn into(self) -> _cef_domdocument_t {
        _cef_domdocument_t {
            base: self.base.into(),
            get_type: self.get_type.into(),
            get_document: self.get_document.into(),
            get_body: self.get_body.into(),
            get_head: self.get_head.into(),
            get_title: self.get_title.into(),
            get_element_by_id: self.get_element_by_id.into(),
            get_focused_node: self.get_focused_node.into(),
            has_selection: self.has_selection.into(),
            get_selection_start_offset: self.get_selection_start_offset.into(),
            get_selection_end_offset: self.get_selection_end_offset.into(),
            get_selection_as_markup: self.get_selection_as_markup.into(),
            get_selection_as_text: self.get_selection_as_text.into(),
            get_base_url: self.get_base_url.into(),
            get_complete_url: self.get_complete_url.into(),
        }
    }
}
impl Default for Domdocument {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_domnode_t] for more documentation.
#[derive(Clone)]
pub struct Domnode {
    pub base: BaseRefCounted,
    pub get_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> cef_dom_node_type_t,
    >,
    pub is_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> ::std::os::raw::c_int,
    >,
    pub is_element: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> ::std::os::raw::c_int,
    >,
    pub is_editable: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> ::std::os::raw::c_int,
    >,
    pub is_form_control_element: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> ::std::os::raw::c_int,
    >,
    pub get_form_control_element_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> cef_dom_form_control_type_t,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_domnode_t,
            that: *mut _cef_domnode_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> cef_string_userfree_t,
    >,
    pub get_value: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> cef_string_userfree_t,
    >,
    pub set_value: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_domnode_t,
            value: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_as_markup: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> cef_string_userfree_t,
    >,
    pub get_document: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> *mut _cef_domdocument_t,
    >,
    pub get_parent: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> *mut _cef_domnode_t,
    >,
    pub get_previous_sibling: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> *mut _cef_domnode_t,
    >,
    pub get_next_sibling: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> *mut _cef_domnode_t,
    >,
    pub has_children: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> ::std::os::raw::c_int,
    >,
    pub get_first_child: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> *mut _cef_domnode_t,
    >,
    pub get_last_child: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> *mut _cef_domnode_t,
    >,
    pub get_element_tag_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> cef_string_userfree_t,
    >,
    pub has_element_attributes: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> ::std::os::raw::c_int,
    >,
    pub has_element_attribute: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_domnode_t,
            attrName: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_element_attribute: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_domnode_t,
            attrName: *const cef_string_t,
        ) -> cef_string_userfree_t,
    >,
    pub get_element_attributes: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t, attrMap: cef_string_map_t),
    >,
    pub set_element_attribute: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_domnode_t,
            attrName: *const cef_string_t,
            value: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_element_inner_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> cef_string_userfree_t,
    >,
    pub get_element_bounds:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_domnode_t) -> cef_rect_t>,
}
impl From<_cef_domnode_t> for Domnode {
    fn from(value: _cef_domnode_t) -> Self {
        Self {
            base: value.base.into(),
            get_type: value.get_type.into(),
            is_text: value.is_text.into(),
            is_element: value.is_element.into(),
            is_editable: value.is_editable.into(),
            is_form_control_element: value.is_form_control_element.into(),
            get_form_control_element_type: value.get_form_control_element_type.into(),
            is_same: value.is_same.into(),
            get_name: value.get_name.into(),
            get_value: value.get_value.into(),
            set_value: value.set_value.into(),
            get_as_markup: value.get_as_markup.into(),
            get_document: value.get_document.into(),
            get_parent: value.get_parent.into(),
            get_previous_sibling: value.get_previous_sibling.into(),
            get_next_sibling: value.get_next_sibling.into(),
            has_children: value.has_children.into(),
            get_first_child: value.get_first_child.into(),
            get_last_child: value.get_last_child.into(),
            get_element_tag_name: value.get_element_tag_name.into(),
            has_element_attributes: value.has_element_attributes.into(),
            has_element_attribute: value.has_element_attribute.into(),
            get_element_attribute: value.get_element_attribute.into(),
            get_element_attributes: value.get_element_attributes.into(),
            set_element_attribute: value.set_element_attribute.into(),
            get_element_inner_text: value.get_element_inner_text.into(),
            get_element_bounds: value.get_element_bounds.into(),
        }
    }
}
impl Into<_cef_domnode_t> for Domnode {
    fn into(self) -> _cef_domnode_t {
        _cef_domnode_t {
            base: self.base.into(),
            get_type: self.get_type.into(),
            is_text: self.is_text.into(),
            is_element: self.is_element.into(),
            is_editable: self.is_editable.into(),
            is_form_control_element: self.is_form_control_element.into(),
            get_form_control_element_type: self.get_form_control_element_type.into(),
            is_same: self.is_same.into(),
            get_name: self.get_name.into(),
            get_value: self.get_value.into(),
            set_value: self.set_value.into(),
            get_as_markup: self.get_as_markup.into(),
            get_document: self.get_document.into(),
            get_parent: self.get_parent.into(),
            get_previous_sibling: self.get_previous_sibling.into(),
            get_next_sibling: self.get_next_sibling.into(),
            has_children: self.has_children.into(),
            get_first_child: self.get_first_child.into(),
            get_last_child: self.get_last_child.into(),
            get_element_tag_name: self.get_element_tag_name.into(),
            has_element_attributes: self.has_element_attributes.into(),
            has_element_attribute: self.has_element_attribute.into(),
            get_element_attribute: self.get_element_attribute.into(),
            get_element_attributes: self.get_element_attributes.into(),
            set_element_attribute: self.set_element_attribute.into(),
            get_element_inner_text: self.get_element_inner_text.into(),
            get_element_bounds: self.get_element_bounds.into(),
        }
    }
}
impl Default for Domnode {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_shared_memory_region_t] for more documentation.
#[derive(Clone)]
pub struct SharedMemoryRegion {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_shared_memory_region_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub size: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_shared_memory_region_t) -> usize,
    >,
    pub memory: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_shared_memory_region_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
impl From<_cef_shared_memory_region_t> for SharedMemoryRegion {
    fn from(value: _cef_shared_memory_region_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            size: value.size.into(),
            memory: value.memory.into(),
        }
    }
}
impl Into<_cef_shared_memory_region_t> for SharedMemoryRegion {
    fn into(self) -> _cef_shared_memory_region_t {
        _cef_shared_memory_region_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            size: self.size.into(),
            memory: self.memory.into(),
        }
    }
}
impl Default for SharedMemoryRegion {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_process_message_t] for more documentation.
#[derive(Clone)]
pub struct ProcessMessage {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_process_message_t) -> ::std::os::raw::c_int,
    >,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_process_message_t) -> ::std::os::raw::c_int,
    >,
    pub copy: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_process_message_t,
        ) -> *mut _cef_process_message_t,
    >,
    pub get_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_process_message_t) -> cef_string_userfree_t,
    >,
    pub get_argument_list: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_process_message_t) -> *mut _cef_list_value_t,
    >,
    pub get_shared_memory_region: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_process_message_t,
        ) -> *mut _cef_shared_memory_region_t,
    >,
}
impl From<_cef_process_message_t> for ProcessMessage {
    fn from(value: _cef_process_message_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            is_read_only: value.is_read_only.into(),
            copy: value.copy.into(),
            get_name: value.get_name.into(),
            get_argument_list: value.get_argument_list.into(),
            get_shared_memory_region: value.get_shared_memory_region.into(),
        }
    }
}
impl Into<_cef_process_message_t> for ProcessMessage {
    fn into(self) -> _cef_process_message_t {
        _cef_process_message_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            is_read_only: self.is_read_only.into(),
            copy: self.copy.into(),
            get_name: self.get_name.into(),
            get_argument_list: self.get_argument_list.into(),
            get_shared_memory_region: self.get_shared_memory_region.into(),
        }
    }
}
impl Default for ProcessMessage {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_request_t] for more documentation.
#[derive(Clone)]
pub struct Request {
    pub base: BaseRefCounted,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> ::std::os::raw::c_int,
    >,
    pub get_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> cef_string_userfree_t,
    >,
    pub set_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t, url: *const cef_string_t),
    >,
    pub get_method: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> cef_string_userfree_t,
    >,
    pub set_method: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t, method: *const cef_string_t),
    >,
    pub set_referrer: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_t,
            referrer_url: *const cef_string_t,
            policy: cef_referrer_policy_t,
        ),
    >,
    pub get_referrer_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> cef_string_userfree_t,
    >,
    pub get_referrer_policy: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> cef_referrer_policy_t,
    >,
    pub get_post_data: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> *mut _cef_post_data_t,
    >,
    pub set_post_data: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t, postData: *mut _cef_post_data_t),
    >,
    pub get_header_map: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t, headerMap: cef_string_multimap_t),
    >,
    pub set_header_map: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t, headerMap: cef_string_multimap_t),
    >,
    pub get_header_by_name: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_t,
            name: *const cef_string_t,
        ) -> cef_string_userfree_t,
    >,
    pub set_header_by_name: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_t,
            name: *const cef_string_t,
            value: *const cef_string_t,
            overwrite: ::std::os::raw::c_int,
        ),
    >,
    pub set: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_t,
            url: *const cef_string_t,
            method: *const cef_string_t,
            postData: *mut _cef_post_data_t,
            headerMap: cef_string_multimap_t,
        ),
    >,
    pub get_flags: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> ::std::os::raw::c_int,
    >,
    pub set_flags: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t, flags: ::std::os::raw::c_int),
    >,
    pub get_first_party_for_cookies: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> cef_string_userfree_t,
    >,
    pub set_first_party_for_cookies: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t, url: *const cef_string_t),
    >,
    pub get_resource_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> cef_resource_type_t,
    >,
    pub get_transition_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> cef_transition_type_t,
    >,
    pub get_identifier:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_request_t) -> u64>,
}
impl From<_cef_request_t> for Request {
    fn from(value: _cef_request_t) -> Self {
        Self {
            base: value.base.into(),
            is_read_only: value.is_read_only.into(),
            get_url: value.get_url.into(),
            set_url: value.set_url.into(),
            get_method: value.get_method.into(),
            set_method: value.set_method.into(),
            set_referrer: value.set_referrer.into(),
            get_referrer_url: value.get_referrer_url.into(),
            get_referrer_policy: value.get_referrer_policy.into(),
            get_post_data: value.get_post_data.into(),
            set_post_data: value.set_post_data.into(),
            get_header_map: value.get_header_map.into(),
            set_header_map: value.set_header_map.into(),
            get_header_by_name: value.get_header_by_name.into(),
            set_header_by_name: value.set_header_by_name.into(),
            set: value.set.into(),
            get_flags: value.get_flags.into(),
            set_flags: value.set_flags.into(),
            get_first_party_for_cookies: value.get_first_party_for_cookies.into(),
            set_first_party_for_cookies: value.set_first_party_for_cookies.into(),
            get_resource_type: value.get_resource_type.into(),
            get_transition_type: value.get_transition_type.into(),
            get_identifier: value.get_identifier.into(),
        }
    }
}
impl Into<_cef_request_t> for Request {
    fn into(self) -> _cef_request_t {
        _cef_request_t {
            base: self.base.into(),
            is_read_only: self.is_read_only.into(),
            get_url: self.get_url.into(),
            set_url: self.set_url.into(),
            get_method: self.get_method.into(),
            set_method: self.set_method.into(),
            set_referrer: self.set_referrer.into(),
            get_referrer_url: self.get_referrer_url.into(),
            get_referrer_policy: self.get_referrer_policy.into(),
            get_post_data: self.get_post_data.into(),
            set_post_data: self.set_post_data.into(),
            get_header_map: self.get_header_map.into(),
            set_header_map: self.set_header_map.into(),
            get_header_by_name: self.get_header_by_name.into(),
            set_header_by_name: self.set_header_by_name.into(),
            set: self.set.into(),
            get_flags: self.get_flags.into(),
            set_flags: self.set_flags.into(),
            get_first_party_for_cookies: self.get_first_party_for_cookies.into(),
            set_first_party_for_cookies: self.set_first_party_for_cookies.into(),
            get_resource_type: self.get_resource_type.into(),
            get_transition_type: self.get_transition_type.into(),
            get_identifier: self.get_identifier.into(),
        }
    }
}
impl Default for Request {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_post_data_t] for more documentation.
#[derive(Clone)]
pub struct PostData {
    pub base: BaseRefCounted,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_post_data_t) -> ::std::os::raw::c_int,
    >,
    pub has_excluded_elements: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_post_data_t) -> ::std::os::raw::c_int,
    >,
    pub get_element_count:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_post_data_t) -> usize>,
    pub get_elements: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_post_data_t,
            elementsCount: *mut usize,
            elements: *mut *mut _cef_post_data_element_t,
        ),
    >,
    pub remove_element: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_post_data_t,
            element: *mut _cef_post_data_element_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub add_element: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_post_data_t,
            element: *mut _cef_post_data_element_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub remove_elements:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_post_data_t)>,
}
impl From<_cef_post_data_t> for PostData {
    fn from(value: _cef_post_data_t) -> Self {
        Self {
            base: value.base.into(),
            is_read_only: value.is_read_only.into(),
            has_excluded_elements: value.has_excluded_elements.into(),
            get_element_count: value.get_element_count.into(),
            get_elements: value.get_elements.into(),
            remove_element: value.remove_element.into(),
            add_element: value.add_element.into(),
            remove_elements: value.remove_elements.into(),
        }
    }
}
impl Into<_cef_post_data_t> for PostData {
    fn into(self) -> _cef_post_data_t {
        _cef_post_data_t {
            base: self.base.into(),
            is_read_only: self.is_read_only.into(),
            has_excluded_elements: self.has_excluded_elements.into(),
            get_element_count: self.get_element_count.into(),
            get_elements: self.get_elements.into(),
            remove_element: self.remove_element.into(),
            add_element: self.add_element.into(),
            remove_elements: self.remove_elements.into(),
        }
    }
}
impl Default for PostData {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_post_data_element_t] for more documentation.
#[derive(Clone)]
pub struct PostDataElement {
    pub base: BaseRefCounted,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_post_data_element_t) -> ::std::os::raw::c_int,
    >,
    pub set_to_empty:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_post_data_element_t)>,
    pub set_to_file: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_post_data_element_t,
            fileName: *const cef_string_t,
        ),
    >,
    pub set_to_bytes: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_post_data_element_t,
            size: usize,
            bytes: *const ::std::os::raw::c_void,
        ),
    >,
    pub get_type: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_post_data_element_t,
        ) -> cef_postdataelement_type_t,
    >,
    pub get_file: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_post_data_element_t) -> cef_string_userfree_t,
    >,
    pub get_bytes_count: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_post_data_element_t) -> usize,
    >,
    pub get_bytes: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_post_data_element_t,
            size: usize,
            bytes: *mut ::std::os::raw::c_void,
        ) -> usize,
    >,
}
impl From<_cef_post_data_element_t> for PostDataElement {
    fn from(value: _cef_post_data_element_t) -> Self {
        Self {
            base: value.base.into(),
            is_read_only: value.is_read_only.into(),
            set_to_empty: value.set_to_empty.into(),
            set_to_file: value.set_to_file.into(),
            set_to_bytes: value.set_to_bytes.into(),
            get_type: value.get_type.into(),
            get_file: value.get_file.into(),
            get_bytes_count: value.get_bytes_count.into(),
            get_bytes: value.get_bytes.into(),
        }
    }
}
impl Into<_cef_post_data_element_t> for PostDataElement {
    fn into(self) -> _cef_post_data_element_t {
        _cef_post_data_element_t {
            base: self.base.into(),
            is_read_only: self.is_read_only.into(),
            set_to_empty: self.set_to_empty.into(),
            set_to_file: self.set_to_file.into(),
            set_to_bytes: self.set_to_bytes.into(),
            get_type: self.get_type.into(),
            get_file: self.get_file.into(),
            get_bytes_count: self.get_bytes_count.into(),
            get_bytes: self.get_bytes.into(),
        }
    }
}
impl Default for PostDataElement {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_string_visitor_t] for more documentation.
#[derive(Clone)]
pub struct CefStringVisitor {
    pub base: BaseRefCounted,
    pub visit: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_string_visitor_t, string: *const cef_string_t),
    >,
}
impl From<_cef_string_visitor_t> for CefStringVisitor {
    fn from(value: _cef_string_visitor_t) -> Self {
        Self {
            base: value.base.into(),
            visit: value.visit.into(),
        }
    }
}
impl Into<_cef_string_visitor_t> for CefStringVisitor {
    fn into(self) -> _cef_string_visitor_t {
        _cef_string_visitor_t {
            base: self.base.into(),
            visit: self.visit.into(),
        }
    }
}
impl Default for CefStringVisitor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_frame_t] for more documentation.
#[derive(Clone)]
pub struct Frame {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t) -> ::std::os::raw::c_int,
    >,
    pub undo: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_frame_t)>,
    pub redo: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_frame_t)>,
    pub cut: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_frame_t)>,
    pub copy: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_frame_t)>,
    pub paste: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_frame_t)>,
    pub paste_and_match_style:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_frame_t)>,
    pub del: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_frame_t)>,
    pub select_all: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_frame_t)>,
    pub view_source: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_frame_t)>,
    pub get_source: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t, visitor: *mut _cef_string_visitor_t),
    >,
    pub get_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t, visitor: *mut _cef_string_visitor_t),
    >,
    pub load_request: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t, request: *mut _cef_request_t),
    >,
    pub load_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t, url: *const cef_string_t),
    >,
    pub execute_java_script: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_frame_t,
            code: *const cef_string_t,
            script_url: *const cef_string_t,
            start_line: ::std::os::raw::c_int,
        ),
    >,
    pub is_main: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t) -> ::std::os::raw::c_int,
    >,
    pub is_focused: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t) -> ::std::os::raw::c_int,
    >,
    pub get_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t) -> cef_string_userfree_t,
    >,
    pub get_identifier: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t) -> cef_string_userfree_t,
    >,
    pub get_parent: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t) -> *mut _cef_frame_t,
    >,
    pub get_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t) -> cef_string_userfree_t,
    >,
    pub get_browser: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t) -> *mut _cef_browser_t,
    >,
    pub get_v_8_context: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t) -> *mut _cef_v8_context_t,
    >,
    pub visit_dom: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_frame_t, visitor: *mut _cef_domvisitor_t),
    >,
    pub create_urlrequest: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            client: *mut _cef_urlrequest_client_t,
        ) -> *mut _cef_urlrequest_t,
    >,
    pub send_process_message: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_frame_t,
            target_process: cef_process_id_t,
            message: *mut _cef_process_message_t,
        ),
    >,
}
impl From<_cef_frame_t> for Frame {
    fn from(value: _cef_frame_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            undo: value.undo.into(),
            redo: value.redo.into(),
            cut: value.cut.into(),
            copy: value.copy.into(),
            paste: value.paste.into(),
            paste_and_match_style: value.paste_and_match_style.into(),
            del: value.del.into(),
            select_all: value.select_all.into(),
            view_source: value.view_source.into(),
            get_source: value.get_source.into(),
            get_text: value.get_text.into(),
            load_request: value.load_request.into(),
            load_url: value.load_url.into(),
            execute_java_script: value.execute_java_script.into(),
            is_main: value.is_main.into(),
            is_focused: value.is_focused.into(),
            get_name: value.get_name.into(),
            get_identifier: value.get_identifier.into(),
            get_parent: value.get_parent.into(),
            get_url: value.get_url.into(),
            get_browser: value.get_browser.into(),
            get_v_8_context: value.get_v8_context.into(),
            visit_dom: value.visit_dom.into(),
            create_urlrequest: value.create_urlrequest.into(),
            send_process_message: value.send_process_message.into(),
        }
    }
}
impl Into<_cef_frame_t> for Frame {
    fn into(self) -> _cef_frame_t {
        _cef_frame_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            undo: self.undo.into(),
            redo: self.redo.into(),
            cut: self.cut.into(),
            copy: self.copy.into(),
            paste: self.paste.into(),
            paste_and_match_style: self.paste_and_match_style.into(),
            del: self.del.into(),
            select_all: self.select_all.into(),
            view_source: self.view_source.into(),
            get_source: self.get_source.into(),
            get_text: self.get_text.into(),
            load_request: self.load_request.into(),
            load_url: self.load_url.into(),
            execute_java_script: self.execute_java_script.into(),
            is_main: self.is_main.into(),
            is_focused: self.is_focused.into(),
            get_name: self.get_name.into(),
            get_identifier: self.get_identifier.into(),
            get_parent: self.get_parent.into(),
            get_url: self.get_url.into(),
            get_browser: self.get_browser.into(),
            get_v8_context: self.get_v_8_context.into(),
            visit_dom: self.visit_dom.into(),
            create_urlrequest: self.create_urlrequest.into(),
            send_process_message: self.send_process_message.into(),
        }
    }
}
impl Default for Frame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_x509_cert_principal_t] for more documentation.
#[derive(Clone)]
pub struct X509CertPrincipal {
    pub base: BaseRefCounted,
    pub get_display_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_cert_principal_t) -> cef_string_userfree_t,
    >,
    pub get_common_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_cert_principal_t) -> cef_string_userfree_t,
    >,
    pub get_locality_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_cert_principal_t) -> cef_string_userfree_t,
    >,
    pub get_state_or_province_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_cert_principal_t) -> cef_string_userfree_t,
    >,
    pub get_country_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_cert_principal_t) -> cef_string_userfree_t,
    >,
    pub get_organization_names: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_x509_cert_principal_t,
            names: cef_string_list_t,
        ),
    >,
    pub get_organization_unit_names: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_x509_cert_principal_t,
            names: cef_string_list_t,
        ),
    >,
}
impl From<_cef_x509_cert_principal_t> for X509CertPrincipal {
    fn from(value: _cef_x509_cert_principal_t) -> Self {
        Self {
            base: value.base.into(),
            get_display_name: value.get_display_name.into(),
            get_common_name: value.get_common_name.into(),
            get_locality_name: value.get_locality_name.into(),
            get_state_or_province_name: value.get_state_or_province_name.into(),
            get_country_name: value.get_country_name.into(),
            get_organization_names: value.get_organization_names.into(),
            get_organization_unit_names: value.get_organization_unit_names.into(),
        }
    }
}
impl Into<_cef_x509_cert_principal_t> for X509CertPrincipal {
    fn into(self) -> _cef_x509_cert_principal_t {
        _cef_x509_cert_principal_t {
            base: self.base.into(),
            get_display_name: self.get_display_name.into(),
            get_common_name: self.get_common_name.into(),
            get_locality_name: self.get_locality_name.into(),
            get_state_or_province_name: self.get_state_or_province_name.into(),
            get_country_name: self.get_country_name.into(),
            get_organization_names: self.get_organization_names.into(),
            get_organization_unit_names: self.get_organization_unit_names.into(),
        }
    }
}
impl Default for X509CertPrincipal {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_x509_certificate_t] for more documentation.
#[derive(Clone)]
pub struct X509Certificate {
    pub base: BaseRefCounted,
    pub get_subject: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_x509_certificate_t,
        ) -> *mut _cef_x509_cert_principal_t,
    >,
    pub get_issuer: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_x509_certificate_t,
        ) -> *mut _cef_x509_cert_principal_t,
    >,
    pub get_serial_number: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_certificate_t) -> *mut _cef_binary_value_t,
    >,
    pub get_valid_start: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_certificate_t) -> cef_basetime_t,
    >,
    pub get_valid_expiry: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_certificate_t) -> cef_basetime_t,
    >,
    pub get_derencoded: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_certificate_t) -> *mut _cef_binary_value_t,
    >,
    pub get_pemencoded: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_certificate_t) -> *mut _cef_binary_value_t,
    >,
    pub get_issuer_chain_size: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_x509_certificate_t) -> usize,
    >,
    pub get_derencoded_issuer_chain: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_x509_certificate_t,
            chainCount: *mut usize,
            chain: *mut *mut _cef_binary_value_t,
        ),
    >,
    pub get_pemencoded_issuer_chain: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_x509_certificate_t,
            chainCount: *mut usize,
            chain: *mut *mut _cef_binary_value_t,
        ),
    >,
}
impl From<_cef_x509_certificate_t> for X509Certificate {
    fn from(value: _cef_x509_certificate_t) -> Self {
        Self {
            base: value.base.into(),
            get_subject: value.get_subject.into(),
            get_issuer: value.get_issuer.into(),
            get_serial_number: value.get_serial_number.into(),
            get_valid_start: value.get_valid_start.into(),
            get_valid_expiry: value.get_valid_expiry.into(),
            get_derencoded: value.get_derencoded.into(),
            get_pemencoded: value.get_pemencoded.into(),
            get_issuer_chain_size: value.get_issuer_chain_size.into(),
            get_derencoded_issuer_chain: value.get_derencoded_issuer_chain.into(),
            get_pemencoded_issuer_chain: value.get_pemencoded_issuer_chain.into(),
        }
    }
}
impl Into<_cef_x509_certificate_t> for X509Certificate {
    fn into(self) -> _cef_x509_certificate_t {
        _cef_x509_certificate_t {
            base: self.base.into(),
            get_subject: self.get_subject.into(),
            get_issuer: self.get_issuer.into(),
            get_serial_number: self.get_serial_number.into(),
            get_valid_start: self.get_valid_start.into(),
            get_valid_expiry: self.get_valid_expiry.into(),
            get_derencoded: self.get_derencoded.into(),
            get_pemencoded: self.get_pemencoded.into(),
            get_issuer_chain_size: self.get_issuer_chain_size.into(),
            get_derencoded_issuer_chain: self.get_derencoded_issuer_chain.into(),
            get_pemencoded_issuer_chain: self.get_pemencoded_issuer_chain.into(),
        }
    }
}
impl Default for X509Certificate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_sslstatus_t] for more documentation.
#[derive(Clone)]
pub struct Sslstatus {
    pub base: BaseRefCounted,
    pub is_secure_connection: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_sslstatus_t) -> ::std::os::raw::c_int,
    >,
    pub get_cert_status: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_sslstatus_t) -> cef_cert_status_t,
    >,
    pub get_sslversion: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_sslstatus_t) -> cef_ssl_version_t,
    >,
    pub get_content_status: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_sslstatus_t) -> cef_ssl_content_status_t,
    >,
    pub get_x_509_certificate: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_sslstatus_t) -> *mut _cef_x509_certificate_t,
    >,
}
impl From<_cef_sslstatus_t> for Sslstatus {
    fn from(value: _cef_sslstatus_t) -> Self {
        Self {
            base: value.base.into(),
            is_secure_connection: value.is_secure_connection.into(),
            get_cert_status: value.get_cert_status.into(),
            get_sslversion: value.get_sslversion.into(),
            get_content_status: value.get_content_status.into(),
            get_x_509_certificate: value.get_x509_certificate.into(),
        }
    }
}
impl Into<_cef_sslstatus_t> for Sslstatus {
    fn into(self) -> _cef_sslstatus_t {
        _cef_sslstatus_t {
            base: self.base.into(),
            is_secure_connection: self.is_secure_connection.into(),
            get_cert_status: self.get_cert_status.into(),
            get_sslversion: self.get_sslversion.into(),
            get_content_status: self.get_content_status.into(),
            get_x509_certificate: self.get_x_509_certificate.into(),
        }
    }
}
impl Default for Sslstatus {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_navigation_entry_t] for more documentation.
#[derive(Clone)]
pub struct NavigationEntry {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_navigation_entry_t) -> ::std::os::raw::c_int,
    >,
    pub get_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_navigation_entry_t) -> cef_string_userfree_t,
    >,
    pub get_display_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_navigation_entry_t) -> cef_string_userfree_t,
    >,
    pub get_original_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_navigation_entry_t) -> cef_string_userfree_t,
    >,
    pub get_title: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_navigation_entry_t) -> cef_string_userfree_t,
    >,
    pub get_transition_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_navigation_entry_t) -> cef_transition_type_t,
    >,
    pub has_post_data: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_navigation_entry_t) -> ::std::os::raw::c_int,
    >,
    pub get_completion_time: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_navigation_entry_t) -> cef_basetime_t,
    >,
    pub get_http_status_code: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_navigation_entry_t) -> ::std::os::raw::c_int,
    >,
    pub get_sslstatus: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_navigation_entry_t) -> *mut _cef_sslstatus_t,
    >,
}
impl From<_cef_navigation_entry_t> for NavigationEntry {
    fn from(value: _cef_navigation_entry_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            get_url: value.get_url.into(),
            get_display_url: value.get_display_url.into(),
            get_original_url: value.get_original_url.into(),
            get_title: value.get_title.into(),
            get_transition_type: value.get_transition_type.into(),
            has_post_data: value.has_post_data.into(),
            get_completion_time: value.get_completion_time.into(),
            get_http_status_code: value.get_http_status_code.into(),
            get_sslstatus: value.get_sslstatus.into(),
        }
    }
}
impl Into<_cef_navigation_entry_t> for NavigationEntry {
    fn into(self) -> _cef_navigation_entry_t {
        _cef_navigation_entry_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            get_url: self.get_url.into(),
            get_display_url: self.get_display_url.into(),
            get_original_url: self.get_original_url.into(),
            get_title: self.get_title.into(),
            get_transition_type: self.get_transition_type.into(),
            has_post_data: self.has_post_data.into(),
            get_completion_time: self.get_completion_time.into(),
            get_http_status_code: self.get_http_status_code.into(),
            get_sslstatus: self.get_sslstatus.into(),
        }
    }
}
impl Default for NavigationEntry {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_registration_t] for more documentation.
#[derive(Clone)]
pub struct Registration(RefGuard<_cef_registration_t>);
pub trait ImplRegistration: Clone + Sized + Rc {
    fn get_raw(&self) -> *mut _cef_registration_t;
}
impl ImplRegistration for Registration {
    fn get_raw(&self) -> *mut _cef_registration_t {
        unsafe { RefGuard::as_raw(&self.0) }
    }
}
impl Rc for _cef_registration_t {
    fn as_base(&self) -> &_cef_base_ref_counted_t {
        self.base.as_base()
    }
}
impl Rc for Registration {
    fn as_base(&self) -> &_cef_base_ref_counted_t {
        self.0.as_base()
    }
}
impl ConvertParam<*mut _cef_registration_t> for &Registration {
    fn as_raw(self) -> *mut _cef_registration_t {
        ImplRegistration::get_raw(self)
    }
}
impl ConvertParam<*mut _cef_registration_t> for &mut Registration {
    fn as_raw(self) -> *mut _cef_registration_t {
        ImplRegistration::get_raw(self)
    }
}
impl ConvertReturnValue<Registration> for *mut _cef_registration_t {
    fn as_wrapper(self) -> Registration {
        Registration(unsafe { RefGuard::from_raw(self) })
    }
}
impl Into<*mut _cef_registration_t> for Registration {
    fn into(self) -> *mut _cef_registration_t {
        let object = ImplRegistration::get_raw(&self);
        std::mem::forget(self);
        object
    }
}
impl Default for Registration {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_callback_t] for more documentation.
#[derive(Clone)]
pub struct Callback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_callback_t)>,
    pub cancel: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_callback_t)>,
}
impl From<_cef_callback_t> for Callback {
    fn from(value: _cef_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
            cancel: value.cancel.into(),
        }
    }
}
impl Into<_cef_callback_t> for Callback {
    fn into(self) -> _cef_callback_t {
        _cef_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
            cancel: self.cancel.into(),
        }
    }
}
impl Default for Callback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_completion_callback_t] for more documentation.
#[derive(Clone)]
pub struct CompletionCallback {
    pub base: BaseRefCounted,
    pub on_complete:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_completion_callback_t)>,
}
impl From<_cef_completion_callback_t> for CompletionCallback {
    fn from(value: _cef_completion_callback_t) -> Self {
        Self {
            base: value.base.into(),
            on_complete: value.on_complete.into(),
        }
    }
}
impl Into<_cef_completion_callback_t> for CompletionCallback {
    fn into(self) -> _cef_completion_callback_t {
        _cef_completion_callback_t {
            base: self.base.into(),
            on_complete: self.on_complete.into(),
        }
    }
}
impl Default for CompletionCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_cookie_manager_t] for more documentation.
#[derive(Clone)]
pub struct CookieManager {
    pub base: BaseRefCounted,
    pub visit_all_cookies: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_cookie_manager_t,
            visitor: *mut _cef_cookie_visitor_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub visit_url_cookies: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_cookie_manager_t,
            url: *const cef_string_t,
            includeHttpOnly: ::std::os::raw::c_int,
            visitor: *mut _cef_cookie_visitor_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_cookie: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_cookie_manager_t,
            url: *const cef_string_t,
            cookie: *const _cef_cookie_t,
            callback: *mut _cef_set_cookie_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub delete_cookies: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_cookie_manager_t,
            url: *const cef_string_t,
            cookie_name: *const cef_string_t,
            callback: *mut _cef_delete_cookies_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub flush_store: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_cookie_manager_t,
            callback: *mut _cef_completion_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_cookie_manager_t> for CookieManager {
    fn from(value: _cef_cookie_manager_t) -> Self {
        Self {
            base: value.base.into(),
            visit_all_cookies: value.visit_all_cookies.into(),
            visit_url_cookies: value.visit_url_cookies.into(),
            set_cookie: value.set_cookie.into(),
            delete_cookies: value.delete_cookies.into(),
            flush_store: value.flush_store.into(),
        }
    }
}
impl Into<_cef_cookie_manager_t> for CookieManager {
    fn into(self) -> _cef_cookie_manager_t {
        _cef_cookie_manager_t {
            base: self.base.into(),
            visit_all_cookies: self.visit_all_cookies.into(),
            visit_url_cookies: self.visit_url_cookies.into(),
            set_cookie: self.set_cookie.into(),
            delete_cookies: self.delete_cookies.into(),
            flush_store: self.flush_store.into(),
        }
    }
}
impl Default for CookieManager {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_cookie_visitor_t] for more documentation.
#[derive(Clone)]
pub struct CookieVisitor {
    pub base: BaseRefCounted,
    pub visit: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_cookie_visitor_t,
            cookie: *const _cef_cookie_t,
            count: ::std::os::raw::c_int,
            total: ::std::os::raw::c_int,
            deleteCookie: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_cookie_visitor_t> for CookieVisitor {
    fn from(value: _cef_cookie_visitor_t) -> Self {
        Self {
            base: value.base.into(),
            visit: value.visit.into(),
        }
    }
}
impl Into<_cef_cookie_visitor_t> for CookieVisitor {
    fn into(self) -> _cef_cookie_visitor_t {
        _cef_cookie_visitor_t {
            base: self.base.into(),
            visit: self.visit.into(),
        }
    }
}
impl Default for CookieVisitor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_set_cookie_callback_t] for more documentation.
#[derive(Clone)]
pub struct SetCookieCallback {
    pub base: BaseRefCounted,
    pub on_complete: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_set_cookie_callback_t,
            success: ::std::os::raw::c_int,
        ),
    >,
}
impl From<_cef_set_cookie_callback_t> for SetCookieCallback {
    fn from(value: _cef_set_cookie_callback_t) -> Self {
        Self {
            base: value.base.into(),
            on_complete: value.on_complete.into(),
        }
    }
}
impl Into<_cef_set_cookie_callback_t> for SetCookieCallback {
    fn into(self) -> _cef_set_cookie_callback_t {
        _cef_set_cookie_callback_t {
            base: self.base.into(),
            on_complete: self.on_complete.into(),
        }
    }
}
impl Default for SetCookieCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_delete_cookies_callback_t] for more documentation.
#[derive(Clone)]
pub struct DeleteCookiesCallback {
    pub base: BaseRefCounted,
    pub on_complete: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_delete_cookies_callback_t,
            num_deleted: ::std::os::raw::c_int,
        ),
    >,
}
impl From<_cef_delete_cookies_callback_t> for DeleteCookiesCallback {
    fn from(value: _cef_delete_cookies_callback_t) -> Self {
        Self {
            base: value.base.into(),
            on_complete: value.on_complete.into(),
        }
    }
}
impl Into<_cef_delete_cookies_callback_t> for DeleteCookiesCallback {
    fn into(self) -> _cef_delete_cookies_callback_t {
        _cef_delete_cookies_callback_t {
            base: self.base.into(),
            on_complete: self.on_complete.into(),
        }
    }
}
impl Default for DeleteCookiesCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_media_router_t] for more documentation.
#[derive(Clone)]
pub struct MediaRouter {
    pub base: BaseRefCounted,
    pub add_observer: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_router_t,
            observer: *mut _cef_media_observer_t,
        ) -> *mut _cef_registration_t,
    >,
    pub get_source: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_router_t,
            urn: *const cef_string_t,
        ) -> *mut _cef_media_source_t,
    >,
    pub notify_current_sinks:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_media_router_t)>,
    pub create_route: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_router_t,
            source: *mut _cef_media_source_t,
            sink: *mut _cef_media_sink_t,
            callback: *mut _cef_media_route_create_callback_t,
        ),
    >,
    pub notify_current_routes:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_media_router_t)>,
}
impl From<_cef_media_router_t> for MediaRouter {
    fn from(value: _cef_media_router_t) -> Self {
        Self {
            base: value.base.into(),
            add_observer: value.add_observer.into(),
            get_source: value.get_source.into(),
            notify_current_sinks: value.notify_current_sinks.into(),
            create_route: value.create_route.into(),
            notify_current_routes: value.notify_current_routes.into(),
        }
    }
}
impl Into<_cef_media_router_t> for MediaRouter {
    fn into(self) -> _cef_media_router_t {
        _cef_media_router_t {
            base: self.base.into(),
            add_observer: self.add_observer.into(),
            get_source: self.get_source.into(),
            notify_current_sinks: self.notify_current_sinks.into(),
            create_route: self.create_route.into(),
            notify_current_routes: self.notify_current_routes.into(),
        }
    }
}
impl Default for MediaRouter {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_media_observer_t] for more documentation.
#[derive(Clone)]
pub struct MediaObserver {
    pub base: BaseRefCounted,
    pub on_sinks: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_observer_t,
            sinksCount: usize,
            sinks: *const *mut _cef_media_sink_t,
        ),
    >,
    pub on_routes: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_observer_t,
            routesCount: usize,
            routes: *const *mut _cef_media_route_t,
        ),
    >,
    pub on_route_state_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_observer_t,
            route: *mut _cef_media_route_t,
            state: cef_media_route_connection_state_t,
        ),
    >,
    pub on_route_message_received: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_observer_t,
            route: *mut _cef_media_route_t,
            message: *const ::std::os::raw::c_void,
            message_size: usize,
        ),
    >,
}
impl From<_cef_media_observer_t> for MediaObserver {
    fn from(value: _cef_media_observer_t) -> Self {
        Self {
            base: value.base.into(),
            on_sinks: value.on_sinks.into(),
            on_routes: value.on_routes.into(),
            on_route_state_changed: value.on_route_state_changed.into(),
            on_route_message_received: value.on_route_message_received.into(),
        }
    }
}
impl Into<_cef_media_observer_t> for MediaObserver {
    fn into(self) -> _cef_media_observer_t {
        _cef_media_observer_t {
            base: self.base.into(),
            on_sinks: self.on_sinks.into(),
            on_routes: self.on_routes.into(),
            on_route_state_changed: self.on_route_state_changed.into(),
            on_route_message_received: self.on_route_message_received.into(),
        }
    }
}
impl Default for MediaObserver {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_media_route_t] for more documentation.
#[derive(Clone)]
pub struct MediaRoute {
    pub base: BaseRefCounted,
    pub get_id: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_route_t) -> cef_string_userfree_t,
    >,
    pub get_source: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_route_t) -> *mut _cef_media_source_t,
    >,
    pub get_sink: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_route_t) -> *mut _cef_media_sink_t,
    >,
    pub send_route_message: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_route_t,
            message: *const ::std::os::raw::c_void,
            message_size: usize,
        ),
    >,
    pub terminate:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_media_route_t)>,
}
impl From<_cef_media_route_t> for MediaRoute {
    fn from(value: _cef_media_route_t) -> Self {
        Self {
            base: value.base.into(),
            get_id: value.get_id.into(),
            get_source: value.get_source.into(),
            get_sink: value.get_sink.into(),
            send_route_message: value.send_route_message.into(),
            terminate: value.terminate.into(),
        }
    }
}
impl Into<_cef_media_route_t> for MediaRoute {
    fn into(self) -> _cef_media_route_t {
        _cef_media_route_t {
            base: self.base.into(),
            get_id: self.get_id.into(),
            get_source: self.get_source.into(),
            get_sink: self.get_sink.into(),
            send_route_message: self.send_route_message.into(),
            terminate: self.terminate.into(),
        }
    }
}
impl Default for MediaRoute {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_media_route_create_callback_t] for more documentation.
#[derive(Clone)]
pub struct MediaRouteCreateCallback {
    pub base: BaseRefCounted,
    pub on_media_route_create_finished: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_route_create_callback_t,
            result: cef_media_route_create_result_t,
            error: *const cef_string_t,
            route: *mut _cef_media_route_t,
        ),
    >,
}
impl From<_cef_media_route_create_callback_t> for MediaRouteCreateCallback {
    fn from(value: _cef_media_route_create_callback_t) -> Self {
        Self {
            base: value.base.into(),
            on_media_route_create_finished: value.on_media_route_create_finished.into(),
        }
    }
}
impl Into<_cef_media_route_create_callback_t> for MediaRouteCreateCallback {
    fn into(self) -> _cef_media_route_create_callback_t {
        _cef_media_route_create_callback_t {
            base: self.base.into(),
            on_media_route_create_finished: self.on_media_route_create_finished.into(),
        }
    }
}
impl Default for MediaRouteCreateCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_media_sink_t] for more documentation.
#[derive(Clone)]
pub struct MediaSink {
    pub base: BaseRefCounted,
    pub get_id: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_sink_t) -> cef_string_userfree_t,
    >,
    pub get_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_sink_t) -> cef_string_userfree_t,
    >,
    pub get_icon_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_sink_t) -> cef_media_sink_icon_type_t,
    >,
    pub get_device_info: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_sink_t,
            callback: *mut _cef_media_sink_device_info_callback_t,
        ),
    >,
    pub is_cast_sink: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_sink_t) -> ::std::os::raw::c_int,
    >,
    pub is_dial_sink: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_sink_t) -> ::std::os::raw::c_int,
    >,
    pub is_compatible_with: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_sink_t,
            source: *mut _cef_media_source_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_media_sink_t> for MediaSink {
    fn from(value: _cef_media_sink_t) -> Self {
        Self {
            base: value.base.into(),
            get_id: value.get_id.into(),
            get_name: value.get_name.into(),
            get_icon_type: value.get_icon_type.into(),
            get_device_info: value.get_device_info.into(),
            is_cast_sink: value.is_cast_sink.into(),
            is_dial_sink: value.is_dial_sink.into(),
            is_compatible_with: value.is_compatible_with.into(),
        }
    }
}
impl Into<_cef_media_sink_t> for MediaSink {
    fn into(self) -> _cef_media_sink_t {
        _cef_media_sink_t {
            base: self.base.into(),
            get_id: self.get_id.into(),
            get_name: self.get_name.into(),
            get_icon_type: self.get_icon_type.into(),
            get_device_info: self.get_device_info.into(),
            is_cast_sink: self.is_cast_sink.into(),
            is_dial_sink: self.is_dial_sink.into(),
            is_compatible_with: self.is_compatible_with.into(),
        }
    }
}
impl Default for MediaSink {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_media_sink_device_info_callback_t] for more documentation.
#[derive(Clone)]
pub struct MediaSinkDeviceInfoCallback {
    pub base: BaseRefCounted,
    pub on_media_sink_device_info: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_sink_device_info_callback_t,
            device_info: *const _cef_media_sink_device_info_t,
        ),
    >,
}
impl From<_cef_media_sink_device_info_callback_t> for MediaSinkDeviceInfoCallback {
    fn from(value: _cef_media_sink_device_info_callback_t) -> Self {
        Self {
            base: value.base.into(),
            on_media_sink_device_info: value.on_media_sink_device_info.into(),
        }
    }
}
impl Into<_cef_media_sink_device_info_callback_t> for MediaSinkDeviceInfoCallback {
    fn into(self) -> _cef_media_sink_device_info_callback_t {
        _cef_media_sink_device_info_callback_t {
            base: self.base.into(),
            on_media_sink_device_info: self.on_media_sink_device_info.into(),
        }
    }
}
impl Default for MediaSinkDeviceInfoCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_media_source_t] for more documentation.
#[derive(Clone)]
pub struct MediaSource {
    pub base: BaseRefCounted,
    pub get_id: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_source_t) -> cef_string_userfree_t,
    >,
    pub is_cast_source: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_source_t) -> ::std::os::raw::c_int,
    >,
    pub is_dial_source: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_media_source_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_media_source_t> for MediaSource {
    fn from(value: _cef_media_source_t) -> Self {
        Self {
            base: value.base.into(),
            get_id: value.get_id.into(),
            is_cast_source: value.is_cast_source.into(),
            is_dial_source: value.is_dial_source.into(),
        }
    }
}
impl Into<_cef_media_source_t> for MediaSource {
    fn into(self) -> _cef_media_source_t {
        _cef_media_source_t {
            base: self.base.into(),
            get_id: self.get_id.into(),
            is_cast_source: self.is_cast_source.into(),
            is_dial_source: self.is_dial_source.into(),
        }
    }
}
impl Default for MediaSource {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_preference_registrar_t] for more documentation.
#[derive(Clone)]
pub struct PreferenceRegistrar {
    pub base: BaseScoped,
    pub add_preference: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_preference_registrar_t,
            name: *const cef_string_t,
            default_value: *mut _cef_value_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_preference_registrar_t> for PreferenceRegistrar {
    fn from(value: _cef_preference_registrar_t) -> Self {
        Self {
            base: value.base.into(),
            add_preference: value.add_preference.into(),
        }
    }
}
impl Into<_cef_preference_registrar_t> for PreferenceRegistrar {
    fn into(self) -> _cef_preference_registrar_t {
        _cef_preference_registrar_t {
            base: self.base.into(),
            add_preference: self.add_preference.into(),
        }
    }
}
impl Default for PreferenceRegistrar {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_preference_observer_t] for more documentation.
#[derive(Clone)]
pub struct PreferenceObserver {
    pub base: BaseRefCounted,
    pub on_preference_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_preference_observer_t,
            name: *const cef_string_t,
        ),
    >,
}
impl From<_cef_preference_observer_t> for PreferenceObserver {
    fn from(value: _cef_preference_observer_t) -> Self {
        Self {
            base: value.base.into(),
            on_preference_changed: value.on_preference_changed.into(),
        }
    }
}
impl Into<_cef_preference_observer_t> for PreferenceObserver {
    fn into(self) -> _cef_preference_observer_t {
        _cef_preference_observer_t {
            base: self.base.into(),
            on_preference_changed: self.on_preference_changed.into(),
        }
    }
}
impl Default for PreferenceObserver {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_preference_manager_t] for more documentation.
#[derive(Clone)]
pub struct PreferenceManager {
    pub base: BaseRefCounted,
    pub has_preference: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_preference_manager_t,
            name: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_preference: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_preference_manager_t,
            name: *const cef_string_t,
        ) -> *mut _cef_value_t,
    >,
    pub get_all_preferences: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_preference_manager_t,
            include_defaults: ::std::os::raw::c_int,
        ) -> *mut _cef_dictionary_value_t,
    >,
    pub can_set_preference: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_preference_manager_t,
            name: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_preference: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_preference_manager_t,
            name: *const cef_string_t,
            value: *mut _cef_value_t,
            error: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub add_preference_observer: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_preference_manager_t,
            name: *const cef_string_t,
            observer: *mut _cef_preference_observer_t,
        ) -> *mut _cef_registration_t,
    >,
}
impl From<_cef_preference_manager_t> for PreferenceManager {
    fn from(value: _cef_preference_manager_t) -> Self {
        Self {
            base: value.base.into(),
            has_preference: value.has_preference.into(),
            get_preference: value.get_preference.into(),
            get_all_preferences: value.get_all_preferences.into(),
            can_set_preference: value.can_set_preference.into(),
            set_preference: value.set_preference.into(),
            add_preference_observer: value.add_preference_observer.into(),
        }
    }
}
impl Into<_cef_preference_manager_t> for PreferenceManager {
    fn into(self) -> _cef_preference_manager_t {
        _cef_preference_manager_t {
            base: self.base.into(),
            has_preference: self.has_preference.into(),
            get_preference: self.get_preference.into(),
            get_all_preferences: self.get_all_preferences.into(),
            can_set_preference: self.can_set_preference.into(),
            set_preference: self.set_preference.into(),
            add_preference_observer: self.add_preference_observer.into(),
        }
    }
}
impl Default for PreferenceManager {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_resolve_callback_t] for more documentation.
#[derive(Clone)]
pub struct ResolveCallback {
    pub base: BaseRefCounted,
    pub on_resolve_completed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resolve_callback_t,
            result: cef_errorcode_t,
            resolved_ips: cef_string_list_t,
        ),
    >,
}
impl From<_cef_resolve_callback_t> for ResolveCallback {
    fn from(value: _cef_resolve_callback_t) -> Self {
        Self {
            base: value.base.into(),
            on_resolve_completed: value.on_resolve_completed.into(),
        }
    }
}
impl Into<_cef_resolve_callback_t> for ResolveCallback {
    fn into(self) -> _cef_resolve_callback_t {
        _cef_resolve_callback_t {
            base: self.base.into(),
            on_resolve_completed: self.on_resolve_completed.into(),
        }
    }
}
impl Default for ResolveCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_setting_observer_t] for more documentation.
#[derive(Clone)]
pub struct SettingObserver {
    pub base: BaseRefCounted,
    pub on_setting_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_setting_observer_t,
            requesting_url: *const cef_string_t,
            top_level_url: *const cef_string_t,
            content_type: cef_content_setting_types_t,
        ),
    >,
}
impl From<_cef_setting_observer_t> for SettingObserver {
    fn from(value: _cef_setting_observer_t) -> Self {
        Self {
            base: value.base.into(),
            on_setting_changed: value.on_setting_changed.into(),
        }
    }
}
impl Into<_cef_setting_observer_t> for SettingObserver {
    fn into(self) -> _cef_setting_observer_t {
        _cef_setting_observer_t {
            base: self.base.into(),
            on_setting_changed: self.on_setting_changed.into(),
        }
    }
}
impl Default for SettingObserver {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_request_context_t] for more documentation.
#[derive(Clone)]
pub struct RequestContext {
    pub base: PreferenceManager,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            other: *mut _cef_request_context_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_sharing_with: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            other: *mut _cef_request_context_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_global: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_context_t) -> ::std::os::raw::c_int,
    >,
    pub get_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
        ) -> *mut _cef_request_context_handler_t,
    >,
    pub get_cache_path: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_context_t) -> cef_string_userfree_t,
    >,
    pub get_cookie_manager: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            callback: *mut _cef_completion_callback_t,
        ) -> *mut _cef_cookie_manager_t,
    >,
    pub register_scheme_handler_factory: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            scheme_name: *const cef_string_t,
            domain_name: *const cef_string_t,
            factory: *mut _cef_scheme_handler_factory_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub clear_scheme_handler_factories: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_context_t) -> ::std::os::raw::c_int,
    >,
    pub clear_certificate_exceptions: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            callback: *mut _cef_completion_callback_t,
        ),
    >,
    pub clear_http_auth_credentials: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            callback: *mut _cef_completion_callback_t,
        ),
    >,
    pub close_all_connections: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            callback: *mut _cef_completion_callback_t,
        ),
    >,
    pub resolve_host: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            origin: *const cef_string_t,
            callback: *mut _cef_resolve_callback_t,
        ),
    >,
    pub get_media_router: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            callback: *mut _cef_completion_callback_t,
        ) -> *mut _cef_media_router_t,
    >,
    pub get_website_setting: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            requesting_url: *const cef_string_t,
            top_level_url: *const cef_string_t,
            content_type: cef_content_setting_types_t,
        ) -> *mut _cef_value_t,
    >,
    pub set_website_setting: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            requesting_url: *const cef_string_t,
            top_level_url: *const cef_string_t,
            content_type: cef_content_setting_types_t,
            value: *mut _cef_value_t,
        ),
    >,
    pub get_content_setting: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            requesting_url: *const cef_string_t,
            top_level_url: *const cef_string_t,
            content_type: cef_content_setting_types_t,
        ) -> cef_content_setting_values_t,
    >,
    pub set_content_setting: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            requesting_url: *const cef_string_t,
            top_level_url: *const cef_string_t,
            content_type: cef_content_setting_types_t,
            value: cef_content_setting_values_t,
        ),
    >,
    pub set_chrome_color_scheme: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            variant: cef_color_variant_t,
            user_color: cef_color_t,
        ),
    >,
    pub get_chrome_color_scheme_mode: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_context_t) -> cef_color_variant_t,
    >,
    pub get_chrome_color_scheme_color: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_context_t) -> cef_color_t,
    >,
    pub get_chrome_color_scheme_variant: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_request_context_t) -> cef_color_variant_t,
    >,
    pub add_setting_observer: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_t,
            observer: *mut _cef_setting_observer_t,
        ) -> *mut _cef_registration_t,
    >,
}
impl From<_cef_request_context_t> for RequestContext {
    fn from(value: _cef_request_context_t) -> Self {
        Self {
            base: value.base.into(),
            is_same: value.is_same.into(),
            is_sharing_with: value.is_sharing_with.into(),
            is_global: value.is_global.into(),
            get_handler: value.get_handler.into(),
            get_cache_path: value.get_cache_path.into(),
            get_cookie_manager: value.get_cookie_manager.into(),
            register_scheme_handler_factory: value.register_scheme_handler_factory.into(),
            clear_scheme_handler_factories: value.clear_scheme_handler_factories.into(),
            clear_certificate_exceptions: value.clear_certificate_exceptions.into(),
            clear_http_auth_credentials: value.clear_http_auth_credentials.into(),
            close_all_connections: value.close_all_connections.into(),
            resolve_host: value.resolve_host.into(),
            get_media_router: value.get_media_router.into(),
            get_website_setting: value.get_website_setting.into(),
            set_website_setting: value.set_website_setting.into(),
            get_content_setting: value.get_content_setting.into(),
            set_content_setting: value.set_content_setting.into(),
            set_chrome_color_scheme: value.set_chrome_color_scheme.into(),
            get_chrome_color_scheme_mode: value.get_chrome_color_scheme_mode.into(),
            get_chrome_color_scheme_color: value.get_chrome_color_scheme_color.into(),
            get_chrome_color_scheme_variant: value.get_chrome_color_scheme_variant.into(),
            add_setting_observer: value.add_setting_observer.into(),
        }
    }
}
impl Into<_cef_request_context_t> for RequestContext {
    fn into(self) -> _cef_request_context_t {
        _cef_request_context_t {
            base: self.base.into(),
            is_same: self.is_same.into(),
            is_sharing_with: self.is_sharing_with.into(),
            is_global: self.is_global.into(),
            get_handler: self.get_handler.into(),
            get_cache_path: self.get_cache_path.into(),
            get_cookie_manager: self.get_cookie_manager.into(),
            register_scheme_handler_factory: self.register_scheme_handler_factory.into(),
            clear_scheme_handler_factories: self.clear_scheme_handler_factories.into(),
            clear_certificate_exceptions: self.clear_certificate_exceptions.into(),
            clear_http_auth_credentials: self.clear_http_auth_credentials.into(),
            close_all_connections: self.close_all_connections.into(),
            resolve_host: self.resolve_host.into(),
            get_media_router: self.get_media_router.into(),
            get_website_setting: self.get_website_setting.into(),
            set_website_setting: self.set_website_setting.into(),
            get_content_setting: self.get_content_setting.into(),
            set_content_setting: self.set_content_setting.into(),
            set_chrome_color_scheme: self.set_chrome_color_scheme.into(),
            get_chrome_color_scheme_mode: self.get_chrome_color_scheme_mode.into(),
            get_chrome_color_scheme_color: self.get_chrome_color_scheme_color.into(),
            get_chrome_color_scheme_variant: self.get_chrome_color_scheme_variant.into(),
            add_setting_observer: self.add_setting_observer.into(),
        }
    }
}
impl Default for RequestContext {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_browser_t] for more documentation.
#[derive(Clone)]
pub struct Browser {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int,
    >,
    pub get_host: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> *mut _cef_browser_host_t,
    >,
    pub can_go_back: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int,
    >,
    pub go_back: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_t)>,
    pub can_go_forward: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int,
    >,
    pub go_forward: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_t)>,
    pub is_loading: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int,
    >,
    pub reload: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_t)>,
    pub reload_ignore_cache:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_t)>,
    pub stop_load: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_t)>,
    pub get_identifier: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_t,
            that: *mut _cef_browser_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_popup: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int,
    >,
    pub has_document: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> ::std::os::raw::c_int,
    >,
    pub get_main_frame: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> *mut _cef_frame_t,
    >,
    pub get_focused_frame: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> *mut _cef_frame_t,
    >,
    pub get_frame_by_identifier: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_t,
            identifier: *const cef_string_t,
        ) -> *mut _cef_frame_t,
    >,
    pub get_frame_by_name: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_t,
            name: *const cef_string_t,
        ) -> *mut _cef_frame_t,
    >,
    pub get_frame_count:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_t) -> usize>,
    pub get_frame_identifiers: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t, identifiers: cef_string_list_t),
    >,
    pub get_frame_names: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_t, names: cef_string_list_t),
    >,
}
impl From<_cef_browser_t> for Browser {
    fn from(value: _cef_browser_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            get_host: value.get_host.into(),
            can_go_back: value.can_go_back.into(),
            go_back: value.go_back.into(),
            can_go_forward: value.can_go_forward.into(),
            go_forward: value.go_forward.into(),
            is_loading: value.is_loading.into(),
            reload: value.reload.into(),
            reload_ignore_cache: value.reload_ignore_cache.into(),
            stop_load: value.stop_load.into(),
            get_identifier: value.get_identifier.into(),
            is_same: value.is_same.into(),
            is_popup: value.is_popup.into(),
            has_document: value.has_document.into(),
            get_main_frame: value.get_main_frame.into(),
            get_focused_frame: value.get_focused_frame.into(),
            get_frame_by_identifier: value.get_frame_by_identifier.into(),
            get_frame_by_name: value.get_frame_by_name.into(),
            get_frame_count: value.get_frame_count.into(),
            get_frame_identifiers: value.get_frame_identifiers.into(),
            get_frame_names: value.get_frame_names.into(),
        }
    }
}
impl Into<_cef_browser_t> for Browser {
    fn into(self) -> _cef_browser_t {
        _cef_browser_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            get_host: self.get_host.into(),
            can_go_back: self.can_go_back.into(),
            go_back: self.go_back.into(),
            can_go_forward: self.can_go_forward.into(),
            go_forward: self.go_forward.into(),
            is_loading: self.is_loading.into(),
            reload: self.reload.into(),
            reload_ignore_cache: self.reload_ignore_cache.into(),
            stop_load: self.stop_load.into(),
            get_identifier: self.get_identifier.into(),
            is_same: self.is_same.into(),
            is_popup: self.is_popup.into(),
            has_document: self.has_document.into(),
            get_main_frame: self.get_main_frame.into(),
            get_focused_frame: self.get_focused_frame.into(),
            get_frame_by_identifier: self.get_frame_by_identifier.into(),
            get_frame_by_name: self.get_frame_by_name.into(),
            get_frame_count: self.get_frame_count.into(),
            get_frame_identifiers: self.get_frame_identifiers.into(),
            get_frame_names: self.get_frame_names.into(),
        }
    }
}
impl Default for Browser {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_run_file_dialog_callback_t] for more documentation.
#[derive(Clone)]
pub struct RunFileDialogCallback {
    pub base: BaseRefCounted,
    pub on_file_dialog_dismissed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_run_file_dialog_callback_t,
            file_paths: cef_string_list_t,
        ),
    >,
}
impl From<_cef_run_file_dialog_callback_t> for RunFileDialogCallback {
    fn from(value: _cef_run_file_dialog_callback_t) -> Self {
        Self {
            base: value.base.into(),
            on_file_dialog_dismissed: value.on_file_dialog_dismissed.into(),
        }
    }
}
impl Into<_cef_run_file_dialog_callback_t> for RunFileDialogCallback {
    fn into(self) -> _cef_run_file_dialog_callback_t {
        _cef_run_file_dialog_callback_t {
            base: self.base.into(),
            on_file_dialog_dismissed: self.on_file_dialog_dismissed.into(),
        }
    }
}
impl Default for RunFileDialogCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_navigation_entry_visitor_t] for more documentation.
#[derive(Clone)]
pub struct NavigationEntryVisitor {
    pub base: BaseRefCounted,
    pub visit: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_navigation_entry_visitor_t,
            entry: *mut _cef_navigation_entry_t,
            current: ::std::os::raw::c_int,
            index: ::std::os::raw::c_int,
            total: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_navigation_entry_visitor_t> for NavigationEntryVisitor {
    fn from(value: _cef_navigation_entry_visitor_t) -> Self {
        Self {
            base: value.base.into(),
            visit: value.visit.into(),
        }
    }
}
impl Into<_cef_navigation_entry_visitor_t> for NavigationEntryVisitor {
    fn into(self) -> _cef_navigation_entry_visitor_t {
        _cef_navigation_entry_visitor_t {
            base: self.base.into(),
            visit: self.visit.into(),
        }
    }
}
impl Default for NavigationEntryVisitor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_pdf_print_callback_t] for more documentation.
#[derive(Clone)]
pub struct PdfPrintCallback {
    pub base: BaseRefCounted,
    pub on_pdf_print_finished: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_pdf_print_callback_t,
            path: *const cef_string_t,
            ok: ::std::os::raw::c_int,
        ),
    >,
}
impl From<_cef_pdf_print_callback_t> for PdfPrintCallback {
    fn from(value: _cef_pdf_print_callback_t) -> Self {
        Self {
            base: value.base.into(),
            on_pdf_print_finished: value.on_pdf_print_finished.into(),
        }
    }
}
impl Into<_cef_pdf_print_callback_t> for PdfPrintCallback {
    fn into(self) -> _cef_pdf_print_callback_t {
        _cef_pdf_print_callback_t {
            base: self.base.into(),
            on_pdf_print_finished: self.on_pdf_print_finished.into(),
        }
    }
}
impl Default for PdfPrintCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_download_image_callback_t] for more documentation.
#[derive(Clone)]
pub struct DownloadImageCallback {
    pub base: BaseRefCounted,
    pub on_download_image_finished: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_download_image_callback_t,
            image_url: *const cef_string_t,
            http_status_code: ::std::os::raw::c_int,
            image: *mut _cef_image_t,
        ),
    >,
}
impl From<_cef_download_image_callback_t> for DownloadImageCallback {
    fn from(value: _cef_download_image_callback_t) -> Self {
        Self {
            base: value.base.into(),
            on_download_image_finished: value.on_download_image_finished.into(),
        }
    }
}
impl Into<_cef_download_image_callback_t> for DownloadImageCallback {
    fn into(self) -> _cef_download_image_callback_t {
        _cef_download_image_callback_t {
            base: self.base.into(),
            on_download_image_finished: self.on_download_image_finished.into(),
        }
    }
}
impl Default for DownloadImageCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_browser_host_t] for more documentation.
#[derive(Clone)]
pub struct BrowserHost {
    pub base: BaseRefCounted,
    pub get_browser: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> *mut _cef_browser_t,
    >,
    pub close_browser: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            force_close: ::std::os::raw::c_int,
        ),
    >,
    pub try_close_browser: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> ::std::os::raw::c_int,
    >,
    pub is_ready_to_be_closed: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> ::std::os::raw::c_int,
    >,
    pub set_focus: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t, focus: ::std::os::raw::c_int),
    >,
    pub get_window_handle: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> cef_window_handle_t,
    >,
    pub get_opener_window_handle: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> cef_window_handle_t,
    >,
    pub get_opener_identifier: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> ::std::os::raw::c_int,
    >,
    pub has_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> ::std::os::raw::c_int,
    >,
    pub get_client: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> *mut _cef_client_t,
    >,
    pub get_request_context: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> *mut _cef_request_context_t,
    >,
    pub can_zoom: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            command: cef_zoom_command_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub zoom: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t, command: cef_zoom_command_t),
    >,
    pub get_default_zoom_level:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> f64>,
    pub get_zoom_level:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> f64>,
    pub set_zoom_level: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t, zoomLevel: f64),
    >,
    pub run_file_dialog: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            mode: cef_file_dialog_mode_t,
            title: *const cef_string_t,
            default_file_path: *const cef_string_t,
            accept_filters: cef_string_list_t,
            callback: *mut _cef_run_file_dialog_callback_t,
        ),
    >,
    pub start_download: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t, url: *const cef_string_t),
    >,
    pub download_image: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            image_url: *const cef_string_t,
            is_favicon: ::std::os::raw::c_int,
            max_image_size: u32,
            bypass_cache: ::std::os::raw::c_int,
            callback: *mut _cef_download_image_callback_t,
        ),
    >,
    pub print: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t)>,
    pub print_to_pdf: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            path: *const cef_string_t,
            settings: *const _cef_pdf_print_settings_t,
            callback: *mut _cef_pdf_print_callback_t,
        ),
    >,
    pub find: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            searchText: *const cef_string_t,
            forward: ::std::os::raw::c_int,
            matchCase: ::std::os::raw::c_int,
            findNext: ::std::os::raw::c_int,
        ),
    >,
    pub stop_finding: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            clearSelection: ::std::os::raw::c_int,
        ),
    >,
    pub show_dev_tools: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            windowInfo: *const _cef_window_info_t,
            client: *mut _cef_client_t,
            settings: *const _cef_browser_settings_t,
            inspect_element_at: *const cef_point_t,
        ),
    >,
    pub close_dev_tools:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t)>,
    pub has_dev_tools: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> ::std::os::raw::c_int,
    >,
    pub send_dev_tools_message: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            message: *const ::std::os::raw::c_void,
            message_size: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub execute_dev_tools_method: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            message_id: ::std::os::raw::c_int,
            method: *const cef_string_t,
            params: *mut _cef_dictionary_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub add_dev_tools_message_observer: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            observer: *mut _cef_dev_tools_message_observer_t,
        ) -> *mut _cef_registration_t,
    >,
    pub get_navigation_entries: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            visitor: *mut _cef_navigation_entry_visitor_t,
            current_only: ::std::os::raw::c_int,
        ),
    >,
    pub replace_misspelling: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t, word: *const cef_string_t),
    >,
    pub add_word_to_dictionary: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t, word: *const cef_string_t),
    >,
    pub is_window_rendering_disabled: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> ::std::os::raw::c_int,
    >,
    pub was_resized:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t)>,
    pub was_hidden: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t, hidden: ::std::os::raw::c_int),
    >,
    pub notify_screen_info_changed:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t)>,
    pub invalidate: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            type_: cef_paint_element_type_t,
        ),
    >,
    pub send_external_begin_frame:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t)>,
    pub send_key_event: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t, event: *const cef_key_event_t),
    >,
    pub send_mouse_click_event: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            event: *const cef_mouse_event_t,
            type_: cef_mouse_button_type_t,
            mouseUp: ::std::os::raw::c_int,
            clickCount: ::std::os::raw::c_int,
        ),
    >,
    pub send_mouse_move_event: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            event: *const cef_mouse_event_t,
            mouseLeave: ::std::os::raw::c_int,
        ),
    >,
    pub send_mouse_wheel_event: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            event: *const cef_mouse_event_t,
            deltaX: ::std::os::raw::c_int,
            deltaY: ::std::os::raw::c_int,
        ),
    >,
    pub send_touch_event: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            event: *const cef_touch_event_t,
        ),
    >,
    pub send_capture_lost_event:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t)>,
    pub notify_move_or_resize_started:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t)>,
    pub get_windowless_frame_rate: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> ::std::os::raw::c_int,
    >,
    pub set_windowless_frame_rate: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            frame_rate: ::std::os::raw::c_int,
        ),
    >,
    pub ime_set_composition: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            text: *const cef_string_t,
            underlinesCount: usize,
            underlines: *const cef_composition_underline_t,
            replacement_range: *const cef_range_t,
            selection_range: *const cef_range_t,
        ),
    >,
    pub ime_commit_text: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            text: *const cef_string_t,
            replacement_range: *const cef_range_t,
            relative_cursor_pos: ::std::os::raw::c_int,
        ),
    >,
    pub ime_finish_composing_text: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            keep_selection: ::std::os::raw::c_int,
        ),
    >,
    pub ime_cancel_composition:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t)>,
    pub drag_target_drag_enter: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            drag_data: *mut _cef_drag_data_t,
            event: *const cef_mouse_event_t,
            allowed_ops: cef_drag_operations_mask_t,
        ),
    >,
    pub drag_target_drag_over: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            event: *const cef_mouse_event_t,
            allowed_ops: cef_drag_operations_mask_t,
        ),
    >,
    pub drag_target_drag_leave:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t)>,
    pub drag_target_drop: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            event: *const cef_mouse_event_t,
        ),
    >,
    pub drag_source_ended_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
            op: cef_drag_operations_mask_t,
        ),
    >,
    pub drag_source_system_drag_ended:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t)>,
    pub get_visible_navigation_entry: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> *mut _cef_navigation_entry_t,
    >,
    pub set_accessibility_state: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            accessibility_state: cef_state_t,
        ),
    >,
    pub set_auto_resize_enabled: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            enabled: ::std::os::raw::c_int,
            min_size: *const cef_size_t,
            max_size: *const cef_size_t,
        ),
    >,
    pub set_audio_muted: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t, mute: ::std::os::raw::c_int),
    >,
    pub is_audio_muted: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> ::std::os::raw::c_int,
    >,
    pub is_fullscreen: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> ::std::os::raw::c_int,
    >,
    pub exit_fullscreen: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            will_cause_resize: ::std::os::raw::c_int,
        ),
    >,
    pub can_execute_chrome_command: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub execute_chrome_command: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_host_t,
            command_id: ::std::os::raw::c_int,
            disposition: cef_window_open_disposition_t,
        ),
    >,
    pub is_render_process_unresponsive: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> ::std::os::raw::c_int,
    >,
    pub get_runtime_style: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_host_t) -> cef_runtime_style_t,
    >,
}
impl From<_cef_browser_host_t> for BrowserHost {
    fn from(value: _cef_browser_host_t) -> Self {
        Self {
            base: value.base.into(),
            get_browser: value.get_browser.into(),
            close_browser: value.close_browser.into(),
            try_close_browser: value.try_close_browser.into(),
            is_ready_to_be_closed: value.is_ready_to_be_closed.into(),
            set_focus: value.set_focus.into(),
            get_window_handle: value.get_window_handle.into(),
            get_opener_window_handle: value.get_opener_window_handle.into(),
            get_opener_identifier: value.get_opener_identifier.into(),
            has_view: value.has_view.into(),
            get_client: value.get_client.into(),
            get_request_context: value.get_request_context.into(),
            can_zoom: value.can_zoom.into(),
            zoom: value.zoom.into(),
            get_default_zoom_level: value.get_default_zoom_level.into(),
            get_zoom_level: value.get_zoom_level.into(),
            set_zoom_level: value.set_zoom_level.into(),
            run_file_dialog: value.run_file_dialog.into(),
            start_download: value.start_download.into(),
            download_image: value.download_image.into(),
            print: value.print.into(),
            print_to_pdf: value.print_to_pdf.into(),
            find: value.find.into(),
            stop_finding: value.stop_finding.into(),
            show_dev_tools: value.show_dev_tools.into(),
            close_dev_tools: value.close_dev_tools.into(),
            has_dev_tools: value.has_dev_tools.into(),
            send_dev_tools_message: value.send_dev_tools_message.into(),
            execute_dev_tools_method: value.execute_dev_tools_method.into(),
            add_dev_tools_message_observer: value.add_dev_tools_message_observer.into(),
            get_navigation_entries: value.get_navigation_entries.into(),
            replace_misspelling: value.replace_misspelling.into(),
            add_word_to_dictionary: value.add_word_to_dictionary.into(),
            is_window_rendering_disabled: value.is_window_rendering_disabled.into(),
            was_resized: value.was_resized.into(),
            was_hidden: value.was_hidden.into(),
            notify_screen_info_changed: value.notify_screen_info_changed.into(),
            invalidate: value.invalidate.into(),
            send_external_begin_frame: value.send_external_begin_frame.into(),
            send_key_event: value.send_key_event.into(),
            send_mouse_click_event: value.send_mouse_click_event.into(),
            send_mouse_move_event: value.send_mouse_move_event.into(),
            send_mouse_wheel_event: value.send_mouse_wheel_event.into(),
            send_touch_event: value.send_touch_event.into(),
            send_capture_lost_event: value.send_capture_lost_event.into(),
            notify_move_or_resize_started: value.notify_move_or_resize_started.into(),
            get_windowless_frame_rate: value.get_windowless_frame_rate.into(),
            set_windowless_frame_rate: value.set_windowless_frame_rate.into(),
            ime_set_composition: value.ime_set_composition.into(),
            ime_commit_text: value.ime_commit_text.into(),
            ime_finish_composing_text: value.ime_finish_composing_text.into(),
            ime_cancel_composition: value.ime_cancel_composition.into(),
            drag_target_drag_enter: value.drag_target_drag_enter.into(),
            drag_target_drag_over: value.drag_target_drag_over.into(),
            drag_target_drag_leave: value.drag_target_drag_leave.into(),
            drag_target_drop: value.drag_target_drop.into(),
            drag_source_ended_at: value.drag_source_ended_at.into(),
            drag_source_system_drag_ended: value.drag_source_system_drag_ended.into(),
            get_visible_navigation_entry: value.get_visible_navigation_entry.into(),
            set_accessibility_state: value.set_accessibility_state.into(),
            set_auto_resize_enabled: value.set_auto_resize_enabled.into(),
            set_audio_muted: value.set_audio_muted.into(),
            is_audio_muted: value.is_audio_muted.into(),
            is_fullscreen: value.is_fullscreen.into(),
            exit_fullscreen: value.exit_fullscreen.into(),
            can_execute_chrome_command: value.can_execute_chrome_command.into(),
            execute_chrome_command: value.execute_chrome_command.into(),
            is_render_process_unresponsive: value.is_render_process_unresponsive.into(),
            get_runtime_style: value.get_runtime_style.into(),
        }
    }
}
impl Into<_cef_browser_host_t> for BrowserHost {
    fn into(self) -> _cef_browser_host_t {
        _cef_browser_host_t {
            base: self.base.into(),
            get_browser: self.get_browser.into(),
            close_browser: self.close_browser.into(),
            try_close_browser: self.try_close_browser.into(),
            is_ready_to_be_closed: self.is_ready_to_be_closed.into(),
            set_focus: self.set_focus.into(),
            get_window_handle: self.get_window_handle.into(),
            get_opener_window_handle: self.get_opener_window_handle.into(),
            get_opener_identifier: self.get_opener_identifier.into(),
            has_view: self.has_view.into(),
            get_client: self.get_client.into(),
            get_request_context: self.get_request_context.into(),
            can_zoom: self.can_zoom.into(),
            zoom: self.zoom.into(),
            get_default_zoom_level: self.get_default_zoom_level.into(),
            get_zoom_level: self.get_zoom_level.into(),
            set_zoom_level: self.set_zoom_level.into(),
            run_file_dialog: self.run_file_dialog.into(),
            start_download: self.start_download.into(),
            download_image: self.download_image.into(),
            print: self.print.into(),
            print_to_pdf: self.print_to_pdf.into(),
            find: self.find.into(),
            stop_finding: self.stop_finding.into(),
            show_dev_tools: self.show_dev_tools.into(),
            close_dev_tools: self.close_dev_tools.into(),
            has_dev_tools: self.has_dev_tools.into(),
            send_dev_tools_message: self.send_dev_tools_message.into(),
            execute_dev_tools_method: self.execute_dev_tools_method.into(),
            add_dev_tools_message_observer: self.add_dev_tools_message_observer.into(),
            get_navigation_entries: self.get_navigation_entries.into(),
            replace_misspelling: self.replace_misspelling.into(),
            add_word_to_dictionary: self.add_word_to_dictionary.into(),
            is_window_rendering_disabled: self.is_window_rendering_disabled.into(),
            was_resized: self.was_resized.into(),
            was_hidden: self.was_hidden.into(),
            notify_screen_info_changed: self.notify_screen_info_changed.into(),
            invalidate: self.invalidate.into(),
            send_external_begin_frame: self.send_external_begin_frame.into(),
            send_key_event: self.send_key_event.into(),
            send_mouse_click_event: self.send_mouse_click_event.into(),
            send_mouse_move_event: self.send_mouse_move_event.into(),
            send_mouse_wheel_event: self.send_mouse_wheel_event.into(),
            send_touch_event: self.send_touch_event.into(),
            send_capture_lost_event: self.send_capture_lost_event.into(),
            notify_move_or_resize_started: self.notify_move_or_resize_started.into(),
            get_windowless_frame_rate: self.get_windowless_frame_rate.into(),
            set_windowless_frame_rate: self.set_windowless_frame_rate.into(),
            ime_set_composition: self.ime_set_composition.into(),
            ime_commit_text: self.ime_commit_text.into(),
            ime_finish_composing_text: self.ime_finish_composing_text.into(),
            ime_cancel_composition: self.ime_cancel_composition.into(),
            drag_target_drag_enter: self.drag_target_drag_enter.into(),
            drag_target_drag_over: self.drag_target_drag_over.into(),
            drag_target_drag_leave: self.drag_target_drag_leave.into(),
            drag_target_drop: self.drag_target_drop.into(),
            drag_source_ended_at: self.drag_source_ended_at.into(),
            drag_source_system_drag_ended: self.drag_source_system_drag_ended.into(),
            get_visible_navigation_entry: self.get_visible_navigation_entry.into(),
            set_accessibility_state: self.set_accessibility_state.into(),
            set_auto_resize_enabled: self.set_auto_resize_enabled.into(),
            set_audio_muted: self.set_audio_muted.into(),
            is_audio_muted: self.is_audio_muted.into(),
            is_fullscreen: self.is_fullscreen.into(),
            exit_fullscreen: self.exit_fullscreen.into(),
            can_execute_chrome_command: self.can_execute_chrome_command.into(),
            execute_chrome_command: self.execute_chrome_command.into(),
            is_render_process_unresponsive: self.is_render_process_unresponsive.into(),
            get_runtime_style: self.get_runtime_style.into(),
        }
    }
}
impl Default for BrowserHost {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_audio_handler_t] for more documentation.
#[derive(Clone)]
pub struct AudioHandler {
    pub base: BaseRefCounted,
    pub get_audio_parameters: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_audio_handler_t,
            browser: *mut _cef_browser_t,
            params: *mut cef_audio_parameters_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_audio_stream_started: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_audio_handler_t,
            browser: *mut _cef_browser_t,
            params: *const cef_audio_parameters_t,
            channels: ::std::os::raw::c_int,
        ),
    >,
    pub on_audio_stream_packet: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_audio_handler_t,
            browser: *mut _cef_browser_t,
            data: *mut *const f32,
            frames: ::std::os::raw::c_int,
            pts: i64,
        ),
    >,
    pub on_audio_stream_stopped: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_audio_handler_t, browser: *mut _cef_browser_t),
    >,
    pub on_audio_stream_error: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_audio_handler_t,
            browser: *mut _cef_browser_t,
            message: *const cef_string_t,
        ),
    >,
}
impl From<_cef_audio_handler_t> for AudioHandler {
    fn from(value: _cef_audio_handler_t) -> Self {
        Self {
            base: value.base.into(),
            get_audio_parameters: value.get_audio_parameters.into(),
            on_audio_stream_started: value.on_audio_stream_started.into(),
            on_audio_stream_packet: value.on_audio_stream_packet.into(),
            on_audio_stream_stopped: value.on_audio_stream_stopped.into(),
            on_audio_stream_error: value.on_audio_stream_error.into(),
        }
    }
}
impl Into<_cef_audio_handler_t> for AudioHandler {
    fn into(self) -> _cef_audio_handler_t {
        _cef_audio_handler_t {
            base: self.base.into(),
            get_audio_parameters: self.get_audio_parameters.into(),
            on_audio_stream_started: self.on_audio_stream_started.into(),
            on_audio_stream_packet: self.on_audio_stream_packet.into(),
            on_audio_stream_stopped: self.on_audio_stream_stopped.into(),
            on_audio_stream_error: self.on_audio_stream_error.into(),
        }
    }
}
impl Default for AudioHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_command_handler_t] for more documentation.
#[derive(Clone)]
pub struct CommandHandler {
    pub base: BaseRefCounted,
    pub on_chrome_command: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_command_handler_t,
            browser: *mut _cef_browser_t,
            command_id: ::std::os::raw::c_int,
            disposition: cef_window_open_disposition_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_chrome_app_menu_item_visible: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_command_handler_t,
            browser: *mut _cef_browser_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_chrome_app_menu_item_enabled: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_command_handler_t,
            browser: *mut _cef_browser_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_chrome_page_action_icon_visible: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_command_handler_t,
            icon_type: cef_chrome_page_action_icon_type_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_chrome_toolbar_button_visible: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_command_handler_t,
            button_type: cef_chrome_toolbar_button_type_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_command_handler_t> for CommandHandler {
    fn from(value: _cef_command_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_chrome_command: value.on_chrome_command.into(),
            is_chrome_app_menu_item_visible: value.is_chrome_app_menu_item_visible.into(),
            is_chrome_app_menu_item_enabled: value.is_chrome_app_menu_item_enabled.into(),
            is_chrome_page_action_icon_visible: value.is_chrome_page_action_icon_visible.into(),
            is_chrome_toolbar_button_visible: value.is_chrome_toolbar_button_visible.into(),
        }
    }
}
impl Into<_cef_command_handler_t> for CommandHandler {
    fn into(self) -> _cef_command_handler_t {
        _cef_command_handler_t {
            base: self.base.into(),
            on_chrome_command: self.on_chrome_command.into(),
            is_chrome_app_menu_item_visible: self.is_chrome_app_menu_item_visible.into(),
            is_chrome_app_menu_item_enabled: self.is_chrome_app_menu_item_enabled.into(),
            is_chrome_page_action_icon_visible: self.is_chrome_page_action_icon_visible.into(),
            is_chrome_toolbar_button_visible: self.is_chrome_toolbar_button_visible.into(),
        }
    }
}
impl Default for CommandHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_menu_model_delegate_t] for more documentation.
#[derive(Clone)]
pub struct MenuModelDelegate {
    pub base: BaseRefCounted,
    pub execute_command: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_delegate_t,
            menu_model: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            event_flags: cef_event_flags_t,
        ),
    >,
    pub mouse_outside_menu: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_delegate_t,
            menu_model: *mut _cef_menu_model_t,
            screen_point: *const cef_point_t,
        ),
    >,
    pub unhandled_open_submenu: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_delegate_t,
            menu_model: *mut _cef_menu_model_t,
            is_rtl: ::std::os::raw::c_int,
        ),
    >,
    pub unhandled_close_submenu: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_delegate_t,
            menu_model: *mut _cef_menu_model_t,
            is_rtl: ::std::os::raw::c_int,
        ),
    >,
    pub menu_will_show: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_delegate_t,
            menu_model: *mut _cef_menu_model_t,
        ),
    >,
    pub menu_closed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_delegate_t,
            menu_model: *mut _cef_menu_model_t,
        ),
    >,
    pub format_label: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_delegate_t,
            menu_model: *mut _cef_menu_model_t,
            label: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_menu_model_delegate_t> for MenuModelDelegate {
    fn from(value: _cef_menu_model_delegate_t) -> Self {
        Self {
            base: value.base.into(),
            execute_command: value.execute_command.into(),
            mouse_outside_menu: value.mouse_outside_menu.into(),
            unhandled_open_submenu: value.unhandled_open_submenu.into(),
            unhandled_close_submenu: value.unhandled_close_submenu.into(),
            menu_will_show: value.menu_will_show.into(),
            menu_closed: value.menu_closed.into(),
            format_label: value.format_label.into(),
        }
    }
}
impl Into<_cef_menu_model_delegate_t> for MenuModelDelegate {
    fn into(self) -> _cef_menu_model_delegate_t {
        _cef_menu_model_delegate_t {
            base: self.base.into(),
            execute_command: self.execute_command.into(),
            mouse_outside_menu: self.mouse_outside_menu.into(),
            unhandled_open_submenu: self.unhandled_open_submenu.into(),
            unhandled_close_submenu: self.unhandled_close_submenu.into(),
            menu_will_show: self.menu_will_show.into(),
            menu_closed: self.menu_closed.into(),
            format_label: self.format_label.into(),
        }
    }
}
impl Default for MenuModelDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_menu_model_t] for more documentation.
#[derive(Clone)]
pub struct MenuModel {
    pub base: BaseRefCounted,
    pub is_sub_menu: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_menu_model_t) -> ::std::os::raw::c_int,
    >,
    pub clear: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_menu_model_t) -> ::std::os::raw::c_int,
    >,
    pub get_count:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_menu_model_t) -> usize>,
    pub add_separator: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_menu_model_t) -> ::std::os::raw::c_int,
    >,
    pub add_item: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            label: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub add_check_item: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            label: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub add_radio_item: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            label: *const cef_string_t,
            group_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub add_sub_menu: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            label: *const cef_string_t,
        ) -> *mut _cef_menu_model_t,
    >,
    pub insert_separator_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub insert_item_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            command_id: ::std::os::raw::c_int,
            label: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub insert_check_item_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            command_id: ::std::os::raw::c_int,
            label: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub insert_radio_item_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            command_id: ::std::os::raw::c_int,
            label: *const cef_string_t,
            group_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub insert_sub_menu_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            command_id: ::std::os::raw::c_int,
            label: *const cef_string_t,
        ) -> *mut _cef_menu_model_t,
    >,
    pub remove: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub remove_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_index_of: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_command_id_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_command_id_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_label: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> cef_string_userfree_t,
    >,
    pub get_label_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> cef_string_userfree_t,
    >,
    pub set_label: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            label: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_label_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            label: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_type: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> cef_menu_item_type_t,
    >,
    pub get_type_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> cef_menu_item_type_t,
    >,
    pub get_group_id: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_group_id_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_group_id: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            group_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_group_id_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            group_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_sub_menu: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> *mut _cef_menu_model_t,
    >,
    pub get_sub_menu_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> *mut _cef_menu_model_t,
    >,
    pub is_visible: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_visible_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_visible: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            visible: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_visible_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            visible: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_enabled: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_enabled_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_enabled: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            enabled: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_enabled_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            enabled: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_checked: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_checked_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_checked: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            checked: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_checked_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            checked: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub has_accelerator: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub has_accelerator_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_accelerator: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            key_code: ::std::os::raw::c_int,
            shift_pressed: ::std::os::raw::c_int,
            ctrl_pressed: ::std::os::raw::c_int,
            alt_pressed: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_accelerator_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            key_code: ::std::os::raw::c_int,
            shift_pressed: ::std::os::raw::c_int,
            ctrl_pressed: ::std::os::raw::c_int,
            alt_pressed: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub remove_accelerator: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub remove_accelerator_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_accelerator: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            key_code: *mut ::std::os::raw::c_int,
            shift_pressed: *mut ::std::os::raw::c_int,
            ctrl_pressed: *mut ::std::os::raw::c_int,
            alt_pressed: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_accelerator_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: usize,
            key_code: *mut ::std::os::raw::c_int,
            shift_pressed: *mut ::std::os::raw::c_int,
            ctrl_pressed: *mut ::std::os::raw::c_int,
            alt_pressed: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_color: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            color_type: cef_menu_color_type_t,
            color: cef_color_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_color_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: ::std::os::raw::c_int,
            color_type: cef_menu_color_type_t,
            color: cef_color_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_color: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            color_type: cef_menu_color_type_t,
            color: *mut cef_color_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_color_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: ::std::os::raw::c_int,
            color_type: cef_menu_color_type_t,
            color: *mut cef_color_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_font_list: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            command_id: ::std::os::raw::c_int,
            font_list: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_font_list_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_model_t,
            index: ::std::os::raw::c_int,
            font_list: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_menu_model_t> for MenuModel {
    fn from(value: _cef_menu_model_t) -> Self {
        Self {
            base: value.base.into(),
            is_sub_menu: value.is_sub_menu.into(),
            clear: value.clear.into(),
            get_count: value.get_count.into(),
            add_separator: value.add_separator.into(),
            add_item: value.add_item.into(),
            add_check_item: value.add_check_item.into(),
            add_radio_item: value.add_radio_item.into(),
            add_sub_menu: value.add_sub_menu.into(),
            insert_separator_at: value.insert_separator_at.into(),
            insert_item_at: value.insert_item_at.into(),
            insert_check_item_at: value.insert_check_item_at.into(),
            insert_radio_item_at: value.insert_radio_item_at.into(),
            insert_sub_menu_at: value.insert_sub_menu_at.into(),
            remove: value.remove.into(),
            remove_at: value.remove_at.into(),
            get_index_of: value.get_index_of.into(),
            get_command_id_at: value.get_command_id_at.into(),
            set_command_id_at: value.set_command_id_at.into(),
            get_label: value.get_label.into(),
            get_label_at: value.get_label_at.into(),
            set_label: value.set_label.into(),
            set_label_at: value.set_label_at.into(),
            get_type: value.get_type.into(),
            get_type_at: value.get_type_at.into(),
            get_group_id: value.get_group_id.into(),
            get_group_id_at: value.get_group_id_at.into(),
            set_group_id: value.set_group_id.into(),
            set_group_id_at: value.set_group_id_at.into(),
            get_sub_menu: value.get_sub_menu.into(),
            get_sub_menu_at: value.get_sub_menu_at.into(),
            is_visible: value.is_visible.into(),
            is_visible_at: value.is_visible_at.into(),
            set_visible: value.set_visible.into(),
            set_visible_at: value.set_visible_at.into(),
            is_enabled: value.is_enabled.into(),
            is_enabled_at: value.is_enabled_at.into(),
            set_enabled: value.set_enabled.into(),
            set_enabled_at: value.set_enabled_at.into(),
            is_checked: value.is_checked.into(),
            is_checked_at: value.is_checked_at.into(),
            set_checked: value.set_checked.into(),
            set_checked_at: value.set_checked_at.into(),
            has_accelerator: value.has_accelerator.into(),
            has_accelerator_at: value.has_accelerator_at.into(),
            set_accelerator: value.set_accelerator.into(),
            set_accelerator_at: value.set_accelerator_at.into(),
            remove_accelerator: value.remove_accelerator.into(),
            remove_accelerator_at: value.remove_accelerator_at.into(),
            get_accelerator: value.get_accelerator.into(),
            get_accelerator_at: value.get_accelerator_at.into(),
            set_color: value.set_color.into(),
            set_color_at: value.set_color_at.into(),
            get_color: value.get_color.into(),
            get_color_at: value.get_color_at.into(),
            set_font_list: value.set_font_list.into(),
            set_font_list_at: value.set_font_list_at.into(),
        }
    }
}
impl Into<_cef_menu_model_t> for MenuModel {
    fn into(self) -> _cef_menu_model_t {
        _cef_menu_model_t {
            base: self.base.into(),
            is_sub_menu: self.is_sub_menu.into(),
            clear: self.clear.into(),
            get_count: self.get_count.into(),
            add_separator: self.add_separator.into(),
            add_item: self.add_item.into(),
            add_check_item: self.add_check_item.into(),
            add_radio_item: self.add_radio_item.into(),
            add_sub_menu: self.add_sub_menu.into(),
            insert_separator_at: self.insert_separator_at.into(),
            insert_item_at: self.insert_item_at.into(),
            insert_check_item_at: self.insert_check_item_at.into(),
            insert_radio_item_at: self.insert_radio_item_at.into(),
            insert_sub_menu_at: self.insert_sub_menu_at.into(),
            remove: self.remove.into(),
            remove_at: self.remove_at.into(),
            get_index_of: self.get_index_of.into(),
            get_command_id_at: self.get_command_id_at.into(),
            set_command_id_at: self.set_command_id_at.into(),
            get_label: self.get_label.into(),
            get_label_at: self.get_label_at.into(),
            set_label: self.set_label.into(),
            set_label_at: self.set_label_at.into(),
            get_type: self.get_type.into(),
            get_type_at: self.get_type_at.into(),
            get_group_id: self.get_group_id.into(),
            get_group_id_at: self.get_group_id_at.into(),
            set_group_id: self.set_group_id.into(),
            set_group_id_at: self.set_group_id_at.into(),
            get_sub_menu: self.get_sub_menu.into(),
            get_sub_menu_at: self.get_sub_menu_at.into(),
            is_visible: self.is_visible.into(),
            is_visible_at: self.is_visible_at.into(),
            set_visible: self.set_visible.into(),
            set_visible_at: self.set_visible_at.into(),
            is_enabled: self.is_enabled.into(),
            is_enabled_at: self.is_enabled_at.into(),
            set_enabled: self.set_enabled.into(),
            set_enabled_at: self.set_enabled_at.into(),
            is_checked: self.is_checked.into(),
            is_checked_at: self.is_checked_at.into(),
            set_checked: self.set_checked.into(),
            set_checked_at: self.set_checked_at.into(),
            has_accelerator: self.has_accelerator.into(),
            has_accelerator_at: self.has_accelerator_at.into(),
            set_accelerator: self.set_accelerator.into(),
            set_accelerator_at: self.set_accelerator_at.into(),
            remove_accelerator: self.remove_accelerator.into(),
            remove_accelerator_at: self.remove_accelerator_at.into(),
            get_accelerator: self.get_accelerator.into(),
            get_accelerator_at: self.get_accelerator_at.into(),
            set_color: self.set_color.into(),
            set_color_at: self.set_color_at.into(),
            get_color: self.get_color.into(),
            get_color_at: self.get_color_at.into(),
            set_font_list: self.set_font_list.into(),
            set_font_list_at: self.set_font_list_at.into(),
        }
    }
}
impl Default for MenuModel {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_run_context_menu_callback_t] for more documentation.
#[derive(Clone)]
pub struct RunContextMenuCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_run_context_menu_callback_t,
            command_id: ::std::os::raw::c_int,
            event_flags: cef_event_flags_t,
        ),
    >,
    pub cancel: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_run_context_menu_callback_t),
    >,
}
impl From<_cef_run_context_menu_callback_t> for RunContextMenuCallback {
    fn from(value: _cef_run_context_menu_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
            cancel: value.cancel.into(),
        }
    }
}
impl Into<_cef_run_context_menu_callback_t> for RunContextMenuCallback {
    fn into(self) -> _cef_run_context_menu_callback_t {
        _cef_run_context_menu_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
            cancel: self.cancel.into(),
        }
    }
}
impl Default for RunContextMenuCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_run_quick_menu_callback_t] for more documentation.
#[derive(Clone)]
pub struct RunQuickMenuCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_run_quick_menu_callback_t,
            command_id: ::std::os::raw::c_int,
            event_flags: cef_event_flags_t,
        ),
    >,
    pub cancel: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_run_quick_menu_callback_t),
    >,
}
impl From<_cef_run_quick_menu_callback_t> for RunQuickMenuCallback {
    fn from(value: _cef_run_quick_menu_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
            cancel: value.cancel.into(),
        }
    }
}
impl Into<_cef_run_quick_menu_callback_t> for RunQuickMenuCallback {
    fn into(self) -> _cef_run_quick_menu_callback_t {
        _cef_run_quick_menu_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
            cancel: self.cancel.into(),
        }
    }
}
impl Default for RunQuickMenuCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_context_menu_handler_t] for more documentation.
#[derive(Clone)]
pub struct ContextMenuHandler {
    pub base: BaseRefCounted,
    pub on_before_context_menu: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            params: *mut _cef_context_menu_params_t,
            model: *mut _cef_menu_model_t,
        ),
    >,
    pub run_context_menu: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            params: *mut _cef_context_menu_params_t,
            model: *mut _cef_menu_model_t,
            callback: *mut _cef_run_context_menu_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_context_menu_command: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            params: *mut _cef_context_menu_params_t,
            command_id: ::std::os::raw::c_int,
            event_flags: cef_event_flags_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_context_menu_dismissed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
        ),
    >,
    pub run_quick_menu: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            location: *const cef_point_t,
            size: *const cef_size_t,
            edit_state_flags: cef_quick_menu_edit_state_flags_t,
            callback: *mut _cef_run_quick_menu_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_quick_menu_command: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            command_id: ::std::os::raw::c_int,
            event_flags: cef_event_flags_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_quick_menu_dismissed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
        ),
    >,
}
impl From<_cef_context_menu_handler_t> for ContextMenuHandler {
    fn from(value: _cef_context_menu_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_before_context_menu: value.on_before_context_menu.into(),
            run_context_menu: value.run_context_menu.into(),
            on_context_menu_command: value.on_context_menu_command.into(),
            on_context_menu_dismissed: value.on_context_menu_dismissed.into(),
            run_quick_menu: value.run_quick_menu.into(),
            on_quick_menu_command: value.on_quick_menu_command.into(),
            on_quick_menu_dismissed: value.on_quick_menu_dismissed.into(),
        }
    }
}
impl Into<_cef_context_menu_handler_t> for ContextMenuHandler {
    fn into(self) -> _cef_context_menu_handler_t {
        _cef_context_menu_handler_t {
            base: self.base.into(),
            on_before_context_menu: self.on_before_context_menu.into(),
            run_context_menu: self.run_context_menu.into(),
            on_context_menu_command: self.on_context_menu_command.into(),
            on_context_menu_dismissed: self.on_context_menu_dismissed.into(),
            run_quick_menu: self.run_quick_menu.into(),
            on_quick_menu_command: self.on_quick_menu_command.into(),
            on_quick_menu_dismissed: self.on_quick_menu_dismissed.into(),
        }
    }
}
impl Default for ContextMenuHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_context_menu_params_t] for more documentation.
#[derive(Clone)]
pub struct ContextMenuParams {
    pub base: BaseRefCounted,
    pub get_xcoord: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> ::std::os::raw::c_int,
    >,
    pub get_ycoord: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> ::std::os::raw::c_int,
    >,
    pub get_type_flags: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_params_t,
        ) -> cef_context_menu_type_flags_t,
    >,
    pub get_link_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> cef_string_userfree_t,
    >,
    pub get_unfiltered_link_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> cef_string_userfree_t,
    >,
    pub get_source_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> cef_string_userfree_t,
    >,
    pub has_image_contents: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> ::std::os::raw::c_int,
    >,
    pub get_title_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> cef_string_userfree_t,
    >,
    pub get_page_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> cef_string_userfree_t,
    >,
    pub get_frame_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> cef_string_userfree_t,
    >,
    pub get_frame_charset: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> cef_string_userfree_t,
    >,
    pub get_media_type: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_params_t,
        ) -> cef_context_menu_media_type_t,
    >,
    pub get_media_state_flags: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_params_t,
        ) -> cef_context_menu_media_state_flags_t,
    >,
    pub get_selection_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> cef_string_userfree_t,
    >,
    pub get_misspelled_word: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> cef_string_userfree_t,
    >,
    pub get_dictionary_suggestions: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_params_t,
            suggestions: cef_string_list_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub is_editable: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> ::std::os::raw::c_int,
    >,
    pub is_spell_check_enabled: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> ::std::os::raw::c_int,
    >,
    pub get_edit_state_flags: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_context_menu_params_t,
        ) -> cef_context_menu_edit_state_flags_t,
    >,
    pub is_custom_menu: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_context_menu_params_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_context_menu_params_t> for ContextMenuParams {
    fn from(value: _cef_context_menu_params_t) -> Self {
        Self {
            base: value.base.into(),
            get_xcoord: value.get_xcoord.into(),
            get_ycoord: value.get_ycoord.into(),
            get_type_flags: value.get_type_flags.into(),
            get_link_url: value.get_link_url.into(),
            get_unfiltered_link_url: value.get_unfiltered_link_url.into(),
            get_source_url: value.get_source_url.into(),
            has_image_contents: value.has_image_contents.into(),
            get_title_text: value.get_title_text.into(),
            get_page_url: value.get_page_url.into(),
            get_frame_url: value.get_frame_url.into(),
            get_frame_charset: value.get_frame_charset.into(),
            get_media_type: value.get_media_type.into(),
            get_media_state_flags: value.get_media_state_flags.into(),
            get_selection_text: value.get_selection_text.into(),
            get_misspelled_word: value.get_misspelled_word.into(),
            get_dictionary_suggestions: value.get_dictionary_suggestions.into(),
            is_editable: value.is_editable.into(),
            is_spell_check_enabled: value.is_spell_check_enabled.into(),
            get_edit_state_flags: value.get_edit_state_flags.into(),
            is_custom_menu: value.is_custom_menu.into(),
        }
    }
}
impl Into<_cef_context_menu_params_t> for ContextMenuParams {
    fn into(self) -> _cef_context_menu_params_t {
        _cef_context_menu_params_t {
            base: self.base.into(),
            get_xcoord: self.get_xcoord.into(),
            get_ycoord: self.get_ycoord.into(),
            get_type_flags: self.get_type_flags.into(),
            get_link_url: self.get_link_url.into(),
            get_unfiltered_link_url: self.get_unfiltered_link_url.into(),
            get_source_url: self.get_source_url.into(),
            has_image_contents: self.has_image_contents.into(),
            get_title_text: self.get_title_text.into(),
            get_page_url: self.get_page_url.into(),
            get_frame_url: self.get_frame_url.into(),
            get_frame_charset: self.get_frame_charset.into(),
            get_media_type: self.get_media_type.into(),
            get_media_state_flags: self.get_media_state_flags.into(),
            get_selection_text: self.get_selection_text.into(),
            get_misspelled_word: self.get_misspelled_word.into(),
            get_dictionary_suggestions: self.get_dictionary_suggestions.into(),
            is_editable: self.is_editable.into(),
            is_spell_check_enabled: self.is_spell_check_enabled.into(),
            get_edit_state_flags: self.get_edit_state_flags.into(),
            is_custom_menu: self.is_custom_menu.into(),
        }
    }
}
impl Default for ContextMenuParams {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_file_dialog_callback_t] for more documentation.
#[derive(Clone)]
pub struct FileDialogCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_file_dialog_callback_t,
            file_paths: cef_string_list_t,
        ),
    >,
    pub cancel:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_file_dialog_callback_t)>,
}
impl From<_cef_file_dialog_callback_t> for FileDialogCallback {
    fn from(value: _cef_file_dialog_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
            cancel: value.cancel.into(),
        }
    }
}
impl Into<_cef_file_dialog_callback_t> for FileDialogCallback {
    fn into(self) -> _cef_file_dialog_callback_t {
        _cef_file_dialog_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
            cancel: self.cancel.into(),
        }
    }
}
impl Default for FileDialogCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_dialog_handler_t] for more documentation.
#[derive(Clone)]
pub struct DialogHandler {
    pub base: BaseRefCounted,
    pub on_file_dialog: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_dialog_handler_t,
            browser: *mut _cef_browser_t,
            mode: cef_file_dialog_mode_t,
            title: *const cef_string_t,
            default_file_path: *const cef_string_t,
            accept_filters: cef_string_list_t,
            accept_extensions: cef_string_list_t,
            accept_descriptions: cef_string_list_t,
            callback: *mut _cef_file_dialog_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_dialog_handler_t> for DialogHandler {
    fn from(value: _cef_dialog_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_file_dialog: value.on_file_dialog.into(),
        }
    }
}
impl Into<_cef_dialog_handler_t> for DialogHandler {
    fn into(self) -> _cef_dialog_handler_t {
        _cef_dialog_handler_t {
            base: self.base.into(),
            on_file_dialog: self.on_file_dialog.into(),
        }
    }
}
impl Default for DialogHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_display_handler_t] for more documentation.
#[derive(Clone)]
pub struct DisplayHandler {
    pub base: BaseRefCounted,
    pub on_address_change: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            url: *const cef_string_t,
        ),
    >,
    pub on_title_change: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            title: *const cef_string_t,
        ),
    >,
    pub on_favicon_urlchange: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            icon_urls: cef_string_list_t,
        ),
    >,
    pub on_fullscreen_mode_change: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            fullscreen: ::std::os::raw::c_int,
        ),
    >,
    pub on_tooltip: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            text: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_status_message: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            value: *const cef_string_t,
        ),
    >,
    pub on_console_message: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            level: cef_log_severity_t,
            message: *const cef_string_t,
            source: *const cef_string_t,
            line: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_auto_resize: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            new_size: *const cef_size_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_loading_progress_change: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            progress: f64,
        ),
    >,
    pub on_cursor_change: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            cursor: cef_cursor_handle_t,
            type_: cef_cursor_type_t,
            custom_cursor_info: *const cef_cursor_info_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_media_access_change: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            has_video_access: ::std::os::raw::c_int,
            has_audio_access: ::std::os::raw::c_int,
        ),
    >,
}
impl From<_cef_display_handler_t> for DisplayHandler {
    fn from(value: _cef_display_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_address_change: value.on_address_change.into(),
            on_title_change: value.on_title_change.into(),
            on_favicon_urlchange: value.on_favicon_urlchange.into(),
            on_fullscreen_mode_change: value.on_fullscreen_mode_change.into(),
            on_tooltip: value.on_tooltip.into(),
            on_status_message: value.on_status_message.into(),
            on_console_message: value.on_console_message.into(),
            on_auto_resize: value.on_auto_resize.into(),
            on_loading_progress_change: value.on_loading_progress_change.into(),
            on_cursor_change: value.on_cursor_change.into(),
            on_media_access_change: value.on_media_access_change.into(),
        }
    }
}
impl Into<_cef_display_handler_t> for DisplayHandler {
    fn into(self) -> _cef_display_handler_t {
        _cef_display_handler_t {
            base: self.base.into(),
            on_address_change: self.on_address_change.into(),
            on_title_change: self.on_title_change.into(),
            on_favicon_urlchange: self.on_favicon_urlchange.into(),
            on_fullscreen_mode_change: self.on_fullscreen_mode_change.into(),
            on_tooltip: self.on_tooltip.into(),
            on_status_message: self.on_status_message.into(),
            on_console_message: self.on_console_message.into(),
            on_auto_resize: self.on_auto_resize.into(),
            on_loading_progress_change: self.on_loading_progress_change.into(),
            on_cursor_change: self.on_cursor_change.into(),
            on_media_access_change: self.on_media_access_change.into(),
        }
    }
}
impl Default for DisplayHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_download_item_t] for more documentation.
#[derive(Clone)]
pub struct DownloadItem {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> ::std::os::raw::c_int,
    >,
    pub is_in_progress: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> ::std::os::raw::c_int,
    >,
    pub is_complete: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> ::std::os::raw::c_int,
    >,
    pub is_canceled: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> ::std::os::raw::c_int,
    >,
    pub is_interrupted: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> ::std::os::raw::c_int,
    >,
    pub get_interrupt_reason: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_download_item_t,
        ) -> cef_download_interrupt_reason_t,
    >,
    pub get_current_speed:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> i64>,
    pub get_percent_complete: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> ::std::os::raw::c_int,
    >,
    pub get_total_bytes:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> i64>,
    pub get_received_bytes:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> i64>,
    pub get_start_time: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> cef_basetime_t,
    >,
    pub get_end_time: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> cef_basetime_t,
    >,
    pub get_full_path: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> cef_string_userfree_t,
    >,
    pub get_id:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> u32>,
    pub get_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> cef_string_userfree_t,
    >,
    pub get_original_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> cef_string_userfree_t,
    >,
    pub get_suggested_file_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> cef_string_userfree_t,
    >,
    pub get_content_disposition: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> cef_string_userfree_t,
    >,
    pub get_mime_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_t) -> cef_string_userfree_t,
    >,
}
impl From<_cef_download_item_t> for DownloadItem {
    fn from(value: _cef_download_item_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            is_in_progress: value.is_in_progress.into(),
            is_complete: value.is_complete.into(),
            is_canceled: value.is_canceled.into(),
            is_interrupted: value.is_interrupted.into(),
            get_interrupt_reason: value.get_interrupt_reason.into(),
            get_current_speed: value.get_current_speed.into(),
            get_percent_complete: value.get_percent_complete.into(),
            get_total_bytes: value.get_total_bytes.into(),
            get_received_bytes: value.get_received_bytes.into(),
            get_start_time: value.get_start_time.into(),
            get_end_time: value.get_end_time.into(),
            get_full_path: value.get_full_path.into(),
            get_id: value.get_id.into(),
            get_url: value.get_url.into(),
            get_original_url: value.get_original_url.into(),
            get_suggested_file_name: value.get_suggested_file_name.into(),
            get_content_disposition: value.get_content_disposition.into(),
            get_mime_type: value.get_mime_type.into(),
        }
    }
}
impl Into<_cef_download_item_t> for DownloadItem {
    fn into(self) -> _cef_download_item_t {
        _cef_download_item_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            is_in_progress: self.is_in_progress.into(),
            is_complete: self.is_complete.into(),
            is_canceled: self.is_canceled.into(),
            is_interrupted: self.is_interrupted.into(),
            get_interrupt_reason: self.get_interrupt_reason.into(),
            get_current_speed: self.get_current_speed.into(),
            get_percent_complete: self.get_percent_complete.into(),
            get_total_bytes: self.get_total_bytes.into(),
            get_received_bytes: self.get_received_bytes.into(),
            get_start_time: self.get_start_time.into(),
            get_end_time: self.get_end_time.into(),
            get_full_path: self.get_full_path.into(),
            get_id: self.get_id.into(),
            get_url: self.get_url.into(),
            get_original_url: self.get_original_url.into(),
            get_suggested_file_name: self.get_suggested_file_name.into(),
            get_content_disposition: self.get_content_disposition.into(),
            get_mime_type: self.get_mime_type.into(),
        }
    }
}
impl Default for DownloadItem {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_before_download_callback_t] for more documentation.
#[derive(Clone)]
pub struct BeforeDownloadCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_before_download_callback_t,
            download_path: *const cef_string_t,
            show_dialog: ::std::os::raw::c_int,
        ),
    >,
}
impl From<_cef_before_download_callback_t> for BeforeDownloadCallback {
    fn from(value: _cef_before_download_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
        }
    }
}
impl Into<_cef_before_download_callback_t> for BeforeDownloadCallback {
    fn into(self) -> _cef_before_download_callback_t {
        _cef_before_download_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
        }
    }
}
impl Default for BeforeDownloadCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_download_item_callback_t] for more documentation.
#[derive(Clone)]
pub struct DownloadItemCallback {
    pub base: BaseRefCounted,
    pub cancel: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_callback_t),
    >,
    pub pause: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_callback_t),
    >,
    pub resume: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_download_item_callback_t),
    >,
}
impl From<_cef_download_item_callback_t> for DownloadItemCallback {
    fn from(value: _cef_download_item_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cancel: value.cancel.into(),
            pause: value.pause.into(),
            resume: value.resume.into(),
        }
    }
}
impl Into<_cef_download_item_callback_t> for DownloadItemCallback {
    fn into(self) -> _cef_download_item_callback_t {
        _cef_download_item_callback_t {
            base: self.base.into(),
            cancel: self.cancel.into(),
            pause: self.pause.into(),
            resume: self.resume.into(),
        }
    }
}
impl Default for DownloadItemCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_download_handler_t] for more documentation.
#[derive(Clone)]
pub struct DownloadHandler {
    pub base: BaseRefCounted,
    pub can_download: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_download_handler_t,
            browser: *mut _cef_browser_t,
            url: *const cef_string_t,
            request_method: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_before_download: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_download_handler_t,
            browser: *mut _cef_browser_t,
            download_item: *mut _cef_download_item_t,
            suggested_name: *const cef_string_t,
            callback: *mut _cef_before_download_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_download_updated: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_download_handler_t,
            browser: *mut _cef_browser_t,
            download_item: *mut _cef_download_item_t,
            callback: *mut _cef_download_item_callback_t,
        ),
    >,
}
impl From<_cef_download_handler_t> for DownloadHandler {
    fn from(value: _cef_download_handler_t) -> Self {
        Self {
            base: value.base.into(),
            can_download: value.can_download.into(),
            on_before_download: value.on_before_download.into(),
            on_download_updated: value.on_download_updated.into(),
        }
    }
}
impl Into<_cef_download_handler_t> for DownloadHandler {
    fn into(self) -> _cef_download_handler_t {
        _cef_download_handler_t {
            base: self.base.into(),
            can_download: self.can_download.into(),
            on_before_download: self.on_before_download.into(),
            on_download_updated: self.on_download_updated.into(),
        }
    }
}
impl Default for DownloadHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_drag_handler_t] for more documentation.
#[derive(Clone)]
pub struct DragHandler {
    pub base: BaseRefCounted,
    pub on_drag_enter: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_drag_handler_t,
            browser: *mut _cef_browser_t,
            dragData: *mut _cef_drag_data_t,
            mask: cef_drag_operations_mask_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_draggable_regions_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_drag_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            regionsCount: usize,
            regions: *const cef_draggable_region_t,
        ),
    >,
}
impl From<_cef_drag_handler_t> for DragHandler {
    fn from(value: _cef_drag_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_drag_enter: value.on_drag_enter.into(),
            on_draggable_regions_changed: value.on_draggable_regions_changed.into(),
        }
    }
}
impl Into<_cef_drag_handler_t> for DragHandler {
    fn into(self) -> _cef_drag_handler_t {
        _cef_drag_handler_t {
            base: self.base.into(),
            on_drag_enter: self.on_drag_enter.into(),
            on_draggable_regions_changed: self.on_draggable_regions_changed.into(),
        }
    }
}
impl Default for DragHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_find_handler_t] for more documentation.
#[derive(Clone)]
pub struct FindHandler {
    pub base: BaseRefCounted,
    pub on_find_result: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_find_handler_t,
            browser: *mut _cef_browser_t,
            identifier: ::std::os::raw::c_int,
            count: ::std::os::raw::c_int,
            selectionRect: *const cef_rect_t,
            activeMatchOrdinal: ::std::os::raw::c_int,
            finalUpdate: ::std::os::raw::c_int,
        ),
    >,
}
impl From<_cef_find_handler_t> for FindHandler {
    fn from(value: _cef_find_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_find_result: value.on_find_result.into(),
        }
    }
}
impl Into<_cef_find_handler_t> for FindHandler {
    fn into(self) -> _cef_find_handler_t {
        _cef_find_handler_t {
            base: self.base.into(),
            on_find_result: self.on_find_result.into(),
        }
    }
}
impl Default for FindHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_focus_handler_t] for more documentation.
#[derive(Clone)]
pub struct FocusHandler {
    pub base: BaseRefCounted,
    pub on_take_focus: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_focus_handler_t,
            browser: *mut _cef_browser_t,
            next: ::std::os::raw::c_int,
        ),
    >,
    pub on_set_focus: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_focus_handler_t,
            browser: *mut _cef_browser_t,
            source: cef_focus_source_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_got_focus: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_focus_handler_t, browser: *mut _cef_browser_t),
    >,
}
impl From<_cef_focus_handler_t> for FocusHandler {
    fn from(value: _cef_focus_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_take_focus: value.on_take_focus.into(),
            on_set_focus: value.on_set_focus.into(),
            on_got_focus: value.on_got_focus.into(),
        }
    }
}
impl Into<_cef_focus_handler_t> for FocusHandler {
    fn into(self) -> _cef_focus_handler_t {
        _cef_focus_handler_t {
            base: self.base.into(),
            on_take_focus: self.on_take_focus.into(),
            on_set_focus: self.on_set_focus.into(),
            on_got_focus: self.on_got_focus.into(),
        }
    }
}
impl Default for FocusHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_frame_handler_t] for more documentation.
#[derive(Clone)]
pub struct FrameHandler {
    pub base: BaseRefCounted,
    pub on_frame_created: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_frame_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
        ),
    >,
    pub on_frame_destroyed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_frame_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
        ),
    >,
    pub on_frame_attached: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_frame_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            reattached: ::std::os::raw::c_int,
        ),
    >,
    pub on_frame_detached: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_frame_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
        ),
    >,
    pub on_main_frame_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_frame_handler_t,
            browser: *mut _cef_browser_t,
            old_frame: *mut _cef_frame_t,
            new_frame: *mut _cef_frame_t,
        ),
    >,
}
impl From<_cef_frame_handler_t> for FrameHandler {
    fn from(value: _cef_frame_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_frame_created: value.on_frame_created.into(),
            on_frame_destroyed: value.on_frame_destroyed.into(),
            on_frame_attached: value.on_frame_attached.into(),
            on_frame_detached: value.on_frame_detached.into(),
            on_main_frame_changed: value.on_main_frame_changed.into(),
        }
    }
}
impl Into<_cef_frame_handler_t> for FrameHandler {
    fn into(self) -> _cef_frame_handler_t {
        _cef_frame_handler_t {
            base: self.base.into(),
            on_frame_created: self.on_frame_created.into(),
            on_frame_destroyed: self.on_frame_destroyed.into(),
            on_frame_attached: self.on_frame_attached.into(),
            on_frame_detached: self.on_frame_detached.into(),
            on_main_frame_changed: self.on_main_frame_changed.into(),
        }
    }
}
impl Default for FrameHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_jsdialog_callback_t] for more documentation.
#[derive(Clone)]
pub struct JsdialogCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_jsdialog_callback_t,
            success: ::std::os::raw::c_int,
            user_input: *const cef_string_t,
        ),
    >,
}
impl From<_cef_jsdialog_callback_t> for JsdialogCallback {
    fn from(value: _cef_jsdialog_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
        }
    }
}
impl Into<_cef_jsdialog_callback_t> for JsdialogCallback {
    fn into(self) -> _cef_jsdialog_callback_t {
        _cef_jsdialog_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
        }
    }
}
impl Default for JsdialogCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_jsdialog_handler_t] for more documentation.
#[derive(Clone)]
pub struct JsdialogHandler {
    pub base: BaseRefCounted,
    pub on_jsdialog: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_jsdialog_handler_t,
            browser: *mut _cef_browser_t,
            origin_url: *const cef_string_t,
            dialog_type: cef_jsdialog_type_t,
            message_text: *const cef_string_t,
            default_prompt_text: *const cef_string_t,
            callback: *mut _cef_jsdialog_callback_t,
            suppress_message: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_before_unload_dialog: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_jsdialog_handler_t,
            browser: *mut _cef_browser_t,
            message_text: *const cef_string_t,
            is_reload: ::std::os::raw::c_int,
            callback: *mut _cef_jsdialog_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_reset_dialog_state: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_jsdialog_handler_t,
            browser: *mut _cef_browser_t,
        ),
    >,
    pub on_dialog_closed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_jsdialog_handler_t,
            browser: *mut _cef_browser_t,
        ),
    >,
}
impl From<_cef_jsdialog_handler_t> for JsdialogHandler {
    fn from(value: _cef_jsdialog_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_jsdialog: value.on_jsdialog.into(),
            on_before_unload_dialog: value.on_before_unload_dialog.into(),
            on_reset_dialog_state: value.on_reset_dialog_state.into(),
            on_dialog_closed: value.on_dialog_closed.into(),
        }
    }
}
impl Into<_cef_jsdialog_handler_t> for JsdialogHandler {
    fn into(self) -> _cef_jsdialog_handler_t {
        _cef_jsdialog_handler_t {
            base: self.base.into(),
            on_jsdialog: self.on_jsdialog.into(),
            on_before_unload_dialog: self.on_before_unload_dialog.into(),
            on_reset_dialog_state: self.on_reset_dialog_state.into(),
            on_dialog_closed: self.on_dialog_closed.into(),
        }
    }
}
impl Default for JsdialogHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_keyboard_handler_t] for more documentation.
#[derive(Clone)]
pub struct KeyboardHandler {
    pub base: BaseRefCounted,
    pub on_pre_key_event: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_keyboard_handler_t,
            browser: *mut _cef_browser_t,
            event: *const cef_key_event_t,
            os_event: cef_event_handle_t,
            is_keyboard_shortcut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_key_event: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_keyboard_handler_t,
            browser: *mut _cef_browser_t,
            event: *const cef_key_event_t,
            os_event: cef_event_handle_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_keyboard_handler_t> for KeyboardHandler {
    fn from(value: _cef_keyboard_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_pre_key_event: value.on_pre_key_event.into(),
            on_key_event: value.on_key_event.into(),
        }
    }
}
impl Into<_cef_keyboard_handler_t> for KeyboardHandler {
    fn into(self) -> _cef_keyboard_handler_t {
        _cef_keyboard_handler_t {
            base: self.base.into(),
            on_pre_key_event: self.on_pre_key_event.into(),
            on_key_event: self.on_key_event.into(),
        }
    }
}
impl Default for KeyboardHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_life_span_handler_t] for more documentation.
#[derive(Clone)]
pub struct LifeSpanHandler {
    pub base: BaseRefCounted,
    pub on_before_popup: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            popup_id: ::std::os::raw::c_int,
            target_url: *const cef_string_t,
            target_frame_name: *const cef_string_t,
            target_disposition: cef_window_open_disposition_t,
            user_gesture: ::std::os::raw::c_int,
            popupFeatures: *const cef_popup_features_t,
            windowInfo: *mut _cef_window_info_t,
            client: *mut *mut _cef_client_t,
            settings: *mut _cef_browser_settings_t,
            extra_info: *mut *mut _cef_dictionary_value_t,
            no_javascript_access: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_before_popup_aborted: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
            popup_id: ::std::os::raw::c_int,
        ),
    >,
    pub on_before_dev_tools_popup: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
            windowInfo: *mut _cef_window_info_t,
            client: *mut *mut _cef_client_t,
            settings: *mut _cef_browser_settings_t,
            extra_info: *mut *mut _cef_dictionary_value_t,
            use_default_window: *mut ::std::os::raw::c_int,
        ),
    >,
    pub on_after_created: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
        ),
    >,
    pub do_close: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_before_close: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_life_span_handler_t,
            browser: *mut _cef_browser_t,
        ),
    >,
}
impl From<_cef_life_span_handler_t> for LifeSpanHandler {
    fn from(value: _cef_life_span_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_before_popup: value.on_before_popup.into(),
            on_before_popup_aborted: value.on_before_popup_aborted.into(),
            on_before_dev_tools_popup: value.on_before_dev_tools_popup.into(),
            on_after_created: value.on_after_created.into(),
            do_close: value.do_close.into(),
            on_before_close: value.on_before_close.into(),
        }
    }
}
impl Into<_cef_life_span_handler_t> for LifeSpanHandler {
    fn into(self) -> _cef_life_span_handler_t {
        _cef_life_span_handler_t {
            base: self.base.into(),
            on_before_popup: self.on_before_popup.into(),
            on_before_popup_aborted: self.on_before_popup_aborted.into(),
            on_before_dev_tools_popup: self.on_before_dev_tools_popup.into(),
            on_after_created: self.on_after_created.into(),
            do_close: self.do_close.into(),
            on_before_close: self.on_before_close.into(),
        }
    }
}
impl Default for LifeSpanHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_load_handler_t] for more documentation.
#[derive(Clone)]
pub struct LoadHandler {
    pub base: BaseRefCounted,
    pub on_loading_state_change: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_load_handler_t,
            browser: *mut _cef_browser_t,
            isLoading: ::std::os::raw::c_int,
            canGoBack: ::std::os::raw::c_int,
            canGoForward: ::std::os::raw::c_int,
        ),
    >,
    pub on_load_start: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_load_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            transition_type: cef_transition_type_t,
        ),
    >,
    pub on_load_end: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_load_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            httpStatusCode: ::std::os::raw::c_int,
        ),
    >,
    pub on_load_error: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_load_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            errorCode: cef_errorcode_t,
            errorText: *const cef_string_t,
            failedUrl: *const cef_string_t,
        ),
    >,
}
impl From<_cef_load_handler_t> for LoadHandler {
    fn from(value: _cef_load_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_loading_state_change: value.on_loading_state_change.into(),
            on_load_start: value.on_load_start.into(),
            on_load_end: value.on_load_end.into(),
            on_load_error: value.on_load_error.into(),
        }
    }
}
impl Into<_cef_load_handler_t> for LoadHandler {
    fn into(self) -> _cef_load_handler_t {
        _cef_load_handler_t {
            base: self.base.into(),
            on_loading_state_change: self.on_loading_state_change.into(),
            on_load_start: self.on_load_start.into(),
            on_load_end: self.on_load_end.into(),
            on_load_error: self.on_load_error.into(),
        }
    }
}
impl Default for LoadHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_media_access_callback_t] for more documentation.
#[derive(Clone)]
pub struct MediaAccessCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_media_access_callback_t,
            allowed_permissions: u32,
        ),
    >,
    pub cancel:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_media_access_callback_t)>,
}
impl From<_cef_media_access_callback_t> for MediaAccessCallback {
    fn from(value: _cef_media_access_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
            cancel: value.cancel.into(),
        }
    }
}
impl Into<_cef_media_access_callback_t> for MediaAccessCallback {
    fn into(self) -> _cef_media_access_callback_t {
        _cef_media_access_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
            cancel: self.cancel.into(),
        }
    }
}
impl Default for MediaAccessCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_permission_prompt_callback_t] for more documentation.
#[derive(Clone)]
pub struct PermissionPromptCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_permission_prompt_callback_t,
            result: cef_permission_request_result_t,
        ),
    >,
}
impl From<_cef_permission_prompt_callback_t> for PermissionPromptCallback {
    fn from(value: _cef_permission_prompt_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
        }
    }
}
impl Into<_cef_permission_prompt_callback_t> for PermissionPromptCallback {
    fn into(self) -> _cef_permission_prompt_callback_t {
        _cef_permission_prompt_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
        }
    }
}
impl Default for PermissionPromptCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_permission_handler_t] for more documentation.
#[derive(Clone)]
pub struct PermissionHandler {
    pub base: BaseRefCounted,
    pub on_request_media_access_permission: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_permission_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            requesting_origin: *const cef_string_t,
            requested_permissions: u32,
            callback: *mut _cef_media_access_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_show_permission_prompt: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_permission_handler_t,
            browser: *mut _cef_browser_t,
            prompt_id: u64,
            requesting_origin: *const cef_string_t,
            requested_permissions: u32,
            callback: *mut _cef_permission_prompt_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_dismiss_permission_prompt: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_permission_handler_t,
            browser: *mut _cef_browser_t,
            prompt_id: u64,
            result: cef_permission_request_result_t,
        ),
    >,
}
impl From<_cef_permission_handler_t> for PermissionHandler {
    fn from(value: _cef_permission_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_request_media_access_permission: value.on_request_media_access_permission.into(),
            on_show_permission_prompt: value.on_show_permission_prompt.into(),
            on_dismiss_permission_prompt: value.on_dismiss_permission_prompt.into(),
        }
    }
}
impl Into<_cef_permission_handler_t> for PermissionHandler {
    fn into(self) -> _cef_permission_handler_t {
        _cef_permission_handler_t {
            base: self.base.into(),
            on_request_media_access_permission: self.on_request_media_access_permission.into(),
            on_show_permission_prompt: self.on_show_permission_prompt.into(),
            on_dismiss_permission_prompt: self.on_dismiss_permission_prompt.into(),
        }
    }
}
impl Default for PermissionHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_print_settings_t] for more documentation.
#[derive(Clone)]
pub struct PrintSettings {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> ::std::os::raw::c_int,
    >,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> ::std::os::raw::c_int,
    >,
    pub set_orientation: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_settings_t,
            landscape: ::std::os::raw::c_int,
        ),
    >,
    pub is_landscape: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> ::std::os::raw::c_int,
    >,
    pub set_printer_printable_area: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_settings_t,
            physical_size_device_units: *const cef_size_t,
            printable_area_device_units: *const cef_rect_t,
            landscape_needs_flip: ::std::os::raw::c_int,
        ),
    >,
    pub set_device_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t, name: *const cef_string_t),
    >,
    pub get_device_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> cef_string_userfree_t,
    >,
    pub set_dpi: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t, dpi: ::std::os::raw::c_int),
    >,
    pub get_dpi: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> ::std::os::raw::c_int,
    >,
    pub set_page_ranges: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_settings_t,
            rangesCount: usize,
            ranges: *const cef_range_t,
        ),
    >,
    pub get_page_ranges_count: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> usize,
    >,
    pub get_page_ranges: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_settings_t,
            rangesCount: *mut usize,
            ranges: *mut cef_range_t,
        ),
    >,
    pub set_selection_only: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_settings_t,
            selection_only: ::std::os::raw::c_int,
        ),
    >,
    pub is_selection_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> ::std::os::raw::c_int,
    >,
    pub set_collate: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_settings_t,
            collate: ::std::os::raw::c_int,
        ),
    >,
    pub will_collate: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> ::std::os::raw::c_int,
    >,
    pub set_color_model: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t, model: cef_color_model_t),
    >,
    pub get_color_model: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> cef_color_model_t,
    >,
    pub set_copies: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_settings_t,
            copies: ::std::os::raw::c_int,
        ),
    >,
    pub get_copies: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> ::std::os::raw::c_int,
    >,
    pub set_duplex_mode: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t, mode: cef_duplex_mode_t),
    >,
    pub get_duplex_mode: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_settings_t) -> cef_duplex_mode_t,
    >,
}
impl From<_cef_print_settings_t> for PrintSettings {
    fn from(value: _cef_print_settings_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            is_read_only: value.is_read_only.into(),
            set_orientation: value.set_orientation.into(),
            is_landscape: value.is_landscape.into(),
            set_printer_printable_area: value.set_printer_printable_area.into(),
            set_device_name: value.set_device_name.into(),
            get_device_name: value.get_device_name.into(),
            set_dpi: value.set_dpi.into(),
            get_dpi: value.get_dpi.into(),
            set_page_ranges: value.set_page_ranges.into(),
            get_page_ranges_count: value.get_page_ranges_count.into(),
            get_page_ranges: value.get_page_ranges.into(),
            set_selection_only: value.set_selection_only.into(),
            is_selection_only: value.is_selection_only.into(),
            set_collate: value.set_collate.into(),
            will_collate: value.will_collate.into(),
            set_color_model: value.set_color_model.into(),
            get_color_model: value.get_color_model.into(),
            set_copies: value.set_copies.into(),
            get_copies: value.get_copies.into(),
            set_duplex_mode: value.set_duplex_mode.into(),
            get_duplex_mode: value.get_duplex_mode.into(),
        }
    }
}
impl Into<_cef_print_settings_t> for PrintSettings {
    fn into(self) -> _cef_print_settings_t {
        _cef_print_settings_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            is_read_only: self.is_read_only.into(),
            set_orientation: self.set_orientation.into(),
            is_landscape: self.is_landscape.into(),
            set_printer_printable_area: self.set_printer_printable_area.into(),
            set_device_name: self.set_device_name.into(),
            get_device_name: self.get_device_name.into(),
            set_dpi: self.set_dpi.into(),
            get_dpi: self.get_dpi.into(),
            set_page_ranges: self.set_page_ranges.into(),
            get_page_ranges_count: self.get_page_ranges_count.into(),
            get_page_ranges: self.get_page_ranges.into(),
            set_selection_only: self.set_selection_only.into(),
            is_selection_only: self.is_selection_only.into(),
            set_collate: self.set_collate.into(),
            will_collate: self.will_collate.into(),
            set_color_model: self.set_color_model.into(),
            get_color_model: self.get_color_model.into(),
            set_copies: self.set_copies.into(),
            get_copies: self.get_copies.into(),
            set_duplex_mode: self.set_duplex_mode.into(),
            get_duplex_mode: self.get_duplex_mode.into(),
        }
    }
}
impl Default for PrintSettings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_print_dialog_callback_t] for more documentation.
#[derive(Clone)]
pub struct PrintDialogCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_dialog_callback_t,
            settings: *mut _cef_print_settings_t,
        ),
    >,
    pub cancel:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_print_dialog_callback_t)>,
}
impl From<_cef_print_dialog_callback_t> for PrintDialogCallback {
    fn from(value: _cef_print_dialog_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
            cancel: value.cancel.into(),
        }
    }
}
impl Into<_cef_print_dialog_callback_t> for PrintDialogCallback {
    fn into(self) -> _cef_print_dialog_callback_t {
        _cef_print_dialog_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
            cancel: self.cancel.into(),
        }
    }
}
impl Default for PrintDialogCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_print_job_callback_t] for more documentation.
#[derive(Clone)]
pub struct PrintJobCallback {
    pub base: BaseRefCounted,
    pub cont:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_print_job_callback_t)>,
}
impl From<_cef_print_job_callback_t> for PrintJobCallback {
    fn from(value: _cef_print_job_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
        }
    }
}
impl Into<_cef_print_job_callback_t> for PrintJobCallback {
    fn into(self) -> _cef_print_job_callback_t {
        _cef_print_job_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
        }
    }
}
impl Default for PrintJobCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_print_handler_t] for more documentation.
#[derive(Clone)]
pub struct PrintHandler {
    pub base: BaseRefCounted,
    pub on_print_start: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_handler_t, browser: *mut _cef_browser_t),
    >,
    pub on_print_settings: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_handler_t,
            browser: *mut _cef_browser_t,
            settings: *mut _cef_print_settings_t,
            get_defaults: ::std::os::raw::c_int,
        ),
    >,
    pub on_print_dialog: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_handler_t,
            browser: *mut _cef_browser_t,
            has_selection: ::std::os::raw::c_int,
            callback: *mut _cef_print_dialog_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_print_job: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_handler_t,
            browser: *mut _cef_browser_t,
            document_name: *const cef_string_t,
            pdf_file_path: *const cef_string_t,
            callback: *mut _cef_print_job_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_print_reset: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_print_handler_t, browser: *mut _cef_browser_t),
    >,
    pub get_pdf_paper_size: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_print_handler_t,
            browser: *mut _cef_browser_t,
            device_units_per_inch: ::std::os::raw::c_int,
        ) -> cef_size_t,
    >,
}
impl From<_cef_print_handler_t> for PrintHandler {
    fn from(value: _cef_print_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_print_start: value.on_print_start.into(),
            on_print_settings: value.on_print_settings.into(),
            on_print_dialog: value.on_print_dialog.into(),
            on_print_job: value.on_print_job.into(),
            on_print_reset: value.on_print_reset.into(),
            get_pdf_paper_size: value.get_pdf_paper_size.into(),
        }
    }
}
impl Into<_cef_print_handler_t> for PrintHandler {
    fn into(self) -> _cef_print_handler_t {
        _cef_print_handler_t {
            base: self.base.into(),
            on_print_start: self.on_print_start.into(),
            on_print_settings: self.on_print_settings.into(),
            on_print_dialog: self.on_print_dialog.into(),
            on_print_job: self.on_print_job.into(),
            on_print_reset: self.on_print_reset.into(),
            get_pdf_paper_size: self.get_pdf_paper_size.into(),
        }
    }
}
impl Default for PrintHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_accessibility_handler_t] for more documentation.
#[derive(Clone)]
pub struct AccessibilityHandler {
    pub base: BaseRefCounted,
    pub on_accessibility_tree_change: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_accessibility_handler_t,
            value: *mut _cef_value_t,
        ),
    >,
    pub on_accessibility_location_change: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_accessibility_handler_t,
            value: *mut _cef_value_t,
        ),
    >,
}
impl From<_cef_accessibility_handler_t> for AccessibilityHandler {
    fn from(value: _cef_accessibility_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_accessibility_tree_change: value.on_accessibility_tree_change.into(),
            on_accessibility_location_change: value.on_accessibility_location_change.into(),
        }
    }
}
impl Into<_cef_accessibility_handler_t> for AccessibilityHandler {
    fn into(self) -> _cef_accessibility_handler_t {
        _cef_accessibility_handler_t {
            base: self.base.into(),
            on_accessibility_tree_change: self.on_accessibility_tree_change.into(),
            on_accessibility_location_change: self.on_accessibility_location_change.into(),
        }
    }
}
impl Default for AccessibilityHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_render_handler_t] for more documentation.
#[derive(Clone)]
pub struct RenderHandler {
    pub base: BaseRefCounted,
    pub get_accessibility_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
        ) -> *mut _cef_accessibility_handler_t,
    >,
    pub get_root_screen_rect: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            rect: *mut cef_rect_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_view_rect: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            rect: *mut cef_rect_t,
        ),
    >,
    pub get_screen_point: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            viewX: ::std::os::raw::c_int,
            viewY: ::std::os::raw::c_int,
            screenX: *mut ::std::os::raw::c_int,
            screenY: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_screen_info: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            screen_info: *mut cef_screen_info_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_popup_show: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            show: ::std::os::raw::c_int,
        ),
    >,
    pub on_popup_size: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            rect: *const cef_rect_t,
        ),
    >,
    pub on_paint: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            type_: cef_paint_element_type_t,
            dirtyRectsCount: usize,
            dirtyRects: *const cef_rect_t,
            buffer: *const ::std::os::raw::c_void,
            width: ::std::os::raw::c_int,
            height: ::std::os::raw::c_int,
        ),
    >,
    pub on_accelerated_paint: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            type_: cef_paint_element_type_t,
            dirtyRectsCount: usize,
            dirtyRects: *const cef_rect_t,
            info: *const cef_accelerated_paint_info_t,
        ),
    >,
    pub get_touch_handle_size: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            orientation: cef_horizontal_alignment_t,
            size: *mut cef_size_t,
        ),
    >,
    pub on_touch_handle_state_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            state: *const cef_touch_handle_state_t,
        ),
    >,
    pub start_dragging: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            drag_data: *mut _cef_drag_data_t,
            allowed_ops: cef_drag_operations_mask_t,
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub update_drag_cursor: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            operation: cef_drag_operations_mask_t,
        ),
    >,
    pub on_scroll_offset_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            x: f64,
            y: f64,
        ),
    >,
    pub on_ime_composition_range_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            selected_range: *const cef_range_t,
            character_boundsCount: usize,
            character_bounds: *const cef_rect_t,
        ),
    >,
    pub on_text_selection_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            selected_text: *const cef_string_t,
            selected_range: *const cef_range_t,
        ),
    >,
    pub on_virtual_keyboard_requested: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_handler_t,
            browser: *mut _cef_browser_t,
            input_mode: cef_text_input_mode_t,
        ),
    >,
}
impl From<_cef_render_handler_t> for RenderHandler {
    fn from(value: _cef_render_handler_t) -> Self {
        Self {
            base: value.base.into(),
            get_accessibility_handler: value.get_accessibility_handler.into(),
            get_root_screen_rect: value.get_root_screen_rect.into(),
            get_view_rect: value.get_view_rect.into(),
            get_screen_point: value.get_screen_point.into(),
            get_screen_info: value.get_screen_info.into(),
            on_popup_show: value.on_popup_show.into(),
            on_popup_size: value.on_popup_size.into(),
            on_paint: value.on_paint.into(),
            on_accelerated_paint: value.on_accelerated_paint.into(),
            get_touch_handle_size: value.get_touch_handle_size.into(),
            on_touch_handle_state_changed: value.on_touch_handle_state_changed.into(),
            start_dragging: value.start_dragging.into(),
            update_drag_cursor: value.update_drag_cursor.into(),
            on_scroll_offset_changed: value.on_scroll_offset_changed.into(),
            on_ime_composition_range_changed: value.on_ime_composition_range_changed.into(),
            on_text_selection_changed: value.on_text_selection_changed.into(),
            on_virtual_keyboard_requested: value.on_virtual_keyboard_requested.into(),
        }
    }
}
impl Into<_cef_render_handler_t> for RenderHandler {
    fn into(self) -> _cef_render_handler_t {
        _cef_render_handler_t {
            base: self.base.into(),
            get_accessibility_handler: self.get_accessibility_handler.into(),
            get_root_screen_rect: self.get_root_screen_rect.into(),
            get_view_rect: self.get_view_rect.into(),
            get_screen_point: self.get_screen_point.into(),
            get_screen_info: self.get_screen_info.into(),
            on_popup_show: self.on_popup_show.into(),
            on_popup_size: self.on_popup_size.into(),
            on_paint: self.on_paint.into(),
            on_accelerated_paint: self.on_accelerated_paint.into(),
            get_touch_handle_size: self.get_touch_handle_size.into(),
            on_touch_handle_state_changed: self.on_touch_handle_state_changed.into(),
            start_dragging: self.start_dragging.into(),
            update_drag_cursor: self.update_drag_cursor.into(),
            on_scroll_offset_changed: self.on_scroll_offset_changed.into(),
            on_ime_composition_range_changed: self.on_ime_composition_range_changed.into(),
            on_text_selection_changed: self.on_text_selection_changed.into(),
            on_virtual_keyboard_requested: self.on_virtual_keyboard_requested.into(),
        }
    }
}
impl Default for RenderHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_auth_callback_t] for more documentation.
#[derive(Clone)]
pub struct AuthCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_auth_callback_t,
            username: *const cef_string_t,
            password: *const cef_string_t,
        ),
    >,
    pub cancel: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_auth_callback_t)>,
}
impl From<_cef_auth_callback_t> for AuthCallback {
    fn from(value: _cef_auth_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
            cancel: value.cancel.into(),
        }
    }
}
impl Into<_cef_auth_callback_t> for AuthCallback {
    fn into(self) -> _cef_auth_callback_t {
        _cef_auth_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
            cancel: self.cancel.into(),
        }
    }
}
impl Default for AuthCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_response_t] for more documentation.
#[derive(Clone)]
pub struct Response {
    pub base: BaseRefCounted,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t) -> ::std::os::raw::c_int,
    >,
    pub get_error: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t) -> cef_errorcode_t,
    >,
    pub set_error: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t, error: cef_errorcode_t),
    >,
    pub get_status: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t) -> ::std::os::raw::c_int,
    >,
    pub set_status: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t, status: ::std::os::raw::c_int),
    >,
    pub get_status_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t) -> cef_string_userfree_t,
    >,
    pub set_status_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t, statusText: *const cef_string_t),
    >,
    pub get_mime_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t) -> cef_string_userfree_t,
    >,
    pub set_mime_type: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t, mimeType: *const cef_string_t),
    >,
    pub get_charset: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t) -> cef_string_userfree_t,
    >,
    pub set_charset: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t, charset: *const cef_string_t),
    >,
    pub get_header_by_name: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_response_t,
            name: *const cef_string_t,
        ) -> cef_string_userfree_t,
    >,
    pub set_header_by_name: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_response_t,
            name: *const cef_string_t,
            value: *const cef_string_t,
            overwrite: ::std::os::raw::c_int,
        ),
    >,
    pub get_header_map: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t, headerMap: cef_string_multimap_t),
    >,
    pub set_header_map: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t, headerMap: cef_string_multimap_t),
    >,
    pub get_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t) -> cef_string_userfree_t,
    >,
    pub set_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_t, url: *const cef_string_t),
    >,
}
impl From<_cef_response_t> for Response {
    fn from(value: _cef_response_t) -> Self {
        Self {
            base: value.base.into(),
            is_read_only: value.is_read_only.into(),
            get_error: value.get_error.into(),
            set_error: value.set_error.into(),
            get_status: value.get_status.into(),
            set_status: value.set_status.into(),
            get_status_text: value.get_status_text.into(),
            set_status_text: value.set_status_text.into(),
            get_mime_type: value.get_mime_type.into(),
            set_mime_type: value.set_mime_type.into(),
            get_charset: value.get_charset.into(),
            set_charset: value.set_charset.into(),
            get_header_by_name: value.get_header_by_name.into(),
            set_header_by_name: value.set_header_by_name.into(),
            get_header_map: value.get_header_map.into(),
            set_header_map: value.set_header_map.into(),
            get_url: value.get_url.into(),
            set_url: value.set_url.into(),
        }
    }
}
impl Into<_cef_response_t> for Response {
    fn into(self) -> _cef_response_t {
        _cef_response_t {
            base: self.base.into(),
            is_read_only: self.is_read_only.into(),
            get_error: self.get_error.into(),
            set_error: self.set_error.into(),
            get_status: self.get_status.into(),
            set_status: self.set_status.into(),
            get_status_text: self.get_status_text.into(),
            set_status_text: self.set_status_text.into(),
            get_mime_type: self.get_mime_type.into(),
            set_mime_type: self.set_mime_type.into(),
            get_charset: self.get_charset.into(),
            set_charset: self.set_charset.into(),
            get_header_by_name: self.get_header_by_name.into(),
            set_header_by_name: self.set_header_by_name.into(),
            get_header_map: self.get_header_map.into(),
            set_header_map: self.set_header_map.into(),
            get_url: self.get_url.into(),
            set_url: self.set_url.into(),
        }
    }
}
impl Default for Response {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_resource_skip_callback_t] for more documentation.
#[derive(Clone)]
pub struct ResourceSkipCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_resource_skip_callback_t, bytes_skipped: i64),
    >,
}
impl From<_cef_resource_skip_callback_t> for ResourceSkipCallback {
    fn from(value: _cef_resource_skip_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
        }
    }
}
impl Into<_cef_resource_skip_callback_t> for ResourceSkipCallback {
    fn into(self) -> _cef_resource_skip_callback_t {
        _cef_resource_skip_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
        }
    }
}
impl Default for ResourceSkipCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_resource_read_callback_t] for more documentation.
#[derive(Clone)]
pub struct ResourceReadCallback {
    pub base: BaseRefCounted,
    pub cont: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_read_callback_t,
            bytes_read: ::std::os::raw::c_int,
        ),
    >,
}
impl From<_cef_resource_read_callback_t> for ResourceReadCallback {
    fn from(value: _cef_resource_read_callback_t) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont.into(),
        }
    }
}
impl Into<_cef_resource_read_callback_t> for ResourceReadCallback {
    fn into(self) -> _cef_resource_read_callback_t {
        _cef_resource_read_callback_t {
            base: self.base.into(),
            cont: self.cont.into(),
        }
    }
}
impl Default for ResourceReadCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_resource_handler_t] for more documentation.
#[derive(Clone)]
pub struct ResourceHandler {
    pub base: BaseRefCounted,
    pub open: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_handler_t,
            request: *mut _cef_request_t,
            handle_request: *mut ::std::os::raw::c_int,
            callback: *mut _cef_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub process_request: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_handler_t,
            request: *mut _cef_request_t,
            callback: *mut _cef_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_response_headers: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_handler_t,
            response: *mut _cef_response_t,
            response_length: *mut i64,
            redirectUrl: *mut cef_string_t,
        ),
    >,
    pub skip: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_handler_t,
            bytes_to_skip: i64,
            bytes_skipped: *mut i64,
            callback: *mut _cef_resource_skip_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub read: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_handler_t,
            data_out: *mut ::std::os::raw::c_void,
            bytes_to_read: ::std::os::raw::c_int,
            bytes_read: *mut ::std::os::raw::c_int,
            callback: *mut _cef_resource_read_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub read_response: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_handler_t,
            data_out: *mut ::std::os::raw::c_void,
            bytes_to_read: ::std::os::raw::c_int,
            bytes_read: *mut ::std::os::raw::c_int,
            callback: *mut _cef_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub cancel:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_resource_handler_t)>,
}
impl From<_cef_resource_handler_t> for ResourceHandler {
    fn from(value: _cef_resource_handler_t) -> Self {
        Self {
            base: value.base.into(),
            open: value.open.into(),
            process_request: value.process_request.into(),
            get_response_headers: value.get_response_headers.into(),
            skip: value.skip.into(),
            read: value.read.into(),
            read_response: value.read_response.into(),
            cancel: value.cancel.into(),
        }
    }
}
impl Into<_cef_resource_handler_t> for ResourceHandler {
    fn into(self) -> _cef_resource_handler_t {
        _cef_resource_handler_t {
            base: self.base.into(),
            open: self.open.into(),
            process_request: self.process_request.into(),
            get_response_headers: self.get_response_headers.into(),
            skip: self.skip.into(),
            read: self.read.into(),
            read_response: self.read_response.into(),
            cancel: self.cancel.into(),
        }
    }
}
impl Default for ResourceHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_response_filter_t] for more documentation.
#[derive(Clone)]
pub struct ResponseFilter {
    pub base: BaseRefCounted,
    pub init_filter: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_response_filter_t) -> ::std::os::raw::c_int,
    >,
    pub filter: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_response_filter_t,
            data_in: *mut ::std::os::raw::c_void,
            data_in_size: usize,
            data_in_read: *mut usize,
            data_out: *mut ::std::os::raw::c_void,
            data_out_size: usize,
            data_out_written: *mut usize,
        ) -> cef_response_filter_status_t,
    >,
}
impl From<_cef_response_filter_t> for ResponseFilter {
    fn from(value: _cef_response_filter_t) -> Self {
        Self {
            base: value.base.into(),
            init_filter: value.init_filter.into(),
            filter: value.filter.into(),
        }
    }
}
impl Into<_cef_response_filter_t> for ResponseFilter {
    fn into(self) -> _cef_response_filter_t {
        _cef_response_filter_t {
            base: self.base.into(),
            init_filter: self.init_filter.into(),
            filter: self.filter.into(),
        }
    }
}
impl Default for ResponseFilter {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_resource_request_handler_t] for more documentation.
#[derive(Clone)]
pub struct ResourceRequestHandler {
    pub base: BaseRefCounted,
    pub get_cookie_access_filter: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
        ) -> *mut _cef_cookie_access_filter_t,
    >,
    pub on_before_resource_load: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            callback: *mut _cef_callback_t,
        ) -> cef_return_value_t,
    >,
    pub get_resource_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
        ) -> *mut _cef_resource_handler_t,
    >,
    pub on_resource_redirect: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            response: *mut _cef_response_t,
            new_url: *mut cef_string_t,
        ),
    >,
    pub on_resource_response: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            response: *mut _cef_response_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_resource_response_filter: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            response: *mut _cef_response_t,
        ) -> *mut _cef_response_filter_t,
    >,
    pub on_resource_load_complete: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            response: *mut _cef_response_t,
            status: cef_urlrequest_status_t,
            received_content_length: i64,
        ),
    >,
    pub on_protocol_execution: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            allow_os_execution: *mut ::std::os::raw::c_int,
        ),
    >,
}
impl From<_cef_resource_request_handler_t> for ResourceRequestHandler {
    fn from(value: _cef_resource_request_handler_t) -> Self {
        Self {
            base: value.base.into(),
            get_cookie_access_filter: value.get_cookie_access_filter.into(),
            on_before_resource_load: value.on_before_resource_load.into(),
            get_resource_handler: value.get_resource_handler.into(),
            on_resource_redirect: value.on_resource_redirect.into(),
            on_resource_response: value.on_resource_response.into(),
            get_resource_response_filter: value.get_resource_response_filter.into(),
            on_resource_load_complete: value.on_resource_load_complete.into(),
            on_protocol_execution: value.on_protocol_execution.into(),
        }
    }
}
impl Into<_cef_resource_request_handler_t> for ResourceRequestHandler {
    fn into(self) -> _cef_resource_request_handler_t {
        _cef_resource_request_handler_t {
            base: self.base.into(),
            get_cookie_access_filter: self.get_cookie_access_filter.into(),
            on_before_resource_load: self.on_before_resource_load.into(),
            get_resource_handler: self.get_resource_handler.into(),
            on_resource_redirect: self.on_resource_redirect.into(),
            on_resource_response: self.on_resource_response.into(),
            get_resource_response_filter: self.get_resource_response_filter.into(),
            on_resource_load_complete: self.on_resource_load_complete.into(),
            on_protocol_execution: self.on_protocol_execution.into(),
        }
    }
}
impl Default for ResourceRequestHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_cookie_access_filter_t] for more documentation.
#[derive(Clone)]
pub struct CookieAccessFilter {
    pub base: BaseRefCounted,
    pub can_send_cookie: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_cookie_access_filter_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            cookie: *const _cef_cookie_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub can_save_cookie: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_cookie_access_filter_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            response: *mut _cef_response_t,
            cookie: *const _cef_cookie_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_cookie_access_filter_t> for CookieAccessFilter {
    fn from(value: _cef_cookie_access_filter_t) -> Self {
        Self {
            base: value.base.into(),
            can_send_cookie: value.can_send_cookie.into(),
            can_save_cookie: value.can_save_cookie.into(),
        }
    }
}
impl Into<_cef_cookie_access_filter_t> for CookieAccessFilter {
    fn into(self) -> _cef_cookie_access_filter_t {
        _cef_cookie_access_filter_t {
            base: self.base.into(),
            can_send_cookie: self.can_send_cookie.into(),
            can_save_cookie: self.can_save_cookie.into(),
        }
    }
}
impl Default for CookieAccessFilter {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_sslinfo_t] for more documentation.
#[derive(Clone)]
pub struct Sslinfo {
    pub base: BaseRefCounted,
    pub get_cert_status: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_sslinfo_t) -> cef_cert_status_t,
    >,
    pub get_x_509_certificate: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_sslinfo_t) -> *mut _cef_x509_certificate_t,
    >,
}
impl From<_cef_sslinfo_t> for Sslinfo {
    fn from(value: _cef_sslinfo_t) -> Self {
        Self {
            base: value.base.into(),
            get_cert_status: value.get_cert_status.into(),
            get_x_509_certificate: value.get_x509_certificate.into(),
        }
    }
}
impl Into<_cef_sslinfo_t> for Sslinfo {
    fn into(self) -> _cef_sslinfo_t {
        _cef_sslinfo_t {
            base: self.base.into(),
            get_cert_status: self.get_cert_status.into(),
            get_x509_certificate: self.get_x_509_certificate.into(),
        }
    }
}
impl Default for Sslinfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_unresponsive_process_callback_t] for more documentation.
#[derive(Clone)]
pub struct UnresponsiveProcessCallback {
    pub base: BaseRefCounted,
    pub wait: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_unresponsive_process_callback_t),
    >,
    pub terminate: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_unresponsive_process_callback_t),
    >,
}
impl From<_cef_unresponsive_process_callback_t> for UnresponsiveProcessCallback {
    fn from(value: _cef_unresponsive_process_callback_t) -> Self {
        Self {
            base: value.base.into(),
            wait: value.wait.into(),
            terminate: value.terminate.into(),
        }
    }
}
impl Into<_cef_unresponsive_process_callback_t> for UnresponsiveProcessCallback {
    fn into(self) -> _cef_unresponsive_process_callback_t {
        _cef_unresponsive_process_callback_t {
            base: self.base.into(),
            wait: self.wait.into(),
            terminate: self.terminate.into(),
        }
    }
}
impl Default for UnresponsiveProcessCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_select_client_certificate_callback_t] for more documentation.
#[derive(Clone)]
pub struct SelectClientCertificateCallback {
    pub base: BaseRefCounted,
    pub select: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_select_client_certificate_callback_t,
            cert: *mut _cef_x509_certificate_t,
        ),
    >,
}
impl From<_cef_select_client_certificate_callback_t> for SelectClientCertificateCallback {
    fn from(value: _cef_select_client_certificate_callback_t) -> Self {
        Self {
            base: value.base.into(),
            select: value.select.into(),
        }
    }
}
impl Into<_cef_select_client_certificate_callback_t> for SelectClientCertificateCallback {
    fn into(self) -> _cef_select_client_certificate_callback_t {
        _cef_select_client_certificate_callback_t {
            base: self.base.into(),
            select: self.select.into(),
        }
    }
}
impl Default for SelectClientCertificateCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_request_handler_t] for more documentation.
#[derive(Clone)]
pub struct RequestHandler {
    pub base: BaseRefCounted,
    pub on_before_browse: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            user_gesture: ::std::os::raw::c_int,
            is_redirect: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_open_urlfrom_tab: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            target_url: *const cef_string_t,
            target_disposition: cef_window_open_disposition_t,
            user_gesture: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_resource_request_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            is_navigation: ::std::os::raw::c_int,
            is_download: ::std::os::raw::c_int,
            request_initiator: *const cef_string_t,
            disable_default_handling: *mut ::std::os::raw::c_int,
        ) -> *mut _cef_resource_request_handler_t,
    >,
    pub get_auth_credentials: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            origin_url: *const cef_string_t,
            isProxy: ::std::os::raw::c_int,
            host: *const cef_string_t,
            port: ::std::os::raw::c_int,
            realm: *const cef_string_t,
            scheme: *const cef_string_t,
            callback: *mut _cef_auth_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_certificate_error: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            cert_error: cef_errorcode_t,
            request_url: *const cef_string_t,
            ssl_info: *mut _cef_sslinfo_t,
            callback: *mut _cef_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_select_client_certificate: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            isProxy: ::std::os::raw::c_int,
            host: *const cef_string_t,
            port: ::std::os::raw::c_int,
            certificatesCount: usize,
            certificates: *const *mut _cef_x509_certificate_t,
            callback: *mut _cef_select_client_certificate_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_render_view_ready: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
        ),
    >,
    pub on_render_process_unresponsive: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            callback: *mut _cef_unresponsive_process_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_render_process_responsive: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
        ),
    >,
    pub on_render_process_terminated: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
            status: cef_termination_status_t,
            error_code: ::std::os::raw::c_int,
            error_string: *const cef_string_t,
        ),
    >,
    pub on_document_available_in_main_frame: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_handler_t,
            browser: *mut _cef_browser_t,
        ),
    >,
}
impl From<_cef_request_handler_t> for RequestHandler {
    fn from(value: _cef_request_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_before_browse: value.on_before_browse.into(),
            on_open_urlfrom_tab: value.on_open_urlfrom_tab.into(),
            get_resource_request_handler: value.get_resource_request_handler.into(),
            get_auth_credentials: value.get_auth_credentials.into(),
            on_certificate_error: value.on_certificate_error.into(),
            on_select_client_certificate: value.on_select_client_certificate.into(),
            on_render_view_ready: value.on_render_view_ready.into(),
            on_render_process_unresponsive: value.on_render_process_unresponsive.into(),
            on_render_process_responsive: value.on_render_process_responsive.into(),
            on_render_process_terminated: value.on_render_process_terminated.into(),
            on_document_available_in_main_frame: value.on_document_available_in_main_frame.into(),
        }
    }
}
impl Into<_cef_request_handler_t> for RequestHandler {
    fn into(self) -> _cef_request_handler_t {
        _cef_request_handler_t {
            base: self.base.into(),
            on_before_browse: self.on_before_browse.into(),
            on_open_urlfrom_tab: self.on_open_urlfrom_tab.into(),
            get_resource_request_handler: self.get_resource_request_handler.into(),
            get_auth_credentials: self.get_auth_credentials.into(),
            on_certificate_error: self.on_certificate_error.into(),
            on_select_client_certificate: self.on_select_client_certificate.into(),
            on_render_view_ready: self.on_render_view_ready.into(),
            on_render_process_unresponsive: self.on_render_process_unresponsive.into(),
            on_render_process_responsive: self.on_render_process_responsive.into(),
            on_render_process_terminated: self.on_render_process_terminated.into(),
            on_document_available_in_main_frame: self.on_document_available_in_main_frame.into(),
        }
    }
}
impl Default for RequestHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_client_t] for more documentation.
#[derive(Clone)]
pub struct Client {
    pub base: BaseRefCounted,
    pub get_audio_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_audio_handler_t,
    >,
    pub get_command_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_command_handler_t,
    >,
    pub get_context_menu_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_context_menu_handler_t,
    >,
    pub get_dialog_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_dialog_handler_t,
    >,
    pub get_display_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_display_handler_t,
    >,
    pub get_download_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_download_handler_t,
    >,
    pub get_drag_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_drag_handler_t,
    >,
    pub get_find_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_find_handler_t,
    >,
    pub get_focus_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_focus_handler_t,
    >,
    pub get_frame_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_frame_handler_t,
    >,
    pub get_permission_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_permission_handler_t,
    >,
    pub get_jsdialog_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_jsdialog_handler_t,
    >,
    pub get_keyboard_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_keyboard_handler_t,
    >,
    pub get_life_span_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_life_span_handler_t,
    >,
    pub get_load_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_load_handler_t,
    >,
    pub get_print_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_print_handler_t,
    >,
    pub get_render_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_render_handler_t,
    >,
    pub get_request_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_client_t) -> *mut _cef_request_handler_t,
    >,
    pub on_process_message_received: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_client_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            source_process: cef_process_id_t,
            message: *mut _cef_process_message_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_client_t> for Client {
    fn from(value: _cef_client_t) -> Self {
        Self {
            base: value.base.into(),
            get_audio_handler: value.get_audio_handler.into(),
            get_command_handler: value.get_command_handler.into(),
            get_context_menu_handler: value.get_context_menu_handler.into(),
            get_dialog_handler: value.get_dialog_handler.into(),
            get_display_handler: value.get_display_handler.into(),
            get_download_handler: value.get_download_handler.into(),
            get_drag_handler: value.get_drag_handler.into(),
            get_find_handler: value.get_find_handler.into(),
            get_focus_handler: value.get_focus_handler.into(),
            get_frame_handler: value.get_frame_handler.into(),
            get_permission_handler: value.get_permission_handler.into(),
            get_jsdialog_handler: value.get_jsdialog_handler.into(),
            get_keyboard_handler: value.get_keyboard_handler.into(),
            get_life_span_handler: value.get_life_span_handler.into(),
            get_load_handler: value.get_load_handler.into(),
            get_print_handler: value.get_print_handler.into(),
            get_render_handler: value.get_render_handler.into(),
            get_request_handler: value.get_request_handler.into(),
            on_process_message_received: value.on_process_message_received.into(),
        }
    }
}
impl Into<_cef_client_t> for Client {
    fn into(self) -> _cef_client_t {
        _cef_client_t {
            base: self.base.into(),
            get_audio_handler: self.get_audio_handler.into(),
            get_command_handler: self.get_command_handler.into(),
            get_context_menu_handler: self.get_context_menu_handler.into(),
            get_dialog_handler: self.get_dialog_handler.into(),
            get_display_handler: self.get_display_handler.into(),
            get_download_handler: self.get_download_handler.into(),
            get_drag_handler: self.get_drag_handler.into(),
            get_find_handler: self.get_find_handler.into(),
            get_focus_handler: self.get_focus_handler.into(),
            get_frame_handler: self.get_frame_handler.into(),
            get_permission_handler: self.get_permission_handler.into(),
            get_jsdialog_handler: self.get_jsdialog_handler.into(),
            get_keyboard_handler: self.get_keyboard_handler.into(),
            get_life_span_handler: self.get_life_span_handler.into(),
            get_load_handler: self.get_load_handler.into(),
            get_print_handler: self.get_print_handler.into(),
            get_render_handler: self.get_render_handler.into(),
            get_request_handler: self.get_request_handler.into(),
            on_process_message_received: self.on_process_message_received.into(),
        }
    }
}
impl Default for Client {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_command_line_t] for more documentation.
#[derive(Clone)]
pub struct CommandLine {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t) -> ::std::os::raw::c_int,
    >,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t) -> ::std::os::raw::c_int,
    >,
    pub copy: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t) -> *mut _cef_command_line_t,
    >,
    pub init_from_argv: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_command_line_t,
            argc: ::std::os::raw::c_int,
            argv: *const *const ::std::os::raw::c_char,
        ),
    >,
    pub init_from_string: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_command_line_t,
            command_line: *const cef_string_t,
        ),
    >,
    pub reset: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t)>,
    pub get_argv: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t, argv: cef_string_list_t),
    >,
    pub get_command_line_string: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t) -> cef_string_userfree_t,
    >,
    pub get_program: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t) -> cef_string_userfree_t,
    >,
    pub set_program: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t, program: *const cef_string_t),
    >,
    pub has_switches: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t) -> ::std::os::raw::c_int,
    >,
    pub has_switch: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_command_line_t,
            name: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_switch_value: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_command_line_t,
            name: *const cef_string_t,
        ) -> cef_string_userfree_t,
    >,
    pub get_switches: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t, switches: cef_string_map_t),
    >,
    pub append_switch: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t, name: *const cef_string_t),
    >,
    pub append_switch_with_value: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_command_line_t,
            name: *const cef_string_t,
            value: *const cef_string_t,
        ),
    >,
    pub has_arguments: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t) -> ::std::os::raw::c_int,
    >,
    pub get_arguments: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t, arguments: cef_string_list_t),
    >,
    pub append_argument: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t, argument: *const cef_string_t),
    >,
    pub prepend_wrapper: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_command_line_t, wrapper: *const cef_string_t),
    >,
}
impl From<_cef_command_line_t> for CommandLine {
    fn from(value: _cef_command_line_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            is_read_only: value.is_read_only.into(),
            copy: value.copy.into(),
            init_from_argv: value.init_from_argv.into(),
            init_from_string: value.init_from_string.into(),
            reset: value.reset.into(),
            get_argv: value.get_argv.into(),
            get_command_line_string: value.get_command_line_string.into(),
            get_program: value.get_program.into(),
            set_program: value.set_program.into(),
            has_switches: value.has_switches.into(),
            has_switch: value.has_switch.into(),
            get_switch_value: value.get_switch_value.into(),
            get_switches: value.get_switches.into(),
            append_switch: value.append_switch.into(),
            append_switch_with_value: value.append_switch_with_value.into(),
            has_arguments: value.has_arguments.into(),
            get_arguments: value.get_arguments.into(),
            append_argument: value.append_argument.into(),
            prepend_wrapper: value.prepend_wrapper.into(),
        }
    }
}
impl Into<_cef_command_line_t> for CommandLine {
    fn into(self) -> _cef_command_line_t {
        _cef_command_line_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            is_read_only: self.is_read_only.into(),
            copy: self.copy.into(),
            init_from_argv: self.init_from_argv.into(),
            init_from_string: self.init_from_string.into(),
            reset: self.reset.into(),
            get_argv: self.get_argv.into(),
            get_command_line_string: self.get_command_line_string.into(),
            get_program: self.get_program.into(),
            set_program: self.set_program.into(),
            has_switches: self.has_switches.into(),
            has_switch: self.has_switch.into(),
            get_switch_value: self.get_switch_value.into(),
            get_switches: self.get_switches.into(),
            append_switch: self.append_switch.into(),
            append_switch_with_value: self.append_switch_with_value.into(),
            has_arguments: self.has_arguments.into(),
            get_arguments: self.get_arguments.into(),
            append_argument: self.append_argument.into(),
            prepend_wrapper: self.prepend_wrapper.into(),
        }
    }
}
impl Default for CommandLine {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_request_context_handler_t] for more documentation.
#[derive(Clone)]
pub struct RequestContextHandler {
    pub base: BaseRefCounted,
    pub on_request_context_initialized: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_handler_t,
            request_context: *mut _cef_request_context_t,
        ),
    >,
    pub get_resource_request_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_request_context_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            request: *mut _cef_request_t,
            is_navigation: ::std::os::raw::c_int,
            is_download: ::std::os::raw::c_int,
            request_initiator: *const cef_string_t,
            disable_default_handling: *mut ::std::os::raw::c_int,
        ) -> *mut _cef_resource_request_handler_t,
    >,
}
impl From<_cef_request_context_handler_t> for RequestContextHandler {
    fn from(value: _cef_request_context_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_request_context_initialized: value.on_request_context_initialized.into(),
            get_resource_request_handler: value.get_resource_request_handler.into(),
        }
    }
}
impl Into<_cef_request_context_handler_t> for RequestContextHandler {
    fn into(self) -> _cef_request_context_handler_t {
        _cef_request_context_handler_t {
            base: self.base.into(),
            on_request_context_initialized: self.on_request_context_initialized.into(),
            get_resource_request_handler: self.get_resource_request_handler.into(),
        }
    }
}
impl Default for RequestContextHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_browser_process_handler_t] for more documentation.
#[derive(Clone)]
pub struct BrowserProcessHandler {
    pub base: BaseRefCounted,
    pub on_register_custom_preferences: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_process_handler_t,
            type_: cef_preferences_type_t,
            registrar: *mut _cef_preference_registrar_t,
        ),
    >,
    pub on_context_initialized: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_process_handler_t),
    >,
    pub on_before_child_process_launch: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_process_handler_t,
            command_line: *mut _cef_command_line_t,
        ),
    >,
    pub on_already_running_app_relaunch: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_process_handler_t,
            command_line: *mut _cef_command_line_t,
            current_directory: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_schedule_message_pump_work: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_process_handler_t, delay_ms: i64),
    >,
    pub get_default_client: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_process_handler_t,
        ) -> *mut _cef_client_t,
    >,
    pub get_default_request_context_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_process_handler_t,
        ) -> *mut _cef_request_context_handler_t,
    >,
}
impl From<_cef_browser_process_handler_t> for BrowserProcessHandler {
    fn from(value: _cef_browser_process_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_register_custom_preferences: value.on_register_custom_preferences.into(),
            on_context_initialized: value.on_context_initialized.into(),
            on_before_child_process_launch: value.on_before_child_process_launch.into(),
            on_already_running_app_relaunch: value.on_already_running_app_relaunch.into(),
            on_schedule_message_pump_work: value.on_schedule_message_pump_work.into(),
            get_default_client: value.get_default_client.into(),
            get_default_request_context_handler: value.get_default_request_context_handler.into(),
        }
    }
}
impl Into<_cef_browser_process_handler_t> for BrowserProcessHandler {
    fn into(self) -> _cef_browser_process_handler_t {
        _cef_browser_process_handler_t {
            base: self.base.into(),
            on_register_custom_preferences: self.on_register_custom_preferences.into(),
            on_context_initialized: self.on_context_initialized.into(),
            on_before_child_process_launch: self.on_before_child_process_launch.into(),
            on_already_running_app_relaunch: self.on_already_running_app_relaunch.into(),
            on_schedule_message_pump_work: self.on_schedule_message_pump_work.into(),
            get_default_client: self.get_default_client.into(),
            get_default_request_context_handler: self.get_default_request_context_handler.into(),
        }
    }
}
impl Default for BrowserProcessHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_task_t] for more documentation.
#[derive(Clone)]
pub struct Task {
    pub base: BaseRefCounted,
    pub execute: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_task_t)>,
}
impl From<_cef_task_t> for Task {
    fn from(value: _cef_task_t) -> Self {
        Self {
            base: value.base.into(),
            execute: value.execute.into(),
        }
    }
}
impl Into<_cef_task_t> for Task {
    fn into(self) -> _cef_task_t {
        _cef_task_t {
            base: self.base.into(),
            execute: self.execute.into(),
        }
    }
}
impl Default for Task {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_task_runner_t] for more documentation.
#[derive(Clone)]
pub struct TaskRunner {
    pub base: BaseRefCounted,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_task_runner_t,
            that: *mut _cef_task_runner_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub belongs_to_current_thread: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_task_runner_t) -> ::std::os::raw::c_int,
    >,
    pub belongs_to_thread: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_task_runner_t,
            threadId: cef_thread_id_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub post_task: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_task_runner_t,
            task: *mut _cef_task_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub post_delayed_task: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_task_runner_t,
            task: *mut _cef_task_t,
            delay_ms: i64,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_task_runner_t> for TaskRunner {
    fn from(value: _cef_task_runner_t) -> Self {
        Self {
            base: value.base.into(),
            is_same: value.is_same.into(),
            belongs_to_current_thread: value.belongs_to_current_thread.into(),
            belongs_to_thread: value.belongs_to_thread.into(),
            post_task: value.post_task.into(),
            post_delayed_task: value.post_delayed_task.into(),
        }
    }
}
impl Into<_cef_task_runner_t> for TaskRunner {
    fn into(self) -> _cef_task_runner_t {
        _cef_task_runner_t {
            base: self.base.into(),
            is_same: self.is_same.into(),
            belongs_to_current_thread: self.belongs_to_current_thread.into(),
            belongs_to_thread: self.belongs_to_thread.into(),
            post_task: self.post_task.into(),
            post_delayed_task: self.post_delayed_task.into(),
        }
    }
}
impl Default for TaskRunner {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_v8_context_t] for more documentation.
#[derive(Clone)]
pub struct V8Context {
    pub base: BaseRefCounted,
    pub get_task_runner: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_context_t) -> *mut _cef_task_runner_t,
    >,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_context_t) -> ::std::os::raw::c_int,
    >,
    pub get_browser: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_context_t) -> *mut _cef_browser_t,
    >,
    pub get_frame: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_context_t) -> *mut _cef_frame_t,
    >,
    pub get_global: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_context_t) -> *mut _cef_v8_value_t,
    >,
    pub enter: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_context_t) -> ::std::os::raw::c_int,
    >,
    pub exit: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_context_t) -> ::std::os::raw::c_int,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_context_t,
            that: *mut _cef_v8_context_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub eval: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_context_t,
            code: *const cef_string_t,
            script_url: *const cef_string_t,
            start_line: ::std::os::raw::c_int,
            retval: *mut *mut _cef_v8_value_t,
            exception: *mut *mut _cef_v8_exception_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_v8_context_t> for V8Context {
    fn from(value: _cef_v8_context_t) -> Self {
        Self {
            base: value.base.into(),
            get_task_runner: value.get_task_runner.into(),
            is_valid: value.is_valid.into(),
            get_browser: value.get_browser.into(),
            get_frame: value.get_frame.into(),
            get_global: value.get_global.into(),
            enter: value.enter.into(),
            exit: value.exit.into(),
            is_same: value.is_same.into(),
            eval: value.eval.into(),
        }
    }
}
impl Into<_cef_v8_context_t> for V8Context {
    fn into(self) -> _cef_v8_context_t {
        _cef_v8_context_t {
            base: self.base.into(),
            get_task_runner: self.get_task_runner.into(),
            is_valid: self.is_valid.into(),
            get_browser: self.get_browser.into(),
            get_frame: self.get_frame.into(),
            get_global: self.get_global.into(),
            enter: self.enter.into(),
            exit: self.exit.into(),
            is_same: self.is_same.into(),
            eval: self.eval.into(),
        }
    }
}
impl Default for V8Context {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_v8_handler_t] for more documentation.
#[derive(Clone)]
pub struct V8Handler {
    pub base: BaseRefCounted,
    pub execute: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_handler_t,
            name: *const cef_string_t,
            object: *mut _cef_v8_value_t,
            argumentsCount: usize,
            arguments: *const *mut _cef_v8_value_t,
            retval: *mut *mut _cef_v8_value_t,
            exception: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_v8_handler_t> for V8Handler {
    fn from(value: _cef_v8_handler_t) -> Self {
        Self {
            base: value.base.into(),
            execute: value.execute.into(),
        }
    }
}
impl Into<_cef_v8_handler_t> for V8Handler {
    fn into(self) -> _cef_v8_handler_t {
        _cef_v8_handler_t {
            base: self.base.into(),
            execute: self.execute.into(),
        }
    }
}
impl Default for V8Handler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_v8_accessor_t] for more documentation.
#[derive(Clone)]
pub struct V8Accessor {
    pub base: BaseRefCounted,
    pub get: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_accessor_t,
            name: *const cef_string_t,
            object: *mut _cef_v8_value_t,
            retval: *mut *mut _cef_v8_value_t,
            exception: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_accessor_t,
            name: *const cef_string_t,
            object: *mut _cef_v8_value_t,
            value: *mut _cef_v8_value_t,
            exception: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_v8_accessor_t> for V8Accessor {
    fn from(value: _cef_v8_accessor_t) -> Self {
        Self {
            base: value.base.into(),
            get: value.get.into(),
            set: value.set.into(),
        }
    }
}
impl Into<_cef_v8_accessor_t> for V8Accessor {
    fn into(self) -> _cef_v8_accessor_t {
        _cef_v8_accessor_t {
            base: self.base.into(),
            get: self.get.into(),
            set: self.set.into(),
        }
    }
}
impl Default for V8Accessor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_v8_interceptor_t] for more documentation.
#[derive(Clone)]
pub struct V8Interceptor {
    pub base: BaseRefCounted,
    pub get_byname: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_interceptor_t,
            name: *const cef_string_t,
            object: *mut _cef_v8_value_t,
            retval: *mut *mut _cef_v8_value_t,
            exception: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_byindex: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_interceptor_t,
            index: ::std::os::raw::c_int,
            object: *mut _cef_v8_value_t,
            retval: *mut *mut _cef_v8_value_t,
            exception: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_byname: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_interceptor_t,
            name: *const cef_string_t,
            object: *mut _cef_v8_value_t,
            value: *mut _cef_v8_value_t,
            exception: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_byindex: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_interceptor_t,
            index: ::std::os::raw::c_int,
            object: *mut _cef_v8_value_t,
            value: *mut _cef_v8_value_t,
            exception: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_v8_interceptor_t> for V8Interceptor {
    fn from(value: _cef_v8_interceptor_t) -> Self {
        Self {
            base: value.base.into(),
            get_byname: value.get_byname.into(),
            get_byindex: value.get_byindex.into(),
            set_byname: value.set_byname.into(),
            set_byindex: value.set_byindex.into(),
        }
    }
}
impl Into<_cef_v8_interceptor_t> for V8Interceptor {
    fn into(self) -> _cef_v8_interceptor_t {
        _cef_v8_interceptor_t {
            base: self.base.into(),
            get_byname: self.get_byname.into(),
            get_byindex: self.get_byindex.into(),
            set_byname: self.set_byname.into(),
            set_byindex: self.set_byindex.into(),
        }
    }
}
impl Default for V8Interceptor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_v8_exception_t] for more documentation.
#[derive(Clone)]
pub struct V8Exception {
    pub base: BaseRefCounted,
    pub get_message: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_exception_t) -> cef_string_userfree_t,
    >,
    pub get_source_line: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_exception_t) -> cef_string_userfree_t,
    >,
    pub get_script_resource_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_exception_t) -> cef_string_userfree_t,
    >,
    pub get_line_number: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_exception_t) -> ::std::os::raw::c_int,
    >,
    pub get_start_position: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_exception_t) -> ::std::os::raw::c_int,
    >,
    pub get_end_position: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_exception_t) -> ::std::os::raw::c_int,
    >,
    pub get_start_column: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_exception_t) -> ::std::os::raw::c_int,
    >,
    pub get_end_column: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_exception_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_v8_exception_t> for V8Exception {
    fn from(value: _cef_v8_exception_t) -> Self {
        Self {
            base: value.base.into(),
            get_message: value.get_message.into(),
            get_source_line: value.get_source_line.into(),
            get_script_resource_name: value.get_script_resource_name.into(),
            get_line_number: value.get_line_number.into(),
            get_start_position: value.get_start_position.into(),
            get_end_position: value.get_end_position.into(),
            get_start_column: value.get_start_column.into(),
            get_end_column: value.get_end_column.into(),
        }
    }
}
impl Into<_cef_v8_exception_t> for V8Exception {
    fn into(self) -> _cef_v8_exception_t {
        _cef_v8_exception_t {
            base: self.base.into(),
            get_message: self.get_message.into(),
            get_source_line: self.get_source_line.into(),
            get_script_resource_name: self.get_script_resource_name.into(),
            get_line_number: self.get_line_number.into(),
            get_start_position: self.get_start_position.into(),
            get_end_position: self.get_end_position.into(),
            get_start_column: self.get_start_column.into(),
            get_end_column: self.get_end_column.into(),
        }
    }
}
impl Default for V8Exception {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_v8_array_buffer_release_callback_t] for more documentation.
#[derive(Clone)]
pub struct V8ArrayBufferReleaseCallback {
    pub base: BaseRefCounted,
    pub release_buffer: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_array_buffer_release_callback_t,
            buffer: *mut ::std::os::raw::c_void,
        ),
    >,
}
impl From<_cef_v8_array_buffer_release_callback_t> for V8ArrayBufferReleaseCallback {
    fn from(value: _cef_v8_array_buffer_release_callback_t) -> Self {
        Self {
            base: value.base.into(),
            release_buffer: value.release_buffer.into(),
        }
    }
}
impl Into<_cef_v8_array_buffer_release_callback_t> for V8ArrayBufferReleaseCallback {
    fn into(self) -> _cef_v8_array_buffer_release_callback_t {
        _cef_v8_array_buffer_release_callback_t {
            base: self.base.into(),
            release_buffer: self.release_buffer.into(),
        }
    }
}
impl Default for V8ArrayBufferReleaseCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_v8_value_t] for more documentation.
#[derive(Clone)]
pub struct V8Value {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_undefined: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_null: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_bool: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_int: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_uint: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_double: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_date: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_string: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_object: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_array: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_array_buffer: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_function: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_promise: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            that: *mut _cef_v8_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_bool_value: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub get_int_value:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> i32>,
    pub get_uint_value:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> u32>,
    pub get_double_value:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> f64>,
    pub get_date_value: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> cef_basetime_t,
    >,
    pub get_string_value: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> cef_string_userfree_t,
    >,
    pub is_user_created: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub has_exception: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub get_exception: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> *mut _cef_v8_exception_t,
    >,
    pub clear_exception: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub will_rethrow_exceptions: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub set_rethrow_exceptions: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            rethrow: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub has_value_bykey: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            key: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub has_value_byindex: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            index: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub delete_value_bykey: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            key: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub delete_value_byindex: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            index: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_value_bykey: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            key: *const cef_string_t,
        ) -> *mut _cef_v8_value_t,
    >,
    pub get_value_byindex: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            index: ::std::os::raw::c_int,
        ) -> *mut _cef_v8_value_t,
    >,
    pub set_value_bykey: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            key: *const cef_string_t,
            value: *mut _cef_v8_value_t,
            attribute: cef_v8_propertyattribute_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_value_byindex: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            index: ::std::os::raw::c_int,
            value: *mut _cef_v8_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_value_byaccessor: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            key: *const cef_string_t,
            attribute: cef_v8_propertyattribute_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_keys: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            keys: cef_string_list_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub set_user_data: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            user_data: *mut _cef_base_ref_counted_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_user_data: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> *mut _cef_base_ref_counted_t,
    >,
    pub get_externally_allocated_memory: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub adjust_externally_allocated_memory: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            change_in_bytes: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_array_length: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub get_array_buffer_release_callback: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
        ) -> *mut _cef_v8_array_buffer_release_callback_t,
    >,
    pub neuter_array_buffer: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> ::std::os::raw::c_int,
    >,
    pub get_array_buffer_byte_length:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> usize>,
    pub get_array_buffer_data: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> *mut ::std::os::raw::c_void,
    >,
    pub get_function_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> cef_string_userfree_t,
    >,
    pub get_function_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_value_t) -> *mut _cef_v8_handler_t,
    >,
    pub execute_function: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            object: *mut _cef_v8_value_t,
            argumentsCount: usize,
            arguments: *const *mut _cef_v8_value_t,
        ) -> *mut _cef_v8_value_t,
    >,
    pub execute_function_with_context: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            context: *mut _cef_v8_context_t,
            object: *mut _cef_v8_value_t,
            argumentsCount: usize,
            arguments: *const *mut _cef_v8_value_t,
        ) -> *mut _cef_v8_value_t,
    >,
    pub resolve_promise: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            arg: *mut _cef_v8_value_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub reject_promise: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_value_t,
            errorMsg: *const cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_v8_value_t> for V8Value {
    fn from(value: _cef_v8_value_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            is_undefined: value.is_undefined.into(),
            is_null: value.is_null.into(),
            is_bool: value.is_bool.into(),
            is_int: value.is_int.into(),
            is_uint: value.is_uint.into(),
            is_double: value.is_double.into(),
            is_date: value.is_date.into(),
            is_string: value.is_string.into(),
            is_object: value.is_object.into(),
            is_array: value.is_array.into(),
            is_array_buffer: value.is_array_buffer.into(),
            is_function: value.is_function.into(),
            is_promise: value.is_promise.into(),
            is_same: value.is_same.into(),
            get_bool_value: value.get_bool_value.into(),
            get_int_value: value.get_int_value.into(),
            get_uint_value: value.get_uint_value.into(),
            get_double_value: value.get_double_value.into(),
            get_date_value: value.get_date_value.into(),
            get_string_value: value.get_string_value.into(),
            is_user_created: value.is_user_created.into(),
            has_exception: value.has_exception.into(),
            get_exception: value.get_exception.into(),
            clear_exception: value.clear_exception.into(),
            will_rethrow_exceptions: value.will_rethrow_exceptions.into(),
            set_rethrow_exceptions: value.set_rethrow_exceptions.into(),
            has_value_bykey: value.has_value_bykey.into(),
            has_value_byindex: value.has_value_byindex.into(),
            delete_value_bykey: value.delete_value_bykey.into(),
            delete_value_byindex: value.delete_value_byindex.into(),
            get_value_bykey: value.get_value_bykey.into(),
            get_value_byindex: value.get_value_byindex.into(),
            set_value_bykey: value.set_value_bykey.into(),
            set_value_byindex: value.set_value_byindex.into(),
            set_value_byaccessor: value.set_value_byaccessor.into(),
            get_keys: value.get_keys.into(),
            set_user_data: value.set_user_data.into(),
            get_user_data: value.get_user_data.into(),
            get_externally_allocated_memory: value.get_externally_allocated_memory.into(),
            adjust_externally_allocated_memory: value.adjust_externally_allocated_memory.into(),
            get_array_length: value.get_array_length.into(),
            get_array_buffer_release_callback: value.get_array_buffer_release_callback.into(),
            neuter_array_buffer: value.neuter_array_buffer.into(),
            get_array_buffer_byte_length: value.get_array_buffer_byte_length.into(),
            get_array_buffer_data: value.get_array_buffer_data.into(),
            get_function_name: value.get_function_name.into(),
            get_function_handler: value.get_function_handler.into(),
            execute_function: value.execute_function.into(),
            execute_function_with_context: value.execute_function_with_context.into(),
            resolve_promise: value.resolve_promise.into(),
            reject_promise: value.reject_promise.into(),
        }
    }
}
impl Into<_cef_v8_value_t> for V8Value {
    fn into(self) -> _cef_v8_value_t {
        _cef_v8_value_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            is_undefined: self.is_undefined.into(),
            is_null: self.is_null.into(),
            is_bool: self.is_bool.into(),
            is_int: self.is_int.into(),
            is_uint: self.is_uint.into(),
            is_double: self.is_double.into(),
            is_date: self.is_date.into(),
            is_string: self.is_string.into(),
            is_object: self.is_object.into(),
            is_array: self.is_array.into(),
            is_array_buffer: self.is_array_buffer.into(),
            is_function: self.is_function.into(),
            is_promise: self.is_promise.into(),
            is_same: self.is_same.into(),
            get_bool_value: self.get_bool_value.into(),
            get_int_value: self.get_int_value.into(),
            get_uint_value: self.get_uint_value.into(),
            get_double_value: self.get_double_value.into(),
            get_date_value: self.get_date_value.into(),
            get_string_value: self.get_string_value.into(),
            is_user_created: self.is_user_created.into(),
            has_exception: self.has_exception.into(),
            get_exception: self.get_exception.into(),
            clear_exception: self.clear_exception.into(),
            will_rethrow_exceptions: self.will_rethrow_exceptions.into(),
            set_rethrow_exceptions: self.set_rethrow_exceptions.into(),
            has_value_bykey: self.has_value_bykey.into(),
            has_value_byindex: self.has_value_byindex.into(),
            delete_value_bykey: self.delete_value_bykey.into(),
            delete_value_byindex: self.delete_value_byindex.into(),
            get_value_bykey: self.get_value_bykey.into(),
            get_value_byindex: self.get_value_byindex.into(),
            set_value_bykey: self.set_value_bykey.into(),
            set_value_byindex: self.set_value_byindex.into(),
            set_value_byaccessor: self.set_value_byaccessor.into(),
            get_keys: self.get_keys.into(),
            set_user_data: self.set_user_data.into(),
            get_user_data: self.get_user_data.into(),
            get_externally_allocated_memory: self.get_externally_allocated_memory.into(),
            adjust_externally_allocated_memory: self.adjust_externally_allocated_memory.into(),
            get_array_length: self.get_array_length.into(),
            get_array_buffer_release_callback: self.get_array_buffer_release_callback.into(),
            neuter_array_buffer: self.neuter_array_buffer.into(),
            get_array_buffer_byte_length: self.get_array_buffer_byte_length.into(),
            get_array_buffer_data: self.get_array_buffer_data.into(),
            get_function_name: self.get_function_name.into(),
            get_function_handler: self.get_function_handler.into(),
            execute_function: self.execute_function.into(),
            execute_function_with_context: self.execute_function_with_context.into(),
            resolve_promise: self.resolve_promise.into(),
            reject_promise: self.reject_promise.into(),
        }
    }
}
impl Default for V8Value {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_v8_stack_trace_t] for more documentation.
#[derive(Clone)]
pub struct V8StackTrace {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_stack_trace_t) -> ::std::os::raw::c_int,
    >,
    pub get_frame_count: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_stack_trace_t) -> ::std::os::raw::c_int,
    >,
    pub get_frame: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_v8_stack_trace_t,
            index: ::std::os::raw::c_int,
        ) -> *mut _cef_v8_stack_frame_t,
    >,
}
impl From<_cef_v8_stack_trace_t> for V8StackTrace {
    fn from(value: _cef_v8_stack_trace_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            get_frame_count: value.get_frame_count.into(),
            get_frame: value.get_frame.into(),
        }
    }
}
impl Into<_cef_v8_stack_trace_t> for V8StackTrace {
    fn into(self) -> _cef_v8_stack_trace_t {
        _cef_v8_stack_trace_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            get_frame_count: self.get_frame_count.into(),
            get_frame: self.get_frame.into(),
        }
    }
}
impl Default for V8StackTrace {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_v8_stack_frame_t] for more documentation.
#[derive(Clone)]
pub struct V8StackFrame {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_stack_frame_t) -> ::std::os::raw::c_int,
    >,
    pub get_script_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_stack_frame_t) -> cef_string_userfree_t,
    >,
    pub get_script_name_or_source_url: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_stack_frame_t) -> cef_string_userfree_t,
    >,
    pub get_function_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_stack_frame_t) -> cef_string_userfree_t,
    >,
    pub get_line_number: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_stack_frame_t) -> ::std::os::raw::c_int,
    >,
    pub get_column: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_stack_frame_t) -> ::std::os::raw::c_int,
    >,
    pub is_eval: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_stack_frame_t) -> ::std::os::raw::c_int,
    >,
    pub is_constructor: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_v8_stack_frame_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_v8_stack_frame_t> for V8StackFrame {
    fn from(value: _cef_v8_stack_frame_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            get_script_name: value.get_script_name.into(),
            get_script_name_or_source_url: value.get_script_name_or_source_url.into(),
            get_function_name: value.get_function_name.into(),
            get_line_number: value.get_line_number.into(),
            get_column: value.get_column.into(),
            is_eval: value.is_eval.into(),
            is_constructor: value.is_constructor.into(),
        }
    }
}
impl Into<_cef_v8_stack_frame_t> for V8StackFrame {
    fn into(self) -> _cef_v8_stack_frame_t {
        _cef_v8_stack_frame_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            get_script_name: self.get_script_name.into(),
            get_script_name_or_source_url: self.get_script_name_or_source_url.into(),
            get_function_name: self.get_function_name.into(),
            get_line_number: self.get_line_number.into(),
            get_column: self.get_column.into(),
            is_eval: self.is_eval.into(),
            is_constructor: self.is_constructor.into(),
        }
    }
}
impl Default for V8StackFrame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_render_process_handler_t] for more documentation.
#[derive(Clone)]
pub struct RenderProcessHandler {
    pub base: BaseRefCounted,
    pub on_web_kit_initialized: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_render_process_handler_t),
    >,
    pub on_browser_created: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            extra_info: *mut _cef_dictionary_value_t,
        ),
    >,
    pub on_browser_destroyed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
        ),
    >,
    pub get_load_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_process_handler_t,
        ) -> *mut _cef_load_handler_t,
    >,
    pub on_context_created: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            context: *mut _cef_v8_context_t,
        ),
    >,
    pub on_context_released: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            context: *mut _cef_v8_context_t,
        ),
    >,
    pub on_uncaught_exception: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            context: *mut _cef_v8_context_t,
            exception: *mut _cef_v8_exception_t,
            stackTrace: *mut _cef_v8_stack_trace_t,
        ),
    >,
    pub on_focused_node_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            node: *mut _cef_domnode_t,
        ),
    >,
    pub on_process_message_received: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_render_process_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            source_process: cef_process_id_t,
            message: *mut _cef_process_message_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_render_process_handler_t> for RenderProcessHandler {
    fn from(value: _cef_render_process_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_web_kit_initialized: value.on_web_kit_initialized.into(),
            on_browser_created: value.on_browser_created.into(),
            on_browser_destroyed: value.on_browser_destroyed.into(),
            get_load_handler: value.get_load_handler.into(),
            on_context_created: value.on_context_created.into(),
            on_context_released: value.on_context_released.into(),
            on_uncaught_exception: value.on_uncaught_exception.into(),
            on_focused_node_changed: value.on_focused_node_changed.into(),
            on_process_message_received: value.on_process_message_received.into(),
        }
    }
}
impl Into<_cef_render_process_handler_t> for RenderProcessHandler {
    fn into(self) -> _cef_render_process_handler_t {
        _cef_render_process_handler_t {
            base: self.base.into(),
            on_web_kit_initialized: self.on_web_kit_initialized.into(),
            on_browser_created: self.on_browser_created.into(),
            on_browser_destroyed: self.on_browser_destroyed.into(),
            get_load_handler: self.get_load_handler.into(),
            on_context_created: self.on_context_created.into(),
            on_context_released: self.on_context_released.into(),
            on_uncaught_exception: self.on_uncaught_exception.into(),
            on_focused_node_changed: self.on_focused_node_changed.into(),
            on_process_message_received: self.on_process_message_received.into(),
        }
    }
}
impl Default for RenderProcessHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_resource_bundle_handler_t] for more documentation.
#[derive(Clone)]
pub struct ResourceBundleHandler {
    pub base: BaseRefCounted,
    pub get_localized_string: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_bundle_handler_t,
            string_id: ::std::os::raw::c_int,
            string: *mut cef_string_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_data_resource: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_bundle_handler_t,
            resource_id: ::std::os::raw::c_int,
            data: *mut *mut ::std::os::raw::c_void,
            data_size: *mut usize,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_data_resource_for_scale: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_resource_bundle_handler_t,
            resource_id: ::std::os::raw::c_int,
            scale_factor: cef_scale_factor_t,
            data: *mut *mut ::std::os::raw::c_void,
            data_size: *mut usize,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_resource_bundle_handler_t> for ResourceBundleHandler {
    fn from(value: _cef_resource_bundle_handler_t) -> Self {
        Self {
            base: value.base.into(),
            get_localized_string: value.get_localized_string.into(),
            get_data_resource: value.get_data_resource.into(),
            get_data_resource_for_scale: value.get_data_resource_for_scale.into(),
        }
    }
}
impl Into<_cef_resource_bundle_handler_t> for ResourceBundleHandler {
    fn into(self) -> _cef_resource_bundle_handler_t {
        _cef_resource_bundle_handler_t {
            base: self.base.into(),
            get_localized_string: self.get_localized_string.into(),
            get_data_resource: self.get_data_resource.into(),
            get_data_resource_for_scale: self.get_data_resource_for_scale.into(),
        }
    }
}
impl Default for ResourceBundleHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_scheme_registrar_t] for more documentation.
#[derive(Clone)]
pub struct SchemeRegistrar {
    pub base: BaseScoped,
    pub add_custom_scheme: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_scheme_registrar_t,
            scheme_name: *const cef_string_t,
            options: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_scheme_registrar_t> for SchemeRegistrar {
    fn from(value: _cef_scheme_registrar_t) -> Self {
        Self {
            base: value.base.into(),
            add_custom_scheme: value.add_custom_scheme.into(),
        }
    }
}
impl Into<_cef_scheme_registrar_t> for SchemeRegistrar {
    fn into(self) -> _cef_scheme_registrar_t {
        _cef_scheme_registrar_t {
            base: self.base.into(),
            add_custom_scheme: self.add_custom_scheme.into(),
        }
    }
}
impl Default for SchemeRegistrar {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_scheme_handler_factory_t] for more documentation.
#[derive(Clone)]
pub struct SchemeHandlerFactory {
    pub base: BaseRefCounted,
    pub create: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_scheme_handler_factory_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            scheme_name: *const cef_string_t,
            request: *mut _cef_request_t,
        ) -> *mut _cef_resource_handler_t,
    >,
}
impl From<_cef_scheme_handler_factory_t> for SchemeHandlerFactory {
    fn from(value: _cef_scheme_handler_factory_t) -> Self {
        Self {
            base: value.base.into(),
            create: value.create.into(),
        }
    }
}
impl Into<_cef_scheme_handler_factory_t> for SchemeHandlerFactory {
    fn into(self) -> _cef_scheme_handler_factory_t {
        _cef_scheme_handler_factory_t {
            base: self.base.into(),
            create: self.create.into(),
        }
    }
}
impl Default for SchemeHandlerFactory {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_app_t] for more documentation.
#[derive(Clone)]
pub struct App {
    pub base: BaseRefCounted,
    pub on_before_command_line_processing: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_app_t,
            process_type: *const cef_string_t,
            command_line: *mut _cef_command_line_t,
        ),
    >,
    pub on_register_custom_schemes: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_app_t, registrar: *mut _cef_scheme_registrar_t),
    >,
    pub get_resource_bundle_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_app_t) -> *mut _cef_resource_bundle_handler_t,
    >,
    pub get_browser_process_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_app_t) -> *mut _cef_browser_process_handler_t,
    >,
    pub get_render_process_handler: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_app_t) -> *mut _cef_render_process_handler_t,
    >,
}
impl From<_cef_app_t> for App {
    fn from(value: _cef_app_t) -> Self {
        Self {
            base: value.base.into(),
            on_before_command_line_processing: value.on_before_command_line_processing.into(),
            on_register_custom_schemes: value.on_register_custom_schemes.into(),
            get_resource_bundle_handler: value.get_resource_bundle_handler.into(),
            get_browser_process_handler: value.get_browser_process_handler.into(),
            get_render_process_handler: value.get_render_process_handler.into(),
        }
    }
}
impl Into<_cef_app_t> for App {
    fn into(self) -> _cef_app_t {
        _cef_app_t {
            base: self.base.into(),
            on_before_command_line_processing: self.on_before_command_line_processing.into(),
            on_register_custom_schemes: self.on_register_custom_schemes.into(),
            get_resource_bundle_handler: self.get_resource_bundle_handler.into(),
            get_browser_process_handler: self.get_browser_process_handler.into(),
            get_render_process_handler: self.get_render_process_handler.into(),
        }
    }
}
impl Default for App {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_urlrequest_t] for more documentation.
#[derive(Clone)]
pub struct Urlrequest {
    pub base: BaseRefCounted,
    pub get_request: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_urlrequest_t) -> *mut _cef_request_t,
    >,
    pub get_client: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_urlrequest_t) -> *mut _cef_urlrequest_client_t,
    >,
    pub get_request_status: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_urlrequest_t) -> cef_urlrequest_status_t,
    >,
    pub get_request_error: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_urlrequest_t) -> cef_errorcode_t,
    >,
    pub get_response: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_urlrequest_t) -> *mut _cef_response_t,
    >,
    pub response_was_cached: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_urlrequest_t) -> ::std::os::raw::c_int,
    >,
    pub cancel: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_urlrequest_t)>,
}
impl From<_cef_urlrequest_t> for Urlrequest {
    fn from(value: _cef_urlrequest_t) -> Self {
        Self {
            base: value.base.into(),
            get_request: value.get_request.into(),
            get_client: value.get_client.into(),
            get_request_status: value.get_request_status.into(),
            get_request_error: value.get_request_error.into(),
            get_response: value.get_response.into(),
            response_was_cached: value.response_was_cached.into(),
            cancel: value.cancel.into(),
        }
    }
}
impl Into<_cef_urlrequest_t> for Urlrequest {
    fn into(self) -> _cef_urlrequest_t {
        _cef_urlrequest_t {
            base: self.base.into(),
            get_request: self.get_request.into(),
            get_client: self.get_client.into(),
            get_request_status: self.get_request_status.into(),
            get_request_error: self.get_request_error.into(),
            get_response: self.get_response.into(),
            response_was_cached: self.response_was_cached.into(),
            cancel: self.cancel.into(),
        }
    }
}
impl Default for Urlrequest {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_urlrequest_client_t] for more documentation.
#[derive(Clone)]
pub struct UrlrequestClient {
    pub base: BaseRefCounted,
    pub on_request_complete: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_urlrequest_client_t,
            request: *mut _cef_urlrequest_t,
        ),
    >,
    pub on_upload_progress: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_urlrequest_client_t,
            request: *mut _cef_urlrequest_t,
            current: i64,
            total: i64,
        ),
    >,
    pub on_download_progress: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_urlrequest_client_t,
            request: *mut _cef_urlrequest_t,
            current: i64,
            total: i64,
        ),
    >,
    pub on_download_data: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_urlrequest_client_t,
            request: *mut _cef_urlrequest_t,
            data: *const ::std::os::raw::c_void,
            data_length: usize,
        ),
    >,
    pub get_auth_credentials: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_urlrequest_client_t,
            isProxy: ::std::os::raw::c_int,
            host: *const cef_string_t,
            port: ::std::os::raw::c_int,
            realm: *const cef_string_t,
            scheme: *const cef_string_t,
            callback: *mut _cef_auth_callback_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_urlrequest_client_t> for UrlrequestClient {
    fn from(value: _cef_urlrequest_client_t) -> Self {
        Self {
            base: value.base.into(),
            on_request_complete: value.on_request_complete.into(),
            on_upload_progress: value.on_upload_progress.into(),
            on_download_progress: value.on_download_progress.into(),
            on_download_data: value.on_download_data.into(),
            get_auth_credentials: value.get_auth_credentials.into(),
        }
    }
}
impl Into<_cef_urlrequest_client_t> for UrlrequestClient {
    fn into(self) -> _cef_urlrequest_client_t {
        _cef_urlrequest_client_t {
            base: self.base.into(),
            on_request_complete: self.on_request_complete.into(),
            on_upload_progress: self.on_upload_progress.into(),
            on_download_progress: self.on_download_progress.into(),
            on_download_data: self.on_download_data.into(),
            get_auth_credentials: self.get_auth_credentials.into(),
        }
    }
}
impl Default for UrlrequestClient {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_layout_t] for more documentation.
#[derive(Clone)]
pub struct Layout {
    pub base: BaseRefCounted,
    pub as_box_layout: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_layout_t) -> *mut _cef_box_layout_t,
    >,
    pub as_fill_layout: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_layout_t) -> *mut _cef_fill_layout_t,
    >,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_layout_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_layout_t> for Layout {
    fn from(value: _cef_layout_t) -> Self {
        Self {
            base: value.base.into(),
            as_box_layout: value.as_box_layout.into(),
            as_fill_layout: value.as_fill_layout.into(),
            is_valid: value.is_valid.into(),
        }
    }
}
impl Into<_cef_layout_t> for Layout {
    fn into(self) -> _cef_layout_t {
        _cef_layout_t {
            base: self.base.into(),
            as_box_layout: self.as_box_layout.into(),
            as_fill_layout: self.as_fill_layout.into(),
            is_valid: self.is_valid.into(),
        }
    }
}
impl Default for Layout {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_box_layout_t] for more documentation.
#[derive(Clone)]
pub struct BoxLayout {
    pub base: Layout,
    pub set_flex_for_view: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_box_layout_t,
            view: *mut _cef_view_t,
            flex: ::std::os::raw::c_int,
        ),
    >,
    pub clear_flex_for_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_box_layout_t, view: *mut _cef_view_t),
    >,
}
impl From<_cef_box_layout_t> for BoxLayout {
    fn from(value: _cef_box_layout_t) -> Self {
        Self {
            base: value.base.into(),
            set_flex_for_view: value.set_flex_for_view.into(),
            clear_flex_for_view: value.clear_flex_for_view.into(),
        }
    }
}
impl Into<_cef_box_layout_t> for BoxLayout {
    fn into(self) -> _cef_box_layout_t {
        _cef_box_layout_t {
            base: self.base.into(),
            set_flex_for_view: self.set_flex_for_view.into(),
            clear_flex_for_view: self.clear_flex_for_view.into(),
        }
    }
}
impl Default for BoxLayout {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_fill_layout_t] for more documentation.
#[derive(Clone)]
pub struct FillLayout {
    pub base: Layout,
}
impl From<_cef_fill_layout_t> for FillLayout {
    fn from(value: _cef_fill_layout_t) -> Self {
        Self {
            base: value.base.into(),
        }
    }
}
impl Into<_cef_fill_layout_t> for FillLayout {
    fn into(self) -> _cef_fill_layout_t {
        _cef_fill_layout_t {
            base: self.base.into(),
        }
    }
}
impl Default for FillLayout {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_view_delegate_t] for more documentation.
#[derive(Clone)]
pub struct ViewDelegate {
    pub base: BaseRefCounted,
    pub get_preferred_size: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_delegate_t,
            view: *mut _cef_view_t,
        ) -> cef_size_t,
    >,
    pub get_minimum_size: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_delegate_t,
            view: *mut _cef_view_t,
        ) -> cef_size_t,
    >,
    pub get_maximum_size: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_delegate_t,
            view: *mut _cef_view_t,
        ) -> cef_size_t,
    >,
    pub get_height_for_width: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_delegate_t,
            view: *mut _cef_view_t,
            width: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_parent_view_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_delegate_t,
            view: *mut _cef_view_t,
            added: ::std::os::raw::c_int,
            parent: *mut _cef_view_t,
        ),
    >,
    pub on_child_view_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_delegate_t,
            view: *mut _cef_view_t,
            added: ::std::os::raw::c_int,
            child: *mut _cef_view_t,
        ),
    >,
    pub on_window_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_delegate_t,
            view: *mut _cef_view_t,
            added: ::std::os::raw::c_int,
        ),
    >,
    pub on_layout_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_delegate_t,
            view: *mut _cef_view_t,
            new_bounds: *const cef_rect_t,
        ),
    >,
    pub on_focus: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_delegate_t, view: *mut _cef_view_t),
    >,
    pub on_blur: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_delegate_t, view: *mut _cef_view_t),
    >,
    pub on_theme_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_delegate_t, view: *mut _cef_view_t),
    >,
}
impl From<_cef_view_delegate_t> for ViewDelegate {
    fn from(value: _cef_view_delegate_t) -> Self {
        Self {
            base: value.base.into(),
            get_preferred_size: value.get_preferred_size.into(),
            get_minimum_size: value.get_minimum_size.into(),
            get_maximum_size: value.get_maximum_size.into(),
            get_height_for_width: value.get_height_for_width.into(),
            on_parent_view_changed: value.on_parent_view_changed.into(),
            on_child_view_changed: value.on_child_view_changed.into(),
            on_window_changed: value.on_window_changed.into(),
            on_layout_changed: value.on_layout_changed.into(),
            on_focus: value.on_focus.into(),
            on_blur: value.on_blur.into(),
            on_theme_changed: value.on_theme_changed.into(),
        }
    }
}
impl Into<_cef_view_delegate_t> for ViewDelegate {
    fn into(self) -> _cef_view_delegate_t {
        _cef_view_delegate_t {
            base: self.base.into(),
            get_preferred_size: self.get_preferred_size.into(),
            get_minimum_size: self.get_minimum_size.into(),
            get_maximum_size: self.get_maximum_size.into(),
            get_height_for_width: self.get_height_for_width.into(),
            on_parent_view_changed: self.on_parent_view_changed.into(),
            on_child_view_changed: self.on_child_view_changed.into(),
            on_window_changed: self.on_window_changed.into(),
            on_layout_changed: self.on_layout_changed.into(),
            on_focus: self.on_focus.into(),
            on_blur: self.on_blur.into(),
            on_theme_changed: self.on_theme_changed.into(),
        }
    }
}
impl Default for ViewDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_view_t] for more documentation.
#[derive(Clone)]
pub struct View {
    pub base: BaseRefCounted,
    pub as_browser_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> *mut _cef_browser_view_t,
    >,
    pub as_button: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> *mut _cef_button_t,
    >,
    pub as_panel: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> *mut _cef_panel_t,
    >,
    pub as_scroll_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> *mut _cef_scroll_view_t,
    >,
    pub as_textfield: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> *mut _cef_textfield_t,
    >,
    pub get_type_string: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> cef_string_userfree_t,
    >,
    pub to_string: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            include_children: ::std::os::raw::c_int,
        ) -> cef_string_userfree_t,
    >,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> ::std::os::raw::c_int,
    >,
    pub is_attached: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> ::std::os::raw::c_int,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            that: *mut _cef_view_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_delegate: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> *mut _cef_view_delegate_t,
    >,
    pub get_window: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> *mut _cef_window_t,
    >,
    pub get_id: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> ::std::os::raw::c_int,
    >,
    pub set_id: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t, id: ::std::os::raw::c_int),
    >,
    pub get_group_id: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> ::std::os::raw::c_int,
    >,
    pub set_group_id: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t, group_id: ::std::os::raw::c_int),
    >,
    pub get_parent_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> *mut _cef_view_t,
    >,
    pub get_view_for_id: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            id: ::std::os::raw::c_int,
        ) -> *mut _cef_view_t,
    >,
    pub set_bounds: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t, bounds: *const cef_rect_t),
    >,
    pub get_bounds:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> cef_rect_t>,
    pub get_bounds_in_screen:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> cef_rect_t>,
    pub set_size: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t, size: *const cef_size_t),
    >,
    pub get_size:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> cef_size_t>,
    pub set_position: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t, position: *const cef_point_t),
    >,
    pub get_position:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> cef_point_t>,
    pub set_insets: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t, insets: *const cef_insets_t),
    >,
    pub get_insets:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> cef_insets_t>,
    pub get_preferred_size:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> cef_size_t>,
    pub size_to_preferred_size:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t)>,
    pub get_minimum_size:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> cef_size_t>,
    pub get_maximum_size:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> cef_size_t>,
    pub get_height_for_width: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            width: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub invalidate_layout:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t)>,
    pub set_visible: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t, visible: ::std::os::raw::c_int),
    >,
    pub is_visible: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> ::std::os::raw::c_int,
    >,
    pub is_drawn: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> ::std::os::raw::c_int,
    >,
    pub set_enabled: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t, enabled: ::std::os::raw::c_int),
    >,
    pub is_enabled: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> ::std::os::raw::c_int,
    >,
    pub set_focusable: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t, focusable: ::std::os::raw::c_int),
    >,
    pub is_focusable: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> ::std::os::raw::c_int,
    >,
    pub is_accessibility_focusable: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> ::std::os::raw::c_int,
    >,
    pub has_focus: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> ::std::os::raw::c_int,
    >,
    pub request_focus: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t)>,
    pub set_background_color: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_view_t, color: cef_color_t),
    >,
    pub get_background_color:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_view_t) -> cef_color_t>,
    pub get_theme_color: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            color_id: ::std::os::raw::c_int,
        ) -> cef_color_t,
    >,
    pub convert_point_to_screen: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            point: *mut cef_point_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub convert_point_from_screen: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            point: *mut cef_point_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub convert_point_to_window: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            point: *mut cef_point_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub convert_point_from_window: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            point: *mut cef_point_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub convert_point_to_view: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            view: *mut _cef_view_t,
            point: *mut cef_point_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub convert_point_from_view: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_view_t,
            view: *mut _cef_view_t,
            point: *mut cef_point_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_view_t> for View {
    fn from(value: _cef_view_t) -> Self {
        Self {
            base: value.base.into(),
            as_browser_view: value.as_browser_view.into(),
            as_button: value.as_button.into(),
            as_panel: value.as_panel.into(),
            as_scroll_view: value.as_scroll_view.into(),
            as_textfield: value.as_textfield.into(),
            get_type_string: value.get_type_string.into(),
            to_string: value.to_string.into(),
            is_valid: value.is_valid.into(),
            is_attached: value.is_attached.into(),
            is_same: value.is_same.into(),
            get_delegate: value.get_delegate.into(),
            get_window: value.get_window.into(),
            get_id: value.get_id.into(),
            set_id: value.set_id.into(),
            get_group_id: value.get_group_id.into(),
            set_group_id: value.set_group_id.into(),
            get_parent_view: value.get_parent_view.into(),
            get_view_for_id: value.get_view_for_id.into(),
            set_bounds: value.set_bounds.into(),
            get_bounds: value.get_bounds.into(),
            get_bounds_in_screen: value.get_bounds_in_screen.into(),
            set_size: value.set_size.into(),
            get_size: value.get_size.into(),
            set_position: value.set_position.into(),
            get_position: value.get_position.into(),
            set_insets: value.set_insets.into(),
            get_insets: value.get_insets.into(),
            get_preferred_size: value.get_preferred_size.into(),
            size_to_preferred_size: value.size_to_preferred_size.into(),
            get_minimum_size: value.get_minimum_size.into(),
            get_maximum_size: value.get_maximum_size.into(),
            get_height_for_width: value.get_height_for_width.into(),
            invalidate_layout: value.invalidate_layout.into(),
            set_visible: value.set_visible.into(),
            is_visible: value.is_visible.into(),
            is_drawn: value.is_drawn.into(),
            set_enabled: value.set_enabled.into(),
            is_enabled: value.is_enabled.into(),
            set_focusable: value.set_focusable.into(),
            is_focusable: value.is_focusable.into(),
            is_accessibility_focusable: value.is_accessibility_focusable.into(),
            has_focus: value.has_focus.into(),
            request_focus: value.request_focus.into(),
            set_background_color: value.set_background_color.into(),
            get_background_color: value.get_background_color.into(),
            get_theme_color: value.get_theme_color.into(),
            convert_point_to_screen: value.convert_point_to_screen.into(),
            convert_point_from_screen: value.convert_point_from_screen.into(),
            convert_point_to_window: value.convert_point_to_window.into(),
            convert_point_from_window: value.convert_point_from_window.into(),
            convert_point_to_view: value.convert_point_to_view.into(),
            convert_point_from_view: value.convert_point_from_view.into(),
        }
    }
}
impl Into<_cef_view_t> for View {
    fn into(self) -> _cef_view_t {
        _cef_view_t {
            base: self.base.into(),
            as_browser_view: self.as_browser_view.into(),
            as_button: self.as_button.into(),
            as_panel: self.as_panel.into(),
            as_scroll_view: self.as_scroll_view.into(),
            as_textfield: self.as_textfield.into(),
            get_type_string: self.get_type_string.into(),
            to_string: self.to_string.into(),
            is_valid: self.is_valid.into(),
            is_attached: self.is_attached.into(),
            is_same: self.is_same.into(),
            get_delegate: self.get_delegate.into(),
            get_window: self.get_window.into(),
            get_id: self.get_id.into(),
            set_id: self.set_id.into(),
            get_group_id: self.get_group_id.into(),
            set_group_id: self.set_group_id.into(),
            get_parent_view: self.get_parent_view.into(),
            get_view_for_id: self.get_view_for_id.into(),
            set_bounds: self.set_bounds.into(),
            get_bounds: self.get_bounds.into(),
            get_bounds_in_screen: self.get_bounds_in_screen.into(),
            set_size: self.set_size.into(),
            get_size: self.get_size.into(),
            set_position: self.set_position.into(),
            get_position: self.get_position.into(),
            set_insets: self.set_insets.into(),
            get_insets: self.get_insets.into(),
            get_preferred_size: self.get_preferred_size.into(),
            size_to_preferred_size: self.size_to_preferred_size.into(),
            get_minimum_size: self.get_minimum_size.into(),
            get_maximum_size: self.get_maximum_size.into(),
            get_height_for_width: self.get_height_for_width.into(),
            invalidate_layout: self.invalidate_layout.into(),
            set_visible: self.set_visible.into(),
            is_visible: self.is_visible.into(),
            is_drawn: self.is_drawn.into(),
            set_enabled: self.set_enabled.into(),
            is_enabled: self.is_enabled.into(),
            set_focusable: self.set_focusable.into(),
            is_focusable: self.is_focusable.into(),
            is_accessibility_focusable: self.is_accessibility_focusable.into(),
            has_focus: self.has_focus.into(),
            request_focus: self.request_focus.into(),
            set_background_color: self.set_background_color.into(),
            get_background_color: self.get_background_color.into(),
            get_theme_color: self.get_theme_color.into(),
            convert_point_to_screen: self.convert_point_to_screen.into(),
            convert_point_from_screen: self.convert_point_from_screen.into(),
            convert_point_to_window: self.convert_point_to_window.into(),
            convert_point_from_window: self.convert_point_from_window.into(),
            convert_point_to_view: self.convert_point_to_view.into(),
            convert_point_from_view: self.convert_point_from_view.into(),
        }
    }
}
impl Default for View {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_button_t] for more documentation.
#[derive(Clone)]
pub struct Button {
    pub base: View,
    pub as_label_button: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_button_t) -> *mut _cef_label_button_t,
    >,
    pub set_state: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_button_t, state: cef_button_state_t),
    >,
    pub get_state: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_button_t) -> cef_button_state_t,
    >,
    pub set_ink_drop_enabled: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_button_t, enabled: ::std::os::raw::c_int),
    >,
    pub set_tooltip_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_button_t, tooltip_text: *const cef_string_t),
    >,
    pub set_accessible_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_button_t, name: *const cef_string_t),
    >,
}
impl From<_cef_button_t> for Button {
    fn from(value: _cef_button_t) -> Self {
        Self {
            base: value.base.into(),
            as_label_button: value.as_label_button.into(),
            set_state: value.set_state.into(),
            get_state: value.get_state.into(),
            set_ink_drop_enabled: value.set_ink_drop_enabled.into(),
            set_tooltip_text: value.set_tooltip_text.into(),
            set_accessible_name: value.set_accessible_name.into(),
        }
    }
}
impl Into<_cef_button_t> for Button {
    fn into(self) -> _cef_button_t {
        _cef_button_t {
            base: self.base.into(),
            as_label_button: self.as_label_button.into(),
            set_state: self.set_state.into(),
            get_state: self.get_state.into(),
            set_ink_drop_enabled: self.set_ink_drop_enabled.into(),
            set_tooltip_text: self.set_tooltip_text.into(),
            set_accessible_name: self.set_accessible_name.into(),
        }
    }
}
impl Default for Button {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_button_delegate_t] for more documentation.
#[derive(Clone)]
pub struct ButtonDelegate {
    pub base: ViewDelegate,
    pub on_button_pressed: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_button_delegate_t, button: *mut _cef_button_t),
    >,
    pub on_button_state_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_button_delegate_t, button: *mut _cef_button_t),
    >,
}
impl From<_cef_button_delegate_t> for ButtonDelegate {
    fn from(value: _cef_button_delegate_t) -> Self {
        Self {
            base: value.base.into(),
            on_button_pressed: value.on_button_pressed.into(),
            on_button_state_changed: value.on_button_state_changed.into(),
        }
    }
}
impl Into<_cef_button_delegate_t> for ButtonDelegate {
    fn into(self) -> _cef_button_delegate_t {
        _cef_button_delegate_t {
            base: self.base.into(),
            on_button_pressed: self.on_button_pressed.into(),
            on_button_state_changed: self.on_button_state_changed.into(),
        }
    }
}
impl Default for ButtonDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_label_button_t] for more documentation.
#[derive(Clone)]
pub struct LabelButton {
    pub base: Button,
    pub as_menu_button: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_label_button_t) -> *mut _cef_menu_button_t,
    >,
    pub set_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_label_button_t, text: *const cef_string_t),
    >,
    pub get_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_label_button_t) -> cef_string_userfree_t,
    >,
    pub set_image: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_label_button_t,
            button_state: cef_button_state_t,
            image: *mut _cef_image_t,
        ),
    >,
    pub get_image: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_label_button_t,
            button_state: cef_button_state_t,
        ) -> *mut _cef_image_t,
    >,
    pub set_text_color: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_label_button_t,
            for_state: cef_button_state_t,
            color: cef_color_t,
        ),
    >,
    pub set_enabled_text_colors: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_label_button_t, color: cef_color_t),
    >,
    pub set_font_list: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_label_button_t, font_list: *const cef_string_t),
    >,
    pub set_horizontal_alignment: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_label_button_t,
            alignment: cef_horizontal_alignment_t,
        ),
    >,
    pub set_minimum_size: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_label_button_t, size: *const cef_size_t),
    >,
    pub set_maximum_size: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_label_button_t, size: *const cef_size_t),
    >,
}
impl From<_cef_label_button_t> for LabelButton {
    fn from(value: _cef_label_button_t) -> Self {
        Self {
            base: value.base.into(),
            as_menu_button: value.as_menu_button.into(),
            set_text: value.set_text.into(),
            get_text: value.get_text.into(),
            set_image: value.set_image.into(),
            get_image: value.get_image.into(),
            set_text_color: value.set_text_color.into(),
            set_enabled_text_colors: value.set_enabled_text_colors.into(),
            set_font_list: value.set_font_list.into(),
            set_horizontal_alignment: value.set_horizontal_alignment.into(),
            set_minimum_size: value.set_minimum_size.into(),
            set_maximum_size: value.set_maximum_size.into(),
        }
    }
}
impl Into<_cef_label_button_t> for LabelButton {
    fn into(self) -> _cef_label_button_t {
        _cef_label_button_t {
            base: self.base.into(),
            as_menu_button: self.as_menu_button.into(),
            set_text: self.set_text.into(),
            get_text: self.get_text.into(),
            set_image: self.set_image.into(),
            get_image: self.get_image.into(),
            set_text_color: self.set_text_color.into(),
            set_enabled_text_colors: self.set_enabled_text_colors.into(),
            set_font_list: self.set_font_list.into(),
            set_horizontal_alignment: self.set_horizontal_alignment.into(),
            set_minimum_size: self.set_minimum_size.into(),
            set_maximum_size: self.set_maximum_size.into(),
        }
    }
}
impl Default for LabelButton {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_menu_button_pressed_lock_t] for more documentation.
#[derive(Clone)]
pub struct MenuButtonPressedLock(RefGuard<_cef_menu_button_pressed_lock_t>);
pub trait ImplMenuButtonPressedLock: Clone + Sized + Rc {
    fn get_raw(&self) -> *mut _cef_menu_button_pressed_lock_t;
}
impl ImplMenuButtonPressedLock for MenuButtonPressedLock {
    fn get_raw(&self) -> *mut _cef_menu_button_pressed_lock_t {
        unsafe { RefGuard::as_raw(&self.0) }
    }
}
impl Rc for _cef_menu_button_pressed_lock_t {
    fn as_base(&self) -> &_cef_base_ref_counted_t {
        self.base.as_base()
    }
}
impl Rc for MenuButtonPressedLock {
    fn as_base(&self) -> &_cef_base_ref_counted_t {
        self.0.as_base()
    }
}
impl ConvertParam<*mut _cef_menu_button_pressed_lock_t> for &MenuButtonPressedLock {
    fn as_raw(self) -> *mut _cef_menu_button_pressed_lock_t {
        ImplMenuButtonPressedLock::get_raw(self)
    }
}
impl ConvertParam<*mut _cef_menu_button_pressed_lock_t> for &mut MenuButtonPressedLock {
    fn as_raw(self) -> *mut _cef_menu_button_pressed_lock_t {
        ImplMenuButtonPressedLock::get_raw(self)
    }
}
impl ConvertReturnValue<MenuButtonPressedLock> for *mut _cef_menu_button_pressed_lock_t {
    fn as_wrapper(self) -> MenuButtonPressedLock {
        MenuButtonPressedLock(unsafe { RefGuard::from_raw(self) })
    }
}
impl Into<*mut _cef_menu_button_pressed_lock_t> for MenuButtonPressedLock {
    fn into(self) -> *mut _cef_menu_button_pressed_lock_t {
        let object = ImplMenuButtonPressedLock::get_raw(&self);
        std::mem::forget(self);
        object
    }
}
impl Default for MenuButtonPressedLock {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_menu_button_delegate_t] for more documentation.
#[derive(Clone)]
pub struct MenuButtonDelegate {
    pub base: ButtonDelegate,
    pub on_menu_button_pressed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_button_delegate_t,
            menu_button: *mut _cef_menu_button_t,
            screen_point: *const cef_point_t,
            button_pressed_lock: *mut _cef_menu_button_pressed_lock_t,
        ),
    >,
}
impl From<_cef_menu_button_delegate_t> for MenuButtonDelegate {
    fn from(value: _cef_menu_button_delegate_t) -> Self {
        Self {
            base: value.base.into(),
            on_menu_button_pressed: value.on_menu_button_pressed.into(),
        }
    }
}
impl Into<_cef_menu_button_delegate_t> for MenuButtonDelegate {
    fn into(self) -> _cef_menu_button_delegate_t {
        _cef_menu_button_delegate_t {
            base: self.base.into(),
            on_menu_button_pressed: self.on_menu_button_pressed.into(),
        }
    }
}
impl Default for MenuButtonDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_menu_button_t] for more documentation.
#[derive(Clone)]
pub struct MenuButton {
    pub base: LabelButton,
    pub show_menu: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_menu_button_t,
            menu_model: *mut _cef_menu_model_t,
            screen_point: *const cef_point_t,
            anchor_position: cef_menu_anchor_position_t,
        ),
    >,
    pub trigger_menu:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_menu_button_t)>,
}
impl From<_cef_menu_button_t> for MenuButton {
    fn from(value: _cef_menu_button_t) -> Self {
        Self {
            base: value.base.into(),
            show_menu: value.show_menu.into(),
            trigger_menu: value.trigger_menu.into(),
        }
    }
}
impl Into<_cef_menu_button_t> for MenuButton {
    fn into(self) -> _cef_menu_button_t {
        _cef_menu_button_t {
            base: self.base.into(),
            show_menu: self.show_menu.into(),
            trigger_menu: self.trigger_menu.into(),
        }
    }
}
impl Default for MenuButton {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_textfield_delegate_t] for more documentation.
#[derive(Clone)]
pub struct TextfieldDelegate {
    pub base: ViewDelegate,
    pub on_key_event: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_textfield_delegate_t,
            textfield: *mut _cef_textfield_t,
            event: *const cef_key_event_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_after_user_action: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_textfield_delegate_t,
            textfield: *mut _cef_textfield_t,
        ),
    >,
}
impl From<_cef_textfield_delegate_t> for TextfieldDelegate {
    fn from(value: _cef_textfield_delegate_t) -> Self {
        Self {
            base: value.base.into(),
            on_key_event: value.on_key_event.into(),
            on_after_user_action: value.on_after_user_action.into(),
        }
    }
}
impl Into<_cef_textfield_delegate_t> for TextfieldDelegate {
    fn into(self) -> _cef_textfield_delegate_t {
        _cef_textfield_delegate_t {
            base: self.base.into(),
            on_key_event: self.on_key_event.into(),
            on_after_user_action: self.on_after_user_action.into(),
        }
    }
}
impl Default for TextfieldDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_textfield_t] for more documentation.
#[derive(Clone)]
pub struct Textfield {
    pub base: View,
    pub set_password_input: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_textfield_t,
            password_input: ::std::os::raw::c_int,
        ),
    >,
    pub is_password_input: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> ::std::os::raw::c_int,
    >,
    pub set_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, read_only: ::std::os::raw::c_int),
    >,
    pub is_read_only: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> ::std::os::raw::c_int,
    >,
    pub get_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> cef_string_userfree_t,
    >,
    pub set_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, text: *const cef_string_t),
    >,
    pub append_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, text: *const cef_string_t),
    >,
    pub insert_or_replace_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, text: *const cef_string_t),
    >,
    pub has_selection: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> ::std::os::raw::c_int,
    >,
    pub get_selected_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> cef_string_userfree_t,
    >,
    pub select_all: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, reversed: ::std::os::raw::c_int),
    >,
    pub clear_selection:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t)>,
    pub get_selected_range: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> cef_range_t,
    >,
    pub select_range: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, range: *const cef_range_t),
    >,
    pub get_cursor_position:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> usize>,
    pub set_text_color: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, color: cef_color_t),
    >,
    pub get_text_color: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> cef_color_t,
    >,
    pub set_selection_text_color: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, color: cef_color_t),
    >,
    pub get_selection_text_color: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> cef_color_t,
    >,
    pub set_selection_background_color: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, color: cef_color_t),
    >,
    pub get_selection_background_color: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> cef_color_t,
    >,
    pub set_font_list: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, font_list: *const cef_string_t),
    >,
    pub apply_text_color: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_textfield_t,
            color: cef_color_t,
            range: *const cef_range_t,
        ),
    >,
    pub apply_text_style: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_textfield_t,
            style: cef_text_style_t,
            add: ::std::os::raw::c_int,
            range: *const cef_range_t,
        ),
    >,
    pub is_command_enabled: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_textfield_t,
            command_id: cef_text_field_commands_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub execute_command: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_textfield_t,
            command_id: cef_text_field_commands_t,
        ),
    >,
    pub clear_edit_history:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t)>,
    pub set_placeholder_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, text: *const cef_string_t),
    >,
    pub get_placeholder_text: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t) -> cef_string_userfree_t,
    >,
    pub set_placeholder_text_color: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, color: cef_color_t),
    >,
    pub set_accessible_name: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_textfield_t, name: *const cef_string_t),
    >,
}
impl From<_cef_textfield_t> for Textfield {
    fn from(value: _cef_textfield_t) -> Self {
        Self {
            base: value.base.into(),
            set_password_input: value.set_password_input.into(),
            is_password_input: value.is_password_input.into(),
            set_read_only: value.set_read_only.into(),
            is_read_only: value.is_read_only.into(),
            get_text: value.get_text.into(),
            set_text: value.set_text.into(),
            append_text: value.append_text.into(),
            insert_or_replace_text: value.insert_or_replace_text.into(),
            has_selection: value.has_selection.into(),
            get_selected_text: value.get_selected_text.into(),
            select_all: value.select_all.into(),
            clear_selection: value.clear_selection.into(),
            get_selected_range: value.get_selected_range.into(),
            select_range: value.select_range.into(),
            get_cursor_position: value.get_cursor_position.into(),
            set_text_color: value.set_text_color.into(),
            get_text_color: value.get_text_color.into(),
            set_selection_text_color: value.set_selection_text_color.into(),
            get_selection_text_color: value.get_selection_text_color.into(),
            set_selection_background_color: value.set_selection_background_color.into(),
            get_selection_background_color: value.get_selection_background_color.into(),
            set_font_list: value.set_font_list.into(),
            apply_text_color: value.apply_text_color.into(),
            apply_text_style: value.apply_text_style.into(),
            is_command_enabled: value.is_command_enabled.into(),
            execute_command: value.execute_command.into(),
            clear_edit_history: value.clear_edit_history.into(),
            set_placeholder_text: value.set_placeholder_text.into(),
            get_placeholder_text: value.get_placeholder_text.into(),
            set_placeholder_text_color: value.set_placeholder_text_color.into(),
            set_accessible_name: value.set_accessible_name.into(),
        }
    }
}
impl Into<_cef_textfield_t> for Textfield {
    fn into(self) -> _cef_textfield_t {
        _cef_textfield_t {
            base: self.base.into(),
            set_password_input: self.set_password_input.into(),
            is_password_input: self.is_password_input.into(),
            set_read_only: self.set_read_only.into(),
            is_read_only: self.is_read_only.into(),
            get_text: self.get_text.into(),
            set_text: self.set_text.into(),
            append_text: self.append_text.into(),
            insert_or_replace_text: self.insert_or_replace_text.into(),
            has_selection: self.has_selection.into(),
            get_selected_text: self.get_selected_text.into(),
            select_all: self.select_all.into(),
            clear_selection: self.clear_selection.into(),
            get_selected_range: self.get_selected_range.into(),
            select_range: self.select_range.into(),
            get_cursor_position: self.get_cursor_position.into(),
            set_text_color: self.set_text_color.into(),
            get_text_color: self.get_text_color.into(),
            set_selection_text_color: self.set_selection_text_color.into(),
            get_selection_text_color: self.get_selection_text_color.into(),
            set_selection_background_color: self.set_selection_background_color.into(),
            get_selection_background_color: self.get_selection_background_color.into(),
            set_font_list: self.set_font_list.into(),
            apply_text_color: self.apply_text_color.into(),
            apply_text_style: self.apply_text_style.into(),
            is_command_enabled: self.is_command_enabled.into(),
            execute_command: self.execute_command.into(),
            clear_edit_history: self.clear_edit_history.into(),
            set_placeholder_text: self.set_placeholder_text.into(),
            get_placeholder_text: self.get_placeholder_text.into(),
            set_placeholder_text_color: self.set_placeholder_text_color.into(),
            set_accessible_name: self.set_accessible_name.into(),
        }
    }
}
impl Default for Textfield {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_browser_view_delegate_t] for more documentation.
#[derive(Clone)]
pub struct BrowserViewDelegate {
    pub base: ViewDelegate,
    pub on_browser_created: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_view_delegate_t,
            browser_view: *mut _cef_browser_view_t,
            browser: *mut _cef_browser_t,
        ),
    >,
    pub on_browser_destroyed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_view_delegate_t,
            browser_view: *mut _cef_browser_view_t,
            browser: *mut _cef_browser_t,
        ),
    >,
    pub get_delegate_for_popup_browser_view: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_view_delegate_t,
            browser_view: *mut _cef_browser_view_t,
            settings: *const _cef_browser_settings_t,
            client: *mut _cef_client_t,
            is_devtools: ::std::os::raw::c_int,
        ) -> *mut _cef_browser_view_delegate_t,
    >,
    pub on_popup_browser_view_created: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_view_delegate_t,
            browser_view: *mut _cef_browser_view_t,
            popup_browser_view: *mut _cef_browser_view_t,
            is_devtools: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_chrome_toolbar_type: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_view_delegate_t,
            browser_view: *mut _cef_browser_view_t,
        ) -> cef_chrome_toolbar_type_t,
    >,
    pub use_frameless_window_for_picture_in_picture: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_view_delegate_t,
            browser_view: *mut _cef_browser_view_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_gesture_command: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_view_delegate_t,
            browser_view: *mut _cef_browser_view_t,
            gesture_command: cef_gesture_command_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_browser_runtime_style: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_view_delegate_t) -> cef_runtime_style_t,
    >,
}
impl From<_cef_browser_view_delegate_t> for BrowserViewDelegate {
    fn from(value: _cef_browser_view_delegate_t) -> Self {
        Self {
            base: value.base.into(),
            on_browser_created: value.on_browser_created.into(),
            on_browser_destroyed: value.on_browser_destroyed.into(),
            get_delegate_for_popup_browser_view: value.get_delegate_for_popup_browser_view.into(),
            on_popup_browser_view_created: value.on_popup_browser_view_created.into(),
            get_chrome_toolbar_type: value.get_chrome_toolbar_type.into(),
            use_frameless_window_for_picture_in_picture: value
                .use_frameless_window_for_picture_in_picture
                .into(),
            on_gesture_command: value.on_gesture_command.into(),
            get_browser_runtime_style: value.get_browser_runtime_style.into(),
        }
    }
}
impl Into<_cef_browser_view_delegate_t> for BrowserViewDelegate {
    fn into(self) -> _cef_browser_view_delegate_t {
        _cef_browser_view_delegate_t {
            base: self.base.into(),
            on_browser_created: self.on_browser_created.into(),
            on_browser_destroyed: self.on_browser_destroyed.into(),
            get_delegate_for_popup_browser_view: self.get_delegate_for_popup_browser_view.into(),
            on_popup_browser_view_created: self.on_popup_browser_view_created.into(),
            get_chrome_toolbar_type: self.get_chrome_toolbar_type.into(),
            use_frameless_window_for_picture_in_picture: self
                .use_frameless_window_for_picture_in_picture
                .into(),
            on_gesture_command: self.on_gesture_command.into(),
            get_browser_runtime_style: self.get_browser_runtime_style.into(),
        }
    }
}
impl Default for BrowserViewDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_browser_view_t] for more documentation.
#[derive(Clone)]
pub struct BrowserView {
    pub base: View,
    pub get_browser: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_view_t) -> *mut _cef_browser_t,
    >,
    pub get_chrome_toolbar: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_view_t) -> *mut _cef_view_t,
    >,
    pub set_prefer_accelerators: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_view_t,
            prefer_accelerators: ::std::os::raw::c_int,
        ),
    >,
    pub get_runtime_style: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_browser_view_t) -> cef_runtime_style_t,
    >,
}
impl From<_cef_browser_view_t> for BrowserView {
    fn from(value: _cef_browser_view_t) -> Self {
        Self {
            base: value.base.into(),
            get_browser: value.get_browser.into(),
            get_chrome_toolbar: value.get_chrome_toolbar.into(),
            set_prefer_accelerators: value.set_prefer_accelerators.into(),
            get_runtime_style: value.get_runtime_style.into(),
        }
    }
}
impl Into<_cef_browser_view_t> for BrowserView {
    fn into(self) -> _cef_browser_view_t {
        _cef_browser_view_t {
            base: self.base.into(),
            get_browser: self.get_browser.into(),
            get_chrome_toolbar: self.get_chrome_toolbar.into(),
            set_prefer_accelerators: self.set_prefer_accelerators.into(),
            get_runtime_style: self.get_runtime_style.into(),
        }
    }
}
impl Default for BrowserView {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_scroll_view_t] for more documentation.
#[derive(Clone)]
pub struct ScrollView {
    pub base: View,
    pub set_content_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_scroll_view_t, view: *mut _cef_view_t),
    >,
    pub get_content_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_scroll_view_t) -> *mut _cef_view_t,
    >,
    pub get_visible_content_rect: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_scroll_view_t) -> cef_rect_t,
    >,
    pub has_horizontal_scrollbar: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_scroll_view_t) -> ::std::os::raw::c_int,
    >,
    pub get_horizontal_scrollbar_height: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_scroll_view_t) -> ::std::os::raw::c_int,
    >,
    pub has_vertical_scrollbar: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_scroll_view_t) -> ::std::os::raw::c_int,
    >,
    pub get_vertical_scrollbar_width: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_scroll_view_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_scroll_view_t> for ScrollView {
    fn from(value: _cef_scroll_view_t) -> Self {
        Self {
            base: value.base.into(),
            set_content_view: value.set_content_view.into(),
            get_content_view: value.get_content_view.into(),
            get_visible_content_rect: value.get_visible_content_rect.into(),
            has_horizontal_scrollbar: value.has_horizontal_scrollbar.into(),
            get_horizontal_scrollbar_height: value.get_horizontal_scrollbar_height.into(),
            has_vertical_scrollbar: value.has_vertical_scrollbar.into(),
            get_vertical_scrollbar_width: value.get_vertical_scrollbar_width.into(),
        }
    }
}
impl Into<_cef_scroll_view_t> for ScrollView {
    fn into(self) -> _cef_scroll_view_t {
        _cef_scroll_view_t {
            base: self.base.into(),
            set_content_view: self.set_content_view.into(),
            get_content_view: self.get_content_view.into(),
            get_visible_content_rect: self.get_visible_content_rect.into(),
            has_horizontal_scrollbar: self.has_horizontal_scrollbar.into(),
            get_horizontal_scrollbar_height: self.get_horizontal_scrollbar_height.into(),
            has_vertical_scrollbar: self.has_vertical_scrollbar.into(),
            get_vertical_scrollbar_width: self.get_vertical_scrollbar_width.into(),
        }
    }
}
impl Default for ScrollView {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_display_t] for more documentation.
#[derive(Clone)]
pub struct Display {
    pub base: BaseRefCounted,
    pub get_id:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_display_t) -> i64>,
    pub get_device_scale_factor:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_display_t) -> f32>,
    pub convert_point_to_pixels: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_display_t, point: *mut cef_point_t),
    >,
    pub convert_point_from_pixels: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_display_t, point: *mut cef_point_t),
    >,
    pub get_bounds:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_display_t) -> cef_rect_t>,
    pub get_work_area:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_display_t) -> cef_rect_t>,
    pub get_rotation: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_display_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_display_t> for Display {
    fn from(value: _cef_display_t) -> Self {
        Self {
            base: value.base.into(),
            get_id: value.get_id.into(),
            get_device_scale_factor: value.get_device_scale_factor.into(),
            convert_point_to_pixels: value.convert_point_to_pixels.into(),
            convert_point_from_pixels: value.convert_point_from_pixels.into(),
            get_bounds: value.get_bounds.into(),
            get_work_area: value.get_work_area.into(),
            get_rotation: value.get_rotation.into(),
        }
    }
}
impl Into<_cef_display_t> for Display {
    fn into(self) -> _cef_display_t {
        _cef_display_t {
            base: self.base.into(),
            get_id: self.get_id.into(),
            get_device_scale_factor: self.get_device_scale_factor.into(),
            convert_point_to_pixels: self.convert_point_to_pixels.into(),
            convert_point_from_pixels: self.convert_point_from_pixels.into(),
            get_bounds: self.get_bounds.into(),
            get_work_area: self.get_work_area.into(),
            get_rotation: self.get_rotation.into(),
        }
    }
}
impl Default for Display {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_overlay_controller_t] for more documentation.
#[derive(Clone)]
pub struct OverlayController {
    pub base: BaseRefCounted,
    pub is_valid: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> ::std::os::raw::c_int,
    >,
    pub is_same: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_overlay_controller_t,
            that: *mut _cef_overlay_controller_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_contents_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> *mut _cef_view_t,
    >,
    pub get_window: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> *mut _cef_window_t,
    >,
    pub get_docking_mode: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> cef_docking_mode_t,
    >,
    pub destroy:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t)>,
    pub set_bounds: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_overlay_controller_t,
            bounds: *const cef_rect_t,
        ),
    >,
    pub get_bounds: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> cef_rect_t,
    >,
    pub get_bounds_in_screen: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> cef_rect_t,
    >,
    pub set_size: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t, size: *const cef_size_t),
    >,
    pub get_size: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> cef_size_t,
    >,
    pub set_position: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_overlay_controller_t,
            position: *const cef_point_t,
        ),
    >,
    pub get_position: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> cef_point_t,
    >,
    pub set_insets: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_overlay_controller_t,
            insets: *const cef_insets_t,
        ),
    >,
    pub get_insets: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> cef_insets_t,
    >,
    pub size_to_preferred_size:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t)>,
    pub set_visible: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_overlay_controller_t,
            visible: ::std::os::raw::c_int,
        ),
    >,
    pub is_visible: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> ::std::os::raw::c_int,
    >,
    pub is_drawn: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_overlay_controller_t) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_overlay_controller_t> for OverlayController {
    fn from(value: _cef_overlay_controller_t) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid.into(),
            is_same: value.is_same.into(),
            get_contents_view: value.get_contents_view.into(),
            get_window: value.get_window.into(),
            get_docking_mode: value.get_docking_mode.into(),
            destroy: value.destroy.into(),
            set_bounds: value.set_bounds.into(),
            get_bounds: value.get_bounds.into(),
            get_bounds_in_screen: value.get_bounds_in_screen.into(),
            set_size: value.set_size.into(),
            get_size: value.get_size.into(),
            set_position: value.set_position.into(),
            get_position: value.get_position.into(),
            set_insets: value.set_insets.into(),
            get_insets: value.get_insets.into(),
            size_to_preferred_size: value.size_to_preferred_size.into(),
            set_visible: value.set_visible.into(),
            is_visible: value.is_visible.into(),
            is_drawn: value.is_drawn.into(),
        }
    }
}
impl Into<_cef_overlay_controller_t> for OverlayController {
    fn into(self) -> _cef_overlay_controller_t {
        _cef_overlay_controller_t {
            base: self.base.into(),
            is_valid: self.is_valid.into(),
            is_same: self.is_same.into(),
            get_contents_view: self.get_contents_view.into(),
            get_window: self.get_window.into(),
            get_docking_mode: self.get_docking_mode.into(),
            destroy: self.destroy.into(),
            set_bounds: self.set_bounds.into(),
            get_bounds: self.get_bounds.into(),
            get_bounds_in_screen: self.get_bounds_in_screen.into(),
            set_size: self.set_size.into(),
            get_size: self.get_size.into(),
            set_position: self.set_position.into(),
            get_position: self.get_position.into(),
            set_insets: self.set_insets.into(),
            get_insets: self.get_insets.into(),
            size_to_preferred_size: self.size_to_preferred_size.into(),
            set_visible: self.set_visible.into(),
            is_visible: self.is_visible.into(),
            is_drawn: self.is_drawn.into(),
        }
    }
}
impl Default for OverlayController {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_panel_delegate_t] for more documentation.
#[derive(Clone)]
pub struct PanelDelegate {
    pub base: ViewDelegate,
}
impl From<_cef_panel_delegate_t> for PanelDelegate {
    fn from(value: _cef_panel_delegate_t) -> Self {
        Self {
            base: value.base.into(),
        }
    }
}
impl Into<_cef_panel_delegate_t> for PanelDelegate {
    fn into(self) -> _cef_panel_delegate_t {
        _cef_panel_delegate_t {
            base: self.base.into(),
        }
    }
}
impl Default for PanelDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_panel_t] for more documentation.
#[derive(Clone)]
pub struct Panel {
    pub base: View,
    pub as_window: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_panel_t) -> *mut _cef_window_t,
    >,
    pub set_to_fill_layout: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_panel_t) -> *mut _cef_fill_layout_t,
    >,
    pub set_to_box_layout: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_panel_t,
            settings: *const cef_box_layout_settings_t,
        ) -> *mut _cef_box_layout_t,
    >,
    pub get_layout: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_panel_t) -> *mut _cef_layout_t,
    >,
    pub layout: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_panel_t)>,
    pub add_child_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_panel_t, view: *mut _cef_view_t),
    >,
    pub add_child_view_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_panel_t,
            view: *mut _cef_view_t,
            index: ::std::os::raw::c_int,
        ),
    >,
    pub reorder_child_view: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_panel_t,
            view: *mut _cef_view_t,
            index: ::std::os::raw::c_int,
        ),
    >,
    pub remove_child_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_panel_t, view: *mut _cef_view_t),
    >,
    pub remove_all_child_views:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_panel_t)>,
    pub get_child_view_count:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_panel_t) -> usize>,
    pub get_child_view_at: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_panel_t,
            index: ::std::os::raw::c_int,
        ) -> *mut _cef_view_t,
    >,
}
impl From<_cef_panel_t> for Panel {
    fn from(value: _cef_panel_t) -> Self {
        Self {
            base: value.base.into(),
            as_window: value.as_window.into(),
            set_to_fill_layout: value.set_to_fill_layout.into(),
            set_to_box_layout: value.set_to_box_layout.into(),
            get_layout: value.get_layout.into(),
            layout: value.layout.into(),
            add_child_view: value.add_child_view.into(),
            add_child_view_at: value.add_child_view_at.into(),
            reorder_child_view: value.reorder_child_view.into(),
            remove_child_view: value.remove_child_view.into(),
            remove_all_child_views: value.remove_all_child_views.into(),
            get_child_view_count: value.get_child_view_count.into(),
            get_child_view_at: value.get_child_view_at.into(),
        }
    }
}
impl Into<_cef_panel_t> for Panel {
    fn into(self) -> _cef_panel_t {
        _cef_panel_t {
            base: self.base.into(),
            as_window: self.as_window.into(),
            set_to_fill_layout: self.set_to_fill_layout.into(),
            set_to_box_layout: self.set_to_box_layout.into(),
            get_layout: self.get_layout.into(),
            layout: self.layout.into(),
            add_child_view: self.add_child_view.into(),
            add_child_view_at: self.add_child_view_at.into(),
            reorder_child_view: self.reorder_child_view.into(),
            remove_child_view: self.remove_child_view.into(),
            remove_all_child_views: self.remove_all_child_views.into(),
            get_child_view_count: self.get_child_view_count.into(),
            get_child_view_at: self.get_child_view_at.into(),
        }
    }
}
impl Default for Panel {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_window_delegate_t] for more documentation.
#[derive(Clone)]
pub struct WindowDelegate {
    pub base: PanelDelegate,
    pub on_window_created: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_delegate_t, window: *mut _cef_window_t),
    >,
    pub on_window_closing: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_delegate_t, window: *mut _cef_window_t),
    >,
    pub on_window_destroyed: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_delegate_t, window: *mut _cef_window_t),
    >,
    pub on_window_activation_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
            active: ::std::os::raw::c_int,
        ),
    >,
    pub on_window_bounds_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
            new_bounds: *const cef_rect_t,
        ),
    >,
    pub on_window_fullscreen_transition: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
            is_completed: ::std::os::raw::c_int,
        ),
    >,
    pub get_parent_window: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
            is_menu: *mut ::std::os::raw::c_int,
            can_activate_menu: *mut ::std::os::raw::c_int,
        ) -> *mut _cef_window_t,
    >,
    pub is_window_modal_dialog: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_initial_bounds: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
        ) -> cef_rect_t,
    >,
    pub get_initial_show_state: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
        ) -> cef_show_state_t,
    >,
    pub is_frameless: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub with_standard_window_buttons: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_titlebar_height: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
            titlebar_height: *mut f32,
        ) -> ::std::os::raw::c_int,
    >,
    pub accepts_first_mouse: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
        ) -> cef_state_t,
    >,
    pub can_resize: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub can_maximize: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub can_minimize: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub can_close: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_accelerator: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_key_event: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
            event: *const cef_key_event_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub on_theme_colors_changed: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
            chrome_theme: ::std::os::raw::c_int,
        ),
    >,
    pub get_window_runtime_style: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_delegate_t) -> cef_runtime_style_t,
    >,
    pub get_linux_window_properties: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_delegate_t,
            window: *mut _cef_window_t,
            properties: *mut _cef_linux_window_properties_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_window_delegate_t> for WindowDelegate {
    fn from(value: _cef_window_delegate_t) -> Self {
        Self {
            base: value.base.into(),
            on_window_created: value.on_window_created.into(),
            on_window_closing: value.on_window_closing.into(),
            on_window_destroyed: value.on_window_destroyed.into(),
            on_window_activation_changed: value.on_window_activation_changed.into(),
            on_window_bounds_changed: value.on_window_bounds_changed.into(),
            on_window_fullscreen_transition: value.on_window_fullscreen_transition.into(),
            get_parent_window: value.get_parent_window.into(),
            is_window_modal_dialog: value.is_window_modal_dialog.into(),
            get_initial_bounds: value.get_initial_bounds.into(),
            get_initial_show_state: value.get_initial_show_state.into(),
            is_frameless: value.is_frameless.into(),
            with_standard_window_buttons: value.with_standard_window_buttons.into(),
            get_titlebar_height: value.get_titlebar_height.into(),
            accepts_first_mouse: value.accepts_first_mouse.into(),
            can_resize: value.can_resize.into(),
            can_maximize: value.can_maximize.into(),
            can_minimize: value.can_minimize.into(),
            can_close: value.can_close.into(),
            on_accelerator: value.on_accelerator.into(),
            on_key_event: value.on_key_event.into(),
            on_theme_colors_changed: value.on_theme_colors_changed.into(),
            get_window_runtime_style: value.get_window_runtime_style.into(),
            get_linux_window_properties: value.get_linux_window_properties.into(),
        }
    }
}
impl Into<_cef_window_delegate_t> for WindowDelegate {
    fn into(self) -> _cef_window_delegate_t {
        _cef_window_delegate_t {
            base: self.base.into(),
            on_window_created: self.on_window_created.into(),
            on_window_closing: self.on_window_closing.into(),
            on_window_destroyed: self.on_window_destroyed.into(),
            on_window_activation_changed: self.on_window_activation_changed.into(),
            on_window_bounds_changed: self.on_window_bounds_changed.into(),
            on_window_fullscreen_transition: self.on_window_fullscreen_transition.into(),
            get_parent_window: self.get_parent_window.into(),
            is_window_modal_dialog: self.is_window_modal_dialog.into(),
            get_initial_bounds: self.get_initial_bounds.into(),
            get_initial_show_state: self.get_initial_show_state.into(),
            is_frameless: self.is_frameless.into(),
            with_standard_window_buttons: self.with_standard_window_buttons.into(),
            get_titlebar_height: self.get_titlebar_height.into(),
            accepts_first_mouse: self.accepts_first_mouse.into(),
            can_resize: self.can_resize.into(),
            can_maximize: self.can_maximize.into(),
            can_minimize: self.can_minimize.into(),
            can_close: self.can_close.into(),
            on_accelerator: self.on_accelerator.into(),
            on_key_event: self.on_key_event.into(),
            on_theme_colors_changed: self.on_theme_colors_changed.into(),
            get_window_runtime_style: self.get_window_runtime_style.into(),
            get_linux_window_properties: self.get_linux_window_properties.into(),
        }
    }
}
impl Default for WindowDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [_cef_window_t] for more documentation.
#[derive(Clone)]
pub struct Window {
    pub base: Panel,
    pub show: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub show_as_browser_modal_dialog: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_t,
            browser_view: *mut _cef_browser_view_t,
        ),
    >,
    pub hide: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub center_window: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t, size: *const cef_size_t),
    >,
    pub close: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub is_closed: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> ::std::os::raw::c_int,
    >,
    pub activate: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub deactivate: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub is_active: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> ::std::os::raw::c_int,
    >,
    pub bring_to_top: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub set_always_on_top: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t, on_top: ::std::os::raw::c_int),
    >,
    pub is_always_on_top: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> ::std::os::raw::c_int,
    >,
    pub maximize: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub minimize: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub restore: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub set_fullscreen: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t, fullscreen: ::std::os::raw::c_int),
    >,
    pub is_maximized: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> ::std::os::raw::c_int,
    >,
    pub is_minimized: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> ::std::os::raw::c_int,
    >,
    pub is_fullscreen: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> ::std::os::raw::c_int,
    >,
    pub get_focused_view: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> *mut _cef_view_t,
    >,
    pub set_title: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t, title: *const cef_string_t),
    >,
    pub get_title: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> cef_string_userfree_t,
    >,
    pub set_window_icon: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t, image: *mut _cef_image_t),
    >,
    pub get_window_icon: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> *mut _cef_image_t,
    >,
    pub set_window_app_icon: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t, image: *mut _cef_image_t),
    >,
    pub get_window_app_icon: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> *mut _cef_image_t,
    >,
    pub add_overlay_view: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_t,
            view: *mut _cef_view_t,
            docking_mode: cef_docking_mode_t,
            can_activate: ::std::os::raw::c_int,
        ) -> *mut _cef_overlay_controller_t,
    >,
    pub show_menu: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_t,
            menu_model: *mut _cef_menu_model_t,
            screen_point: *const cef_point_t,
            anchor_position: cef_menu_anchor_position_t,
        ),
    >,
    pub cancel_menu: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub get_display: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> *mut _cef_display_t,
    >,
    pub get_client_area_bounds_in_screen:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> cef_rect_t>,
    pub set_draggable_regions: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_t,
            regionsCount: usize,
            regions: *const cef_draggable_region_t,
        ),
    >,
    pub get_window_handle: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> cef_window_handle_t,
    >,
    pub send_key_press: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_t,
            key_code: ::std::os::raw::c_int,
            event_flags: u32,
        ),
    >,
    pub send_mouse_move: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_t,
            screen_x: ::std::os::raw::c_int,
            screen_y: ::std::os::raw::c_int,
        ),
    >,
    pub send_mouse_events: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_t,
            button: cef_mouse_button_type_t,
            mouse_down: ::std::os::raw::c_int,
            mouse_up: ::std::os::raw::c_int,
        ),
    >,
    pub set_accelerator: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_t,
            command_id: ::std::os::raw::c_int,
            key_code: ::std::os::raw::c_int,
            shift_pressed: ::std::os::raw::c_int,
            ctrl_pressed: ::std::os::raw::c_int,
            alt_pressed: ::std::os::raw::c_int,
            high_priority: ::std::os::raw::c_int,
        ),
    >,
    pub remove_accelerator: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t, command_id: ::std::os::raw::c_int),
    >,
    pub remove_all_accelerators:
        ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub set_theme_color: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_window_t,
            color_id: ::std::os::raw::c_int,
            color: cef_color_t,
        ),
    >,
    pub theme_changed: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_window_t)>,
    pub get_runtime_style: ::std::option::Option<
        unsafe extern "stdcall" fn(self_: *mut _cef_window_t) -> cef_runtime_style_t,
    >,
}
impl From<_cef_window_t> for Window {
    fn from(value: _cef_window_t) -> Self {
        Self {
            base: value.base.into(),
            show: value.show.into(),
            show_as_browser_modal_dialog: value.show_as_browser_modal_dialog.into(),
            hide: value.hide.into(),
            center_window: value.center_window.into(),
            close: value.close.into(),
            is_closed: value.is_closed.into(),
            activate: value.activate.into(),
            deactivate: value.deactivate.into(),
            is_active: value.is_active.into(),
            bring_to_top: value.bring_to_top.into(),
            set_always_on_top: value.set_always_on_top.into(),
            is_always_on_top: value.is_always_on_top.into(),
            maximize: value.maximize.into(),
            minimize: value.minimize.into(),
            restore: value.restore.into(),
            set_fullscreen: value.set_fullscreen.into(),
            is_maximized: value.is_maximized.into(),
            is_minimized: value.is_minimized.into(),
            is_fullscreen: value.is_fullscreen.into(),
            get_focused_view: value.get_focused_view.into(),
            set_title: value.set_title.into(),
            get_title: value.get_title.into(),
            set_window_icon: value.set_window_icon.into(),
            get_window_icon: value.get_window_icon.into(),
            set_window_app_icon: value.set_window_app_icon.into(),
            get_window_app_icon: value.get_window_app_icon.into(),
            add_overlay_view: value.add_overlay_view.into(),
            show_menu: value.show_menu.into(),
            cancel_menu: value.cancel_menu.into(),
            get_display: value.get_display.into(),
            get_client_area_bounds_in_screen: value.get_client_area_bounds_in_screen.into(),
            set_draggable_regions: value.set_draggable_regions.into(),
            get_window_handle: value.get_window_handle.into(),
            send_key_press: value.send_key_press.into(),
            send_mouse_move: value.send_mouse_move.into(),
            send_mouse_events: value.send_mouse_events.into(),
            set_accelerator: value.set_accelerator.into(),
            remove_accelerator: value.remove_accelerator.into(),
            remove_all_accelerators: value.remove_all_accelerators.into(),
            set_theme_color: value.set_theme_color.into(),
            theme_changed: value.theme_changed.into(),
            get_runtime_style: value.get_runtime_style.into(),
        }
    }
}
impl Into<_cef_window_t> for Window {
    fn into(self) -> _cef_window_t {
        _cef_window_t {
            base: self.base.into(),
            show: self.show.into(),
            show_as_browser_modal_dialog: self.show_as_browser_modal_dialog.into(),
            hide: self.hide.into(),
            center_window: self.center_window.into(),
            close: self.close.into(),
            is_closed: self.is_closed.into(),
            activate: self.activate.into(),
            deactivate: self.deactivate.into(),
            is_active: self.is_active.into(),
            bring_to_top: self.bring_to_top.into(),
            set_always_on_top: self.set_always_on_top.into(),
            is_always_on_top: self.is_always_on_top.into(),
            maximize: self.maximize.into(),
            minimize: self.minimize.into(),
            restore: self.restore.into(),
            set_fullscreen: self.set_fullscreen.into(),
            is_maximized: self.is_maximized.into(),
            is_minimized: self.is_minimized.into(),
            is_fullscreen: self.is_fullscreen.into(),
            get_focused_view: self.get_focused_view.into(),
            set_title: self.set_title.into(),
            get_title: self.get_title.into(),
            set_window_icon: self.set_window_icon.into(),
            get_window_icon: self.get_window_icon.into(),
            set_window_app_icon: self.set_window_app_icon.into(),
            get_window_app_icon: self.get_window_app_icon.into(),
            add_overlay_view: self.add_overlay_view.into(),
            show_menu: self.show_menu.into(),
            cancel_menu: self.cancel_menu.into(),
            get_display: self.get_display.into(),
            get_client_area_bounds_in_screen: self.get_client_area_bounds_in_screen.into(),
            set_draggable_regions: self.set_draggable_regions.into(),
            get_window_handle: self.get_window_handle.into(),
            send_key_press: self.send_key_press.into(),
            send_mouse_move: self.send_mouse_move.into(),
            send_mouse_events: self.send_mouse_events.into(),
            set_accelerator: self.set_accelerator.into(),
            remove_accelerator: self.remove_accelerator.into(),
            remove_all_accelerators: self.remove_all_accelerators.into(),
            set_theme_color: self.set_theme_color.into(),
            theme_changed: self.theme_changed.into(),
            get_runtime_style: self.get_runtime_style.into(),
        }
    }
}
impl Default for Window {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_content_setting_types_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContentSettingTypes(cef_content_setting_types_t);
impl AsRef<cef_content_setting_types_t> for ContentSettingTypes {
    fn as_ref(&self) -> &cef_content_setting_types_t {
        &self.0
    }
}
impl AsMut<cef_content_setting_types_t> for ContentSettingTypes {
    fn as_mut(&mut self) -> &mut cef_content_setting_types_t {
        &mut self.0
    }
}
impl From<cef_content_setting_types_t> for ContentSettingTypes {
    fn from(value: cef_content_setting_types_t) -> Self {
        Self(value)
    }
}
impl Into<cef_content_setting_types_t> for ContentSettingTypes {
    fn into(self) -> cef_content_setting_types_t {
        self.0
    }
}
impl Default for ContentSettingTypes {
    fn default() -> Self {
        Self(cef_content_setting_types_t::CEF_CONTENT_SETTING_TYPE_COOKIES)
    }
}

/// See [cef_content_setting_values_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContentSettingValues(cef_content_setting_values_t);
impl AsRef<cef_content_setting_values_t> for ContentSettingValues {
    fn as_ref(&self) -> &cef_content_setting_values_t {
        &self.0
    }
}
impl AsMut<cef_content_setting_values_t> for ContentSettingValues {
    fn as_mut(&mut self) -> &mut cef_content_setting_values_t {
        &mut self.0
    }
}
impl From<cef_content_setting_values_t> for ContentSettingValues {
    fn from(value: cef_content_setting_values_t) -> Self {
        Self(value)
    }
}
impl Into<cef_content_setting_values_t> for ContentSettingValues {
    fn into(self) -> cef_content_setting_values_t {
        self.0
    }
}
impl Default for ContentSettingValues {
    fn default() -> Self {
        Self(cef_content_setting_values_t::CEF_CONTENT_SETTING_VALUE_DEFAULT)
    }
}

/// See [cef_color_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ColorType(cef_color_type_t);
impl AsRef<cef_color_type_t> for ColorType {
    fn as_ref(&self) -> &cef_color_type_t {
        &self.0
    }
}
impl AsMut<cef_color_type_t> for ColorType {
    fn as_mut(&mut self) -> &mut cef_color_type_t {
        &mut self.0
    }
}
impl From<cef_color_type_t> for ColorType {
    fn from(value: cef_color_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_color_type_t> for ColorType {
    fn into(self) -> cef_color_type_t {
        self.0
    }
}
impl Default for ColorType {
    fn default() -> Self {
        Self(cef_color_type_t::CEF_COLOR_TYPE_RGBA_8888)
    }
}

/// See [cef_runtime_style_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct RuntimeStyle(cef_runtime_style_t);
impl AsRef<cef_runtime_style_t> for RuntimeStyle {
    fn as_ref(&self) -> &cef_runtime_style_t {
        &self.0
    }
}
impl AsMut<cef_runtime_style_t> for RuntimeStyle {
    fn as_mut(&mut self) -> &mut cef_runtime_style_t {
        &mut self.0
    }
}
impl From<cef_runtime_style_t> for RuntimeStyle {
    fn from(value: cef_runtime_style_t) -> Self {
        Self(value)
    }
}
impl Into<cef_runtime_style_t> for RuntimeStyle {
    fn into(self) -> cef_runtime_style_t {
        self.0
    }
}
impl Default for RuntimeStyle {
    fn default() -> Self {
        Self(cef_runtime_style_t::CEF_RUNTIME_STYLE_DEFAULT)
    }
}

/// See [cef_log_severity_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LogSeverity(cef_log_severity_t);
impl AsRef<cef_log_severity_t> for LogSeverity {
    fn as_ref(&self) -> &cef_log_severity_t {
        &self.0
    }
}
impl AsMut<cef_log_severity_t> for LogSeverity {
    fn as_mut(&mut self) -> &mut cef_log_severity_t {
        &mut self.0
    }
}
impl From<cef_log_severity_t> for LogSeverity {
    fn from(value: cef_log_severity_t) -> Self {
        Self(value)
    }
}
impl Into<cef_log_severity_t> for LogSeverity {
    fn into(self) -> cef_log_severity_t {
        self.0
    }
}
impl Default for LogSeverity {
    fn default() -> Self {
        Self(cef_log_severity_t::LOGSEVERITY_DEFAULT)
    }
}

/// See [cef_log_items_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LogItems(cef_log_items_t);
impl AsRef<cef_log_items_t> for LogItems {
    fn as_ref(&self) -> &cef_log_items_t {
        &self.0
    }
}
impl AsMut<cef_log_items_t> for LogItems {
    fn as_mut(&mut self) -> &mut cef_log_items_t {
        &mut self.0
    }
}
impl From<cef_log_items_t> for LogItems {
    fn from(value: cef_log_items_t) -> Self {
        Self(value)
    }
}
impl Into<cef_log_items_t> for LogItems {
    fn into(self) -> cef_log_items_t {
        self.0
    }
}
impl Default for LogItems {
    fn default() -> Self {
        Self(cef_log_items_t::LOG_ITEMS_DEFAULT)
    }
}

/// See [cef_state_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct State(cef_state_t);
impl AsRef<cef_state_t> for State {
    fn as_ref(&self) -> &cef_state_t {
        &self.0
    }
}
impl AsMut<cef_state_t> for State {
    fn as_mut(&mut self) -> &mut cef_state_t {
        &mut self.0
    }
}
impl From<cef_state_t> for State {
    fn from(value: cef_state_t) -> Self {
        Self(value)
    }
}
impl Into<cef_state_t> for State {
    fn into(self) -> cef_state_t {
        self.0
    }
}
impl Default for State {
    fn default() -> Self {
        Self(cef_state_t::STATE_DEFAULT)
    }
}

/// See [cef_return_value_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ReturnValue(cef_return_value_t);
impl AsRef<cef_return_value_t> for ReturnValue {
    fn as_ref(&self) -> &cef_return_value_t {
        &self.0
    }
}
impl AsMut<cef_return_value_t> for ReturnValue {
    fn as_mut(&mut self) -> &mut cef_return_value_t {
        &mut self.0
    }
}
impl From<cef_return_value_t> for ReturnValue {
    fn from(value: cef_return_value_t) -> Self {
        Self(value)
    }
}
impl Into<cef_return_value_t> for ReturnValue {
    fn into(self) -> cef_return_value_t {
        self.0
    }
}
impl Default for ReturnValue {
    fn default() -> Self {
        Self(cef_return_value_t::RV_CANCEL)
    }
}

/// See [cef_cookie_priority_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CookiePriority(cef_cookie_priority_t);
impl AsRef<cef_cookie_priority_t> for CookiePriority {
    fn as_ref(&self) -> &cef_cookie_priority_t {
        &self.0
    }
}
impl AsMut<cef_cookie_priority_t> for CookiePriority {
    fn as_mut(&mut self) -> &mut cef_cookie_priority_t {
        &mut self.0
    }
}
impl From<cef_cookie_priority_t> for CookiePriority {
    fn from(value: cef_cookie_priority_t) -> Self {
        Self(value)
    }
}
impl Into<cef_cookie_priority_t> for CookiePriority {
    fn into(self) -> cef_cookie_priority_t {
        self.0
    }
}
impl Default for CookiePriority {
    fn default() -> Self {
        Self(cef_cookie_priority_t::CEF_COOKIE_PRIORITY_LOW)
    }
}

/// See [cef_cookie_same_site_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CookieSameSite(cef_cookie_same_site_t);
impl AsRef<cef_cookie_same_site_t> for CookieSameSite {
    fn as_ref(&self) -> &cef_cookie_same_site_t {
        &self.0
    }
}
impl AsMut<cef_cookie_same_site_t> for CookieSameSite {
    fn as_mut(&mut self) -> &mut cef_cookie_same_site_t {
        &mut self.0
    }
}
impl From<cef_cookie_same_site_t> for CookieSameSite {
    fn from(value: cef_cookie_same_site_t) -> Self {
        Self(value)
    }
}
impl Into<cef_cookie_same_site_t> for CookieSameSite {
    fn into(self) -> cef_cookie_same_site_t {
        self.0
    }
}
impl Default for CookieSameSite {
    fn default() -> Self {
        Self(cef_cookie_same_site_t::CEF_COOKIE_SAME_SITE_UNSPECIFIED)
    }
}

/// See [cef_termination_status_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TerminationStatus(cef_termination_status_t);
impl AsRef<cef_termination_status_t> for TerminationStatus {
    fn as_ref(&self) -> &cef_termination_status_t {
        &self.0
    }
}
impl AsMut<cef_termination_status_t> for TerminationStatus {
    fn as_mut(&mut self) -> &mut cef_termination_status_t {
        &mut self.0
    }
}
impl From<cef_termination_status_t> for TerminationStatus {
    fn from(value: cef_termination_status_t) -> Self {
        Self(value)
    }
}
impl Into<cef_termination_status_t> for TerminationStatus {
    fn into(self) -> cef_termination_status_t {
        self.0
    }
}
impl Default for TerminationStatus {
    fn default() -> Self {
        Self(cef_termination_status_t::TS_ABNORMAL_TERMINATION)
    }
}

/// See [cef_path_key_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PathKey(cef_path_key_t);
impl AsRef<cef_path_key_t> for PathKey {
    fn as_ref(&self) -> &cef_path_key_t {
        &self.0
    }
}
impl AsMut<cef_path_key_t> for PathKey {
    fn as_mut(&mut self) -> &mut cef_path_key_t {
        &mut self.0
    }
}
impl From<cef_path_key_t> for PathKey {
    fn from(value: cef_path_key_t) -> Self {
        Self(value)
    }
}
impl Into<cef_path_key_t> for PathKey {
    fn into(self) -> cef_path_key_t {
        self.0
    }
}
impl Default for PathKey {
    fn default() -> Self {
        Self(cef_path_key_t::PK_DIR_CURRENT)
    }
}

/// See [cef_storage_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct StorageType(cef_storage_type_t);
impl AsRef<cef_storage_type_t> for StorageType {
    fn as_ref(&self) -> &cef_storage_type_t {
        &self.0
    }
}
impl AsMut<cef_storage_type_t> for StorageType {
    fn as_mut(&mut self) -> &mut cef_storage_type_t {
        &mut self.0
    }
}
impl From<cef_storage_type_t> for StorageType {
    fn from(value: cef_storage_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_storage_type_t> for StorageType {
    fn into(self) -> cef_storage_type_t {
        self.0
    }
}
impl Default for StorageType {
    fn default() -> Self {
        Self(cef_storage_type_t::ST_LOCALSTORAGE)
    }
}

/// See [cef_errorcode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Errorcode(cef_errorcode_t);
impl AsRef<cef_errorcode_t> for Errorcode {
    fn as_ref(&self) -> &cef_errorcode_t {
        &self.0
    }
}
impl AsMut<cef_errorcode_t> for Errorcode {
    fn as_mut(&mut self) -> &mut cef_errorcode_t {
        &mut self.0
    }
}
impl From<cef_errorcode_t> for Errorcode {
    fn from(value: cef_errorcode_t) -> Self {
        Self(value)
    }
}
impl Into<cef_errorcode_t> for Errorcode {
    fn into(self) -> cef_errorcode_t {
        self.0
    }
}
impl Default for Errorcode {
    fn default() -> Self {
        Self(cef_errorcode_t::ERR_NONE)
    }
}

/// See [cef_cert_status_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CertStatus(cef_cert_status_t);
impl AsRef<cef_cert_status_t> for CertStatus {
    fn as_ref(&self) -> &cef_cert_status_t {
        &self.0
    }
}
impl AsMut<cef_cert_status_t> for CertStatus {
    fn as_mut(&mut self) -> &mut cef_cert_status_t {
        &mut self.0
    }
}
impl From<cef_cert_status_t> for CertStatus {
    fn from(value: cef_cert_status_t) -> Self {
        Self(value)
    }
}
impl Into<cef_cert_status_t> for CertStatus {
    fn into(self) -> cef_cert_status_t {
        self.0
    }
}
impl Default for CertStatus {
    fn default() -> Self {
        Self(cef_cert_status_t::CERT_STATUS_NONE)
    }
}

/// See [cef_resultcode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Resultcode(cef_resultcode_t);
impl AsRef<cef_resultcode_t> for Resultcode {
    fn as_ref(&self) -> &cef_resultcode_t {
        &self.0
    }
}
impl AsMut<cef_resultcode_t> for Resultcode {
    fn as_mut(&mut self) -> &mut cef_resultcode_t {
        &mut self.0
    }
}
impl From<cef_resultcode_t> for Resultcode {
    fn from(value: cef_resultcode_t) -> Self {
        Self(value)
    }
}
impl Into<cef_resultcode_t> for Resultcode {
    fn into(self) -> cef_resultcode_t {
        self.0
    }
}
impl Default for Resultcode {
    fn default() -> Self {
        Self(cef_resultcode_t::CEF_RESULT_CODE_NORMAL_EXIT)
    }
}

/// See [cef_window_open_disposition_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct WindowOpenDisposition(cef_window_open_disposition_t);
impl AsRef<cef_window_open_disposition_t> for WindowOpenDisposition {
    fn as_ref(&self) -> &cef_window_open_disposition_t {
        &self.0
    }
}
impl AsMut<cef_window_open_disposition_t> for WindowOpenDisposition {
    fn as_mut(&mut self) -> &mut cef_window_open_disposition_t {
        &mut self.0
    }
}
impl From<cef_window_open_disposition_t> for WindowOpenDisposition {
    fn from(value: cef_window_open_disposition_t) -> Self {
        Self(value)
    }
}
impl Into<cef_window_open_disposition_t> for WindowOpenDisposition {
    fn into(self) -> cef_window_open_disposition_t {
        self.0
    }
}
impl Default for WindowOpenDisposition {
    fn default() -> Self {
        Self(cef_window_open_disposition_t::CEF_WOD_UNKNOWN)
    }
}

/// See [cef_drag_operations_mask_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DragOperationsMask(cef_drag_operations_mask_t);
impl AsRef<cef_drag_operations_mask_t> for DragOperationsMask {
    fn as_ref(&self) -> &cef_drag_operations_mask_t {
        &self.0
    }
}
impl AsMut<cef_drag_operations_mask_t> for DragOperationsMask {
    fn as_mut(&mut self) -> &mut cef_drag_operations_mask_t {
        &mut self.0
    }
}
impl From<cef_drag_operations_mask_t> for DragOperationsMask {
    fn from(value: cef_drag_operations_mask_t) -> Self {
        Self(value)
    }
}
impl Into<cef_drag_operations_mask_t> for DragOperationsMask {
    fn into(self) -> cef_drag_operations_mask_t {
        self.0
    }
}
impl Default for DragOperationsMask {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [cef_text_input_mode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TextInputMode(cef_text_input_mode_t);
impl AsRef<cef_text_input_mode_t> for TextInputMode {
    fn as_ref(&self) -> &cef_text_input_mode_t {
        &self.0
    }
}
impl AsMut<cef_text_input_mode_t> for TextInputMode {
    fn as_mut(&mut self) -> &mut cef_text_input_mode_t {
        &mut self.0
    }
}
impl From<cef_text_input_mode_t> for TextInputMode {
    fn from(value: cef_text_input_mode_t) -> Self {
        Self(value)
    }
}
impl Into<cef_text_input_mode_t> for TextInputMode {
    fn into(self) -> cef_text_input_mode_t {
        self.0
    }
}
impl Default for TextInputMode {
    fn default() -> Self {
        Self(cef_text_input_mode_t::CEF_TEXT_INPUT_MODE_DEFAULT)
    }
}

/// See [cef_v8_propertyattribute_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct V8Propertyattribute(cef_v8_propertyattribute_t);
impl AsRef<cef_v8_propertyattribute_t> for V8Propertyattribute {
    fn as_ref(&self) -> &cef_v8_propertyattribute_t {
        &self.0
    }
}
impl AsMut<cef_v8_propertyattribute_t> for V8Propertyattribute {
    fn as_mut(&mut self) -> &mut cef_v8_propertyattribute_t {
        &mut self.0
    }
}
impl From<cef_v8_propertyattribute_t> for V8Propertyattribute {
    fn from(value: cef_v8_propertyattribute_t) -> Self {
        Self(value)
    }
}
impl Into<cef_v8_propertyattribute_t> for V8Propertyattribute {
    fn into(self) -> cef_v8_propertyattribute_t {
        self.0
    }
}
impl Default for V8Propertyattribute {
    fn default() -> Self {
        Self(cef_v8_propertyattribute_t::V8_PROPERTY_ATTRIBUTE_NONE)
    }
}

/// See [cef_postdataelement_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PostdataelementType(cef_postdataelement_type_t);
impl AsRef<cef_postdataelement_type_t> for PostdataelementType {
    fn as_ref(&self) -> &cef_postdataelement_type_t {
        &self.0
    }
}
impl AsMut<cef_postdataelement_type_t> for PostdataelementType {
    fn as_mut(&mut self) -> &mut cef_postdataelement_type_t {
        &mut self.0
    }
}
impl From<cef_postdataelement_type_t> for PostdataelementType {
    fn from(value: cef_postdataelement_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_postdataelement_type_t> for PostdataelementType {
    fn into(self) -> cef_postdataelement_type_t {
        self.0
    }
}
impl Default for PostdataelementType {
    fn default() -> Self {
        Self(cef_postdataelement_type_t::PDE_TYPE_EMPTY)
    }
}

/// See [cef_resource_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ResourceType(cef_resource_type_t);
impl AsRef<cef_resource_type_t> for ResourceType {
    fn as_ref(&self) -> &cef_resource_type_t {
        &self.0
    }
}
impl AsMut<cef_resource_type_t> for ResourceType {
    fn as_mut(&mut self) -> &mut cef_resource_type_t {
        &mut self.0
    }
}
impl From<cef_resource_type_t> for ResourceType {
    fn from(value: cef_resource_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_resource_type_t> for ResourceType {
    fn into(self) -> cef_resource_type_t {
        self.0
    }
}
impl Default for ResourceType {
    fn default() -> Self {
        Self(cef_resource_type_t::RT_MAIN_FRAME)
    }
}

/// See [cef_transition_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TransitionType(cef_transition_type_t);
impl AsRef<cef_transition_type_t> for TransitionType {
    fn as_ref(&self) -> &cef_transition_type_t {
        &self.0
    }
}
impl AsMut<cef_transition_type_t> for TransitionType {
    fn as_mut(&mut self) -> &mut cef_transition_type_t {
        &mut self.0
    }
}
impl From<cef_transition_type_t> for TransitionType {
    fn from(value: cef_transition_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_transition_type_t> for TransitionType {
    fn into(self) -> cef_transition_type_t {
        self.0
    }
}
impl Default for TransitionType {
    fn default() -> Self {
        Self(cef_transition_type_t::TT_LINK)
    }
}

/// See [cef_urlrequest_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct UrlrequestFlags(cef_urlrequest_flags_t);
impl AsRef<cef_urlrequest_flags_t> for UrlrequestFlags {
    fn as_ref(&self) -> &cef_urlrequest_flags_t {
        &self.0
    }
}
impl AsMut<cef_urlrequest_flags_t> for UrlrequestFlags {
    fn as_mut(&mut self) -> &mut cef_urlrequest_flags_t {
        &mut self.0
    }
}
impl From<cef_urlrequest_flags_t> for UrlrequestFlags {
    fn from(value: cef_urlrequest_flags_t) -> Self {
        Self(value)
    }
}
impl Into<cef_urlrequest_flags_t> for UrlrequestFlags {
    fn into(self) -> cef_urlrequest_flags_t {
        self.0
    }
}
impl Default for UrlrequestFlags {
    fn default() -> Self {
        Self(cef_urlrequest_flags_t::UR_FLAG_NONE)
    }
}

/// See [cef_urlrequest_status_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct UrlrequestStatus(cef_urlrequest_status_t);
impl AsRef<cef_urlrequest_status_t> for UrlrequestStatus {
    fn as_ref(&self) -> &cef_urlrequest_status_t {
        &self.0
    }
}
impl AsMut<cef_urlrequest_status_t> for UrlrequestStatus {
    fn as_mut(&mut self) -> &mut cef_urlrequest_status_t {
        &mut self.0
    }
}
impl From<cef_urlrequest_status_t> for UrlrequestStatus {
    fn from(value: cef_urlrequest_status_t) -> Self {
        Self(value)
    }
}
impl Into<cef_urlrequest_status_t> for UrlrequestStatus {
    fn into(self) -> cef_urlrequest_status_t {
        self.0
    }
}
impl Default for UrlrequestStatus {
    fn default() -> Self {
        Self(cef_urlrequest_status_t::UR_UNKNOWN)
    }
}

/// See [cef_process_id_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ProcessId(cef_process_id_t);
impl AsRef<cef_process_id_t> for ProcessId {
    fn as_ref(&self) -> &cef_process_id_t {
        &self.0
    }
}
impl AsMut<cef_process_id_t> for ProcessId {
    fn as_mut(&mut self) -> &mut cef_process_id_t {
        &mut self.0
    }
}
impl From<cef_process_id_t> for ProcessId {
    fn from(value: cef_process_id_t) -> Self {
        Self(value)
    }
}
impl Into<cef_process_id_t> for ProcessId {
    fn into(self) -> cef_process_id_t {
        self.0
    }
}
impl Default for ProcessId {
    fn default() -> Self {
        Self(cef_process_id_t::PID_BROWSER)
    }
}

/// See [cef_thread_id_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ThreadId(cef_thread_id_t);
impl AsRef<cef_thread_id_t> for ThreadId {
    fn as_ref(&self) -> &cef_thread_id_t {
        &self.0
    }
}
impl AsMut<cef_thread_id_t> for ThreadId {
    fn as_mut(&mut self) -> &mut cef_thread_id_t {
        &mut self.0
    }
}
impl From<cef_thread_id_t> for ThreadId {
    fn from(value: cef_thread_id_t) -> Self {
        Self(value)
    }
}
impl Into<cef_thread_id_t> for ThreadId {
    fn into(self) -> cef_thread_id_t {
        self.0
    }
}
impl Default for ThreadId {
    fn default() -> Self {
        Self(cef_thread_id_t::TID_UI)
    }
}

/// See [cef_thread_priority_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ThreadPriority(cef_thread_priority_t);
impl AsRef<cef_thread_priority_t> for ThreadPriority {
    fn as_ref(&self) -> &cef_thread_priority_t {
        &self.0
    }
}
impl AsMut<cef_thread_priority_t> for ThreadPriority {
    fn as_mut(&mut self) -> &mut cef_thread_priority_t {
        &mut self.0
    }
}
impl From<cef_thread_priority_t> for ThreadPriority {
    fn from(value: cef_thread_priority_t) -> Self {
        Self(value)
    }
}
impl Into<cef_thread_priority_t> for ThreadPriority {
    fn into(self) -> cef_thread_priority_t {
        self.0
    }
}
impl Default for ThreadPriority {
    fn default() -> Self {
        Self(cef_thread_priority_t::TP_BACKGROUND)
    }
}

/// See [cef_message_loop_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MessageLoopType(cef_message_loop_type_t);
impl AsRef<cef_message_loop_type_t> for MessageLoopType {
    fn as_ref(&self) -> &cef_message_loop_type_t {
        &self.0
    }
}
impl AsMut<cef_message_loop_type_t> for MessageLoopType {
    fn as_mut(&mut self) -> &mut cef_message_loop_type_t {
        &mut self.0
    }
}
impl From<cef_message_loop_type_t> for MessageLoopType {
    fn from(value: cef_message_loop_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_message_loop_type_t> for MessageLoopType {
    fn into(self) -> cef_message_loop_type_t {
        self.0
    }
}
impl Default for MessageLoopType {
    fn default() -> Self {
        Self(cef_message_loop_type_t::ML_TYPE_DEFAULT)
    }
}

/// See [cef_com_init_mode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ComInitMode(cef_com_init_mode_t);
impl AsRef<cef_com_init_mode_t> for ComInitMode {
    fn as_ref(&self) -> &cef_com_init_mode_t {
        &self.0
    }
}
impl AsMut<cef_com_init_mode_t> for ComInitMode {
    fn as_mut(&mut self) -> &mut cef_com_init_mode_t {
        &mut self.0
    }
}
impl From<cef_com_init_mode_t> for ComInitMode {
    fn from(value: cef_com_init_mode_t) -> Self {
        Self(value)
    }
}
impl Into<cef_com_init_mode_t> for ComInitMode {
    fn into(self) -> cef_com_init_mode_t {
        self.0
    }
}
impl Default for ComInitMode {
    fn default() -> Self {
        Self(cef_com_init_mode_t::COM_INIT_MODE_NONE)
    }
}

/// See [cef_value_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ValueType(cef_value_type_t);
impl AsRef<cef_value_type_t> for ValueType {
    fn as_ref(&self) -> &cef_value_type_t {
        &self.0
    }
}
impl AsMut<cef_value_type_t> for ValueType {
    fn as_mut(&mut self) -> &mut cef_value_type_t {
        &mut self.0
    }
}
impl From<cef_value_type_t> for ValueType {
    fn from(value: cef_value_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_value_type_t> for ValueType {
    fn into(self) -> cef_value_type_t {
        self.0
    }
}
impl Default for ValueType {
    fn default() -> Self {
        Self(cef_value_type_t::VTYPE_INVALID)
    }
}

/// See [cef_jsdialog_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct JsdialogType(cef_jsdialog_type_t);
impl AsRef<cef_jsdialog_type_t> for JsdialogType {
    fn as_ref(&self) -> &cef_jsdialog_type_t {
        &self.0
    }
}
impl AsMut<cef_jsdialog_type_t> for JsdialogType {
    fn as_mut(&mut self) -> &mut cef_jsdialog_type_t {
        &mut self.0
    }
}
impl From<cef_jsdialog_type_t> for JsdialogType {
    fn from(value: cef_jsdialog_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_jsdialog_type_t> for JsdialogType {
    fn into(self) -> cef_jsdialog_type_t {
        self.0
    }
}
impl Default for JsdialogType {
    fn default() -> Self {
        Self(cef_jsdialog_type_t::JSDIALOGTYPE_ALERT)
    }
}

/// See [cef_menu_id_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MenuId(cef_menu_id_t);
impl AsRef<cef_menu_id_t> for MenuId {
    fn as_ref(&self) -> &cef_menu_id_t {
        &self.0
    }
}
impl AsMut<cef_menu_id_t> for MenuId {
    fn as_mut(&mut self) -> &mut cef_menu_id_t {
        &mut self.0
    }
}
impl From<cef_menu_id_t> for MenuId {
    fn from(value: cef_menu_id_t) -> Self {
        Self(value)
    }
}
impl Into<cef_menu_id_t> for MenuId {
    fn into(self) -> cef_menu_id_t {
        self.0
    }
}
impl Default for MenuId {
    fn default() -> Self {
        Self(cef_menu_id_t::MENU_ID_BACK)
    }
}

/// See [cef_mouse_button_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MouseButtonType(cef_mouse_button_type_t);
impl AsRef<cef_mouse_button_type_t> for MouseButtonType {
    fn as_ref(&self) -> &cef_mouse_button_type_t {
        &self.0
    }
}
impl AsMut<cef_mouse_button_type_t> for MouseButtonType {
    fn as_mut(&mut self) -> &mut cef_mouse_button_type_t {
        &mut self.0
    }
}
impl From<cef_mouse_button_type_t> for MouseButtonType {
    fn from(value: cef_mouse_button_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_mouse_button_type_t> for MouseButtonType {
    fn into(self) -> cef_mouse_button_type_t {
        self.0
    }
}
impl Default for MouseButtonType {
    fn default() -> Self {
        Self(cef_mouse_button_type_t::MBT_LEFT)
    }
}

/// See [cef_touch_event_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TouchEventType(cef_touch_event_type_t);
impl AsRef<cef_touch_event_type_t> for TouchEventType {
    fn as_ref(&self) -> &cef_touch_event_type_t {
        &self.0
    }
}
impl AsMut<cef_touch_event_type_t> for TouchEventType {
    fn as_mut(&mut self) -> &mut cef_touch_event_type_t {
        &mut self.0
    }
}
impl From<cef_touch_event_type_t> for TouchEventType {
    fn from(value: cef_touch_event_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_touch_event_type_t> for TouchEventType {
    fn into(self) -> cef_touch_event_type_t {
        self.0
    }
}
impl Default for TouchEventType {
    fn default() -> Self {
        Self(cef_touch_event_type_t::CEF_TET_RELEASED)
    }
}

/// See [cef_pointer_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PointerType(cef_pointer_type_t);
impl AsRef<cef_pointer_type_t> for PointerType {
    fn as_ref(&self) -> &cef_pointer_type_t {
        &self.0
    }
}
impl AsMut<cef_pointer_type_t> for PointerType {
    fn as_mut(&mut self) -> &mut cef_pointer_type_t {
        &mut self.0
    }
}
impl From<cef_pointer_type_t> for PointerType {
    fn from(value: cef_pointer_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_pointer_type_t> for PointerType {
    fn into(self) -> cef_pointer_type_t {
        self.0
    }
}
impl Default for PointerType {
    fn default() -> Self {
        Self(cef_pointer_type_t::CEF_POINTER_TYPE_TOUCH)
    }
}

/// See [cef_paint_element_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PaintElementType(cef_paint_element_type_t);
impl AsRef<cef_paint_element_type_t> for PaintElementType {
    fn as_ref(&self) -> &cef_paint_element_type_t {
        &self.0
    }
}
impl AsMut<cef_paint_element_type_t> for PaintElementType {
    fn as_mut(&mut self) -> &mut cef_paint_element_type_t {
        &mut self.0
    }
}
impl From<cef_paint_element_type_t> for PaintElementType {
    fn from(value: cef_paint_element_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_paint_element_type_t> for PaintElementType {
    fn into(self) -> cef_paint_element_type_t {
        self.0
    }
}
impl Default for PaintElementType {
    fn default() -> Self {
        Self(cef_paint_element_type_t::PET_VIEW)
    }
}

/// See [cef_event_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct EventFlags(cef_event_flags_t);
impl AsRef<cef_event_flags_t> for EventFlags {
    fn as_ref(&self) -> &cef_event_flags_t {
        &self.0
    }
}
impl AsMut<cef_event_flags_t> for EventFlags {
    fn as_mut(&mut self) -> &mut cef_event_flags_t {
        &mut self.0
    }
}
impl From<cef_event_flags_t> for EventFlags {
    fn from(value: cef_event_flags_t) -> Self {
        Self(value)
    }
}
impl Into<cef_event_flags_t> for EventFlags {
    fn into(self) -> cef_event_flags_t {
        self.0
    }
}
impl Default for EventFlags {
    fn default() -> Self {
        Self(cef_event_flags_t::EVENTFLAG_NONE)
    }
}

/// See [cef_menu_item_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MenuItemType(cef_menu_item_type_t);
impl AsRef<cef_menu_item_type_t> for MenuItemType {
    fn as_ref(&self) -> &cef_menu_item_type_t {
        &self.0
    }
}
impl AsMut<cef_menu_item_type_t> for MenuItemType {
    fn as_mut(&mut self) -> &mut cef_menu_item_type_t {
        &mut self.0
    }
}
impl From<cef_menu_item_type_t> for MenuItemType {
    fn from(value: cef_menu_item_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_menu_item_type_t> for MenuItemType {
    fn into(self) -> cef_menu_item_type_t {
        self.0
    }
}
impl Default for MenuItemType {
    fn default() -> Self {
        Self(cef_menu_item_type_t::MENUITEMTYPE_NONE)
    }
}

/// See [cef_context_menu_type_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContextMenuTypeFlags(cef_context_menu_type_flags_t);
impl AsRef<cef_context_menu_type_flags_t> for ContextMenuTypeFlags {
    fn as_ref(&self) -> &cef_context_menu_type_flags_t {
        &self.0
    }
}
impl AsMut<cef_context_menu_type_flags_t> for ContextMenuTypeFlags {
    fn as_mut(&mut self) -> &mut cef_context_menu_type_flags_t {
        &mut self.0
    }
}
impl From<cef_context_menu_type_flags_t> for ContextMenuTypeFlags {
    fn from(value: cef_context_menu_type_flags_t) -> Self {
        Self(value)
    }
}
impl Into<cef_context_menu_type_flags_t> for ContextMenuTypeFlags {
    fn into(self) -> cef_context_menu_type_flags_t {
        self.0
    }
}
impl Default for ContextMenuTypeFlags {
    fn default() -> Self {
        Self(cef_context_menu_type_flags_t::CM_TYPEFLAG_NONE)
    }
}

/// See [cef_context_menu_media_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContextMenuMediaType(cef_context_menu_media_type_t);
impl AsRef<cef_context_menu_media_type_t> for ContextMenuMediaType {
    fn as_ref(&self) -> &cef_context_menu_media_type_t {
        &self.0
    }
}
impl AsMut<cef_context_menu_media_type_t> for ContextMenuMediaType {
    fn as_mut(&mut self) -> &mut cef_context_menu_media_type_t {
        &mut self.0
    }
}
impl From<cef_context_menu_media_type_t> for ContextMenuMediaType {
    fn from(value: cef_context_menu_media_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_context_menu_media_type_t> for ContextMenuMediaType {
    fn into(self) -> cef_context_menu_media_type_t {
        self.0
    }
}
impl Default for ContextMenuMediaType {
    fn default() -> Self {
        Self(cef_context_menu_media_type_t::CM_MEDIATYPE_NONE)
    }
}

/// See [cef_context_menu_media_state_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContextMenuMediaStateFlags(cef_context_menu_media_state_flags_t);
impl AsRef<cef_context_menu_media_state_flags_t> for ContextMenuMediaStateFlags {
    fn as_ref(&self) -> &cef_context_menu_media_state_flags_t {
        &self.0
    }
}
impl AsMut<cef_context_menu_media_state_flags_t> for ContextMenuMediaStateFlags {
    fn as_mut(&mut self) -> &mut cef_context_menu_media_state_flags_t {
        &mut self.0
    }
}
impl From<cef_context_menu_media_state_flags_t> for ContextMenuMediaStateFlags {
    fn from(value: cef_context_menu_media_state_flags_t) -> Self {
        Self(value)
    }
}
impl Into<cef_context_menu_media_state_flags_t> for ContextMenuMediaStateFlags {
    fn into(self) -> cef_context_menu_media_state_flags_t {
        self.0
    }
}
impl Default for ContextMenuMediaStateFlags {
    fn default() -> Self {
        Self(cef_context_menu_media_state_flags_t::CM_MEDIAFLAG_NONE)
    }
}

/// See [cef_context_menu_edit_state_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ContextMenuEditStateFlags(cef_context_menu_edit_state_flags_t);
impl AsRef<cef_context_menu_edit_state_flags_t> for ContextMenuEditStateFlags {
    fn as_ref(&self) -> &cef_context_menu_edit_state_flags_t {
        &self.0
    }
}
impl AsMut<cef_context_menu_edit_state_flags_t> for ContextMenuEditStateFlags {
    fn as_mut(&mut self) -> &mut cef_context_menu_edit_state_flags_t {
        &mut self.0
    }
}
impl From<cef_context_menu_edit_state_flags_t> for ContextMenuEditStateFlags {
    fn from(value: cef_context_menu_edit_state_flags_t) -> Self {
        Self(value)
    }
}
impl Into<cef_context_menu_edit_state_flags_t> for ContextMenuEditStateFlags {
    fn into(self) -> cef_context_menu_edit_state_flags_t {
        self.0
    }
}
impl Default for ContextMenuEditStateFlags {
    fn default() -> Self {
        Self(cef_context_menu_edit_state_flags_t::CM_EDITFLAG_NONE)
    }
}

/// See [cef_quick_menu_edit_state_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct QuickMenuEditStateFlags(cef_quick_menu_edit_state_flags_t);
impl AsRef<cef_quick_menu_edit_state_flags_t> for QuickMenuEditStateFlags {
    fn as_ref(&self) -> &cef_quick_menu_edit_state_flags_t {
        &self.0
    }
}
impl AsMut<cef_quick_menu_edit_state_flags_t> for QuickMenuEditStateFlags {
    fn as_mut(&mut self) -> &mut cef_quick_menu_edit_state_flags_t {
        &mut self.0
    }
}
impl From<cef_quick_menu_edit_state_flags_t> for QuickMenuEditStateFlags {
    fn from(value: cef_quick_menu_edit_state_flags_t) -> Self {
        Self(value)
    }
}
impl Into<cef_quick_menu_edit_state_flags_t> for QuickMenuEditStateFlags {
    fn into(self) -> cef_quick_menu_edit_state_flags_t {
        self.0
    }
}
impl Default for QuickMenuEditStateFlags {
    fn default() -> Self {
        Self(cef_quick_menu_edit_state_flags_t::QM_EDITFLAG_NONE)
    }
}

/// See [cef_key_event_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct KeyEventType(cef_key_event_type_t);
impl AsRef<cef_key_event_type_t> for KeyEventType {
    fn as_ref(&self) -> &cef_key_event_type_t {
        &self.0
    }
}
impl AsMut<cef_key_event_type_t> for KeyEventType {
    fn as_mut(&mut self) -> &mut cef_key_event_type_t {
        &mut self.0
    }
}
impl From<cef_key_event_type_t> for KeyEventType {
    fn from(value: cef_key_event_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_key_event_type_t> for KeyEventType {
    fn into(self) -> cef_key_event_type_t {
        self.0
    }
}
impl Default for KeyEventType {
    fn default() -> Self {
        Self(cef_key_event_type_t::KEYEVENT_RAWKEYDOWN)
    }
}

/// See [cef_focus_source_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FocusSource(cef_focus_source_t);
impl AsRef<cef_focus_source_t> for FocusSource {
    fn as_ref(&self) -> &cef_focus_source_t {
        &self.0
    }
}
impl AsMut<cef_focus_source_t> for FocusSource {
    fn as_mut(&mut self) -> &mut cef_focus_source_t {
        &mut self.0
    }
}
impl From<cef_focus_source_t> for FocusSource {
    fn from(value: cef_focus_source_t) -> Self {
        Self(value)
    }
}
impl Into<cef_focus_source_t> for FocusSource {
    fn into(self) -> cef_focus_source_t {
        self.0
    }
}
impl Default for FocusSource {
    fn default() -> Self {
        Self(cef_focus_source_t::FOCUS_SOURCE_NAVIGATION)
    }
}

/// See [cef_navigation_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NavigationType(cef_navigation_type_t);
impl AsRef<cef_navigation_type_t> for NavigationType {
    fn as_ref(&self) -> &cef_navigation_type_t {
        &self.0
    }
}
impl AsMut<cef_navigation_type_t> for NavigationType {
    fn as_mut(&mut self) -> &mut cef_navigation_type_t {
        &mut self.0
    }
}
impl From<cef_navigation_type_t> for NavigationType {
    fn from(value: cef_navigation_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_navigation_type_t> for NavigationType {
    fn into(self) -> cef_navigation_type_t {
        self.0
    }
}
impl Default for NavigationType {
    fn default() -> Self {
        Self(cef_navigation_type_t::NAVIGATION_LINK_CLICKED)
    }
}

/// See [cef_xml_encoding_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct XmlEncodingType(cef_xml_encoding_type_t);
impl AsRef<cef_xml_encoding_type_t> for XmlEncodingType {
    fn as_ref(&self) -> &cef_xml_encoding_type_t {
        &self.0
    }
}
impl AsMut<cef_xml_encoding_type_t> for XmlEncodingType {
    fn as_mut(&mut self) -> &mut cef_xml_encoding_type_t {
        &mut self.0
    }
}
impl From<cef_xml_encoding_type_t> for XmlEncodingType {
    fn from(value: cef_xml_encoding_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_xml_encoding_type_t> for XmlEncodingType {
    fn into(self) -> cef_xml_encoding_type_t {
        self.0
    }
}
impl Default for XmlEncodingType {
    fn default() -> Self {
        Self(cef_xml_encoding_type_t::XML_ENCODING_NONE)
    }
}

/// See [cef_xml_node_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct XmlNodeType(cef_xml_node_type_t);
impl AsRef<cef_xml_node_type_t> for XmlNodeType {
    fn as_ref(&self) -> &cef_xml_node_type_t {
        &self.0
    }
}
impl AsMut<cef_xml_node_type_t> for XmlNodeType {
    fn as_mut(&mut self) -> &mut cef_xml_node_type_t {
        &mut self.0
    }
}
impl From<cef_xml_node_type_t> for XmlNodeType {
    fn from(value: cef_xml_node_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_xml_node_type_t> for XmlNodeType {
    fn into(self) -> cef_xml_node_type_t {
        self.0
    }
}
impl Default for XmlNodeType {
    fn default() -> Self {
        Self(cef_xml_node_type_t::XML_NODE_UNSUPPORTED)
    }
}

/// See [cef_dom_document_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DomDocumentType(cef_dom_document_type_t);
impl AsRef<cef_dom_document_type_t> for DomDocumentType {
    fn as_ref(&self) -> &cef_dom_document_type_t {
        &self.0
    }
}
impl AsMut<cef_dom_document_type_t> for DomDocumentType {
    fn as_mut(&mut self) -> &mut cef_dom_document_type_t {
        &mut self.0
    }
}
impl From<cef_dom_document_type_t> for DomDocumentType {
    fn from(value: cef_dom_document_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_dom_document_type_t> for DomDocumentType {
    fn into(self) -> cef_dom_document_type_t {
        self.0
    }
}
impl Default for DomDocumentType {
    fn default() -> Self {
        Self(cef_dom_document_type_t::DOM_DOCUMENT_TYPE_UNKNOWN)
    }
}

/// See [cef_dom_event_category_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DomEventCategory(cef_dom_event_category_t);
impl AsRef<cef_dom_event_category_t> for DomEventCategory {
    fn as_ref(&self) -> &cef_dom_event_category_t {
        &self.0
    }
}
impl AsMut<cef_dom_event_category_t> for DomEventCategory {
    fn as_mut(&mut self) -> &mut cef_dom_event_category_t {
        &mut self.0
    }
}
impl From<cef_dom_event_category_t> for DomEventCategory {
    fn from(value: cef_dom_event_category_t) -> Self {
        Self(value)
    }
}
impl Into<cef_dom_event_category_t> for DomEventCategory {
    fn into(self) -> cef_dom_event_category_t {
        self.0
    }
}
impl Default for DomEventCategory {
    fn default() -> Self {
        Self(cef_dom_event_category_t::DOM_EVENT_CATEGORY_UNKNOWN)
    }
}

/// See [cef_dom_event_phase_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DomEventPhase(cef_dom_event_phase_t);
impl AsRef<cef_dom_event_phase_t> for DomEventPhase {
    fn as_ref(&self) -> &cef_dom_event_phase_t {
        &self.0
    }
}
impl AsMut<cef_dom_event_phase_t> for DomEventPhase {
    fn as_mut(&mut self) -> &mut cef_dom_event_phase_t {
        &mut self.0
    }
}
impl From<cef_dom_event_phase_t> for DomEventPhase {
    fn from(value: cef_dom_event_phase_t) -> Self {
        Self(value)
    }
}
impl Into<cef_dom_event_phase_t> for DomEventPhase {
    fn into(self) -> cef_dom_event_phase_t {
        self.0
    }
}
impl Default for DomEventPhase {
    fn default() -> Self {
        Self(cef_dom_event_phase_t::DOM_EVENT_PHASE_UNKNOWN)
    }
}

/// See [cef_dom_node_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DomNodeType(cef_dom_node_type_t);
impl AsRef<cef_dom_node_type_t> for DomNodeType {
    fn as_ref(&self) -> &cef_dom_node_type_t {
        &self.0
    }
}
impl AsMut<cef_dom_node_type_t> for DomNodeType {
    fn as_mut(&mut self) -> &mut cef_dom_node_type_t {
        &mut self.0
    }
}
impl From<cef_dom_node_type_t> for DomNodeType {
    fn from(value: cef_dom_node_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_dom_node_type_t> for DomNodeType {
    fn into(self) -> cef_dom_node_type_t {
        self.0
    }
}
impl Default for DomNodeType {
    fn default() -> Self {
        Self(cef_dom_node_type_t::DOM_NODE_TYPE_UNSUPPORTED)
    }
}

/// See [cef_dom_form_control_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DomFormControlType(cef_dom_form_control_type_t);
impl AsRef<cef_dom_form_control_type_t> for DomFormControlType {
    fn as_ref(&self) -> &cef_dom_form_control_type_t {
        &self.0
    }
}
impl AsMut<cef_dom_form_control_type_t> for DomFormControlType {
    fn as_mut(&mut self) -> &mut cef_dom_form_control_type_t {
        &mut self.0
    }
}
impl From<cef_dom_form_control_type_t> for DomFormControlType {
    fn from(value: cef_dom_form_control_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_dom_form_control_type_t> for DomFormControlType {
    fn into(self) -> cef_dom_form_control_type_t {
        self.0
    }
}
impl Default for DomFormControlType {
    fn default() -> Self {
        Self(cef_dom_form_control_type_t::DOM_FORM_CONTROL_TYPE_UNSUPPORTED)
    }
}

/// See [cef_file_dialog_mode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct FileDialogMode(cef_file_dialog_mode_t);
impl AsRef<cef_file_dialog_mode_t> for FileDialogMode {
    fn as_ref(&self) -> &cef_file_dialog_mode_t {
        &self.0
    }
}
impl AsMut<cef_file_dialog_mode_t> for FileDialogMode {
    fn as_mut(&mut self) -> &mut cef_file_dialog_mode_t {
        &mut self.0
    }
}
impl From<cef_file_dialog_mode_t> for FileDialogMode {
    fn from(value: cef_file_dialog_mode_t) -> Self {
        Self(value)
    }
}
impl Into<cef_file_dialog_mode_t> for FileDialogMode {
    fn into(self) -> cef_file_dialog_mode_t {
        self.0
    }
}
impl Default for FileDialogMode {
    fn default() -> Self {
        Self(cef_file_dialog_mode_t::FILE_DIALOG_OPEN)
    }
}

/// See [cef_color_model_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ColorModel(cef_color_model_t);
impl AsRef<cef_color_model_t> for ColorModel {
    fn as_ref(&self) -> &cef_color_model_t {
        &self.0
    }
}
impl AsMut<cef_color_model_t> for ColorModel {
    fn as_mut(&mut self) -> &mut cef_color_model_t {
        &mut self.0
    }
}
impl From<cef_color_model_t> for ColorModel {
    fn from(value: cef_color_model_t) -> Self {
        Self(value)
    }
}
impl Into<cef_color_model_t> for ColorModel {
    fn into(self) -> cef_color_model_t {
        self.0
    }
}
impl Default for ColorModel {
    fn default() -> Self {
        Self(cef_color_model_t::COLOR_MODEL_UNKNOWN)
    }
}

/// See [cef_duplex_mode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DuplexMode(cef_duplex_mode_t);
impl AsRef<cef_duplex_mode_t> for DuplexMode {
    fn as_ref(&self) -> &cef_duplex_mode_t {
        &self.0
    }
}
impl AsMut<cef_duplex_mode_t> for DuplexMode {
    fn as_mut(&mut self) -> &mut cef_duplex_mode_t {
        &mut self.0
    }
}
impl From<cef_duplex_mode_t> for DuplexMode {
    fn from(value: cef_duplex_mode_t) -> Self {
        Self(value)
    }
}
impl Into<cef_duplex_mode_t> for DuplexMode {
    fn into(self) -> cef_duplex_mode_t {
        self.0
    }
}
impl Default for DuplexMode {
    fn default() -> Self {
        Self(cef_duplex_mode_t::DUPLEX_MODE_UNKNOWN)
    }
}

/// See [cef_cursor_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CursorType(cef_cursor_type_t);
impl AsRef<cef_cursor_type_t> for CursorType {
    fn as_ref(&self) -> &cef_cursor_type_t {
        &self.0
    }
}
impl AsMut<cef_cursor_type_t> for CursorType {
    fn as_mut(&mut self) -> &mut cef_cursor_type_t {
        &mut self.0
    }
}
impl From<cef_cursor_type_t> for CursorType {
    fn from(value: cef_cursor_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_cursor_type_t> for CursorType {
    fn into(self) -> cef_cursor_type_t {
        self.0
    }
}
impl Default for CursorType {
    fn default() -> Self {
        Self(cef_cursor_type_t::CT_POINTER)
    }
}

/// See [cef_uri_unescape_rule_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct UriUnescapeRule(cef_uri_unescape_rule_t);
impl AsRef<cef_uri_unescape_rule_t> for UriUnescapeRule {
    fn as_ref(&self) -> &cef_uri_unescape_rule_t {
        &self.0
    }
}
impl AsMut<cef_uri_unescape_rule_t> for UriUnescapeRule {
    fn as_mut(&mut self) -> &mut cef_uri_unescape_rule_t {
        &mut self.0
    }
}
impl From<cef_uri_unescape_rule_t> for UriUnescapeRule {
    fn from(value: cef_uri_unescape_rule_t) -> Self {
        Self(value)
    }
}
impl Into<cef_uri_unescape_rule_t> for UriUnescapeRule {
    fn into(self) -> cef_uri_unescape_rule_t {
        self.0
    }
}
impl Default for UriUnescapeRule {
    fn default() -> Self {
        Self(cef_uri_unescape_rule_t::UU_NONE)
    }
}

/// See [cef_json_parser_options_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct JsonParserOptions(cef_json_parser_options_t);
impl AsRef<cef_json_parser_options_t> for JsonParserOptions {
    fn as_ref(&self) -> &cef_json_parser_options_t {
        &self.0
    }
}
impl AsMut<cef_json_parser_options_t> for JsonParserOptions {
    fn as_mut(&mut self) -> &mut cef_json_parser_options_t {
        &mut self.0
    }
}
impl From<cef_json_parser_options_t> for JsonParserOptions {
    fn from(value: cef_json_parser_options_t) -> Self {
        Self(value)
    }
}
impl Into<cef_json_parser_options_t> for JsonParserOptions {
    fn into(self) -> cef_json_parser_options_t {
        self.0
    }
}
impl Default for JsonParserOptions {
    fn default() -> Self {
        Self(cef_json_parser_options_t::JSON_PARSER_RFC)
    }
}

/// See [cef_json_writer_options_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct JsonWriterOptions(cef_json_writer_options_t);
impl AsRef<cef_json_writer_options_t> for JsonWriterOptions {
    fn as_ref(&self) -> &cef_json_writer_options_t {
        &self.0
    }
}
impl AsMut<cef_json_writer_options_t> for JsonWriterOptions {
    fn as_mut(&mut self) -> &mut cef_json_writer_options_t {
        &mut self.0
    }
}
impl From<cef_json_writer_options_t> for JsonWriterOptions {
    fn from(value: cef_json_writer_options_t) -> Self {
        Self(value)
    }
}
impl Into<cef_json_writer_options_t> for JsonWriterOptions {
    fn into(self) -> cef_json_writer_options_t {
        self.0
    }
}
impl Default for JsonWriterOptions {
    fn default() -> Self {
        Self(cef_json_writer_options_t::JSON_WRITER_DEFAULT)
    }
}

/// See [cef_pdf_print_margin_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PdfPrintMarginType(cef_pdf_print_margin_type_t);
impl AsRef<cef_pdf_print_margin_type_t> for PdfPrintMarginType {
    fn as_ref(&self) -> &cef_pdf_print_margin_type_t {
        &self.0
    }
}
impl AsMut<cef_pdf_print_margin_type_t> for PdfPrintMarginType {
    fn as_mut(&mut self) -> &mut cef_pdf_print_margin_type_t {
        &mut self.0
    }
}
impl From<cef_pdf_print_margin_type_t> for PdfPrintMarginType {
    fn from(value: cef_pdf_print_margin_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_pdf_print_margin_type_t> for PdfPrintMarginType {
    fn into(self) -> cef_pdf_print_margin_type_t {
        self.0
    }
}
impl Default for PdfPrintMarginType {
    fn default() -> Self {
        Self(cef_pdf_print_margin_type_t::PDF_PRINT_MARGIN_DEFAULT)
    }
}

/// See [cef_scale_factor_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ScaleFactor(cef_scale_factor_t);
impl AsRef<cef_scale_factor_t> for ScaleFactor {
    fn as_ref(&self) -> &cef_scale_factor_t {
        &self.0
    }
}
impl AsMut<cef_scale_factor_t> for ScaleFactor {
    fn as_mut(&mut self) -> &mut cef_scale_factor_t {
        &mut self.0
    }
}
impl From<cef_scale_factor_t> for ScaleFactor {
    fn from(value: cef_scale_factor_t) -> Self {
        Self(value)
    }
}
impl Into<cef_scale_factor_t> for ScaleFactor {
    fn into(self) -> cef_scale_factor_t {
        self.0
    }
}
impl Default for ScaleFactor {
    fn default() -> Self {
        Self(cef_scale_factor_t::SCALE_FACTOR_NONE)
    }
}

/// See [cef_referrer_policy_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ReferrerPolicy(cef_referrer_policy_t);
impl AsRef<cef_referrer_policy_t> for ReferrerPolicy {
    fn as_ref(&self) -> &cef_referrer_policy_t {
        &self.0
    }
}
impl AsMut<cef_referrer_policy_t> for ReferrerPolicy {
    fn as_mut(&mut self) -> &mut cef_referrer_policy_t {
        &mut self.0
    }
}
impl From<cef_referrer_policy_t> for ReferrerPolicy {
    fn from(value: cef_referrer_policy_t) -> Self {
        Self(value)
    }
}
impl Into<cef_referrer_policy_t> for ReferrerPolicy {
    fn into(self) -> cef_referrer_policy_t {
        self.0
    }
}
impl Default for ReferrerPolicy {
    fn default() -> Self {
        Self (cef_referrer_policy_t :: REFERRER_POLICY_CLEAR_REFERRER_ON_TRANSITION_FROM_SECURE_TO_INSECURE)
    }
}

/// See [cef_response_filter_status_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ResponseFilterStatus(cef_response_filter_status_t);
impl AsRef<cef_response_filter_status_t> for ResponseFilterStatus {
    fn as_ref(&self) -> &cef_response_filter_status_t {
        &self.0
    }
}
impl AsMut<cef_response_filter_status_t> for ResponseFilterStatus {
    fn as_mut(&mut self) -> &mut cef_response_filter_status_t {
        &mut self.0
    }
}
impl From<cef_response_filter_status_t> for ResponseFilterStatus {
    fn from(value: cef_response_filter_status_t) -> Self {
        Self(value)
    }
}
impl Into<cef_response_filter_status_t> for ResponseFilterStatus {
    fn into(self) -> cef_response_filter_status_t {
        self.0
    }
}
impl Default for ResponseFilterStatus {
    fn default() -> Self {
        Self(cef_response_filter_status_t::RESPONSE_FILTER_NEED_MORE_DATA)
    }
}

/// See [cef_alpha_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct AlphaType(cef_alpha_type_t);
impl AsRef<cef_alpha_type_t> for AlphaType {
    fn as_ref(&self) -> &cef_alpha_type_t {
        &self.0
    }
}
impl AsMut<cef_alpha_type_t> for AlphaType {
    fn as_mut(&mut self) -> &mut cef_alpha_type_t {
        &mut self.0
    }
}
impl From<cef_alpha_type_t> for AlphaType {
    fn from(value: cef_alpha_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_alpha_type_t> for AlphaType {
    fn into(self) -> cef_alpha_type_t {
        self.0
    }
}
impl Default for AlphaType {
    fn default() -> Self {
        Self(cef_alpha_type_t::CEF_ALPHA_TYPE_OPAQUE)
    }
}

/// See [cef_text_style_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TextStyle(cef_text_style_t);
impl AsRef<cef_text_style_t> for TextStyle {
    fn as_ref(&self) -> &cef_text_style_t {
        &self.0
    }
}
impl AsMut<cef_text_style_t> for TextStyle {
    fn as_mut(&mut self) -> &mut cef_text_style_t {
        &mut self.0
    }
}
impl From<cef_text_style_t> for TextStyle {
    fn from(value: cef_text_style_t) -> Self {
        Self(value)
    }
}
impl Into<cef_text_style_t> for TextStyle {
    fn into(self) -> cef_text_style_t {
        self.0
    }
}
impl Default for TextStyle {
    fn default() -> Self {
        Self(cef_text_style_t::CEF_TEXT_STYLE_BOLD)
    }
}

/// See [cef_axis_alignment_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct AxisAlignment(cef_axis_alignment_t);
impl AsRef<cef_axis_alignment_t> for AxisAlignment {
    fn as_ref(&self) -> &cef_axis_alignment_t {
        &self.0
    }
}
impl AsMut<cef_axis_alignment_t> for AxisAlignment {
    fn as_mut(&mut self) -> &mut cef_axis_alignment_t {
        &mut self.0
    }
}
impl From<cef_axis_alignment_t> for AxisAlignment {
    fn from(value: cef_axis_alignment_t) -> Self {
        Self(value)
    }
}
impl Into<cef_axis_alignment_t> for AxisAlignment {
    fn into(self) -> cef_axis_alignment_t {
        self.0
    }
}
impl Default for AxisAlignment {
    fn default() -> Self {
        Self(cef_axis_alignment_t::CEF_AXIS_ALIGNMENT_START)
    }
}

/// See [cef_button_state_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ButtonState(cef_button_state_t);
impl AsRef<cef_button_state_t> for ButtonState {
    fn as_ref(&self) -> &cef_button_state_t {
        &self.0
    }
}
impl AsMut<cef_button_state_t> for ButtonState {
    fn as_mut(&mut self) -> &mut cef_button_state_t {
        &mut self.0
    }
}
impl From<cef_button_state_t> for ButtonState {
    fn from(value: cef_button_state_t) -> Self {
        Self(value)
    }
}
impl Into<cef_button_state_t> for ButtonState {
    fn into(self) -> cef_button_state_t {
        self.0
    }
}
impl Default for ButtonState {
    fn default() -> Self {
        Self(cef_button_state_t::CEF_BUTTON_STATE_NORMAL)
    }
}

/// See [cef_horizontal_alignment_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HorizontalAlignment(cef_horizontal_alignment_t);
impl AsRef<cef_horizontal_alignment_t> for HorizontalAlignment {
    fn as_ref(&self) -> &cef_horizontal_alignment_t {
        &self.0
    }
}
impl AsMut<cef_horizontal_alignment_t> for HorizontalAlignment {
    fn as_mut(&mut self) -> &mut cef_horizontal_alignment_t {
        &mut self.0
    }
}
impl From<cef_horizontal_alignment_t> for HorizontalAlignment {
    fn from(value: cef_horizontal_alignment_t) -> Self {
        Self(value)
    }
}
impl Into<cef_horizontal_alignment_t> for HorizontalAlignment {
    fn into(self) -> cef_horizontal_alignment_t {
        self.0
    }
}
impl Default for HorizontalAlignment {
    fn default() -> Self {
        Self(cef_horizontal_alignment_t::CEF_HORIZONTAL_ALIGNMENT_LEFT)
    }
}

/// See [cef_menu_anchor_position_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MenuAnchorPosition(cef_menu_anchor_position_t);
impl AsRef<cef_menu_anchor_position_t> for MenuAnchorPosition {
    fn as_ref(&self) -> &cef_menu_anchor_position_t {
        &self.0
    }
}
impl AsMut<cef_menu_anchor_position_t> for MenuAnchorPosition {
    fn as_mut(&mut self) -> &mut cef_menu_anchor_position_t {
        &mut self.0
    }
}
impl From<cef_menu_anchor_position_t> for MenuAnchorPosition {
    fn from(value: cef_menu_anchor_position_t) -> Self {
        Self(value)
    }
}
impl Into<cef_menu_anchor_position_t> for MenuAnchorPosition {
    fn into(self) -> cef_menu_anchor_position_t {
        self.0
    }
}
impl Default for MenuAnchorPosition {
    fn default() -> Self {
        Self(cef_menu_anchor_position_t::CEF_MENU_ANCHOR_TOPLEFT)
    }
}

/// See [cef_menu_color_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MenuColorType(cef_menu_color_type_t);
impl AsRef<cef_menu_color_type_t> for MenuColorType {
    fn as_ref(&self) -> &cef_menu_color_type_t {
        &self.0
    }
}
impl AsMut<cef_menu_color_type_t> for MenuColorType {
    fn as_mut(&mut self) -> &mut cef_menu_color_type_t {
        &mut self.0
    }
}
impl From<cef_menu_color_type_t> for MenuColorType {
    fn from(value: cef_menu_color_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_menu_color_type_t> for MenuColorType {
    fn into(self) -> cef_menu_color_type_t {
        self.0
    }
}
impl Default for MenuColorType {
    fn default() -> Self {
        Self(cef_menu_color_type_t::CEF_MENU_COLOR_TEXT)
    }
}

/// See [cef_ssl_version_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SslVersion(cef_ssl_version_t);
impl AsRef<cef_ssl_version_t> for SslVersion {
    fn as_ref(&self) -> &cef_ssl_version_t {
        &self.0
    }
}
impl AsMut<cef_ssl_version_t> for SslVersion {
    fn as_mut(&mut self) -> &mut cef_ssl_version_t {
        &mut self.0
    }
}
impl From<cef_ssl_version_t> for SslVersion {
    fn from(value: cef_ssl_version_t) -> Self {
        Self(value)
    }
}
impl Into<cef_ssl_version_t> for SslVersion {
    fn into(self) -> cef_ssl_version_t {
        self.0
    }
}
impl Default for SslVersion {
    fn default() -> Self {
        Self(cef_ssl_version_t::SSL_CONNECTION_VERSION_UNKNOWN)
    }
}

/// See [cef_ssl_content_status_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SslContentStatus(cef_ssl_content_status_t);
impl AsRef<cef_ssl_content_status_t> for SslContentStatus {
    fn as_ref(&self) -> &cef_ssl_content_status_t {
        &self.0
    }
}
impl AsMut<cef_ssl_content_status_t> for SslContentStatus {
    fn as_mut(&mut self) -> &mut cef_ssl_content_status_t {
        &mut self.0
    }
}
impl From<cef_ssl_content_status_t> for SslContentStatus {
    fn from(value: cef_ssl_content_status_t) -> Self {
        Self(value)
    }
}
impl Into<cef_ssl_content_status_t> for SslContentStatus {
    fn into(self) -> cef_ssl_content_status_t {
        self.0
    }
}
impl Default for SslContentStatus {
    fn default() -> Self {
        Self(cef_ssl_content_status_t::SSL_CONTENT_NORMAL_CONTENT)
    }
}

/// See [cef_scheme_options_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SchemeOptions(cef_scheme_options_t);
impl AsRef<cef_scheme_options_t> for SchemeOptions {
    fn as_ref(&self) -> &cef_scheme_options_t {
        &self.0
    }
}
impl AsMut<cef_scheme_options_t> for SchemeOptions {
    fn as_mut(&mut self) -> &mut cef_scheme_options_t {
        &mut self.0
    }
}
impl From<cef_scheme_options_t> for SchemeOptions {
    fn from(value: cef_scheme_options_t) -> Self {
        Self(value)
    }
}
impl Into<cef_scheme_options_t> for SchemeOptions {
    fn into(self) -> cef_scheme_options_t {
        self.0
    }
}
impl Default for SchemeOptions {
    fn default() -> Self {
        Self(cef_scheme_options_t::CEF_SCHEME_OPTION_NONE)
    }
}

/// See [cef_composition_underline_style_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CompositionUnderlineStyle(cef_composition_underline_style_t);
impl AsRef<cef_composition_underline_style_t> for CompositionUnderlineStyle {
    fn as_ref(&self) -> &cef_composition_underline_style_t {
        &self.0
    }
}
impl AsMut<cef_composition_underline_style_t> for CompositionUnderlineStyle {
    fn as_mut(&mut self) -> &mut cef_composition_underline_style_t {
        &mut self.0
    }
}
impl From<cef_composition_underline_style_t> for CompositionUnderlineStyle {
    fn from(value: cef_composition_underline_style_t) -> Self {
        Self(value)
    }
}
impl Into<cef_composition_underline_style_t> for CompositionUnderlineStyle {
    fn into(self) -> cef_composition_underline_style_t {
        self.0
    }
}
impl Default for CompositionUnderlineStyle {
    fn default() -> Self {
        Self(cef_composition_underline_style_t::CEF_CUS_SOLID)
    }
}

/// See [cef_channel_layout_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ChannelLayout(cef_channel_layout_t);
impl AsRef<cef_channel_layout_t> for ChannelLayout {
    fn as_ref(&self) -> &cef_channel_layout_t {
        &self.0
    }
}
impl AsMut<cef_channel_layout_t> for ChannelLayout {
    fn as_mut(&mut self) -> &mut cef_channel_layout_t {
        &mut self.0
    }
}
impl From<cef_channel_layout_t> for ChannelLayout {
    fn from(value: cef_channel_layout_t) -> Self {
        Self(value)
    }
}
impl Into<cef_channel_layout_t> for ChannelLayout {
    fn into(self) -> cef_channel_layout_t {
        self.0
    }
}
impl Default for ChannelLayout {
    fn default() -> Self {
        Self(cef_channel_layout_t::CEF_CHANNEL_LAYOUT_NONE)
    }
}

/// See [cef_media_route_create_result_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MediaRouteCreateResult(cef_media_route_create_result_t);
impl AsRef<cef_media_route_create_result_t> for MediaRouteCreateResult {
    fn as_ref(&self) -> &cef_media_route_create_result_t {
        &self.0
    }
}
impl AsMut<cef_media_route_create_result_t> for MediaRouteCreateResult {
    fn as_mut(&mut self) -> &mut cef_media_route_create_result_t {
        &mut self.0
    }
}
impl From<cef_media_route_create_result_t> for MediaRouteCreateResult {
    fn from(value: cef_media_route_create_result_t) -> Self {
        Self(value)
    }
}
impl Into<cef_media_route_create_result_t> for MediaRouteCreateResult {
    fn into(self) -> cef_media_route_create_result_t {
        self.0
    }
}
impl Default for MediaRouteCreateResult {
    fn default() -> Self {
        Self(cef_media_route_create_result_t::CEF_MRCR_UNKNOWN_ERROR)
    }
}

/// See [cef_media_route_connection_state_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MediaRouteConnectionState(cef_media_route_connection_state_t);
impl AsRef<cef_media_route_connection_state_t> for MediaRouteConnectionState {
    fn as_ref(&self) -> &cef_media_route_connection_state_t {
        &self.0
    }
}
impl AsMut<cef_media_route_connection_state_t> for MediaRouteConnectionState {
    fn as_mut(&mut self) -> &mut cef_media_route_connection_state_t {
        &mut self.0
    }
}
impl From<cef_media_route_connection_state_t> for MediaRouteConnectionState {
    fn from(value: cef_media_route_connection_state_t) -> Self {
        Self(value)
    }
}
impl Into<cef_media_route_connection_state_t> for MediaRouteConnectionState {
    fn into(self) -> cef_media_route_connection_state_t {
        self.0
    }
}
impl Default for MediaRouteConnectionState {
    fn default() -> Self {
        Self(cef_media_route_connection_state_t::CEF_MRCS_UNKNOWN)
    }
}

/// See [cef_media_sink_icon_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MediaSinkIconType(cef_media_sink_icon_type_t);
impl AsRef<cef_media_sink_icon_type_t> for MediaSinkIconType {
    fn as_ref(&self) -> &cef_media_sink_icon_type_t {
        &self.0
    }
}
impl AsMut<cef_media_sink_icon_type_t> for MediaSinkIconType {
    fn as_mut(&mut self) -> &mut cef_media_sink_icon_type_t {
        &mut self.0
    }
}
impl From<cef_media_sink_icon_type_t> for MediaSinkIconType {
    fn from(value: cef_media_sink_icon_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_media_sink_icon_type_t> for MediaSinkIconType {
    fn into(self) -> cef_media_sink_icon_type_t {
        self.0
    }
}
impl Default for MediaSinkIconType {
    fn default() -> Self {
        Self(cef_media_sink_icon_type_t::CEF_MSIT_CAST)
    }
}

/// See [cef_text_field_commands_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TextFieldCommands(cef_text_field_commands_t);
impl AsRef<cef_text_field_commands_t> for TextFieldCommands {
    fn as_ref(&self) -> &cef_text_field_commands_t {
        &self.0
    }
}
impl AsMut<cef_text_field_commands_t> for TextFieldCommands {
    fn as_mut(&mut self) -> &mut cef_text_field_commands_t {
        &mut self.0
    }
}
impl From<cef_text_field_commands_t> for TextFieldCommands {
    fn from(value: cef_text_field_commands_t) -> Self {
        Self(value)
    }
}
impl Into<cef_text_field_commands_t> for TextFieldCommands {
    fn into(self) -> cef_text_field_commands_t {
        self.0
    }
}
impl Default for TextFieldCommands {
    fn default() -> Self {
        Self(cef_text_field_commands_t::CEF_TFC_UNKNOWN)
    }
}

/// See [cef_chrome_toolbar_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ChromeToolbarType(cef_chrome_toolbar_type_t);
impl AsRef<cef_chrome_toolbar_type_t> for ChromeToolbarType {
    fn as_ref(&self) -> &cef_chrome_toolbar_type_t {
        &self.0
    }
}
impl AsMut<cef_chrome_toolbar_type_t> for ChromeToolbarType {
    fn as_mut(&mut self) -> &mut cef_chrome_toolbar_type_t {
        &mut self.0
    }
}
impl From<cef_chrome_toolbar_type_t> for ChromeToolbarType {
    fn from(value: cef_chrome_toolbar_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_chrome_toolbar_type_t> for ChromeToolbarType {
    fn into(self) -> cef_chrome_toolbar_type_t {
        self.0
    }
}
impl Default for ChromeToolbarType {
    fn default() -> Self {
        Self(cef_chrome_toolbar_type_t::CEF_CTT_UNKNOWN)
    }
}

/// See [cef_chrome_page_action_icon_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ChromePageActionIconType(cef_chrome_page_action_icon_type_t);
impl AsRef<cef_chrome_page_action_icon_type_t> for ChromePageActionIconType {
    fn as_ref(&self) -> &cef_chrome_page_action_icon_type_t {
        &self.0
    }
}
impl AsMut<cef_chrome_page_action_icon_type_t> for ChromePageActionIconType {
    fn as_mut(&mut self) -> &mut cef_chrome_page_action_icon_type_t {
        &mut self.0
    }
}
impl From<cef_chrome_page_action_icon_type_t> for ChromePageActionIconType {
    fn from(value: cef_chrome_page_action_icon_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_chrome_page_action_icon_type_t> for ChromePageActionIconType {
    fn into(self) -> cef_chrome_page_action_icon_type_t {
        self.0
    }
}
impl Default for ChromePageActionIconType {
    fn default() -> Self {
        Self(cef_chrome_page_action_icon_type_t::CEF_CPAIT_BOOKMARK_STAR)
    }
}

/// See [cef_chrome_toolbar_button_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ChromeToolbarButtonType(cef_chrome_toolbar_button_type_t);
impl AsRef<cef_chrome_toolbar_button_type_t> for ChromeToolbarButtonType {
    fn as_ref(&self) -> &cef_chrome_toolbar_button_type_t {
        &self.0
    }
}
impl AsMut<cef_chrome_toolbar_button_type_t> for ChromeToolbarButtonType {
    fn as_mut(&mut self) -> &mut cef_chrome_toolbar_button_type_t {
        &mut self.0
    }
}
impl From<cef_chrome_toolbar_button_type_t> for ChromeToolbarButtonType {
    fn from(value: cef_chrome_toolbar_button_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_chrome_toolbar_button_type_t> for ChromeToolbarButtonType {
    fn into(self) -> cef_chrome_toolbar_button_type_t {
        self.0
    }
}
impl Default for ChromeToolbarButtonType {
    fn default() -> Self {
        Self(cef_chrome_toolbar_button_type_t::CEF_CTBT_CAST)
    }
}

/// See [cef_docking_mode_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DockingMode(cef_docking_mode_t);
impl AsRef<cef_docking_mode_t> for DockingMode {
    fn as_ref(&self) -> &cef_docking_mode_t {
        &self.0
    }
}
impl AsMut<cef_docking_mode_t> for DockingMode {
    fn as_mut(&mut self) -> &mut cef_docking_mode_t {
        &mut self.0
    }
}
impl From<cef_docking_mode_t> for DockingMode {
    fn from(value: cef_docking_mode_t) -> Self {
        Self(value)
    }
}
impl Into<cef_docking_mode_t> for DockingMode {
    fn into(self) -> cef_docking_mode_t {
        self.0
    }
}
impl Default for DockingMode {
    fn default() -> Self {
        Self(cef_docking_mode_t::CEF_DOCKING_MODE_TOP_LEFT)
    }
}

/// See [cef_show_state_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ShowState(cef_show_state_t);
impl AsRef<cef_show_state_t> for ShowState {
    fn as_ref(&self) -> &cef_show_state_t {
        &self.0
    }
}
impl AsMut<cef_show_state_t> for ShowState {
    fn as_mut(&mut self) -> &mut cef_show_state_t {
        &mut self.0
    }
}
impl From<cef_show_state_t> for ShowState {
    fn from(value: cef_show_state_t) -> Self {
        Self(value)
    }
}
impl Into<cef_show_state_t> for ShowState {
    fn into(self) -> cef_show_state_t {
        self.0
    }
}
impl Default for ShowState {
    fn default() -> Self {
        Self(cef_show_state_t::CEF_SHOW_STATE_NORMAL)
    }
}

/// See [cef_touch_handle_state_flags_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TouchHandleStateFlags(cef_touch_handle_state_flags_t);
impl AsRef<cef_touch_handle_state_flags_t> for TouchHandleStateFlags {
    fn as_ref(&self) -> &cef_touch_handle_state_flags_t {
        &self.0
    }
}
impl AsMut<cef_touch_handle_state_flags_t> for TouchHandleStateFlags {
    fn as_mut(&mut self) -> &mut cef_touch_handle_state_flags_t {
        &mut self.0
    }
}
impl From<cef_touch_handle_state_flags_t> for TouchHandleStateFlags {
    fn from(value: cef_touch_handle_state_flags_t) -> Self {
        Self(value)
    }
}
impl Into<cef_touch_handle_state_flags_t> for TouchHandleStateFlags {
    fn into(self) -> cef_touch_handle_state_flags_t {
        self.0
    }
}
impl Default for TouchHandleStateFlags {
    fn default() -> Self {
        Self(cef_touch_handle_state_flags_t::CEF_THS_FLAG_NONE)
    }
}

/// See [cef_media_access_permission_types_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MediaAccessPermissionTypes(cef_media_access_permission_types_t);
impl AsRef<cef_media_access_permission_types_t> for MediaAccessPermissionTypes {
    fn as_ref(&self) -> &cef_media_access_permission_types_t {
        &self.0
    }
}
impl AsMut<cef_media_access_permission_types_t> for MediaAccessPermissionTypes {
    fn as_mut(&mut self) -> &mut cef_media_access_permission_types_t {
        &mut self.0
    }
}
impl From<cef_media_access_permission_types_t> for MediaAccessPermissionTypes {
    fn from(value: cef_media_access_permission_types_t) -> Self {
        Self(value)
    }
}
impl Into<cef_media_access_permission_types_t> for MediaAccessPermissionTypes {
    fn into(self) -> cef_media_access_permission_types_t {
        self.0
    }
}
impl Default for MediaAccessPermissionTypes {
    fn default() -> Self {
        Self(cef_media_access_permission_types_t::CEF_MEDIA_PERMISSION_NONE)
    }
}

/// See [cef_permission_request_types_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PermissionRequestTypes(cef_permission_request_types_t);
impl AsRef<cef_permission_request_types_t> for PermissionRequestTypes {
    fn as_ref(&self) -> &cef_permission_request_types_t {
        &self.0
    }
}
impl AsMut<cef_permission_request_types_t> for PermissionRequestTypes {
    fn as_mut(&mut self) -> &mut cef_permission_request_types_t {
        &mut self.0
    }
}
impl From<cef_permission_request_types_t> for PermissionRequestTypes {
    fn from(value: cef_permission_request_types_t) -> Self {
        Self(value)
    }
}
impl Into<cef_permission_request_types_t> for PermissionRequestTypes {
    fn into(self) -> cef_permission_request_types_t {
        self.0
    }
}
impl Default for PermissionRequestTypes {
    fn default() -> Self {
        Self(cef_permission_request_types_t::CEF_PERMISSION_TYPE_NONE)
    }
}

/// See [cef_permission_request_result_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PermissionRequestResult(cef_permission_request_result_t);
impl AsRef<cef_permission_request_result_t> for PermissionRequestResult {
    fn as_ref(&self) -> &cef_permission_request_result_t {
        &self.0
    }
}
impl AsMut<cef_permission_request_result_t> for PermissionRequestResult {
    fn as_mut(&mut self) -> &mut cef_permission_request_result_t {
        &mut self.0
    }
}
impl From<cef_permission_request_result_t> for PermissionRequestResult {
    fn from(value: cef_permission_request_result_t) -> Self {
        Self(value)
    }
}
impl Into<cef_permission_request_result_t> for PermissionRequestResult {
    fn into(self) -> cef_permission_request_result_t {
        self.0
    }
}
impl Default for PermissionRequestResult {
    fn default() -> Self {
        Self(cef_permission_request_result_t::CEF_PERMISSION_RESULT_ACCEPT)
    }
}

/// See [cef_test_cert_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TestCertType(cef_test_cert_type_t);
impl AsRef<cef_test_cert_type_t> for TestCertType {
    fn as_ref(&self) -> &cef_test_cert_type_t {
        &self.0
    }
}
impl AsMut<cef_test_cert_type_t> for TestCertType {
    fn as_mut(&mut self) -> &mut cef_test_cert_type_t {
        &mut self.0
    }
}
impl From<cef_test_cert_type_t> for TestCertType {
    fn from(value: cef_test_cert_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_test_cert_type_t> for TestCertType {
    fn into(self) -> cef_test_cert_type_t {
        self.0
    }
}
impl Default for TestCertType {
    fn default() -> Self {
        Self(cef_test_cert_type_t::CEF_TEST_CERT_OK_IP)
    }
}

/// See [cef_preferences_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct PreferencesType(cef_preferences_type_t);
impl AsRef<cef_preferences_type_t> for PreferencesType {
    fn as_ref(&self) -> &cef_preferences_type_t {
        &self.0
    }
}
impl AsMut<cef_preferences_type_t> for PreferencesType {
    fn as_mut(&mut self) -> &mut cef_preferences_type_t {
        &mut self.0
    }
}
impl From<cef_preferences_type_t> for PreferencesType {
    fn from(value: cef_preferences_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_preferences_type_t> for PreferencesType {
    fn into(self) -> cef_preferences_type_t {
        self.0
    }
}
impl Default for PreferencesType {
    fn default() -> Self {
        Self(cef_preferences_type_t::CEF_PREFERENCES_TYPE_GLOBAL)
    }
}

/// See [cef_download_interrupt_reason_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DownloadInterruptReason(cef_download_interrupt_reason_t);
impl AsRef<cef_download_interrupt_reason_t> for DownloadInterruptReason {
    fn as_ref(&self) -> &cef_download_interrupt_reason_t {
        &self.0
    }
}
impl AsMut<cef_download_interrupt_reason_t> for DownloadInterruptReason {
    fn as_mut(&mut self) -> &mut cef_download_interrupt_reason_t {
        &mut self.0
    }
}
impl From<cef_download_interrupt_reason_t> for DownloadInterruptReason {
    fn from(value: cef_download_interrupt_reason_t) -> Self {
        Self(value)
    }
}
impl Into<cef_download_interrupt_reason_t> for DownloadInterruptReason {
    fn into(self) -> cef_download_interrupt_reason_t {
        self.0
    }
}
impl Default for DownloadInterruptReason {
    fn default() -> Self {
        Self(cef_download_interrupt_reason_t::CEF_DOWNLOAD_INTERRUPT_REASON_NONE)
    }
}

/// See [cef_gesture_command_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct GestureCommand(cef_gesture_command_t);
impl AsRef<cef_gesture_command_t> for GestureCommand {
    fn as_ref(&self) -> &cef_gesture_command_t {
        &self.0
    }
}
impl AsMut<cef_gesture_command_t> for GestureCommand {
    fn as_mut(&mut self) -> &mut cef_gesture_command_t {
        &mut self.0
    }
}
impl From<cef_gesture_command_t> for GestureCommand {
    fn from(value: cef_gesture_command_t) -> Self {
        Self(value)
    }
}
impl Into<cef_gesture_command_t> for GestureCommand {
    fn into(self) -> cef_gesture_command_t {
        self.0
    }
}
impl Default for GestureCommand {
    fn default() -> Self {
        Self(cef_gesture_command_t::CEF_GESTURE_COMMAND_BACK)
    }
}

/// See [cef_zoom_command_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ZoomCommand(cef_zoom_command_t);
impl AsRef<cef_zoom_command_t> for ZoomCommand {
    fn as_ref(&self) -> &cef_zoom_command_t {
        &self.0
    }
}
impl AsMut<cef_zoom_command_t> for ZoomCommand {
    fn as_mut(&mut self) -> &mut cef_zoom_command_t {
        &mut self.0
    }
}
impl From<cef_zoom_command_t> for ZoomCommand {
    fn from(value: cef_zoom_command_t) -> Self {
        Self(value)
    }
}
impl Into<cef_zoom_command_t> for ZoomCommand {
    fn into(self) -> cef_zoom_command_t {
        self.0
    }
}
impl Default for ZoomCommand {
    fn default() -> Self {
        Self(cef_zoom_command_t::CEF_ZOOM_COMMAND_OUT)
    }
}

/// See [cef_color_variant_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ColorVariant(cef_color_variant_t);
impl AsRef<cef_color_variant_t> for ColorVariant {
    fn as_ref(&self) -> &cef_color_variant_t {
        &self.0
    }
}
impl AsMut<cef_color_variant_t> for ColorVariant {
    fn as_mut(&mut self) -> &mut cef_color_variant_t {
        &mut self.0
    }
}
impl From<cef_color_variant_t> for ColorVariant {
    fn from(value: cef_color_variant_t) -> Self {
        Self(value)
    }
}
impl Into<cef_color_variant_t> for ColorVariant {
    fn into(self) -> cef_color_variant_t {
        self.0
    }
}
impl Default for ColorVariant {
    fn default() -> Self {
        Self(cef_color_variant_t::CEF_COLOR_VARIANT_SYSTEM)
    }
}

/// See [cef_task_type_t] for more documentation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TaskType(cef_task_type_t);
impl AsRef<cef_task_type_t> for TaskType {
    fn as_ref(&self) -> &cef_task_type_t {
        &self.0
    }
}
impl AsMut<cef_task_type_t> for TaskType {
    fn as_mut(&mut self) -> &mut cef_task_type_t {
        &mut self.0
    }
}
impl From<cef_task_type_t> for TaskType {
    fn from(value: cef_task_type_t) -> Self {
        Self(value)
    }
}
impl Into<cef_task_type_t> for TaskType {
    fn into(self) -> cef_task_type_t {
        self.0
    }
}
impl Default for TaskType {
    fn default() -> Self {
        Self(cef_task_type_t::CEF_TASK_TYPE_UNKNOWN)
    }
}

/// See [cef_sandbox_info_create] for more documentation.
pub fn sandbox_info_create() -> *mut ::std::os::raw::c_void {
    unsafe {
        let result = cef_sandbox_info_create();
        result.as_wrapper()
    }
}

/// See [cef_sandbox_info_destroy] for more documentation.
pub fn sandbox_info_destroy(sandbox_info: *mut u8) {
    unsafe {
        let arg_sandbox_info = sandbox_info;
        let arg_sandbox_info = arg_sandbox_info as *mut _;
        let result = cef_sandbox_info_destroy(arg_sandbox_info);
        result.as_wrapper()
    }
}

/// See [cef_api_hash] for more documentation.
pub fn api_hash(
    version: ::std::os::raw::c_int,
    entry: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    unsafe {
        let (arg_version, arg_entry) = (version, entry);
        let arg_version = arg_version;
        let arg_entry = arg_entry;
        let result = cef_api_hash(arg_version, arg_entry);
        result.as_wrapper()
    }
}

/// See [cef_api_version] for more documentation.
pub fn api_version() -> ::std::os::raw::c_int {
    unsafe {
        let result = cef_api_version();
        result.as_wrapper()
    }
}

/// See [cef_string_wide_set] for more documentation.
pub fn string_wide_set(
    src: Option<&[wchar_t]>,
    output: Option<&mut CefStringWide>,
    copy: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output, arg_copy) = (src, output, copy);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_copy = arg_copy;
        let result = cef_string_wide_set(arg_src, arg_src_len, arg_output, arg_copy);
        result.as_wrapper()
    }
}

/// See [cef_string_utf8_set] for more documentation.
pub fn string_utf8_set(
    src: Option<&[::std::os::raw::c_char]>,
    output: Option<&mut CefStringUtf8>,
    copy: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output, arg_copy) = (src, output, copy);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_copy = arg_copy;
        let result = cef_string_utf8_set(arg_src, arg_src_len, arg_output, arg_copy);
        result.as_wrapper()
    }
}

/// See [cef_string_utf16_set] for more documentation.
pub fn string_utf16_set(
    src: Option<&[char16_t]>,
    output: Option<&mut CefStringUtf16>,
    copy: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output, arg_copy) = (src, output, copy);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_copy = arg_copy;
        let result = cef_string_utf16_set(arg_src, arg_src_len, arg_output, arg_copy);
        result.as_wrapper()
    }
}

/// See [cef_string_wide_clear] for more documentation.
pub fn string_wide_clear(str_: Option<&mut CefStringWide>) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_wide_clear(arg_str_);
        result.as_wrapper()
    }
}

/// See [cef_string_utf8_clear] for more documentation.
pub fn string_utf8_clear(str_: Option<&mut CefStringUtf8>) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf8_clear(arg_str_);
        result.as_wrapper()
    }
}

/// See [cef_string_utf16_clear] for more documentation.
pub fn string_utf16_clear(str_: Option<&mut CefStringUtf16>) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf16_clear(arg_str_);
        result.as_wrapper()
    }
}

/// See [cef_string_wide_cmp] for more documentation.
pub fn string_wide_cmp(
    str_1: Option<&CefStringWide>,
    str_2: Option<&CefStringWide>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_str_1, arg_str_2) = (str_1, str_2);
        let arg_str_1 = arg_str_1
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let arg_str_2 = arg_str_2
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_wide_cmp(arg_str_1, arg_str_2);
        result.as_wrapper()
    }
}

/// See [cef_string_utf8_cmp] for more documentation.
pub fn string_utf8_cmp(
    str_1: Option<&CefStringUtf8>,
    str_2: Option<&CefStringUtf8>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_str_1, arg_str_2) = (str_1, str_2);
        let arg_str_1 = arg_str_1
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let arg_str_2 = arg_str_2
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_utf8_cmp(arg_str_1, arg_str_2);
        result.as_wrapper()
    }
}

/// See [cef_string_utf16_cmp] for more documentation.
pub fn string_utf16_cmp(
    str_1: Option<&CefStringUtf16>,
    str_2: Option<&CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_str_1, arg_str_2) = (str_1, str_2);
        let arg_str_1 = arg_str_1
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let arg_str_2 = arg_str_2
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_utf16_cmp(arg_str_1, arg_str_2);
        result.as_wrapper()
    }
}

/// See [cef_string_wide_to_utf8] for more documentation.
pub fn string_wide_to_utf8(
    src: Option<&[wchar_t]>,
    output: Option<&mut CefStringUtf8>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output) = (src, output);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_wide_to_utf8(arg_src, arg_src_len, arg_output);
        result.as_wrapper()
    }
}

/// See [cef_string_utf8_to_wide] for more documentation.
pub fn string_utf8_to_wide(
    src: Option<&[::std::os::raw::c_char]>,
    output: Option<&mut CefStringWide>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output) = (src, output);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf8_to_wide(arg_src, arg_src_len, arg_output);
        result.as_wrapper()
    }
}

/// See [cef_string_wide_to_utf16] for more documentation.
pub fn string_wide_to_utf16(
    src: Option<&[wchar_t]>,
    output: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output) = (src, output);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_wide_to_utf16(arg_src, arg_src_len, arg_output);
        result.as_wrapper()
    }
}

/// See [cef_string_utf16_to_wide] for more documentation.
pub fn string_utf16_to_wide(
    src: Option<&[char16_t]>,
    output: Option<&mut CefStringWide>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output) = (src, output);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf16_to_wide(arg_src, arg_src_len, arg_output);
        result.as_wrapper()
    }
}

/// See [cef_string_utf8_to_utf16] for more documentation.
pub fn string_utf8_to_utf16(
    src: Option<&[::std::os::raw::c_char]>,
    output: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output) = (src, output);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf8_to_utf16(arg_src, arg_src_len, arg_output);
        result.as_wrapper()
    }
}

/// See [cef_string_utf16_to_utf8] for more documentation.
pub fn string_utf16_to_utf8(
    src: Option<&[char16_t]>,
    output: Option<&mut CefStringUtf8>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output) = (src, output);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf16_to_utf8(arg_src, arg_src_len, arg_output);
        result.as_wrapper()
    }
}

/// See [cef_string_ascii_to_wide] for more documentation.
pub fn string_ascii_to_wide(
    src: Option<&[::std::os::raw::c_char]>,
    output: Option<&mut CefStringWide>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output) = (src, output);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_ascii_to_wide(arg_src, arg_src_len, arg_output);
        result.as_wrapper()
    }
}

/// See [cef_string_ascii_to_utf16] for more documentation.
pub fn string_ascii_to_utf16(
    src: Option<&[::std::os::raw::c_char]>,
    output: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output) = (src, output);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_ascii_to_utf16(arg_src, arg_src_len, arg_output);
        result.as_wrapper()
    }
}

/// See [cef_string_userfree_wide_alloc] for more documentation.
pub fn string_userfree_wide_alloc() -> Option<CefStringWide> {
    unsafe {
        let result = cef_string_userfree_wide_alloc();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_string_userfree_utf8_alloc] for more documentation.
pub fn string_userfree_utf8_alloc() -> Option<CefStringUtf8> {
    unsafe {
        let result = cef_string_userfree_utf8_alloc();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_string_userfree_utf16_alloc] for more documentation.
pub fn string_userfree_utf16_alloc() -> Option<CefStringUtf16> {
    unsafe {
        let result = cef_string_userfree_utf16_alloc();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_string_userfree_wide_free] for more documentation.
pub fn string_userfree_wide_free(str_: Option<&mut CefStringWide>) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_userfree_wide_free(arg_str_);
        result.as_wrapper()
    }
}

/// See [cef_string_userfree_utf8_free] for more documentation.
pub fn string_userfree_utf8_free(str_: Option<&mut CefStringUtf8>) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_userfree_utf8_free(arg_str_);
        result.as_wrapper()
    }
}

/// See [cef_string_userfree_utf16_free] for more documentation.
pub fn string_userfree_utf16_free(str_: Option<&mut CefStringUtf16>) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_userfree_utf16_free(arg_str_);
        result.as_wrapper()
    }
}

/// See [cef_string_utf16_to_lower] for more documentation.
pub fn string_utf16_to_lower(
    src: Option<&[char16_t]>,
    output: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output) = (src, output);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf16_to_lower(arg_src, arg_src_len, arg_output);
        result.as_wrapper()
    }
}

/// See [cef_string_utf16_to_upper] for more documentation.
pub fn string_utf16_to_upper(
    src: Option<&[char16_t]>,
    output: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_src, arg_output) = (src, output);
        let arg_src_len = arg_src.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_src = arg_src
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf16_to_upper(arg_src, arg_src_len, arg_output);
        result.as_wrapper()
    }
}

/// See [cef_string_list_alloc] for more documentation.
pub fn string_list_alloc() -> Option<CefStringList> {
    unsafe {
        let result = cef_string_list_alloc();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_string_list_size] for more documentation.
pub fn string_list_size(list: Option<&mut CefStringList>) -> usize {
    unsafe {
        let arg_list = list;
        let arg_list = arg_list
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_list_size(arg_list);
        result.as_wrapper()
    }
}

/// See [cef_string_list_value] for more documentation.
pub fn string_list_value(
    list: Option<&mut CefStringList>,
    index: usize,
    value: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_list, arg_index, arg_value) = (list, index, value);
        let arg_list = arg_list
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_index = arg_index;
        let arg_value = arg_value
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_list_value(arg_list, arg_index, arg_value);
        result.as_wrapper()
    }
}

/// See [cef_string_list_append] for more documentation.
pub fn string_list_append(list: Option<&mut CefStringList>, value: Option<&CefStringUtf16>) {
    unsafe {
        let (arg_list, arg_value) = (list, value);
        let arg_list = arg_list
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_value = arg_value
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_list_append(arg_list, arg_value);
        result.as_wrapper()
    }
}

/// See [cef_string_list_clear] for more documentation.
pub fn string_list_clear(list: Option<&mut CefStringList>) {
    unsafe {
        let arg_list = list;
        let arg_list = arg_list
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_list_clear(arg_list);
        result.as_wrapper()
    }
}

/// See [cef_string_list_free] for more documentation.
pub fn string_list_free(list: Option<&mut CefStringList>) {
    unsafe {
        let arg_list = list;
        let arg_list = arg_list
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_list_free(arg_list);
        result.as_wrapper()
    }
}

/// See [cef_string_list_copy] for more documentation.
pub fn string_list_copy(list: Option<&mut CefStringList>) -> Option<CefStringList> {
    unsafe {
        let arg_list = list;
        let arg_list = arg_list
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_list_copy(arg_list);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_string_map_alloc] for more documentation.
pub fn string_map_alloc() -> Option<CefStringMap> {
    unsafe {
        let result = cef_string_map_alloc();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_string_map_size] for more documentation.
pub fn string_map_size(map: Option<&mut CefStringMap>) -> usize {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_map_size(arg_map);
        result.as_wrapper()
    }
}

/// See [cef_string_map_find] for more documentation.
pub fn string_map_find(
    map: Option<&mut CefStringMap>,
    key: Option<&CefStringUtf16>,
    value: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_key, arg_value) = (map, key, value);
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let arg_value = arg_value
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_map_find(arg_map, arg_key, arg_value);
        result.as_wrapper()
    }
}

/// See [cef_string_map_key] for more documentation.
pub fn string_map_key(
    map: Option<&mut CefStringMap>,
    index: usize,
    key: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_index, arg_key) = (map, index, key);
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_index = arg_index;
        let arg_key = arg_key
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_map_key(arg_map, arg_index, arg_key);
        result.as_wrapper()
    }
}

/// See [cef_string_map_value] for more documentation.
pub fn string_map_value(
    map: Option<&mut CefStringMap>,
    index: usize,
    value: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_index, arg_value) = (map, index, value);
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_index = arg_index;
        let arg_value = arg_value
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_map_value(arg_map, arg_index, arg_value);
        result.as_wrapper()
    }
}

/// See [cef_string_map_append] for more documentation.
pub fn string_map_append(
    map: Option<&mut CefStringMap>,
    key: Option<&CefStringUtf16>,
    value: Option<&CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_key, arg_value) = (map, key, value);
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let arg_value = arg_value
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_map_append(arg_map, arg_key, arg_value);
        result.as_wrapper()
    }
}

/// See [cef_string_map_clear] for more documentation.
pub fn string_map_clear(map: Option<&mut CefStringMap>) {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_map_clear(arg_map);
        result.as_wrapper()
    }
}

/// See [cef_string_map_free] for more documentation.
pub fn string_map_free(map: Option<&mut CefStringMap>) {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_map_free(arg_map);
        result.as_wrapper()
    }
}

/// See [cef_string_multimap_alloc] for more documentation.
pub fn string_multimap_alloc() -> Option<CefStringMultimap> {
    unsafe {
        let result = cef_string_multimap_alloc();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_string_multimap_size] for more documentation.
pub fn string_multimap_size(map: Option<&mut CefStringMultimap>) -> usize {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_multimap_size(arg_map);
        result.as_wrapper()
    }
}

/// See [cef_string_multimap_find_count] for more documentation.
pub fn string_multimap_find_count(
    map: Option<&mut CefStringMultimap>,
    key: Option<&CefStringUtf16>,
) -> usize {
    unsafe {
        let (arg_map, arg_key) = (map, key);
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let result = cef_string_multimap_find_count(arg_map, arg_key);
        result.as_wrapper()
    }
}

/// See [cef_string_multimap_enumerate] for more documentation.
pub fn string_multimap_enumerate(
    map: Option<&mut CefStringMultimap>,
    key: Option<&CefStringUtf16>,
    value_index: usize,
    value: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_key, arg_value_index, arg_value) = (map, key, value_index, value);
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let arg_value_index = arg_value_index;
        let arg_value = arg_value
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_multimap_enumerate(arg_map, arg_key, arg_value_index, arg_value);
        result.as_wrapper()
    }
}

/// See [cef_string_multimap_key] for more documentation.
pub fn string_multimap_key(
    map: Option<&mut CefStringMultimap>,
    index: usize,
    key: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_index, arg_key) = (map, index, key);
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_index = arg_index;
        let arg_key = arg_key
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_multimap_key(arg_map, arg_index, arg_key);
        result.as_wrapper()
    }
}

/// See [cef_string_multimap_value] for more documentation.
pub fn string_multimap_value(
    map: Option<&mut CefStringMultimap>,
    index: usize,
    value: Option<&mut CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_index, arg_value) = (map, index, value);
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_index = arg_index;
        let arg_value = arg_value
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_multimap_value(arg_map, arg_index, arg_value);
        result.as_wrapper()
    }
}

/// See [cef_string_multimap_append] for more documentation.
pub fn string_multimap_append(
    map: Option<&mut CefStringMultimap>,
    key: Option<&CefStringUtf16>,
    value: Option<&CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_key, arg_value) = (map, key, value);
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let arg_value = arg_value
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_multimap_append(arg_map, arg_key, arg_value);
        result.as_wrapper()
    }
}

/// See [cef_string_multimap_clear] for more documentation.
pub fn string_multimap_clear(map: Option<&mut CefStringMultimap>) {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_multimap_clear(arg_map);
        result.as_wrapper()
    }
}

/// See [cef_string_multimap_free] for more documentation.
pub fn string_multimap_free(map: Option<&mut CefStringMultimap>) {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_multimap_free(arg_map);
        result.as_wrapper()
    }
}

/// See [cef_time_to_timet] for more documentation.
pub fn time_to_timet(cef_time: Option<&Time>, time: Option<&mut time_t>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_cef_time, arg_time) = (cef_time, time);
        let arg_cef_time = arg_cef_time.cloned().map(|arg| arg.into());
        let arg_cef_time = arg_cef_time
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let arg_time = arg_time
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_to_timet(arg_cef_time, arg_time);
        result.as_wrapper()
    }
}

/// See [cef_time_from_timet] for more documentation.
pub fn time_from_timet(time: time_t, cef_time: Option<&mut Time>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_time, arg_cef_time) = (time, cef_time);
        let arg_time = arg_time;
        let mut arg_cef_time = arg_cef_time.cloned().map(|arg| arg.into());
        let arg_cef_time = arg_cef_time
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_from_timet(arg_time, arg_cef_time);
        result.as_wrapper()
    }
}

/// See [cef_time_to_doublet] for more documentation.
pub fn time_to_doublet(cef_time: Option<&Time>, time: Option<&mut f64>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_cef_time, arg_time) = (cef_time, time);
        let arg_cef_time = arg_cef_time.cloned().map(|arg| arg.into());
        let arg_cef_time = arg_cef_time
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let arg_time = arg_time
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_to_doublet(arg_cef_time, arg_time);
        result.as_wrapper()
    }
}

/// See [cef_time_from_doublet] for more documentation.
pub fn time_from_doublet(time: f64, cef_time: Option<&mut Time>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_time, arg_cef_time) = (time, cef_time);
        let arg_time = arg_time;
        let mut arg_cef_time = arg_cef_time.cloned().map(|arg| arg.into());
        let arg_cef_time = arg_cef_time
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_from_doublet(arg_time, arg_cef_time);
        result.as_wrapper()
    }
}

/// See [cef_time_now] for more documentation.
pub fn time_now(cef_time: Option<&mut Time>) -> ::std::os::raw::c_int {
    unsafe {
        let arg_cef_time = cef_time;
        let mut arg_cef_time = arg_cef_time.cloned().map(|arg| arg.into());
        let arg_cef_time = arg_cef_time
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_now(arg_cef_time);
        result.as_wrapper()
    }
}

/// See [cef_basetime_now] for more documentation.
pub fn basetime_now() -> Basetime {
    unsafe {
        let result = cef_basetime_now();
        result.as_wrapper()
    }
}

/// See [cef_time_delta] for more documentation.
pub fn time_delta(
    cef_time_1: Option<&Time>,
    cef_time_2: Option<&Time>,
    delta: Option<&mut ::std::os::raw::c_longlong>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_cef_time_1, arg_cef_time_2, arg_delta) = (cef_time_1, cef_time_2, delta);
        let arg_cef_time_1 = arg_cef_time_1.cloned().map(|arg| arg.into());
        let arg_cef_time_1 = arg_cef_time_1
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let arg_cef_time_2 = arg_cef_time_2.cloned().map(|arg| arg.into());
        let arg_cef_time_2 = arg_cef_time_2
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let arg_delta = arg_delta
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_delta(arg_cef_time_1, arg_cef_time_2, arg_delta);
        result.as_wrapper()
    }
}

/// See [cef_time_to_basetime] for more documentation.
pub fn time_to_basetime(from: Option<&Time>, to: Option<&mut Basetime>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_from, arg_to) = (from, to);
        let arg_from = arg_from.cloned().map(|arg| arg.into());
        let arg_from = arg_from
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let mut arg_to = arg_to.cloned().map(|arg| arg.into());
        let arg_to = arg_to
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_to_basetime(arg_from, arg_to);
        result.as_wrapper()
    }
}

/// See [cef_time_from_basetime] for more documentation.
pub fn time_from_basetime(from: _cef_basetime_t, to: Option<&mut Time>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_from, arg_to) = (from, to);
        let mut arg_to = arg_to.cloned().map(|arg| arg.into());
        let arg_to = arg_to
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_from_basetime(arg_from, arg_to);
        result.as_wrapper()
    }
}

/// See [cef_value_create] for more documentation.
pub fn value_create() -> Option<Value> {
    unsafe {
        let result = cef_value_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_binary_value_create] for more documentation.
pub fn binary_value_create(data: Option<&[u8]>) -> Option<BinaryValue> {
    unsafe {
        let arg_data = data;
        let arg_data_size = arg_data.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_data = arg_data
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr() as *const _)
                }
            })
            .unwrap_or(std::ptr::null());
        let result = cef_binary_value_create(arg_data, arg_data_size);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_dictionary_value_create] for more documentation.
pub fn dictionary_value_create() -> Option<DictionaryValue> {
    unsafe {
        let result = cef_dictionary_value_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_list_value_create] for more documentation.
pub fn list_value_create() -> Option<ListValue> {
    unsafe {
        let result = cef_list_value_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_image_create] for more documentation.
pub fn image_create() -> Option<Image> {
    unsafe {
        let result = cef_image_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_stream_reader_create_for_file] for more documentation.
pub fn stream_reader_create_for_file(file_name: Option<&CefStringUtf16>) -> Option<StreamReader> {
    unsafe {
        let arg_file_name = file_name;
        let arg_file_name = arg_file_name
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_stream_reader_create_for_file(arg_file_name);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_stream_reader_create_for_data] for more documentation.
pub fn stream_reader_create_for_data(data: *mut u8, size: usize) -> Option<StreamReader> {
    unsafe {
        let (arg_data, arg_size) = (data, size);
        let arg_data = arg_data as *mut _;
        let arg_size = arg_size;
        let result = cef_stream_reader_create_for_data(arg_data, arg_size);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_stream_reader_create_for_handler] for more documentation.
pub fn stream_reader_create_for_handler(handler: Option<&mut ReadHandler>) -> Option<StreamReader> {
    unsafe {
        let arg_handler = handler;
        let mut arg_handler = arg_handler.cloned().map(|arg| arg.into());
        let arg_handler = arg_handler
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_stream_reader_create_for_handler(arg_handler);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_stream_writer_create_for_file] for more documentation.
pub fn stream_writer_create_for_file(file_name: Option<&CefStringUtf16>) -> Option<StreamWriter> {
    unsafe {
        let arg_file_name = file_name;
        let arg_file_name = arg_file_name
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_stream_writer_create_for_file(arg_file_name);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_stream_writer_create_for_handler] for more documentation.
pub fn stream_writer_create_for_handler(
    handler: Option<&mut WriteHandler>,
) -> Option<StreamWriter> {
    unsafe {
        let arg_handler = handler;
        let mut arg_handler = arg_handler.cloned().map(|arg| arg.into());
        let arg_handler = arg_handler
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_stream_writer_create_for_handler(arg_handler);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_drag_data_create] for more documentation.
pub fn drag_data_create() -> Option<DragData> {
    unsafe {
        let result = cef_drag_data_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_process_message_create] for more documentation.
pub fn process_message_create(name: Option<&CefStringUtf16>) -> Option<ProcessMessage> {
    unsafe {
        let arg_name = name;
        let arg_name = arg_name.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let result = cef_process_message_create(arg_name);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_request_create] for more documentation.
pub fn request_create() -> Option<Request> {
    unsafe {
        let result = cef_request_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_post_data_create] for more documentation.
pub fn post_data_create() -> Option<PostData> {
    unsafe {
        let result = cef_post_data_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_post_data_element_create] for more documentation.
pub fn post_data_element_create() -> Option<PostDataElement> {
    unsafe {
        let result = cef_post_data_element_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_cookie_manager_get_global_manager] for more documentation.
pub fn cookie_manager_get_global_manager(
    callback: Option<&mut CompletionCallback>,
) -> Option<CookieManager> {
    unsafe {
        let arg_callback = callback;
        let mut arg_callback = arg_callback.cloned().map(|arg| arg.into());
        let arg_callback = arg_callback
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_cookie_manager_get_global_manager(arg_callback);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_media_router_get_global] for more documentation.
pub fn media_router_get_global(callback: Option<&mut CompletionCallback>) -> Option<MediaRouter> {
    unsafe {
        let arg_callback = callback;
        let mut arg_callback = arg_callback.cloned().map(|arg| arg.into());
        let arg_callback = arg_callback
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_media_router_get_global(arg_callback);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_preference_manager_get_chrome_variations_as_switches] for more documentation.
pub fn preference_manager_get_chrome_variations_as_switches(switches: Option<&mut CefStringList>) {
    unsafe {
        let arg_switches = switches;
        let arg_switches = arg_switches
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_preference_manager_get_chrome_variations_as_switches(arg_switches);
        result.as_wrapper()
    }
}

/// See [cef_preference_manager_get_chrome_variations_as_strings] for more documentation.
pub fn preference_manager_get_chrome_variations_as_strings(strings: Option<&mut CefStringList>) {
    unsafe {
        let arg_strings = strings;
        let arg_strings = arg_strings
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_preference_manager_get_chrome_variations_as_strings(arg_strings);
        result.as_wrapper()
    }
}

/// See [cef_preference_manager_get_global] for more documentation.
pub fn preference_manager_get_global() -> Option<PreferenceManager> {
    unsafe {
        let result = cef_preference_manager_get_global();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_request_context_get_global_context] for more documentation.
pub fn request_context_get_global_context() -> Option<RequestContext> {
    unsafe {
        let result = cef_request_context_get_global_context();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_request_context_create_context] for more documentation.
pub fn request_context_create_context(
    settings: Option<&RequestContextSettings>,
    handler: Option<&mut RequestContextHandler>,
) -> Option<RequestContext> {
    unsafe {
        let (arg_settings, arg_handler) = (settings, handler);
        let arg_settings = arg_settings.cloned().map(|arg| arg.into());
        let arg_settings = arg_settings
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let mut arg_handler = arg_handler.cloned().map(|arg| arg.into());
        let arg_handler = arg_handler
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_request_context_create_context(arg_settings, arg_handler);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_request_context_cef_create_context_shared] for more documentation.
pub fn request_context_cef_create_context_shared(
    other: Option<&mut RequestContext>,
    handler: Option<&mut RequestContextHandler>,
) -> Option<RequestContext> {
    unsafe {
        let (arg_other, arg_handler) = (other, handler);
        let mut arg_other = arg_other.cloned().map(|arg| arg.into());
        let arg_other = arg_other
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let mut arg_handler = arg_handler.cloned().map(|arg| arg.into());
        let arg_handler = arg_handler
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_request_context_cef_create_context_shared(arg_other, arg_handler);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_browser_host_create_browser] for more documentation.
pub fn browser_host_create_browser(
    window_info: Option<&WindowInfo>,
    client: Option<&mut Client>,
    url: Option<&CefStringUtf16>,
    settings: Option<&BrowserSettings>,
    extra_info: Option<&mut DictionaryValue>,
    request_context: Option<&mut RequestContext>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (
            arg_window_info,
            arg_client,
            arg_url,
            arg_settings,
            arg_extra_info,
            arg_request_context,
        ) = (
            window_info,
            client,
            url,
            settings,
            extra_info,
            request_context,
        );
        let arg_window_info = arg_window_info.cloned().map(|arg| arg.into());
        let arg_window_info = arg_window_info
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let mut arg_client = arg_client.cloned().map(|arg| arg.into());
        let arg_client = arg_client
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let arg_url = arg_url.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let arg_settings = arg_settings.cloned().map(|arg| arg.into());
        let arg_settings = arg_settings
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let mut arg_extra_info = arg_extra_info.cloned().map(|arg| arg.into());
        let arg_extra_info = arg_extra_info
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let mut arg_request_context = arg_request_context.cloned().map(|arg| arg.into());
        let arg_request_context = arg_request_context
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_browser_host_create_browser(
            arg_window_info,
            arg_client,
            arg_url,
            arg_settings,
            arg_extra_info,
            arg_request_context,
        );
        result.as_wrapper()
    }
}

/// See [cef_browser_host_create_browser_sync] for more documentation.
pub fn browser_host_create_browser_sync(
    window_info: Option<&WindowInfo>,
    client: Option<&mut Client>,
    url: Option<&CefStringUtf16>,
    settings: Option<&BrowserSettings>,
    extra_info: Option<&mut DictionaryValue>,
    request_context: Option<&mut RequestContext>,
) -> Option<Browser> {
    unsafe {
        let (
            arg_window_info,
            arg_client,
            arg_url,
            arg_settings,
            arg_extra_info,
            arg_request_context,
        ) = (
            window_info,
            client,
            url,
            settings,
            extra_info,
            request_context,
        );
        let arg_window_info = arg_window_info.cloned().map(|arg| arg.into());
        let arg_window_info = arg_window_info
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let mut arg_client = arg_client.cloned().map(|arg| arg.into());
        let arg_client = arg_client
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let arg_url = arg_url.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let arg_settings = arg_settings.cloned().map(|arg| arg.into());
        let arg_settings = arg_settings
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let mut arg_extra_info = arg_extra_info.cloned().map(|arg| arg.into());
        let arg_extra_info = arg_extra_info
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let mut arg_request_context = arg_request_context.cloned().map(|arg| arg.into());
        let arg_request_context = arg_request_context
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_browser_host_create_browser_sync(
            arg_window_info,
            arg_client,
            arg_url,
            arg_settings,
            arg_extra_info,
            arg_request_context,
        );
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_browser_host_get_browser_by_identifier] for more documentation.
pub fn browser_host_get_browser_by_identifier(
    browser_id: ::std::os::raw::c_int,
) -> Option<Browser> {
    unsafe {
        let arg_browser_id = browser_id;
        let arg_browser_id = arg_browser_id;
        let result = cef_browser_host_get_browser_by_identifier(arg_browser_id);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_menu_model_create] for more documentation.
pub fn menu_model_create(delegate: Option<&mut MenuModelDelegate>) -> Option<MenuModel> {
    unsafe {
        let arg_delegate = delegate;
        let mut arg_delegate = arg_delegate.cloned().map(|arg| arg.into());
        let arg_delegate = arg_delegate
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_menu_model_create(arg_delegate);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_print_settings_create] for more documentation.
pub fn print_settings_create() -> Option<PrintSettings> {
    unsafe {
        let result = cef_print_settings_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_response_create] for more documentation.
pub fn response_create() -> Option<Response> {
    unsafe {
        let result = cef_response_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_is_cert_status_error] for more documentation.
pub fn is_cert_status_error(status: CertStatus) -> ::std::os::raw::c_int {
    unsafe {
        let arg_status = status;
        let arg_status = arg_status.as_raw();
        let result = cef_is_cert_status_error(arg_status);
        result.as_wrapper()
    }
}

/// See [cef_command_line_create] for more documentation.
pub fn command_line_create() -> Option<CommandLine> {
    unsafe {
        let result = cef_command_line_create();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_command_line_get_global] for more documentation.
pub fn command_line_get_global() -> Option<CommandLine> {
    unsafe {
        let result = cef_command_line_get_global();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_task_runner_get_for_current_thread] for more documentation.
pub fn task_runner_get_for_current_thread() -> Option<TaskRunner> {
    unsafe {
        let result = cef_task_runner_get_for_current_thread();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_task_runner_get_for_thread] for more documentation.
pub fn task_runner_get_for_thread(thread_id: ThreadId) -> Option<TaskRunner> {
    unsafe {
        let arg_thread_id = thread_id;
        let arg_thread_id = arg_thread_id.as_raw();
        let result = cef_task_runner_get_for_thread(arg_thread_id);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_currently_on] for more documentation.
pub fn currently_on(thread_id: ThreadId) -> ::std::os::raw::c_int {
    unsafe {
        let arg_thread_id = thread_id;
        let arg_thread_id = arg_thread_id.as_raw();
        let result = cef_currently_on(arg_thread_id);
        result.as_wrapper()
    }
}

/// See [cef_post_task] for more documentation.
pub fn post_task(thread_id: ThreadId, task: Option<&mut Task>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_thread_id, arg_task) = (thread_id, task);
        let arg_thread_id = arg_thread_id.as_raw();
        let mut arg_task = arg_task.cloned().map(|arg| arg.into());
        let arg_task = arg_task
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_post_task(arg_thread_id, arg_task);
        result.as_wrapper()
    }
}

/// See [cef_post_delayed_task] for more documentation.
pub fn post_delayed_task(
    thread_id: ThreadId,
    task: Option<&mut Task>,
    delay_ms: i64,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_thread_id, arg_task, arg_delay_ms) = (thread_id, task, delay_ms);
        let arg_thread_id = arg_thread_id.as_raw();
        let mut arg_task = arg_task.cloned().map(|arg| arg.into());
        let arg_task = arg_task
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let arg_delay_ms = arg_delay_ms;
        let result = cef_post_delayed_task(arg_thread_id, arg_task, arg_delay_ms);
        result.as_wrapper()
    }
}

/// See [cef_v8_context_get_current_context] for more documentation.
pub fn v8_context_get_current_context() -> Option<V8Context> {
    unsafe {
        let result = cef_v8_context_get_current_context();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_context_get_entered_context] for more documentation.
pub fn v8_context_get_entered_context() -> Option<V8Context> {
    unsafe {
        let result = cef_v8_context_get_entered_context();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_context_in_context] for more documentation.
pub fn v8_context_in_context() -> ::std::os::raw::c_int {
    unsafe {
        let result = cef_v8_context_in_context();
        result.as_wrapper()
    }
}

/// See [cef_v8_value_create_undefined] for more documentation.
pub fn v8_value_create_undefined() -> Option<V8Value> {
    unsafe {
        let result = cef_v8_value_create_undefined();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_null] for more documentation.
pub fn v8_value_create_null() -> Option<V8Value> {
    unsafe {
        let result = cef_v8_value_create_null();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_bool] for more documentation.
pub fn v8_value_create_bool(value: ::std::os::raw::c_int) -> Option<V8Value> {
    unsafe {
        let arg_value = value;
        let arg_value = arg_value;
        let result = cef_v8_value_create_bool(arg_value);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_int] for more documentation.
pub fn v8_value_create_int(value: i32) -> Option<V8Value> {
    unsafe {
        let arg_value = value;
        let arg_value = arg_value;
        let result = cef_v8_value_create_int(arg_value);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_uint] for more documentation.
pub fn v8_value_create_uint(value: u32) -> Option<V8Value> {
    unsafe {
        let arg_value = value;
        let arg_value = arg_value;
        let result = cef_v8_value_create_uint(arg_value);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_double] for more documentation.
pub fn v8_value_create_double(value: f64) -> Option<V8Value> {
    unsafe {
        let arg_value = value;
        let arg_value = arg_value;
        let result = cef_v8_value_create_double(arg_value);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_date] for more documentation.
pub fn v8_value_create_date(date: _cef_basetime_t) -> Option<V8Value> {
    unsafe {
        let arg_date = date;
        let result = cef_v8_value_create_date(arg_date);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_string] for more documentation.
pub fn v8_value_create_string(value: Option<&CefStringUtf16>) -> Option<V8Value> {
    unsafe {
        let arg_value = value;
        let arg_value = arg_value
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_v8_value_create_string(arg_value);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_object] for more documentation.
pub fn v8_value_create_object(
    accessor: Option<&mut V8Accessor>,
    interceptor: Option<&mut V8Interceptor>,
) -> Option<V8Value> {
    unsafe {
        let (arg_accessor, arg_interceptor) = (accessor, interceptor);
        let mut arg_accessor = arg_accessor.cloned().map(|arg| arg.into());
        let arg_accessor = arg_accessor
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let mut arg_interceptor = arg_interceptor.cloned().map(|arg| arg.into());
        let arg_interceptor = arg_interceptor
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_v8_value_create_object(arg_accessor, arg_interceptor);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_array] for more documentation.
pub fn v8_value_create_array(length: ::std::os::raw::c_int) -> Option<V8Value> {
    unsafe {
        let arg_length = length;
        let arg_length = arg_length;
        let result = cef_v8_value_create_array(arg_length);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_array_buffer] for more documentation.
pub fn v8_value_create_array_buffer(
    buffer: *mut u8,
    length: usize,
    release_callback: Option<&mut V8ArrayBufferReleaseCallback>,
) -> Option<V8Value> {
    unsafe {
        let (arg_buffer, arg_length, arg_release_callback) = (buffer, length, release_callback);
        let arg_buffer = arg_buffer as *mut _;
        let arg_length = arg_length;
        let mut arg_release_callback = arg_release_callback.cloned().map(|arg| arg.into());
        let arg_release_callback = arg_release_callback
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_v8_value_create_array_buffer(arg_buffer, arg_length, arg_release_callback);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_array_buffer_with_copy] for more documentation.
pub fn v8_value_create_array_buffer_with_copy(buffer: *mut u8, length: usize) -> Option<V8Value> {
    unsafe {
        let (arg_buffer, arg_length) = (buffer, length);
        let arg_buffer = arg_buffer as *mut _;
        let arg_length = arg_length;
        let result = cef_v8_value_create_array_buffer_with_copy(arg_buffer, arg_length);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_function] for more documentation.
pub fn v8_value_create_function(
    name: Option<&CefStringUtf16>,
    handler: Option<&mut V8Handler>,
) -> Option<V8Value> {
    unsafe {
        let (arg_name, arg_handler) = (name, handler);
        let arg_name = arg_name.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let mut arg_handler = arg_handler.cloned().map(|arg| arg.into());
        let arg_handler = arg_handler
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_v8_value_create_function(arg_name, arg_handler);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_value_create_promise] for more documentation.
pub fn v8_value_create_promise() -> Option<V8Value> {
    unsafe {
        let result = cef_v8_value_create_promise();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_v8_stack_trace_get_current] for more documentation.
pub fn v8_stack_trace_get_current(frame_limit: ::std::os::raw::c_int) -> Option<V8StackTrace> {
    unsafe {
        let arg_frame_limit = frame_limit;
        let arg_frame_limit = arg_frame_limit;
        let result = cef_v8_stack_trace_get_current(arg_frame_limit);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_register_extension] for more documentation.
pub fn register_extension(
    extension_name: Option<&CefStringUtf16>,
    javascript_code: Option<&CefStringUtf16>,
    handler: Option<&mut V8Handler>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_extension_name, arg_javascript_code, arg_handler) =
            (extension_name, javascript_code, handler);
        let arg_extension_name = arg_extension_name
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let arg_javascript_code = arg_javascript_code
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let mut arg_handler = arg_handler.cloned().map(|arg| arg.into());
        let arg_handler = arg_handler
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_register_extension(arg_extension_name, arg_javascript_code, arg_handler);
        result.as_wrapper()
    }
}

/// See [cef_register_scheme_handler_factory] for more documentation.
pub fn register_scheme_handler_factory(
    scheme_name: Option<&CefStringUtf16>,
    domain_name: Option<&CefStringUtf16>,
    factory: Option<&mut SchemeHandlerFactory>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_scheme_name, arg_domain_name, arg_factory) = (scheme_name, domain_name, factory);
        let arg_scheme_name = arg_scheme_name
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let arg_domain_name = arg_domain_name
            .map(|arg| arg.as_raw())
            .unwrap_or(std::ptr::null());
        let mut arg_factory = arg_factory.cloned().map(|arg| arg.into());
        let arg_factory = arg_factory
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result =
            cef_register_scheme_handler_factory(arg_scheme_name, arg_domain_name, arg_factory);
        result.as_wrapper()
    }
}

/// See [cef_clear_scheme_handler_factories] for more documentation.
pub fn clear_scheme_handler_factories() -> ::std::os::raw::c_int {
    unsafe {
        let result = cef_clear_scheme_handler_factories();
        result.as_wrapper()
    }
}

/// See [cef_execute_process] for more documentation.
pub fn execute_process(
    args: Option<&MainArgs>,
    application: Option<&mut App>,
    windows_sandbox_info: *mut u8,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_args, arg_application, arg_windows_sandbox_info) =
            (args, application, windows_sandbox_info);
        let arg_args = arg_args.cloned().map(|arg| arg.into());
        let arg_args = arg_args
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let mut arg_application = arg_application.cloned().map(|arg| arg.into());
        let arg_application = arg_application
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let arg_windows_sandbox_info = arg_windows_sandbox_info as *mut _;
        let result = cef_execute_process(arg_args, arg_application, arg_windows_sandbox_info);
        result.as_wrapper()
    }
}

/// See [cef_initialize] for more documentation.
pub fn initialize(
    args: Option<&MainArgs>,
    settings: Option<&Settings>,
    application: Option<&mut App>,
    windows_sandbox_info: *mut u8,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_args, arg_settings, arg_application, arg_windows_sandbox_info) =
            (args, settings, application, windows_sandbox_info);
        let arg_args = arg_args.cloned().map(|arg| arg.into());
        let arg_args = arg_args
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let arg_settings = arg_settings.cloned().map(|arg| arg.into());
        let arg_settings = arg_settings
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let mut arg_application = arg_application.cloned().map(|arg| arg.into());
        let arg_application = arg_application
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let arg_windows_sandbox_info = arg_windows_sandbox_info as *mut _;
        let result = cef_initialize(
            arg_args,
            arg_settings,
            arg_application,
            arg_windows_sandbox_info,
        );
        result.as_wrapper()
    }
}

/// See [cef_get_exit_code] for more documentation.
pub fn get_exit_code() -> ::std::os::raw::c_int {
    unsafe {
        let result = cef_get_exit_code();
        result.as_wrapper()
    }
}

/// See [cef_shutdown] for more documentation.
pub fn shutdown() {
    unsafe {
        let result = cef_shutdown();
        result.as_wrapper()
    }
}

/// See [cef_do_message_loop_work] for more documentation.
pub fn do_message_loop_work() {
    unsafe {
        let result = cef_do_message_loop_work();
        result.as_wrapper()
    }
}

/// See [cef_run_message_loop] for more documentation.
pub fn run_message_loop() {
    unsafe {
        let result = cef_run_message_loop();
        result.as_wrapper()
    }
}

/// See [cef_quit_message_loop] for more documentation.
pub fn quit_message_loop() {
    unsafe {
        let result = cef_quit_message_loop();
        result.as_wrapper()
    }
}

/// See [cef_urlrequest_create] for more documentation.
pub fn urlrequest_create(
    request: Option<&mut Request>,
    client: Option<&mut UrlrequestClient>,
    request_context: Option<&mut RequestContext>,
) -> Option<Urlrequest> {
    unsafe {
        let (arg_request, arg_client, arg_request_context) = (request, client, request_context);
        let mut arg_request = arg_request.cloned().map(|arg| arg.into());
        let arg_request = arg_request
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let mut arg_client = arg_client.cloned().map(|arg| arg.into());
        let arg_client = arg_client
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let mut arg_request_context = arg_request_context.cloned().map(|arg| arg.into());
        let arg_request_context = arg_request_context
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_urlrequest_create(arg_request, arg_client, arg_request_context);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_label_button_create] for more documentation.
pub fn label_button_create(
    delegate: Option<&mut ButtonDelegate>,
    text: Option<&CefStringUtf16>,
) -> Option<LabelButton> {
    unsafe {
        let (arg_delegate, arg_text) = (delegate, text);
        let mut arg_delegate = arg_delegate.cloned().map(|arg| arg.into());
        let arg_delegate = arg_delegate
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let arg_text = arg_text.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let result = cef_label_button_create(arg_delegate, arg_text);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_menu_button_create] for more documentation.
pub fn menu_button_create(
    delegate: Option<&mut MenuButtonDelegate>,
    text: Option<&CefStringUtf16>,
) -> Option<MenuButton> {
    unsafe {
        let (arg_delegate, arg_text) = (delegate, text);
        let mut arg_delegate = arg_delegate.cloned().map(|arg| arg.into());
        let arg_delegate = arg_delegate
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let arg_text = arg_text.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let result = cef_menu_button_create(arg_delegate, arg_text);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_textfield_create] for more documentation.
pub fn textfield_create(delegate: Option<&mut TextfieldDelegate>) -> Option<Textfield> {
    unsafe {
        let arg_delegate = delegate;
        let mut arg_delegate = arg_delegate.cloned().map(|arg| arg.into());
        let arg_delegate = arg_delegate
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_textfield_create(arg_delegate);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_browser_view_create] for more documentation.
pub fn browser_view_create(
    client: Option<&mut Client>,
    url: Option<&CefStringUtf16>,
    settings: Option<&BrowserSettings>,
    extra_info: Option<&mut DictionaryValue>,
    request_context: Option<&mut RequestContext>,
    delegate: Option<&mut BrowserViewDelegate>,
) -> Option<BrowserView> {
    unsafe {
        let (arg_client, arg_url, arg_settings, arg_extra_info, arg_request_context, arg_delegate) =
            (client, url, settings, extra_info, request_context, delegate);
        let mut arg_client = arg_client.cloned().map(|arg| arg.into());
        let arg_client = arg_client
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let arg_url = arg_url.map(|arg| arg.as_raw()).unwrap_or(std::ptr::null());
        let arg_settings = arg_settings.cloned().map(|arg| arg.into());
        let arg_settings = arg_settings
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let mut arg_extra_info = arg_extra_info.cloned().map(|arg| arg.into());
        let arg_extra_info = arg_extra_info
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let mut arg_request_context = arg_request_context.cloned().map(|arg| arg.into());
        let arg_request_context = arg_request_context
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let mut arg_delegate = arg_delegate.cloned().map(|arg| arg.into());
        let arg_delegate = arg_delegate
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_browser_view_create(
            arg_client,
            arg_url,
            arg_settings,
            arg_extra_info,
            arg_request_context,
            arg_delegate,
        );
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_browser_view_get_for_browser] for more documentation.
pub fn browser_view_get_for_browser(browser: Option<&mut Browser>) -> Option<BrowserView> {
    unsafe {
        let arg_browser = browser;
        let mut arg_browser = arg_browser.cloned().map(|arg| arg.into());
        let arg_browser = arg_browser
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_browser_view_get_for_browser(arg_browser);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_scroll_view_create] for more documentation.
pub fn scroll_view_create(delegate: Option<&mut ViewDelegate>) -> Option<ScrollView> {
    unsafe {
        let arg_delegate = delegate;
        let mut arg_delegate = arg_delegate.cloned().map(|arg| arg.into());
        let arg_delegate = arg_delegate
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_scroll_view_create(arg_delegate);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_display_get_primary] for more documentation.
pub fn display_get_primary() -> Option<Display> {
    unsafe {
        let result = cef_display_get_primary();
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_display_get_nearest_point] for more documentation.
pub fn display_get_nearest_point(
    point: Option<&Point>,
    input_pixel_coords: ::std::os::raw::c_int,
) -> Option<Display> {
    unsafe {
        let (arg_point, arg_input_pixel_coords) = (point, input_pixel_coords);
        let arg_point = arg_point.cloned().map(|arg| arg.into());
        let arg_point = arg_point
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let arg_input_pixel_coords = arg_input_pixel_coords;
        let result = cef_display_get_nearest_point(arg_point, arg_input_pixel_coords);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_display_get_matching_bounds] for more documentation.
pub fn display_get_matching_bounds(
    bounds: Option<&Rect>,
    input_pixel_coords: ::std::os::raw::c_int,
) -> Option<Display> {
    unsafe {
        let (arg_bounds, arg_input_pixel_coords) = (bounds, input_pixel_coords);
        let arg_bounds = arg_bounds.cloned().map(|arg| arg.into());
        let arg_bounds = arg_bounds
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let arg_input_pixel_coords = arg_input_pixel_coords;
        let result = cef_display_get_matching_bounds(arg_bounds, arg_input_pixel_coords);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_display_get_count] for more documentation.
pub fn display_get_count() -> usize {
    unsafe {
        let result = cef_display_get_count();
        result.as_wrapper()
    }
}

/// See [cef_display_get_alls] for more documentation.
pub fn display_get_alls(displays: Option<&mut Vec<Option<Display>>>) {
    unsafe {
        let arg_displays = displays;
        let mut out_displays_count = arg_displays
            .as_ref()
            .map(|arg| arg.len())
            .unwrap_or_default();
        let arg_displays_count = &mut out_displays_count;
        let out_displays = arg_displays;
        let mut vec_displays = out_displays
            .as_ref()
            .map(|arg| {
                arg.iter()
                    .map(|elem| {
                        elem.as_ref()
                            .map(|elem| elem.get_raw())
                            .unwrap_or(std::ptr::null_mut())
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let arg_displays = if vec_displays.is_empty() {
            std::ptr::null_mut()
        } else {
            vec_displays.as_mut_ptr()
        };
        let result = cef_display_get_alls(arg_displays_count, arg_displays);
        if let Some(out_displays) = out_displays {
            *out_displays = vec_displays
                .into_iter()
                .take(out_displays_count)
                .map(|elem| {
                    if elem.is_null() {
                        None
                    } else {
                        Some(elem.as_wrapper())
                    }
                })
                .collect();
        }
        result.as_wrapper()
    }
}

/// See [cef_display_convert_screen_point_to_pixels] for more documentation.
pub fn display_convert_screen_point_to_pixels(point: Option<&Point>) -> Point {
    unsafe {
        let arg_point = point;
        let arg_point = arg_point.cloned().map(|arg| arg.into());
        let arg_point = arg_point
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let result = cef_display_convert_screen_point_to_pixels(arg_point);
        result.as_wrapper()
    }
}

/// See [cef_display_convert_screen_point_from_pixels] for more documentation.
pub fn display_convert_screen_point_from_pixels(point: Option<&Point>) -> Point {
    unsafe {
        let arg_point = point;
        let arg_point = arg_point.cloned().map(|arg| arg.into());
        let arg_point = arg_point
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let result = cef_display_convert_screen_point_from_pixels(arg_point);
        result.as_wrapper()
    }
}

/// See [cef_display_convert_screen_rect_to_pixels] for more documentation.
pub fn display_convert_screen_rect_to_pixels(rect: Option<&Rect>) -> Rect {
    unsafe {
        let arg_rect = rect;
        let arg_rect = arg_rect.cloned().map(|arg| arg.into());
        let arg_rect = arg_rect
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let result = cef_display_convert_screen_rect_to_pixels(arg_rect);
        result.as_wrapper()
    }
}

/// See [cef_display_convert_screen_rect_from_pixels] for more documentation.
pub fn display_convert_screen_rect_from_pixels(rect: Option<&Rect>) -> Rect {
    unsafe {
        let arg_rect = rect;
        let arg_rect = arg_rect.cloned().map(|arg| arg.into());
        let arg_rect = arg_rect
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let result = cef_display_convert_screen_rect_from_pixels(arg_rect);
        result.as_wrapper()
    }
}

/// See [cef_panel_create] for more documentation.
pub fn panel_create(delegate: Option<&mut PanelDelegate>) -> Option<Panel> {
    unsafe {
        let arg_delegate = delegate;
        let mut arg_delegate = arg_delegate.cloned().map(|arg| arg.into());
        let arg_delegate = arg_delegate
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_panel_create(arg_delegate);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}

/// See [cef_window_create_top_level] for more documentation.
pub fn window_create_top_level(delegate: Option<&mut WindowDelegate>) -> Option<Window> {
    unsafe {
        let arg_delegate = delegate;
        let mut arg_delegate = arg_delegate.cloned().map(|arg| arg.into());
        let arg_delegate = arg_delegate
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_window_create_top_level(arg_delegate);
        if result.is_null() {
            None
        } else {
            Some(result.as_wrapper())
        }
    }
}
