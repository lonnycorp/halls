use strum::{EnumCount, EnumIter};
use winit::event_loop::ActiveEventLoop;

use crate::{Status, StatusBuffer};

#[derive(EnumCount, EnumIter)]
pub enum MenuHomeItem {
    Visit,
    Settings,
    Quit,
}

impl MenuHomeItem {
    pub fn name(&self) -> &'static str {
        return match self {
            MenuHomeItem::Visit => "VISIT",
            MenuHomeItem::Settings => "SETTINGS",
            MenuHomeItem::Quit => "QUIT",
        };
    }

    pub fn on_select(&self, event_loop: &ActiveEventLoop, status: &mut StatusBuffer) {
        match self {
            MenuHomeItem::Visit => status.set(Status::MenuVisit),
            MenuHomeItem::Settings => status.set(Status::MenuSettings),
            MenuHomeItem::Quit => event_loop.exit(),
        }
    }
}
