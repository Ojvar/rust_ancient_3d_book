//! Page physics: a leaf is a rigid hinge with a bend that lags behind it.

mod simulation;

use bevy::prelude::*;

pub use simulation::step_leaf;

use crate::types::LeafState;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, simulate_leaves);
    }
}

pub fn simulate_leaves(time: Res<Time>, mut leaves: Query<&mut LeafState>) {
    let dt = time.delta_secs();
    for mut state in &mut leaves {
        let next = step_leaf(*state, dt);
        // Only write when something changed so idle leaves keep their mesh.
        if next != *state {
            *state = next;
        }
    }
}
