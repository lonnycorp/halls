use std::f32::consts::PI;

use glam::{Mat3, Vec3};
use url::Url;

use super::PortalKind;

const EPSILON: f32 = 0.001;

pub struct PortalLink {
    pub url: Url,
    pub portal_name: String,
    pub src_yaw: f32,
    pub src_center: Vec3,
    pub dst_yaw: f32,
    pub dst_center: Vec3,
    pub dst_kind: PortalKind,
}

impl PortalLink {
    pub fn yaw_delta(&self) -> f32 {
        return self.dst_yaw - self.src_yaw + PI;
    }

    pub fn transform_position(&self, pos: Vec3, apply_nudge: bool) -> Vec3 {
        let local = pos - self.src_center;
        let rot = Mat3::from_rotation_y(self.yaw_delta());
        let new_pos = self.dst_center + rot * local;

        if !apply_nudge {
            return new_pos;
        }

        // Nudge towards portal center in the portal plane
        let offset = new_pos - self.dst_center;
        let normal = self.dst_normal();
        let to_center = -offset;
        let parallel = to_center.dot(normal) * normal;
        let nudge_dir = (to_center - parallel).normalize_or_zero();

        return new_pos + nudge_dir * EPSILON;
    }

    pub fn transform_velocity(&self, vel: Vec3) -> Vec3 {
        let rot = Mat3::from_rotation_y(self.yaw_delta());
        return rot * vel;
    }

    pub fn dst_normal(&self) -> Vec3 {
        return match self.dst_kind {
            PortalKind::Wall => Vec3::new(self.dst_yaw.sin(), 0.0, self.dst_yaw.cos()),
            PortalKind::Floor => Vec3::Y,
            PortalKind::Ceiling => Vec3::NEG_Y,
        };
    }
}
