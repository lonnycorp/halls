use glam::Vec2;
use std::collections::HashMap;
use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::Key;
use winit::window::{CursorGrabMode, Window as WinitWindow};

pub enum WindowInputEdge {
    Pressed,
    Released,
}

pub struct WindowInputKey {
    pub tick: u64,
    pub edge: WindowInputEdge,
}

pub struct WindowState {
    pub handle: Arc<WinitWindow>,
    pub surface: wgpu::Surface<'static>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub config: wgpu::SurfaceConfiguration,
    pub input_mouse_delta: Vec2,
    pub input_typed_chars: String,
    pub input_tick: u64,
    pub input_keys: HashMap<Key, WindowInputKey>,
    pub input_last_pressed: Option<Key>,
}

impl WindowState {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let handle = Arc::new(
            event_loop
                .create_window(
                    WinitWindow::default_attributes()
                        .with_title(crate::WINDOW_TITLE)
                        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None))),
                )
                .unwrap(),
        );

        let size = handle.inner_size();

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(handle.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::DEPTH_CLIP_CONTROL
                    | wgpu::Features::TEXTURE_BINDING_ARRAY
                    | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
                    | wgpu::Features::PUSH_CONSTANTS,
                required_limits: wgpu::Limits {
                    max_push_constant_size: 128,
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        ))
        .unwrap();

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        config.present_mode = wgpu::PresentMode::Fifo;
        surface.configure(&device, &config);

        let _ = handle
            .set_cursor_grab(CursorGrabMode::Locked)
            .or_else(|_| handle.set_cursor_grab(CursorGrabMode::Confined));
        handle.set_cursor_visible(false);

        return Self {
            handle,
            surface,
            device,
            queue,
            config,
            input_mouse_delta: Vec2::ZERO,
            input_typed_chars: String::new(),
            input_tick: 1,
            input_keys: HashMap::new(),
            input_last_pressed: None,
        };
    }
}
