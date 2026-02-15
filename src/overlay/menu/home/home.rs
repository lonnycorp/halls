use glam::Vec2;
use strum::{EnumCount, IntoEnumIterator};
use winit::keyboard::{Key, NamedKey};

use crate::audio::Track;
use crate::graphics::model::ModelVertex;
use crate::graphics::sprite::{SpriteBorder, TEXT_SIZE};
use crate::window::{WindowContext, WindowKeyState};
use crate::{Status, StatusBuffer};

use super::item::{MenuHomeItem, MenuHomeItemOnSelectParams, MAX_ITEM_LEN};

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const INSET: f32 = BORDER + TEXT_PADDING;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

const ROW_WIDTH: f32 = ITEM_INDENT + MAX_ITEM_LEN as f32 * TEXT_SIZE.x;
const BOX_WIDTH: f32 = ROW_WIDTH + INSET * 2.0;
const BOX_HEIGHT: f32 = MenuHomeItem::COUNT as f32 * TEXT_SIZE.y + INSET * 2.0;

pub struct MenuHomeUpdateParams<'a> {
    pub buffer: &'a mut Vec<ModelVertex>,
    pub resolution: Vec2,
    pub window: &'a WindowContext<'a>,
    pub status: &'a mut StatusBuffer,
    pub select_track: &'a Track,
    pub move_track: &'a Track,
}

pub struct MenuHome {
    selected: usize,
}

impl MenuHome {
    pub fn new() -> Self {
        return Self { selected: 0 };
    }

    pub fn update(&mut self, params: &mut MenuHomeUpdateParams) {
        if !matches!(params.status.get(), Status::MenuHome) {
            return;
        }

        if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::ArrowUp)) {
            self.selected = (self.selected + MenuHomeItem::COUNT - 1) % MenuHomeItem::COUNT;
            params.move_track.reset();
            params.move_track.play();
        }
        if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::ArrowDown)) {
            self.selected = (self.selected + 1) % MenuHomeItem::COUNT;
            params.move_track.reset();
            params.move_track.play();
        }
        if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Escape)) {
            params.move_track.reset();
            params.move_track.play();
            params.status.set(Status::Simulation);
        } else if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Enter)) {
            if let Some(item) = MenuHomeItem::iter().nth(self.selected) {
                item.on_select(&mut MenuHomeItemOnSelectParams {
                    window: params.window,
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

        for (i, item) in MenuHomeItem::iter().enumerate() {
            let y = content_y + i as f32 * TEXT_SIZE.y;
            let hovered = i == self.selected;
            params.buffer.extend(
                item.vertices(Vec2::new(content_x, y), hovered)
                    .map(|vertex| vertex.to_model_vertex(params.resolution)),
            );
        }
    }
}
