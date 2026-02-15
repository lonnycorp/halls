use crate::graphics::sprite::SpriteMaterial;

#[derive(Debug, Clone, Copy)]
pub enum TextColor {
    White,
    Gray,
    Cyan,
    Black,
}

impl TextColor {
    pub fn font_material(&self) -> SpriteMaterial {
        return match self {
            TextColor::White => SpriteMaterial::TextWhite,
            TextColor::Gray => SpriteMaterial::TextGray,
            TextColor::Cyan => SpriteMaterial::TextCyan,
            TextColor::Black => SpriteMaterial::TextBlack,
        };
    }
}
