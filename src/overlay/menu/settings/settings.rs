use glam::Vec2;
use winit::keyboard::KeyCode;

use strum::{EnumCount, IntoEnumIterator};

use crate::audio::Effect;
use crate::graphics::model::ModelBuffer;
use crate::graphics::sprite::{SpriteBorder, TEXT_SIZE};
use crate::window::{InputController, KeyState};
use crate::{Status, StatusBuffer};

use crate::config::Config;

use super::item::{MenuSettingsItem, MAX_ITEM_NAME_LEN, MAX_ITEM_VALUE_LEN};

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const INSET: f32 = BORDER + TEXT_PADDING;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

const ITEM_COUNT: usize = MenuSettingsItem::COUNT;
const ROW_WIDTH: f32 = ITEM_INDENT + (MAX_ITEM_NAME_LEN + MAX_ITEM_VALUE_LEN) as f32 * TEXT_SIZE.x;
const BOX_WIDTH: f32 = ROW_WIDTH + INSET * 2.0;
const BOX_HEIGHT: f32 = ITEM_COUNT as f32 * TEXT_SIZE.y + INSET * 2.0;

pub struct MenuSettingsState {
    pub hovered: usize,
    pub selected: bool,
    pub buffered_state: Config,
    pub default_url: String,
    pub tick: u32,
}

impl MenuSettingsState {
    pub fn clear(&mut self, config: &Config) {
        self.buffered_state = config.clone();
        self.default_url = config.default_url.to_string();
        self.hovered = 0;
        self.selected = false;
        self.tick = 0;
    }
}

pub struct MenuSettingsUpdateContext<'a> {
    pub buffer: &'a mut ModelBuffer,
    pub resolution: Vec2,
    pub input: &'a InputController<'a>,
    pub status: &'a mut StatusBuffer,
    pub config: &'a mut Config,
    pub select_effect: &'a Effect,
    pub move_effect: &'a Effect,
}

pub struct MenuSettings {
    state: MenuSettingsState,
}

impl MenuSettings {
    pub fn new(config: &Config) -> Self {
        return Self {
            state: MenuSettingsState {
                hovered: 0,
                selected: false,
                buffered_state: config.clone(),
                default_url: config.default_url.to_string(),
                tick: 0,
            },
        };
    }

    pub fn update(&mut self, ctx: &mut MenuSettingsUpdateContext) {
        if !matches!(ctx.status.get(), Status::MenuSettings) {
            return;
        }

        self.state.tick += 1;

        let items: Vec<MenuSettingsItem> = MenuSettingsItem::iter().collect();

        if self.state.selected {
            items[self.state.hovered].update(&mut self.state, ctx.input, ctx.move_effect);
        } else {
            if let KeyState::Pressed = ctx.input.key(KeyCode::ArrowUp) {
                self.state.hovered = (self.state.hovered + ITEM_COUNT - 1) % ITEM_COUNT;
                ctx.move_effect.reset();
                ctx.move_effect.play();
            } else if let KeyState::Pressed = ctx.input.key(KeyCode::ArrowDown) {
                self.state.hovered = (self.state.hovered + 1) % ITEM_COUNT;
                ctx.move_effect.reset();
                ctx.move_effect.play();
            }

            if let KeyState::Pressed = ctx.input.key(KeyCode::Escape) {
                ctx.move_effect.reset();
                ctx.move_effect.play();
                self.state.clear(ctx.config);
                ctx.status.set(Status::MenuHome);
            } else if let KeyState::Pressed = ctx.input.key(KeyCode::Enter) {
                ctx.select_effect.reset();
                ctx.select_effect.play();
                items[self.state.hovered].on_select(&mut self.state, ctx.config, ctx.status);
            }
        }

        let box_pos = Vec2::new(SCREEN_PADDING, SCREEN_PADDING);
        SpriteBorder::new(box_pos, Vec2::new(BOX_WIDTH, BOX_HEIGHT))
            .write_to_model_buffer(ctx.buffer, ctx.resolution);

        let content_x = box_pos.x + INSET;
        let content_y = box_pos.y + INSET;

        for (i, item) in items.iter().enumerate() {
            let y = content_y + i as f32 * TEXT_SIZE.y;
            let hovered = i == self.state.hovered;
            let active = hovered && self.state.selected;
            item.write_to_model_buffer(
                &self.state,
                ctx.buffer,
                ctx.resolution,
                Vec2::new(content_x, y),
                hovered,
                active,
            );
        }
    }
}
