//! Page physics: a leaf is a rigid hinge with a bend that lags behind it.

mod simulation;

use bevy::prelude::*;

use simulation::step_leaf;

use crate::constants::{COVER_PHYSICS, PAPER_PHYSICS};
use crate::types::{Leaf, LeafKind, LeafState, PhysicsParams};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, simulate_leaves);
    }
}

pub fn simulate_leaves(time: Res<Time>, mut leaves: Query<(&Leaf, &mut LeafState)>) {
    let dt = time.delta_secs();
    for (leaf, mut state) in &mut leaves {
        let next = step_leaf(*state, params_for(leaf.kind), dt);
        // Only write when something changed so idle leaves keep their mesh.
        if next != *state {
            *state = next;
        }
    }
}

/// Boards are heavy and rigid, paper is light and flexible.
fn params_for(kind: LeafKind) -> &'static PhysicsParams {
    match kind {
        LeafKind::Cover => &COVER_PHYSICS,
        LeafKind::Paper => &PAPER_PHYSICS,
    }
}
