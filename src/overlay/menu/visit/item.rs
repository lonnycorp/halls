use glam::Vec2;
use url::Url;
use winit::keyboard::{Key, NamedKey};

use crate::audio::Track;
use crate::graphics::sprite::{
    OptionState, SpriteTextInput, SpriteTextOption, SpriteVertex, TEXT_SIZE,
};
use crate::level::cache::LevelCache;
use crate::level::cache::LevelCacheResult;
use crate::player::Player;
use crate::window::{WindowContext, WindowKeyState};
use crate::{Status, StatusBuffer};

use super::visit::MenuVisitState;

const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

pub const MAX_ITEM_NAME_LEN: usize = 14;
pub const MAX_ITEM_VALUE_LEN: usize = 48;

pub struct MenuVisitItemOnSelectParams<'a> {
    pub state: &'a mut MenuVisitState,
    pub status: &'a mut StatusBuffer,
}

pub struct MenuVisitItemUpdateParams<'a> {
    pub state: &'a mut MenuVisitState,
    pub window: &'a WindowContext<'a>,
    pub status: &'a mut StatusBuffer,
    pub player: &'a mut Player,
    pub cache: &'a mut LevelCache,
    pub move_track: &'a Track,
}

#[derive(strum::EnumIter, strum::EnumCount)]
pub enum MenuVisitItem {
    LevelUrl,
    Visit,
    GoBack,
}

impl MenuVisitItem {
    pub fn name(&self) -> &str {
        return match self {
            MenuVisitItem::LevelUrl => "LEVEL URL",
            MenuVisitItem::Visit => "VISIT",
            MenuVisitItem::GoBack => "BACK",
        };
    }

    pub fn on_select(&self, params: &mut MenuVisitItemOnSelectParams<'_>) {
        match self {
            MenuVisitItem::LevelUrl => {
                params.state.selected = true;
            }
            MenuVisitItem::Visit => {
                if let Ok(url) = Url::parse(&params.state.level_url) {
                    params.state.visiting = Some(url);
                    params.state.selected = true;
                }
            }
            MenuVisitItem::GoBack => {
                params.state.clear();
                params.status.set(Status::MenuHome);
            }
        }
    }

    pub fn update(&self, params: &mut MenuVisitItemUpdateParams<'_>) {
        if let MenuVisitItem::Visit = self {
            if let Some(ref visiting_url) = params.state.visiting {
                if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Escape)) {
                    params.state.visiting = None;
                    params.state.selected = false;
                    params.state.status_message = None;
                    params.move_track.reset();
                    params.move_track.play();
                    return;
                }
                match params.cache.get(visiting_url) {
                    LevelCacheResult::Loading => {
                        params.state.status_message = Some("Loading...".to_string());
                    }
                    LevelCacheResult::Ready(level) => {
                        params.player.set_position(level.spawn_position());
                        params.player.set_level_url(visiting_url.clone());
                        params.state.clear();
                        params.status.set(Status::Simulation);
                    }
                    LevelCacheResult::Failed(err) => {
                        params.state.status_message = Some(err.to_string());
                    }
                }
            } else {
                params.state.selected = false;
            }
            return;
        }
        if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Escape)) {
            params.state.selected = false;
            params.move_track.reset();
            params.move_track.play();
            return;
        }
        if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Enter)) {
            params.state.selected = false;
            params.move_track.reset();
            params.move_track.play();
            return;
        }
        match self {
            MenuVisitItem::LevelUrl => {
                if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Backspace))
                {
                    params.state.level_url.pop();
                }
                params.state.level_url.push_str(params.window.typed_chars());
            }
            MenuVisitItem::Visit | MenuVisitItem::GoBack => {}
        }
    }

    pub fn vertices<'a>(
        &'a self,
        state: &'a MenuVisitState,
        position: Vec2,
        hovered: bool,
        active: bool,
    ) -> impl Iterator<Item = SpriteVertex> + 'a {
        let option_state = match self {
            MenuVisitItem::Visit => {
                if Url::parse(&state.level_url).is_err() {
                    OptionState::Disabled
                } else if active {
                    OptionState::Selected
                } else {
                    OptionState::Unselected
                }
            }
            _ => {
                if active {
                    OptionState::Selected
                } else {
                    OptionState::Unselected
                }
            }
        };

        let option = SpriteTextOption::new(
            position,
            MAX_ITEM_NAME_LEN,
            hovered,
            option_state,
            self.name(),
        );

        let input = matches!(self, MenuVisitItem::LevelUrl).then_some(SpriteTextInput::new(
            Vec2::new(
                position.x + ITEM_INDENT + MAX_ITEM_NAME_LEN as f32 * TEXT_SIZE.x,
                position.y,
            ),
            MAX_ITEM_VALUE_LEN,
            &state.level_url,
            active,
            state.tick,
        ));
        let input_vertices = input.into_iter().flat_map(move |input| {
            return input.vertices();
        });

        return option.vertices().chain(input_vertices);
    }
}
