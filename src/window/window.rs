use glam::Vec2;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

use super::context::WindowContext;
use super::handler::WindowHandlerEvent;
use super::input::{window_input_key_handle, window_input_mouse_handle};
use super::state::WindowState;
use super::WindowHandler;

pub struct Window<H: WindowHandler> {
    handler: H,
    state: Option<WindowState>,
}

impl<H: WindowHandler> Window<H> {
    pub fn new(handler: H) -> Self {
        return Self {
            handler,
            state: None,
        };
    }

    pub fn run(&mut self) {
        env_logger::init();
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self).unwrap();
    }
}

impl<H: WindowHandler> ApplicationHandler for Window<H> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let mut state = WindowState::new(event_loop);
        let mut on_event_ctx = WindowContext::new(event_loop, &mut state);
        self.handler
            .on_event(&mut on_event_ctx, WindowHandlerEvent::Resume);
        self.state = Some(state);
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.state = None;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Self { handler, state } = self;
        let Some(state) = state else { return };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                window_input_key_handle(state, &event);
            }
            WindowEvent::Resized(size) => {
                let mut on_event_ctx = WindowContext::new(event_loop, state);
                on_event_ctx.resize(Vec2::new(size.width as f32, size.height as f32));
                handler.on_event(&mut on_event_ctx, WindowHandlerEvent::Resize);
            }
            WindowEvent::RedrawRequested => {
                let mut on_event_ctx = WindowContext::new(event_loop, state);
                handler.on_event(&mut on_event_ctx, WindowHandlerEvent::Redraw);
                on_event_ctx.request_redraw();
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        let Some(state) = self.state.as_mut() else {
            return;
        };

        if let DeviceEvent::MouseMotion { delta } = event {
            window_input_mouse_handle(state, (delta.0 as f32, delta.1 as f32));
        }
    }
}
