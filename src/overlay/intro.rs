use std::f32::consts::PI;

use glam::Vec2;
use winit::keyboard::{Key, NamedKey};

use crate::audio::Track;
use crate::graphics::model::ModelVertex;
use crate::graphics::sprite::{
    SpriteLogo, SpriteMaterial, SpriteSolid, SpriteText, TextColor, TEXT_SIZE,
};
use crate::window::{WindowContext, WindowKeyState};
use crate::{Status, StatusBuffer};

const TIME_STEP: f32 = 0.002;

const INTRO_START: f32 = 0.15;

const TITLE: &[u8] = b"LONNYCORP";
const TITLE_CHAR_BOUNCE_DURATION: f32 = 0.05;
const TITLE_CHAR_STAGGER_DELAY: f32 = 0.01;
const TITLE_SPACING: f32 = 2.0;
const LOGO_UV_SIZE: f32 = 480.0;
const TITLE_BOUNCE_HEIGHT: f32 = 32.0;

fn ramp(t: f32, start: f32, end: f32) -> f32 {
    return ((t - start) / (end - start)).clamp(0.0, 1.0);
}

pub struct IntroUpdateParams<'a> {
    pub buffer: &'a mut Vec<ModelVertex>,
    pub resolution: Vec2,
    pub window: &'a WindowContext<'a>,
    pub status: &'a mut StatusBuffer,
    pub jingle_track: &'a Track,
}

pub struct Intro {
    time: f32,
    jingle_played: bool,
}

impl Intro {
    pub fn new() -> Self {
        return Self {
            time: 0.0,
            jingle_played: false,
        };
    }

    pub fn update(&mut self, params: &mut IntroUpdateParams) {
        if !matches!(params.status.get(), Status::Intro) {
            return;
        }

        if let WindowKeyState::Pressed = params.window.key(&Key::Named(NamedKey::Escape)) {
            params.status.set(Status::MenuHome);
            return;
        }

        self.time = (self.time + TIME_STEP).min(1.0);
        if self.time >= 1.0 {
            params.status.set(Status::MenuHome);
        }

        if !self.jingle_played && self.time >= INTRO_START {
            params.jingle_track.reset();
            params.jingle_track.play();
            self.jingle_played = true;
        }

        params.buffer.extend(
            SpriteSolid::new(Vec2::ZERO, params.resolution, SpriteMaterial::SystemWhite)
                .vertices()
                .map(|vertex| vertex.to_model_vertex(params.resolution)),
        );
        SpriteLogo::new(params.resolution / 2.0)
            .write_to_model_buffer(params.buffer, params.resolution);

        let title_width = ((TITLE.len() - 1) as f32 * TITLE_SPACING + 1.0) * TEXT_SIZE.x;
        let title_x = (params.resolution.x - title_width) / 2.0;
        let title_y = params.resolution.y / 2.0 + LOGO_UV_SIZE / 2.0 + TEXT_SIZE.y;

        for (i, &c) in TITLE.iter().enumerate() {
            let char_start = INTRO_START + i as f32 * TITLE_CHAR_STAGGER_DELAY;
            let char_life = ramp(
                self.time,
                char_start,
                char_start + TITLE_CHAR_BOUNCE_DURATION,
            );
            let x = title_x + i as f32 * TITLE_SPACING * TEXT_SIZE.x;
            let y_offset = (char_life * PI).sin() * TITLE_BOUNCE_HEIGHT;
            params.buffer.extend(
                SpriteText::new(
                    c as char,
                    true,
                    Vec2::new(x, title_y - y_offset),
                    TextColor::Black,
                )
                .vertices()
                .map(|vertex| vertex.to_model_vertex(params.resolution)),
            );
        }
    }
}
