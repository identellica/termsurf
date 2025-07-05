use cef::{
    self, BrowserProcessHandler, ImplBrowserProcessHandler, WrapBrowserProcessHandler,
    rc::{Rc, RcImpl},
    sys::{self, cef_color_type_t},
    *,
};
use cef::{ImplRequestContextHandler, RequestContextHandler, WrapRequestContextHandler};
use std::cell::RefCell;
use std::ptr::null_mut;
use wgpu::{
    Extent3d, Texture, TextureDescriptor, TextureDimension, TextureUsages, TextureUses,
    hal::MemoryFlags,
    wgc::api::{Dx12, Vulkan},
};
use windows::Win32::Graphics::Direct3D12;

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
    }
}

#[derive(Clone)]
pub struct OsrRenderHandler {
    device_scale_factor: f32,
    device: wgpu::Device,
}

impl OsrRenderHandler {
    pub fn new(device: wgpu::Device, device_scale_factor: f32) -> Self {
        Self {
            device_scale_factor,
            device,
        }
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
            rect.x = 0;
            rect.y = 0;
            rect.width = 800;
            rect.height = 600;
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
        return false as _;
    }

    fn screen_point(
        &self,
        _browser: Option<&mut Browser>,
        _view_x: ::std::os::raw::c_int,
        _view_y: ::std::os::raw::c_int,
        _screen_x: Option<&mut ::std::os::raw::c_int>,
        _screen_y: Option<&mut ::std::os::raw::c_int>,
    ) -> ::std::os::raw::c_int {
        return false as _;
    }

    fn on_accelerated_paint(
        &self,
        browser: Option<&mut Browser>,
        type_: PaintElementType,
        dirty_rects_count: usize,
        dirty_rects: Option<&Rect>,
        info: Option<&AcceleratedPaintInfo>,
    ) {
        let Some(info) = info else { return };
        let format = match info.format.as_ref() {
            cef_color_type_t::CEF_COLOR_TYPE_BGRA_8888 => wgpu::TextureFormat::Bgra8UnormSrgb,
            cef_color_type_t::CEF_COLOR_TYPE_RGBA_8888 => wgpu::TextureFormat::Rgba8UnormSrgb,
            _ => panic!("Unsupported color type"),
        };
        let texture_desc = TextureDescriptor {
            label: Some("cef"),
            size: Extent3d {
                width: info.extra.coded_size.width as _,
                height: info.extra.coded_size.height as _,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };
        let handle = windows::Win32::Foundation::HANDLE(info.shared_texture_handle.cast());
        let resource = unsafe {
            //self.handler.device.as_hal::<Vulkan, _, _>(|hdevice| {
            //    hdevice.map(|device| {
            //        let desc = wgpu::hal::TextureDescriptor {
            //            label: "cef-vulkan".into(),
            //            size: Extent3d {
            //                width: info.extra.coded_size.width as _,
            //                height: info.extra.coded_size.height as _,
            //                depth_or_array_layers: 1,
            //            },
            //            mip_level_count: 1,
            //            sample_count: 1,
            //            dimension: TextureDimension::D2,
            //            format,
            //            usage: TextureUses::COPY_DST,
            //            memory_flags: MemoryFlags::empty(),
            //            view_formats: vec![],
            //        };
            //        device.texture_from_d3d11_shared_handle(handle, &desc)
            //    })
            //})
            self.handler.device.as_hal::<Dx12, _, _>(|hdevice| {
                hdevice.map(|hdevice| {
                    let raw_device = hdevice.raw_device();

                    let mut resource = None::<Direct3D12::ID3D12Resource>;
                    match raw_device.OpenSharedHandle(handle, &mut resource) {
                        Ok(_) => Ok(resource.unwrap()),
                        Err(e) => Err(e),
                    }
                })
            })
        };
        let resource = resource.unwrap().unwrap();

        unsafe {
            let texture = <Dx12 as wgpu::hal::Api>::Device::texture_from_raw(
                resource,
                texture_desc.format,
                texture_desc.dimension,
                texture_desc.size,
                1,
                1,
            );

            TEXTURE.with_borrow_mut(|t| {
                t.replace(
                    self.handler
                        .device
                        .create_texture_from_hal::<Dx12>(texture, &texture_desc),
                )
            });
        }
    }
}

thread_local! {
    pub static TEXTURE: RefCell<Option<Texture>> = RefCell::new(None);
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

    fn on_request_context_initialized(&self, _request_context: Option<&mut cef::RequestContext>) {
        dbg!("on_request_context_initialized");
    }
}
