use glam::Vec2;
use url::Url;
use winit::keyboard::{Key, NamedKey};

use strum::{EnumCount, IntoEnumIterator};

use crate::audio::Track;
use crate::config::Config;
use crate::graphics::model::ModelVertex;
use crate::graphics::sprite::{
    SpriteBorder, SpriteLabel, SpriteLabelAlignment, TextColor, TEXT_SIZE,
};
use crate::level::cache::LevelCache;
use crate::player::Player;
use crate::window::{WindowContext, WindowKeyState};
use crate::{Status, StatusBuffer};

use super::item::{
    MenuVisitItem, MenuVisitItemOnSelectParams, MenuVisitItemUpdateParams, MAX_ITEM_NAME_LEN,
    MAX_ITEM_VALUE_LEN,
};

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

const WHITE: TextColor = TextColor::White;

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

pub struct MenuVisitUpdateParams<'a> {
    pub buffer: &'a mut Vec<ModelVertex>,
    pub resolution: Vec2,
    pub window: &'a WindowContext<'a>,
    pub status: &'a mut StatusBuffer,
    pub player: &'a mut Player,
    pub cache: &'a mut LevelCache,
    pub select_track: &'a Track,
    pub move_track: &'a Track,
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

    pub fn update(&mut self, params: &mut MenuVisitUpdateParams) {
        if !matches!(params.status.get(), Status::MenuVisit) {
            return;
        }

        self.state.tick += 1;

        let items: Vec<MenuVisitItem> = MenuVisitItem::iter().collect();

        if self.state.selected {
            items[self.state.hovered].update(&mut MenuVisitItemUpdateParams {
                state: &mut self.state,
                window: params.window,
                status: params.status,
                player: params.player,
                cache: params.cache,
                move_track: params.move_track,
            });
        } else {
            if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::ArrowUp)) {
                self.state.hovered = (self.state.hovered + ITEM_COUNT - 1) % ITEM_COUNT;
                params.move_track.reset();
                params.move_track.play();
            } else if let WindowKeyState::Pressed =
                params.window.key(&Key::Named(NamedKey::ArrowDown))
            {
                self.state.hovered = (self.state.hovered + 1) % ITEM_COUNT;
                params.move_track.reset();
                params.move_track.play();
            }

            if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Escape)) {
                params.move_track.reset();
                params.move_track.play();
                self.state.clear();
                params.status.set(Status::MenuHome);
            } else if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Enter))
            {
                params.select_track.reset();
                params.select_track.play();
                items[self.state.hovered].on_select(&mut MenuVisitItemOnSelectParams {
                    state: &mut self.state,
                    status: params.status,
                });
            }
        }

        let box_pos = Vec2::new(SCREEN_PADDING, SCREEN_PADDING);
        params.buffer.extend(
            SpriteBorder::new(box_pos, Vec2::new(BOX_WIDTH, BOX_HEIGHT))
                .vertices()
                .map(|vertex| vertex.to_model_vertex(params.resolution)),
        );

        let content_x = box_pos.x + INSET;
        let content_y = box_pos.y + INSET;

        for (i, item) in items.iter().enumerate() {
            let y = content_y + i as f32 * TEXT_SIZE.y;
            let hovered = i == self.state.hovered;
            let active = hovered && self.state.selected;
            params.buffer.extend(
                item.vertices(&self.state, Vec2::new(content_x, y), hovered, active)
                    .map(|vertex| vertex.to_model_vertex(params.resolution)),
            );
        }

        if let Some(ref message) = self.state.status_message {
            let status_y = SCREEN_PADDING + BOX_HEIGHT + SCREEN_PADDING;
            let status_height = TEXT_SIZE.y + INSET * 2.0;
            params.buffer.extend(
                SpriteBorder::new(
                    Vec2::new(SCREEN_PADDING, status_y),
                    Vec2::new(BOX_WIDTH, status_height),
                )
                .vertices()
                .map(|vertex| vertex.to_model_vertex(params.resolution)),
            );

            let text_pos = Vec2::new(SCREEN_PADDING + INSET, status_y + INSET);
            params.buffer.extend(
                SpriteLabel::new(
                    text_pos,
                    STATUS_MAX_CHARS,
                    WHITE,
                    false,
                    SpriteLabelAlignment::Left,
                    message,
                )
                .vertices()
                .map(|vertex| vertex.to_model_vertex(params.resolution)),
            );
        }
    }
}
