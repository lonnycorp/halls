use glam::Vec2;
use strum::{EnumCount, IntoEnumIterator};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

use crate::audio::Effect;
use crate::graphics::model::ModelBuffer;
use crate::graphics::sprite::{OptionState, SpriteBorder, SpriteTextOption, TEXT_SIZE};
use crate::window::{InputController, KeyState};
use crate::{Status, StatusBuffer};

use super::item::MenuHomeItem;

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const INSET: f32 = BORDER + TEXT_PADDING;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

const MAX_ITEM_LEN: usize = 12;
const ROW_WIDTH: f32 = ITEM_INDENT + MAX_ITEM_LEN as f32 * TEXT_SIZE.x;
const BOX_WIDTH: f32 = ROW_WIDTH + INSET * 2.0;
const BOX_HEIGHT: f32 = MenuHomeItem::COUNT as f32 * TEXT_SIZE.y + INSET * 2.0;

pub struct MenuHomeUpdateContext<'a> {
    pub event_loop: &'a ActiveEventLoop,
    pub buffer: &'a mut ModelBuffer,
    pub resolution: Vec2,
    pub input: &'a InputController<'a>,
    pub status: &'a mut StatusBuffer,
    pub select_effect: &'a Effect,
    pub move_effect: &'a Effect,
}

pub struct MenuHome {
    selected: usize,
}

impl MenuHome {
    pub fn new() -> Self {
        return Self { selected: 0 };
    }

    pub fn update(&mut self, ctx: &mut MenuHomeUpdateContext) {
        if !matches!(ctx.status.get(), Status::MenuHome) {
            return;
        }

        if let KeyState::Pressed = ctx.input.key(KeyCode::ArrowUp) {
            self.selected = (self.selected + MenuHomeItem::COUNT - 1) % MenuHomeItem::COUNT;
            ctx.move_effect.reset();
            ctx.move_effect.play();
        }
        if let KeyState::Pressed = ctx.input.key(KeyCode::ArrowDown) {
            self.selected = (self.selected + 1) % MenuHomeItem::COUNT;
            ctx.move_effect.reset();
            ctx.move_effect.play();
        }
        if let KeyState::Pressed = ctx.input.key(KeyCode::Escape) {
            ctx.move_effect.reset();
            ctx.move_effect.play();
            ctx.status.set(Status::Simulation);
        } else if let KeyState::Pressed = ctx.input.key(KeyCode::Enter) {
            ctx.select_effect.reset();
            ctx.select_effect.play();
            if let Some(item) = MenuHomeItem::iter().nth(self.selected) {
                item.on_select(ctx.event_loop, ctx.status);
            }
        }

        let box_pos = Vec2::new(SCREEN_PADDING, SCREEN_PADDING);
        SpriteBorder::new(box_pos, Vec2::new(BOX_WIDTH, BOX_HEIGHT))
            .write_to_model_buffer(ctx.buffer, ctx.resolution);

        let content_x = box_pos.x + INSET;
        let content_y = box_pos.y + INSET;

        for (i, item) in MenuHomeItem::iter().enumerate() {
            let y = content_y + i as f32 * TEXT_SIZE.y;
            let hovered = i == self.selected;
            SpriteTextOption::new(
                Vec2::new(content_x, y),
                MAX_ITEM_LEN,
                hovered,
                OptionState::Unselected,
                item.name(),
            )
            .write_to_model_buffer(ctx.buffer, ctx.resolution);
        }
    }
}
