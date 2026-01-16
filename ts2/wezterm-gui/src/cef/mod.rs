//! CEF (Chromium Embedded Framework) integration for browser panes.
//!
//! This module provides browser functionality for TermSurf 2.0, enabling web content
//! to be rendered alongside terminal panes.

use anyhow::Result;
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Use ::cef to refer to the external cef crate, since this module is also named 'cef'
use ::cef as cef_crate;

// Import traits required by CEF macros
use cef_crate::{rc::Rc, App, ImplApp, ImplBrowser, WrapApp};
use cef_crate::{
    Client, ImplClient, ImplLifeSpanHandler, ImplRenderHandler, LifeSpanHandler, RenderHandler,
    WrapClient, WrapLifeSpanHandler, WrapRenderHandler,
};

/// Global flag indicating whether CEF is initialized and available.
static CEF_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Returns true if CEF has been successfully initialized.
pub fn is_cef_available() -> bool {
    CEF_INITIALIZED.load(Ordering::Relaxed)
}

/// Minimal CEF App implementation.
#[derive(Clone)]
struct MinimalApp;

cef_crate::wrap_app! {
    struct MinimalAppBuilder {
        app: MinimalApp,
    }

    impl App {
        // Minimal implementation - no custom behavior for now
    }
}

impl MinimalAppBuilder {
    fn build() -> cef_crate::App {
        Self::new(MinimalApp)
    }
}

/// CEF context that manages the CEF runtime.
/// This must be kept alive for the lifetime of the application while using CEF.
pub struct CefContext {
    #[cfg(target_os = "macos")]
    _library_loader: Option<cef_crate::library_loader::LibraryLoader>,
}

impl CefContext {
    /// Initialize CEF and return a context handle.
    /// Returns None if CEF framework is not available (e.g., running unbundled).
    pub fn init() -> Result<Option<Self>> {
        // Check if already initialized
        if CEF_INITIALIZED.load(Ordering::Relaxed) {
            return Ok(None);
        }

        #[cfg(target_os = "macos")]
        {
            let exe_path = std::env::current_exe()?;

            // Check if CEF framework exists
            let framework_path = exe_path.parent().and_then(|p| {
                p.join("../Frameworks/Chromium Embedded Framework.framework/Chromium Embedded Framework")
                    .canonicalize()
                    .ok()
            });

            if framework_path.is_none() {
                log::info!("CEF framework not found - browser features disabled");
                return Ok(None);
            }

            // Load the CEF library
            let loader = cef_crate::library_loader::LibraryLoader::new(&exe_path, false);
            if !loader.load() {
                log::warn!("Failed to load CEF library");
                return Ok(None);
            }

            // Initialize CEF settings
            let cache_path = Self::cache_path();
            let settings = cef_crate::Settings {
                windowless_rendering_enabled: 1,
                external_message_pump: 1,
                no_sandbox: 1,
                cache_path: cache_path.to_string_lossy().as_ref().into(),
                log_severity: cef_crate::LogSeverity::WARNING,
                ..Default::default()
            };

            // Create minimal app handler
            let mut app = MinimalAppBuilder::build();

            // Initialize CEF
            let args = cef_crate::args::Args::new();
            let result = cef_crate::initialize(
                Some(args.as_main_args()),
                Some(&settings),
                Some(&mut app),
                std::ptr::null_mut(),
            );

            if result != 1 {
                log::error!("CEF initialization failed");
                return Ok(None);
            }

            // Register message pump hook with the window crate's event loop
            ::window::set_message_pump_hook(cef_message_pump);

            CEF_INITIALIZED.store(true, Ordering::Relaxed);
            log::info!("CEF initialized successfully");

            Ok(Some(CefContext {
                _library_loader: Some(loader),
            }))
        }

        #[cfg(not(target_os = "macos"))]
        {
            // TODO: Implement CEF initialization for other platforms
            log::info!("CEF not yet implemented for this platform");
            Ok(None)
        }
    }

    /// Get the cache path for CEF data.
    fn cache_path() -> std::path::PathBuf {
        config::CACHE_DIR.join("cef")
    }
}

impl Drop for CefContext {
    fn drop(&mut self) {
        if CEF_INITIALIZED.load(Ordering::Relaxed) {
            log::info!("Shutting down CEF");
            cef_crate::shutdown();
            CEF_INITIALIZED.store(false, Ordering::Relaxed);
        }
    }
}

/// Message pump function called on each iteration of the main event loop.
fn cef_message_pump() {
    if CEF_INITIALIZED.load(Ordering::Relaxed) {
        cef_crate::do_message_loop_work();
    }
}

/// Shared texture holder type for CEF browser textures.
/// This holds the wgpu bind group for the CEF-rendered texture.
pub type TextureHolder = std::rc::Rc<RefCell<Option<wgpu::BindGroup>>>;

/// Browser state for a single pane.
/// Holds the CEF browser instance and rendering state.
pub struct BrowserState {
    /// The CEF browser instance
    pub browser: cef_crate::Browser,
    /// The texture holder for the rendered browser content
    pub texture_holder: TextureHolder,
    /// Current logical size of the browser
    pub size: std::rc::Rc<RefCell<(f32, f32)>>,
    /// Device scale factor for HiDPI
    pub device_scale_factor: f32,
}

impl BrowserState {
    /// Get the browser host for sending events
    pub fn host(&self) -> Option<cef_crate::BrowserHost> {
        self.browser.host()
    }

    /// Check if the texture is ready for rendering
    pub fn has_texture(&self) -> bool {
        self.texture_holder.borrow().is_some()
    }

    /// Send a key event to the browser.
    /// Returns true if the event was sent successfully.
    pub fn send_key_event(
        &self,
        key: &::window::KeyCode,
        modifiers: ::window::Modifiers,
        is_down: bool,
    ) -> bool {
        let Some(host) = self.host() else {
            return false;
        };

        use cef_crate::ImplBrowserHost;

        let cef_modifiers = modifiers_to_cef_flags(modifiers);
        let windows_key_code = keycode_to_windows_vk(key);
        let native_key_code = keycode_to_native(key);

        let event_type = if is_down {
            cef_crate::KeyEventType::KEYDOWN
        } else {
            cef_crate::KeyEventType::KEYUP
        };

        let key_event = cef_crate::KeyEvent {
            size: std::mem::size_of::<cef_crate::KeyEvent>(),
            type_: event_type,
            modifiers: cef_modifiers,
            windows_key_code,
            native_key_code,
            is_system_key: 0,
            character: 0,
            unmodified_character: 0,
            focus_on_editable_field: 0,
        };

        host.send_key_event(Some(&key_event));

        // For key down events with printable characters, also send CHAR event
        if is_down {
            if let ::window::KeyCode::Char(c) = key {
                let char_event = cef_crate::KeyEvent {
                    size: std::mem::size_of::<cef_crate::KeyEvent>(),
                    type_: cef_crate::KeyEventType::CHAR,
                    modifiers: cef_modifiers,
                    windows_key_code: *c as i32,
                    native_key_code: 0,
                    is_system_key: 0,
                    character: *c as u16,
                    unmodified_character: *c as u16,
                    focus_on_editable_field: 0,
                };
                host.send_key_event(Some(&char_event));
            }
        }

        true
    }

    /// Send a composed string to the browser (for IME input).
    pub fn send_composed_string(&self, s: &str, modifiers: ::window::Modifiers) -> bool {
        let Some(host) = self.host() else {
            return false;
        };

        use cef_crate::ImplBrowserHost;

        let cef_modifiers = modifiers_to_cef_flags(modifiers);

        // Send each character as a CHAR event
        for c in s.chars() {
            let char_event = cef_crate::KeyEvent {
                size: std::mem::size_of::<cef_crate::KeyEvent>(),
                type_: cef_crate::KeyEventType::CHAR,
                modifiers: cef_modifiers,
                windows_key_code: c as i32,
                native_key_code: 0,
                is_system_key: 0,
                character: c as u16,
                unmodified_character: c as u16,
                focus_on_editable_field: 0,
            };
            host.send_key_event(Some(&char_event));
        }

        true
    }
}

/// Internal render handler state
#[derive(Clone)]
struct BrowserRenderHandler {
    device_scale_factor: f32,
    size: std::rc::Rc<RefCell<(f32, f32)>>,
    texture_holder: TextureHolder,
    device: wgpu::Device,
    queue: wgpu::Queue,
    /// Callback to notify the window that a new frame is ready
    invalidate_callback: Arc<dyn Fn() + Send + Sync>,
}

cef_crate::wrap_render_handler! {
    struct RenderHandlerBuilder {
        handler: BrowserRenderHandler,
    }

    impl RenderHandler {
        fn view_rect(&self, _browser: Option<&mut cef_crate::Browser>, rect: Option<&mut cef_crate::Rect>) {
            if let Some(rect) = rect {
                let size = self.handler.size.borrow();
                // Size must be non-zero
                if size.0 > 0.0 && size.1 > 0.0 {
                    rect.width = size.0 as _;
                    rect.height = size.1 as _;
                }
            }
        }

        fn screen_info(
            &self,
            _browser: Option<&mut cef_crate::Browser>,
            screen_info: Option<&mut cef_crate::ScreenInfo>,
        ) -> ::std::os::raw::c_int {
            if let Some(screen_info) = screen_info {
                screen_info.device_scale_factor = self.handler.device_scale_factor;
                return 1; // true
            }
            0 // false
        }

        fn screen_point(
            &self,
            _browser: Option<&mut cef_crate::Browser>,
            _view_x: ::std::os::raw::c_int,
            _view_y: ::std::os::raw::c_int,
            _screen_x: Option<&mut ::std::os::raw::c_int>,
            _screen_y: Option<&mut ::std::os::raw::c_int>,
        ) -> ::std::os::raw::c_int {
            0 // false
        }

        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
        fn on_accelerated_paint(
            &self,
            _browser: Option<&mut cef_crate::Browser>,
            type_: cef_crate::PaintElementType,
            _dirty_rects: Option<&[cef_crate::Rect]>,
            info: Option<&cef_crate::AcceleratedPaintInfo>,
        ) {
            let Some(info) = info else { return };

            // Only handle PET_VIEW (main content), not popups
            if type_ != cef_crate::PaintElementType::default() {
                return;
            }

            use cef_crate::osr_texture_import::shared_texture_handle::SharedTextureHandle;

            let shared_handle = SharedTextureHandle::new(info);
            if let SharedTextureHandle::Unsupported = shared_handle {
                log::error!("Platform does not support accelerated painting");
                return;
            }

            let src_texture = match shared_handle.import_texture(&self.handler.device) {
                Ok(texture) => texture,
                Err(e) => {
                    log::error!("Failed to import CEF shared texture: {:?}", e);
                    return;
                }
            };

            let sampler = self.handler.device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::MipmapFilterMode::Linear,
                ..Default::default()
            });

            let texture_bind_group_layout =
                self.handler
                    .device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some("CEF Texture Bind Group Layout"),
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Texture {
                                    multisampled: false,
                                    view_dimension: wgpu::TextureViewDimension::D2,
                                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 1,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                                count: None,
                            },
                        ],
                    });

            let bind_group = self.handler.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("CEF Texture Bind Group"),
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &src_texture.create_view(&wgpu::TextureViewDescriptor {
                                label: Some("CEF Texture View"),
                                ..Default::default()
                            }),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });

            // Store the new texture
            *self.handler.texture_holder.borrow_mut() = Some(bind_group);

            // Signal that a new frame is ready
            (self.handler.invalidate_callback)();
        }
    }
}

impl RenderHandlerBuilder {
    fn build(handler: BrowserRenderHandler) -> RenderHandler {
        Self::new(handler)
    }
}

/// Minimal life span handler that just tracks when browser is closed
#[derive(Clone)]
struct BrowserLifeSpanHandler;

cef_crate::wrap_life_span_handler! {
    struct LifeSpanHandlerBuilder {
        handler: BrowserLifeSpanHandler,
    }

    impl LifeSpanHandler {
        // No custom implementation for now - use defaults
    }
}

impl LifeSpanHandlerBuilder {
    fn build(handler: BrowserLifeSpanHandler) -> LifeSpanHandler {
        Self::new(handler)
    }
}

/// Client that combines render handler and life span handler
#[derive(Clone)]
struct BrowserClient {
    render_handler: RenderHandler,
    life_span_handler: LifeSpanHandler,
}

cef_crate::wrap_client! {
    struct ClientBuilder {
        client: BrowserClient,
    }

    impl Client {
        fn render_handler(&self) -> Option<RenderHandler> {
            Some(self.client.render_handler.clone())
        }

        fn life_span_handler(&self) -> Option<LifeSpanHandler> {
            Some(self.client.life_span_handler.clone())
        }
    }
}

impl ClientBuilder {
    fn build(client: BrowserClient) -> Client {
        Self::new(client)
    }
}

// CEF event flag constants for keyboard/mouse modifiers
pub const EVENTFLAG_SHIFT_DOWN: u32 = 1 << 1;
pub const EVENTFLAG_CONTROL_DOWN: u32 = 1 << 2;
pub const EVENTFLAG_ALT_DOWN: u32 = 1 << 3;
pub const EVENTFLAG_COMMAND_DOWN: u32 = 1 << 7;

/// Convert WezTerm KeyCode to Windows virtual key code (used by CEF).
pub fn keycode_to_windows_vk(key: &::window::KeyCode) -> i32 {
    use ::window::KeyCode as WK;
    match key {
        WK::Char(c) => {
            match *c {
                // Control characters
                '\r' | '\n' => 0x0D, // VK_RETURN (Enter)
                '\t' => 0x09,        // VK_TAB
                '\u{08}' => 0x08,    // VK_BACK (Backspace)
                '\u{7f}' => 0x2E,    // VK_DELETE
                '\u{1b}' => 0x1B,    // VK_ESCAPE
                ' ' => 0x20,         // VK_SPACE
                // Punctuation
                ',' => 0xBC,  // VK_OEM_COMMA
                '.' => 0xBE,  // VK_OEM_PERIOD
                ';' => 0xBA,  // VK_OEM_1
                '/' => 0xBF,  // VK_OEM_2
                '`' => 0xC0,  // VK_OEM_3
                '[' => 0xDB,  // VK_OEM_4
                '\\' => 0xDC, // VK_OEM_5
                ']' => 0xDD,  // VK_OEM_6
                '\'' => 0xDE, // VK_OEM_7
                '-' => 0xBD,  // VK_OEM_MINUS
                '=' => 0xBB,  // VK_OEM_PLUS
                // Alphanumeric
                c => {
                    let c = c.to_ascii_uppercase();
                    if c.is_ascii_alphanumeric() {
                        c as i32
                    } else {
                        0
                    }
                }
            }
        }
        WK::UpArrow => 0x26,
        WK::DownArrow => 0x28,
        WK::LeftArrow => 0x25,
        WK::RightArrow => 0x27,
        WK::Home => 0x24,
        WK::End => 0x23,
        WK::PageUp => 0x21,
        WK::PageDown => 0x22,
        WK::Insert => 0x2D,
        WK::Function(1) => 0x70,
        WK::Function(2) => 0x71,
        WK::Function(3) => 0x72,
        WK::Function(4) => 0x73,
        WK::Function(5) => 0x74,
        WK::Function(6) => 0x75,
        WK::Function(7) => 0x76,
        WK::Function(8) => 0x77,
        WK::Function(9) => 0x78,
        WK::Function(10) => 0x79,
        WK::Function(11) => 0x7A,
        WK::Function(12) => 0x7B,
        WK::Function(_) => 0,
        WK::Numpad(n) => 0x60 + (*n as i32), // VK_NUMPAD0 = 0x60
        WK::Shift | WK::LeftShift | WK::RightShift => 0x10,
        WK::Control | WK::LeftControl | WK::RightControl => 0x11,
        WK::Alt | WK::LeftAlt | WK::RightAlt => 0x12,
        WK::CapsLock => 0x14,
        WK::NumLock => 0x90,
        WK::ScrollLock => 0x91,
        WK::Clear => 0x0C,
        WK::Pause => 0x13,
        WK::Print | WK::PrintScreen => 0x2C,
        WK::Cancel => 0x03,
        WK::Multiply => 0x6A,
        WK::Add => 0x6B,
        WK::Separator => 0x6C,
        WK::Subtract => 0x6D,
        WK::Decimal => 0x6E,
        WK::Divide => 0x6F,
        _ => 0,
    }
}

/// Convert WezTerm KeyCode to native macOS key code.
#[cfg(target_os = "macos")]
pub fn keycode_to_native(key: &::window::KeyCode) -> i32 {
    use ::window::KeyCode as WK;
    match key {
        WK::Char('a') | WK::Char('A') => 0x00,
        WK::Char('s') | WK::Char('S') => 0x01,
        WK::Char('d') | WK::Char('D') => 0x02,
        WK::Char('f') | WK::Char('F') => 0x03,
        WK::Char('h') | WK::Char('H') => 0x04,
        WK::Char('g') | WK::Char('G') => 0x05,
        WK::Char('z') | WK::Char('Z') => 0x06,
        WK::Char('x') | WK::Char('X') => 0x07,
        WK::Char('c') | WK::Char('C') => 0x08,
        WK::Char('v') | WK::Char('V') => 0x09,
        WK::Char('b') | WK::Char('B') => 0x0B,
        WK::Char('q') | WK::Char('Q') => 0x0C,
        WK::Char('w') | WK::Char('W') => 0x0D,
        WK::Char('e') | WK::Char('E') => 0x0E,
        WK::Char('r') | WK::Char('R') => 0x0F,
        WK::Char('y') | WK::Char('Y') => 0x10,
        WK::Char('t') | WK::Char('T') => 0x11,
        WK::Char('o') | WK::Char('O') => 0x1F,
        WK::Char('u') | WK::Char('U') => 0x20,
        WK::Char('i') | WK::Char('I') => 0x22,
        WK::Char('p') | WK::Char('P') => 0x23,
        WK::Char('l') | WK::Char('L') => 0x25,
        WK::Char('j') | WK::Char('J') => 0x26,
        WK::Char('k') | WK::Char('K') => 0x28,
        WK::Char('n') | WK::Char('N') => 0x2D,
        WK::Char('m') | WK::Char('M') => 0x2E,
        WK::Char('\r') | WK::Char('\n') => 0x24, // Enter
        WK::Char('\t') => 0x30, // Tab
        WK::Char(' ') => 0x31, // Space
        WK::Char('\u{08}') => 0x33, // Backspace
        WK::Char('\u{1b}') => 0x35, // Escape
        WK::Home => 0x73,
        WK::PageUp => 0x74,
        WK::Char('\u{7f}') => 0x75, // Delete
        WK::End => 0x77,
        WK::PageDown => 0x79,
        WK::LeftArrow => 0x7B,
        WK::RightArrow => 0x7C,
        WK::DownArrow => 0x7D,
        WK::UpArrow => 0x7E,
        _ => 0,
    }
}

#[cfg(not(target_os = "macos"))]
pub fn keycode_to_native(_key: &::window::KeyCode) -> i32 {
    0
}

/// Convert WezTerm Modifiers to CEF modifier flags.
pub fn modifiers_to_cef_flags(mods: ::window::Modifiers) -> u32 {
    let mut flags = 0u32;
    if mods.contains(::window::Modifiers::SHIFT) {
        flags |= EVENTFLAG_SHIFT_DOWN;
    }
    if mods.contains(::window::Modifiers::CTRL) {
        flags |= EVENTFLAG_CONTROL_DOWN;
    }
    if mods.contains(::window::Modifiers::ALT) {
        flags |= EVENTFLAG_ALT_DOWN;
    }
    if mods.contains(::window::Modifiers::SUPER) {
        flags |= EVENTFLAG_COMMAND_DOWN;
    }
    flags
}

/// Create a new CEF browser for the given URL.
/// Returns the BrowserState if successful.
pub fn create_browser(
    url: &str,
    width: f32,
    height: f32,
    device_scale_factor: f32,
    device: wgpu::Device,
    queue: wgpu::Queue,
    invalidate_callback: Arc<dyn Fn() + Send + Sync>,
) -> Result<Option<BrowserState>> {
    if !is_cef_available() {
        return Ok(None);
    }

    let size = std::rc::Rc::new(RefCell::new((width, height)));
    let texture_holder: TextureHolder = std::rc::Rc::new(RefCell::new(None));

    let window_info = cef_crate::WindowInfo {
        windowless_rendering_enabled: 1,
        shared_texture_enabled: 1,
        external_begin_frame_enabled: 0,
        ..Default::default()
    };

    let browser_settings = cef_crate::BrowserSettings {
        windowless_frame_rate: 60,
        ..Default::default()
    };

    let render_handler = RenderHandlerBuilder::build(BrowserRenderHandler {
        device_scale_factor,
        size: size.clone(),
        texture_holder: texture_holder.clone(),
        device,
        queue,
        invalidate_callback,
    });

    let life_span_handler = LifeSpanHandlerBuilder::build(BrowserLifeSpanHandler);

    let mut client = ClientBuilder::build(BrowserClient {
        render_handler,
        life_span_handler,
    });

    let browser = cef_crate::browser_host_create_browser_sync(
        Some(&window_info),
        Some(&mut client),
        Some(&url.into()),
        Some(&browser_settings),
        None,
        None,
    );

    match browser {
        Some(browser) => {
            log::info!("Created CEF browser for: {}", url);
            Ok(Some(BrowserState {
                browser,
                texture_holder,
                size,
                device_scale_factor,
            }))
        }
        None => {
            log::error!("Failed to create CEF browser for: {}", url);
            Ok(None)
        }
    }
}
