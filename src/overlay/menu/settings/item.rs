use glam::Vec2;
use url::Url;
use winit::keyboard::KeyCode;

use super::settings::MenuSettingsState;
use crate::audio::Effect;
use crate::config::{Config, ConfigControl};
use crate::graphics::color::Color;
use crate::graphics::model::ModelBuffer;
use crate::graphics::sprite::{
    OptionState, SpriteText, SpriteTextInput, SpriteTextOption, TEXT_SIZE,
};
use crate::window::{InputController, KeyState};
use crate::{Status, StatusBuffer};

pub const MAX_ITEM_NAME_LEN: usize = 14;
pub const MAX_ITEM_VALUE_LEN: usize = 48;

const ADJUST_STEP: f32 = 0.1;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

#[derive(strum::EnumIter, strum::EnumCount)]
pub enum MenuSettingsItem {
    Volume,
    MouseSensitivity,
    Forward,
    Back,
    StrafeLeft,
    StrafeRight,
    Jump,
    Crouch,
    DefaultUrl,
    Save,
    GoBack,
}

impl MenuSettingsItem {
    pub fn name(&self) -> &str {
        return match self {
            MenuSettingsItem::Volume => "VOLUME",
            MenuSettingsItem::MouseSensitivity => "MOUSE SENS",
            MenuSettingsItem::Forward => "FORWARDS",
            MenuSettingsItem::Back => "BACKWARDS",
            MenuSettingsItem::StrafeLeft => "STRAFE LEFT",
            MenuSettingsItem::StrafeRight => "STRAFE RIGHT",
            MenuSettingsItem::Jump => "JUMP",
            MenuSettingsItem::Crouch => "CROUCH",
            MenuSettingsItem::DefaultUrl => "DEFAULT URL",
            MenuSettingsItem::Save => "SAVE",
            MenuSettingsItem::GoBack => "BACK",
        };
    }

    pub fn on_select(
        &self,
        state: &mut MenuSettingsState,
        config: &mut Config,
        status: &mut StatusBuffer,
    ) {
        match self {
            MenuSettingsItem::Save => {
                if Url::parse(&state.default_url).is_ok() {
                    state.buffered_state.default_url = Url::parse(&state.default_url).unwrap();
                    *config = state.buffered_state.clone();
                    config.save();
                    state.clear(config);
                    status.set(Status::MenuHome);
                }
            }
            MenuSettingsItem::GoBack => {
                state.clear(config);
                status.set(Status::MenuHome);
            }
            _ => {
                state.selected = true;
            }
        }
    }

    pub fn update(
        &self,
        state: &mut MenuSettingsState,
        input: &InputController<'_>,
        move_effect: &Effect,
    ) {
        if let KeyState::Pressed = input.key(KeyCode::Escape) {
            state.selected = false;
            move_effect.reset();
            move_effect.play();
            return;
        }
        if let KeyState::Pressed = input.key(KeyCode::Enter) {
            state.selected = false;
            move_effect.reset();
            move_effect.play();
            return;
        }
        match self {
            MenuSettingsItem::Volume => {
                adjust_slider(&mut state.buffered_state.volume, input, move_effect);
            }
            MenuSettingsItem::MouseSensitivity => {
                adjust_slider(
                    &mut state.buffered_state.mouse_sensitivity,
                    input,
                    move_effect,
                );
            }
            MenuSettingsItem::Forward
            | MenuSettingsItem::Back
            | MenuSettingsItem::StrafeLeft
            | MenuSettingsItem::StrafeRight
            | MenuSettingsItem::Jump
            | MenuSettingsItem::Crouch => {
                let control = self.as_control();
                if let Some(key) = input.last_pressed() {
                    state.buffered_state.keycode_set(control, key);
                    state.selected = false;
                    move_effect.reset();
                    move_effect.play();
                }
            }
            MenuSettingsItem::DefaultUrl => {
                if let KeyState::Pressed = input.key(KeyCode::Backspace) {
                    state.default_url.pop();
                }
                state.default_url.push_str(input.typed_chars());
            }
            MenuSettingsItem::Save | MenuSettingsItem::GoBack => {}
        }
    }

    pub fn write_to_model_buffer(
        &self,
        state: &MenuSettingsState,
        buffer: &mut ModelBuffer,
        resolution: Vec2,
        position: Vec2,
        hovered: bool,
        active: bool,
    ) {
        let option_state = match self {
            MenuSettingsItem::Save => {
                if Url::parse(&state.default_url).is_err() {
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

        let value_x = position.x + ITEM_INDENT + MAX_ITEM_NAME_LEN as f32 * TEXT_SIZE.x;

        match self {
            MenuSettingsItem::Volume => {
                draw_pct(
                    buffer,
                    resolution,
                    Vec2::new(value_x, position.y),
                    state.buffered_state.volume,
                    Color::WHITE,
                );
            }
            MenuSettingsItem::MouseSensitivity => {
                draw_pct(
                    buffer,
                    resolution,
                    Vec2::new(value_x, position.y),
                    state.buffered_state.mouse_sensitivity,
                    Color::WHITE,
                );
            }
            MenuSettingsItem::Forward
            | MenuSettingsItem::Back
            | MenuSettingsItem::StrafeLeft
            | MenuSettingsItem::StrafeRight
            | MenuSettingsItem::Jump
            | MenuSettingsItem::Crouch => {
                let control = self.as_control();
                let key = state.buffered_state.keycode_get(control);
                let name = format!("{:?}", key);
                let x = value_x + MAX_ITEM_VALUE_LEN as f32 * TEXT_SIZE.x
                    - name.len() as f32 * TEXT_SIZE.x;
                for (j, c) in name.chars().enumerate() {
                    let pos = Vec2::new(x + j as f32 * TEXT_SIZE.x, position.y);
                    SpriteText::new(c, false, pos, Color::WHITE)
                        .write_to_model_buffer(buffer, resolution);
                }
            }
            MenuSettingsItem::DefaultUrl => {
                SpriteTextInput::new(
                    Vec2::new(value_x, position.y),
                    MAX_ITEM_VALUE_LEN,
                    &state.default_url,
                    active,
                    state.tick,
                )
                .write_to_model_buffer(buffer, resolution);
            }
            MenuSettingsItem::Save | MenuSettingsItem::GoBack => {}
        }
    }

    fn as_control(&self) -> ConfigControl {
        return match self {
            MenuSettingsItem::Forward => ConfigControl::Forward,
            MenuSettingsItem::Back => ConfigControl::Back,
            MenuSettingsItem::StrafeLeft => ConfigControl::StrafeLeft,
            MenuSettingsItem::StrafeRight => ConfigControl::StrafeRight,
            MenuSettingsItem::Jump => ConfigControl::Jump,
            MenuSettingsItem::Crouch => ConfigControl::Crouch,
            _ => unreachable!(),
        };
    }
}

fn adjust_slider(value: &mut f32, input: &InputController<'_>, move_effect: &Effect) {
    let mut next = *value;

    if let KeyState::Pressed = input.key(KeyCode::ArrowLeft) {
        next = (next - ADJUST_STEP).clamp(0.0, 1.0);
        move_effect.reset();
        move_effect.play();
    }
    if let KeyState::Pressed = input.key(KeyCode::ArrowRight) {
        next = (next + ADJUST_STEP).clamp(0.0, 1.0);
        move_effect.reset();
        move_effect.play();
    }

    *value = next;
}

fn draw_pct(buffer: &mut ModelBuffer, resolution: Vec2, position: Vec2, value: f32, color: Color) {
    let pct = format!("{}%", (value * 100.0).round() as u32);
    let x = position.x + MAX_ITEM_VALUE_LEN as f32 * TEXT_SIZE.x - pct.len() as f32 * TEXT_SIZE.x;
    for (j, c) in pct.chars().enumerate() {
        let pos = Vec2::new(x + j as f32 * TEXT_SIZE.x, position.y);
        SpriteText::new(c, false, pos, color).write_to_model_buffer(buffer, resolution);
    }
}
