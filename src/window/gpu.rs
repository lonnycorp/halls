use std::sync::Arc;
use winit::window::{CursorGrabMode, Window as WinitWindow};

pub struct GPUContext {
    pub handle: Arc<WinitWindow>,
    surface: wgpu::Surface<'static>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    config: wgpu::SurfaceConfiguration,
}

impl GPUContext {
    pub fn new(handle: Arc<WinitWindow>) -> Self {
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
        };
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn current_texture(&self) -> wgpu::SurfaceTexture {
        return self.surface.get_current_texture().unwrap();
    }

    pub fn size(&self) -> (u32, u32) {
        return (self.config.width, self.config.height);
    }

    pub fn aspect(&self) -> f32 {
        return self.config.width as f32 / self.config.height as f32;
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        return self.config.format;
    }
}
