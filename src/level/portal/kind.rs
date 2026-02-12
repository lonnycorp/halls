use glam::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortalKind {
    Wall,
    Floor,
    Ceiling,
}

impl PortalKind {
    pub fn reference_axis(&self) -> Vec3 {
        return match self {
            PortalKind::Wall => Vec3::Y,
            PortalKind::Floor | PortalKind::Ceiling => Vec3::X,
        };
    }
}
