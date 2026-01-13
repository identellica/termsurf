mod webrender;

use cef::{args::Args, *};
use std::{cell::RefCell, process::ExitCode, sync::Arc, thread::sleep, time::Duration};
use wgpu::Backends;
use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    platform::pump_events::{EventLoopExtPumpEvents, PumpStatus},
    window::{Window, WindowAttributes, WindowId},
};


use crate::webrender::{
    ClientBuilder, OsrApp, OsrRenderHandler, OsrRequestContextHandler,
    RequestContextHandlerBuilder, TEXTURE,
};

struct State {
    window: Arc<Window>,
    device: wgpu::Device,
    pipeline: wgpu::RenderPipeline,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    quad: Geometry,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(target_os = "windows")]
            backends: Backends::from_comma_list("dx12"),
            #[cfg(target_os = "macos")]
            backends: Backends::from_comma_list("metal"),
            #[cfg(target_os = "linux")]
            backends: Backends::from_comma_list("vulkan"),
            //flags: wgpu::InstanceFlags::debugging(),
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                ..Default::default()
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_limits: wgpu::Limits {
                    max_non_sampler_bindings: 2048,
                    ..Default::default()
                },
                ..Default::default()
            })
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let surface_format = wgpu::TextureFormat::Bgra8Unorm;
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cef Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Cef Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                immediate_size: 0,
            });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cef Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::OVER,
                        alpha: wgpu::BlendComponent::OVER,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });
        let quad = Geometry::new(&device);

        let state = State {
            window,
            pipeline,
            device,
            queue,
            size,
            surface,
            surface_format,
            quad,
        };

        state.configure_surface();

        state
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.configure_surface();
        }
    }

    fn render(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let frame = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                label: Some("Surface"),
                format: Some(wgpu::TextureFormat::Bgra8Unorm),
                ..Default::default()
            });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Always clear the screen, only draw quad if we have a CEF texture
        TEXTURE.with_borrow(|textures| {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Cef Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                ..Default::default()
            });

            // Only draw the textured quad if CEF has provided a frame
            if let Some(bind_group) = textures.as_ref() {
                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_bind_group(0, bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.quad.vertex_buffer.slice(..));
                render_pass.draw(0..self.quad.vertex_count, 0..1);
            }
        });

        self.queue.submit(std::iter::once(encoder.finish()));
        self.window.pre_present_notify();
        surface_texture.present();
    }
}

struct App {
    state: Option<State>,
    browser: Option<Browser>,
    /// Current cursor position in logical coordinates (for CEF)
    cursor_pos: (f64, f64),
    /// Current keyboard modifier state (shift, ctrl, alt, cmd)
    key_modifiers: u32,
    /// Current mouse button state
    mouse_buttons: u32,
    /// Flag to track if we're in the process of closing
    closing: bool,
}

struct Browser {
    browser: cef::Browser,
    size: std::rc::Rc<RefCell<winit::dpi::LogicalSize<f32>>>,
}

impl App {
    fn new() -> Self {
        App {
            state: None,
            browser: None,
            cursor_pos: (0.0, 0.0),
            key_modifiers: 0,
            mouse_buttons: 0,
            closing: false,
        }
    }

    /// Get the browser host if available
    fn host(&self) -> Option<BrowserHost> {
        self.browser.as_ref().and_then(|b| b.browser.host())
    }

    /// Get the window's scale factor
    fn scale_factor(&self) -> f64 {
        self.state
            .as_ref()
            .map(|s| s.window.scale_factor())
            .unwrap_or(1.0)
    }

    /// Convert winit MouseButton to CEF MouseButtonType
    fn to_cef_button(button: MouseButton) -> MouseButtonType {
        match button {
            MouseButton::Left => MouseButtonType::LEFT,
            MouseButton::Right => MouseButtonType::RIGHT,
            MouseButton::Middle => MouseButtonType::MIDDLE,
            _ => MouseButtonType::LEFT,
        }
    }

    /// Create a CEF MouseEvent from current cursor position
    fn mouse_event(&self) -> MouseEvent {
        MouseEvent {
            x: self.cursor_pos.0 as i32,
            y: self.cursor_pos.1 as i32,
            modifiers: self.key_modifiers | self.mouse_buttons,
        }
    }

    /// Get combined modifiers for key events
    fn all_modifiers(&self) -> u32 {
        self.key_modifiers | self.mouse_buttons
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = WindowAttributes::default()
            .with_title("CEF OSR Example")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 800.0));

        let window = Arc::new(
            event_loop
                .create_window(window_attrs)
                .unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone()));
        self.state = Some(state);
        let accelerated_osr = cfg!(all(
            any(
                target_os = "macos",
                target_os = "windows",
                target_os = "linux"
            ),
            feature = "accelerated_osr"
        ));
        let window_info = WindowInfo {
            windowless_rendering_enabled: true as _,
            shared_texture_enabled: accelerated_osr as _,
            external_begin_frame_enabled: accelerated_osr as _,
            ..Default::default()
        };

        let device_scale_factor = window.scale_factor();
        let (render_handler, browser_size) = OsrRenderHandler::new(
            self.state.as_ref().unwrap().device.clone(),
            self.state.as_ref().unwrap().queue.clone(),
            device_scale_factor as _,
            window.inner_size().to_logical(device_scale_factor),
        );

        let browser_settings = BrowserSettings {
            windowless_frame_rate: 60,
            ..Default::default()
        };
        let mut context = cef::request_context_create_context(
            Some(&RequestContextSettings::default()),
            Some(&mut RequestContextHandlerBuilder::build(
                OsrRequestContextHandler {},
            )),
        );

        let browser = cef::browser_host_create_browser_sync(
            Some(&window_info),
            Some(&mut ClientBuilder::build(render_handler)),
            Some(&"https:://github.com".into()),
            Some(&browser_settings),
            None,
            context.as_mut(),
        );
        assert!(browser.is_some());

        self.browser.replace(Browser {
            browser: browser.unwrap(),
            size: browser_size,
        });

        println!("CEF OSR Example running");
        println!("  Note: Fullscreen is disabled (CEF OSR has rendering issues with fullscreen)");

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                // Properly close the browser before exiting
                if !self.closing {
                    self.closing = true;
                    if let Some(host) = self.host() {
                        // force_close=1 to skip beforeunload dialog
                        host.close_browser(1);
                    }
                }
                // Give CEF time to clean up, then exit
                // The browser will be dropped, then we can safely exit
                self.browser = None;
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
                if let Some(host) = self.browser.as_mut().and_then(|b| b.browser.host()) {
                    host.send_external_begin_frame();
                }
                state.render();
                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                state.resize(size);
                if let Some(browser) = self.browser.as_mut() {
                    *browser.size.borrow_mut() =
                        size.to_logical(state.window.scale_factor());
                    if let Some(host) = browser.browser.host() {
                        host.was_resized();
                    }
                }
            }

            // Mouse movement
            WindowEvent::CursorMoved { position, .. } => {
                // Convert physical to logical coordinates
                let scale = self.scale_factor();
                self.cursor_pos = (position.x / scale, position.y / scale);

                if let Some(host) = self.host() {
                    host.send_mouse_move_event(Some(&self.mouse_event()), 0);
                }
            }

            // Mouse clicks
            WindowEvent::MouseInput { state: elem_state, button, .. } => {
                // Update mouse button state for drag selection to work
                let button_flag = match button {
                    MouseButton::Left => EVENTFLAG_LEFT_MOUSE_BUTTON,
                    MouseButton::Middle => EVENTFLAG_MIDDLE_MOUSE_BUTTON,
                    MouseButton::Right => EVENTFLAG_RIGHT_MOUSE_BUTTON,
                    _ => 0,
                };

                match elem_state {
                    ElementState::Pressed => self.mouse_buttons |= button_flag,
                    ElementState::Released => self.mouse_buttons &= !button_flag,
                }

                if let Some(host) = self.host() {
                    let mouse_up = match elem_state {
                        ElementState::Pressed => 0,
                        ElementState::Released => 1,
                    };
                    let cef_button = Self::to_cef_button(button);
                    // click_count: 1 for single click
                    host.send_mouse_click_event(Some(&self.mouse_event()), cef_button, mouse_up, 1);
                }
            }

            // Mouse wheel scrolling
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(host) = self.host() {
                    let (delta_x, delta_y) = match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            // Line delta - multiply by pixels-per-line (120 is Windows standard)
                            ((x * 120.0) as i32, (y * 120.0) as i32)
                        }
                        MouseScrollDelta::PixelDelta(pos) => {
                            // Pixel delta from trackpad - scale up for responsiveness
                            ((pos.x * 2.0) as i32, (pos.y * 2.0) as i32)
                        }
                    };
                    host.send_mouse_wheel_event(Some(&self.mouse_event()), delta_x, delta_y);
                }
            }

            // Cursor left window
            WindowEvent::CursorLeft { .. } => {
                if let Some(host) = self.host() {
                    host.send_mouse_move_event(Some(&self.mouse_event()), 1); // mouse_leave = 1
                }
            }

            // Window focus
            WindowEvent::Focused(focused) => {
                if let Some(host) = self.host() {
                    host.set_focus(focused as i32);
                }
            }

            // Modifier keys changed
            WindowEvent::ModifiersChanged(mods) => {
                self.key_modifiers = 0;
                let mods_state = mods.state();
                if mods_state.shift_key() {
                    self.key_modifiers |= EVENTFLAG_SHIFT_DOWN;
                }
                if mods_state.control_key() {
                    self.key_modifiers |= EVENTFLAG_CONTROL_DOWN;
                }
                if mods_state.alt_key() {
                    self.key_modifiers |= EVENTFLAG_ALT_DOWN;
                }
                if mods_state.super_key() {
                    // Command key on macOS
                    self.key_modifiers |= EVENTFLAG_COMMAND_DOWN;
                }
            }

            // Keyboard input
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(host) = self.host() {
                    // Get the key code
                    let physical_key = if let PhysicalKey::Code(code) = event.physical_key {
                        Some(code)
                    } else {
                        None
                    };

                    // Check if this is a navigation key (arrows, home, end, etc.)
                    let is_navigation_key = physical_key.map_or(false, |code| matches!(code,
                        KeyCode::ArrowUp | KeyCode::ArrowDown | KeyCode::ArrowLeft | KeyCode::ArrowRight |
                        KeyCode::Home | KeyCode::End | KeyCode::PageUp | KeyCode::PageDown |
                        KeyCode::Backspace | KeyCode::Delete
                    ));

                    // For navigation keys, only send KEYDOWN (skip KEYUP to avoid double-action)
                    if is_navigation_key && event.state == ElementState::Released {
                        return;
                    }

                    // Determine key event type
                    let event_type = match event.state {
                        ElementState::Pressed => KeyEventType::KEYDOWN,
                        ElementState::Released => KeyEventType::KEYUP,
                    };

                    // Get the Windows virtual key code and native key code
                    let (windows_key_code, native_key_code) = if let Some(code) = physical_key {
                        (keycode_to_windows_vk(code), keycode_to_native(code))
                    } else {
                        (0, 0)
                    };

                    let modifiers = self.all_modifiers();

                    // Create and send the key event
                    let key_event = KeyEvent {
                        size: std::mem::size_of::<KeyEvent>(),
                        type_: event_type,
                        modifiers,
                        windows_key_code,
                        native_key_code,
                        is_system_key: 0,
                        character: 0,
                        unmodified_character: 0,
                        focus_on_editable_field: 0,
                    };
                    host.send_key_event(Some(&key_event));

                    // For printable characters, also send a CHAR event on key press
                    if event.state == ElementState::Pressed {
                        if let Some(text) = &event.text {
                            for ch in text.chars() {
                                let char_event = KeyEvent {
                                    size: std::mem::size_of::<KeyEvent>(),
                                    type_: KeyEventType::CHAR,
                                    modifiers,
                                    windows_key_code: ch as i32,
                                    native_key_code: 0,
                                    is_system_key: 0,
                                    character: ch as u16,
                                    unmodified_character: ch as u16,
                                    focus_on_editable_field: 0,
                                };
                                host.send_key_event(Some(&char_event));
                            }
                        }
                    }
                }
            }

            _ => (),
        }
    }
}

// CEF event flag constants
const EVENTFLAG_SHIFT_DOWN: u32 = 1 << 1;
const EVENTFLAG_CONTROL_DOWN: u32 = 1 << 2;
const EVENTFLAG_ALT_DOWN: u32 = 1 << 3;
const EVENTFLAG_LEFT_MOUSE_BUTTON: u32 = 1 << 4;
const EVENTFLAG_MIDDLE_MOUSE_BUTTON: u32 = 1 << 5;
const EVENTFLAG_RIGHT_MOUSE_BUTTON: u32 = 1 << 6;
const EVENTFLAG_COMMAND_DOWN: u32 = 1 << 7; // Meta/Command key

/// Convert winit KeyCode to macOS native key code (Carbon virtual key codes)
#[cfg(target_os = "macos")]
fn keycode_to_native(code: KeyCode) -> i32 {
    match code {
        // Letters (QWERTY layout)
        KeyCode::KeyA => 0x00,
        KeyCode::KeyS => 0x01,
        KeyCode::KeyD => 0x02,
        KeyCode::KeyF => 0x03,
        KeyCode::KeyH => 0x04,
        KeyCode::KeyG => 0x05,
        KeyCode::KeyZ => 0x06,
        KeyCode::KeyX => 0x07,
        KeyCode::KeyC => 0x08,
        KeyCode::KeyV => 0x09,
        KeyCode::KeyB => 0x0B,
        KeyCode::KeyQ => 0x0C,
        KeyCode::KeyW => 0x0D,
        KeyCode::KeyE => 0x0E,
        KeyCode::KeyR => 0x0F,
        KeyCode::KeyY => 0x10,
        KeyCode::KeyT => 0x11,
        KeyCode::Digit1 => 0x12,
        KeyCode::Digit2 => 0x13,
        KeyCode::Digit3 => 0x14,
        KeyCode::Digit4 => 0x15,
        KeyCode::Digit6 => 0x16,
        KeyCode::Digit5 => 0x17,
        KeyCode::Equal => 0x18,
        KeyCode::Digit9 => 0x19,
        KeyCode::Digit7 => 0x1A,
        KeyCode::Minus => 0x1B,
        KeyCode::Digit8 => 0x1C,
        KeyCode::Digit0 => 0x1D,
        KeyCode::BracketRight => 0x1E,
        KeyCode::KeyO => 0x1F,
        KeyCode::KeyU => 0x20,
        KeyCode::BracketLeft => 0x21,
        KeyCode::KeyI => 0x22,
        KeyCode::KeyP => 0x23,
        KeyCode::Enter => 0x24,
        KeyCode::KeyL => 0x25,
        KeyCode::KeyJ => 0x26,
        KeyCode::Quote => 0x27,
        KeyCode::KeyK => 0x28,
        KeyCode::Semicolon => 0x29,
        KeyCode::Backslash => 0x2A,
        KeyCode::Comma => 0x2B,
        KeyCode::Slash => 0x2C,
        KeyCode::KeyN => 0x2D,
        KeyCode::KeyM => 0x2E,
        KeyCode::Period => 0x2F,
        KeyCode::Tab => 0x30,
        KeyCode::Space => 0x31,
        KeyCode::Backquote => 0x32,
        KeyCode::Backspace => 0x33,  // kVK_Delete (backspace)
        KeyCode::Escape => 0x35,

        // Function keys
        KeyCode::F1 => 0x7A,
        KeyCode::F2 => 0x78,
        KeyCode::F3 => 0x63,
        KeyCode::F4 => 0x76,
        KeyCode::F5 => 0x60,
        KeyCode::F6 => 0x61,
        KeyCode::F7 => 0x62,
        KeyCode::F8 => 0x64,
        KeyCode::F9 => 0x65,
        KeyCode::F10 => 0x6D,
        KeyCode::F11 => 0x67,
        KeyCode::F12 => 0x6F,

        // Navigation
        KeyCode::Home => 0x73,
        KeyCode::PageUp => 0x74,
        KeyCode::Delete => 0x75,      // kVK_ForwardDelete
        KeyCode::End => 0x77,
        KeyCode::PageDown => 0x79,
        KeyCode::ArrowLeft => 0x7B,   // kVK_LeftArrow
        KeyCode::ArrowRight => 0x7C,  // kVK_RightArrow
        KeyCode::ArrowDown => 0x7D,   // kVK_DownArrow
        KeyCode::ArrowUp => 0x7E,     // kVK_UpArrow

        _ => 0,
    }
}

#[cfg(not(target_os = "macos"))]
fn keycode_to_native(_code: KeyCode) -> i32 {
    0 // Native key codes not needed on other platforms
}

/// Convert winit KeyCode to Windows virtual key code
/// CEF uses Windows VK codes on all platforms
fn keycode_to_windows_vk(code: KeyCode) -> i32 {
    match code {
        // Letters
        KeyCode::KeyA => 0x41,
        KeyCode::KeyB => 0x42,
        KeyCode::KeyC => 0x43,
        KeyCode::KeyD => 0x44,
        KeyCode::KeyE => 0x45,
        KeyCode::KeyF => 0x46,
        KeyCode::KeyG => 0x47,
        KeyCode::KeyH => 0x48,
        KeyCode::KeyI => 0x49,
        KeyCode::KeyJ => 0x4A,
        KeyCode::KeyK => 0x4B,
        KeyCode::KeyL => 0x4C,
        KeyCode::KeyM => 0x4D,
        KeyCode::KeyN => 0x4E,
        KeyCode::KeyO => 0x4F,
        KeyCode::KeyP => 0x50,
        KeyCode::KeyQ => 0x51,
        KeyCode::KeyR => 0x52,
        KeyCode::KeyS => 0x53,
        KeyCode::KeyT => 0x54,
        KeyCode::KeyU => 0x55,
        KeyCode::KeyV => 0x56,
        KeyCode::KeyW => 0x57,
        KeyCode::KeyX => 0x58,
        KeyCode::KeyY => 0x59,
        KeyCode::KeyZ => 0x5A,

        // Numbers
        KeyCode::Digit0 => 0x30,
        KeyCode::Digit1 => 0x31,
        KeyCode::Digit2 => 0x32,
        KeyCode::Digit3 => 0x33,
        KeyCode::Digit4 => 0x34,
        KeyCode::Digit5 => 0x35,
        KeyCode::Digit6 => 0x36,
        KeyCode::Digit7 => 0x37,
        KeyCode::Digit8 => 0x38,
        KeyCode::Digit9 => 0x39,

        // Function keys
        KeyCode::F1 => 0x70,
        KeyCode::F2 => 0x71,
        KeyCode::F3 => 0x72,
        KeyCode::F4 => 0x73,
        KeyCode::F5 => 0x74,
        KeyCode::F6 => 0x75,
        KeyCode::F7 => 0x76,
        KeyCode::F8 => 0x77,
        KeyCode::F9 => 0x78,
        KeyCode::F10 => 0x79,
        KeyCode::F11 => 0x7A,
        KeyCode::F12 => 0x7B,

        // Navigation
        KeyCode::ArrowUp => 0x26,
        KeyCode::ArrowDown => 0x28,
        KeyCode::ArrowLeft => 0x25,
        KeyCode::ArrowRight => 0x27,
        KeyCode::Home => 0x24,
        KeyCode::End => 0x23,
        KeyCode::PageUp => 0x21,
        KeyCode::PageDown => 0x22,

        // Editing
        KeyCode::Backspace => 0x08,
        KeyCode::Delete => 0x2E,
        KeyCode::Insert => 0x2D,
        KeyCode::Enter => 0x0D,
        KeyCode::Tab => 0x09,
        KeyCode::Escape => 0x1B,
        KeyCode::Space => 0x20,

        // Modifiers
        KeyCode::ShiftLeft | KeyCode::ShiftRight => 0x10,
        KeyCode::ControlLeft | KeyCode::ControlRight => 0x11,
        KeyCode::AltLeft | KeyCode::AltRight => 0x12,
        KeyCode::SuperLeft | KeyCode::SuperRight => 0x5B, // Windows/Command key

        // Punctuation
        KeyCode::Semicolon => 0xBA,
        KeyCode::Equal => 0xBB,
        KeyCode::Comma => 0xBC,
        KeyCode::Minus => 0xBD,
        KeyCode::Period => 0xBE,
        KeyCode::Slash => 0xBF,
        KeyCode::Backquote => 0xC0,
        KeyCode::BracketLeft => 0xDB,
        KeyCode::Backslash => 0xDC,
        KeyCode::BracketRight => 0xDD,
        KeyCode::Quote => 0xDE,

        _ => 0,
    }
}

fn main() -> std::process::ExitCode {
    #[cfg(all(target_os = "windows", debug_assertions))]
    pix::load_winpix_gpu_capturer().unwrap();

    #[cfg(target_os = "macos")]
    let _loader = {
        let loader = library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), false);
        assert!(loader.load());
        loader
    };

    env_logger::init();

    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);

    let args = Args::new();
    let cmd = args.as_cmd_line().unwrap();

    let switch = CefString::from("type");
    let is_browser_process = cmd.has_switch(Some(&switch)) != 1;
    let mut app = webrender::AppBuilder::build(OsrApp::new());
    let ret = execute_process(
        Some(args.as_main_args()),
        Some(&mut app),
        std::ptr::null_mut(),
    );

    if is_browser_process {
        assert!(ret == -1, "cannot execute browser process");
    } else {
        let process_type = CefString::from(&cmd.switch_value(Some(&switch)));
        println!("launch process {process_type}");
        assert!(ret >= 0, "cannot execute non-browser process");
        // non-browser process does not initialize cef
        return 0.into();
    }
    let settings = Settings {
        windowless_rendering_enabled: true as _,
        external_message_pump: true as _,
        ..Default::default()
    };
    assert_eq!(
        initialize(
            Some(args.as_main_args()),
            Some(&settings),
            Some(&mut app),
            std::ptr::null_mut(),
        ),
        1
    );

    let mut event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    let ret = loop {
        do_message_loop_work();
        let timeout = Some(Duration::ZERO);
        let status = event_loop.pump_app_events(timeout, &mut app);

        if let PumpStatus::Exit(exit_code) = status {
            break ExitCode::from(exit_code as u8);
        }

        // Target ~60fps (16ms per frame)
        sleep(Duration::from_millis(16));
    };

    // Give CEF time to finish cleanup before shutdown
    for _ in 0..10 {
        do_message_loop_work();
        sleep(Duration::from_millis(10));
    }

    cef::shutdown();
    ret
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

struct Geometry {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
}

impl Geometry {
    fn new(device: &wgpu::Device) -> Self {
        let x = -1.0;
        let y = 1.0;
        let width = 2.0;
        let height = 2.0;
        let z = 1.0; // Z value for 2D quad

        let vertices = [
            Vertex {
                position: [x, y, z],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [x + width, y, z],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [x, y - height, z],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [x + width, y - height, z],
                tex_coords: [1.0, 1.0],
            },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Quad Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertex_buffer,
            vertex_count: vertices.len() as u32,
        }
    }
}

#[cfg(all(target_os = "windows", debug_assertions))]
mod pix {
    use libloading::Library;
    use std::io::{Error, ErrorKind, Result};
    use std::path::PathBuf;
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::core::{HSTRING, PCWSTR};

    fn get_latest_winpix_gpu_capturer_path() -> PathBuf {
        PathBuf::from(r"C:\Program Files")
            .join("Microsoft PIX")
            .join("2505.30")
            .join("WinPixGpuCapturer.dll")
    }

    pub fn load_winpix_gpu_capturer() -> Result<()> {
        let module_name = HSTRING::from("WinPixGpuCapturer.dll");

        unsafe {
            let module_pcwstr = PCWSTR::from_raw(module_name.as_ptr());
            let is_loaded = GetModuleHandleW(module_pcwstr).is_ok();

            if !is_loaded {
                let path = get_latest_winpix_gpu_capturer_path();

                if !path.exists() {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("WinPixGpuCapturer.dll not found at {}", path.display()),
                    ));
                }

                match Library::new(&path) {
                    Ok(lib) => {
                        use std::sync::Once;
                        static INIT: Once = Once::new();
                        static mut LIBRARY: Option<Library> = None;

                        INIT.call_once(|| {
                            LIBRARY = Some(lib);
                        });

                        Ok(())
                    }
                    Err(e) => Err(Error::other(format!(
                        "Failed to load WinPixGpuCapturer.dll: {e}"
                    ))),
                }
            } else {
                Ok(())
            }
        }
    }
}
