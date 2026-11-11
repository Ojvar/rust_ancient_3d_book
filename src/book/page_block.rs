//! The page block: the stacked paper on each side, shown as two slabs whose
//! thickness follows how many leaves have been turned.

use bevy::prelude::*;

use crate::constants::{
    ANGLE_LEFT, BLOCK_INSET, BLOCK_THICKNESS, PAGE_HEIGHT, PAGE_WIDTH, PAPER_LEAF_COUNT,
};
use crate::types::{BookMaterials, Leaf, LeafKind, LeafMode, LeafState, PageBlock};

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
            slab_transform(side, slab_thickness(initial_share(side))),
        ));
    }
}

/// Resizes both slabs to the leaves resting on each side.
///
/// A leaf that is moving (held or falling) belongs to neither side: the slab
/// it left must already shrink so the next leaf, which sits on top of the
/// remaining paper, stays visible instead of being buried inside the slab.
pub fn resize_page_block(
    leaves: Query<(&Leaf, &LeafState)>,
    mut slabs: Query<(&PageBlock, &mut Transform, &mut Visibility)>,
) {
    let resting_on = |side: PageBlock| {
        leaves
            .iter()
            .filter(|(leaf, state)| leaf.kind == LeafKind::Paper && state.mode == LeafMode::Resting)
            .filter(|(_, state)| resting_side(state.angle) == side)
            .count()
    };
    let counts = (resting_on(PageBlock::Right), resting_on(PageBlock::Left));
    for (side, mut transform, mut visibility) in &mut slabs {
        let share = match side {
            PageBlock::Right => counts.0,
            PageBlock::Left => counts.1,
        };
        let thickness = slab_thickness(share);
        transform.set_if_neq(slab_transform(*side, thickness));
        visibility.set_if_neq(if thickness > MIN_VISIBLE { Visibility::Inherited } else { Visibility::Hidden });
    }
}

/// Which stack a resting leaf lies on.
fn resting_side(angle: f32) -> PageBlock {
    if angle > ANGLE_LEFT / 2.0 { PageBlock::Left } else { PageBlock::Right }
}

/// Paper under the leaves on one side: one slice per leaf lying there.
fn slab_thickness(leaves: usize) -> f32 {
    BLOCK_THICKNESS * leaves as f32 / PAPER_LEAF_COUNT as f32
}

/// At startup every paper leaf rests on the right.
fn initial_share(side: PageBlock) -> usize {
    match side {
        PageBlock::Right => PAPER_LEAF_COUNT,
        PageBlock::Left => 0,
    }
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
