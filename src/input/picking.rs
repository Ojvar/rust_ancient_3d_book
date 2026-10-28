//! Ray casting from the cursor onto the book and choosing which leaf to grab.

use bevy::prelude::*;

use crate::constants::{ANGLE_LEFT, ANGLE_RIGHT, LEAF_COUNT, LEAF_GAP, PAGE_HEIGHT, PAGE_WIDTH};
use crate::types::{Leaf, LeafMode, LeafState};

/// Where the cursor ray meets the top of the page stacks.
#[derive(Clone, Copy, Debug)]
pub struct BookHit {
    pub point: Vec3,
}

impl BookHit {
    pub fn is_on_page(&self) -> bool {
        self.point.x.abs() <= PAGE_WIDTH && self.point.z.abs() <= PAGE_HEIGHT / 2.0
    }
}

pub fn cursor_hit(window: &Window, camera: &(&Camera, &GlobalTransform)) -> Option<BookHit> {
    let (camera, transform) = *camera;
    let cursor = window.cursor_position()?;
    let ray = camera.viewport_to_world(transform, cursor).ok()?;
    let top = Vec3::Y * (LEAF_COUNT as f32 * LEAF_GAP);
    ray.plane_intersection_point(top, InfinitePlane3d::new(Vec3::Y))
        .map(|point| BookHit { point })
}

/// Right half: the top unturned leaf. Left half: the top turned leaf.
pub fn pick_leaf(
    leaves: impl Iterator<Item = (Entity, Leaf, LeafState)>,
    hit_x: f32,
) -> Option<Entity> {
    let resting_at = |angle: f32| {
        move |(_, _, s): &(Entity, Leaf, LeafState)| s.mode == LeafMode::Resting && s.angle == angle
    };
    if hit_x >= 0.0 {
        leaves
            .filter(resting_at(ANGLE_RIGHT))
            .min_by_key(|(_, leaf, _)| leaf.index)
            .map(|(entity, _, _)| entity)
    } else {
        leaves
            .filter(resting_at(ANGLE_LEFT))
            .max_by_key(|(_, leaf, _)| leaf.index)
            .map(|(entity, _, _)| entity)
    }
}
