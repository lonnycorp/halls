use glam::Vec2;
use strum::{EnumCount, EnumIter};

use crate::audio::Track;
use crate::graphics::sprite::{OptionState, SpriteTextOption, SpriteVertex};
use crate::window::WindowContext;
use crate::{Status, StatusBuffer};

pub struct MenuHomeItemOnSelectParams<'a> {
    pub window: &'a WindowContext<'a>,
    pub status: &'a mut StatusBuffer,
    pub select_track: &'a Track,
}

pub const MAX_ITEM_LEN: usize = 12;

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

    pub fn on_select(&self, params: &mut MenuHomeItemOnSelectParams) {
        params.select_track.reset();
        params.select_track.play();

        match self {
            MenuHomeItem::Visit => params.status.set(Status::MenuVisit),
            MenuHomeItem::Settings => params.status.set(Status::MenuSettings),
            MenuHomeItem::Quit => params.window.exit(),
        }
    }

    pub fn vertices(
        &self,
        position: Vec2,
        hovered: bool,
    ) -> impl Iterator<Item = SpriteVertex> + '_ {
        return SpriteTextOption::new(
            position,
            MAX_ITEM_LEN,
            hovered,
            OptionState::Unselected,
            self.name(),
        )
        .vertices();
    }
}
