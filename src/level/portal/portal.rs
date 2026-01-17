use std::f32::consts::FRAC_PI_2;

use glam::{Vec2, Vec3};
use parry3d::math::{Isometry, Vector};
use parry3d::na::{UnitQuaternion, Vector3};
use parry3d::query::{cast_shapes, ShapeCastHit, ShapeCastOptions};
use parry3d::shape::Cuboid;
use url::Url;

use crate::level::cache::{LevelCache, LevelCacheResult};

use super::spec::PortalSpec;
use super::PortalLink;

const EPSILON: f32 = 0.001;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortalKind {
    Wall,
    Floor,
    Ceiling,
}

impl PortalKind {
    pub fn pitch(&self) -> f32 {
        return match self {
            PortalKind::Wall => 0.0,
            PortalKind::Floor => -FRAC_PI_2,
            PortalKind::Ceiling => FRAC_PI_2,
        };
    }
}

#[derive(Debug, Clone)]
pub enum PortalError {
    InsufficientVertices,
    DegenerateGeometry,
    NotCoplanar,
    TiltedPortal,
    NotRectangularQuad,
    MissingUV,
    InconsistentUVs,
    InvalidUVLayout,
    RolledWallPortal,
}

pub struct LevelPortal {
    pub name: String,
    pub center: Vec3,
    pub yaw: f32,
    pub kind: PortalKind,
    pub dimensions: Vec2,
    collider: Cuboid,
    isometry: Isometry<f32>,
    pub link: Url,
}

impl LevelPortal {
    pub fn new(name: String, spec: &PortalSpec, link: Url) -> Self {
        let half_w = spec.dimensions.x / 2.0;
        let half_h = spec.dimensions.y / 2.0;
        let collider = Cuboid::new(Vector::new(half_w, half_h, 0.0));

        let rotation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), spec.yaw)
            * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), spec.kind.pitch());
        let translation = Vector3::new(spec.center.x, spec.center.y, spec.center.z).into();
        let isometry = Isometry::from_parts(translation, rotation);

        return Self {
            name,
            center: spec.center,
            yaw: spec.yaw,
            kind: spec.kind,
            dimensions: spec.dimensions,
            collider,
            isometry,
            link,
        };
    }

    pub fn local_axes(&self) -> (Vec3, Vec3) {
        let right = Vec3::new(-self.yaw.cos(), 0.0, self.yaw.sin());
        let up = self.normal().cross(right);
        return (right, up);
    }

    pub fn normal(&self) -> Vec3 {
        return match self.kind {
            PortalKind::Wall => Vec3::new(self.yaw.sin(), 0.0, self.yaw.cos()),
            PortalKind::Floor => Vec3::Y,
            PortalKind::Ceiling => Vec3::NEG_Y,
        };
    }

    pub fn sweep(
        &self,
        pos: &Isometry<f32>,
        vel: &Vector<f32>,
        shape: &Cuboid,
        max_toi: f32,
    ) -> Option<ShapeCastHit> {
        return cast_shapes(
            pos,
            vel,
            shape,
            &self.isometry,
            &Vector::zeros(),
            &self.collider,
            ShapeCastOptions::with_max_time_of_impact(max_toi),
        )
        .unwrap();
    }

    pub fn link(&self, cache: &mut LevelCache) -> Option<PortalLink> {
        let fragment = self.link.fragment()?;
        let mut url = self.link.clone();
        url.set_fragment(None);

        let LevelCacheResult::Ready(level) = cache.get(&url) else {
            return None;
        };
        let dst_portal = level.portal(fragment)?;

        // Validate kind compatibility: wall <=> wall, floor <=> ceiling
        let compatible = matches!(
            (&self.kind, &dst_portal.kind),
            (PortalKind::Wall, PortalKind::Wall)
                | (PortalKind::Floor, PortalKind::Ceiling)
                | (PortalKind::Ceiling, PortalKind::Floor)
        );
        if !compatible {
            return None;
        }

        // Validate matching dimensions (with epsilon tolerance)
        if (self.dimensions - dst_portal.dimensions).length() > EPSILON {
            return None;
        }

        return Some(PortalLink {
            url,
            portal_name: fragment.to_string(),
            src_yaw: self.yaw,
            src_center: self.center,
            dst_yaw: dst_portal.yaw,
            dst_center: dst_portal.center,
            dst_kind: dst_portal.kind,
        });
    }
}
