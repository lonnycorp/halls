use glam::Vec2;

use crate::graphics::model::ModelVertex;
use crate::graphics::sprite::{
    SpriteBorder, SpriteLabel, SpriteLabelAlignment, TextColor, TEXT_SIZE,
};
use crate::level::cache::{LevelCache, LevelCacheResult};
use crate::player::Player;

const BORDER: f32 = 3.0;
const TEXT_PADDING: f32 = 3.0;
const SCREEN_PADDING: f32 = 6.0;
const LINE_COUNT: f32 = 3.0;
const MAX_CHARS: usize = 42;
const LABEL_LEN: usize = 7;
const INSET: f32 = BORDER + TEXT_PADDING;
const BOX_WIDTH: f32 = MAX_CHARS as f32 * TEXT_SIZE.x + INSET * 2.0;
const BOX_HEIGHT: f32 = LINE_COUNT * TEXT_SIZE.y + INSET * 2.0;
const TEXT_COLOR: TextColor = TextColor::White;

pub fn update_banner(
    buffer: &mut Vec<ModelVertex>,
    resolution: Vec2,
    player: &Player,
    cache: &mut LevelCache,
) {
    let Some(level_url) = player.level_url() else {
        return;
    };

    let level = match cache.get(level_url) {
        LevelCacheResult::Ready(level) => level,
        _ => return,
    };

    let box_pos = Vec2::new(
        resolution.x - BOX_WIDTH - SCREEN_PADDING,
        resolution.y - BOX_HEIGHT - SCREEN_PADDING,
    );
    buffer.extend(
        SpriteBorder::new(box_pos, Vec2::new(BOX_WIDTH, BOX_HEIGHT))
            .vertices()
            .map(|vertex| vertex.to_model_vertex(resolution)),
    );

    let label_x = box_pos.x + INSET;
    let value_x = label_x + LABEL_LEN as f32 * TEXT_SIZE.x;
    let text_y = box_pos.y + INSET;

    let meta = level.meta();
    let author = meta.author().unwrap_or("N/A");
    let track = meta.track().unwrap_or("N/A");

    let lines: [(&str, &str, bool); 3] = [
        ("LEVEL", meta.name(), true),
        ("AUTHOR", author, false),
        ("AUDIO", track, false),
    ];

    for (i, &(label, value, bold)) in lines.iter().enumerate() {
        let y = text_y + i as f32 * TEXT_SIZE.y;
        buffer.extend(
            SpriteLabel::new(
                Vec2::new(label_x, y),
                LABEL_LEN,
                TEXT_COLOR,
                false,
                SpriteLabelAlignment::Left,
                label,
            )
            .vertices()
            .map(|vertex| vertex.to_model_vertex(resolution)),
        );
        buffer.extend(
            SpriteLabel::new(
                Vec2::new(value_x, y),
                MAX_CHARS - LABEL_LEN,
                TEXT_COLOR,
                bold,
                SpriteLabelAlignment::Left,
                value,
            )
            .vertices()
            .map(|vertex| vertex.to_model_vertex(resolution)),
        );
    }
}
