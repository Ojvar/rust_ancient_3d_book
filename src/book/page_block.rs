//! The page block: the stacked paper on each side, shown as two slabs whose
//! thickness follows how many leaves have been turned.

use bevy::prelude::*;

use crate::constants::{
    ANGLE_LEFT, BLOCK_INSET, BLOCK_THICKNESS, PAGE_HEIGHT, PAGE_WIDTH, PAPER_LEAF_COUNT,
};
use crate::types::{BookMaterials, Leaf, LeafKind, LeafState, PageBlock};

/// Thinner than this a slab is hidden instead of drawn flat.
const MIN_VISIBLE: f32 = 1e-3;

pub fn spawn_page_block(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<BookMaterials>,
) {
    let unit = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    for side in [PageBlock::Right, PageBlock::Left] {
        commands.spawn((
            side,
            Mesh3d(unit.clone()),
            MeshMaterial3d(materials.page_edge.clone()),
            slab_transform(side, slab_thickness(side, 0)),
        ));
    }
}

/// Counts turned paper leaves and resizes both slabs to match.
pub fn resize_page_block(
    leaves: Query<(&Leaf, &LeafState)>,
    mut slabs: Query<(&PageBlock, &mut Transform, &mut Visibility)>,
) {
    let turned = leaves
        .iter()
        .filter(|(leaf, state)| leaf.kind == LeafKind::Paper && state.angle > ANGLE_LEFT / 2.0)
        .count();
    for (side, mut transform, mut visibility) in &mut slabs {
        let thickness = slab_thickness(*side, turned);
        transform.set_if_neq(slab_transform(*side, thickness));
        visibility.set_if_neq(if thickness > MIN_VISIBLE { Visibility::Inherited } else { Visibility::Hidden });
    }
}

fn slab_thickness(side: PageBlock, turned: usize) -> f32 {
    let share = match side {
        PageBlock::Right => PAPER_LEAF_COUNT - turned,
        PageBlock::Left => turned,
    };
    BLOCK_THICKNESS * share as f32 / PAPER_LEAF_COUNT as f32
}

/// A unit cube scaled to a slab lying on the board on its side of the spine.
fn slab_transform(side: PageBlock, thickness: f32) -> Transform {
    let width = PAGE_WIDTH * BLOCK_INSET;
    let x = match side {
        PageBlock::Right => width / 2.0,
        PageBlock::Left => -width / 2.0,
    };
    Transform::from_xyz(x, thickness / 2.0, 0.0)
        .with_scale(Vec3::new(width, thickness.max(MIN_VISIBLE), PAGE_HEIGHT * BLOCK_INSET))
}
