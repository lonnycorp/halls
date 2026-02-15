use super::context::WindowContext;

pub enum WindowHandlerEvent {
    Resume,
    Resize,
    Redraw,
}

pub trait WindowHandler {
    fn on_event(&mut self, ctx: &mut WindowContext<'_>, event: WindowHandlerEvent);
}
