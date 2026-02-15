use glam::Vec2;
use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::Key;

use super::input::{
    window_input_key, window_input_last_pressed, window_input_mouse_delta, window_input_reset,
    window_input_typed_chars, WindowKeyState,
};
use super::state::WindowState;

pub struct WindowContext<'a> {
    event_loop: &'a ActiveEventLoop,
    state: &'a mut WindowState,
}

impl<'a> WindowContext<'a> {
    pub fn new(event_loop: &'a ActiveEventLoop, state: &'a mut WindowState) -> Self {
        return Self { event_loop, state };
    }

    pub fn exit(&self) {
        self.event_loop.exit();
    }

    pub fn device(&self) -> &Arc<wgpu::Device> {
        return &self.state.device;
    }

    pub fn queue(&self) -> &Arc<wgpu::Queue> {
        return &self.state.queue;
    }

    pub fn size(&self) -> Vec2 {
        return Vec2::new(
            self.state.config.width as f32,
            self.state.config.height as f32,
        );
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        return self.state.config.format;
    }

    pub fn current_texture(&self) -> wgpu::SurfaceTexture {
        return self.state.surface.get_current_texture().unwrap();
    }

    pub fn request_redraw(&self) {
        self.state.handle.request_redraw();
    }

    pub fn resize(&mut self, size: Vec2) {
        if size.x > 0.0 && size.y > 0.0 {
            self.state.config.width = size.x as u32;
            self.state.config.height = size.y as u32;
            self.state
                .surface
                .configure(self.device(), &self.state.config);
        }
    }

    pub fn input_reset(&mut self) {
        window_input_reset(self.state);
    }

    pub fn key(&self, key: &Key) -> WindowKeyState {
        return window_input_key(self.state, key);
    }

    pub fn mouse_delta(&self) -> Vec2 {
        return window_input_mouse_delta(self.state);
    }

    pub fn typed_chars(&self) -> &str {
        return window_input_typed_chars(self.state);
    }

    pub fn last_pressed(&self) -> Option<Key> {
        return window_input_last_pressed(self.state);
    }
}
