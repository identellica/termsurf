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
