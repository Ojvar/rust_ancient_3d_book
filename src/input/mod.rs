//! Mouse input: grab a page with the left button and drag it.

mod picking;

use bevy::prelude::*;

use crate::constants::PAGE_WIDTH;
use crate::types::{DragState, Leaf, LeafMode, LeafState};
use picking::{BookHit, cursor_hit, pick_leaf};

/// Never let the grab point sit on the spine, where the angle is undefined.
const MIN_GRAB_FRACTION: f32 = 0.25;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DragState>().add_systems(
            Update,
            (start_drag, follow_cursor, release_drag)
                .chain()
                .before(crate::physics::simulate_leaves),
        );
    }
}

fn start_drag(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut drag: ResMut<DragState>,
    mut leaves: Query<(Entity, &Leaf, &mut LeafState)>,
) {
    if !buttons.just_pressed(MouseButton::Left) || drag.leaf.is_some() {
        return;
    }
    let Some(hit) = cursor_hit(&window, &camera).filter(BookHit::is_on_page) else {
        return;
    };
    let candidates = leaves.iter().map(|(entity, leaf, state)| (entity, *leaf, *state));
    let Some(entity) = pick_leaf(candidates, hit.point.x) else {
        return;
    };
    if let Ok((_, _, mut state)) = leaves.get_mut(entity) {
        state.grab_z = hit.point.z;
        state.mode = LeafMode::Dragged { target: state.angle, grab_z: hit.point.z };
        *drag = DragState {
            leaf: Some(entity),
            grab_fraction: (hit.point.x.abs() / PAGE_WIDTH).max(MIN_GRAB_FRACTION),
        };
    }
}

/// Converts the cursor position into the angle the grabbed leaf should reach.
fn follow_cursor(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    drag: Res<DragState>,
    mut leaves: Query<&mut LeafState>,
) {
    let Some(mut state) = drag.leaf.and_then(|entity| leaves.get_mut(entity).ok()) else {
        return;
    };
    let (Some(hit), LeafMode::Dragged { grab_z, .. }) = (cursor_hit(&window, &camera), state.mode)
    else {
        return;
    };
    let target = target_angle(hit.point.x, drag.grab_fraction);
    state.mode = LeafMode::Dragged { target, grab_z };
}

/// The grabbed point stays under the cursor: x = cos(angle) * grab radius.
fn target_angle(cursor_x: f32, grab_fraction: f32) -> f32 {
    (cursor_x / (grab_fraction * PAGE_WIDTH)).clamp(-1.0, 1.0).acos()
}

/// On release the leaf keeps its velocity and physics takes over.
fn release_drag(
    buttons: Res<ButtonInput<MouseButton>>,
    mut drag: ResMut<DragState>,
    mut leaves: Query<&mut LeafState>,
) {
    if !buttons.just_released(MouseButton::Left) {
        return;
    }
    if let Some(mut state) = drag.leaf.take().and_then(|entity| leaves.get_mut(entity).ok()) {
        state.mode = LeafMode::Free;
    }
}
