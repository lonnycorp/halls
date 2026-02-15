use crate::graphics::sprite::SpriteMaterial;

#[derive(Debug, Clone, Copy)]
pub enum SystemColor {
    White,
    Gray,
    Cyan,
}

impl SystemColor {
    pub fn material(&self) -> SpriteMaterial {
        return match self {
            SystemColor::White => SpriteMaterial::SystemWhite,
            SystemColor::Gray => SpriteMaterial::SystemGray,
            SystemColor::Cyan => SpriteMaterial::SystemCyan,
        };
    }
}
