use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::{Window as WinitWindow, WindowId};

use super::gpu::GPUContext;
use super::handler::{Event, WindowOnEventContext};
use super::input::{InputController, InputState};
use super::WindowHandler;

pub struct Window<H: WindowHandler> {
    handler: H,
    ctx: Option<GPUContext>,
    input_state: InputState,
}

impl<H: WindowHandler> Window<H> {
    pub fn new(handler: H) -> Self {
        return Self {
            handler,
            ctx: None,
            input_state: InputState::new(),
        };
    }

    pub fn run(mut self) {
        env_logger::init();
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut self).unwrap();
    }
}

impl<H: WindowHandler> ApplicationHandler for Window<H> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.ctx.is_some() {
            return;
        }
        let handle = Arc::new(
            event_loop
                .create_window(
                    WinitWindow::default_attributes()
                        .with_title(crate::WINDOW_TITLE)
                        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None))),
                )
                .unwrap(),
        );
        let mut ctx = GPUContext::new(handle);
        let mut on_event_ctx = WindowOnEventContext {
            gpu: &mut ctx,
            event_loop,
            input: InputController::new(&mut self.input_state),
        };
        self.handler.on_event(&mut on_event_ctx, Event::Resume);
        self.ctx = Some(ctx);
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.ctx = None;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Self {
            handler,
            ctx,
            input_state,
        } = self;
        let Some(ref mut ctx) = ctx else { return };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;
                let text = event.text.map(|s| s.to_string());
                if let PhysicalKey::Code(key) = event.physical_key {
                    input_state.handle_key(key, pressed, text.as_deref());
                }
            }
            WindowEvent::Resized(size) => {
                ctx.resize(size.width, size.height);
                let mut on_event_ctx = WindowOnEventContext {
                    gpu: ctx,
                    event_loop,
                    input: InputController::new(input_state),
                };
                handler.on_event(
                    &mut on_event_ctx,
                    Event::Resize {
                        width: size.width,
                        height: size.height,
                    },
                );
            }
            WindowEvent::RedrawRequested => {
                let mut on_event_ctx = WindowOnEventContext {
                    gpu: ctx,
                    event_loop,
                    input: InputController::new(input_state),
                };
                handler.on_event(&mut on_event_ctx, Event::Redraw);
                ctx.handle.request_redraw();
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
        if let DeviceEvent::MouseMotion { delta } = event {
            self.input_state
                .handle_mouse((delta.0 as f32, delta.1 as f32));
        }
    }
}
