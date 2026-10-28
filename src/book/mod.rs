//! The tome: leather boards, a page block and leaves driven by physics.

mod cover;
mod deformation;
mod materials;
mod mesh;
mod page_block;
mod spawn;

use bevy::prelude::*;

use crate::constants::{
    ANGLE_LEFT, BLOCK_THICKNESS, COVER_INDEX, LEAF_GAP, PAPER_LEAF_COUNT,
};
use crate::types::{BendParams, Leaf, LeafKind, LeafState};

pub struct BookPlugin;

impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (materials::create_materials, (cover::spawn_cover, spawn::spawn_pages, page_block::spawn_page_block))
                .chain(),
        )
        .add_systems(
            PostUpdate,
            (bend_paper_meshes, rotate_covers, lift_leaves, page_block::resize_page_block),
        );
    }
}

/// Re-deforms the face meshes of every paper leaf whose state changed.
fn bend_paper_meshes(
    leaves: Query<(&Leaf, &LeafState, &Children), Changed<LeafState>>,
    faces: Query<&Mesh3d>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (_, state, children) in leaves.iter().filter(|(leaf, ..)| leaf.kind == LeafKind::Paper) {
        let positions = deformation::deform_page(BendParams {
            angle: state.angle,
            bend: state.bend,
            grab_z: state.grab_z,
        });
        for face in children.iter().filter_map(|child| faces.get(child).ok()) {
            if let Some(mut mesh) = meshes.get_mut(&face.0) {
                mesh::apply_positions(&mut mesh, positions.clone());
            }
        }
    }
}

/// A rigid board simply rotates around the spine.
fn rotate_covers(mut leaves: Query<(&Leaf, &LeafState, &mut Transform), Changed<LeafState>>) {
    for (_, state, mut transform) in leaves.iter_mut().filter(|(leaf, ..)| leaf.kind == LeafKind::Cover) {
        transform.rotation = Quat::from_rotation_z(state.angle);
    }
}

/// Moves each leaf between its resting heights on the right and left.
fn lift_leaves(mut leaves: Query<(&Leaf, &LeafState, &mut Transform), Changed<LeafState>>) {
    for (leaf, state, mut transform) in &mut leaves {
        transform.translation.y = leaf_height(leaf, state.angle);
    }
}

/// Height of a leaf's hinge, blended from its right to its left resting height.
pub fn leaf_height(leaf: &Leaf, angle: f32) -> f32 {
    let (right, left) = resting_heights(leaf);
    right + (left - right) * (angle / ANGLE_LEFT)
}

/// The closed cover lies on top of the block; opened, it is the base on the left.
/// Paper leaf `k` lies on top of the part of the block that is on its side.
fn resting_heights(leaf: &Leaf) -> (f32, f32) {
    match leaf.kind {
        LeafKind::Cover => (BLOCK_THICKNESS + 2.0 * LEAF_GAP, 0.0),
        LeafKind::Paper => {
            let k = (leaf.index - COVER_INDEX - 1) as f32;
            let sheets = PAPER_LEAF_COUNT as f32;
            let slice = BLOCK_THICKNESS / sheets;
            (slice * (sheets - k) + LEAF_GAP, slice * (k + 1.0) + LEAF_GAP)
        }
    }
}
