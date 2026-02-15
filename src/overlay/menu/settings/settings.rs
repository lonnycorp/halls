use glam::Vec2;
use winit::keyboard::{Key, NamedKey};

use strum::{EnumCount, IntoEnumIterator};

use crate::audio::Track;
use crate::graphics::model::ModelVertex;
use crate::graphics::sprite::{SpriteBorder, TEXT_SIZE};
use crate::window::{WindowContext, WindowKeyState};
use crate::{Status, StatusBuffer};

use crate::config::Config;

use super::item::{
    MenuSettingsItem, MenuSettingsItemOnSelectParams, MenuSettingsItemUpdateParams,
    MAX_ITEM_NAME_LEN, MAX_ITEM_VALUE_LEN,
};
use super::key::MenuSettingsKeyCache;

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
    pub buffered_config: Config,
    pub default_url: String,
    pub tick: u32,
}

impl MenuSettingsState {
    pub fn clear(&mut self, config: &Config) {
        self.buffered_config = config.clone();
        self.default_url = config.default_url.to_string();
        self.hovered = 0;
        self.selected = false;
        self.tick = 0;
    }
}

pub struct MenuSettingsUpdateParams<'a> {
    pub buffer: &'a mut Vec<ModelVertex>,
    pub resolution: Vec2,
    pub window: &'a WindowContext<'a>,
    pub status: &'a mut StatusBuffer,
    pub config: &'a mut Config,
    pub select_track: &'a Track,
    pub move_track: &'a Track,
}

pub struct MenuSettings {
    state: MenuSettingsState,
    key_cache: MenuSettingsKeyCache,
}

impl MenuSettings {
    pub fn new(config: &Config) -> Self {
        return Self {
            state: MenuSettingsState {
                hovered: 0,
                selected: false,
                buffered_config: config.clone(),
                default_url: config.default_url.to_string(),
                tick: 0,
            },
            key_cache: MenuSettingsKeyCache::new(),
        };
    }

    pub fn update(&mut self, params: &mut MenuSettingsUpdateParams) {
        if !matches!(params.status.get(), Status::MenuSettings) {
            return;
        }

        self.state.tick += 1;

        let items: Vec<MenuSettingsItem> = MenuSettingsItem::iter().collect();

        if self.state.selected {
            items[self.state.hovered].update(&mut MenuSettingsItemUpdateParams {
                state: &mut self.state,
                window: params.window,
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
                self.state.clear(params.config);
                params.status.set(Status::MenuHome);
            } else if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Enter))
            {
                items[self.state.hovered].on_select(&mut MenuSettingsItemOnSelectParams {
                    state: &mut self.state,
                    config: params.config,
                    status: params.status,
                    select_track: params.select_track,
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
                item.vertices(
                    &self.state,
                    &mut self.key_cache,
                    Vec2::new(content_x, y),
                    hovered,
                    active,
                )
                .map(|vertex| vertex.to_model_vertex(params.resolution)),
            );
        }
    }
}
