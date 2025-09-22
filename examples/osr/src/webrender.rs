#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
use cef::sys::cef_color_type_t;
use cef::{
    self, BrowserProcessHandler, ImplBrowserProcessHandler, WrapBrowserProcessHandler,
    rc::{Rc, RcImpl},
    sys::{self},
    *,
};
use cef::{ImplRequestContextHandler, RequestContextHandler, WrapRequestContextHandler};
use std::cell::RefCell;
use std::ptr::null_mut;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
use wgpu::{Extent3d, TextureDescriptor, TextureDimension, TextureUsages};

#[derive(Clone)]
pub struct OsrApp {}

impl OsrApp {
    pub fn new() -> Self {
        Self {}
    }
}

pub(crate) struct AppBuilder {
    object: *mut RcImpl<cef::sys::_cef_app_t, Self>,
    app: OsrApp,
}

impl AppBuilder {
    pub(crate) fn build(app: OsrApp) -> cef::App {
        cef::App::new(Self {
            object: std::ptr::null_mut(),
            app,
        })
    }
}

impl WrapApp for AppBuilder {
    fn wrap_rc(&mut self, object: *mut RcImpl<cef::sys::_cef_app_t, Self>) {
        self.object = object;
    }
}

impl Clone for AppBuilder {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc = &mut *self.object;
            rc.interface.add_ref();
            self.object
        };
        Self {
            object,
            app: self.app.clone(),
        }
    }
}

impl Rc for AppBuilder {
    fn as_base(&self) -> &cef::sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl ImplApp for AppBuilder {
    fn get_raw(&self) -> *mut cef::sys::_cef_app_t {
        self.object as *mut cef::sys::_cef_app_t
    }

    fn on_before_command_line_processing(
        &self,
        _process_type: Option<&cef::CefStringUtf16>,
        command_line: Option<&mut cef::CommandLine>,
    ) {
        let Some(command_line) = command_line else {
            return;
        };

        command_line.append_switch(Some(&"no-startup-window".into()));
        command_line.append_switch(Some(&"noerrdialogs".into()));
        command_line.append_switch(Some(&"hide-crash-restore-bubble".into()));
        command_line.append_switch(Some(&"use-mock-keychain".into()));
        command_line.append_switch(Some(&"enable-logging=stderr".into()));
        command_line
            .append_switch_with_value(Some(&"remote-debugging-port".into()), Some(&"9229".into()));
    }

    fn browser_process_handler(&self) -> Option<cef::BrowserProcessHandler> {
        Some(BrowserProcessHandlerBuilder::build(
            OsrBrowserProcessHandler::new(),
        ))
    }
}

#[derive(Clone)]
pub struct OsrBrowserProcessHandler {
    is_cef_ready: RefCell<bool>,
}

impl OsrBrowserProcessHandler {
    pub fn new() -> Self {
        Self {
            is_cef_ready: RefCell::new(false),
        }
    }
}

pub(crate) struct BrowserProcessHandlerBuilder {
    object: *mut RcImpl<sys::cef_browser_process_handler_t, Self>,
    handler: OsrBrowserProcessHandler,
}

impl BrowserProcessHandlerBuilder {
    pub(crate) fn build(handler: OsrBrowserProcessHandler) -> BrowserProcessHandler {
        BrowserProcessHandler::new(Self {
            object: std::ptr::null_mut(),
            handler,
        })
    }
}

impl Rc for BrowserProcessHandlerBuilder {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapBrowserProcessHandler for BrowserProcessHandlerBuilder {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_browser_process_handler_t, Self>) {
        self.object = object;
    }
}

impl Clone for BrowserProcessHandlerBuilder {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self {
            object,
            handler: self.handler.clone(),
        }
    }
}

impl ImplBrowserProcessHandler for BrowserProcessHandlerBuilder {
    fn get_raw(&self) -> *mut sys::_cef_browser_process_handler_t {
        self.object.cast()
    }

    fn on_context_initialized(&self) {
        *self.handler.is_cef_ready.borrow_mut() = true;
    }

    fn on_before_child_process_launch(&self, command_line: Option<&mut CommandLine>) {
        let Some(command_line) = command_line else {
            return;
        };

        command_line.append_switch(Some(&"disable-web-security".into()));
        command_line.append_switch(Some(&"allow-running-insecure-content".into()));
        command_line.append_switch(Some(&"disable-session-crashed-bubble".into()));
        command_line.append_switch(Some(&"ignore-certificate-errors".into()));
        command_line.append_switch(Some(&"ignore-ssl-errors".into()));
        command_line.append_switch(Some(&"enable-logging=stderr".into()));
    }
}

#[derive(Clone)]
pub struct OsrRenderHandler {
    device_scale_factor: f32,
    size: std::rc::Rc<RefCell<winit::dpi::LogicalSize<f32>>>,
    device: wgpu::Device,
    _queue: wgpu::Queue,
}

impl OsrRenderHandler {
    pub fn new(
        device: wgpu::Device,
        _queue: wgpu::Queue,
        device_scale_factor: f32,
        size: winit::dpi::LogicalSize<f32>,
    ) -> (Self, std::rc::Rc<RefCell<winit::dpi::LogicalSize<f32>>>) {
        let size = std::rc::Rc::new(RefCell::new(size));
        (
            Self {
                size: size.clone(),
                device_scale_factor,
                device,
                _queue,
            },
            size,
        )
    }
}

pub struct RenderHandlerBuilder {
    object: *mut RcImpl<sys::cef_render_handler_t, Self>,
    handler: OsrRenderHandler,
}

impl RenderHandlerBuilder {
    pub fn build(handler: OsrRenderHandler) -> RenderHandler {
        RenderHandler::new(Self {
            object: null_mut(),
            handler,
        })
    }
}

impl Rc for RenderHandlerBuilder {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}
impl WrapRenderHandler for RenderHandlerBuilder {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_render_handler_t, Self>) {
        self.object = object;
    }
}
impl Clone for RenderHandlerBuilder {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self {
            object,
            handler: self.handler.clone(),
        }
    }
}

impl ImplRenderHandler for RenderHandlerBuilder {
    fn get_raw(&self) -> *mut sys::_cef_render_handler_t {
        self.object.cast()
    }

    fn view_rect(&self, _browser: Option<&mut Browser>, rect: Option<&mut Rect>) {
        if let Some(rect) = rect {
            let size = self.handler.size.borrow();
            // size must be non-zero
            if size.width > 0.0 && size.height > 0.0 {
                rect.width = size.width as _;
                rect.height = size.height as _;
            }
        }
    }

    fn screen_info(
        &self,
        _browser: Option<&mut Browser>,
        screen_info: Option<&mut ScreenInfo>,
    ) -> ::std::os::raw::c_int {
        if let Some(screen_info) = screen_info {
            screen_info.device_scale_factor = self.handler.device_scale_factor;
            return true as _;
        }
        false as _
    }

    fn screen_point(
        &self,
        _browser: Option<&mut Browser>,
        _view_x: ::std::os::raw::c_int,
        _view_y: ::std::os::raw::c_int,
        _screen_x: Option<&mut ::std::os::raw::c_int>,
        _screen_y: Option<&mut ::std::os::raw::c_int>,
    ) -> ::std::os::raw::c_int {
        false as _
    }

    #[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
    fn on_accelerated_paint(
        &self,
        _browser: Option<&mut Browser>,
        type_: PaintElementType,
        _dirty_rects_count: usize,
        _dirty_rects: Option<&Rect>,
        info: Option<&AcceleratedPaintInfo>,
    ) {
        let Some(info) = info else { return };

        let src_texture = {
            // Import from current relative path: texture_import::shared_texture_handle::SharedTextureHandle;
            use cef::osr_texture_import::shared_texture_handle::SharedTextureHandle;

            if type_ != PaintElementType::default() {
                return;
            }

            let shared_handle = SharedTextureHandle::new(info);
            if let SharedTextureHandle::Unsupported = shared_handle {
                eprintln!("Platform does not support accelerated painting");
                return;
            }

            match shared_handle.import_texture(&self.handler.device) {
                Ok(texture) => texture,
                Err(e) => {
                    eprintln!("Failed to import shared texture: {:?}", e);
                    return;
                }
            }
        };

        let sampler = self
            .handler
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });

        let texture_bind_group_layout =
            self.handler
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Cef Texture Bind Group Layout"),
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

        let bind_group = self
            .handler
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Cef Texture Bind Group"),
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&src_texture.create_view(
                            &wgpu::TextureViewDescriptor {
                                label: Some("Cef Texture View"),
                                ..Default::default()
                            },
                        )),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });

        TEXTURE.with_borrow_mut(|texture| {
            texture.replace(bind_group);
        });
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    fn on_paint(
        &self,
        _browser: Option<&mut Browser>,
        _type_: PaintElementType,
        _dirty_rects_count: usize,
        _dirty_rects: Option<&Rect>,
        buffer: *const u8,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) {
        if buffer.is_null() || width <= 0 || height <= 0 {
            return;
        }

        let buffer_size = (width * height * 4) as usize; // BGRA format
        let buffer_slice = unsafe { std::slice::from_raw_parts(buffer, buffer_size) };

        // Create texture from CEF paint buffer
        let texture_desc = TextureDescriptor {
            label: Some("CEF Paint Texture"),
            size: Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        };

        let texture = self.handler.device.create_texture(&texture_desc);

        // Upload the CEF buffer data to the texture
        self.handler._queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            buffer_slice,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width as u32),
                rows_per_image: Some(height as u32),
            },
            texture_desc.size,
        );

        // Create sampler
        let sampler = self
            .handler
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });

        // Create bind group layout (matching the existing one)
        let texture_bind_group_layout =
            self.handler
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("CEF Texture Bind Group Layout Linux"),
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

        // Create bind group
        let bind_group = self
            .handler
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("CEF Texture Bind Group Linux"),
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.create_view(
                            &wgpu::TextureViewDescriptor {
                                label: Some("CEF Texture View Linux"),
                                ..Default::default()
                            },
                        )),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            });

        // Update the global texture
        TEXTURE.with_borrow_mut(|texture| {
            texture.replace(bind_group);
        });
    }
}

thread_local! {
    pub static TEXTURE: RefCell<Option<wgpu::BindGroup>> = RefCell::new(None);
}

pub(crate) struct ClientBuilder {
    object: *mut RcImpl<sys::cef_client_t, Self>,
    render_handler: RenderHandler,
}

impl ClientBuilder {
    pub(crate) fn build(render_handler: OsrRenderHandler) -> Client {
        Client::new(Self {
            object: null_mut(),
            render_handler: RenderHandlerBuilder::build(render_handler),
        })
    }
}

impl Rc for ClientBuilder {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl WrapClient for ClientBuilder {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::cef_client_t, Self>) {
        self.object = object;
    }
}

impl Clone for ClientBuilder {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self {
            object,
            render_handler: self.render_handler.clone(),
        }
    }
}

impl ImplClient for ClientBuilder {
    fn get_raw(&self) -> *mut sys::_cef_client_t {
        self.object.cast()
    }

    fn render_handler(&self) -> Option<cef::RenderHandler> {
        Some(self.render_handler.clone())
    }
}

#[derive(Clone)]
pub struct OsrRequestContextHandler {}

pub(crate) struct RequestContextHandlerBuilder {
    object: *mut RcImpl<sys::cef_request_context_handler_t, Self>,
    handler: OsrRequestContextHandler,
}

impl RequestContextHandlerBuilder {
    pub(crate) fn build(handler: OsrRequestContextHandler) -> RequestContextHandler {
        RequestContextHandler::new(Self {
            object: null_mut(),
            handler,
        })
    }
}

impl WrapRequestContextHandler for RequestContextHandlerBuilder {
    fn wrap_rc(&mut self, object: *mut RcImpl<sys::_cef_request_context_handler_t, Self>) {
        self.object = object;
    }
}

impl Rc for RequestContextHandlerBuilder {
    fn as_base(&self) -> &sys::cef_base_ref_counted_t {
        unsafe {
            let base = &*self.object;
            std::mem::transmute(&base.cef_object)
        }
    }
}

impl Clone for RequestContextHandlerBuilder {
    fn clone(&self) -> Self {
        let object = unsafe {
            let rc_impl = &mut *self.object;
            rc_impl.interface.add_ref();
            rc_impl
        };

        Self {
            object,
            handler: self.handler.clone(),
        }
    }
}

impl ImplRequestContextHandler for RequestContextHandlerBuilder {
    fn get_raw(&self) -> *mut sys::_cef_request_context_handler_t {
        self.object.cast()
    }
}
