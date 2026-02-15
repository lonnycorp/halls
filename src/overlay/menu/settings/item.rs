use glam::Vec2;
use url::Url;
use winit::keyboard::{Key, NamedKey};

use super::key::MenuSettingsKeyCache;
use super::settings::MenuSettingsState;
use crate::audio::Track;
use crate::config::{Config, ConfigControl};
use crate::graphics::sprite::{
    OptionState, SpriteLabel, SpriteLabelAlignment, SpriteText, SpriteTextInput, SpriteTextOption,
    SpriteVertex, TextColor, TEXT_SIZE,
};
use crate::window::{WindowContext, WindowKeyState};
use crate::{Status, StatusBuffer};

pub const MAX_ITEM_NAME_LEN: usize = 14;
pub const MAX_ITEM_VALUE_LEN: usize = 48;

const ADJUST_STEP: f32 = 0.1;
const ITEM_INDENT: f32 = TEXT_SIZE.x + 2.0;

pub struct MenuSettingsItemOnSelectParams<'a> {
    pub state: &'a mut MenuSettingsState,
    pub config: &'a mut Config,
    pub status: &'a mut StatusBuffer,
    pub select_track: &'a Track,
}

pub struct MenuSettingsItemUpdateParams<'a> {
    pub state: &'a mut MenuSettingsState,
    pub window: &'a WindowContext<'a>,
    pub move_track: &'a Track,
}

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

fn slider_adjust(value: &mut f32, window: &WindowContext<'_>, move_track: &Track) {
    let mut next = *value;

    if let WindowKeyState::Pressed = window.key(&Key::Named(NamedKey::ArrowLeft)) {
        next = (next - ADJUST_STEP).clamp(0.0, 1.0);
        move_track.reset();
        move_track.play();
    }
    if let WindowKeyState::Pressed = window.key(&Key::Named(NamedKey::ArrowRight)) {
        next = (next + ADJUST_STEP).clamp(0.0, 1.0);
        move_track.reset();
        move_track.play();
    }

    *value = next;
}

fn pct_vertices(
    position: Vec2,
    value: f32,
    color: TextColor,
) -> impl Iterator<Item = SpriteVertex> {
    let pct = format!("{}%", (value * 100.0).round() as u32);
    let width = pct.len() as f32 * TEXT_SIZE.x;
    let x = position.x + MAX_ITEM_VALUE_LEN as f32 * TEXT_SIZE.x - width;
    return pct
        .into_bytes()
        .into_iter()
        .enumerate()
        .flat_map(move |(j, byte)| {
            let pos = Vec2::new(x + j as f32 * TEXT_SIZE.x, position.y);
            return SpriteText::new(char::from(byte), false, pos, color).vertices();
        });
}

impl MenuSettingsItem {
    fn control(&self) -> Option<ConfigControl> {
        return match self {
            MenuSettingsItem::Forward => Some(ConfigControl::Forward),
            MenuSettingsItem::Back => Some(ConfigControl::Back),
            MenuSettingsItem::StrafeLeft => Some(ConfigControl::StrafeLeft),
            MenuSettingsItem::StrafeRight => Some(ConfigControl::StrafeRight),
            MenuSettingsItem::Jump => Some(ConfigControl::Jump),
            MenuSettingsItem::Crouch => Some(ConfigControl::Crouch),
            _ => None,
        };
    }

    pub fn name(&self) -> &'static str {
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

    pub fn on_select(&self, params: &mut MenuSettingsItemOnSelectParams<'_>) {
        params.select_track.reset();
        params.select_track.play();

        match self {
            MenuSettingsItem::Save => {
                if Url::parse(&params.state.default_url).is_ok() {
                    params.state.buffered_config.default_url =
                        Url::parse(&params.state.default_url).unwrap();
                    *params.config = params.state.buffered_config.clone();
                    params.config.save();
                    params.state.clear(params.config);
                    params.status.set(Status::MenuHome);
                }
            }
            MenuSettingsItem::GoBack => {
                params.state.clear(params.config);
                params.status.set(Status::MenuHome);
            }
            _ => {
                params.state.selected = true;
            }
        }
    }

    pub fn update(&self, params: &mut MenuSettingsItemUpdateParams<'_>) {
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
            MenuSettingsItem::Volume => {
                slider_adjust(
                    &mut params.state.buffered_config.volume,
                    params.window,
                    params.move_track,
                );
            }
            MenuSettingsItem::MouseSensitivity => {
                slider_adjust(
                    &mut params.state.buffered_config.mouse_sensitivity,
                    params.window,
                    params.move_track,
                );
            }
            MenuSettingsItem::Forward
            | MenuSettingsItem::Back
            | MenuSettingsItem::StrafeLeft
            | MenuSettingsItem::StrafeRight
            | MenuSettingsItem::Jump
            | MenuSettingsItem::Crouch => {
                let control = self.control().unwrap();
                if let Some(key) = params.window.last_pressed() {
                    params.state.buffered_config.key_set(control, key);
                    params.state.selected = false;
                    params.move_track.reset();
                    params.move_track.play();
                }
            }
            MenuSettingsItem::DefaultUrl => {
                if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Backspace))
                {
                    params.state.default_url.pop();
                }
                params
                    .state
                    .default_url
                    .push_str(params.window.typed_chars());
            }
            MenuSettingsItem::Save | MenuSettingsItem::GoBack => {}
        }
    }

    pub fn vertices<'a>(
        &self,
        state: &'a MenuSettingsState,
        key_cache: &'a mut MenuSettingsKeyCache,
        position: Vec2,
        hovered: bool,
        active: bool,
    ) -> impl Iterator<Item = SpriteVertex> + 'a {
        let option_state =
            if matches!(self, MenuSettingsItem::Save) && Url::parse(&state.default_url).is_err() {
                OptionState::Disabled
            } else if active {
                OptionState::Selected
            } else {
                OptionState::Unselected
            };

        let option = SpriteTextOption::new(
            position,
            MAX_ITEM_NAME_LEN,
            hovered,
            option_state,
            self.name(),
        );

        let value_x = position.x + ITEM_INDENT + MAX_ITEM_NAME_LEN as f32 * TEXT_SIZE.x;
        let value_y = position.y;

        let pct_value = match self {
            MenuSettingsItem::Volume => Some(state.buffered_config.volume),
            MenuSettingsItem::MouseSensitivity => Some(state.buffered_config.mouse_sensitivity),
            _ => None,
        };
        let pct_value_vertices = pct_value.into_iter().flat_map(move |value| {
            return pct_vertices(Vec2::new(value_x, value_y), value, TextColor::White);
        });

        let key_name = match self {
            MenuSettingsItem::Forward
            | MenuSettingsItem::Back
            | MenuSettingsItem::StrafeLeft
            | MenuSettingsItem::StrafeRight
            | MenuSettingsItem::Jump
            | MenuSettingsItem::Crouch => {
                let control = self.control().unwrap();
                Some(key_cache.name(state.buffered_config.key_get(control)))
            }
            _ => None,
        };
        let key_vertices = key_name.into_iter().flat_map(move |name| {
            let position = Vec2::new(value_x, value_y);
            return SpriteLabel::new(
                position,
                MAX_ITEM_VALUE_LEN,
                TextColor::White,
                false,
                SpriteLabelAlignment::Right,
                name,
            )
            .vertices();
        });

        let input = matches!(self, MenuSettingsItem::DefaultUrl).then_some(SpriteTextInput::new(
            Vec2::new(value_x, value_y),
            MAX_ITEM_VALUE_LEN,
            &state.default_url,
            active,
            state.tick,
        ));
        let input_vertices = input.into_iter().flat_map(move |input| {
            return input.vertices();
        });

        return option
            .vertices()
            .chain(pct_value_vertices)
            .chain(key_vertices)
            .chain(input_vertices);
    }
}
