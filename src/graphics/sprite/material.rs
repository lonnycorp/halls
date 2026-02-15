use crate::color::Color;
use crate::graphics::storage::MaterialTextureRef;

const STATIC_ANIMATION_SPEED: f32 = 0.0;
pub const TEXT_TEXTURE_REF: MaterialTextureRef = MaterialTextureRef {
    bucket: 0,
    layer: 0,
};
pub const SYSTEM_TEXTURE_REF: MaterialTextureRef = MaterialTextureRef {
    bucket: 0,
    layer: 1,
};

const TEXT_TEXTURE_REFS: [MaterialTextureRef; 1] = [TEXT_TEXTURE_REF];
const SYSTEM_TEXTURE_REFS: [MaterialTextureRef; 1] = [SYSTEM_TEXTURE_REF];

#[derive(Debug, Clone, Copy)]
pub(crate) struct SpriteMaterialIndexData<'a> {
    pub material_ix: u32,
    pub speed: f32,
    pub texture_refs: &'a [MaterialTextureRef],
    pub color: Color,
}

#[derive(Debug, Clone, Copy, strum::EnumIter)]
pub enum SpriteMaterial {
    TextWhite,
    TextGray,
    TextCyan,
    TextBlack,
    SystemWhite,
    SystemGray,
    SystemCyan,
    SystemBlack,
}

impl SpriteMaterial {
    pub(crate) fn data(&self) -> SpriteMaterialIndexData<'static> {
        return match self {
            SpriteMaterial::TextWhite => SpriteMaterialIndexData {
                material_ix: 0,
                speed: STATIC_ANIMATION_SPEED,
                texture_refs: &TEXT_TEXTURE_REFS,
                color: Color::WHITE,
            },
            SpriteMaterial::TextGray => SpriteMaterialIndexData {
                material_ix: 1,
                speed: STATIC_ANIMATION_SPEED,
                texture_refs: &TEXT_TEXTURE_REFS,
                color: Color::GRAY,
            },
            SpriteMaterial::TextCyan => SpriteMaterialIndexData {
                material_ix: 2,
                speed: STATIC_ANIMATION_SPEED,
                texture_refs: &TEXT_TEXTURE_REFS,
                color: Color::CYAN,
            },
            SpriteMaterial::TextBlack => SpriteMaterialIndexData {
                material_ix: 3,
                speed: STATIC_ANIMATION_SPEED,
                texture_refs: &TEXT_TEXTURE_REFS,
                color: Color::BLACK,
            },
            SpriteMaterial::SystemWhite => SpriteMaterialIndexData {
                material_ix: 4,
                speed: STATIC_ANIMATION_SPEED,
                texture_refs: &SYSTEM_TEXTURE_REFS,
                color: Color::WHITE,
            },
            SpriteMaterial::SystemGray => SpriteMaterialIndexData {
                material_ix: 5,
                speed: STATIC_ANIMATION_SPEED,
                texture_refs: &SYSTEM_TEXTURE_REFS,
                color: Color::GRAY,
            },
            SpriteMaterial::SystemCyan => SpriteMaterialIndexData {
                material_ix: 6,
                speed: STATIC_ANIMATION_SPEED,
                texture_refs: &SYSTEM_TEXTURE_REFS,
                color: Color::CYAN,
            },
            SpriteMaterial::SystemBlack => SpriteMaterialIndexData {
                material_ix: 7,
                speed: STATIC_ANIMATION_SPEED,
                texture_refs: &SYSTEM_TEXTURE_REFS,
                color: Color::BLACK,
            },
        };
    }
}
