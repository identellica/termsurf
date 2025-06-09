#![allow(
    dead_code,
    improper_ctypes_definitions,
    non_camel_case_types,
    unused_variables,
    clippy::not_unsafe_ptr_arg_deref,
    clippy::too_many_arguments
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

/// See [`cef_string_wide_t`] for more documentation.
pub use crate::string::CefStringUserfreeWide;

/// See [`cef_string_utf8_t`] for more documentation.
pub use crate::string::CefStringUserfreeUtf8;

/// See [`cef_string_utf16_t`] for more documentation.
pub use crate::string::CefStringUserfreeUtf16;

/// See [`char16_t`] for more documentation.
pub type Char = char16_t;

/// See [`cef_string_userfree_utf16_t`] for more documentation.
pub type CefStringUserfree = CefStringUserfreeUtf16;

/// See [`cef_string_utf16_t`] for more documentation.
pub type CefString = CefStringUtf16;

/// See [`HCURSOR`] for more documentation.
pub type CursorHandle = HCURSOR;

/// See [`MSG`] for more documentation.
pub type EventHandle = *mut MSG;

/// See [`HWND`] for more documentation.
pub type WindowHandle = HWND;

/// See [`HANDLE`] for more documentation.
pub type SharedTextureHandle = HANDLE;

/// See [`u32`] for more documentation.
pub type Color = u32;

/// See [`_cef_string_wide_t`] for more documentation.
pub use crate::string::CefStringWide;

/// See [`_cef_string_utf8_t`] for more documentation.
pub use crate::string::CefStringUtf8;

/// See [`_cef_string_utf16_t`] for more documentation.
pub use crate::string::CefStringUtf16;

/// See [`_cef_string_list_t`] for more documentation.
pub use crate::string::CefStringList;

/// See [`_cef_string_map_t`] for more documentation.
pub use crate::string::CefStringMap;

/// See [`_cef_string_multimap_t`] for more documentation.
pub use crate::string::CefStringMultimap;

/// See [`_cef_basetime_t`] for more documentation.
#[derive(Clone)]
pub struct Basetime {
    pub val: i64,
}
impl From<_cef_basetime_t> for Basetime {
    fn from(value: _cef_basetime_t) -> Self {
        Self { val: value.val }
    }
}
impl From<Basetime> for _cef_basetime_t {
    fn from(value: Basetime) -> Self {
        Self { val: value.val }
    }
}
impl Default for Basetime {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_time_t`] for more documentation.
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
            year: value.year,
            month: value.month,
            day_of_week: value.day_of_week,
            day_of_month: value.day_of_month,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
            millisecond: value.millisecond,
        }
    }
}
impl From<Time> for _cef_time_t {
    fn from(value: Time) -> Self {
        Self {
            year: value.year,
            month: value.month,
            day_of_week: value.day_of_week,
            day_of_month: value.day_of_month,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
            millisecond: value.millisecond,
        }
    }
}
impl Default for Time {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_point_t`] for more documentation.
#[derive(Clone)]
pub struct Point {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
impl From<_cef_point_t> for Point {
    fn from(value: _cef_point_t) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<Point> for _cef_point_t {
    fn from(value: Point) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
impl Default for Point {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_rect_t`] for more documentation.
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
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}
impl From<Rect> for _cef_rect_t {
    fn from(value: Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}
impl Default for Rect {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_size_t`] for more documentation.
#[derive(Clone)]
pub struct Size {
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}
impl From<_cef_size_t> for Size {
    fn from(value: _cef_size_t) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}
impl From<Size> for _cef_size_t {
    fn from(value: Size) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}
impl Default for Size {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_insets_t`] for more documentation.
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
            top: value.top,
            left: value.left,
            bottom: value.bottom,
            right: value.right,
        }
    }
}
impl From<Insets> for _cef_insets_t {
    fn from(value: Insets) -> Self {
        Self {
            top: value.top,
            left: value.left,
            bottom: value.bottom,
            right: value.right,
        }
    }
}
impl Default for Insets {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_accelerated_paint_info_common_t`] for more documentation.
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
            size: value.size,
            timestamp: value.timestamp,
            coded_size: value.coded_size.into(),
            visible_rect: value.visible_rect.into(),
            content_rect: value.content_rect.into(),
            source_size: value.source_size.into(),
            capture_update_rect: value.capture_update_rect.into(),
            region_capture_rect: value.region_capture_rect.into(),
            capture_counter: value.capture_counter,
            has_capture_update_rect: value.has_capture_update_rect,
            has_region_capture_rect: value.has_region_capture_rect,
            has_source_size: value.has_source_size,
            has_capture_counter: value.has_capture_counter,
        }
    }
}
impl From<AcceleratedPaintInfoCommon> for _cef_accelerated_paint_info_common_t {
    fn from(value: AcceleratedPaintInfoCommon) -> Self {
        Self {
            size: value.size,
            timestamp: value.timestamp,
            coded_size: value.coded_size.into(),
            visible_rect: value.visible_rect.into(),
            content_rect: value.content_rect.into(),
            source_size: value.source_size.into(),
            capture_update_rect: value.capture_update_rect.into(),
            region_capture_rect: value.region_capture_rect.into(),
            capture_counter: value.capture_counter,
            has_capture_update_rect: value.has_capture_update_rect,
            has_region_capture_rect: value.has_region_capture_rect,
            has_source_size: value.has_source_size,
            has_capture_counter: value.has_capture_counter,
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

/// See [`_cef_main_args_t`] for more documentation.
#[derive(Clone)]
pub struct MainArgs {
    pub instance: HINSTANCE,
}
impl From<_cef_main_args_t> for MainArgs {
    fn from(value: _cef_main_args_t) -> Self {
        Self {
            instance: value.instance,
        }
    }
}
impl From<MainArgs> for _cef_main_args_t {
    fn from(value: MainArgs) -> Self {
        Self {
            instance: value.instance,
        }
    }
}
impl Default for MainArgs {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_window_info_t`] for more documentation.
#[derive(Clone)]
pub struct WindowInfo {
    pub size: usize,
    pub ex_style: DWORD,
    pub window_name: CefString,
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
            size: value.size,
            ex_style: value.ex_style,
            window_name: value.window_name.into(),
            style: value.style,
            bounds: value.bounds.into(),
            parent_window: value.parent_window,
            menu: value.menu,
            windowless_rendering_enabled: value.windowless_rendering_enabled,
            shared_texture_enabled: value.shared_texture_enabled,
            external_begin_frame_enabled: value.external_begin_frame_enabled,
            window: value.window,
            runtime_style: value.runtime_style.into(),
        }
    }
}
impl From<WindowInfo> for _cef_window_info_t {
    fn from(value: WindowInfo) -> Self {
        Self {
            size: value.size,
            ex_style: value.ex_style,
            window_name: value.window_name.into(),
            style: value.style,
            bounds: value.bounds.into(),
            parent_window: value.parent_window,
            menu: value.menu,
            windowless_rendering_enabled: value.windowless_rendering_enabled,
            shared_texture_enabled: value.shared_texture_enabled,
            external_begin_frame_enabled: value.external_begin_frame_enabled,
            window: value.window,
            runtime_style: value.runtime_style.into(),
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

/// See [`_cef_accelerated_paint_info_t`] for more documentation.
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
            size: value.size,
            shared_texture_handle: value.shared_texture_handle,
            format: value.format.into(),
            extra: value.extra.into(),
        }
    }
}
impl From<AcceleratedPaintInfo> for _cef_accelerated_paint_info_t {
    fn from(value: AcceleratedPaintInfo) -> Self {
        Self {
            size: value.size,
            shared_texture_handle: value.shared_texture_handle,
            format: value.format.into(),
            extra: value.extra.into(),
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

/// See [`_cef_settings_t`] for more documentation.
#[derive(Clone)]
pub struct Settings {
    pub size: usize,
    pub no_sandbox: ::std::os::raw::c_int,
    pub browser_subprocess_path: CefString,
    pub framework_dir_path: CefString,
    pub main_bundle_path: CefString,
    pub multi_threaded_message_loop: ::std::os::raw::c_int,
    pub external_message_pump: ::std::os::raw::c_int,
    pub windowless_rendering_enabled: ::std::os::raw::c_int,
    pub command_line_args_disabled: ::std::os::raw::c_int,
    pub cache_path: CefString,
    pub root_cache_path: CefString,
    pub persist_session_cookies: ::std::os::raw::c_int,
    pub user_agent: CefString,
    pub user_agent_product: CefString,
    pub locale: CefString,
    pub log_file: CefString,
    pub log_severity: LogSeverity,
    pub log_items: LogItems,
    pub javascript_flags: CefString,
    pub resources_dir_path: CefString,
    pub locales_dir_path: CefString,
    pub remote_debugging_port: ::std::os::raw::c_int,
    pub uncaught_exception_stack_size: ::std::os::raw::c_int,
    pub background_color: u32,
    pub accept_language_list: CefString,
    pub cookieable_schemes_list: CefString,
    pub cookieable_schemes_exclude_defaults: ::std::os::raw::c_int,
    pub chrome_policy_id: CefString,
    pub chrome_app_icon_id: ::std::os::raw::c_int,
    pub disable_signal_handlers: ::std::os::raw::c_int,
}
impl From<_cef_settings_t> for Settings {
    fn from(value: _cef_settings_t) -> Self {
        Self {
            size: value.size,
            no_sandbox: value.no_sandbox,
            browser_subprocess_path: value.browser_subprocess_path.into(),
            framework_dir_path: value.framework_dir_path.into(),
            main_bundle_path: value.main_bundle_path.into(),
            multi_threaded_message_loop: value.multi_threaded_message_loop,
            external_message_pump: value.external_message_pump,
            windowless_rendering_enabled: value.windowless_rendering_enabled,
            command_line_args_disabled: value.command_line_args_disabled,
            cache_path: value.cache_path.into(),
            root_cache_path: value.root_cache_path.into(),
            persist_session_cookies: value.persist_session_cookies,
            user_agent: value.user_agent.into(),
            user_agent_product: value.user_agent_product.into(),
            locale: value.locale.into(),
            log_file: value.log_file.into(),
            log_severity: value.log_severity.into(),
            log_items: value.log_items.into(),
            javascript_flags: value.javascript_flags.into(),
            resources_dir_path: value.resources_dir_path.into(),
            locales_dir_path: value.locales_dir_path.into(),
            remote_debugging_port: value.remote_debugging_port,
            uncaught_exception_stack_size: value.uncaught_exception_stack_size,
            background_color: value.background_color,
            accept_language_list: value.accept_language_list.into(),
            cookieable_schemes_list: value.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: value.cookieable_schemes_exclude_defaults,
            chrome_policy_id: value.chrome_policy_id.into(),
            chrome_app_icon_id: value.chrome_app_icon_id,
            disable_signal_handlers: value.disable_signal_handlers,
        }
    }
}
impl From<Settings> for _cef_settings_t {
    fn from(value: Settings) -> Self {
        Self {
            size: value.size,
            no_sandbox: value.no_sandbox,
            browser_subprocess_path: value.browser_subprocess_path.into(),
            framework_dir_path: value.framework_dir_path.into(),
            main_bundle_path: value.main_bundle_path.into(),
            multi_threaded_message_loop: value.multi_threaded_message_loop,
            external_message_pump: value.external_message_pump,
            windowless_rendering_enabled: value.windowless_rendering_enabled,
            command_line_args_disabled: value.command_line_args_disabled,
            cache_path: value.cache_path.into(),
            root_cache_path: value.root_cache_path.into(),
            persist_session_cookies: value.persist_session_cookies,
            user_agent: value.user_agent.into(),
            user_agent_product: value.user_agent_product.into(),
            locale: value.locale.into(),
            log_file: value.log_file.into(),
            log_severity: value.log_severity.into(),
            log_items: value.log_items.into(),
            javascript_flags: value.javascript_flags.into(),
            resources_dir_path: value.resources_dir_path.into(),
            locales_dir_path: value.locales_dir_path.into(),
            remote_debugging_port: value.remote_debugging_port,
            uncaught_exception_stack_size: value.uncaught_exception_stack_size,
            background_color: value.background_color,
            accept_language_list: value.accept_language_list.into(),
            cookieable_schemes_list: value.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: value.cookieable_schemes_exclude_defaults,
            chrome_policy_id: value.chrome_policy_id.into(),
            chrome_app_icon_id: value.chrome_app_icon_id,
            disable_signal_handlers: value.disable_signal_handlers,
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

/// See [`_cef_request_context_settings_t`] for more documentation.
#[derive(Clone)]
pub struct RequestContextSettings {
    pub size: usize,
    pub cache_path: CefString,
    pub persist_session_cookies: ::std::os::raw::c_int,
    pub accept_language_list: CefString,
    pub cookieable_schemes_list: CefString,
    pub cookieable_schemes_exclude_defaults: ::std::os::raw::c_int,
}
impl From<_cef_request_context_settings_t> for RequestContextSettings {
    fn from(value: _cef_request_context_settings_t) -> Self {
        Self {
            size: value.size,
            cache_path: value.cache_path.into(),
            persist_session_cookies: value.persist_session_cookies,
            accept_language_list: value.accept_language_list.into(),
            cookieable_schemes_list: value.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: value.cookieable_schemes_exclude_defaults,
        }
    }
}
impl From<RequestContextSettings> for _cef_request_context_settings_t {
    fn from(value: RequestContextSettings) -> Self {
        Self {
            size: value.size,
            cache_path: value.cache_path.into(),
            persist_session_cookies: value.persist_session_cookies,
            accept_language_list: value.accept_language_list.into(),
            cookieable_schemes_list: value.cookieable_schemes_list.into(),
            cookieable_schemes_exclude_defaults: value.cookieable_schemes_exclude_defaults,
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

/// See [`_cef_browser_settings_t`] for more documentation.
#[derive(Clone)]
pub struct BrowserSettings {
    pub size: usize,
    pub windowless_frame_rate: ::std::os::raw::c_int,
    pub standard_font_family: CefString,
    pub fixed_font_family: CefString,
    pub serif_font_family: CefString,
    pub sans_serif_font_family: CefString,
    pub cursive_font_family: CefString,
    pub fantasy_font_family: CefString,
    pub default_font_size: ::std::os::raw::c_int,
    pub default_fixed_font_size: ::std::os::raw::c_int,
    pub minimum_font_size: ::std::os::raw::c_int,
    pub minimum_logical_font_size: ::std::os::raw::c_int,
    pub default_encoding: CefString,
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
            size: value.size,
            windowless_frame_rate: value.windowless_frame_rate,
            standard_font_family: value.standard_font_family.into(),
            fixed_font_family: value.fixed_font_family.into(),
            serif_font_family: value.serif_font_family.into(),
            sans_serif_font_family: value.sans_serif_font_family.into(),
            cursive_font_family: value.cursive_font_family.into(),
            fantasy_font_family: value.fantasy_font_family.into(),
            default_font_size: value.default_font_size,
            default_fixed_font_size: value.default_fixed_font_size,
            minimum_font_size: value.minimum_font_size,
            minimum_logical_font_size: value.minimum_logical_font_size,
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
            background_color: value.background_color,
            chrome_status_bubble: value.chrome_status_bubble.into(),
            chrome_zoom_bubble: value.chrome_zoom_bubble.into(),
        }
    }
}
impl From<BrowserSettings> for _cef_browser_settings_t {
    fn from(value: BrowserSettings) -> Self {
        Self {
            size: value.size,
            windowless_frame_rate: value.windowless_frame_rate,
            standard_font_family: value.standard_font_family.into(),
            fixed_font_family: value.fixed_font_family.into(),
            serif_font_family: value.serif_font_family.into(),
            sans_serif_font_family: value.sans_serif_font_family.into(),
            cursive_font_family: value.cursive_font_family.into(),
            fantasy_font_family: value.fantasy_font_family.into(),
            default_font_size: value.default_font_size,
            default_fixed_font_size: value.default_fixed_font_size,
            minimum_font_size: value.minimum_font_size,
            minimum_logical_font_size: value.minimum_logical_font_size,
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
            background_color: value.background_color,
            chrome_status_bubble: value.chrome_status_bubble.into(),
            chrome_zoom_bubble: value.chrome_zoom_bubble.into(),
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

/// See [`_cef_urlparts_t`] for more documentation.
#[derive(Clone)]
pub struct Urlparts {
    pub size: usize,
    pub spec: CefString,
    pub scheme: CefString,
    pub username: CefString,
    pub password: CefString,
    pub host: CefString,
    pub port: CefString,
    pub origin: CefString,
    pub path: CefString,
    pub query: CefString,
    pub fragment: CefString,
}
impl From<_cef_urlparts_t> for Urlparts {
    fn from(value: _cef_urlparts_t) -> Self {
        Self {
            size: value.size,
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
impl From<Urlparts> for _cef_urlparts_t {
    fn from(value: Urlparts) -> Self {
        Self {
            size: value.size,
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
impl Default for Urlparts {
    fn default() -> Self {
        Self {
            size: std::mem::size_of::<_cef_urlparts_t>(),
            ..unsafe { std::mem::zeroed() }
        }
    }
}

/// See [`_cef_cookie_t`] for more documentation.
#[derive(Clone)]
pub struct Cookie {
    pub size: usize,
    pub name: CefString,
    pub value: CefString,
    pub domain: CefString,
    pub path: CefString,
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
            size: value.size,
            name: value.name.into(),
            value: value.value.into(),
            domain: value.domain.into(),
            path: value.path.into(),
            secure: value.secure,
            httponly: value.httponly,
            creation: value.creation.into(),
            last_access: value.last_access.into(),
            has_expires: value.has_expires,
            expires: value.expires.into(),
            same_site: value.same_site.into(),
            priority: value.priority.into(),
        }
    }
}
impl From<Cookie> for _cef_cookie_t {
    fn from(value: Cookie) -> Self {
        Self {
            size: value.size,
            name: value.name.into(),
            value: value.value.into(),
            domain: value.domain.into(),
            path: value.path.into(),
            secure: value.secure,
            httponly: value.httponly,
            creation: value.creation.into(),
            last_access: value.last_access.into(),
            has_expires: value.has_expires,
            expires: value.expires.into(),
            same_site: value.same_site.into(),
            priority: value.priority.into(),
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

/// See [`_cef_draggable_region_t`] for more documentation.
#[derive(Clone)]
pub struct DraggableRegion {
    pub bounds: Rect,
    pub draggable: ::std::os::raw::c_int,
}
impl From<_cef_draggable_region_t> for DraggableRegion {
    fn from(value: _cef_draggable_region_t) -> Self {
        Self {
            bounds: value.bounds.into(),
            draggable: value.draggable,
        }
    }
}
impl From<DraggableRegion> for _cef_draggable_region_t {
    fn from(value: DraggableRegion) -> Self {
        Self {
            bounds: value.bounds.into(),
            draggable: value.draggable,
        }
    }
}
impl Default for DraggableRegion {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_screen_info_t`] for more documentation.
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
            size: value.size,
            device_scale_factor: value.device_scale_factor,
            depth: value.depth,
            depth_per_component: value.depth_per_component,
            is_monochrome: value.is_monochrome,
            rect: value.rect.into(),
            available_rect: value.available_rect.into(),
        }
    }
}
impl From<ScreenInfo> for _cef_screen_info_t {
    fn from(value: ScreenInfo) -> Self {
        Self {
            size: value.size,
            device_scale_factor: value.device_scale_factor,
            depth: value.depth,
            depth_per_component: value.depth_per_component,
            is_monochrome: value.is_monochrome,
            rect: value.rect.into(),
            available_rect: value.available_rect.into(),
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

/// See [`_cef_linux_window_properties_t`] for more documentation.
#[derive(Clone)]
pub struct LinuxWindowProperties {
    pub size: usize,
    pub wayland_app_id: CefString,
    pub wm_class_class: CefString,
    pub wm_class_name: CefString,
    pub wm_role_name: CefString,
}
impl From<_cef_linux_window_properties_t> for LinuxWindowProperties {
    fn from(value: _cef_linux_window_properties_t) -> Self {
        Self {
            size: value.size,
            wayland_app_id: value.wayland_app_id.into(),
            wm_class_class: value.wm_class_class.into(),
            wm_class_name: value.wm_class_name.into(),
            wm_role_name: value.wm_role_name.into(),
        }
    }
}
impl From<LinuxWindowProperties> for _cef_linux_window_properties_t {
    fn from(value: LinuxWindowProperties) -> Self {
        Self {
            size: value.size,
            wayland_app_id: value.wayland_app_id.into(),
            wm_class_class: value.wm_class_class.into(),
            wm_class_name: value.wm_class_name.into(),
            wm_role_name: value.wm_role_name.into(),
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

/// See [`_cef_mouse_event_t`] for more documentation.
#[derive(Clone)]
pub struct MouseEvent {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub modifiers: u32,
}
impl From<_cef_mouse_event_t> for MouseEvent {
    fn from(value: _cef_mouse_event_t) -> Self {
        Self {
            x: value.x,
            y: value.y,
            modifiers: value.modifiers,
        }
    }
}
impl From<MouseEvent> for _cef_mouse_event_t {
    fn from(value: MouseEvent) -> Self {
        Self {
            x: value.x,
            y: value.y,
            modifiers: value.modifiers,
        }
    }
}
impl Default for MouseEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_touch_event_t`] for more documentation.
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
            id: value.id,
            x: value.x,
            y: value.y,
            radius_x: value.radius_x,
            radius_y: value.radius_y,
            rotation_angle: value.rotation_angle,
            pressure: value.pressure,
            type_: value.type_.into(),
            modifiers: value.modifiers,
            pointer_type: value.pointer_type.into(),
        }
    }
}
impl From<TouchEvent> for _cef_touch_event_t {
    fn from(value: TouchEvent) -> Self {
        Self {
            id: value.id,
            x: value.x,
            y: value.y,
            radius_x: value.radius_x,
            radius_y: value.radius_y,
            rotation_angle: value.rotation_angle,
            pressure: value.pressure,
            type_: value.type_.into(),
            modifiers: value.modifiers,
            pointer_type: value.pointer_type.into(),
        }
    }
}
impl Default for TouchEvent {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_key_event_t`] for more documentation.
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
            size: value.size,
            type_: value.type_.into(),
            modifiers: value.modifiers,
            windows_key_code: value.windows_key_code,
            native_key_code: value.native_key_code,
            is_system_key: value.is_system_key,
            character: value.character,
            unmodified_character: value.unmodified_character,
            focus_on_editable_field: value.focus_on_editable_field,
        }
    }
}
impl From<KeyEvent> for _cef_key_event_t {
    fn from(value: KeyEvent) -> Self {
        Self {
            size: value.size,
            type_: value.type_.into(),
            modifiers: value.modifiers,
            windows_key_code: value.windows_key_code,
            native_key_code: value.native_key_code,
            is_system_key: value.is_system_key,
            character: value.character,
            unmodified_character: value.unmodified_character,
            focus_on_editable_field: value.focus_on_editable_field,
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

/// See [`_cef_popup_features_t`] for more documentation.
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
            size: value.size,
            x: value.x,
            x_set: value.xSet,
            y: value.y,
            y_set: value.ySet,
            width: value.width,
            width_set: value.widthSet,
            height: value.height,
            height_set: value.heightSet,
            is_popup: value.isPopup,
        }
    }
}
impl From<PopupFeatures> for _cef_popup_features_t {
    fn from(value: PopupFeatures) -> Self {
        Self {
            size: value.size,
            x: value.x,
            xSet: value.x_set,
            y: value.y,
            ySet: value.y_set,
            width: value.width,
            widthSet: value.width_set,
            height: value.height,
            heightSet: value.height_set,
            isPopup: value.is_popup,
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

/// See [`_cef_cursor_info_t`] for more documentation.
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
            image_scale_factor: value.image_scale_factor,
            buffer: value.buffer,
            size: value.size.into(),
        }
    }
}
impl From<CursorInfo> for _cef_cursor_info_t {
    fn from(value: CursorInfo) -> Self {
        Self {
            hotspot: value.hotspot.into(),
            image_scale_factor: value.image_scale_factor,
            buffer: value.buffer,
            size: value.size.into(),
        }
    }
}
impl Default for CursorInfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_pdf_print_settings_t`] for more documentation.
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
    pub page_ranges: CefString,
    pub display_header_footer: ::std::os::raw::c_int,
    pub header_template: CefString,
    pub footer_template: CefString,
    pub generate_tagged_pdf: ::std::os::raw::c_int,
    pub generate_document_outline: ::std::os::raw::c_int,
}
impl From<_cef_pdf_print_settings_t> for PdfPrintSettings {
    fn from(value: _cef_pdf_print_settings_t) -> Self {
        Self {
            size: value.size,
            landscape: value.landscape,
            print_background: value.print_background,
            scale: value.scale,
            paper_width: value.paper_width,
            paper_height: value.paper_height,
            prefer_css_page_size: value.prefer_css_page_size,
            margin_type: value.margin_type.into(),
            margin_top: value.margin_top,
            margin_right: value.margin_right,
            margin_bottom: value.margin_bottom,
            margin_left: value.margin_left,
            page_ranges: value.page_ranges.into(),
            display_header_footer: value.display_header_footer,
            header_template: value.header_template.into(),
            footer_template: value.footer_template.into(),
            generate_tagged_pdf: value.generate_tagged_pdf,
            generate_document_outline: value.generate_document_outline,
        }
    }
}
impl From<PdfPrintSettings> for _cef_pdf_print_settings_t {
    fn from(value: PdfPrintSettings) -> Self {
        Self {
            size: value.size,
            landscape: value.landscape,
            print_background: value.print_background,
            scale: value.scale,
            paper_width: value.paper_width,
            paper_height: value.paper_height,
            prefer_css_page_size: value.prefer_css_page_size,
            margin_type: value.margin_type.into(),
            margin_top: value.margin_top,
            margin_right: value.margin_right,
            margin_bottom: value.margin_bottom,
            margin_left: value.margin_left,
            page_ranges: value.page_ranges.into(),
            display_header_footer: value.display_header_footer,
            header_template: value.header_template.into(),
            footer_template: value.footer_template.into(),
            generate_tagged_pdf: value.generate_tagged_pdf,
            generate_document_outline: value.generate_document_outline,
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

/// See [`_cef_box_layout_settings_t`] for more documentation.
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
            size: value.size,
            horizontal: value.horizontal,
            inside_border_horizontal_spacing: value.inside_border_horizontal_spacing,
            inside_border_vertical_spacing: value.inside_border_vertical_spacing,
            inside_border_insets: value.inside_border_insets.into(),
            between_child_spacing: value.between_child_spacing,
            main_axis_alignment: value.main_axis_alignment.into(),
            cross_axis_alignment: value.cross_axis_alignment.into(),
            minimum_cross_axis_size: value.minimum_cross_axis_size,
            default_flex: value.default_flex,
        }
    }
}
impl From<BoxLayoutSettings> for _cef_box_layout_settings_t {
    fn from(value: BoxLayoutSettings) -> Self {
        Self {
            size: value.size,
            horizontal: value.horizontal,
            inside_border_horizontal_spacing: value.inside_border_horizontal_spacing,
            inside_border_vertical_spacing: value.inside_border_vertical_spacing,
            inside_border_insets: value.inside_border_insets.into(),
            between_child_spacing: value.between_child_spacing,
            main_axis_alignment: value.main_axis_alignment.into(),
            cross_axis_alignment: value.cross_axis_alignment.into(),
            minimum_cross_axis_size: value.minimum_cross_axis_size,
            default_flex: value.default_flex,
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

/// See [`_cef_range_t`] for more documentation.
#[derive(Clone)]
pub struct Range {
    pub from: u32,
    pub to: u32,
}
impl From<_cef_range_t> for Range {
    fn from(value: _cef_range_t) -> Self {
        Self {
            from: value.from,
            to: value.to,
        }
    }
}
impl From<Range> for _cef_range_t {
    fn from(value: Range) -> Self {
        Self {
            from: value.from,
            to: value.to,
        }
    }
}
impl Default for Range {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_composition_underline_t`] for more documentation.
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
            size: value.size,
            range: value.range.into(),
            color: value.color,
            background_color: value.background_color,
            thick: value.thick,
            style: value.style.into(),
        }
    }
}
impl From<CompositionUnderline> for _cef_composition_underline_t {
    fn from(value: CompositionUnderline) -> Self {
        Self {
            size: value.size,
            range: value.range.into(),
            color: value.color,
            background_color: value.background_color,
            thick: value.thick,
            style: value.style.into(),
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

/// See [`_cef_audio_parameters_t`] for more documentation.
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
            size: value.size,
            channel_layout: value.channel_layout.into(),
            sample_rate: value.sample_rate,
            frames_per_buffer: value.frames_per_buffer,
        }
    }
}
impl From<AudioParameters> for _cef_audio_parameters_t {
    fn from(value: AudioParameters) -> Self {
        Self {
            size: value.size,
            channel_layout: value.channel_layout.into(),
            sample_rate: value.sample_rate,
            frames_per_buffer: value.frames_per_buffer,
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

/// See [`_cef_media_sink_device_info_t`] for more documentation.
#[derive(Clone)]
pub struct MediaSinkDeviceInfo {
    pub size: usize,
    pub ip_address: CefString,
    pub port: ::std::os::raw::c_int,
    pub model_name: CefString,
}
impl From<_cef_media_sink_device_info_t> for MediaSinkDeviceInfo {
    fn from(value: _cef_media_sink_device_info_t) -> Self {
        Self {
            size: value.size,
            ip_address: value.ip_address.into(),
            port: value.port,
            model_name: value.model_name.into(),
        }
    }
}
impl From<MediaSinkDeviceInfo> for _cef_media_sink_device_info_t {
    fn from(value: MediaSinkDeviceInfo) -> Self {
        Self {
            size: value.size,
            ip_address: value.ip_address.into(),
            port: value.port,
            model_name: value.model_name.into(),
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

/// See [`_cef_touch_handle_state_t`] for more documentation.
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
            size: value.size,
            touch_handle_id: value.touch_handle_id,
            flags: value.flags,
            enabled: value.enabled,
            orientation: value.orientation.into(),
            mirror_vertical: value.mirror_vertical,
            mirror_horizontal: value.mirror_horizontal,
            origin: value.origin.into(),
            alpha: value.alpha,
        }
    }
}
impl From<TouchHandleState> for _cef_touch_handle_state_t {
    fn from(value: TouchHandleState) -> Self {
        Self {
            size: value.size,
            touch_handle_id: value.touch_handle_id,
            flags: value.flags,
            enabled: value.enabled,
            orientation: value.orientation.into(),
            mirror_vertical: value.mirror_vertical,
            mirror_horizontal: value.mirror_horizontal,
            origin: value.origin.into(),
            alpha: value.alpha,
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

/// See [`_cef_task_info_t`] for more documentation.
#[derive(Clone)]
pub struct TaskInfo {
    pub size: usize,
    pub id: i64,
    pub type_: TaskType,
    pub is_killable: ::std::os::raw::c_int,
    pub title: CefString,
    pub cpu_usage: f64,
    pub number_of_processors: ::std::os::raw::c_int,
    pub memory: i64,
    pub gpu_memory: i64,
    pub is_gpu_memory_inflated: ::std::os::raw::c_int,
}
impl From<_cef_task_info_t> for TaskInfo {
    fn from(value: _cef_task_info_t) -> Self {
        Self {
            size: value.size,
            id: value.id,
            type_: value.type_.into(),
            is_killable: value.is_killable,
            title: value.title.into(),
            cpu_usage: value.cpu_usage,
            number_of_processors: value.number_of_processors,
            memory: value.memory,
            gpu_memory: value.gpu_memory,
            is_gpu_memory_inflated: value.is_gpu_memory_inflated,
        }
    }
}
impl From<TaskInfo> for _cef_task_info_t {
    fn from(value: TaskInfo) -> Self {
        Self {
            size: value.size,
            id: value.id,
            type_: value.type_.into(),
            is_killable: value.is_killable,
            title: value.title.into(),
            cpu_usage: value.cpu_usage,
            number_of_processors: value.number_of_processors,
            memory: value.memory,
            gpu_memory: value.gpu_memory,
            is_gpu_memory_inflated: value.is_gpu_memory_inflated,
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

/// See [`_cef_base_ref_counted_t`] for more documentation.
#[derive(Clone)]
pub struct BaseRefCounted(RefGuard<_cef_base_ref_counted_t>);
impl BaseRefCounted {
    fn get_raw(&self) -> *mut _cef_base_ref_counted_t {
        unsafe { RefGuard::into_raw(&self.0) }
    }
}
impl Rc for BaseRefCounted {
    fn as_base(&self) -> &_cef_base_ref_counted_t {
        self.0.as_base()
    }
}
impl ConvertParam<*mut _cef_base_ref_counted_t> for &BaseRefCounted {
    fn into_raw(self) -> *mut _cef_base_ref_counted_t {
        self.get_raw()
    }
}
impl ConvertParam<*mut _cef_base_ref_counted_t> for &mut BaseRefCounted {
    fn into_raw(self) -> *mut _cef_base_ref_counted_t {
        self.get_raw()
    }
}
impl ConvertReturnValue<BaseRefCounted> for *mut _cef_base_ref_counted_t {
    fn wrap_result(self) -> BaseRefCounted {
        BaseRefCounted(unsafe { RefGuard::from_raw(self) })
    }
}
impl From<BaseRefCounted> for *mut _cef_base_ref_counted_t {
    fn from(value: BaseRefCounted) -> Self {
        let object = value.get_raw();
        std::mem::forget(value);
        object
    }
}
impl Default for BaseRefCounted {
    fn default() -> Self {
        Self(unsafe { RefGuard::from_raw(std::ptr::null_mut()) })
    }
}

/// See [`_cef_base_scoped_t`] for more documentation.
#[derive(Clone, Copy)]
pub struct BaseScoped(*mut _cef_base_scoped_t);
impl BaseScoped {
    fn get_raw(&self) -> *mut _cef_base_scoped_t {
        self.0
    }
}
impl ConvertParam<*mut _cef_base_scoped_t> for &BaseScoped {
    fn into_raw(self) -> *mut _cef_base_scoped_t {
        self.get_raw()
    }
}
impl ConvertParam<*mut _cef_base_scoped_t> for &mut BaseScoped {
    fn into_raw(self) -> *mut _cef_base_scoped_t {
        self.get_raw()
    }
}
impl ConvertReturnValue<BaseScoped> for *mut _cef_base_scoped_t {
    fn wrap_result(self) -> BaseScoped {
        BaseScoped(self)
    }
}
impl From<BaseScoped> for *mut _cef_base_scoped_t {
    fn from(value: BaseScoped) -> Self {
        value.get_raw()
    }
}
impl Default for BaseScoped {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

/// See [`_cef_dev_tools_message_observer_t`] for more documentation.
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
            on_dev_tools_message: value.on_dev_tools_message,
            on_dev_tools_method_result: value.on_dev_tools_method_result,
            on_dev_tools_event: value.on_dev_tools_event,
            on_dev_tools_agent_attached: value.on_dev_tools_agent_attached,
            on_dev_tools_agent_detached: value.on_dev_tools_agent_detached,
        }
    }
}
impl From<DevToolsMessageObserver> for _cef_dev_tools_message_observer_t {
    fn from(value: DevToolsMessageObserver) -> Self {
        Self {
            base: value.base.into(),
            on_dev_tools_message: value.on_dev_tools_message,
            on_dev_tools_method_result: value.on_dev_tools_method_result,
            on_dev_tools_event: value.on_dev_tools_event,
            on_dev_tools_agent_attached: value.on_dev_tools_agent_attached,
            on_dev_tools_agent_detached: value.on_dev_tools_agent_detached,
        }
    }
}
impl Default for DevToolsMessageObserver {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_value_t`] for more documentation.
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
            is_valid: value.is_valid,
            is_owned: value.is_owned,
            is_read_only: value.is_read_only,
            is_same: value.is_same,
            is_equal: value.is_equal,
            copy: value.copy,
            get_type: value.get_type,
            get_bool: value.get_bool,
            get_int: value.get_int,
            get_double: value.get_double,
            get_string: value.get_string,
            get_binary: value.get_binary,
            get_dictionary: value.get_dictionary,
            get_list: value.get_list,
            set_null: value.set_null,
            set_bool: value.set_bool,
            set_int: value.set_int,
            set_double: value.set_double,
            set_string: value.set_string,
            set_binary: value.set_binary,
            set_dictionary: value.set_dictionary,
            set_list: value.set_list,
        }
    }
}
impl From<Value> for _cef_value_t {
    fn from(value: Value) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            is_owned: value.is_owned,
            is_read_only: value.is_read_only,
            is_same: value.is_same,
            is_equal: value.is_equal,
            copy: value.copy,
            get_type: value.get_type,
            get_bool: value.get_bool,
            get_int: value.get_int,
            get_double: value.get_double,
            get_string: value.get_string,
            get_binary: value.get_binary,
            get_dictionary: value.get_dictionary,
            get_list: value.get_list,
            set_null: value.set_null,
            set_bool: value.set_bool,
            set_int: value.set_int,
            set_double: value.set_double,
            set_string: value.set_string,
            set_binary: value.set_binary,
            set_dictionary: value.set_dictionary,
            set_list: value.set_list,
        }
    }
}
impl Default for Value {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_binary_value_t`] for more documentation.
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
            is_valid: value.is_valid,
            is_owned: value.is_owned,
            is_same: value.is_same,
            is_equal: value.is_equal,
            copy: value.copy,
            get_raw_data: value.get_raw_data,
            get_size: value.get_size,
            get_data: value.get_data,
        }
    }
}
impl From<BinaryValue> for _cef_binary_value_t {
    fn from(value: BinaryValue) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            is_owned: value.is_owned,
            is_same: value.is_same,
            is_equal: value.is_equal,
            copy: value.copy,
            get_raw_data: value.get_raw_data,
            get_size: value.get_size,
            get_data: value.get_data,
        }
    }
}
impl Default for BinaryValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_dictionary_value_t`] for more documentation.
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
            is_valid: value.is_valid,
            is_owned: value.is_owned,
            is_read_only: value.is_read_only,
            is_same: value.is_same,
            is_equal: value.is_equal,
            copy: value.copy,
            get_size: value.get_size,
            clear: value.clear,
            has_key: value.has_key,
            get_keys: value.get_keys,
            remove: value.remove,
            get_type: value.get_type,
            get_value: value.get_value,
            get_bool: value.get_bool,
            get_int: value.get_int,
            get_double: value.get_double,
            get_string: value.get_string,
            get_binary: value.get_binary,
            get_dictionary: value.get_dictionary,
            get_list: value.get_list,
            set_value: value.set_value,
            set_null: value.set_null,
            set_bool: value.set_bool,
            set_int: value.set_int,
            set_double: value.set_double,
            set_string: value.set_string,
            set_binary: value.set_binary,
            set_dictionary: value.set_dictionary,
            set_list: value.set_list,
        }
    }
}
impl From<DictionaryValue> for _cef_dictionary_value_t {
    fn from(value: DictionaryValue) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            is_owned: value.is_owned,
            is_read_only: value.is_read_only,
            is_same: value.is_same,
            is_equal: value.is_equal,
            copy: value.copy,
            get_size: value.get_size,
            clear: value.clear,
            has_key: value.has_key,
            get_keys: value.get_keys,
            remove: value.remove,
            get_type: value.get_type,
            get_value: value.get_value,
            get_bool: value.get_bool,
            get_int: value.get_int,
            get_double: value.get_double,
            get_string: value.get_string,
            get_binary: value.get_binary,
            get_dictionary: value.get_dictionary,
            get_list: value.get_list,
            set_value: value.set_value,
            set_null: value.set_null,
            set_bool: value.set_bool,
            set_int: value.set_int,
            set_double: value.set_double,
            set_string: value.set_string,
            set_binary: value.set_binary,
            set_dictionary: value.set_dictionary,
            set_list: value.set_list,
        }
    }
}
impl Default for DictionaryValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_list_value_t`] for more documentation.
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
            is_valid: value.is_valid,
            is_owned: value.is_owned,
            is_read_only: value.is_read_only,
            is_same: value.is_same,
            is_equal: value.is_equal,
            copy: value.copy,
            set_size: value.set_size,
            get_size: value.get_size,
            clear: value.clear,
            remove: value.remove,
            get_type: value.get_type,
            get_value: value.get_value,
            get_bool: value.get_bool,
            get_int: value.get_int,
            get_double: value.get_double,
            get_string: value.get_string,
            get_binary: value.get_binary,
            get_dictionary: value.get_dictionary,
            get_list: value.get_list,
            set_value: value.set_value,
            set_null: value.set_null,
            set_bool: value.set_bool,
            set_int: value.set_int,
            set_double: value.set_double,
            set_string: value.set_string,
            set_binary: value.set_binary,
            set_dictionary: value.set_dictionary,
            set_list: value.set_list,
        }
    }
}
impl From<ListValue> for _cef_list_value_t {
    fn from(value: ListValue) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            is_owned: value.is_owned,
            is_read_only: value.is_read_only,
            is_same: value.is_same,
            is_equal: value.is_equal,
            copy: value.copy,
            set_size: value.set_size,
            get_size: value.get_size,
            clear: value.clear,
            remove: value.remove,
            get_type: value.get_type,
            get_value: value.get_value,
            get_bool: value.get_bool,
            get_int: value.get_int,
            get_double: value.get_double,
            get_string: value.get_string,
            get_binary: value.get_binary,
            get_dictionary: value.get_dictionary,
            get_list: value.get_list,
            set_value: value.set_value,
            set_null: value.set_null,
            set_bool: value.set_bool,
            set_int: value.set_int,
            set_double: value.set_double,
            set_string: value.set_string,
            set_binary: value.set_binary,
            set_dictionary: value.set_dictionary,
            set_list: value.set_list,
        }
    }
}
impl Default for ListValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_image_t`] for more documentation.
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
            is_empty: value.is_empty,
            is_same: value.is_same,
            add_bitmap: value.add_bitmap,
            add_png: value.add_png,
            add_jpeg: value.add_jpeg,
            get_width: value.get_width,
            get_height: value.get_height,
            has_representation: value.has_representation,
            remove_representation: value.remove_representation,
            get_representation_info: value.get_representation_info,
            get_as_bitmap: value.get_as_bitmap,
            get_as_png: value.get_as_png,
            get_as_jpeg: value.get_as_jpeg,
        }
    }
}
impl From<Image> for _cef_image_t {
    fn from(value: Image) -> Self {
        Self {
            base: value.base.into(),
            is_empty: value.is_empty,
            is_same: value.is_same,
            add_bitmap: value.add_bitmap,
            add_png: value.add_png,
            add_jpeg: value.add_jpeg,
            get_width: value.get_width,
            get_height: value.get_height,
            has_representation: value.has_representation,
            remove_representation: value.remove_representation,
            get_representation_info: value.get_representation_info,
            get_as_bitmap: value.get_as_bitmap,
            get_as_png: value.get_as_png,
            get_as_jpeg: value.get_as_jpeg,
        }
    }
}
impl Default for Image {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_read_handler_t`] for more documentation.
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
            read: value.read,
            seek: value.seek,
            tell: value.tell,
            eof: value.eof,
            may_block: value.may_block,
        }
    }
}
impl From<ReadHandler> for _cef_read_handler_t {
    fn from(value: ReadHandler) -> Self {
        Self {
            base: value.base.into(),
            read: value.read,
            seek: value.seek,
            tell: value.tell,
            eof: value.eof,
            may_block: value.may_block,
        }
    }
}
impl Default for ReadHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_stream_reader_t`] for more documentation.
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
            read: value.read,
            seek: value.seek,
            tell: value.tell,
            eof: value.eof,
            may_block: value.may_block,
        }
    }
}
impl From<StreamReader> for _cef_stream_reader_t {
    fn from(value: StreamReader) -> Self {
        Self {
            base: value.base.into(),
            read: value.read,
            seek: value.seek,
            tell: value.tell,
            eof: value.eof,
            may_block: value.may_block,
        }
    }
}
impl Default for StreamReader {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_write_handler_t`] for more documentation.
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
            write: value.write,
            seek: value.seek,
            tell: value.tell,
            flush: value.flush,
            may_block: value.may_block,
        }
    }
}
impl From<WriteHandler> for _cef_write_handler_t {
    fn from(value: WriteHandler) -> Self {
        Self {
            base: value.base.into(),
            write: value.write,
            seek: value.seek,
            tell: value.tell,
            flush: value.flush,
            may_block: value.may_block,
        }
    }
}
impl Default for WriteHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_stream_writer_t`] for more documentation.
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
            write: value.write,
            seek: value.seek,
            tell: value.tell,
            flush: value.flush,
            may_block: value.may_block,
        }
    }
}
impl From<StreamWriter> for _cef_stream_writer_t {
    fn from(value: StreamWriter) -> Self {
        Self {
            base: value.base.into(),
            write: value.write,
            seek: value.seek,
            tell: value.tell,
            flush: value.flush,
            may_block: value.may_block,
        }
    }
}
impl Default for StreamWriter {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_drag_data_t`] for more documentation.
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
            clone: value.clone,
            is_read_only: value.is_read_only,
            is_link: value.is_link,
            is_fragment: value.is_fragment,
            is_file: value.is_file,
            get_link_url: value.get_link_url,
            get_link_title: value.get_link_title,
            get_link_metadata: value.get_link_metadata,
            get_fragment_text: value.get_fragment_text,
            get_fragment_html: value.get_fragment_html,
            get_fragment_base_url: value.get_fragment_base_url,
            get_file_name: value.get_file_name,
            get_file_contents: value.get_file_contents,
            get_file_names: value.get_file_names,
            get_file_paths: value.get_file_paths,
            set_link_url: value.set_link_url,
            set_link_title: value.set_link_title,
            set_link_metadata: value.set_link_metadata,
            set_fragment_text: value.set_fragment_text,
            set_fragment_html: value.set_fragment_html,
            set_fragment_base_url: value.set_fragment_base_url,
            reset_file_contents: value.reset_file_contents,
            add_file: value.add_file,
            clear_filenames: value.clear_filenames,
            get_image: value.get_image,
            get_image_hotspot: value.get_image_hotspot,
            has_image: value.has_image,
        }
    }
}
impl From<DragData> for _cef_drag_data_t {
    fn from(value: DragData) -> Self {
        Self {
            base: value.base.into(),
            clone: value.clone,
            is_read_only: value.is_read_only,
            is_link: value.is_link,
            is_fragment: value.is_fragment,
            is_file: value.is_file,
            get_link_url: value.get_link_url,
            get_link_title: value.get_link_title,
            get_link_metadata: value.get_link_metadata,
            get_fragment_text: value.get_fragment_text,
            get_fragment_html: value.get_fragment_html,
            get_fragment_base_url: value.get_fragment_base_url,
            get_file_name: value.get_file_name,
            get_file_contents: value.get_file_contents,
            get_file_names: value.get_file_names,
            get_file_paths: value.get_file_paths,
            set_link_url: value.set_link_url,
            set_link_title: value.set_link_title,
            set_link_metadata: value.set_link_metadata,
            set_fragment_text: value.set_fragment_text,
            set_fragment_html: value.set_fragment_html,
            set_fragment_base_url: value.set_fragment_base_url,
            reset_file_contents: value.reset_file_contents,
            add_file: value.add_file,
            clear_filenames: value.clear_filenames,
            get_image: value.get_image,
            get_image_hotspot: value.get_image_hotspot,
            has_image: value.has_image,
        }
    }
}
impl Default for DragData {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_domvisitor_t`] for more documentation.
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
            visit: value.visit,
        }
    }
}
impl From<Domvisitor> for _cef_domvisitor_t {
    fn from(value: Domvisitor) -> Self {
        Self {
            base: value.base.into(),
            visit: value.visit,
        }
    }
}
impl Default for Domvisitor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_domdocument_t`] for more documentation.
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
            get_type: value.get_type,
            get_document: value.get_document,
            get_body: value.get_body,
            get_head: value.get_head,
            get_title: value.get_title,
            get_element_by_id: value.get_element_by_id,
            get_focused_node: value.get_focused_node,
            has_selection: value.has_selection,
            get_selection_start_offset: value.get_selection_start_offset,
            get_selection_end_offset: value.get_selection_end_offset,
            get_selection_as_markup: value.get_selection_as_markup,
            get_selection_as_text: value.get_selection_as_text,
            get_base_url: value.get_base_url,
            get_complete_url: value.get_complete_url,
        }
    }
}
impl From<Domdocument> for _cef_domdocument_t {
    fn from(value: Domdocument) -> Self {
        Self {
            base: value.base.into(),
            get_type: value.get_type,
            get_document: value.get_document,
            get_body: value.get_body,
            get_head: value.get_head,
            get_title: value.get_title,
            get_element_by_id: value.get_element_by_id,
            get_focused_node: value.get_focused_node,
            has_selection: value.has_selection,
            get_selection_start_offset: value.get_selection_start_offset,
            get_selection_end_offset: value.get_selection_end_offset,
            get_selection_as_markup: value.get_selection_as_markup,
            get_selection_as_text: value.get_selection_as_text,
            get_base_url: value.get_base_url,
            get_complete_url: value.get_complete_url,
        }
    }
}
impl Default for Domdocument {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_domnode_t`] for more documentation.
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
            get_type: value.get_type,
            is_text: value.is_text,
            is_element: value.is_element,
            is_editable: value.is_editable,
            is_form_control_element: value.is_form_control_element,
            get_form_control_element_type: value.get_form_control_element_type,
            is_same: value.is_same,
            get_name: value.get_name,
            get_value: value.get_value,
            set_value: value.set_value,
            get_as_markup: value.get_as_markup,
            get_document: value.get_document,
            get_parent: value.get_parent,
            get_previous_sibling: value.get_previous_sibling,
            get_next_sibling: value.get_next_sibling,
            has_children: value.has_children,
            get_first_child: value.get_first_child,
            get_last_child: value.get_last_child,
            get_element_tag_name: value.get_element_tag_name,
            has_element_attributes: value.has_element_attributes,
            has_element_attribute: value.has_element_attribute,
            get_element_attribute: value.get_element_attribute,
            get_element_attributes: value.get_element_attributes,
            set_element_attribute: value.set_element_attribute,
            get_element_inner_text: value.get_element_inner_text,
            get_element_bounds: value.get_element_bounds,
        }
    }
}
impl From<Domnode> for _cef_domnode_t {
    fn from(value: Domnode) -> Self {
        Self {
            base: value.base.into(),
            get_type: value.get_type,
            is_text: value.is_text,
            is_element: value.is_element,
            is_editable: value.is_editable,
            is_form_control_element: value.is_form_control_element,
            get_form_control_element_type: value.get_form_control_element_type,
            is_same: value.is_same,
            get_name: value.get_name,
            get_value: value.get_value,
            set_value: value.set_value,
            get_as_markup: value.get_as_markup,
            get_document: value.get_document,
            get_parent: value.get_parent,
            get_previous_sibling: value.get_previous_sibling,
            get_next_sibling: value.get_next_sibling,
            has_children: value.has_children,
            get_first_child: value.get_first_child,
            get_last_child: value.get_last_child,
            get_element_tag_name: value.get_element_tag_name,
            has_element_attributes: value.has_element_attributes,
            has_element_attribute: value.has_element_attribute,
            get_element_attribute: value.get_element_attribute,
            get_element_attributes: value.get_element_attributes,
            set_element_attribute: value.set_element_attribute,
            get_element_inner_text: value.get_element_inner_text,
            get_element_bounds: value.get_element_bounds,
        }
    }
}
impl Default for Domnode {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_shared_memory_region_t`] for more documentation.
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
            is_valid: value.is_valid,
            size: value.size,
            memory: value.memory,
        }
    }
}
impl From<SharedMemoryRegion> for _cef_shared_memory_region_t {
    fn from(value: SharedMemoryRegion) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            size: value.size,
            memory: value.memory,
        }
    }
}
impl Default for SharedMemoryRegion {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_process_message_t`] for more documentation.
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
            is_valid: value.is_valid,
            is_read_only: value.is_read_only,
            copy: value.copy,
            get_name: value.get_name,
            get_argument_list: value.get_argument_list,
            get_shared_memory_region: value.get_shared_memory_region,
        }
    }
}
impl From<ProcessMessage> for _cef_process_message_t {
    fn from(value: ProcessMessage) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            is_read_only: value.is_read_only,
            copy: value.copy,
            get_name: value.get_name,
            get_argument_list: value.get_argument_list,
            get_shared_memory_region: value.get_shared_memory_region,
        }
    }
}
impl Default for ProcessMessage {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_request_t`] for more documentation.
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
            is_read_only: value.is_read_only,
            get_url: value.get_url,
            set_url: value.set_url,
            get_method: value.get_method,
            set_method: value.set_method,
            set_referrer: value.set_referrer,
            get_referrer_url: value.get_referrer_url,
            get_referrer_policy: value.get_referrer_policy,
            get_post_data: value.get_post_data,
            set_post_data: value.set_post_data,
            get_header_map: value.get_header_map,
            set_header_map: value.set_header_map,
            get_header_by_name: value.get_header_by_name,
            set_header_by_name: value.set_header_by_name,
            set: value.set,
            get_flags: value.get_flags,
            set_flags: value.set_flags,
            get_first_party_for_cookies: value.get_first_party_for_cookies,
            set_first_party_for_cookies: value.set_first_party_for_cookies,
            get_resource_type: value.get_resource_type,
            get_transition_type: value.get_transition_type,
            get_identifier: value.get_identifier,
        }
    }
}
impl From<Request> for _cef_request_t {
    fn from(value: Request) -> Self {
        Self {
            base: value.base.into(),
            is_read_only: value.is_read_only,
            get_url: value.get_url,
            set_url: value.set_url,
            get_method: value.get_method,
            set_method: value.set_method,
            set_referrer: value.set_referrer,
            get_referrer_url: value.get_referrer_url,
            get_referrer_policy: value.get_referrer_policy,
            get_post_data: value.get_post_data,
            set_post_data: value.set_post_data,
            get_header_map: value.get_header_map,
            set_header_map: value.set_header_map,
            get_header_by_name: value.get_header_by_name,
            set_header_by_name: value.set_header_by_name,
            set: value.set,
            get_flags: value.get_flags,
            set_flags: value.set_flags,
            get_first_party_for_cookies: value.get_first_party_for_cookies,
            set_first_party_for_cookies: value.set_first_party_for_cookies,
            get_resource_type: value.get_resource_type,
            get_transition_type: value.get_transition_type,
            get_identifier: value.get_identifier,
        }
    }
}
impl Default for Request {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_post_data_t`] for more documentation.
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
            is_read_only: value.is_read_only,
            has_excluded_elements: value.has_excluded_elements,
            get_element_count: value.get_element_count,
            get_elements: value.get_elements,
            remove_element: value.remove_element,
            add_element: value.add_element,
            remove_elements: value.remove_elements,
        }
    }
}
impl From<PostData> for _cef_post_data_t {
    fn from(value: PostData) -> Self {
        Self {
            base: value.base.into(),
            is_read_only: value.is_read_only,
            has_excluded_elements: value.has_excluded_elements,
            get_element_count: value.get_element_count,
            get_elements: value.get_elements,
            remove_element: value.remove_element,
            add_element: value.add_element,
            remove_elements: value.remove_elements,
        }
    }
}
impl Default for PostData {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_post_data_element_t`] for more documentation.
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
            is_read_only: value.is_read_only,
            set_to_empty: value.set_to_empty,
            set_to_file: value.set_to_file,
            set_to_bytes: value.set_to_bytes,
            get_type: value.get_type,
            get_file: value.get_file,
            get_bytes_count: value.get_bytes_count,
            get_bytes: value.get_bytes,
        }
    }
}
impl From<PostDataElement> for _cef_post_data_element_t {
    fn from(value: PostDataElement) -> Self {
        Self {
            base: value.base.into(),
            is_read_only: value.is_read_only,
            set_to_empty: value.set_to_empty,
            set_to_file: value.set_to_file,
            set_to_bytes: value.set_to_bytes,
            get_type: value.get_type,
            get_file: value.get_file,
            get_bytes_count: value.get_bytes_count,
            get_bytes: value.get_bytes,
        }
    }
}
impl Default for PostDataElement {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_string_visitor_t`] for more documentation.
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
            visit: value.visit,
        }
    }
}
impl From<CefStringVisitor> for _cef_string_visitor_t {
    fn from(value: CefStringVisitor) -> Self {
        Self {
            base: value.base.into(),
            visit: value.visit,
        }
    }
}
impl Default for CefStringVisitor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_frame_t`] for more documentation.
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
            is_valid: value.is_valid,
            undo: value.undo,
            redo: value.redo,
            cut: value.cut,
            copy: value.copy,
            paste: value.paste,
            paste_and_match_style: value.paste_and_match_style,
            del: value.del,
            select_all: value.select_all,
            view_source: value.view_source,
            get_source: value.get_source,
            get_text: value.get_text,
            load_request: value.load_request,
            load_url: value.load_url,
            execute_java_script: value.execute_java_script,
            is_main: value.is_main,
            is_focused: value.is_focused,
            get_name: value.get_name,
            get_identifier: value.get_identifier,
            get_parent: value.get_parent,
            get_url: value.get_url,
            get_browser: value.get_browser,
            get_v_8_context: value.get_v8_context,
            visit_dom: value.visit_dom,
            create_urlrequest: value.create_urlrequest,
            send_process_message: value.send_process_message,
        }
    }
}
impl From<Frame> for _cef_frame_t {
    fn from(value: Frame) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            undo: value.undo,
            redo: value.redo,
            cut: value.cut,
            copy: value.copy,
            paste: value.paste,
            paste_and_match_style: value.paste_and_match_style,
            del: value.del,
            select_all: value.select_all,
            view_source: value.view_source,
            get_source: value.get_source,
            get_text: value.get_text,
            load_request: value.load_request,
            load_url: value.load_url,
            execute_java_script: value.execute_java_script,
            is_main: value.is_main,
            is_focused: value.is_focused,
            get_name: value.get_name,
            get_identifier: value.get_identifier,
            get_parent: value.get_parent,
            get_url: value.get_url,
            get_browser: value.get_browser,
            get_v8_context: value.get_v_8_context,
            visit_dom: value.visit_dom,
            create_urlrequest: value.create_urlrequest,
            send_process_message: value.send_process_message,
        }
    }
}
impl Default for Frame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_x509_cert_principal_t`] for more documentation.
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
            get_display_name: value.get_display_name,
            get_common_name: value.get_common_name,
            get_locality_name: value.get_locality_name,
            get_state_or_province_name: value.get_state_or_province_name,
            get_country_name: value.get_country_name,
            get_organization_names: value.get_organization_names,
            get_organization_unit_names: value.get_organization_unit_names,
        }
    }
}
impl From<X509CertPrincipal> for _cef_x509_cert_principal_t {
    fn from(value: X509CertPrincipal) -> Self {
        Self {
            base: value.base.into(),
            get_display_name: value.get_display_name,
            get_common_name: value.get_common_name,
            get_locality_name: value.get_locality_name,
            get_state_or_province_name: value.get_state_or_province_name,
            get_country_name: value.get_country_name,
            get_organization_names: value.get_organization_names,
            get_organization_unit_names: value.get_organization_unit_names,
        }
    }
}
impl Default for X509CertPrincipal {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_x509_certificate_t`] for more documentation.
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
            get_subject: value.get_subject,
            get_issuer: value.get_issuer,
            get_serial_number: value.get_serial_number,
            get_valid_start: value.get_valid_start,
            get_valid_expiry: value.get_valid_expiry,
            get_derencoded: value.get_derencoded,
            get_pemencoded: value.get_pemencoded,
            get_issuer_chain_size: value.get_issuer_chain_size,
            get_derencoded_issuer_chain: value.get_derencoded_issuer_chain,
            get_pemencoded_issuer_chain: value.get_pemencoded_issuer_chain,
        }
    }
}
impl From<X509Certificate> for _cef_x509_certificate_t {
    fn from(value: X509Certificate) -> Self {
        Self {
            base: value.base.into(),
            get_subject: value.get_subject,
            get_issuer: value.get_issuer,
            get_serial_number: value.get_serial_number,
            get_valid_start: value.get_valid_start,
            get_valid_expiry: value.get_valid_expiry,
            get_derencoded: value.get_derencoded,
            get_pemencoded: value.get_pemencoded,
            get_issuer_chain_size: value.get_issuer_chain_size,
            get_derencoded_issuer_chain: value.get_derencoded_issuer_chain,
            get_pemencoded_issuer_chain: value.get_pemencoded_issuer_chain,
        }
    }
}
impl Default for X509Certificate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_sslstatus_t`] for more documentation.
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
            is_secure_connection: value.is_secure_connection,
            get_cert_status: value.get_cert_status,
            get_sslversion: value.get_sslversion,
            get_content_status: value.get_content_status,
            get_x_509_certificate: value.get_x509_certificate,
        }
    }
}
impl From<Sslstatus> for _cef_sslstatus_t {
    fn from(value: Sslstatus) -> Self {
        Self {
            base: value.base.into(),
            is_secure_connection: value.is_secure_connection,
            get_cert_status: value.get_cert_status,
            get_sslversion: value.get_sslversion,
            get_content_status: value.get_content_status,
            get_x509_certificate: value.get_x_509_certificate,
        }
    }
}
impl Default for Sslstatus {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_navigation_entry_t`] for more documentation.
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
            is_valid: value.is_valid,
            get_url: value.get_url,
            get_display_url: value.get_display_url,
            get_original_url: value.get_original_url,
            get_title: value.get_title,
            get_transition_type: value.get_transition_type,
            has_post_data: value.has_post_data,
            get_completion_time: value.get_completion_time,
            get_http_status_code: value.get_http_status_code,
            get_sslstatus: value.get_sslstatus,
        }
    }
}
impl From<NavigationEntry> for _cef_navigation_entry_t {
    fn from(value: NavigationEntry) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            get_url: value.get_url,
            get_display_url: value.get_display_url,
            get_original_url: value.get_original_url,
            get_title: value.get_title,
            get_transition_type: value.get_transition_type,
            has_post_data: value.has_post_data,
            get_completion_time: value.get_completion_time,
            get_http_status_code: value.get_http_status_code,
            get_sslstatus: value.get_sslstatus,
        }
    }
}
impl Default for NavigationEntry {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_registration_t`] for more documentation.
#[derive(Clone)]
pub struct Registration(RefGuard<_cef_registration_t>);
pub trait ImplRegistration: Clone + Sized + Rc {
    fn get_raw(&self) -> *mut _cef_registration_t;
}
impl ImplRegistration for Registration {
    fn get_raw(&self) -> *mut _cef_registration_t {
        unsafe { RefGuard::into_raw(&self.0) }
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
    fn into_raw(self) -> *mut _cef_registration_t {
        ImplRegistration::get_raw(self)
    }
}
impl ConvertParam<*mut _cef_registration_t> for &mut Registration {
    fn into_raw(self) -> *mut _cef_registration_t {
        ImplRegistration::get_raw(self)
    }
}
impl ConvertReturnValue<Registration> for *mut _cef_registration_t {
    fn wrap_result(self) -> Registration {
        Registration(unsafe { RefGuard::from_raw(self) })
    }
}
impl From<Registration> for *mut _cef_registration_t {
    fn from(value: Registration) -> Self {
        let object = ImplRegistration::get_raw(&value);
        std::mem::forget(value);
        object
    }
}
impl Default for Registration {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_callback_t`] for more documentation.
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
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl From<Callback> for _cef_callback_t {
    fn from(value: Callback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl Default for Callback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_completion_callback_t`] for more documentation.
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
            on_complete: value.on_complete,
        }
    }
}
impl From<CompletionCallback> for _cef_completion_callback_t {
    fn from(value: CompletionCallback) -> Self {
        Self {
            base: value.base.into(),
            on_complete: value.on_complete,
        }
    }
}
impl Default for CompletionCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_cookie_manager_t`] for more documentation.
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
            visit_all_cookies: value.visit_all_cookies,
            visit_url_cookies: value.visit_url_cookies,
            set_cookie: value.set_cookie,
            delete_cookies: value.delete_cookies,
            flush_store: value.flush_store,
        }
    }
}
impl From<CookieManager> for _cef_cookie_manager_t {
    fn from(value: CookieManager) -> Self {
        Self {
            base: value.base.into(),
            visit_all_cookies: value.visit_all_cookies,
            visit_url_cookies: value.visit_url_cookies,
            set_cookie: value.set_cookie,
            delete_cookies: value.delete_cookies,
            flush_store: value.flush_store,
        }
    }
}
impl Default for CookieManager {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_cookie_visitor_t`] for more documentation.
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
            visit: value.visit,
        }
    }
}
impl From<CookieVisitor> for _cef_cookie_visitor_t {
    fn from(value: CookieVisitor) -> Self {
        Self {
            base: value.base.into(),
            visit: value.visit,
        }
    }
}
impl Default for CookieVisitor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_set_cookie_callback_t`] for more documentation.
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
            on_complete: value.on_complete,
        }
    }
}
impl From<SetCookieCallback> for _cef_set_cookie_callback_t {
    fn from(value: SetCookieCallback) -> Self {
        Self {
            base: value.base.into(),
            on_complete: value.on_complete,
        }
    }
}
impl Default for SetCookieCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_delete_cookies_callback_t`] for more documentation.
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
            on_complete: value.on_complete,
        }
    }
}
impl From<DeleteCookiesCallback> for _cef_delete_cookies_callback_t {
    fn from(value: DeleteCookiesCallback) -> Self {
        Self {
            base: value.base.into(),
            on_complete: value.on_complete,
        }
    }
}
impl Default for DeleteCookiesCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_media_router_t`] for more documentation.
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
            add_observer: value.add_observer,
            get_source: value.get_source,
            notify_current_sinks: value.notify_current_sinks,
            create_route: value.create_route,
            notify_current_routes: value.notify_current_routes,
        }
    }
}
impl From<MediaRouter> for _cef_media_router_t {
    fn from(value: MediaRouter) -> Self {
        Self {
            base: value.base.into(),
            add_observer: value.add_observer,
            get_source: value.get_source,
            notify_current_sinks: value.notify_current_sinks,
            create_route: value.create_route,
            notify_current_routes: value.notify_current_routes,
        }
    }
}
impl Default for MediaRouter {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_media_observer_t`] for more documentation.
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
            on_sinks: value.on_sinks,
            on_routes: value.on_routes,
            on_route_state_changed: value.on_route_state_changed,
            on_route_message_received: value.on_route_message_received,
        }
    }
}
impl From<MediaObserver> for _cef_media_observer_t {
    fn from(value: MediaObserver) -> Self {
        Self {
            base: value.base.into(),
            on_sinks: value.on_sinks,
            on_routes: value.on_routes,
            on_route_state_changed: value.on_route_state_changed,
            on_route_message_received: value.on_route_message_received,
        }
    }
}
impl Default for MediaObserver {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_media_route_t`] for more documentation.
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
            get_id: value.get_id,
            get_source: value.get_source,
            get_sink: value.get_sink,
            send_route_message: value.send_route_message,
            terminate: value.terminate,
        }
    }
}
impl From<MediaRoute> for _cef_media_route_t {
    fn from(value: MediaRoute) -> Self {
        Self {
            base: value.base.into(),
            get_id: value.get_id,
            get_source: value.get_source,
            get_sink: value.get_sink,
            send_route_message: value.send_route_message,
            terminate: value.terminate,
        }
    }
}
impl Default for MediaRoute {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_media_route_create_callback_t`] for more documentation.
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
            on_media_route_create_finished: value.on_media_route_create_finished,
        }
    }
}
impl From<MediaRouteCreateCallback> for _cef_media_route_create_callback_t {
    fn from(value: MediaRouteCreateCallback) -> Self {
        Self {
            base: value.base.into(),
            on_media_route_create_finished: value.on_media_route_create_finished,
        }
    }
}
impl Default for MediaRouteCreateCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_media_sink_t`] for more documentation.
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
            get_id: value.get_id,
            get_name: value.get_name,
            get_icon_type: value.get_icon_type,
            get_device_info: value.get_device_info,
            is_cast_sink: value.is_cast_sink,
            is_dial_sink: value.is_dial_sink,
            is_compatible_with: value.is_compatible_with,
        }
    }
}
impl From<MediaSink> for _cef_media_sink_t {
    fn from(value: MediaSink) -> Self {
        Self {
            base: value.base.into(),
            get_id: value.get_id,
            get_name: value.get_name,
            get_icon_type: value.get_icon_type,
            get_device_info: value.get_device_info,
            is_cast_sink: value.is_cast_sink,
            is_dial_sink: value.is_dial_sink,
            is_compatible_with: value.is_compatible_with,
        }
    }
}
impl Default for MediaSink {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_media_sink_device_info_callback_t`] for more documentation.
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
            on_media_sink_device_info: value.on_media_sink_device_info,
        }
    }
}
impl From<MediaSinkDeviceInfoCallback> for _cef_media_sink_device_info_callback_t {
    fn from(value: MediaSinkDeviceInfoCallback) -> Self {
        Self {
            base: value.base.into(),
            on_media_sink_device_info: value.on_media_sink_device_info,
        }
    }
}
impl Default for MediaSinkDeviceInfoCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_media_source_t`] for more documentation.
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
            get_id: value.get_id,
            is_cast_source: value.is_cast_source,
            is_dial_source: value.is_dial_source,
        }
    }
}
impl From<MediaSource> for _cef_media_source_t {
    fn from(value: MediaSource) -> Self {
        Self {
            base: value.base.into(),
            get_id: value.get_id,
            is_cast_source: value.is_cast_source,
            is_dial_source: value.is_dial_source,
        }
    }
}
impl Default for MediaSource {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_preference_registrar_t`] for more documentation.
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
            add_preference: value.add_preference,
        }
    }
}
impl From<PreferenceRegistrar> for _cef_preference_registrar_t {
    fn from(value: PreferenceRegistrar) -> Self {
        Self {
            base: value.base.into(),
            add_preference: value.add_preference,
        }
    }
}
impl Default for PreferenceRegistrar {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_preference_observer_t`] for more documentation.
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
            on_preference_changed: value.on_preference_changed,
        }
    }
}
impl From<PreferenceObserver> for _cef_preference_observer_t {
    fn from(value: PreferenceObserver) -> Self {
        Self {
            base: value.base.into(),
            on_preference_changed: value.on_preference_changed,
        }
    }
}
impl Default for PreferenceObserver {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_preference_manager_t`] for more documentation.
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
            has_preference: value.has_preference,
            get_preference: value.get_preference,
            get_all_preferences: value.get_all_preferences,
            can_set_preference: value.can_set_preference,
            set_preference: value.set_preference,
            add_preference_observer: value.add_preference_observer,
        }
    }
}
impl From<PreferenceManager> for _cef_preference_manager_t {
    fn from(value: PreferenceManager) -> Self {
        Self {
            base: value.base.into(),
            has_preference: value.has_preference,
            get_preference: value.get_preference,
            get_all_preferences: value.get_all_preferences,
            can_set_preference: value.can_set_preference,
            set_preference: value.set_preference,
            add_preference_observer: value.add_preference_observer,
        }
    }
}
impl Default for PreferenceManager {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_resolve_callback_t`] for more documentation.
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
            on_resolve_completed: value.on_resolve_completed,
        }
    }
}
impl From<ResolveCallback> for _cef_resolve_callback_t {
    fn from(value: ResolveCallback) -> Self {
        Self {
            base: value.base.into(),
            on_resolve_completed: value.on_resolve_completed,
        }
    }
}
impl Default for ResolveCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_setting_observer_t`] for more documentation.
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
            on_setting_changed: value.on_setting_changed,
        }
    }
}
impl From<SettingObserver> for _cef_setting_observer_t {
    fn from(value: SettingObserver) -> Self {
        Self {
            base: value.base.into(),
            on_setting_changed: value.on_setting_changed,
        }
    }
}
impl Default for SettingObserver {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_request_context_t`] for more documentation.
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
            is_same: value.is_same,
            is_sharing_with: value.is_sharing_with,
            is_global: value.is_global,
            get_handler: value.get_handler,
            get_cache_path: value.get_cache_path,
            get_cookie_manager: value.get_cookie_manager,
            register_scheme_handler_factory: value.register_scheme_handler_factory,
            clear_scheme_handler_factories: value.clear_scheme_handler_factories,
            clear_certificate_exceptions: value.clear_certificate_exceptions,
            clear_http_auth_credentials: value.clear_http_auth_credentials,
            close_all_connections: value.close_all_connections,
            resolve_host: value.resolve_host,
            get_media_router: value.get_media_router,
            get_website_setting: value.get_website_setting,
            set_website_setting: value.set_website_setting,
            get_content_setting: value.get_content_setting,
            set_content_setting: value.set_content_setting,
            set_chrome_color_scheme: value.set_chrome_color_scheme,
            get_chrome_color_scheme_mode: value.get_chrome_color_scheme_mode,
            get_chrome_color_scheme_color: value.get_chrome_color_scheme_color,
            get_chrome_color_scheme_variant: value.get_chrome_color_scheme_variant,
            add_setting_observer: value.add_setting_observer,
        }
    }
}
impl From<RequestContext> for _cef_request_context_t {
    fn from(value: RequestContext) -> Self {
        Self {
            base: value.base.into(),
            is_same: value.is_same,
            is_sharing_with: value.is_sharing_with,
            is_global: value.is_global,
            get_handler: value.get_handler,
            get_cache_path: value.get_cache_path,
            get_cookie_manager: value.get_cookie_manager,
            register_scheme_handler_factory: value.register_scheme_handler_factory,
            clear_scheme_handler_factories: value.clear_scheme_handler_factories,
            clear_certificate_exceptions: value.clear_certificate_exceptions,
            clear_http_auth_credentials: value.clear_http_auth_credentials,
            close_all_connections: value.close_all_connections,
            resolve_host: value.resolve_host,
            get_media_router: value.get_media_router,
            get_website_setting: value.get_website_setting,
            set_website_setting: value.set_website_setting,
            get_content_setting: value.get_content_setting,
            set_content_setting: value.set_content_setting,
            set_chrome_color_scheme: value.set_chrome_color_scheme,
            get_chrome_color_scheme_mode: value.get_chrome_color_scheme_mode,
            get_chrome_color_scheme_color: value.get_chrome_color_scheme_color,
            get_chrome_color_scheme_variant: value.get_chrome_color_scheme_variant,
            add_setting_observer: value.add_setting_observer,
        }
    }
}
impl Default for RequestContext {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_browser_t`] for more documentation.
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
            is_valid: value.is_valid,
            get_host: value.get_host,
            can_go_back: value.can_go_back,
            go_back: value.go_back,
            can_go_forward: value.can_go_forward,
            go_forward: value.go_forward,
            is_loading: value.is_loading,
            reload: value.reload,
            reload_ignore_cache: value.reload_ignore_cache,
            stop_load: value.stop_load,
            get_identifier: value.get_identifier,
            is_same: value.is_same,
            is_popup: value.is_popup,
            has_document: value.has_document,
            get_main_frame: value.get_main_frame,
            get_focused_frame: value.get_focused_frame,
            get_frame_by_identifier: value.get_frame_by_identifier,
            get_frame_by_name: value.get_frame_by_name,
            get_frame_count: value.get_frame_count,
            get_frame_identifiers: value.get_frame_identifiers,
            get_frame_names: value.get_frame_names,
        }
    }
}
impl From<Browser> for _cef_browser_t {
    fn from(value: Browser) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            get_host: value.get_host,
            can_go_back: value.can_go_back,
            go_back: value.go_back,
            can_go_forward: value.can_go_forward,
            go_forward: value.go_forward,
            is_loading: value.is_loading,
            reload: value.reload,
            reload_ignore_cache: value.reload_ignore_cache,
            stop_load: value.stop_load,
            get_identifier: value.get_identifier,
            is_same: value.is_same,
            is_popup: value.is_popup,
            has_document: value.has_document,
            get_main_frame: value.get_main_frame,
            get_focused_frame: value.get_focused_frame,
            get_frame_by_identifier: value.get_frame_by_identifier,
            get_frame_by_name: value.get_frame_by_name,
            get_frame_count: value.get_frame_count,
            get_frame_identifiers: value.get_frame_identifiers,
            get_frame_names: value.get_frame_names,
        }
    }
}
impl Default for Browser {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_run_file_dialog_callback_t`] for more documentation.
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
            on_file_dialog_dismissed: value.on_file_dialog_dismissed,
        }
    }
}
impl From<RunFileDialogCallback> for _cef_run_file_dialog_callback_t {
    fn from(value: RunFileDialogCallback) -> Self {
        Self {
            base: value.base.into(),
            on_file_dialog_dismissed: value.on_file_dialog_dismissed,
        }
    }
}
impl Default for RunFileDialogCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_navigation_entry_visitor_t`] for more documentation.
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
            visit: value.visit,
        }
    }
}
impl From<NavigationEntryVisitor> for _cef_navigation_entry_visitor_t {
    fn from(value: NavigationEntryVisitor) -> Self {
        Self {
            base: value.base.into(),
            visit: value.visit,
        }
    }
}
impl Default for NavigationEntryVisitor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_pdf_print_callback_t`] for more documentation.
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
            on_pdf_print_finished: value.on_pdf_print_finished,
        }
    }
}
impl From<PdfPrintCallback> for _cef_pdf_print_callback_t {
    fn from(value: PdfPrintCallback) -> Self {
        Self {
            base: value.base.into(),
            on_pdf_print_finished: value.on_pdf_print_finished,
        }
    }
}
impl Default for PdfPrintCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_download_image_callback_t`] for more documentation.
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
            on_download_image_finished: value.on_download_image_finished,
        }
    }
}
impl From<DownloadImageCallback> for _cef_download_image_callback_t {
    fn from(value: DownloadImageCallback) -> Self {
        Self {
            base: value.base.into(),
            on_download_image_finished: value.on_download_image_finished,
        }
    }
}
impl Default for DownloadImageCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_browser_host_t`] for more documentation.
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
            get_browser: value.get_browser,
            close_browser: value.close_browser,
            try_close_browser: value.try_close_browser,
            is_ready_to_be_closed: value.is_ready_to_be_closed,
            set_focus: value.set_focus,
            get_window_handle: value.get_window_handle,
            get_opener_window_handle: value.get_opener_window_handle,
            get_opener_identifier: value.get_opener_identifier,
            has_view: value.has_view,
            get_client: value.get_client,
            get_request_context: value.get_request_context,
            can_zoom: value.can_zoom,
            zoom: value.zoom,
            get_default_zoom_level: value.get_default_zoom_level,
            get_zoom_level: value.get_zoom_level,
            set_zoom_level: value.set_zoom_level,
            run_file_dialog: value.run_file_dialog,
            start_download: value.start_download,
            download_image: value.download_image,
            print: value.print,
            print_to_pdf: value.print_to_pdf,
            find: value.find,
            stop_finding: value.stop_finding,
            show_dev_tools: value.show_dev_tools,
            close_dev_tools: value.close_dev_tools,
            has_dev_tools: value.has_dev_tools,
            send_dev_tools_message: value.send_dev_tools_message,
            execute_dev_tools_method: value.execute_dev_tools_method,
            add_dev_tools_message_observer: value.add_dev_tools_message_observer,
            get_navigation_entries: value.get_navigation_entries,
            replace_misspelling: value.replace_misspelling,
            add_word_to_dictionary: value.add_word_to_dictionary,
            is_window_rendering_disabled: value.is_window_rendering_disabled,
            was_resized: value.was_resized,
            was_hidden: value.was_hidden,
            notify_screen_info_changed: value.notify_screen_info_changed,
            invalidate: value.invalidate,
            send_external_begin_frame: value.send_external_begin_frame,
            send_key_event: value.send_key_event,
            send_mouse_click_event: value.send_mouse_click_event,
            send_mouse_move_event: value.send_mouse_move_event,
            send_mouse_wheel_event: value.send_mouse_wheel_event,
            send_touch_event: value.send_touch_event,
            send_capture_lost_event: value.send_capture_lost_event,
            notify_move_or_resize_started: value.notify_move_or_resize_started,
            get_windowless_frame_rate: value.get_windowless_frame_rate,
            set_windowless_frame_rate: value.set_windowless_frame_rate,
            ime_set_composition: value.ime_set_composition,
            ime_commit_text: value.ime_commit_text,
            ime_finish_composing_text: value.ime_finish_composing_text,
            ime_cancel_composition: value.ime_cancel_composition,
            drag_target_drag_enter: value.drag_target_drag_enter,
            drag_target_drag_over: value.drag_target_drag_over,
            drag_target_drag_leave: value.drag_target_drag_leave,
            drag_target_drop: value.drag_target_drop,
            drag_source_ended_at: value.drag_source_ended_at,
            drag_source_system_drag_ended: value.drag_source_system_drag_ended,
            get_visible_navigation_entry: value.get_visible_navigation_entry,
            set_accessibility_state: value.set_accessibility_state,
            set_auto_resize_enabled: value.set_auto_resize_enabled,
            set_audio_muted: value.set_audio_muted,
            is_audio_muted: value.is_audio_muted,
            is_fullscreen: value.is_fullscreen,
            exit_fullscreen: value.exit_fullscreen,
            can_execute_chrome_command: value.can_execute_chrome_command,
            execute_chrome_command: value.execute_chrome_command,
            is_render_process_unresponsive: value.is_render_process_unresponsive,
            get_runtime_style: value.get_runtime_style,
        }
    }
}
impl From<BrowserHost> for _cef_browser_host_t {
    fn from(value: BrowserHost) -> Self {
        Self {
            base: value.base.into(),
            get_browser: value.get_browser,
            close_browser: value.close_browser,
            try_close_browser: value.try_close_browser,
            is_ready_to_be_closed: value.is_ready_to_be_closed,
            set_focus: value.set_focus,
            get_window_handle: value.get_window_handle,
            get_opener_window_handle: value.get_opener_window_handle,
            get_opener_identifier: value.get_opener_identifier,
            has_view: value.has_view,
            get_client: value.get_client,
            get_request_context: value.get_request_context,
            can_zoom: value.can_zoom,
            zoom: value.zoom,
            get_default_zoom_level: value.get_default_zoom_level,
            get_zoom_level: value.get_zoom_level,
            set_zoom_level: value.set_zoom_level,
            run_file_dialog: value.run_file_dialog,
            start_download: value.start_download,
            download_image: value.download_image,
            print: value.print,
            print_to_pdf: value.print_to_pdf,
            find: value.find,
            stop_finding: value.stop_finding,
            show_dev_tools: value.show_dev_tools,
            close_dev_tools: value.close_dev_tools,
            has_dev_tools: value.has_dev_tools,
            send_dev_tools_message: value.send_dev_tools_message,
            execute_dev_tools_method: value.execute_dev_tools_method,
            add_dev_tools_message_observer: value.add_dev_tools_message_observer,
            get_navigation_entries: value.get_navigation_entries,
            replace_misspelling: value.replace_misspelling,
            add_word_to_dictionary: value.add_word_to_dictionary,
            is_window_rendering_disabled: value.is_window_rendering_disabled,
            was_resized: value.was_resized,
            was_hidden: value.was_hidden,
            notify_screen_info_changed: value.notify_screen_info_changed,
            invalidate: value.invalidate,
            send_external_begin_frame: value.send_external_begin_frame,
            send_key_event: value.send_key_event,
            send_mouse_click_event: value.send_mouse_click_event,
            send_mouse_move_event: value.send_mouse_move_event,
            send_mouse_wheel_event: value.send_mouse_wheel_event,
            send_touch_event: value.send_touch_event,
            send_capture_lost_event: value.send_capture_lost_event,
            notify_move_or_resize_started: value.notify_move_or_resize_started,
            get_windowless_frame_rate: value.get_windowless_frame_rate,
            set_windowless_frame_rate: value.set_windowless_frame_rate,
            ime_set_composition: value.ime_set_composition,
            ime_commit_text: value.ime_commit_text,
            ime_finish_composing_text: value.ime_finish_composing_text,
            ime_cancel_composition: value.ime_cancel_composition,
            drag_target_drag_enter: value.drag_target_drag_enter,
            drag_target_drag_over: value.drag_target_drag_over,
            drag_target_drag_leave: value.drag_target_drag_leave,
            drag_target_drop: value.drag_target_drop,
            drag_source_ended_at: value.drag_source_ended_at,
            drag_source_system_drag_ended: value.drag_source_system_drag_ended,
            get_visible_navigation_entry: value.get_visible_navigation_entry,
            set_accessibility_state: value.set_accessibility_state,
            set_auto_resize_enabled: value.set_auto_resize_enabled,
            set_audio_muted: value.set_audio_muted,
            is_audio_muted: value.is_audio_muted,
            is_fullscreen: value.is_fullscreen,
            exit_fullscreen: value.exit_fullscreen,
            can_execute_chrome_command: value.can_execute_chrome_command,
            execute_chrome_command: value.execute_chrome_command,
            is_render_process_unresponsive: value.is_render_process_unresponsive,
            get_runtime_style: value.get_runtime_style,
        }
    }
}
impl Default for BrowserHost {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_audio_handler_t`] for more documentation.
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
            get_audio_parameters: value.get_audio_parameters,
            on_audio_stream_started: value.on_audio_stream_started,
            on_audio_stream_packet: value.on_audio_stream_packet,
            on_audio_stream_stopped: value.on_audio_stream_stopped,
            on_audio_stream_error: value.on_audio_stream_error,
        }
    }
}
impl From<AudioHandler> for _cef_audio_handler_t {
    fn from(value: AudioHandler) -> Self {
        Self {
            base: value.base.into(),
            get_audio_parameters: value.get_audio_parameters,
            on_audio_stream_started: value.on_audio_stream_started,
            on_audio_stream_packet: value.on_audio_stream_packet,
            on_audio_stream_stopped: value.on_audio_stream_stopped,
            on_audio_stream_error: value.on_audio_stream_error,
        }
    }
}
impl Default for AudioHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_command_handler_t`] for more documentation.
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
            on_chrome_command: value.on_chrome_command,
            is_chrome_app_menu_item_visible: value.is_chrome_app_menu_item_visible,
            is_chrome_app_menu_item_enabled: value.is_chrome_app_menu_item_enabled,
            is_chrome_page_action_icon_visible: value.is_chrome_page_action_icon_visible,
            is_chrome_toolbar_button_visible: value.is_chrome_toolbar_button_visible,
        }
    }
}
impl From<CommandHandler> for _cef_command_handler_t {
    fn from(value: CommandHandler) -> Self {
        Self {
            base: value.base.into(),
            on_chrome_command: value.on_chrome_command,
            is_chrome_app_menu_item_visible: value.is_chrome_app_menu_item_visible,
            is_chrome_app_menu_item_enabled: value.is_chrome_app_menu_item_enabled,
            is_chrome_page_action_icon_visible: value.is_chrome_page_action_icon_visible,
            is_chrome_toolbar_button_visible: value.is_chrome_toolbar_button_visible,
        }
    }
}
impl Default for CommandHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_menu_model_delegate_t`] for more documentation.
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
            execute_command: value.execute_command,
            mouse_outside_menu: value.mouse_outside_menu,
            unhandled_open_submenu: value.unhandled_open_submenu,
            unhandled_close_submenu: value.unhandled_close_submenu,
            menu_will_show: value.menu_will_show,
            menu_closed: value.menu_closed,
            format_label: value.format_label,
        }
    }
}
impl From<MenuModelDelegate> for _cef_menu_model_delegate_t {
    fn from(value: MenuModelDelegate) -> Self {
        Self {
            base: value.base.into(),
            execute_command: value.execute_command,
            mouse_outside_menu: value.mouse_outside_menu,
            unhandled_open_submenu: value.unhandled_open_submenu,
            unhandled_close_submenu: value.unhandled_close_submenu,
            menu_will_show: value.menu_will_show,
            menu_closed: value.menu_closed,
            format_label: value.format_label,
        }
    }
}
impl Default for MenuModelDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_menu_model_t`] for more documentation.
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
            is_sub_menu: value.is_sub_menu,
            clear: value.clear,
            get_count: value.get_count,
            add_separator: value.add_separator,
            add_item: value.add_item,
            add_check_item: value.add_check_item,
            add_radio_item: value.add_radio_item,
            add_sub_menu: value.add_sub_menu,
            insert_separator_at: value.insert_separator_at,
            insert_item_at: value.insert_item_at,
            insert_check_item_at: value.insert_check_item_at,
            insert_radio_item_at: value.insert_radio_item_at,
            insert_sub_menu_at: value.insert_sub_menu_at,
            remove: value.remove,
            remove_at: value.remove_at,
            get_index_of: value.get_index_of,
            get_command_id_at: value.get_command_id_at,
            set_command_id_at: value.set_command_id_at,
            get_label: value.get_label,
            get_label_at: value.get_label_at,
            set_label: value.set_label,
            set_label_at: value.set_label_at,
            get_type: value.get_type,
            get_type_at: value.get_type_at,
            get_group_id: value.get_group_id,
            get_group_id_at: value.get_group_id_at,
            set_group_id: value.set_group_id,
            set_group_id_at: value.set_group_id_at,
            get_sub_menu: value.get_sub_menu,
            get_sub_menu_at: value.get_sub_menu_at,
            is_visible: value.is_visible,
            is_visible_at: value.is_visible_at,
            set_visible: value.set_visible,
            set_visible_at: value.set_visible_at,
            is_enabled: value.is_enabled,
            is_enabled_at: value.is_enabled_at,
            set_enabled: value.set_enabled,
            set_enabled_at: value.set_enabled_at,
            is_checked: value.is_checked,
            is_checked_at: value.is_checked_at,
            set_checked: value.set_checked,
            set_checked_at: value.set_checked_at,
            has_accelerator: value.has_accelerator,
            has_accelerator_at: value.has_accelerator_at,
            set_accelerator: value.set_accelerator,
            set_accelerator_at: value.set_accelerator_at,
            remove_accelerator: value.remove_accelerator,
            remove_accelerator_at: value.remove_accelerator_at,
            get_accelerator: value.get_accelerator,
            get_accelerator_at: value.get_accelerator_at,
            set_color: value.set_color,
            set_color_at: value.set_color_at,
            get_color: value.get_color,
            get_color_at: value.get_color_at,
            set_font_list: value.set_font_list,
            set_font_list_at: value.set_font_list_at,
        }
    }
}
impl From<MenuModel> for _cef_menu_model_t {
    fn from(value: MenuModel) -> Self {
        Self {
            base: value.base.into(),
            is_sub_menu: value.is_sub_menu,
            clear: value.clear,
            get_count: value.get_count,
            add_separator: value.add_separator,
            add_item: value.add_item,
            add_check_item: value.add_check_item,
            add_radio_item: value.add_radio_item,
            add_sub_menu: value.add_sub_menu,
            insert_separator_at: value.insert_separator_at,
            insert_item_at: value.insert_item_at,
            insert_check_item_at: value.insert_check_item_at,
            insert_radio_item_at: value.insert_radio_item_at,
            insert_sub_menu_at: value.insert_sub_menu_at,
            remove: value.remove,
            remove_at: value.remove_at,
            get_index_of: value.get_index_of,
            get_command_id_at: value.get_command_id_at,
            set_command_id_at: value.set_command_id_at,
            get_label: value.get_label,
            get_label_at: value.get_label_at,
            set_label: value.set_label,
            set_label_at: value.set_label_at,
            get_type: value.get_type,
            get_type_at: value.get_type_at,
            get_group_id: value.get_group_id,
            get_group_id_at: value.get_group_id_at,
            set_group_id: value.set_group_id,
            set_group_id_at: value.set_group_id_at,
            get_sub_menu: value.get_sub_menu,
            get_sub_menu_at: value.get_sub_menu_at,
            is_visible: value.is_visible,
            is_visible_at: value.is_visible_at,
            set_visible: value.set_visible,
            set_visible_at: value.set_visible_at,
            is_enabled: value.is_enabled,
            is_enabled_at: value.is_enabled_at,
            set_enabled: value.set_enabled,
            set_enabled_at: value.set_enabled_at,
            is_checked: value.is_checked,
            is_checked_at: value.is_checked_at,
            set_checked: value.set_checked,
            set_checked_at: value.set_checked_at,
            has_accelerator: value.has_accelerator,
            has_accelerator_at: value.has_accelerator_at,
            set_accelerator: value.set_accelerator,
            set_accelerator_at: value.set_accelerator_at,
            remove_accelerator: value.remove_accelerator,
            remove_accelerator_at: value.remove_accelerator_at,
            get_accelerator: value.get_accelerator,
            get_accelerator_at: value.get_accelerator_at,
            set_color: value.set_color,
            set_color_at: value.set_color_at,
            get_color: value.get_color,
            get_color_at: value.get_color_at,
            set_font_list: value.set_font_list,
            set_font_list_at: value.set_font_list_at,
        }
    }
}
impl Default for MenuModel {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_run_context_menu_callback_t`] for more documentation.
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
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl From<RunContextMenuCallback> for _cef_run_context_menu_callback_t {
    fn from(value: RunContextMenuCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl Default for RunContextMenuCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_run_quick_menu_callback_t`] for more documentation.
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
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl From<RunQuickMenuCallback> for _cef_run_quick_menu_callback_t {
    fn from(value: RunQuickMenuCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl Default for RunQuickMenuCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_context_menu_handler_t`] for more documentation.
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
            on_before_context_menu: value.on_before_context_menu,
            run_context_menu: value.run_context_menu,
            on_context_menu_command: value.on_context_menu_command,
            on_context_menu_dismissed: value.on_context_menu_dismissed,
            run_quick_menu: value.run_quick_menu,
            on_quick_menu_command: value.on_quick_menu_command,
            on_quick_menu_dismissed: value.on_quick_menu_dismissed,
        }
    }
}
impl From<ContextMenuHandler> for _cef_context_menu_handler_t {
    fn from(value: ContextMenuHandler) -> Self {
        Self {
            base: value.base.into(),
            on_before_context_menu: value.on_before_context_menu,
            run_context_menu: value.run_context_menu,
            on_context_menu_command: value.on_context_menu_command,
            on_context_menu_dismissed: value.on_context_menu_dismissed,
            run_quick_menu: value.run_quick_menu,
            on_quick_menu_command: value.on_quick_menu_command,
            on_quick_menu_dismissed: value.on_quick_menu_dismissed,
        }
    }
}
impl Default for ContextMenuHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_context_menu_params_t`] for more documentation.
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
            get_xcoord: value.get_xcoord,
            get_ycoord: value.get_ycoord,
            get_type_flags: value.get_type_flags,
            get_link_url: value.get_link_url,
            get_unfiltered_link_url: value.get_unfiltered_link_url,
            get_source_url: value.get_source_url,
            has_image_contents: value.has_image_contents,
            get_title_text: value.get_title_text,
            get_page_url: value.get_page_url,
            get_frame_url: value.get_frame_url,
            get_frame_charset: value.get_frame_charset,
            get_media_type: value.get_media_type,
            get_media_state_flags: value.get_media_state_flags,
            get_selection_text: value.get_selection_text,
            get_misspelled_word: value.get_misspelled_word,
            get_dictionary_suggestions: value.get_dictionary_suggestions,
            is_editable: value.is_editable,
            is_spell_check_enabled: value.is_spell_check_enabled,
            get_edit_state_flags: value.get_edit_state_flags,
            is_custom_menu: value.is_custom_menu,
        }
    }
}
impl From<ContextMenuParams> for _cef_context_menu_params_t {
    fn from(value: ContextMenuParams) -> Self {
        Self {
            base: value.base.into(),
            get_xcoord: value.get_xcoord,
            get_ycoord: value.get_ycoord,
            get_type_flags: value.get_type_flags,
            get_link_url: value.get_link_url,
            get_unfiltered_link_url: value.get_unfiltered_link_url,
            get_source_url: value.get_source_url,
            has_image_contents: value.has_image_contents,
            get_title_text: value.get_title_text,
            get_page_url: value.get_page_url,
            get_frame_url: value.get_frame_url,
            get_frame_charset: value.get_frame_charset,
            get_media_type: value.get_media_type,
            get_media_state_flags: value.get_media_state_flags,
            get_selection_text: value.get_selection_text,
            get_misspelled_word: value.get_misspelled_word,
            get_dictionary_suggestions: value.get_dictionary_suggestions,
            is_editable: value.is_editable,
            is_spell_check_enabled: value.is_spell_check_enabled,
            get_edit_state_flags: value.get_edit_state_flags,
            is_custom_menu: value.is_custom_menu,
        }
    }
}
impl Default for ContextMenuParams {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_file_dialog_callback_t`] for more documentation.
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
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl From<FileDialogCallback> for _cef_file_dialog_callback_t {
    fn from(value: FileDialogCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl Default for FileDialogCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_dialog_handler_t`] for more documentation.
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
            on_file_dialog: value.on_file_dialog,
        }
    }
}
impl From<DialogHandler> for _cef_dialog_handler_t {
    fn from(value: DialogHandler) -> Self {
        Self {
            base: value.base.into(),
            on_file_dialog: value.on_file_dialog,
        }
    }
}
impl Default for DialogHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_display_handler_t`] for more documentation.
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
    pub on_contents_bounds_change: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            new_bounds: *const cef_rect_t,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_root_window_screen_rect: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_display_handler_t,
            browser: *mut _cef_browser_t,
            rect: *mut cef_rect_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_display_handler_t> for DisplayHandler {
    fn from(value: _cef_display_handler_t) -> Self {
        Self {
            base: value.base.into(),
            on_address_change: value.on_address_change,
            on_title_change: value.on_title_change,
            on_favicon_urlchange: value.on_favicon_urlchange,
            on_fullscreen_mode_change: value.on_fullscreen_mode_change,
            on_tooltip: value.on_tooltip,
            on_status_message: value.on_status_message,
            on_console_message: value.on_console_message,
            on_auto_resize: value.on_auto_resize,
            on_loading_progress_change: value.on_loading_progress_change,
            on_cursor_change: value.on_cursor_change,
            on_media_access_change: value.on_media_access_change,
            on_contents_bounds_change: value.on_contents_bounds_change,
            get_root_window_screen_rect: value.get_root_window_screen_rect,
        }
    }
}
impl From<DisplayHandler> for _cef_display_handler_t {
    fn from(value: DisplayHandler) -> Self {
        Self {
            base: value.base.into(),
            on_address_change: value.on_address_change,
            on_title_change: value.on_title_change,
            on_favicon_urlchange: value.on_favicon_urlchange,
            on_fullscreen_mode_change: value.on_fullscreen_mode_change,
            on_tooltip: value.on_tooltip,
            on_status_message: value.on_status_message,
            on_console_message: value.on_console_message,
            on_auto_resize: value.on_auto_resize,
            on_loading_progress_change: value.on_loading_progress_change,
            on_cursor_change: value.on_cursor_change,
            on_media_access_change: value.on_media_access_change,
            on_contents_bounds_change: value.on_contents_bounds_change,
            get_root_window_screen_rect: value.get_root_window_screen_rect,
        }
    }
}
impl Default for DisplayHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_download_item_t`] for more documentation.
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
            is_valid: value.is_valid,
            is_in_progress: value.is_in_progress,
            is_complete: value.is_complete,
            is_canceled: value.is_canceled,
            is_interrupted: value.is_interrupted,
            get_interrupt_reason: value.get_interrupt_reason,
            get_current_speed: value.get_current_speed,
            get_percent_complete: value.get_percent_complete,
            get_total_bytes: value.get_total_bytes,
            get_received_bytes: value.get_received_bytes,
            get_start_time: value.get_start_time,
            get_end_time: value.get_end_time,
            get_full_path: value.get_full_path,
            get_id: value.get_id,
            get_url: value.get_url,
            get_original_url: value.get_original_url,
            get_suggested_file_name: value.get_suggested_file_name,
            get_content_disposition: value.get_content_disposition,
            get_mime_type: value.get_mime_type,
        }
    }
}
impl From<DownloadItem> for _cef_download_item_t {
    fn from(value: DownloadItem) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            is_in_progress: value.is_in_progress,
            is_complete: value.is_complete,
            is_canceled: value.is_canceled,
            is_interrupted: value.is_interrupted,
            get_interrupt_reason: value.get_interrupt_reason,
            get_current_speed: value.get_current_speed,
            get_percent_complete: value.get_percent_complete,
            get_total_bytes: value.get_total_bytes,
            get_received_bytes: value.get_received_bytes,
            get_start_time: value.get_start_time,
            get_end_time: value.get_end_time,
            get_full_path: value.get_full_path,
            get_id: value.get_id,
            get_url: value.get_url,
            get_original_url: value.get_original_url,
            get_suggested_file_name: value.get_suggested_file_name,
            get_content_disposition: value.get_content_disposition,
            get_mime_type: value.get_mime_type,
        }
    }
}
impl Default for DownloadItem {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_before_download_callback_t`] for more documentation.
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
            cont: value.cont,
        }
    }
}
impl From<BeforeDownloadCallback> for _cef_before_download_callback_t {
    fn from(value: BeforeDownloadCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
        }
    }
}
impl Default for BeforeDownloadCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_download_item_callback_t`] for more documentation.
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
            cancel: value.cancel,
            pause: value.pause,
            resume: value.resume,
        }
    }
}
impl From<DownloadItemCallback> for _cef_download_item_callback_t {
    fn from(value: DownloadItemCallback) -> Self {
        Self {
            base: value.base.into(),
            cancel: value.cancel,
            pause: value.pause,
            resume: value.resume,
        }
    }
}
impl Default for DownloadItemCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_download_handler_t`] for more documentation.
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
            can_download: value.can_download,
            on_before_download: value.on_before_download,
            on_download_updated: value.on_download_updated,
        }
    }
}
impl From<DownloadHandler> for _cef_download_handler_t {
    fn from(value: DownloadHandler) -> Self {
        Self {
            base: value.base.into(),
            can_download: value.can_download,
            on_before_download: value.on_before_download,
            on_download_updated: value.on_download_updated,
        }
    }
}
impl Default for DownloadHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_drag_handler_t`] for more documentation.
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
            on_drag_enter: value.on_drag_enter,
            on_draggable_regions_changed: value.on_draggable_regions_changed,
        }
    }
}
impl From<DragHandler> for _cef_drag_handler_t {
    fn from(value: DragHandler) -> Self {
        Self {
            base: value.base.into(),
            on_drag_enter: value.on_drag_enter,
            on_draggable_regions_changed: value.on_draggable_regions_changed,
        }
    }
}
impl Default for DragHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_find_handler_t`] for more documentation.
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
            on_find_result: value.on_find_result,
        }
    }
}
impl From<FindHandler> for _cef_find_handler_t {
    fn from(value: FindHandler) -> Self {
        Self {
            base: value.base.into(),
            on_find_result: value.on_find_result,
        }
    }
}
impl Default for FindHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_focus_handler_t`] for more documentation.
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
            on_take_focus: value.on_take_focus,
            on_set_focus: value.on_set_focus,
            on_got_focus: value.on_got_focus,
        }
    }
}
impl From<FocusHandler> for _cef_focus_handler_t {
    fn from(value: FocusHandler) -> Self {
        Self {
            base: value.base.into(),
            on_take_focus: value.on_take_focus,
            on_set_focus: value.on_set_focus,
            on_got_focus: value.on_got_focus,
        }
    }
}
impl Default for FocusHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_frame_handler_t`] for more documentation.
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
            on_frame_created: value.on_frame_created,
            on_frame_destroyed: value.on_frame_destroyed,
            on_frame_attached: value.on_frame_attached,
            on_frame_detached: value.on_frame_detached,
            on_main_frame_changed: value.on_main_frame_changed,
        }
    }
}
impl From<FrameHandler> for _cef_frame_handler_t {
    fn from(value: FrameHandler) -> Self {
        Self {
            base: value.base.into(),
            on_frame_created: value.on_frame_created,
            on_frame_destroyed: value.on_frame_destroyed,
            on_frame_attached: value.on_frame_attached,
            on_frame_detached: value.on_frame_detached,
            on_main_frame_changed: value.on_main_frame_changed,
        }
    }
}
impl Default for FrameHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_jsdialog_callback_t`] for more documentation.
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
            cont: value.cont,
        }
    }
}
impl From<JsdialogCallback> for _cef_jsdialog_callback_t {
    fn from(value: JsdialogCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
        }
    }
}
impl Default for JsdialogCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_jsdialog_handler_t`] for more documentation.
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
            on_jsdialog: value.on_jsdialog,
            on_before_unload_dialog: value.on_before_unload_dialog,
            on_reset_dialog_state: value.on_reset_dialog_state,
            on_dialog_closed: value.on_dialog_closed,
        }
    }
}
impl From<JsdialogHandler> for _cef_jsdialog_handler_t {
    fn from(value: JsdialogHandler) -> Self {
        Self {
            base: value.base.into(),
            on_jsdialog: value.on_jsdialog,
            on_before_unload_dialog: value.on_before_unload_dialog,
            on_reset_dialog_state: value.on_reset_dialog_state,
            on_dialog_closed: value.on_dialog_closed,
        }
    }
}
impl Default for JsdialogHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_keyboard_handler_t`] for more documentation.
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
            on_pre_key_event: value.on_pre_key_event,
            on_key_event: value.on_key_event,
        }
    }
}
impl From<KeyboardHandler> for _cef_keyboard_handler_t {
    fn from(value: KeyboardHandler) -> Self {
        Self {
            base: value.base.into(),
            on_pre_key_event: value.on_pre_key_event,
            on_key_event: value.on_key_event,
        }
    }
}
impl Default for KeyboardHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_life_span_handler_t`] for more documentation.
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
            on_before_popup: value.on_before_popup,
            on_before_popup_aborted: value.on_before_popup_aborted,
            on_before_dev_tools_popup: value.on_before_dev_tools_popup,
            on_after_created: value.on_after_created,
            do_close: value.do_close,
            on_before_close: value.on_before_close,
        }
    }
}
impl From<LifeSpanHandler> for _cef_life_span_handler_t {
    fn from(value: LifeSpanHandler) -> Self {
        Self {
            base: value.base.into(),
            on_before_popup: value.on_before_popup,
            on_before_popup_aborted: value.on_before_popup_aborted,
            on_before_dev_tools_popup: value.on_before_dev_tools_popup,
            on_after_created: value.on_after_created,
            do_close: value.do_close,
            on_before_close: value.on_before_close,
        }
    }
}
impl Default for LifeSpanHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_load_handler_t`] for more documentation.
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
            on_loading_state_change: value.on_loading_state_change,
            on_load_start: value.on_load_start,
            on_load_end: value.on_load_end,
            on_load_error: value.on_load_error,
        }
    }
}
impl From<LoadHandler> for _cef_load_handler_t {
    fn from(value: LoadHandler) -> Self {
        Self {
            base: value.base.into(),
            on_loading_state_change: value.on_loading_state_change,
            on_load_start: value.on_load_start,
            on_load_end: value.on_load_end,
            on_load_error: value.on_load_error,
        }
    }
}
impl Default for LoadHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_media_access_callback_t`] for more documentation.
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
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl From<MediaAccessCallback> for _cef_media_access_callback_t {
    fn from(value: MediaAccessCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl Default for MediaAccessCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_permission_prompt_callback_t`] for more documentation.
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
            cont: value.cont,
        }
    }
}
impl From<PermissionPromptCallback> for _cef_permission_prompt_callback_t {
    fn from(value: PermissionPromptCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
        }
    }
}
impl Default for PermissionPromptCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_permission_handler_t`] for more documentation.
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
            on_request_media_access_permission: value.on_request_media_access_permission,
            on_show_permission_prompt: value.on_show_permission_prompt,
            on_dismiss_permission_prompt: value.on_dismiss_permission_prompt,
        }
    }
}
impl From<PermissionHandler> for _cef_permission_handler_t {
    fn from(value: PermissionHandler) -> Self {
        Self {
            base: value.base.into(),
            on_request_media_access_permission: value.on_request_media_access_permission,
            on_show_permission_prompt: value.on_show_permission_prompt,
            on_dismiss_permission_prompt: value.on_dismiss_permission_prompt,
        }
    }
}
impl Default for PermissionHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_print_settings_t`] for more documentation.
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
            is_valid: value.is_valid,
            is_read_only: value.is_read_only,
            set_orientation: value.set_orientation,
            is_landscape: value.is_landscape,
            set_printer_printable_area: value.set_printer_printable_area,
            set_device_name: value.set_device_name,
            get_device_name: value.get_device_name,
            set_dpi: value.set_dpi,
            get_dpi: value.get_dpi,
            set_page_ranges: value.set_page_ranges,
            get_page_ranges_count: value.get_page_ranges_count,
            get_page_ranges: value.get_page_ranges,
            set_selection_only: value.set_selection_only,
            is_selection_only: value.is_selection_only,
            set_collate: value.set_collate,
            will_collate: value.will_collate,
            set_color_model: value.set_color_model,
            get_color_model: value.get_color_model,
            set_copies: value.set_copies,
            get_copies: value.get_copies,
            set_duplex_mode: value.set_duplex_mode,
            get_duplex_mode: value.get_duplex_mode,
        }
    }
}
impl From<PrintSettings> for _cef_print_settings_t {
    fn from(value: PrintSettings) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            is_read_only: value.is_read_only,
            set_orientation: value.set_orientation,
            is_landscape: value.is_landscape,
            set_printer_printable_area: value.set_printer_printable_area,
            set_device_name: value.set_device_name,
            get_device_name: value.get_device_name,
            set_dpi: value.set_dpi,
            get_dpi: value.get_dpi,
            set_page_ranges: value.set_page_ranges,
            get_page_ranges_count: value.get_page_ranges_count,
            get_page_ranges: value.get_page_ranges,
            set_selection_only: value.set_selection_only,
            is_selection_only: value.is_selection_only,
            set_collate: value.set_collate,
            will_collate: value.will_collate,
            set_color_model: value.set_color_model,
            get_color_model: value.get_color_model,
            set_copies: value.set_copies,
            get_copies: value.get_copies,
            set_duplex_mode: value.set_duplex_mode,
            get_duplex_mode: value.get_duplex_mode,
        }
    }
}
impl Default for PrintSettings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_print_dialog_callback_t`] for more documentation.
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
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl From<PrintDialogCallback> for _cef_print_dialog_callback_t {
    fn from(value: PrintDialogCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl Default for PrintDialogCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_print_job_callback_t`] for more documentation.
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
            cont: value.cont,
        }
    }
}
impl From<PrintJobCallback> for _cef_print_job_callback_t {
    fn from(value: PrintJobCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
        }
    }
}
impl Default for PrintJobCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_print_handler_t`] for more documentation.
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
            on_print_start: value.on_print_start,
            on_print_settings: value.on_print_settings,
            on_print_dialog: value.on_print_dialog,
            on_print_job: value.on_print_job,
            on_print_reset: value.on_print_reset,
            get_pdf_paper_size: value.get_pdf_paper_size,
        }
    }
}
impl From<PrintHandler> for _cef_print_handler_t {
    fn from(value: PrintHandler) -> Self {
        Self {
            base: value.base.into(),
            on_print_start: value.on_print_start,
            on_print_settings: value.on_print_settings,
            on_print_dialog: value.on_print_dialog,
            on_print_job: value.on_print_job,
            on_print_reset: value.on_print_reset,
            get_pdf_paper_size: value.get_pdf_paper_size,
        }
    }
}
impl Default for PrintHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_accessibility_handler_t`] for more documentation.
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
            on_accessibility_tree_change: value.on_accessibility_tree_change,
            on_accessibility_location_change: value.on_accessibility_location_change,
        }
    }
}
impl From<AccessibilityHandler> for _cef_accessibility_handler_t {
    fn from(value: AccessibilityHandler) -> Self {
        Self {
            base: value.base.into(),
            on_accessibility_tree_change: value.on_accessibility_tree_change,
            on_accessibility_location_change: value.on_accessibility_location_change,
        }
    }
}
impl Default for AccessibilityHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_render_handler_t`] for more documentation.
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
            get_accessibility_handler: value.get_accessibility_handler,
            get_root_screen_rect: value.get_root_screen_rect,
            get_view_rect: value.get_view_rect,
            get_screen_point: value.get_screen_point,
            get_screen_info: value.get_screen_info,
            on_popup_show: value.on_popup_show,
            on_popup_size: value.on_popup_size,
            on_paint: value.on_paint,
            on_accelerated_paint: value.on_accelerated_paint,
            get_touch_handle_size: value.get_touch_handle_size,
            on_touch_handle_state_changed: value.on_touch_handle_state_changed,
            start_dragging: value.start_dragging,
            update_drag_cursor: value.update_drag_cursor,
            on_scroll_offset_changed: value.on_scroll_offset_changed,
            on_ime_composition_range_changed: value.on_ime_composition_range_changed,
            on_text_selection_changed: value.on_text_selection_changed,
            on_virtual_keyboard_requested: value.on_virtual_keyboard_requested,
        }
    }
}
impl From<RenderHandler> for _cef_render_handler_t {
    fn from(value: RenderHandler) -> Self {
        Self {
            base: value.base.into(),
            get_accessibility_handler: value.get_accessibility_handler,
            get_root_screen_rect: value.get_root_screen_rect,
            get_view_rect: value.get_view_rect,
            get_screen_point: value.get_screen_point,
            get_screen_info: value.get_screen_info,
            on_popup_show: value.on_popup_show,
            on_popup_size: value.on_popup_size,
            on_paint: value.on_paint,
            on_accelerated_paint: value.on_accelerated_paint,
            get_touch_handle_size: value.get_touch_handle_size,
            on_touch_handle_state_changed: value.on_touch_handle_state_changed,
            start_dragging: value.start_dragging,
            update_drag_cursor: value.update_drag_cursor,
            on_scroll_offset_changed: value.on_scroll_offset_changed,
            on_ime_composition_range_changed: value.on_ime_composition_range_changed,
            on_text_selection_changed: value.on_text_selection_changed,
            on_virtual_keyboard_requested: value.on_virtual_keyboard_requested,
        }
    }
}
impl Default for RenderHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_auth_callback_t`] for more documentation.
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
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl From<AuthCallback> for _cef_auth_callback_t {
    fn from(value: AuthCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
            cancel: value.cancel,
        }
    }
}
impl Default for AuthCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_response_t`] for more documentation.
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
            is_read_only: value.is_read_only,
            get_error: value.get_error,
            set_error: value.set_error,
            get_status: value.get_status,
            set_status: value.set_status,
            get_status_text: value.get_status_text,
            set_status_text: value.set_status_text,
            get_mime_type: value.get_mime_type,
            set_mime_type: value.set_mime_type,
            get_charset: value.get_charset,
            set_charset: value.set_charset,
            get_header_by_name: value.get_header_by_name,
            set_header_by_name: value.set_header_by_name,
            get_header_map: value.get_header_map,
            set_header_map: value.set_header_map,
            get_url: value.get_url,
            set_url: value.set_url,
        }
    }
}
impl From<Response> for _cef_response_t {
    fn from(value: Response) -> Self {
        Self {
            base: value.base.into(),
            is_read_only: value.is_read_only,
            get_error: value.get_error,
            set_error: value.set_error,
            get_status: value.get_status,
            set_status: value.set_status,
            get_status_text: value.get_status_text,
            set_status_text: value.set_status_text,
            get_mime_type: value.get_mime_type,
            set_mime_type: value.set_mime_type,
            get_charset: value.get_charset,
            set_charset: value.set_charset,
            get_header_by_name: value.get_header_by_name,
            set_header_by_name: value.set_header_by_name,
            get_header_map: value.get_header_map,
            set_header_map: value.set_header_map,
            get_url: value.get_url,
            set_url: value.set_url,
        }
    }
}
impl Default for Response {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_resource_skip_callback_t`] for more documentation.
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
            cont: value.cont,
        }
    }
}
impl From<ResourceSkipCallback> for _cef_resource_skip_callback_t {
    fn from(value: ResourceSkipCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
        }
    }
}
impl Default for ResourceSkipCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_resource_read_callback_t`] for more documentation.
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
            cont: value.cont,
        }
    }
}
impl From<ResourceReadCallback> for _cef_resource_read_callback_t {
    fn from(value: ResourceReadCallback) -> Self {
        Self {
            base: value.base.into(),
            cont: value.cont,
        }
    }
}
impl Default for ResourceReadCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_resource_handler_t`] for more documentation.
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
            open: value.open,
            process_request: value.process_request,
            get_response_headers: value.get_response_headers,
            skip: value.skip,
            read: value.read,
            read_response: value.read_response,
            cancel: value.cancel,
        }
    }
}
impl From<ResourceHandler> for _cef_resource_handler_t {
    fn from(value: ResourceHandler) -> Self {
        Self {
            base: value.base.into(),
            open: value.open,
            process_request: value.process_request,
            get_response_headers: value.get_response_headers,
            skip: value.skip,
            read: value.read,
            read_response: value.read_response,
            cancel: value.cancel,
        }
    }
}
impl Default for ResourceHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_response_filter_t`] for more documentation.
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
            init_filter: value.init_filter,
            filter: value.filter,
        }
    }
}
impl From<ResponseFilter> for _cef_response_filter_t {
    fn from(value: ResponseFilter) -> Self {
        Self {
            base: value.base.into(),
            init_filter: value.init_filter,
            filter: value.filter,
        }
    }
}
impl Default for ResponseFilter {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_resource_request_handler_t`] for more documentation.
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
            get_cookie_access_filter: value.get_cookie_access_filter,
            on_before_resource_load: value.on_before_resource_load,
            get_resource_handler: value.get_resource_handler,
            on_resource_redirect: value.on_resource_redirect,
            on_resource_response: value.on_resource_response,
            get_resource_response_filter: value.get_resource_response_filter,
            on_resource_load_complete: value.on_resource_load_complete,
            on_protocol_execution: value.on_protocol_execution,
        }
    }
}
impl From<ResourceRequestHandler> for _cef_resource_request_handler_t {
    fn from(value: ResourceRequestHandler) -> Self {
        Self {
            base: value.base.into(),
            get_cookie_access_filter: value.get_cookie_access_filter,
            on_before_resource_load: value.on_before_resource_load,
            get_resource_handler: value.get_resource_handler,
            on_resource_redirect: value.on_resource_redirect,
            on_resource_response: value.on_resource_response,
            get_resource_response_filter: value.get_resource_response_filter,
            on_resource_load_complete: value.on_resource_load_complete,
            on_protocol_execution: value.on_protocol_execution,
        }
    }
}
impl Default for ResourceRequestHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_cookie_access_filter_t`] for more documentation.
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
            can_send_cookie: value.can_send_cookie,
            can_save_cookie: value.can_save_cookie,
        }
    }
}
impl From<CookieAccessFilter> for _cef_cookie_access_filter_t {
    fn from(value: CookieAccessFilter) -> Self {
        Self {
            base: value.base.into(),
            can_send_cookie: value.can_send_cookie,
            can_save_cookie: value.can_save_cookie,
        }
    }
}
impl Default for CookieAccessFilter {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_sslinfo_t`] for more documentation.
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
            get_cert_status: value.get_cert_status,
            get_x_509_certificate: value.get_x509_certificate,
        }
    }
}
impl From<Sslinfo> for _cef_sslinfo_t {
    fn from(value: Sslinfo) -> Self {
        Self {
            base: value.base.into(),
            get_cert_status: value.get_cert_status,
            get_x509_certificate: value.get_x_509_certificate,
        }
    }
}
impl Default for Sslinfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_unresponsive_process_callback_t`] for more documentation.
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
            wait: value.wait,
            terminate: value.terminate,
        }
    }
}
impl From<UnresponsiveProcessCallback> for _cef_unresponsive_process_callback_t {
    fn from(value: UnresponsiveProcessCallback) -> Self {
        Self {
            base: value.base.into(),
            wait: value.wait,
            terminate: value.terminate,
        }
    }
}
impl Default for UnresponsiveProcessCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_select_client_certificate_callback_t`] for more documentation.
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
            select: value.select,
        }
    }
}
impl From<SelectClientCertificateCallback> for _cef_select_client_certificate_callback_t {
    fn from(value: SelectClientCertificateCallback) -> Self {
        Self {
            base: value.base.into(),
            select: value.select,
        }
    }
}
impl Default for SelectClientCertificateCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_request_handler_t`] for more documentation.
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
            on_before_browse: value.on_before_browse,
            on_open_urlfrom_tab: value.on_open_urlfrom_tab,
            get_resource_request_handler: value.get_resource_request_handler,
            get_auth_credentials: value.get_auth_credentials,
            on_certificate_error: value.on_certificate_error,
            on_select_client_certificate: value.on_select_client_certificate,
            on_render_view_ready: value.on_render_view_ready,
            on_render_process_unresponsive: value.on_render_process_unresponsive,
            on_render_process_responsive: value.on_render_process_responsive,
            on_render_process_terminated: value.on_render_process_terminated,
            on_document_available_in_main_frame: value.on_document_available_in_main_frame,
        }
    }
}
impl From<RequestHandler> for _cef_request_handler_t {
    fn from(value: RequestHandler) -> Self {
        Self {
            base: value.base.into(),
            on_before_browse: value.on_before_browse,
            on_open_urlfrom_tab: value.on_open_urlfrom_tab,
            get_resource_request_handler: value.get_resource_request_handler,
            get_auth_credentials: value.get_auth_credentials,
            on_certificate_error: value.on_certificate_error,
            on_select_client_certificate: value.on_select_client_certificate,
            on_render_view_ready: value.on_render_view_ready,
            on_render_process_unresponsive: value.on_render_process_unresponsive,
            on_render_process_responsive: value.on_render_process_responsive,
            on_render_process_terminated: value.on_render_process_terminated,
            on_document_available_in_main_frame: value.on_document_available_in_main_frame,
        }
    }
}
impl Default for RequestHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_client_t`] for more documentation.
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
            get_audio_handler: value.get_audio_handler,
            get_command_handler: value.get_command_handler,
            get_context_menu_handler: value.get_context_menu_handler,
            get_dialog_handler: value.get_dialog_handler,
            get_display_handler: value.get_display_handler,
            get_download_handler: value.get_download_handler,
            get_drag_handler: value.get_drag_handler,
            get_find_handler: value.get_find_handler,
            get_focus_handler: value.get_focus_handler,
            get_frame_handler: value.get_frame_handler,
            get_permission_handler: value.get_permission_handler,
            get_jsdialog_handler: value.get_jsdialog_handler,
            get_keyboard_handler: value.get_keyboard_handler,
            get_life_span_handler: value.get_life_span_handler,
            get_load_handler: value.get_load_handler,
            get_print_handler: value.get_print_handler,
            get_render_handler: value.get_render_handler,
            get_request_handler: value.get_request_handler,
            on_process_message_received: value.on_process_message_received,
        }
    }
}
impl From<Client> for _cef_client_t {
    fn from(value: Client) -> Self {
        Self {
            base: value.base.into(),
            get_audio_handler: value.get_audio_handler,
            get_command_handler: value.get_command_handler,
            get_context_menu_handler: value.get_context_menu_handler,
            get_dialog_handler: value.get_dialog_handler,
            get_display_handler: value.get_display_handler,
            get_download_handler: value.get_download_handler,
            get_drag_handler: value.get_drag_handler,
            get_find_handler: value.get_find_handler,
            get_focus_handler: value.get_focus_handler,
            get_frame_handler: value.get_frame_handler,
            get_permission_handler: value.get_permission_handler,
            get_jsdialog_handler: value.get_jsdialog_handler,
            get_keyboard_handler: value.get_keyboard_handler,
            get_life_span_handler: value.get_life_span_handler,
            get_load_handler: value.get_load_handler,
            get_print_handler: value.get_print_handler,
            get_render_handler: value.get_render_handler,
            get_request_handler: value.get_request_handler,
            on_process_message_received: value.on_process_message_received,
        }
    }
}
impl Default for Client {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_command_line_t`] for more documentation.
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
            is_valid: value.is_valid,
            is_read_only: value.is_read_only,
            copy: value.copy,
            init_from_argv: value.init_from_argv,
            init_from_string: value.init_from_string,
            reset: value.reset,
            get_argv: value.get_argv,
            get_command_line_string: value.get_command_line_string,
            get_program: value.get_program,
            set_program: value.set_program,
            has_switches: value.has_switches,
            has_switch: value.has_switch,
            get_switch_value: value.get_switch_value,
            get_switches: value.get_switches,
            append_switch: value.append_switch,
            append_switch_with_value: value.append_switch_with_value,
            has_arguments: value.has_arguments,
            get_arguments: value.get_arguments,
            append_argument: value.append_argument,
            prepend_wrapper: value.prepend_wrapper,
        }
    }
}
impl From<CommandLine> for _cef_command_line_t {
    fn from(value: CommandLine) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            is_read_only: value.is_read_only,
            copy: value.copy,
            init_from_argv: value.init_from_argv,
            init_from_string: value.init_from_string,
            reset: value.reset,
            get_argv: value.get_argv,
            get_command_line_string: value.get_command_line_string,
            get_program: value.get_program,
            set_program: value.set_program,
            has_switches: value.has_switches,
            has_switch: value.has_switch,
            get_switch_value: value.get_switch_value,
            get_switches: value.get_switches,
            append_switch: value.append_switch,
            append_switch_with_value: value.append_switch_with_value,
            has_arguments: value.has_arguments,
            get_arguments: value.get_arguments,
            append_argument: value.append_argument,
            prepend_wrapper: value.prepend_wrapper,
        }
    }
}
impl Default for CommandLine {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_request_context_handler_t`] for more documentation.
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
            on_request_context_initialized: value.on_request_context_initialized,
            get_resource_request_handler: value.get_resource_request_handler,
        }
    }
}
impl From<RequestContextHandler> for _cef_request_context_handler_t {
    fn from(value: RequestContextHandler) -> Self {
        Self {
            base: value.base.into(),
            on_request_context_initialized: value.on_request_context_initialized,
            get_resource_request_handler: value.get_resource_request_handler,
        }
    }
}
impl Default for RequestContextHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_browser_process_handler_t`] for more documentation.
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
            on_register_custom_preferences: value.on_register_custom_preferences,
            on_context_initialized: value.on_context_initialized,
            on_before_child_process_launch: value.on_before_child_process_launch,
            on_already_running_app_relaunch: value.on_already_running_app_relaunch,
            on_schedule_message_pump_work: value.on_schedule_message_pump_work,
            get_default_client: value.get_default_client,
            get_default_request_context_handler: value.get_default_request_context_handler,
        }
    }
}
impl From<BrowserProcessHandler> for _cef_browser_process_handler_t {
    fn from(value: BrowserProcessHandler) -> Self {
        Self {
            base: value.base.into(),
            on_register_custom_preferences: value.on_register_custom_preferences,
            on_context_initialized: value.on_context_initialized,
            on_before_child_process_launch: value.on_before_child_process_launch,
            on_already_running_app_relaunch: value.on_already_running_app_relaunch,
            on_schedule_message_pump_work: value.on_schedule_message_pump_work,
            get_default_client: value.get_default_client,
            get_default_request_context_handler: value.get_default_request_context_handler,
        }
    }
}
impl Default for BrowserProcessHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_task_t`] for more documentation.
#[derive(Clone)]
pub struct Task {
    pub base: BaseRefCounted,
    pub execute: ::std::option::Option<unsafe extern "stdcall" fn(self_: *mut _cef_task_t)>,
}
impl From<_cef_task_t> for Task {
    fn from(value: _cef_task_t) -> Self {
        Self {
            base: value.base.into(),
            execute: value.execute,
        }
    }
}
impl From<Task> for _cef_task_t {
    fn from(value: Task) -> Self {
        Self {
            base: value.base.into(),
            execute: value.execute,
        }
    }
}
impl Default for Task {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_task_runner_t`] for more documentation.
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
            is_same: value.is_same,
            belongs_to_current_thread: value.belongs_to_current_thread,
            belongs_to_thread: value.belongs_to_thread,
            post_task: value.post_task,
            post_delayed_task: value.post_delayed_task,
        }
    }
}
impl From<TaskRunner> for _cef_task_runner_t {
    fn from(value: TaskRunner) -> Self {
        Self {
            base: value.base.into(),
            is_same: value.is_same,
            belongs_to_current_thread: value.belongs_to_current_thread,
            belongs_to_thread: value.belongs_to_thread,
            post_task: value.post_task,
            post_delayed_task: value.post_delayed_task,
        }
    }
}
impl Default for TaskRunner {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_v8_context_t`] for more documentation.
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
            get_task_runner: value.get_task_runner,
            is_valid: value.is_valid,
            get_browser: value.get_browser,
            get_frame: value.get_frame,
            get_global: value.get_global,
            enter: value.enter,
            exit: value.exit,
            is_same: value.is_same,
            eval: value.eval,
        }
    }
}
impl From<V8Context> for _cef_v8_context_t {
    fn from(value: V8Context) -> Self {
        Self {
            base: value.base.into(),
            get_task_runner: value.get_task_runner,
            is_valid: value.is_valid,
            get_browser: value.get_browser,
            get_frame: value.get_frame,
            get_global: value.get_global,
            enter: value.enter,
            exit: value.exit,
            is_same: value.is_same,
            eval: value.eval,
        }
    }
}
impl Default for V8Context {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_v8_handler_t`] for more documentation.
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
            execute: value.execute,
        }
    }
}
impl From<V8Handler> for _cef_v8_handler_t {
    fn from(value: V8Handler) -> Self {
        Self {
            base: value.base.into(),
            execute: value.execute,
        }
    }
}
impl Default for V8Handler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_v8_accessor_t`] for more documentation.
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
            get: value.get,
            set: value.set,
        }
    }
}
impl From<V8Accessor> for _cef_v8_accessor_t {
    fn from(value: V8Accessor) -> Self {
        Self {
            base: value.base.into(),
            get: value.get,
            set: value.set,
        }
    }
}
impl Default for V8Accessor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_v8_interceptor_t`] for more documentation.
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
            get_byname: value.get_byname,
            get_byindex: value.get_byindex,
            set_byname: value.set_byname,
            set_byindex: value.set_byindex,
        }
    }
}
impl From<V8Interceptor> for _cef_v8_interceptor_t {
    fn from(value: V8Interceptor) -> Self {
        Self {
            base: value.base.into(),
            get_byname: value.get_byname,
            get_byindex: value.get_byindex,
            set_byname: value.set_byname,
            set_byindex: value.set_byindex,
        }
    }
}
impl Default for V8Interceptor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_v8_exception_t`] for more documentation.
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
            get_message: value.get_message,
            get_source_line: value.get_source_line,
            get_script_resource_name: value.get_script_resource_name,
            get_line_number: value.get_line_number,
            get_start_position: value.get_start_position,
            get_end_position: value.get_end_position,
            get_start_column: value.get_start_column,
            get_end_column: value.get_end_column,
        }
    }
}
impl From<V8Exception> for _cef_v8_exception_t {
    fn from(value: V8Exception) -> Self {
        Self {
            base: value.base.into(),
            get_message: value.get_message,
            get_source_line: value.get_source_line,
            get_script_resource_name: value.get_script_resource_name,
            get_line_number: value.get_line_number,
            get_start_position: value.get_start_position,
            get_end_position: value.get_end_position,
            get_start_column: value.get_start_column,
            get_end_column: value.get_end_column,
        }
    }
}
impl Default for V8Exception {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_v8_array_buffer_release_callback_t`] for more documentation.
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
            release_buffer: value.release_buffer,
        }
    }
}
impl From<V8ArrayBufferReleaseCallback> for _cef_v8_array_buffer_release_callback_t {
    fn from(value: V8ArrayBufferReleaseCallback) -> Self {
        Self {
            base: value.base.into(),
            release_buffer: value.release_buffer,
        }
    }
}
impl Default for V8ArrayBufferReleaseCallback {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_v8_value_t`] for more documentation.
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
            is_valid: value.is_valid,
            is_undefined: value.is_undefined,
            is_null: value.is_null,
            is_bool: value.is_bool,
            is_int: value.is_int,
            is_uint: value.is_uint,
            is_double: value.is_double,
            is_date: value.is_date,
            is_string: value.is_string,
            is_object: value.is_object,
            is_array: value.is_array,
            is_array_buffer: value.is_array_buffer,
            is_function: value.is_function,
            is_promise: value.is_promise,
            is_same: value.is_same,
            get_bool_value: value.get_bool_value,
            get_int_value: value.get_int_value,
            get_uint_value: value.get_uint_value,
            get_double_value: value.get_double_value,
            get_date_value: value.get_date_value,
            get_string_value: value.get_string_value,
            is_user_created: value.is_user_created,
            has_exception: value.has_exception,
            get_exception: value.get_exception,
            clear_exception: value.clear_exception,
            will_rethrow_exceptions: value.will_rethrow_exceptions,
            set_rethrow_exceptions: value.set_rethrow_exceptions,
            has_value_bykey: value.has_value_bykey,
            has_value_byindex: value.has_value_byindex,
            delete_value_bykey: value.delete_value_bykey,
            delete_value_byindex: value.delete_value_byindex,
            get_value_bykey: value.get_value_bykey,
            get_value_byindex: value.get_value_byindex,
            set_value_bykey: value.set_value_bykey,
            set_value_byindex: value.set_value_byindex,
            set_value_byaccessor: value.set_value_byaccessor,
            get_keys: value.get_keys,
            set_user_data: value.set_user_data,
            get_user_data: value.get_user_data,
            get_externally_allocated_memory: value.get_externally_allocated_memory,
            adjust_externally_allocated_memory: value.adjust_externally_allocated_memory,
            get_array_length: value.get_array_length,
            get_array_buffer_release_callback: value.get_array_buffer_release_callback,
            neuter_array_buffer: value.neuter_array_buffer,
            get_array_buffer_byte_length: value.get_array_buffer_byte_length,
            get_array_buffer_data: value.get_array_buffer_data,
            get_function_name: value.get_function_name,
            get_function_handler: value.get_function_handler,
            execute_function: value.execute_function,
            execute_function_with_context: value.execute_function_with_context,
            resolve_promise: value.resolve_promise,
            reject_promise: value.reject_promise,
        }
    }
}
impl From<V8Value> for _cef_v8_value_t {
    fn from(value: V8Value) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            is_undefined: value.is_undefined,
            is_null: value.is_null,
            is_bool: value.is_bool,
            is_int: value.is_int,
            is_uint: value.is_uint,
            is_double: value.is_double,
            is_date: value.is_date,
            is_string: value.is_string,
            is_object: value.is_object,
            is_array: value.is_array,
            is_array_buffer: value.is_array_buffer,
            is_function: value.is_function,
            is_promise: value.is_promise,
            is_same: value.is_same,
            get_bool_value: value.get_bool_value,
            get_int_value: value.get_int_value,
            get_uint_value: value.get_uint_value,
            get_double_value: value.get_double_value,
            get_date_value: value.get_date_value,
            get_string_value: value.get_string_value,
            is_user_created: value.is_user_created,
            has_exception: value.has_exception,
            get_exception: value.get_exception,
            clear_exception: value.clear_exception,
            will_rethrow_exceptions: value.will_rethrow_exceptions,
            set_rethrow_exceptions: value.set_rethrow_exceptions,
            has_value_bykey: value.has_value_bykey,
            has_value_byindex: value.has_value_byindex,
            delete_value_bykey: value.delete_value_bykey,
            delete_value_byindex: value.delete_value_byindex,
            get_value_bykey: value.get_value_bykey,
            get_value_byindex: value.get_value_byindex,
            set_value_bykey: value.set_value_bykey,
            set_value_byindex: value.set_value_byindex,
            set_value_byaccessor: value.set_value_byaccessor,
            get_keys: value.get_keys,
            set_user_data: value.set_user_data,
            get_user_data: value.get_user_data,
            get_externally_allocated_memory: value.get_externally_allocated_memory,
            adjust_externally_allocated_memory: value.adjust_externally_allocated_memory,
            get_array_length: value.get_array_length,
            get_array_buffer_release_callback: value.get_array_buffer_release_callback,
            neuter_array_buffer: value.neuter_array_buffer,
            get_array_buffer_byte_length: value.get_array_buffer_byte_length,
            get_array_buffer_data: value.get_array_buffer_data,
            get_function_name: value.get_function_name,
            get_function_handler: value.get_function_handler,
            execute_function: value.execute_function,
            execute_function_with_context: value.execute_function_with_context,
            resolve_promise: value.resolve_promise,
            reject_promise: value.reject_promise,
        }
    }
}
impl Default for V8Value {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_v8_stack_trace_t`] for more documentation.
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
            is_valid: value.is_valid,
            get_frame_count: value.get_frame_count,
            get_frame: value.get_frame,
        }
    }
}
impl From<V8StackTrace> for _cef_v8_stack_trace_t {
    fn from(value: V8StackTrace) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            get_frame_count: value.get_frame_count,
            get_frame: value.get_frame,
        }
    }
}
impl Default for V8StackTrace {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_v8_stack_frame_t`] for more documentation.
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
            is_valid: value.is_valid,
            get_script_name: value.get_script_name,
            get_script_name_or_source_url: value.get_script_name_or_source_url,
            get_function_name: value.get_function_name,
            get_line_number: value.get_line_number,
            get_column: value.get_column,
            is_eval: value.is_eval,
            is_constructor: value.is_constructor,
        }
    }
}
impl From<V8StackFrame> for _cef_v8_stack_frame_t {
    fn from(value: V8StackFrame) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            get_script_name: value.get_script_name,
            get_script_name_or_source_url: value.get_script_name_or_source_url,
            get_function_name: value.get_function_name,
            get_line_number: value.get_line_number,
            get_column: value.get_column,
            is_eval: value.is_eval,
            is_constructor: value.is_constructor,
        }
    }
}
impl Default for V8StackFrame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_render_process_handler_t`] for more documentation.
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
            on_web_kit_initialized: value.on_web_kit_initialized,
            on_browser_created: value.on_browser_created,
            on_browser_destroyed: value.on_browser_destroyed,
            get_load_handler: value.get_load_handler,
            on_context_created: value.on_context_created,
            on_context_released: value.on_context_released,
            on_uncaught_exception: value.on_uncaught_exception,
            on_focused_node_changed: value.on_focused_node_changed,
            on_process_message_received: value.on_process_message_received,
        }
    }
}
impl From<RenderProcessHandler> for _cef_render_process_handler_t {
    fn from(value: RenderProcessHandler) -> Self {
        Self {
            base: value.base.into(),
            on_web_kit_initialized: value.on_web_kit_initialized,
            on_browser_created: value.on_browser_created,
            on_browser_destroyed: value.on_browser_destroyed,
            get_load_handler: value.get_load_handler,
            on_context_created: value.on_context_created,
            on_context_released: value.on_context_released,
            on_uncaught_exception: value.on_uncaught_exception,
            on_focused_node_changed: value.on_focused_node_changed,
            on_process_message_received: value.on_process_message_received,
        }
    }
}
impl Default for RenderProcessHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_resource_bundle_handler_t`] for more documentation.
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
            get_localized_string: value.get_localized_string,
            get_data_resource: value.get_data_resource,
            get_data_resource_for_scale: value.get_data_resource_for_scale,
        }
    }
}
impl From<ResourceBundleHandler> for _cef_resource_bundle_handler_t {
    fn from(value: ResourceBundleHandler) -> Self {
        Self {
            base: value.base.into(),
            get_localized_string: value.get_localized_string,
            get_data_resource: value.get_data_resource,
            get_data_resource_for_scale: value.get_data_resource_for_scale,
        }
    }
}
impl Default for ResourceBundleHandler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_scheme_registrar_t`] for more documentation.
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
            add_custom_scheme: value.add_custom_scheme,
        }
    }
}
impl From<SchemeRegistrar> for _cef_scheme_registrar_t {
    fn from(value: SchemeRegistrar) -> Self {
        Self {
            base: value.base.into(),
            add_custom_scheme: value.add_custom_scheme,
        }
    }
}
impl Default for SchemeRegistrar {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_scheme_handler_factory_t`] for more documentation.
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
            create: value.create,
        }
    }
}
impl From<SchemeHandlerFactory> for _cef_scheme_handler_factory_t {
    fn from(value: SchemeHandlerFactory) -> Self {
        Self {
            base: value.base.into(),
            create: value.create,
        }
    }
}
impl Default for SchemeHandlerFactory {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_app_t`] for more documentation.
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
            on_before_command_line_processing: value.on_before_command_line_processing,
            on_register_custom_schemes: value.on_register_custom_schemes,
            get_resource_bundle_handler: value.get_resource_bundle_handler,
            get_browser_process_handler: value.get_browser_process_handler,
            get_render_process_handler: value.get_render_process_handler,
        }
    }
}
impl From<App> for _cef_app_t {
    fn from(value: App) -> Self {
        Self {
            base: value.base.into(),
            on_before_command_line_processing: value.on_before_command_line_processing,
            on_register_custom_schemes: value.on_register_custom_schemes,
            get_resource_bundle_handler: value.get_resource_bundle_handler,
            get_browser_process_handler: value.get_browser_process_handler,
            get_render_process_handler: value.get_render_process_handler,
        }
    }
}
impl Default for App {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_urlrequest_t`] for more documentation.
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
            get_request: value.get_request,
            get_client: value.get_client,
            get_request_status: value.get_request_status,
            get_request_error: value.get_request_error,
            get_response: value.get_response,
            response_was_cached: value.response_was_cached,
            cancel: value.cancel,
        }
    }
}
impl From<Urlrequest> for _cef_urlrequest_t {
    fn from(value: Urlrequest) -> Self {
        Self {
            base: value.base.into(),
            get_request: value.get_request,
            get_client: value.get_client,
            get_request_status: value.get_request_status,
            get_request_error: value.get_request_error,
            get_response: value.get_response,
            response_was_cached: value.response_was_cached,
            cancel: value.cancel,
        }
    }
}
impl Default for Urlrequest {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_urlrequest_client_t`] for more documentation.
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
            on_request_complete: value.on_request_complete,
            on_upload_progress: value.on_upload_progress,
            on_download_progress: value.on_download_progress,
            on_download_data: value.on_download_data,
            get_auth_credentials: value.get_auth_credentials,
        }
    }
}
impl From<UrlrequestClient> for _cef_urlrequest_client_t {
    fn from(value: UrlrequestClient) -> Self {
        Self {
            base: value.base.into(),
            on_request_complete: value.on_request_complete,
            on_upload_progress: value.on_upload_progress,
            on_download_progress: value.on_download_progress,
            on_download_data: value.on_download_data,
            get_auth_credentials: value.get_auth_credentials,
        }
    }
}
impl Default for UrlrequestClient {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_layout_t`] for more documentation.
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
            as_box_layout: value.as_box_layout,
            as_fill_layout: value.as_fill_layout,
            is_valid: value.is_valid,
        }
    }
}
impl From<Layout> for _cef_layout_t {
    fn from(value: Layout) -> Self {
        Self {
            base: value.base.into(),
            as_box_layout: value.as_box_layout,
            as_fill_layout: value.as_fill_layout,
            is_valid: value.is_valid,
        }
    }
}
impl Default for Layout {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_box_layout_t`] for more documentation.
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
            set_flex_for_view: value.set_flex_for_view,
            clear_flex_for_view: value.clear_flex_for_view,
        }
    }
}
impl From<BoxLayout> for _cef_box_layout_t {
    fn from(value: BoxLayout) -> Self {
        Self {
            base: value.base.into(),
            set_flex_for_view: value.set_flex_for_view,
            clear_flex_for_view: value.clear_flex_for_view,
        }
    }
}
impl Default for BoxLayout {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_fill_layout_t`] for more documentation.
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
impl From<FillLayout> for _cef_fill_layout_t {
    fn from(value: FillLayout) -> Self {
        Self {
            base: value.base.into(),
        }
    }
}
impl Default for FillLayout {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_view_delegate_t`] for more documentation.
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
            get_preferred_size: value.get_preferred_size,
            get_minimum_size: value.get_minimum_size,
            get_maximum_size: value.get_maximum_size,
            get_height_for_width: value.get_height_for_width,
            on_parent_view_changed: value.on_parent_view_changed,
            on_child_view_changed: value.on_child_view_changed,
            on_window_changed: value.on_window_changed,
            on_layout_changed: value.on_layout_changed,
            on_focus: value.on_focus,
            on_blur: value.on_blur,
            on_theme_changed: value.on_theme_changed,
        }
    }
}
impl From<ViewDelegate> for _cef_view_delegate_t {
    fn from(value: ViewDelegate) -> Self {
        Self {
            base: value.base.into(),
            get_preferred_size: value.get_preferred_size,
            get_minimum_size: value.get_minimum_size,
            get_maximum_size: value.get_maximum_size,
            get_height_for_width: value.get_height_for_width,
            on_parent_view_changed: value.on_parent_view_changed,
            on_child_view_changed: value.on_child_view_changed,
            on_window_changed: value.on_window_changed,
            on_layout_changed: value.on_layout_changed,
            on_focus: value.on_focus,
            on_blur: value.on_blur,
            on_theme_changed: value.on_theme_changed,
        }
    }
}
impl Default for ViewDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_view_t`] for more documentation.
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
            as_browser_view: value.as_browser_view,
            as_button: value.as_button,
            as_panel: value.as_panel,
            as_scroll_view: value.as_scroll_view,
            as_textfield: value.as_textfield,
            get_type_string: value.get_type_string,
            to_string: value.to_string,
            is_valid: value.is_valid,
            is_attached: value.is_attached,
            is_same: value.is_same,
            get_delegate: value.get_delegate,
            get_window: value.get_window,
            get_id: value.get_id,
            set_id: value.set_id,
            get_group_id: value.get_group_id,
            set_group_id: value.set_group_id,
            get_parent_view: value.get_parent_view,
            get_view_for_id: value.get_view_for_id,
            set_bounds: value.set_bounds,
            get_bounds: value.get_bounds,
            get_bounds_in_screen: value.get_bounds_in_screen,
            set_size: value.set_size,
            get_size: value.get_size,
            set_position: value.set_position,
            get_position: value.get_position,
            set_insets: value.set_insets,
            get_insets: value.get_insets,
            get_preferred_size: value.get_preferred_size,
            size_to_preferred_size: value.size_to_preferred_size,
            get_minimum_size: value.get_minimum_size,
            get_maximum_size: value.get_maximum_size,
            get_height_for_width: value.get_height_for_width,
            invalidate_layout: value.invalidate_layout,
            set_visible: value.set_visible,
            is_visible: value.is_visible,
            is_drawn: value.is_drawn,
            set_enabled: value.set_enabled,
            is_enabled: value.is_enabled,
            set_focusable: value.set_focusable,
            is_focusable: value.is_focusable,
            is_accessibility_focusable: value.is_accessibility_focusable,
            has_focus: value.has_focus,
            request_focus: value.request_focus,
            set_background_color: value.set_background_color,
            get_background_color: value.get_background_color,
            get_theme_color: value.get_theme_color,
            convert_point_to_screen: value.convert_point_to_screen,
            convert_point_from_screen: value.convert_point_from_screen,
            convert_point_to_window: value.convert_point_to_window,
            convert_point_from_window: value.convert_point_from_window,
            convert_point_to_view: value.convert_point_to_view,
            convert_point_from_view: value.convert_point_from_view,
        }
    }
}
impl From<View> for _cef_view_t {
    fn from(value: View) -> Self {
        Self {
            base: value.base.into(),
            as_browser_view: value.as_browser_view,
            as_button: value.as_button,
            as_panel: value.as_panel,
            as_scroll_view: value.as_scroll_view,
            as_textfield: value.as_textfield,
            get_type_string: value.get_type_string,
            to_string: value.to_string,
            is_valid: value.is_valid,
            is_attached: value.is_attached,
            is_same: value.is_same,
            get_delegate: value.get_delegate,
            get_window: value.get_window,
            get_id: value.get_id,
            set_id: value.set_id,
            get_group_id: value.get_group_id,
            set_group_id: value.set_group_id,
            get_parent_view: value.get_parent_view,
            get_view_for_id: value.get_view_for_id,
            set_bounds: value.set_bounds,
            get_bounds: value.get_bounds,
            get_bounds_in_screen: value.get_bounds_in_screen,
            set_size: value.set_size,
            get_size: value.get_size,
            set_position: value.set_position,
            get_position: value.get_position,
            set_insets: value.set_insets,
            get_insets: value.get_insets,
            get_preferred_size: value.get_preferred_size,
            size_to_preferred_size: value.size_to_preferred_size,
            get_minimum_size: value.get_minimum_size,
            get_maximum_size: value.get_maximum_size,
            get_height_for_width: value.get_height_for_width,
            invalidate_layout: value.invalidate_layout,
            set_visible: value.set_visible,
            is_visible: value.is_visible,
            is_drawn: value.is_drawn,
            set_enabled: value.set_enabled,
            is_enabled: value.is_enabled,
            set_focusable: value.set_focusable,
            is_focusable: value.is_focusable,
            is_accessibility_focusable: value.is_accessibility_focusable,
            has_focus: value.has_focus,
            request_focus: value.request_focus,
            set_background_color: value.set_background_color,
            get_background_color: value.get_background_color,
            get_theme_color: value.get_theme_color,
            convert_point_to_screen: value.convert_point_to_screen,
            convert_point_from_screen: value.convert_point_from_screen,
            convert_point_to_window: value.convert_point_to_window,
            convert_point_from_window: value.convert_point_from_window,
            convert_point_to_view: value.convert_point_to_view,
            convert_point_from_view: value.convert_point_from_view,
        }
    }
}
impl Default for View {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_button_t`] for more documentation.
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
            as_label_button: value.as_label_button,
            set_state: value.set_state,
            get_state: value.get_state,
            set_ink_drop_enabled: value.set_ink_drop_enabled,
            set_tooltip_text: value.set_tooltip_text,
            set_accessible_name: value.set_accessible_name,
        }
    }
}
impl From<Button> for _cef_button_t {
    fn from(value: Button) -> Self {
        Self {
            base: value.base.into(),
            as_label_button: value.as_label_button,
            set_state: value.set_state,
            get_state: value.get_state,
            set_ink_drop_enabled: value.set_ink_drop_enabled,
            set_tooltip_text: value.set_tooltip_text,
            set_accessible_name: value.set_accessible_name,
        }
    }
}
impl Default for Button {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_button_delegate_t`] for more documentation.
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
            on_button_pressed: value.on_button_pressed,
            on_button_state_changed: value.on_button_state_changed,
        }
    }
}
impl From<ButtonDelegate> for _cef_button_delegate_t {
    fn from(value: ButtonDelegate) -> Self {
        Self {
            base: value.base.into(),
            on_button_pressed: value.on_button_pressed,
            on_button_state_changed: value.on_button_state_changed,
        }
    }
}
impl Default for ButtonDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_label_button_t`] for more documentation.
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
            as_menu_button: value.as_menu_button,
            set_text: value.set_text,
            get_text: value.get_text,
            set_image: value.set_image,
            get_image: value.get_image,
            set_text_color: value.set_text_color,
            set_enabled_text_colors: value.set_enabled_text_colors,
            set_font_list: value.set_font_list,
            set_horizontal_alignment: value.set_horizontal_alignment,
            set_minimum_size: value.set_minimum_size,
            set_maximum_size: value.set_maximum_size,
        }
    }
}
impl From<LabelButton> for _cef_label_button_t {
    fn from(value: LabelButton) -> Self {
        Self {
            base: value.base.into(),
            as_menu_button: value.as_menu_button,
            set_text: value.set_text,
            get_text: value.get_text,
            set_image: value.set_image,
            get_image: value.get_image,
            set_text_color: value.set_text_color,
            set_enabled_text_colors: value.set_enabled_text_colors,
            set_font_list: value.set_font_list,
            set_horizontal_alignment: value.set_horizontal_alignment,
            set_minimum_size: value.set_minimum_size,
            set_maximum_size: value.set_maximum_size,
        }
    }
}
impl Default for LabelButton {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_menu_button_pressed_lock_t`] for more documentation.
#[derive(Clone)]
pub struct MenuButtonPressedLock(RefGuard<_cef_menu_button_pressed_lock_t>);
pub trait ImplMenuButtonPressedLock: Clone + Sized + Rc {
    fn get_raw(&self) -> *mut _cef_menu_button_pressed_lock_t;
}
impl ImplMenuButtonPressedLock for MenuButtonPressedLock {
    fn get_raw(&self) -> *mut _cef_menu_button_pressed_lock_t {
        unsafe { RefGuard::into_raw(&self.0) }
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
    fn into_raw(self) -> *mut _cef_menu_button_pressed_lock_t {
        ImplMenuButtonPressedLock::get_raw(self)
    }
}
impl ConvertParam<*mut _cef_menu_button_pressed_lock_t> for &mut MenuButtonPressedLock {
    fn into_raw(self) -> *mut _cef_menu_button_pressed_lock_t {
        ImplMenuButtonPressedLock::get_raw(self)
    }
}
impl ConvertReturnValue<MenuButtonPressedLock> for *mut _cef_menu_button_pressed_lock_t {
    fn wrap_result(self) -> MenuButtonPressedLock {
        MenuButtonPressedLock(unsafe { RefGuard::from_raw(self) })
    }
}
impl From<MenuButtonPressedLock> for *mut _cef_menu_button_pressed_lock_t {
    fn from(value: MenuButtonPressedLock) -> Self {
        let object = ImplMenuButtonPressedLock::get_raw(&value);
        std::mem::forget(value);
        object
    }
}
impl Default for MenuButtonPressedLock {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_menu_button_delegate_t`] for more documentation.
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
            on_menu_button_pressed: value.on_menu_button_pressed,
        }
    }
}
impl From<MenuButtonDelegate> for _cef_menu_button_delegate_t {
    fn from(value: MenuButtonDelegate) -> Self {
        Self {
            base: value.base.into(),
            on_menu_button_pressed: value.on_menu_button_pressed,
        }
    }
}
impl Default for MenuButtonDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_menu_button_t`] for more documentation.
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
            show_menu: value.show_menu,
            trigger_menu: value.trigger_menu,
        }
    }
}
impl From<MenuButton> for _cef_menu_button_t {
    fn from(value: MenuButton) -> Self {
        Self {
            base: value.base.into(),
            show_menu: value.show_menu,
            trigger_menu: value.trigger_menu,
        }
    }
}
impl Default for MenuButton {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_textfield_delegate_t`] for more documentation.
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
            on_key_event: value.on_key_event,
            on_after_user_action: value.on_after_user_action,
        }
    }
}
impl From<TextfieldDelegate> for _cef_textfield_delegate_t {
    fn from(value: TextfieldDelegate) -> Self {
        Self {
            base: value.base.into(),
            on_key_event: value.on_key_event,
            on_after_user_action: value.on_after_user_action,
        }
    }
}
impl Default for TextfieldDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_textfield_t`] for more documentation.
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
            set_password_input: value.set_password_input,
            is_password_input: value.is_password_input,
            set_read_only: value.set_read_only,
            is_read_only: value.is_read_only,
            get_text: value.get_text,
            set_text: value.set_text,
            append_text: value.append_text,
            insert_or_replace_text: value.insert_or_replace_text,
            has_selection: value.has_selection,
            get_selected_text: value.get_selected_text,
            select_all: value.select_all,
            clear_selection: value.clear_selection,
            get_selected_range: value.get_selected_range,
            select_range: value.select_range,
            get_cursor_position: value.get_cursor_position,
            set_text_color: value.set_text_color,
            get_text_color: value.get_text_color,
            set_selection_text_color: value.set_selection_text_color,
            get_selection_text_color: value.get_selection_text_color,
            set_selection_background_color: value.set_selection_background_color,
            get_selection_background_color: value.get_selection_background_color,
            set_font_list: value.set_font_list,
            apply_text_color: value.apply_text_color,
            apply_text_style: value.apply_text_style,
            is_command_enabled: value.is_command_enabled,
            execute_command: value.execute_command,
            clear_edit_history: value.clear_edit_history,
            set_placeholder_text: value.set_placeholder_text,
            get_placeholder_text: value.get_placeholder_text,
            set_placeholder_text_color: value.set_placeholder_text_color,
            set_accessible_name: value.set_accessible_name,
        }
    }
}
impl From<Textfield> for _cef_textfield_t {
    fn from(value: Textfield) -> Self {
        Self {
            base: value.base.into(),
            set_password_input: value.set_password_input,
            is_password_input: value.is_password_input,
            set_read_only: value.set_read_only,
            is_read_only: value.is_read_only,
            get_text: value.get_text,
            set_text: value.set_text,
            append_text: value.append_text,
            insert_or_replace_text: value.insert_or_replace_text,
            has_selection: value.has_selection,
            get_selected_text: value.get_selected_text,
            select_all: value.select_all,
            clear_selection: value.clear_selection,
            get_selected_range: value.get_selected_range,
            select_range: value.select_range,
            get_cursor_position: value.get_cursor_position,
            set_text_color: value.set_text_color,
            get_text_color: value.get_text_color,
            set_selection_text_color: value.set_selection_text_color,
            get_selection_text_color: value.get_selection_text_color,
            set_selection_background_color: value.set_selection_background_color,
            get_selection_background_color: value.get_selection_background_color,
            set_font_list: value.set_font_list,
            apply_text_color: value.apply_text_color,
            apply_text_style: value.apply_text_style,
            is_command_enabled: value.is_command_enabled,
            execute_command: value.execute_command,
            clear_edit_history: value.clear_edit_history,
            set_placeholder_text: value.set_placeholder_text,
            get_placeholder_text: value.get_placeholder_text,
            set_placeholder_text_color: value.set_placeholder_text_color,
            set_accessible_name: value.set_accessible_name,
        }
    }
}
impl Default for Textfield {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_browser_view_delegate_t`] for more documentation.
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
    pub allow_move_for_picture_in_picture: ::std::option::Option<
        unsafe extern "stdcall" fn(
            self_: *mut _cef_browser_view_delegate_t,
            browser_view: *mut _cef_browser_view_t,
        ) -> ::std::os::raw::c_int,
    >,
}
impl From<_cef_browser_view_delegate_t> for BrowserViewDelegate {
    fn from(value: _cef_browser_view_delegate_t) -> Self {
        Self {
            base: value.base.into(),
            on_browser_created: value.on_browser_created,
            on_browser_destroyed: value.on_browser_destroyed,
            get_delegate_for_popup_browser_view: value.get_delegate_for_popup_browser_view,
            on_popup_browser_view_created: value.on_popup_browser_view_created,
            get_chrome_toolbar_type: value.get_chrome_toolbar_type,
            use_frameless_window_for_picture_in_picture: value
                .use_frameless_window_for_picture_in_picture,
            on_gesture_command: value.on_gesture_command,
            get_browser_runtime_style: value.get_browser_runtime_style,
            allow_move_for_picture_in_picture: value.allow_move_for_picture_in_picture,
        }
    }
}
impl From<BrowserViewDelegate> for _cef_browser_view_delegate_t {
    fn from(value: BrowserViewDelegate) -> Self {
        Self {
            base: value.base.into(),
            on_browser_created: value.on_browser_created,
            on_browser_destroyed: value.on_browser_destroyed,
            get_delegate_for_popup_browser_view: value.get_delegate_for_popup_browser_view,
            on_popup_browser_view_created: value.on_popup_browser_view_created,
            get_chrome_toolbar_type: value.get_chrome_toolbar_type,
            use_frameless_window_for_picture_in_picture: value
                .use_frameless_window_for_picture_in_picture,
            on_gesture_command: value.on_gesture_command,
            get_browser_runtime_style: value.get_browser_runtime_style,
            allow_move_for_picture_in_picture: value.allow_move_for_picture_in_picture,
        }
    }
}
impl Default for BrowserViewDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_browser_view_t`] for more documentation.
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
            get_browser: value.get_browser,
            get_chrome_toolbar: value.get_chrome_toolbar,
            set_prefer_accelerators: value.set_prefer_accelerators,
            get_runtime_style: value.get_runtime_style,
        }
    }
}
impl From<BrowserView> for _cef_browser_view_t {
    fn from(value: BrowserView) -> Self {
        Self {
            base: value.base.into(),
            get_browser: value.get_browser,
            get_chrome_toolbar: value.get_chrome_toolbar,
            set_prefer_accelerators: value.set_prefer_accelerators,
            get_runtime_style: value.get_runtime_style,
        }
    }
}
impl Default for BrowserView {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_scroll_view_t`] for more documentation.
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
            set_content_view: value.set_content_view,
            get_content_view: value.get_content_view,
            get_visible_content_rect: value.get_visible_content_rect,
            has_horizontal_scrollbar: value.has_horizontal_scrollbar,
            get_horizontal_scrollbar_height: value.get_horizontal_scrollbar_height,
            has_vertical_scrollbar: value.has_vertical_scrollbar,
            get_vertical_scrollbar_width: value.get_vertical_scrollbar_width,
        }
    }
}
impl From<ScrollView> for _cef_scroll_view_t {
    fn from(value: ScrollView) -> Self {
        Self {
            base: value.base.into(),
            set_content_view: value.set_content_view,
            get_content_view: value.get_content_view,
            get_visible_content_rect: value.get_visible_content_rect,
            has_horizontal_scrollbar: value.has_horizontal_scrollbar,
            get_horizontal_scrollbar_height: value.get_horizontal_scrollbar_height,
            has_vertical_scrollbar: value.has_vertical_scrollbar,
            get_vertical_scrollbar_width: value.get_vertical_scrollbar_width,
        }
    }
}
impl Default for ScrollView {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_display_t`] for more documentation.
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
            get_id: value.get_id,
            get_device_scale_factor: value.get_device_scale_factor,
            convert_point_to_pixels: value.convert_point_to_pixels,
            convert_point_from_pixels: value.convert_point_from_pixels,
            get_bounds: value.get_bounds,
            get_work_area: value.get_work_area,
            get_rotation: value.get_rotation,
        }
    }
}
impl From<Display> for _cef_display_t {
    fn from(value: Display) -> Self {
        Self {
            base: value.base.into(),
            get_id: value.get_id,
            get_device_scale_factor: value.get_device_scale_factor,
            convert_point_to_pixels: value.convert_point_to_pixels,
            convert_point_from_pixels: value.convert_point_from_pixels,
            get_bounds: value.get_bounds,
            get_work_area: value.get_work_area,
            get_rotation: value.get_rotation,
        }
    }
}
impl Default for Display {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_overlay_controller_t`] for more documentation.
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
            is_valid: value.is_valid,
            is_same: value.is_same,
            get_contents_view: value.get_contents_view,
            get_window: value.get_window,
            get_docking_mode: value.get_docking_mode,
            destroy: value.destroy,
            set_bounds: value.set_bounds,
            get_bounds: value.get_bounds,
            get_bounds_in_screen: value.get_bounds_in_screen,
            set_size: value.set_size,
            get_size: value.get_size,
            set_position: value.set_position,
            get_position: value.get_position,
            set_insets: value.set_insets,
            get_insets: value.get_insets,
            size_to_preferred_size: value.size_to_preferred_size,
            set_visible: value.set_visible,
            is_visible: value.is_visible,
            is_drawn: value.is_drawn,
        }
    }
}
impl From<OverlayController> for _cef_overlay_controller_t {
    fn from(value: OverlayController) -> Self {
        Self {
            base: value.base.into(),
            is_valid: value.is_valid,
            is_same: value.is_same,
            get_contents_view: value.get_contents_view,
            get_window: value.get_window,
            get_docking_mode: value.get_docking_mode,
            destroy: value.destroy,
            set_bounds: value.set_bounds,
            get_bounds: value.get_bounds,
            get_bounds_in_screen: value.get_bounds_in_screen,
            set_size: value.set_size,
            get_size: value.get_size,
            set_position: value.set_position,
            get_position: value.get_position,
            set_insets: value.set_insets,
            get_insets: value.get_insets,
            size_to_preferred_size: value.size_to_preferred_size,
            set_visible: value.set_visible,
            is_visible: value.is_visible,
            is_drawn: value.is_drawn,
        }
    }
}
impl Default for OverlayController {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_panel_delegate_t`] for more documentation.
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
impl From<PanelDelegate> for _cef_panel_delegate_t {
    fn from(value: PanelDelegate) -> Self {
        Self {
            base: value.base.into(),
        }
    }
}
impl Default for PanelDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_panel_t`] for more documentation.
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
            as_window: value.as_window,
            set_to_fill_layout: value.set_to_fill_layout,
            set_to_box_layout: value.set_to_box_layout,
            get_layout: value.get_layout,
            layout: value.layout,
            add_child_view: value.add_child_view,
            add_child_view_at: value.add_child_view_at,
            reorder_child_view: value.reorder_child_view,
            remove_child_view: value.remove_child_view,
            remove_all_child_views: value.remove_all_child_views,
            get_child_view_count: value.get_child_view_count,
            get_child_view_at: value.get_child_view_at,
        }
    }
}
impl From<Panel> for _cef_panel_t {
    fn from(value: Panel) -> Self {
        Self {
            base: value.base.into(),
            as_window: value.as_window,
            set_to_fill_layout: value.set_to_fill_layout,
            set_to_box_layout: value.set_to_box_layout,
            get_layout: value.get_layout,
            layout: value.layout,
            add_child_view: value.add_child_view,
            add_child_view_at: value.add_child_view_at,
            reorder_child_view: value.reorder_child_view,
            remove_child_view: value.remove_child_view,
            remove_all_child_views: value.remove_all_child_views,
            get_child_view_count: value.get_child_view_count,
            get_child_view_at: value.get_child_view_at,
        }
    }
}
impl Default for Panel {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_window_delegate_t`] for more documentation.
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
            on_window_created: value.on_window_created,
            on_window_closing: value.on_window_closing,
            on_window_destroyed: value.on_window_destroyed,
            on_window_activation_changed: value.on_window_activation_changed,
            on_window_bounds_changed: value.on_window_bounds_changed,
            on_window_fullscreen_transition: value.on_window_fullscreen_transition,
            get_parent_window: value.get_parent_window,
            is_window_modal_dialog: value.is_window_modal_dialog,
            get_initial_bounds: value.get_initial_bounds,
            get_initial_show_state: value.get_initial_show_state,
            is_frameless: value.is_frameless,
            with_standard_window_buttons: value.with_standard_window_buttons,
            get_titlebar_height: value.get_titlebar_height,
            accepts_first_mouse: value.accepts_first_mouse,
            can_resize: value.can_resize,
            can_maximize: value.can_maximize,
            can_minimize: value.can_minimize,
            can_close: value.can_close,
            on_accelerator: value.on_accelerator,
            on_key_event: value.on_key_event,
            on_theme_colors_changed: value.on_theme_colors_changed,
            get_window_runtime_style: value.get_window_runtime_style,
            get_linux_window_properties: value.get_linux_window_properties,
        }
    }
}
impl From<WindowDelegate> for _cef_window_delegate_t {
    fn from(value: WindowDelegate) -> Self {
        Self {
            base: value.base.into(),
            on_window_created: value.on_window_created,
            on_window_closing: value.on_window_closing,
            on_window_destroyed: value.on_window_destroyed,
            on_window_activation_changed: value.on_window_activation_changed,
            on_window_bounds_changed: value.on_window_bounds_changed,
            on_window_fullscreen_transition: value.on_window_fullscreen_transition,
            get_parent_window: value.get_parent_window,
            is_window_modal_dialog: value.is_window_modal_dialog,
            get_initial_bounds: value.get_initial_bounds,
            get_initial_show_state: value.get_initial_show_state,
            is_frameless: value.is_frameless,
            with_standard_window_buttons: value.with_standard_window_buttons,
            get_titlebar_height: value.get_titlebar_height,
            accepts_first_mouse: value.accepts_first_mouse,
            can_resize: value.can_resize,
            can_maximize: value.can_maximize,
            can_minimize: value.can_minimize,
            can_close: value.can_close,
            on_accelerator: value.on_accelerator,
            on_key_event: value.on_key_event,
            on_theme_colors_changed: value.on_theme_colors_changed,
            get_window_runtime_style: value.get_window_runtime_style,
            get_linux_window_properties: value.get_linux_window_properties,
        }
    }
}
impl Default for WindowDelegate {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`_cef_window_t`] for more documentation.
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
            show: value.show,
            show_as_browser_modal_dialog: value.show_as_browser_modal_dialog,
            hide: value.hide,
            center_window: value.center_window,
            close: value.close,
            is_closed: value.is_closed,
            activate: value.activate,
            deactivate: value.deactivate,
            is_active: value.is_active,
            bring_to_top: value.bring_to_top,
            set_always_on_top: value.set_always_on_top,
            is_always_on_top: value.is_always_on_top,
            maximize: value.maximize,
            minimize: value.minimize,
            restore: value.restore,
            set_fullscreen: value.set_fullscreen,
            is_maximized: value.is_maximized,
            is_minimized: value.is_minimized,
            is_fullscreen: value.is_fullscreen,
            get_focused_view: value.get_focused_view,
            set_title: value.set_title,
            get_title: value.get_title,
            set_window_icon: value.set_window_icon,
            get_window_icon: value.get_window_icon,
            set_window_app_icon: value.set_window_app_icon,
            get_window_app_icon: value.get_window_app_icon,
            add_overlay_view: value.add_overlay_view,
            show_menu: value.show_menu,
            cancel_menu: value.cancel_menu,
            get_display: value.get_display,
            get_client_area_bounds_in_screen: value.get_client_area_bounds_in_screen,
            set_draggable_regions: value.set_draggable_regions,
            get_window_handle: value.get_window_handle,
            send_key_press: value.send_key_press,
            send_mouse_move: value.send_mouse_move,
            send_mouse_events: value.send_mouse_events,
            set_accelerator: value.set_accelerator,
            remove_accelerator: value.remove_accelerator,
            remove_all_accelerators: value.remove_all_accelerators,
            set_theme_color: value.set_theme_color,
            theme_changed: value.theme_changed,
            get_runtime_style: value.get_runtime_style,
        }
    }
}
impl From<Window> for _cef_window_t {
    fn from(value: Window) -> Self {
        Self {
            base: value.base.into(),
            show: value.show,
            show_as_browser_modal_dialog: value.show_as_browser_modal_dialog,
            hide: value.hide,
            center_window: value.center_window,
            close: value.close,
            is_closed: value.is_closed,
            activate: value.activate,
            deactivate: value.deactivate,
            is_active: value.is_active,
            bring_to_top: value.bring_to_top,
            set_always_on_top: value.set_always_on_top,
            is_always_on_top: value.is_always_on_top,
            maximize: value.maximize,
            minimize: value.minimize,
            restore: value.restore,
            set_fullscreen: value.set_fullscreen,
            is_maximized: value.is_maximized,
            is_minimized: value.is_minimized,
            is_fullscreen: value.is_fullscreen,
            get_focused_view: value.get_focused_view,
            set_title: value.set_title,
            get_title: value.get_title,
            set_window_icon: value.set_window_icon,
            get_window_icon: value.get_window_icon,
            set_window_app_icon: value.set_window_app_icon,
            get_window_app_icon: value.get_window_app_icon,
            add_overlay_view: value.add_overlay_view,
            show_menu: value.show_menu,
            cancel_menu: value.cancel_menu,
            get_display: value.get_display,
            get_client_area_bounds_in_screen: value.get_client_area_bounds_in_screen,
            set_draggable_regions: value.set_draggable_regions,
            get_window_handle: value.get_window_handle,
            send_key_press: value.send_key_press,
            send_mouse_move: value.send_mouse_move,
            send_mouse_events: value.send_mouse_events,
            set_accelerator: value.set_accelerator,
            remove_accelerator: value.remove_accelerator,
            remove_all_accelerators: value.remove_all_accelerators,
            set_theme_color: value.set_theme_color,
            theme_changed: value.theme_changed,
            get_runtime_style: value.get_runtime_style,
        }
    }
}
impl Default for Window {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`cef_content_setting_types_t`] for more documentation.
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
impl From<ContentSettingTypes> for cef_content_setting_types_t {
    fn from(value: ContentSettingTypes) -> Self {
        value.0
    }
}
impl Default for ContentSettingTypes {
    fn default() -> Self {
        Self(cef_content_setting_types_t::CEF_CONTENT_SETTING_TYPE_COOKIES)
    }
}

/// See [`cef_content_setting_values_t`] for more documentation.
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
impl From<ContentSettingValues> for cef_content_setting_values_t {
    fn from(value: ContentSettingValues) -> Self {
        value.0
    }
}
impl Default for ContentSettingValues {
    fn default() -> Self {
        Self(cef_content_setting_values_t::CEF_CONTENT_SETTING_VALUE_DEFAULT)
    }
}

/// See [`cef_color_type_t`] for more documentation.
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
impl From<ColorType> for cef_color_type_t {
    fn from(value: ColorType) -> Self {
        value.0
    }
}
impl Default for ColorType {
    fn default() -> Self {
        Self(cef_color_type_t::CEF_COLOR_TYPE_RGBA_8888)
    }
}

/// See [`cef_runtime_style_t`] for more documentation.
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
impl From<RuntimeStyle> for cef_runtime_style_t {
    fn from(value: RuntimeStyle) -> Self {
        value.0
    }
}
impl Default for RuntimeStyle {
    fn default() -> Self {
        Self(cef_runtime_style_t::CEF_RUNTIME_STYLE_DEFAULT)
    }
}

/// See [`cef_log_severity_t`] for more documentation.
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
impl From<LogSeverity> for cef_log_severity_t {
    fn from(value: LogSeverity) -> Self {
        value.0
    }
}
impl Default for LogSeverity {
    fn default() -> Self {
        Self(cef_log_severity_t::LOGSEVERITY_DEFAULT)
    }
}

/// See [`cef_log_items_t`] for more documentation.
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
impl From<LogItems> for cef_log_items_t {
    fn from(value: LogItems) -> Self {
        value.0
    }
}
impl Default for LogItems {
    fn default() -> Self {
        Self(cef_log_items_t::LOG_ITEMS_DEFAULT)
    }
}

/// See [`cef_state_t`] for more documentation.
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
impl From<State> for cef_state_t {
    fn from(value: State) -> Self {
        value.0
    }
}
impl Default for State {
    fn default() -> Self {
        Self(cef_state_t::STATE_DEFAULT)
    }
}

/// See [`cef_return_value_t`] for more documentation.
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
impl From<ReturnValue> for cef_return_value_t {
    fn from(value: ReturnValue) -> Self {
        value.0
    }
}
impl Default for ReturnValue {
    fn default() -> Self {
        Self(cef_return_value_t::RV_CANCEL)
    }
}

/// See [`cef_cookie_priority_t`] for more documentation.
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
impl From<CookiePriority> for cef_cookie_priority_t {
    fn from(value: CookiePriority) -> Self {
        value.0
    }
}
impl Default for CookiePriority {
    fn default() -> Self {
        Self(cef_cookie_priority_t::CEF_COOKIE_PRIORITY_LOW)
    }
}

/// See [`cef_cookie_same_site_t`] for more documentation.
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
impl From<CookieSameSite> for cef_cookie_same_site_t {
    fn from(value: CookieSameSite) -> Self {
        value.0
    }
}
impl Default for CookieSameSite {
    fn default() -> Self {
        Self(cef_cookie_same_site_t::CEF_COOKIE_SAME_SITE_UNSPECIFIED)
    }
}

/// See [`cef_termination_status_t`] for more documentation.
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
impl From<TerminationStatus> for cef_termination_status_t {
    fn from(value: TerminationStatus) -> Self {
        value.0
    }
}
impl Default for TerminationStatus {
    fn default() -> Self {
        Self(cef_termination_status_t::TS_ABNORMAL_TERMINATION)
    }
}

/// See [`cef_path_key_t`] for more documentation.
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
impl From<PathKey> for cef_path_key_t {
    fn from(value: PathKey) -> Self {
        value.0
    }
}
impl Default for PathKey {
    fn default() -> Self {
        Self(cef_path_key_t::PK_DIR_CURRENT)
    }
}

/// See [`cef_storage_type_t`] for more documentation.
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
impl From<StorageType> for cef_storage_type_t {
    fn from(value: StorageType) -> Self {
        value.0
    }
}
impl Default for StorageType {
    fn default() -> Self {
        Self(cef_storage_type_t::ST_LOCALSTORAGE)
    }
}

/// See [`cef_errorcode_t`] for more documentation.
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
impl From<Errorcode> for cef_errorcode_t {
    fn from(value: Errorcode) -> Self {
        value.0
    }
}
impl Default for Errorcode {
    fn default() -> Self {
        Self(cef_errorcode_t::ERR_NONE)
    }
}

/// See [`cef_cert_status_t`] for more documentation.
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
impl From<CertStatus> for cef_cert_status_t {
    fn from(value: CertStatus) -> Self {
        value.0
    }
}
impl Default for CertStatus {
    fn default() -> Self {
        Self(cef_cert_status_t::CERT_STATUS_NONE)
    }
}

/// See [`cef_resultcode_t`] for more documentation.
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
impl From<Resultcode> for cef_resultcode_t {
    fn from(value: Resultcode) -> Self {
        value.0
    }
}
impl Default for Resultcode {
    fn default() -> Self {
        Self(cef_resultcode_t::CEF_RESULT_CODE_NORMAL_EXIT)
    }
}

/// See [`cef_window_open_disposition_t`] for more documentation.
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
impl From<WindowOpenDisposition> for cef_window_open_disposition_t {
    fn from(value: WindowOpenDisposition) -> Self {
        value.0
    }
}
impl Default for WindowOpenDisposition {
    fn default() -> Self {
        Self(cef_window_open_disposition_t::CEF_WOD_UNKNOWN)
    }
}

/// See [`cef_drag_operations_mask_t`] for more documentation.
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
impl From<DragOperationsMask> for cef_drag_operations_mask_t {
    fn from(value: DragOperationsMask) -> Self {
        value.0
    }
}
impl Default for DragOperationsMask {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

/// See [`cef_text_input_mode_t`] for more documentation.
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
impl From<TextInputMode> for cef_text_input_mode_t {
    fn from(value: TextInputMode) -> Self {
        value.0
    }
}
impl Default for TextInputMode {
    fn default() -> Self {
        Self(cef_text_input_mode_t::CEF_TEXT_INPUT_MODE_DEFAULT)
    }
}

/// See [`cef_v8_propertyattribute_t`] for more documentation.
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
impl From<V8Propertyattribute> for cef_v8_propertyattribute_t {
    fn from(value: V8Propertyattribute) -> Self {
        value.0
    }
}
impl Default for V8Propertyattribute {
    fn default() -> Self {
        Self(cef_v8_propertyattribute_t::V8_PROPERTY_ATTRIBUTE_NONE)
    }
}

/// See [`cef_postdataelement_type_t`] for more documentation.
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
impl From<PostdataelementType> for cef_postdataelement_type_t {
    fn from(value: PostdataelementType) -> Self {
        value.0
    }
}
impl Default for PostdataelementType {
    fn default() -> Self {
        Self(cef_postdataelement_type_t::PDE_TYPE_EMPTY)
    }
}

/// See [`cef_resource_type_t`] for more documentation.
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
impl From<ResourceType> for cef_resource_type_t {
    fn from(value: ResourceType) -> Self {
        value.0
    }
}
impl Default for ResourceType {
    fn default() -> Self {
        Self(cef_resource_type_t::RT_MAIN_FRAME)
    }
}

/// See [`cef_transition_type_t`] for more documentation.
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
impl From<TransitionType> for cef_transition_type_t {
    fn from(value: TransitionType) -> Self {
        value.0
    }
}
impl Default for TransitionType {
    fn default() -> Self {
        Self(cef_transition_type_t::TT_LINK)
    }
}

/// See [`cef_urlrequest_flags_t`] for more documentation.
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
impl From<UrlrequestFlags> for cef_urlrequest_flags_t {
    fn from(value: UrlrequestFlags) -> Self {
        value.0
    }
}
impl Default for UrlrequestFlags {
    fn default() -> Self {
        Self(cef_urlrequest_flags_t::UR_FLAG_NONE)
    }
}

/// See [`cef_urlrequest_status_t`] for more documentation.
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
impl From<UrlrequestStatus> for cef_urlrequest_status_t {
    fn from(value: UrlrequestStatus) -> Self {
        value.0
    }
}
impl Default for UrlrequestStatus {
    fn default() -> Self {
        Self(cef_urlrequest_status_t::UR_UNKNOWN)
    }
}

/// See [`cef_process_id_t`] for more documentation.
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
impl From<ProcessId> for cef_process_id_t {
    fn from(value: ProcessId) -> Self {
        value.0
    }
}
impl Default for ProcessId {
    fn default() -> Self {
        Self(cef_process_id_t::PID_BROWSER)
    }
}

/// See [`cef_thread_id_t`] for more documentation.
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
impl From<ThreadId> for cef_thread_id_t {
    fn from(value: ThreadId) -> Self {
        value.0
    }
}
impl Default for ThreadId {
    fn default() -> Self {
        Self(cef_thread_id_t::TID_UI)
    }
}

/// See [`cef_thread_priority_t`] for more documentation.
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
impl From<ThreadPriority> for cef_thread_priority_t {
    fn from(value: ThreadPriority) -> Self {
        value.0
    }
}
impl Default for ThreadPriority {
    fn default() -> Self {
        Self(cef_thread_priority_t::TP_BACKGROUND)
    }
}

/// See [`cef_message_loop_type_t`] for more documentation.
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
impl From<MessageLoopType> for cef_message_loop_type_t {
    fn from(value: MessageLoopType) -> Self {
        value.0
    }
}
impl Default for MessageLoopType {
    fn default() -> Self {
        Self(cef_message_loop_type_t::ML_TYPE_DEFAULT)
    }
}

/// See [`cef_com_init_mode_t`] for more documentation.
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
impl From<ComInitMode> for cef_com_init_mode_t {
    fn from(value: ComInitMode) -> Self {
        value.0
    }
}
impl Default for ComInitMode {
    fn default() -> Self {
        Self(cef_com_init_mode_t::COM_INIT_MODE_NONE)
    }
}

/// See [`cef_value_type_t`] for more documentation.
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
impl From<ValueType> for cef_value_type_t {
    fn from(value: ValueType) -> Self {
        value.0
    }
}
impl Default for ValueType {
    fn default() -> Self {
        Self(cef_value_type_t::VTYPE_INVALID)
    }
}

/// See [`cef_jsdialog_type_t`] for more documentation.
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
impl From<JsdialogType> for cef_jsdialog_type_t {
    fn from(value: JsdialogType) -> Self {
        value.0
    }
}
impl Default for JsdialogType {
    fn default() -> Self {
        Self(cef_jsdialog_type_t::JSDIALOGTYPE_ALERT)
    }
}

/// See [`cef_menu_id_t`] for more documentation.
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
impl From<MenuId> for cef_menu_id_t {
    fn from(value: MenuId) -> Self {
        value.0
    }
}
impl Default for MenuId {
    fn default() -> Self {
        Self(cef_menu_id_t::MENU_ID_BACK)
    }
}

/// See [`cef_mouse_button_type_t`] for more documentation.
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
impl From<MouseButtonType> for cef_mouse_button_type_t {
    fn from(value: MouseButtonType) -> Self {
        value.0
    }
}
impl Default for MouseButtonType {
    fn default() -> Self {
        Self(cef_mouse_button_type_t::MBT_LEFT)
    }
}

/// See [`cef_touch_event_type_t`] for more documentation.
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
impl From<TouchEventType> for cef_touch_event_type_t {
    fn from(value: TouchEventType) -> Self {
        value.0
    }
}
impl Default for TouchEventType {
    fn default() -> Self {
        Self(cef_touch_event_type_t::CEF_TET_RELEASED)
    }
}

/// See [`cef_pointer_type_t`] for more documentation.
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
impl From<PointerType> for cef_pointer_type_t {
    fn from(value: PointerType) -> Self {
        value.0
    }
}
impl Default for PointerType {
    fn default() -> Self {
        Self(cef_pointer_type_t::CEF_POINTER_TYPE_TOUCH)
    }
}

/// See [`cef_paint_element_type_t`] for more documentation.
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
impl From<PaintElementType> for cef_paint_element_type_t {
    fn from(value: PaintElementType) -> Self {
        value.0
    }
}
impl Default for PaintElementType {
    fn default() -> Self {
        Self(cef_paint_element_type_t::PET_VIEW)
    }
}

/// See [`cef_event_flags_t`] for more documentation.
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
impl From<EventFlags> for cef_event_flags_t {
    fn from(value: EventFlags) -> Self {
        value.0
    }
}
impl Default for EventFlags {
    fn default() -> Self {
        Self(cef_event_flags_t::EVENTFLAG_NONE)
    }
}

/// See [`cef_menu_item_type_t`] for more documentation.
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
impl From<MenuItemType> for cef_menu_item_type_t {
    fn from(value: MenuItemType) -> Self {
        value.0
    }
}
impl Default for MenuItemType {
    fn default() -> Self {
        Self(cef_menu_item_type_t::MENUITEMTYPE_NONE)
    }
}

/// See [`cef_context_menu_type_flags_t`] for more documentation.
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
impl From<ContextMenuTypeFlags> for cef_context_menu_type_flags_t {
    fn from(value: ContextMenuTypeFlags) -> Self {
        value.0
    }
}
impl Default for ContextMenuTypeFlags {
    fn default() -> Self {
        Self(cef_context_menu_type_flags_t::CM_TYPEFLAG_NONE)
    }
}

/// See [`cef_context_menu_media_type_t`] for more documentation.
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
impl From<ContextMenuMediaType> for cef_context_menu_media_type_t {
    fn from(value: ContextMenuMediaType) -> Self {
        value.0
    }
}
impl Default for ContextMenuMediaType {
    fn default() -> Self {
        Self(cef_context_menu_media_type_t::CM_MEDIATYPE_NONE)
    }
}

/// See [`cef_context_menu_media_state_flags_t`] for more documentation.
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
impl From<ContextMenuMediaStateFlags> for cef_context_menu_media_state_flags_t {
    fn from(value: ContextMenuMediaStateFlags) -> Self {
        value.0
    }
}
impl Default for ContextMenuMediaStateFlags {
    fn default() -> Self {
        Self(cef_context_menu_media_state_flags_t::CM_MEDIAFLAG_NONE)
    }
}

/// See [`cef_context_menu_edit_state_flags_t`] for more documentation.
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
impl From<ContextMenuEditStateFlags> for cef_context_menu_edit_state_flags_t {
    fn from(value: ContextMenuEditStateFlags) -> Self {
        value.0
    }
}
impl Default for ContextMenuEditStateFlags {
    fn default() -> Self {
        Self(cef_context_menu_edit_state_flags_t::CM_EDITFLAG_NONE)
    }
}

/// See [`cef_quick_menu_edit_state_flags_t`] for more documentation.
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
impl From<QuickMenuEditStateFlags> for cef_quick_menu_edit_state_flags_t {
    fn from(value: QuickMenuEditStateFlags) -> Self {
        value.0
    }
}
impl Default for QuickMenuEditStateFlags {
    fn default() -> Self {
        Self(cef_quick_menu_edit_state_flags_t::QM_EDITFLAG_NONE)
    }
}

/// See [`cef_key_event_type_t`] for more documentation.
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
impl From<KeyEventType> for cef_key_event_type_t {
    fn from(value: KeyEventType) -> Self {
        value.0
    }
}
impl Default for KeyEventType {
    fn default() -> Self {
        Self(cef_key_event_type_t::KEYEVENT_RAWKEYDOWN)
    }
}

/// See [`cef_focus_source_t`] for more documentation.
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
impl From<FocusSource> for cef_focus_source_t {
    fn from(value: FocusSource) -> Self {
        value.0
    }
}
impl Default for FocusSource {
    fn default() -> Self {
        Self(cef_focus_source_t::FOCUS_SOURCE_NAVIGATION)
    }
}

/// See [`cef_navigation_type_t`] for more documentation.
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
impl From<NavigationType> for cef_navigation_type_t {
    fn from(value: NavigationType) -> Self {
        value.0
    }
}
impl Default for NavigationType {
    fn default() -> Self {
        Self(cef_navigation_type_t::NAVIGATION_LINK_CLICKED)
    }
}

/// See [`cef_xml_encoding_type_t`] for more documentation.
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
impl From<XmlEncodingType> for cef_xml_encoding_type_t {
    fn from(value: XmlEncodingType) -> Self {
        value.0
    }
}
impl Default for XmlEncodingType {
    fn default() -> Self {
        Self(cef_xml_encoding_type_t::XML_ENCODING_NONE)
    }
}

/// See [`cef_xml_node_type_t`] for more documentation.
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
impl From<XmlNodeType> for cef_xml_node_type_t {
    fn from(value: XmlNodeType) -> Self {
        value.0
    }
}
impl Default for XmlNodeType {
    fn default() -> Self {
        Self(cef_xml_node_type_t::XML_NODE_UNSUPPORTED)
    }
}

/// See [`cef_dom_document_type_t`] for more documentation.
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
impl From<DomDocumentType> for cef_dom_document_type_t {
    fn from(value: DomDocumentType) -> Self {
        value.0
    }
}
impl Default for DomDocumentType {
    fn default() -> Self {
        Self(cef_dom_document_type_t::DOM_DOCUMENT_TYPE_UNKNOWN)
    }
}

/// See [`cef_dom_event_category_t`] for more documentation.
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
impl From<DomEventCategory> for cef_dom_event_category_t {
    fn from(value: DomEventCategory) -> Self {
        value.0
    }
}
impl Default for DomEventCategory {
    fn default() -> Self {
        Self(cef_dom_event_category_t::DOM_EVENT_CATEGORY_UNKNOWN)
    }
}

/// See [`cef_dom_event_phase_t`] for more documentation.
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
impl From<DomEventPhase> for cef_dom_event_phase_t {
    fn from(value: DomEventPhase) -> Self {
        value.0
    }
}
impl Default for DomEventPhase {
    fn default() -> Self {
        Self(cef_dom_event_phase_t::DOM_EVENT_PHASE_UNKNOWN)
    }
}

/// See [`cef_dom_node_type_t`] for more documentation.
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
impl From<DomNodeType> for cef_dom_node_type_t {
    fn from(value: DomNodeType) -> Self {
        value.0
    }
}
impl Default for DomNodeType {
    fn default() -> Self {
        Self(cef_dom_node_type_t::DOM_NODE_TYPE_UNSUPPORTED)
    }
}

/// See [`cef_dom_form_control_type_t`] for more documentation.
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
impl From<DomFormControlType> for cef_dom_form_control_type_t {
    fn from(value: DomFormControlType) -> Self {
        value.0
    }
}
impl Default for DomFormControlType {
    fn default() -> Self {
        Self(cef_dom_form_control_type_t::DOM_FORM_CONTROL_TYPE_UNSUPPORTED)
    }
}

/// See [`cef_file_dialog_mode_t`] for more documentation.
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
impl From<FileDialogMode> for cef_file_dialog_mode_t {
    fn from(value: FileDialogMode) -> Self {
        value.0
    }
}
impl Default for FileDialogMode {
    fn default() -> Self {
        Self(cef_file_dialog_mode_t::FILE_DIALOG_OPEN)
    }
}

/// See [`cef_color_model_t`] for more documentation.
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
impl From<ColorModel> for cef_color_model_t {
    fn from(value: ColorModel) -> Self {
        value.0
    }
}
impl Default for ColorModel {
    fn default() -> Self {
        Self(cef_color_model_t::COLOR_MODEL_UNKNOWN)
    }
}

/// See [`cef_duplex_mode_t`] for more documentation.
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
impl From<DuplexMode> for cef_duplex_mode_t {
    fn from(value: DuplexMode) -> Self {
        value.0
    }
}
impl Default for DuplexMode {
    fn default() -> Self {
        Self(cef_duplex_mode_t::DUPLEX_MODE_UNKNOWN)
    }
}

/// See [`cef_cursor_type_t`] for more documentation.
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
impl From<CursorType> for cef_cursor_type_t {
    fn from(value: CursorType) -> Self {
        value.0
    }
}
impl Default for CursorType {
    fn default() -> Self {
        Self(cef_cursor_type_t::CT_POINTER)
    }
}

/// See [`cef_uri_unescape_rule_t`] for more documentation.
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
impl From<UriUnescapeRule> for cef_uri_unescape_rule_t {
    fn from(value: UriUnescapeRule) -> Self {
        value.0
    }
}
impl Default for UriUnescapeRule {
    fn default() -> Self {
        Self(cef_uri_unescape_rule_t::UU_NONE)
    }
}

/// See [`cef_json_parser_options_t`] for more documentation.
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
impl From<JsonParserOptions> for cef_json_parser_options_t {
    fn from(value: JsonParserOptions) -> Self {
        value.0
    }
}
impl Default for JsonParserOptions {
    fn default() -> Self {
        Self(cef_json_parser_options_t::JSON_PARSER_RFC)
    }
}

/// See [`cef_json_writer_options_t`] for more documentation.
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
impl From<JsonWriterOptions> for cef_json_writer_options_t {
    fn from(value: JsonWriterOptions) -> Self {
        value.0
    }
}
impl Default for JsonWriterOptions {
    fn default() -> Self {
        Self(cef_json_writer_options_t::JSON_WRITER_DEFAULT)
    }
}

/// See [`cef_pdf_print_margin_type_t`] for more documentation.
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
impl From<PdfPrintMarginType> for cef_pdf_print_margin_type_t {
    fn from(value: PdfPrintMarginType) -> Self {
        value.0
    }
}
impl Default for PdfPrintMarginType {
    fn default() -> Self {
        Self(cef_pdf_print_margin_type_t::PDF_PRINT_MARGIN_DEFAULT)
    }
}

/// See [`cef_scale_factor_t`] for more documentation.
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
impl From<ScaleFactor> for cef_scale_factor_t {
    fn from(value: ScaleFactor) -> Self {
        value.0
    }
}
impl Default for ScaleFactor {
    fn default() -> Self {
        Self(cef_scale_factor_t::SCALE_FACTOR_NONE)
    }
}

/// See [`cef_referrer_policy_t`] for more documentation.
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
impl From<ReferrerPolicy> for cef_referrer_policy_t {
    fn from(value: ReferrerPolicy) -> Self {
        value.0
    }
}
impl Default for ReferrerPolicy {
    fn default() -> Self {
        Self (cef_referrer_policy_t :: REFERRER_POLICY_CLEAR_REFERRER_ON_TRANSITION_FROM_SECURE_TO_INSECURE)
    }
}

/// See [`cef_response_filter_status_t`] for more documentation.
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
impl From<ResponseFilterStatus> for cef_response_filter_status_t {
    fn from(value: ResponseFilterStatus) -> Self {
        value.0
    }
}
impl Default for ResponseFilterStatus {
    fn default() -> Self {
        Self(cef_response_filter_status_t::RESPONSE_FILTER_NEED_MORE_DATA)
    }
}

/// See [`cef_alpha_type_t`] for more documentation.
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
impl From<AlphaType> for cef_alpha_type_t {
    fn from(value: AlphaType) -> Self {
        value.0
    }
}
impl Default for AlphaType {
    fn default() -> Self {
        Self(cef_alpha_type_t::CEF_ALPHA_TYPE_OPAQUE)
    }
}

/// See [`cef_text_style_t`] for more documentation.
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
impl From<TextStyle> for cef_text_style_t {
    fn from(value: TextStyle) -> Self {
        value.0
    }
}
impl Default for TextStyle {
    fn default() -> Self {
        Self(cef_text_style_t::CEF_TEXT_STYLE_BOLD)
    }
}

/// See [`cef_axis_alignment_t`] for more documentation.
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
impl From<AxisAlignment> for cef_axis_alignment_t {
    fn from(value: AxisAlignment) -> Self {
        value.0
    }
}
impl Default for AxisAlignment {
    fn default() -> Self {
        Self(cef_axis_alignment_t::CEF_AXIS_ALIGNMENT_START)
    }
}

/// See [`cef_button_state_t`] for more documentation.
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
impl From<ButtonState> for cef_button_state_t {
    fn from(value: ButtonState) -> Self {
        value.0
    }
}
impl Default for ButtonState {
    fn default() -> Self {
        Self(cef_button_state_t::CEF_BUTTON_STATE_NORMAL)
    }
}

/// See [`cef_horizontal_alignment_t`] for more documentation.
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
impl From<HorizontalAlignment> for cef_horizontal_alignment_t {
    fn from(value: HorizontalAlignment) -> Self {
        value.0
    }
}
impl Default for HorizontalAlignment {
    fn default() -> Self {
        Self(cef_horizontal_alignment_t::CEF_HORIZONTAL_ALIGNMENT_LEFT)
    }
}

/// See [`cef_menu_anchor_position_t`] for more documentation.
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
impl From<MenuAnchorPosition> for cef_menu_anchor_position_t {
    fn from(value: MenuAnchorPosition) -> Self {
        value.0
    }
}
impl Default for MenuAnchorPosition {
    fn default() -> Self {
        Self(cef_menu_anchor_position_t::CEF_MENU_ANCHOR_TOPLEFT)
    }
}

/// See [`cef_menu_color_type_t`] for more documentation.
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
impl From<MenuColorType> for cef_menu_color_type_t {
    fn from(value: MenuColorType) -> Self {
        value.0
    }
}
impl Default for MenuColorType {
    fn default() -> Self {
        Self(cef_menu_color_type_t::CEF_MENU_COLOR_TEXT)
    }
}

/// See [`cef_ssl_version_t`] for more documentation.
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
impl From<SslVersion> for cef_ssl_version_t {
    fn from(value: SslVersion) -> Self {
        value.0
    }
}
impl Default for SslVersion {
    fn default() -> Self {
        Self(cef_ssl_version_t::SSL_CONNECTION_VERSION_UNKNOWN)
    }
}

/// See [`cef_ssl_content_status_t`] for more documentation.
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
impl From<SslContentStatus> for cef_ssl_content_status_t {
    fn from(value: SslContentStatus) -> Self {
        value.0
    }
}
impl Default for SslContentStatus {
    fn default() -> Self {
        Self(cef_ssl_content_status_t::SSL_CONTENT_NORMAL_CONTENT)
    }
}

/// See [`cef_scheme_options_t`] for more documentation.
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
impl From<SchemeOptions> for cef_scheme_options_t {
    fn from(value: SchemeOptions) -> Self {
        value.0
    }
}
impl Default for SchemeOptions {
    fn default() -> Self {
        Self(cef_scheme_options_t::CEF_SCHEME_OPTION_NONE)
    }
}

/// See [`cef_composition_underline_style_t`] for more documentation.
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
impl From<CompositionUnderlineStyle> for cef_composition_underline_style_t {
    fn from(value: CompositionUnderlineStyle) -> Self {
        value.0
    }
}
impl Default for CompositionUnderlineStyle {
    fn default() -> Self {
        Self(cef_composition_underline_style_t::CEF_CUS_SOLID)
    }
}

/// See [`cef_channel_layout_t`] for more documentation.
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
impl From<ChannelLayout> for cef_channel_layout_t {
    fn from(value: ChannelLayout) -> Self {
        value.0
    }
}
impl Default for ChannelLayout {
    fn default() -> Self {
        Self(cef_channel_layout_t::CEF_CHANNEL_LAYOUT_NONE)
    }
}

/// See [`cef_media_route_create_result_t`] for more documentation.
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
impl From<MediaRouteCreateResult> for cef_media_route_create_result_t {
    fn from(value: MediaRouteCreateResult) -> Self {
        value.0
    }
}
impl Default for MediaRouteCreateResult {
    fn default() -> Self {
        Self(cef_media_route_create_result_t::CEF_MRCR_UNKNOWN_ERROR)
    }
}

/// See [`cef_media_route_connection_state_t`] for more documentation.
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
impl From<MediaRouteConnectionState> for cef_media_route_connection_state_t {
    fn from(value: MediaRouteConnectionState) -> Self {
        value.0
    }
}
impl Default for MediaRouteConnectionState {
    fn default() -> Self {
        Self(cef_media_route_connection_state_t::CEF_MRCS_UNKNOWN)
    }
}

/// See [`cef_media_sink_icon_type_t`] for more documentation.
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
impl From<MediaSinkIconType> for cef_media_sink_icon_type_t {
    fn from(value: MediaSinkIconType) -> Self {
        value.0
    }
}
impl Default for MediaSinkIconType {
    fn default() -> Self {
        Self(cef_media_sink_icon_type_t::CEF_MSIT_CAST)
    }
}

/// See [`cef_text_field_commands_t`] for more documentation.
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
impl From<TextFieldCommands> for cef_text_field_commands_t {
    fn from(value: TextFieldCommands) -> Self {
        value.0
    }
}
impl Default for TextFieldCommands {
    fn default() -> Self {
        Self(cef_text_field_commands_t::CEF_TFC_UNKNOWN)
    }
}

/// See [`cef_chrome_toolbar_type_t`] for more documentation.
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
impl From<ChromeToolbarType> for cef_chrome_toolbar_type_t {
    fn from(value: ChromeToolbarType) -> Self {
        value.0
    }
}
impl Default for ChromeToolbarType {
    fn default() -> Self {
        Self(cef_chrome_toolbar_type_t::CEF_CTT_UNKNOWN)
    }
}

/// See [`cef_chrome_page_action_icon_type_t`] for more documentation.
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
impl From<ChromePageActionIconType> for cef_chrome_page_action_icon_type_t {
    fn from(value: ChromePageActionIconType) -> Self {
        value.0
    }
}
impl Default for ChromePageActionIconType {
    fn default() -> Self {
        Self(cef_chrome_page_action_icon_type_t::CEF_CPAIT_BOOKMARK_STAR)
    }
}

/// See [`cef_chrome_toolbar_button_type_t`] for more documentation.
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
impl From<ChromeToolbarButtonType> for cef_chrome_toolbar_button_type_t {
    fn from(value: ChromeToolbarButtonType) -> Self {
        value.0
    }
}
impl Default for ChromeToolbarButtonType {
    fn default() -> Self {
        Self(cef_chrome_toolbar_button_type_t::CEF_CTBT_CAST)
    }
}

/// See [`cef_docking_mode_t`] for more documentation.
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
impl From<DockingMode> for cef_docking_mode_t {
    fn from(value: DockingMode) -> Self {
        value.0
    }
}
impl Default for DockingMode {
    fn default() -> Self {
        Self(cef_docking_mode_t::CEF_DOCKING_MODE_TOP_LEFT)
    }
}

/// See [`cef_show_state_t`] for more documentation.
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
impl From<ShowState> for cef_show_state_t {
    fn from(value: ShowState) -> Self {
        value.0
    }
}
impl Default for ShowState {
    fn default() -> Self {
        Self(cef_show_state_t::CEF_SHOW_STATE_NORMAL)
    }
}

/// See [`cef_touch_handle_state_flags_t`] for more documentation.
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
impl From<TouchHandleStateFlags> for cef_touch_handle_state_flags_t {
    fn from(value: TouchHandleStateFlags) -> Self {
        value.0
    }
}
impl Default for TouchHandleStateFlags {
    fn default() -> Self {
        Self(cef_touch_handle_state_flags_t::CEF_THS_FLAG_NONE)
    }
}

/// See [`cef_media_access_permission_types_t`] for more documentation.
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
impl From<MediaAccessPermissionTypes> for cef_media_access_permission_types_t {
    fn from(value: MediaAccessPermissionTypes) -> Self {
        value.0
    }
}
impl Default for MediaAccessPermissionTypes {
    fn default() -> Self {
        Self(cef_media_access_permission_types_t::CEF_MEDIA_PERMISSION_NONE)
    }
}

/// See [`cef_permission_request_types_t`] for more documentation.
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
impl From<PermissionRequestTypes> for cef_permission_request_types_t {
    fn from(value: PermissionRequestTypes) -> Self {
        value.0
    }
}
impl Default for PermissionRequestTypes {
    fn default() -> Self {
        Self(cef_permission_request_types_t::CEF_PERMISSION_TYPE_NONE)
    }
}

/// See [`cef_permission_request_result_t`] for more documentation.
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
impl From<PermissionRequestResult> for cef_permission_request_result_t {
    fn from(value: PermissionRequestResult) -> Self {
        value.0
    }
}
impl Default for PermissionRequestResult {
    fn default() -> Self {
        Self(cef_permission_request_result_t::CEF_PERMISSION_RESULT_ACCEPT)
    }
}

/// See [`cef_test_cert_type_t`] for more documentation.
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
impl From<TestCertType> for cef_test_cert_type_t {
    fn from(value: TestCertType) -> Self {
        value.0
    }
}
impl Default for TestCertType {
    fn default() -> Self {
        Self(cef_test_cert_type_t::CEF_TEST_CERT_OK_IP)
    }
}

/// See [`cef_preferences_type_t`] for more documentation.
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
impl From<PreferencesType> for cef_preferences_type_t {
    fn from(value: PreferencesType) -> Self {
        value.0
    }
}
impl Default for PreferencesType {
    fn default() -> Self {
        Self(cef_preferences_type_t::CEF_PREFERENCES_TYPE_GLOBAL)
    }
}

/// See [`cef_download_interrupt_reason_t`] for more documentation.
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
impl From<DownloadInterruptReason> for cef_download_interrupt_reason_t {
    fn from(value: DownloadInterruptReason) -> Self {
        value.0
    }
}
impl Default for DownloadInterruptReason {
    fn default() -> Self {
        Self(cef_download_interrupt_reason_t::CEF_DOWNLOAD_INTERRUPT_REASON_NONE)
    }
}

/// See [`cef_gesture_command_t`] for more documentation.
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
impl From<GestureCommand> for cef_gesture_command_t {
    fn from(value: GestureCommand) -> Self {
        value.0
    }
}
impl Default for GestureCommand {
    fn default() -> Self {
        Self(cef_gesture_command_t::CEF_GESTURE_COMMAND_BACK)
    }
}

/// See [`cef_zoom_command_t`] for more documentation.
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
impl From<ZoomCommand> for cef_zoom_command_t {
    fn from(value: ZoomCommand) -> Self {
        value.0
    }
}
impl Default for ZoomCommand {
    fn default() -> Self {
        Self(cef_zoom_command_t::CEF_ZOOM_COMMAND_OUT)
    }
}

/// See [`cef_color_variant_t`] for more documentation.
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
impl From<ColorVariant> for cef_color_variant_t {
    fn from(value: ColorVariant) -> Self {
        value.0
    }
}
impl Default for ColorVariant {
    fn default() -> Self {
        Self(cef_color_variant_t::CEF_COLOR_VARIANT_SYSTEM)
    }
}

/// See [`cef_task_type_t`] for more documentation.
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
impl From<TaskType> for cef_task_type_t {
    fn from(value: TaskType) -> Self {
        value.0
    }
}
impl Default for TaskType {
    fn default() -> Self {
        Self(cef_task_type_t::CEF_TASK_TYPE_UNKNOWN)
    }
}

/// See [`cef_sandbox_info_create`] for more documentation.
pub fn sandbox_info_create() -> *mut ::std::os::raw::c_void {
    unsafe {
        let result = cef_sandbox_info_create();
        result.wrap_result()
    }
}

/// See [`cef_sandbox_info_destroy`] for more documentation.
pub fn sandbox_info_destroy(sandbox_info: *mut u8) {
    unsafe {
        let arg_sandbox_info = sandbox_info;
        let arg_sandbox_info = arg_sandbox_info.cast();
        cef_sandbox_info_destroy(arg_sandbox_info);
    }
}

/// See [`cef_api_hash`] for more documentation.
pub fn api_hash(
    version: ::std::os::raw::c_int,
    entry: ::std::os::raw::c_int,
) -> *const ::std::os::raw::c_char {
    unsafe {
        let (arg_version, arg_entry) = (version, entry);
        let result = cef_api_hash(arg_version, arg_entry);
        result.wrap_result()
    }
}

/// See [`cef_api_version`] for more documentation.
pub fn api_version() -> ::std::os::raw::c_int {
    unsafe {
        let result = cef_api_version();
        result.wrap_result()
    }
}

/// See [`cef_string_wide_set`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_wide_set(arg_src, arg_src_len, arg_output, arg_copy);
        result.wrap_result()
    }
}

/// See [`cef_string_utf8_set`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf8_set(arg_src, arg_src_len, arg_output, arg_copy);
        result.wrap_result()
    }
}

/// See [`cef_string_utf16_set`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf16_set(arg_src, arg_src_len, arg_output, arg_copy);
        result.wrap_result()
    }
}

/// See [`cef_string_wide_clear`] for more documentation.
pub fn string_wide_clear(str_: Option<&mut CefStringWide>) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_string_wide_clear(arg_str_);
    }
}

/// See [`cef_string_utf8_clear`] for more documentation.
pub fn string_utf8_clear(str_: Option<&mut CefStringUtf8>) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_string_utf8_clear(arg_str_);
    }
}

/// See [`cef_string_utf16_clear`] for more documentation.
pub fn string_utf16_clear(str_: Option<&mut CefStringUtf16>) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_string_utf16_clear(arg_str_);
    }
}

/// See [`cef_string_wide_cmp`] for more documentation.
pub fn string_wide_cmp(
    str_1: Option<&CefStringWide>,
    str_2: Option<&CefStringWide>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_str_1, arg_str_2) = (str_1, str_2);
        let arg_str_1 = arg_str_1
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let arg_str_2 = arg_str_2
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_wide_cmp(arg_str_1, arg_str_2);
        result.wrap_result()
    }
}

/// See [`cef_string_utf8_cmp`] for more documentation.
pub fn string_utf8_cmp(
    str_1: Option<&CefStringUtf8>,
    str_2: Option<&CefStringUtf8>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_str_1, arg_str_2) = (str_1, str_2);
        let arg_str_1 = arg_str_1
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let arg_str_2 = arg_str_2
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_utf8_cmp(arg_str_1, arg_str_2);
        result.wrap_result()
    }
}

/// See [`cef_string_utf16_cmp`] for more documentation.
pub fn string_utf16_cmp(
    str_1: Option<&CefStringUtf16>,
    str_2: Option<&CefStringUtf16>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_str_1, arg_str_2) = (str_1, str_2);
        let arg_str_1 = arg_str_1
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let arg_str_2 = arg_str_2
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_utf16_cmp(arg_str_1, arg_str_2);
        result.wrap_result()
    }
}

/// See [`cef_string_wide_to_utf8`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_wide_to_utf8(arg_src, arg_src_len, arg_output);
        result.wrap_result()
    }
}

/// See [`cef_string_utf8_to_wide`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf8_to_wide(arg_src, arg_src_len, arg_output);
        result.wrap_result()
    }
}

/// See [`cef_string_wide_to_utf16`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_wide_to_utf16(arg_src, arg_src_len, arg_output);
        result.wrap_result()
    }
}

/// See [`cef_string_utf16_to_wide`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf16_to_wide(arg_src, arg_src_len, arg_output);
        result.wrap_result()
    }
}

/// See [`cef_string_utf8_to_utf16`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf8_to_utf16(arg_src, arg_src_len, arg_output);
        result.wrap_result()
    }
}

/// See [`cef_string_utf16_to_utf8`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf16_to_utf8(arg_src, arg_src_len, arg_output);
        result.wrap_result()
    }
}

/// See [`cef_string_ascii_to_wide`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_ascii_to_wide(arg_src, arg_src_len, arg_output);
        result.wrap_result()
    }
}

/// See [`cef_string_ascii_to_utf16`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_ascii_to_utf16(arg_src, arg_src_len, arg_output);
        result.wrap_result()
    }
}

/// See [`cef_string_userfree_wide_alloc`] for more documentation.
pub fn string_userfree_wide_alloc() -> CefStringUserfreeWide {
    unsafe {
        let result = cef_string_userfree_wide_alloc();
        result.wrap_result()
    }
}

/// See [`cef_string_userfree_utf8_alloc`] for more documentation.
pub fn string_userfree_utf8_alloc() -> CefStringUserfreeUtf8 {
    unsafe {
        let result = cef_string_userfree_utf8_alloc();
        result.wrap_result()
    }
}

/// See [`cef_string_userfree_utf16_alloc`] for more documentation.
pub fn string_userfree_utf16_alloc() -> CefStringUserfreeUtf16 {
    unsafe {
        let result = cef_string_userfree_utf16_alloc();
        result.wrap_result()
    }
}

/// See [`cef_string_userfree_wide_free`] for more documentation.
pub fn string_userfree_wide_free(str_: CefStringUserfreeWide) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_.into_raw();
        cef_string_userfree_wide_free(arg_str_);
    }
}

/// See [`cef_string_userfree_utf8_free`] for more documentation.
pub fn string_userfree_utf8_free(str_: CefStringUserfreeUtf8) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_.into_raw();
        cef_string_userfree_utf8_free(arg_str_);
    }
}

/// See [`cef_string_userfree_utf16_free`] for more documentation.
pub fn string_userfree_utf16_free(str_: CefStringUserfreeUtf16) {
    unsafe {
        let arg_str_ = str_;
        let arg_str_ = arg_str_.into_raw();
        cef_string_userfree_utf16_free(arg_str_);
    }
}

/// See [`cef_string_utf16_to_lower`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf16_to_lower(arg_src, arg_src_len, arg_output);
        result.wrap_result()
    }
}

/// See [`cef_string_utf16_to_upper`] for more documentation.
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
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let arg_output = arg_output
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_utf16_to_upper(arg_src, arg_src_len, arg_output);
        result.wrap_result()
    }
}

/// See [`cef_string_list_alloc`] for more documentation.
pub fn string_list_alloc() -> Option<CefStringList> {
    unsafe {
        let result = cef_string_list_alloc();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_string_list_size`] for more documentation.
pub fn string_list_size(list: Option<&mut CefStringList>) -> usize {
    unsafe {
        let arg_list = list;
        let arg_list = arg_list
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_list_size(arg_list);
        result.wrap_result()
    }
}

/// See [`cef_string_list_value`] for more documentation.
pub fn string_list_value(
    list: Option<&mut CefStringList>,
    index: usize,
    value: Option<&mut CefString>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_list, arg_index, arg_value) = (list, index, value);
        let arg_list = arg_list
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_value = arg_value
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_list_value(arg_list, arg_index, arg_value);
        result.wrap_result()
    }
}

/// See [`cef_string_list_append`] for more documentation.
pub fn string_list_append(list: Option<&mut CefStringList>, value: Option<&CefString>) {
    unsafe {
        let (arg_list, arg_value) = (list, value);
        let arg_list = arg_list
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_value = arg_value
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        cef_string_list_append(arg_list, arg_value);
    }
}

/// See [`cef_string_list_clear`] for more documentation.
pub fn string_list_clear(list: Option<&mut CefStringList>) {
    unsafe {
        let arg_list = list;
        let arg_list = arg_list
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_string_list_clear(arg_list);
    }
}

/// See [`cef_string_list_free`] for more documentation.
pub fn string_list_free(list: Option<&mut CefStringList>) {
    unsafe {
        let arg_list = list;
        let arg_list = arg_list
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_string_list_free(arg_list);
    }
}

/// See [`cef_string_list_copy`] for more documentation.
pub fn string_list_copy(list: Option<&mut CefStringList>) -> Option<CefStringList> {
    unsafe {
        let arg_list = list;
        let arg_list = arg_list
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_list_copy(arg_list);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_string_map_alloc`] for more documentation.
pub fn string_map_alloc() -> Option<CefStringMap> {
    unsafe {
        let result = cef_string_map_alloc();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_string_map_size`] for more documentation.
pub fn string_map_size(map: Option<&mut CefStringMap>) -> usize {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_map_size(arg_map);
        result.wrap_result()
    }
}

/// See [`cef_string_map_find`] for more documentation.
pub fn string_map_find(
    map: Option<&mut CefStringMap>,
    key: Option<&CefString>,
    value: Option<&mut CefString>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_key, arg_value) = (map, key, value);
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let arg_value = arg_value
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_map_find(arg_map, arg_key, arg_value);
        result.wrap_result()
    }
}

/// See [`cef_string_map_key`] for more documentation.
pub fn string_map_key(
    map: Option<&mut CefStringMap>,
    index: usize,
    key: Option<&mut CefString>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_index, arg_key) = (map, index, key);
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_map_key(arg_map, arg_index, arg_key);
        result.wrap_result()
    }
}

/// See [`cef_string_map_value`] for more documentation.
pub fn string_map_value(
    map: Option<&mut CefStringMap>,
    index: usize,
    value: Option<&mut CefString>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_index, arg_value) = (map, index, value);
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_value = arg_value
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_map_value(arg_map, arg_index, arg_value);
        result.wrap_result()
    }
}

/// See [`cef_string_map_append`] for more documentation.
pub fn string_map_append(
    map: Option<&mut CefStringMap>,
    key: Option<&CefString>,
    value: Option<&CefString>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_key, arg_value) = (map, key, value);
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let arg_value = arg_value
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_map_append(arg_map, arg_key, arg_value);
        result.wrap_result()
    }
}

/// See [`cef_string_map_clear`] for more documentation.
pub fn string_map_clear(map: Option<&mut CefStringMap>) {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_string_map_clear(arg_map);
    }
}

/// See [`cef_string_map_free`] for more documentation.
pub fn string_map_free(map: Option<&mut CefStringMap>) {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_string_map_free(arg_map);
    }
}

/// See [`cef_string_multimap_alloc`] for more documentation.
pub fn string_multimap_alloc() -> Option<CefStringMultimap> {
    unsafe {
        let result = cef_string_multimap_alloc();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_string_multimap_size`] for more documentation.
pub fn string_multimap_size(map: Option<&mut CefStringMultimap>) -> usize {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_multimap_size(arg_map);
        result.wrap_result()
    }
}

/// See [`cef_string_multimap_find_count`] for more documentation.
pub fn string_multimap_find_count(
    map: Option<&mut CefStringMultimap>,
    key: Option<&CefString>,
) -> usize {
    unsafe {
        let (arg_map, arg_key) = (map, key);
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_multimap_find_count(arg_map, arg_key);
        result.wrap_result()
    }
}

/// See [`cef_string_multimap_enumerate`] for more documentation.
pub fn string_multimap_enumerate(
    map: Option<&mut CefStringMultimap>,
    key: Option<&CefString>,
    value_index: usize,
    value: Option<&mut CefString>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_key, arg_value_index, arg_value) = (map, key, value_index, value);
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let arg_value = arg_value
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_multimap_enumerate(arg_map, arg_key, arg_value_index, arg_value);
        result.wrap_result()
    }
}

/// See [`cef_string_multimap_key`] for more documentation.
pub fn string_multimap_key(
    map: Option<&mut CefStringMultimap>,
    index: usize,
    key: Option<&mut CefString>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_index, arg_key) = (map, index, key);
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_multimap_key(arg_map, arg_index, arg_key);
        result.wrap_result()
    }
}

/// See [`cef_string_multimap_value`] for more documentation.
pub fn string_multimap_value(
    map: Option<&mut CefStringMultimap>,
    index: usize,
    value: Option<&mut CefString>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_index, arg_value) = (map, index, value);
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_value = arg_value
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let result = cef_string_multimap_value(arg_map, arg_index, arg_value);
        result.wrap_result()
    }
}

/// See [`cef_string_multimap_append`] for more documentation.
pub fn string_multimap_append(
    map: Option<&mut CefStringMultimap>,
    key: Option<&CefString>,
    value: Option<&CefString>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_map, arg_key, arg_value) = (map, key, value);
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        let arg_key = arg_key
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let arg_value = arg_value
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_string_multimap_append(arg_map, arg_key, arg_value);
        result.wrap_result()
    }
}

/// See [`cef_string_multimap_clear`] for more documentation.
pub fn string_multimap_clear(map: Option<&mut CefStringMultimap>) {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_string_multimap_clear(arg_map);
    }
}

/// See [`cef_string_multimap_free`] for more documentation.
pub fn string_multimap_free(map: Option<&mut CefStringMultimap>) {
    unsafe {
        let arg_map = map;
        let arg_map = arg_map
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_string_multimap_free(arg_map);
    }
}

/// See [`cef_time_to_timet`] for more documentation.
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
        result.wrap_result()
    }
}

/// See [`cef_time_from_timet`] for more documentation.
pub fn time_from_timet(time: time_t, cef_time: Option<&mut Time>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_time, arg_cef_time) = (time, cef_time);
        let mut arg_cef_time = arg_cef_time.cloned().map(|arg| arg.into());
        let arg_cef_time = arg_cef_time
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_from_timet(arg_time, arg_cef_time);
        result.wrap_result()
    }
}

/// See [`cef_time_to_doublet`] for more documentation.
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
        result.wrap_result()
    }
}

/// See [`cef_time_from_doublet`] for more documentation.
pub fn time_from_doublet(time: f64, cef_time: Option<&mut Time>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_time, arg_cef_time) = (time, cef_time);
        let mut arg_cef_time = arg_cef_time.cloned().map(|arg| arg.into());
        let arg_cef_time = arg_cef_time
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_from_doublet(arg_time, arg_cef_time);
        result.wrap_result()
    }
}

/// See [`cef_time_now`] for more documentation.
pub fn time_now(cef_time: Option<&mut Time>) -> ::std::os::raw::c_int {
    unsafe {
        let arg_cef_time = cef_time;
        let mut arg_cef_time = arg_cef_time.cloned().map(|arg| arg.into());
        let arg_cef_time = arg_cef_time
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_now(arg_cef_time);
        result.wrap_result()
    }
}

/// See [`cef_basetime_now`] for more documentation.
pub fn basetime_now() -> Basetime {
    unsafe {
        let result = cef_basetime_now();
        result.wrap_result()
    }
}

/// See [`cef_time_delta`] for more documentation.
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
        result.wrap_result()
    }
}

/// See [`cef_time_to_basetime`] for more documentation.
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
        result.wrap_result()
    }
}

/// See [`cef_time_from_basetime`] for more documentation.
pub fn time_from_basetime(from: _cef_basetime_t, to: Option<&mut Time>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_from, arg_to) = (from, to);
        let mut arg_to = arg_to.cloned().map(|arg| arg.into());
        let arg_to = arg_to
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_time_from_basetime(arg_from, arg_to);
        result.wrap_result()
    }
}

/// See [`cef_value_create`] for more documentation.
pub fn value_create() -> Option<Value> {
    unsafe {
        let result = cef_value_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_binary_value_create`] for more documentation.
pub fn binary_value_create(data: Option<&[u8]>) -> Option<BinaryValue> {
    unsafe {
        let arg_data = data;
        let arg_data_size = arg_data.as_ref().map(|arg| arg.len()).unwrap_or_default();
        let arg_data = arg_data
            .and_then(|arg| {
                if arg.is_empty() {
                    None
                } else {
                    Some(arg.as_ptr().cast())
                }
            })
            .unwrap_or(std::ptr::null());
        let result = cef_binary_value_create(arg_data, arg_data_size);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_dictionary_value_create`] for more documentation.
pub fn dictionary_value_create() -> Option<DictionaryValue> {
    unsafe {
        let result = cef_dictionary_value_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_list_value_create`] for more documentation.
pub fn list_value_create() -> Option<ListValue> {
    unsafe {
        let result = cef_list_value_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_image_create`] for more documentation.
pub fn image_create() -> Option<Image> {
    unsafe {
        let result = cef_image_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_stream_reader_create_for_file`] for more documentation.
pub fn stream_reader_create_for_file(file_name: Option<&CefString>) -> Option<StreamReader> {
    unsafe {
        let arg_file_name = file_name;
        let arg_file_name = arg_file_name
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_stream_reader_create_for_file(arg_file_name);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_stream_reader_create_for_data`] for more documentation.
pub fn stream_reader_create_for_data(data: *mut u8, size: usize) -> Option<StreamReader> {
    unsafe {
        let (arg_data, arg_size) = (data, size);
        let arg_data = arg_data.cast();
        let result = cef_stream_reader_create_for_data(arg_data, arg_size);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_stream_reader_create_for_handler`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_stream_writer_create_for_file`] for more documentation.
pub fn stream_writer_create_for_file(file_name: Option<&CefString>) -> Option<StreamWriter> {
    unsafe {
        let arg_file_name = file_name;
        let arg_file_name = arg_file_name
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_stream_writer_create_for_file(arg_file_name);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_stream_writer_create_for_handler`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_drag_data_create`] for more documentation.
pub fn drag_data_create() -> Option<DragData> {
    unsafe {
        let result = cef_drag_data_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_process_message_create`] for more documentation.
pub fn process_message_create(name: Option<&CefString>) -> Option<ProcessMessage> {
    unsafe {
        let arg_name = name;
        let arg_name = arg_name
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_process_message_create(arg_name);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_request_create`] for more documentation.
pub fn request_create() -> Option<Request> {
    unsafe {
        let result = cef_request_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_post_data_create`] for more documentation.
pub fn post_data_create() -> Option<PostData> {
    unsafe {
        let result = cef_post_data_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_post_data_element_create`] for more documentation.
pub fn post_data_element_create() -> Option<PostDataElement> {
    unsafe {
        let result = cef_post_data_element_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_cookie_manager_get_global_manager`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_media_router_get_global`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_preference_manager_get_chrome_variations_as_switches`] for more documentation.
pub fn preference_manager_get_chrome_variations_as_switches(switches: Option<&mut CefStringList>) {
    unsafe {
        let arg_switches = switches;
        let arg_switches = arg_switches
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_preference_manager_get_chrome_variations_as_switches(arg_switches);
    }
}

/// See [`cef_preference_manager_get_chrome_variations_as_strings`] for more documentation.
pub fn preference_manager_get_chrome_variations_as_strings(strings: Option<&mut CefStringList>) {
    unsafe {
        let arg_strings = strings;
        let arg_strings = arg_strings
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null_mut());
        cef_preference_manager_get_chrome_variations_as_strings(arg_strings);
    }
}

/// See [`cef_preference_manager_get_global`] for more documentation.
pub fn preference_manager_get_global() -> Option<PreferenceManager> {
    unsafe {
        let result = cef_preference_manager_get_global();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_request_context_get_global_context`] for more documentation.
pub fn request_context_get_global_context() -> Option<RequestContext> {
    unsafe {
        let result = cef_request_context_get_global_context();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_request_context_create_context`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_request_context_cef_create_context_shared`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_browser_host_create_browser`] for more documentation.
pub fn browser_host_create_browser(
    window_info: Option<&WindowInfo>,
    client: Option<&mut Client>,
    url: Option<&CefString>,
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
        let arg_url = arg_url
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
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
        result.wrap_result()
    }
}

/// See [`cef_browser_host_create_browser_sync`] for more documentation.
pub fn browser_host_create_browser_sync(
    window_info: Option<&WindowInfo>,
    client: Option<&mut Client>,
    url: Option<&CefString>,
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
        let arg_url = arg_url
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_browser_host_get_browser_by_identifier`] for more documentation.
pub fn browser_host_get_browser_by_identifier(
    browser_id: ::std::os::raw::c_int,
) -> Option<Browser> {
    unsafe {
        let arg_browser_id = browser_id;
        let result = cef_browser_host_get_browser_by_identifier(arg_browser_id);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_menu_model_create`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_print_settings_create`] for more documentation.
pub fn print_settings_create() -> Option<PrintSettings> {
    unsafe {
        let result = cef_print_settings_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_response_create`] for more documentation.
pub fn response_create() -> Option<Response> {
    unsafe {
        let result = cef_response_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_is_cert_status_error`] for more documentation.
pub fn is_cert_status_error(status: CertStatus) -> ::std::os::raw::c_int {
    unsafe {
        let arg_status = status;
        let arg_status = arg_status.into_raw();
        let result = cef_is_cert_status_error(arg_status);
        result.wrap_result()
    }
}

/// See [`cef_command_line_create`] for more documentation.
pub fn command_line_create() -> Option<CommandLine> {
    unsafe {
        let result = cef_command_line_create();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_command_line_get_global`] for more documentation.
pub fn command_line_get_global() -> Option<CommandLine> {
    unsafe {
        let result = cef_command_line_get_global();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_task_runner_get_for_current_thread`] for more documentation.
pub fn task_runner_get_for_current_thread() -> Option<TaskRunner> {
    unsafe {
        let result = cef_task_runner_get_for_current_thread();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_task_runner_get_for_thread`] for more documentation.
pub fn task_runner_get_for_thread(thread_id: ThreadId) -> Option<TaskRunner> {
    unsafe {
        let arg_thread_id = thread_id;
        let arg_thread_id = arg_thread_id.into_raw();
        let result = cef_task_runner_get_for_thread(arg_thread_id);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_currently_on`] for more documentation.
pub fn currently_on(thread_id: ThreadId) -> ::std::os::raw::c_int {
    unsafe {
        let arg_thread_id = thread_id;
        let arg_thread_id = arg_thread_id.into_raw();
        let result = cef_currently_on(arg_thread_id);
        result.wrap_result()
    }
}

/// See [`cef_post_task`] for more documentation.
pub fn post_task(thread_id: ThreadId, task: Option<&mut Task>) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_thread_id, arg_task) = (thread_id, task);
        let arg_thread_id = arg_thread_id.into_raw();
        let mut arg_task = arg_task.cloned().map(|arg| arg.into());
        let arg_task = arg_task
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_post_task(arg_thread_id, arg_task);
        result.wrap_result()
    }
}

/// See [`cef_post_delayed_task`] for more documentation.
pub fn post_delayed_task(
    thread_id: ThreadId,
    task: Option<&mut Task>,
    delay_ms: i64,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_thread_id, arg_task, arg_delay_ms) = (thread_id, task, delay_ms);
        let arg_thread_id = arg_thread_id.into_raw();
        let mut arg_task = arg_task.cloned().map(|arg| arg.into());
        let arg_task = arg_task
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_post_delayed_task(arg_thread_id, arg_task, arg_delay_ms);
        result.wrap_result()
    }
}

/// See [`cef_v8_context_get_current_context`] for more documentation.
pub fn v8_context_get_current_context() -> Option<V8Context> {
    unsafe {
        let result = cef_v8_context_get_current_context();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_context_get_entered_context`] for more documentation.
pub fn v8_context_get_entered_context() -> Option<V8Context> {
    unsafe {
        let result = cef_v8_context_get_entered_context();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_context_in_context`] for more documentation.
pub fn v8_context_in_context() -> ::std::os::raw::c_int {
    unsafe {
        let result = cef_v8_context_in_context();
        result.wrap_result()
    }
}

/// See [`cef_v8_value_create_undefined`] for more documentation.
pub fn v8_value_create_undefined() -> Option<V8Value> {
    unsafe {
        let result = cef_v8_value_create_undefined();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_null`] for more documentation.
pub fn v8_value_create_null() -> Option<V8Value> {
    unsafe {
        let result = cef_v8_value_create_null();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_bool`] for more documentation.
pub fn v8_value_create_bool(value: ::std::os::raw::c_int) -> Option<V8Value> {
    unsafe {
        let arg_value = value;
        let result = cef_v8_value_create_bool(arg_value);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_int`] for more documentation.
pub fn v8_value_create_int(value: i32) -> Option<V8Value> {
    unsafe {
        let arg_value = value;
        let result = cef_v8_value_create_int(arg_value);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_uint`] for more documentation.
pub fn v8_value_create_uint(value: u32) -> Option<V8Value> {
    unsafe {
        let arg_value = value;
        let result = cef_v8_value_create_uint(arg_value);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_double`] for more documentation.
pub fn v8_value_create_double(value: f64) -> Option<V8Value> {
    unsafe {
        let arg_value = value;
        let result = cef_v8_value_create_double(arg_value);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_date`] for more documentation.
pub fn v8_value_create_date(date: _cef_basetime_t) -> Option<V8Value> {
    unsafe {
        let arg_date = date;
        let result = cef_v8_value_create_date(arg_date);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_string`] for more documentation.
pub fn v8_value_create_string(value: Option<&CefString>) -> Option<V8Value> {
    unsafe {
        let arg_value = value;
        let arg_value = arg_value
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_v8_value_create_string(arg_value);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_object`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_array`] for more documentation.
pub fn v8_value_create_array(length: ::std::os::raw::c_int) -> Option<V8Value> {
    unsafe {
        let arg_length = length;
        let result = cef_v8_value_create_array(arg_length);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_array_buffer`] for more documentation.
pub fn v8_value_create_array_buffer(
    buffer: *mut u8,
    length: usize,
    release_callback: Option<&mut V8ArrayBufferReleaseCallback>,
) -> Option<V8Value> {
    unsafe {
        let (arg_buffer, arg_length, arg_release_callback) = (buffer, length, release_callback);
        let arg_buffer = arg_buffer.cast();
        let mut arg_release_callback = arg_release_callback.cloned().map(|arg| arg.into());
        let arg_release_callback = arg_release_callback
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_v8_value_create_array_buffer(arg_buffer, arg_length, arg_release_callback);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_array_buffer_with_copy`] for more documentation.
pub fn v8_value_create_array_buffer_with_copy(buffer: *mut u8, length: usize) -> Option<V8Value> {
    unsafe {
        let (arg_buffer, arg_length) = (buffer, length);
        let arg_buffer = arg_buffer.cast();
        let result = cef_v8_value_create_array_buffer_with_copy(arg_buffer, arg_length);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_function`] for more documentation.
pub fn v8_value_create_function(
    name: Option<&CefString>,
    handler: Option<&mut V8Handler>,
) -> Option<V8Value> {
    unsafe {
        let (arg_name, arg_handler) = (name, handler);
        let arg_name = arg_name
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let mut arg_handler = arg_handler.cloned().map(|arg| arg.into());
        let arg_handler = arg_handler
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_v8_value_create_function(arg_name, arg_handler);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_value_create_promise`] for more documentation.
pub fn v8_value_create_promise() -> Option<V8Value> {
    unsafe {
        let result = cef_v8_value_create_promise();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_v8_stack_trace_get_current`] for more documentation.
pub fn v8_stack_trace_get_current(frame_limit: ::std::os::raw::c_int) -> Option<V8StackTrace> {
    unsafe {
        let arg_frame_limit = frame_limit;
        let result = cef_v8_stack_trace_get_current(arg_frame_limit);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_register_extension`] for more documentation.
pub fn register_extension(
    extension_name: Option<&CefString>,
    javascript_code: Option<&CefString>,
    handler: Option<&mut V8Handler>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_extension_name, arg_javascript_code, arg_handler) =
            (extension_name, javascript_code, handler);
        let arg_extension_name = arg_extension_name
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let arg_javascript_code = arg_javascript_code
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let mut arg_handler = arg_handler.cloned().map(|arg| arg.into());
        let arg_handler = arg_handler
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result = cef_register_extension(arg_extension_name, arg_javascript_code, arg_handler);
        result.wrap_result()
    }
}

/// See [`cef_register_scheme_handler_factory`] for more documentation.
pub fn register_scheme_handler_factory(
    scheme_name: Option<&CefString>,
    domain_name: Option<&CefString>,
    factory: Option<&mut SchemeHandlerFactory>,
) -> ::std::os::raw::c_int {
    unsafe {
        let (arg_scheme_name, arg_domain_name, arg_factory) = (scheme_name, domain_name, factory);
        let arg_scheme_name = arg_scheme_name
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let arg_domain_name = arg_domain_name
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let mut arg_factory = arg_factory.cloned().map(|arg| arg.into());
        let arg_factory = arg_factory
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let result =
            cef_register_scheme_handler_factory(arg_scheme_name, arg_domain_name, arg_factory);
        result.wrap_result()
    }
}

/// See [`cef_clear_scheme_handler_factories`] for more documentation.
pub fn clear_scheme_handler_factories() -> ::std::os::raw::c_int {
    unsafe {
        let result = cef_clear_scheme_handler_factories();
        result.wrap_result()
    }
}

/// See [`cef_execute_process`] for more documentation.
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
        let arg_windows_sandbox_info = arg_windows_sandbox_info.cast();
        let result = cef_execute_process(arg_args, arg_application, arg_windows_sandbox_info);
        result.wrap_result()
    }
}

/// See [`cef_initialize`] for more documentation.
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
        let arg_windows_sandbox_info = arg_windows_sandbox_info.cast();
        let result = cef_initialize(
            arg_args,
            arg_settings,
            arg_application,
            arg_windows_sandbox_info,
        );
        result.wrap_result()
    }
}

/// See [`cef_get_exit_code`] for more documentation.
pub fn get_exit_code() -> ::std::os::raw::c_int {
    unsafe {
        let result = cef_get_exit_code();
        result.wrap_result()
    }
}

/// See [`cef_shutdown`] for more documentation.
pub fn shutdown() {
    unsafe {
        cef_shutdown();
    }
}

/// See [`cef_do_message_loop_work`] for more documentation.
pub fn do_message_loop_work() {
    unsafe {
        cef_do_message_loop_work();
    }
}

/// See [`cef_run_message_loop`] for more documentation.
pub fn run_message_loop() {
    unsafe {
        cef_run_message_loop();
    }
}

/// See [`cef_quit_message_loop`] for more documentation.
pub fn quit_message_loop() {
    unsafe {
        cef_quit_message_loop();
    }
}

/// See [`cef_urlrequest_create`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_label_button_create`] for more documentation.
pub fn label_button_create(
    delegate: Option<&mut ButtonDelegate>,
    text: Option<&CefString>,
) -> Option<LabelButton> {
    unsafe {
        let (arg_delegate, arg_text) = (delegate, text);
        let mut arg_delegate = arg_delegate.cloned().map(|arg| arg.into());
        let arg_delegate = arg_delegate
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let arg_text = arg_text
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_label_button_create(arg_delegate, arg_text);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_menu_button_create`] for more documentation.
pub fn menu_button_create(
    delegate: Option<&mut MenuButtonDelegate>,
    text: Option<&CefString>,
) -> Option<MenuButton> {
    unsafe {
        let (arg_delegate, arg_text) = (delegate, text);
        let mut arg_delegate = arg_delegate.cloned().map(|arg| arg.into());
        let arg_delegate = arg_delegate
            .as_mut()
            .map(std::ptr::from_mut)
            .unwrap_or(std::ptr::null_mut());
        let arg_text = arg_text
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
        let result = cef_menu_button_create(arg_delegate, arg_text);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_textfield_create`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_browser_view_create`] for more documentation.
pub fn browser_view_create(
    client: Option<&mut Client>,
    url: Option<&CefString>,
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
        let arg_url = arg_url
            .map(|arg| arg.into_raw())
            .unwrap_or(std::ptr::null());
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_browser_view_get_for_browser`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_scroll_view_create`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_display_get_primary`] for more documentation.
pub fn display_get_primary() -> Option<Display> {
    unsafe {
        let result = cef_display_get_primary();
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_display_get_nearest_point`] for more documentation.
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
        let result = cef_display_get_nearest_point(arg_point, arg_input_pixel_coords);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_display_get_matching_bounds`] for more documentation.
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
        let result = cef_display_get_matching_bounds(arg_bounds, arg_input_pixel_coords);
        if result.is_null() {
            None
        } else {
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_display_get_count`] for more documentation.
pub fn display_get_count() -> usize {
    unsafe {
        let result = cef_display_get_count();
        result.wrap_result()
    }
}

/// See [`cef_display_get_alls`] for more documentation.
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
        cef_display_get_alls(arg_displays_count, arg_displays);
        if let Some(out_displays) = out_displays {
            *out_displays = vec_displays
                .into_iter()
                .take(out_displays_count)
                .map(|elem| {
                    if elem.is_null() {
                        None
                    } else {
                        Some(elem.wrap_result())
                    }
                })
                .collect();
        }
    }
}

/// See [`cef_display_convert_screen_point_to_pixels`] for more documentation.
pub fn display_convert_screen_point_to_pixels(point: Option<&Point>) -> Point {
    unsafe {
        let arg_point = point;
        let arg_point = arg_point.cloned().map(|arg| arg.into());
        let arg_point = arg_point
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let result = cef_display_convert_screen_point_to_pixels(arg_point);
        result.wrap_result()
    }
}

/// See [`cef_display_convert_screen_point_from_pixels`] for more documentation.
pub fn display_convert_screen_point_from_pixels(point: Option<&Point>) -> Point {
    unsafe {
        let arg_point = point;
        let arg_point = arg_point.cloned().map(|arg| arg.into());
        let arg_point = arg_point
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let result = cef_display_convert_screen_point_from_pixels(arg_point);
        result.wrap_result()
    }
}

/// See [`cef_display_convert_screen_rect_to_pixels`] for more documentation.
pub fn display_convert_screen_rect_to_pixels(rect: Option<&Rect>) -> Rect {
    unsafe {
        let arg_rect = rect;
        let arg_rect = arg_rect.cloned().map(|arg| arg.into());
        let arg_rect = arg_rect
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let result = cef_display_convert_screen_rect_to_pixels(arg_rect);
        result.wrap_result()
    }
}

/// See [`cef_display_convert_screen_rect_from_pixels`] for more documentation.
pub fn display_convert_screen_rect_from_pixels(rect: Option<&Rect>) -> Rect {
    unsafe {
        let arg_rect = rect;
        let arg_rect = arg_rect.cloned().map(|arg| arg.into());
        let arg_rect = arg_rect
            .as_ref()
            .map(std::ptr::from_ref)
            .unwrap_or(std::ptr::null());
        let result = cef_display_convert_screen_rect_from_pixels(arg_rect);
        result.wrap_result()
    }
}

/// See [`cef_panel_create`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}

/// See [`cef_window_create_top_level`] for more documentation.
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
            Some(result.wrap_result())
        }
    }
}
