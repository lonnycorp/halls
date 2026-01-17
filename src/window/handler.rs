use winit::event_loop::ActiveEventLoop;

use super::gpu::GPUContext;
use super::input::InputController;

pub enum Event {
    Resume,
    Resize { width: u32, height: u32 },
    Redraw,
}

pub struct WindowOnEventContext<'a> {
    pub gpu: &'a mut GPUContext,
    pub event_loop: &'a ActiveEventLoop,
    pub input: InputController<'a>,
}

pub trait WindowHandler {
    fn on_event(&mut self, ctx: &mut WindowOnEventContext<'_>, event: Event);
}
