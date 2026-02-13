use glam::Vec2;
use url::Url;
use winit::keyboard::KeyCode;

use crate::graphics::model::ModelBuffer;
use crate::graphics::sprite::{OptionState, SpriteTextInput, SpriteTextOption, TEXT_SIZE};
use crate::level::cache::LevelCacheResult;
use crate::window::KeyState;
use crate::{Status, StatusBuffer};

use super::visit::{MenuVisitState, MenuVisitUpdateContext};

const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

pub const MAX_ITEM_NAME_LEN: usize = 14;
pub const MAX_ITEM_VALUE_LEN: usize = 48;

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

    pub fn on_select(&self, state: &mut MenuVisitState, status: &mut StatusBuffer) {
        match self {
            MenuVisitItem::LevelUrl => {
                state.selected = true;
            }
            MenuVisitItem::Visit => {
                if let Ok(url) = Url::parse(&state.level_url) {
                    state.visiting = Some(url);
                    state.selected = true;
                }
            }
            MenuVisitItem::GoBack => {
                state.clear();
                status.set(Status::MenuHome);
            }
        }
    }

    pub fn update(&self, state: &mut MenuVisitState, ctx: &mut MenuVisitUpdateContext) {
        if let MenuVisitItem::Visit = self {
            if let Some(ref visiting_url) = state.visiting {
                if let KeyState::Pressed = ctx.input.key(KeyCode::Escape) {
                    state.visiting = None;
                    state.selected = false;
                    state.status_message = None;
                    ctx.move_effect.reset();
                    ctx.move_effect.play();
                    return;
                }
                match ctx.cache.get(visiting_url) {
                    LevelCacheResult::Loading => {
                        state.status_message = Some("Loading...".to_string());
                    }
                    LevelCacheResult::Ready(level) => {
                        ctx.player.set_position(level.spawn_position());
                        ctx.player.set_level_url(visiting_url.clone());
                        state.clear();
                        ctx.status.set(Status::Simulation);
                    }
                    LevelCacheResult::Failed(err) => {
                        state.status_message = Some(err.to_string());
                    }
                }
            } else {
                state.selected = false;
            }
            return;
        }
        if let KeyState::Pressed = ctx.input.key(KeyCode::Escape) {
            state.selected = false;
            ctx.move_effect.reset();
            ctx.move_effect.play();
            return;
        }
        if let KeyState::Pressed = ctx.input.key(KeyCode::Enter) {
            state.selected = false;
            ctx.move_effect.reset();
            ctx.move_effect.play();
            return;
        }
        match self {
            MenuVisitItem::LevelUrl => {
                if let KeyState::Pressed = ctx.input.key(KeyCode::Backspace) {
                    state.level_url.pop();
                }
                state.level_url.push_str(ctx.input.typed_chars());
            }
            MenuVisitItem::Visit | MenuVisitItem::GoBack => {}
        }
    }

    pub fn write_to_model_buffer(
        &self,
        state: &MenuVisitState,
        buffer: &mut ModelBuffer,
        resolution: Vec2,
        position: Vec2,
        hovered: bool,
        active: bool,
    ) {
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

        SpriteTextOption::new(
            position,
            MAX_ITEM_NAME_LEN,
            hovered,
            option_state,
            self.name(),
        )
        .write_to_model_buffer(buffer, resolution);

        match self {
            MenuVisitItem::LevelUrl => {
                let value_x = position.x + ITEM_INDENT + MAX_ITEM_NAME_LEN as f32 * TEXT_SIZE.x;
                SpriteTextInput::new(
                    Vec2::new(value_x, position.y),
                    MAX_ITEM_VALUE_LEN,
                    &state.level_url,
                    active,
                    state.tick,
                )
                .write_to_model_buffer(buffer, resolution);
            }
            MenuVisitItem::Visit | MenuVisitItem::GoBack => {}
        }
    }
}
