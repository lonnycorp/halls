use std::f32::consts::PI;

use glam::Vec2;

use winit::keyboard::KeyCode;

use crate::audio::Effect;
use crate::graphics::color::Color;
use crate::graphics::model::ModelBuffer;
use crate::graphics::sprite::{SpriteLogo, SpriteSolid, SpriteText, TEXT_SIZE};
use crate::window::{InputController, KeyState};
use crate::{Status, StatusBuffer};

const TIME_STEP: f32 = 0.002;
const WHITE: Color = Color::WHITE;

const INTRO_START: f32 = 0.15;
const INTRO_FADE_OUT_START: f32 = 0.85;
const INTRO_FADE_OUT_END: f32 = 0.90;

const TITLE: &[u8] = b"LONNYCORP";
const TITLE_CHAR_FADE_DURATION: f32 = 0.05;
const TITLE_CHAR_FADE_DELAY: f32 = 0.01;
const TITLE_SPACING: f32 = 2.0;
const LOGO_UV_SIZE: f32 = 480.0;
const TITLE_BOUNCE_HEIGHT: f32 = 32.0;

fn ramp(t: f32, start: f32, end: f32) -> f32 {
    return ((t - start) / (end - start)).clamp(0.0, 1.0);
}

pub struct IntroUpdateContext<'a> {
    pub buffer: &'a mut ModelBuffer,
    pub resolution: Vec2,
    pub input: &'a InputController<'a>,
    pub status: &'a mut StatusBuffer,
    pub jingle_effect: &'a Effect,
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

    pub fn update(&mut self, ctx: &mut IntroUpdateContext) {
        if !matches!(ctx.status.get(), Status::Intro) {
            return;
        }

        if let KeyState::Pressed = ctx.input.key(KeyCode::Escape) {
            ctx.status.set(Status::MenuHome);
            return;
        }

        self.time = (self.time + TIME_STEP).min(1.0);
        if self.time >= 1.0 {
            ctx.status.set(Status::MenuHome);
        }

        if !self.jingle_played && self.time >= INTRO_START {
            ctx.jingle_effect.reset();
            ctx.jingle_effect.play();
            self.jingle_played = true;
        }

        let fade_out = 1.0 - ramp(self.time, INTRO_FADE_OUT_START, INTRO_FADE_OUT_END);
        let logo_alpha = (fade_out * 255.0) as u8;

        SpriteSolid::new(Vec2::ZERO, ctx.resolution, WHITE)
            .write_to_model_buffer(ctx.buffer, ctx.resolution);
        SpriteLogo::new(ctx.resolution / 2.0, logo_alpha)
            .write_to_model_buffer(ctx.buffer, ctx.resolution);

        let title_width = ((TITLE.len() - 1) as f32 * TITLE_SPACING + 1.0) * TEXT_SIZE.x;
        let title_x = (ctx.resolution.x - title_width) / 2.0;
        let title_y = ctx.resolution.y / 2.0 + LOGO_UV_SIZE / 2.0 + TEXT_SIZE.y;

        for (i, &c) in TITLE.iter().enumerate() {
            let char_start = INTRO_START + i as f32 * TITLE_CHAR_FADE_DELAY;
            let char_life = ramp(self.time, char_start, char_start + TITLE_CHAR_FADE_DURATION);
            let char_alpha = char_life * fade_out;
            let a = (char_alpha * 255.0) as u8;
            let x = title_x + i as f32 * TITLE_SPACING * TEXT_SIZE.x;
            let y_offset = (char_life * PI).sin() * TITLE_BOUNCE_HEIGHT;
            SpriteText::new(
                c as char,
                true,
                Vec2::new(x, title_y - y_offset),
                Color::new(0, 0, 0, a),
            )
            .write_to_model_buffer(ctx.buffer, ctx.resolution);
        }
    }
}
