use glam::Vec2;
use url::Url;
use winit::keyboard::KeyCode;

use strum::{EnumCount, IntoEnumIterator};

use crate::audio::Effect;
use crate::config::Config;
use crate::graphics::model::ModelBuffer;
use crate::graphics::sprite::{SpriteBorder, SpriteLabel, TEXT_SIZE};
use crate::level::cache::LevelCache;
use crate::player::Player;
use crate::window::{InputController, KeyState};
use crate::{Status, StatusBuffer};

use super::item::{MenuVisitItem, MAX_ITEM_NAME_LEN, MAX_ITEM_VALUE_LEN};

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const INSET: f32 = BORDER + TEXT_PADDING;
const ITEM_COUNT: usize = MenuVisitItem::COUNT;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;
const ROW_WIDTH: f32 = ITEM_INDENT + (MAX_ITEM_NAME_LEN + MAX_ITEM_VALUE_LEN) as f32 * TEXT_SIZE.x;
const BOX_WIDTH: f32 = ROW_WIDTH + INSET * 2.0;
const BOX_HEIGHT: f32 = ITEM_COUNT as f32 * TEXT_SIZE.y + INSET * 2.0;
const STATUS_MAX_CHARS: usize = ((BOX_WIDTH - INSET * 2.0) / TEXT_SIZE.x) as usize;

const WHITE: [u8; 4] = [255, 255, 255, 255];

pub struct MenuVisitState {
    pub hovered: usize,
    pub selected: bool,
    pub visiting: Option<Url>,
    pub level_url: String,
    pub tick: u32,
    pub status_message: Option<String>,
}

impl MenuVisitState {
    pub fn clear(&mut self) {
        self.hovered = 0;
        self.selected = false;
        self.visiting = None;
        self.tick = 0;
        self.status_message = None;
    }
}

pub struct MenuVisitUpdateContext<'a> {
    pub buffer: &'a mut ModelBuffer,
    pub resolution: Vec2,
    pub input: &'a InputController<'a>,
    pub status: &'a mut StatusBuffer,
    pub player: &'a mut Player,
    pub cache: &'a mut LevelCache,
    pub select_effect: &'a Effect,
    pub move_effect: &'a Effect,
}

pub struct MenuVisit {
    state: MenuVisitState,
}

impl MenuVisit {
    pub fn new(config: &Config) -> Self {
        return Self {
            state: MenuVisitState {
                hovered: 0,
                selected: false,
                visiting: None,
                level_url: config.default_url.to_string(),
                tick: 0,
                status_message: None,
            },
        };
    }

    pub fn update(&mut self, ctx: &mut MenuVisitUpdateContext) {
        if *ctx.status.get() != Status::MenuVisit {
            return;
        }

        self.state.tick += 1;

        let items: Vec<MenuVisitItem> = MenuVisitItem::iter().collect();

        if self.state.selected {
            items[self.state.hovered].update(&mut self.state, ctx);
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
                self.state.clear();
                ctx.status.set(Status::MenuHome);
            } else if let KeyState::Pressed = ctx.input.key(KeyCode::Enter) {
                ctx.select_effect.reset();
                ctx.select_effect.play();
                items[self.state.hovered].on_select(&mut self.state, ctx.status);
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

        if let Some(ref message) = self.state.status_message {
            let status_y = SCREEN_PADDING + BOX_HEIGHT + SCREEN_PADDING;
            let status_height = TEXT_SIZE.y + INSET * 2.0;
            SpriteBorder::new(
                Vec2::new(SCREEN_PADDING, status_y),
                Vec2::new(BOX_WIDTH, status_height),
            )
            .write_to_model_buffer(ctx.buffer, ctx.resolution);

            let text_pos = Vec2::new(SCREEN_PADDING + INSET, status_y + INSET);
            SpriteLabel::new(text_pos, STATUS_MAX_CHARS, WHITE, false, message)
                .write_to_model_buffer(ctx.buffer, ctx.resolution);
        }
    }
}
