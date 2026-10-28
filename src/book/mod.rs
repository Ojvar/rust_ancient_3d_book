//! The book: a cover plus leaves whose meshes bend with their physics state.

mod deformation;
mod mesh;
mod spawn;

use bevy::prelude::*;

use crate::constants::{ANGLE_LEFT, LEAF_COUNT, LEAF_GAP};
use crate::types::{BendParams, Leaf, LeafState};

pub struct BookPlugin;

impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::spawn_book)
            .add_systems(PostUpdate, (bend_leaf_meshes, lift_leaves));
    }
}

/// Re-deforms the face meshes of every leaf whose state changed.
fn bend_leaf_meshes(
    leaves: Query<(&LeafState, &Children), Changed<LeafState>>,
    faces: Query<&Mesh3d>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (state, children) in &leaves {
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

/// Moves each leaf between its height on the right and left stacks.
fn lift_leaves(mut leaves: Query<(&Leaf, &LeafState, &mut Transform), Changed<LeafState>>) {
    for (leaf, state, mut transform) in &mut leaves {
        transform.translation.y = stack_height(leaf.index, state.angle);
    }
}

/// Unturned leaves stack on the right (first on top), turned ones on the left.
pub fn stack_height(index: usize, angle: f32) -> f32 {
    let right = (LEAF_COUNT - index) as f32 * LEAF_GAP;
    let left = (index + 1) as f32 * LEAF_GAP;
    right + (left - right) * (angle / ANGLE_LEFT)
}
