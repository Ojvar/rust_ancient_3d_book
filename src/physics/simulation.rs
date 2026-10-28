//! Pure, frame-rate independent integration of one leaf.
//!
//! Semi-implicit Euler: velocity is updated from the acceleration first,
//! then the angle from the new velocity. The frame time is clamped and split
//! into fixed sub-steps, so the result does not depend on the frame rate.

use crate::constants::{ANGLE_LEFT, ANGLE_RIGHT, physics::*};
use crate::types::{LeafMode, LeafState};

/// Advances a leaf by `dt` seconds and returns its new state.
pub fn step_leaf(state: LeafState, dt: f32) -> LeafState {
    if state.mode == LeafMode::Resting {
        return state;
    }
    let dt = dt.clamp(0.0, MAX_DT);
    let steps = (dt / SUB_STEP).ceil().max(1.0) as usize;
    let h = dt / steps as f32;
    (0..steps).fold(state, |s, _| sub_step(s, h))
}

fn sub_step(state: LeafState, h: f32) -> LeafState {
    let velocity = state.velocity + acceleration(&state) * h;
    let angle = state.angle + velocity * h;
    let moved = LeafState { angle, velocity, ..state };
    let collided = collide_with_stacks(moved);
    let bent = update_bend(collided, h);
    settle(bent)
}

/// Angular acceleration from the cursor spring, or from gravity and air.
fn acceleration(state: &LeafState) -> f32 {
    match state.mode {
        LeafMode::Dragged { target, .. } => {
            DRAG_STIFFNESS * (target - state.angle) - DRAG_DAMPING * state.velocity
        }
        LeafMode::Free => gravity_torque(state.angle) - DAMPING * state.velocity,
        LeafMode::Resting => 0.0,
    }
}

/// Gravity pulls a lifted page down to the side it leans toward:
/// negative below 90° (back to the right), positive above (on to the left).
fn gravity_torque(angle: f32) -> f32 {
    -GRAVITY * angle.cos()
}

/// Keeps the leaf between the two stacks, bouncing a little on impact.
fn collide_with_stacks(state: LeafState) -> LeafState {
    let bounce = |angle: f32| LeafState {
        angle,
        velocity: -state.velocity * RESTITUTION,
        ..state
    };
    match state.angle {
        a if a < ANGLE_RIGHT => bounce(ANGLE_RIGHT),
        a if a > ANGLE_LEFT => bounce(ANGLE_LEFT),
        _ => state,
    }
}

/// The free edge lags behind the spine when moving fast and sags when lifted.
fn update_bend(state: LeafState, h: f32) -> LeafState {
    let sag = DROOP * (2.0 * state.angle).sin();
    let target = (state.velocity * BEND_PER_SPEED + sag).clamp(-MAX_BEND, MAX_BEND);
    let blend = 1.0 - (-BEND_RESPONSE * h).exp();
    LeafState {
        bend: state.bend + (target - state.bend) * blend,
        ..state
    }
}

/// A released leaf lying slowly on a stack comes to rest there.
fn settle(state: LeafState) -> LeafState {
    let on_stack = |a: f32| (a - ANGLE_RIGHT).abs() < 1e-3 || (a - ANGLE_LEFT).abs() < 1e-3;
    let calm = state.velocity.abs() < REST_SPEED && state.bend.abs() < 0.01;
    if state.mode == LeafMode::Free && on_stack(state.angle) && calm {
        LeafState {
            velocity: 0.0,
            bend: 0.0,
            mode: LeafMode::Resting,
            ..state
        }
    } else {
        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_2;

    fn free(angle: f32, velocity: f32) -> LeafState {
        LeafState { angle, velocity, bend: 0.0, grab_z: 0.0, mode: LeafMode::Free }
    }

    fn run(mut s: LeafState, seconds: f32) -> LeafState {
        for _ in 0..(seconds * 60.0) as usize {
            s = step_leaf(s, 1.0 / 60.0);
        }
        s
    }

    #[test]
    fn falls_back_when_released_before_middle_without_speed() {
        let s = run(free(FRAC_PI_2 - 0.3, 0.0), 6.0);
        assert_eq!(s.mode, LeafMode::Resting);
        assert_eq!(s.angle, ANGLE_RIGHT);
    }

    #[test]
    fn completes_turn_when_flicked() {
        let s = run(free(0.6, 7.0), 6.0);
        assert_eq!(s.mode, LeafMode::Resting);
        assert_eq!(s.angle, ANGLE_LEFT);
    }

    #[test]
    fn result_is_frame_rate_independent() {
        let at_60 = run(free(1.0, 2.0), 0.5);
        let mut at_240 = free(1.0, 2.0);
        for _ in 0..120 {
            at_240 = step_leaf(at_240, 1.0 / 240.0);
        }
        assert!((at_60.angle - at_240.angle).abs() < 1e-3);
    }
}
