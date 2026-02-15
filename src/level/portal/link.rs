use std::f32::consts::PI;

use glam::{Mat3, Vec3};
use url::Url;

use super::{LevelPortalGeometry, LevelPortalKind};

const EPSILON: f32 = 0.001;

pub struct LevelPortalLink {
    url: Url,
    portal_name: String,
    src_yaw: f32,
    src_roll: f32,
    src_kind: LevelPortalKind,
    src_center: Vec3,
    dst_yaw: f32,
    dst_roll: f32,
    dst_center: Vec3,
    dst_kind: LevelPortalKind,
    dst_normal_world: Vec3,
}

impl LevelPortalLink {
    pub fn from_geometry_pair(
        url: Url,
        portal_name: String,
        src: LevelPortalGeometry,
        dst: LevelPortalGeometry,
    ) -> Self {
        return Self {
            url,
            portal_name,
            src_yaw: src.yaw(),
            src_roll: src.roll(),
            src_kind: src.kind(),
            src_center: src.center(),
            dst_yaw: dst.yaw(),
            dst_roll: dst.roll(),
            dst_center: dst.center(),
            dst_kind: dst.kind(),
            dst_normal_world: dst.normal(),
        };
    }

    pub fn url(&self) -> &Url {
        return &self.url;
    }

    pub fn name(&self) -> &str {
        return &self.portal_name;
    }

    pub fn dst_center(&self) -> Vec3 {
        return self.dst_center;
    }

    pub fn dst_normal(&self) -> Vec3 {
        return self.dst_normal_world;
    }

    pub fn yaw_delta(&self) -> f32 {
        return match (self.src_kind, self.dst_kind) {
            (LevelPortalKind::Wall, LevelPortalKind::Wall) => self.dst_yaw - self.src_yaw + PI,
            (LevelPortalKind::Floor, LevelPortalKind::Ceiling)
            | (LevelPortalKind::Ceiling, LevelPortalKind::Floor) => {
                self.dst_roll - self.src_roll + PI
            }
            _ => panic!("invalid portal kind pairing"),
        };
    }

    pub fn position_transform(&self, pos: Vec3, apply_nudge: bool) -> Vec3 {
        let local = pos - self.src_center;
        let rot = Mat3::from_rotation_y(self.yaw_delta());
        let new_pos = self.dst_center + rot * local;

        if !apply_nudge {
            return new_pos;
        }

        let normal = self.dst_normal();
        let reference = self.dst_kind.reference_axis();
        let axis_u = normal.cross(reference).normalize_or_zero();
        let axis_v = normal.cross(axis_u).normalize_or_zero();
        let to_center = self.dst_center - new_pos;
        let du = to_center.dot(axis_u);
        let dv = to_center.dot(axis_v);

        let nudge = axis_u * du.signum() * EPSILON + axis_v * dv.signum() * EPSILON;
        return new_pos + nudge;
    }

    pub fn velocity_transform(&self, vel: Vec3) -> Vec3 {
        let rot = Mat3::from_rotation_y(self.yaw_delta());
        return rot * vel;
    }
}
