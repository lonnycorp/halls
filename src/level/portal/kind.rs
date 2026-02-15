use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LevelPortalKind {
    Wall,
    Floor,
    Ceiling,
}

impl LevelPortalKind {
    pub fn reference_axis(&self) -> Vec3 {
        return match self {
            LevelPortalKind::Wall => Vec3::Y,
            LevelPortalKind::Floor | LevelPortalKind::Ceiling => Vec3::X,
        };
    }
}
