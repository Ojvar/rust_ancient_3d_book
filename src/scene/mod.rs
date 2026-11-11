//! Camera, lights and an orbit control (orbit, pan and zoom with mouse or keys).

mod environment;
mod screenshot;

use bevy::{
    camera::Hdr,
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    post_process::bloom::Bloom,
    prelude::*,
};

use crate::constants::{camera::*, environment::BACKGROUND};
use crate::types::{DragState, OrbitCamera};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        let (r, g, b) = BACKGROUND;
        app.insert_resource(ClearColor(Color::srgb(r, g, b)))
            .add_plugins(screenshot::ScreenshotPlugin)
            .add_systems(Startup, (spawn_camera, environment::spawn_environment))
            .add_systems(Update, (orbit_camera, environment::flicker_candle));
    }
}

/// HDR camera with filmic tone mapping and a soft bloom for the flame.
fn spawn_camera(mut commands: Commands) {
    let orbit = default_orbit();
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        orbit_transform(&orbit),
        orbit,
        CameraTarget(orbit),
    ));
}

/// Where the camera is heading; the real [`OrbitCamera`] eases toward it.
#[derive(Component, Clone, Copy, Debug)]
struct CameraTarget(OrbitCamera);

fn default_orbit() -> OrbitCamera {
    let (x, y, z) = FOCUS;
    OrbitCamera { focus: Vec3::new(x, y, z), yaw: YAW, pitch: PITCH, distance: DISTANCE }
}

/// Mouse: right drag orbits, middle drag (or Shift + right drag) pans, the
/// wheel zooms. Keyboard: WASD / arrows pan, Q/E orbit, +/- zoom, R resets.
/// Mouse orbiting and panning are disabled while a page is held.
fn orbit_camera(
    time: Res<Time>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    drag: Res<DragState>,
    camera: Single<(&mut OrbitCamera, &mut CameraTarget, &mut Transform)>,
) {
    let (mut orbit, mut target, mut transform) = camera.into_inner();
    let dt = time.delta_secs();
    let mut next = target.0;

    if keys.just_pressed(KeyCode::KeyR) {
        next = default_orbit();
    }

    let holding_page = drag.leaf.is_some();
    let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let right = buttons.pressed(MouseButton::Right) && !holding_page;
    let panning = !holding_page && (buttons.pressed(MouseButton::Middle) || (right && shift));
    let orbiting = right && !shift;

    // Orbit.
    let mut orbit_delta = if orbiting { motion.delta * ORBIT_SPEED } else { Vec2::ZERO };
    orbit_delta.x += axis(&keys, &[KeyCode::KeyQ], &[KeyCode::KeyE]) * KEY_ORBIT_SPEED * dt;
    next.yaw -= orbit_delta.x;
    next.pitch = (next.pitch + orbit_delta.y).clamp(MIN_PITCH, MAX_PITCH);

    // Pan on the table plane, relative to where the camera faces.
    let (right_dir, forward_dir) = ground_axes(next.yaw);
    let mut pan = Vec2::ZERO;
    if panning {
        pan += Vec2::new(-motion.delta.x, motion.delta.y) * PAN_SPEED * next.distance;
    }
    let key_pan = Vec2::new(
        axis(&keys, &[KeyCode::KeyA, KeyCode::ArrowLeft], &[KeyCode::KeyD, KeyCode::ArrowRight]),
        axis(&keys, &[KeyCode::KeyS, KeyCode::ArrowDown], &[KeyCode::KeyW, KeyCode::ArrowUp]),
    );
    pan += key_pan * KEY_PAN_SPEED * next.distance * dt;
    next.focus += right_dir * pan.x + forward_dir * pan.y;
    let (limit_x, limit_z) = PAN_LIMIT;
    next.focus.x = next.focus.x.clamp(-limit_x, limit_x);
    next.focus.z = next.focus.z.clamp(-limit_z, limit_z);

    // Zoom: wheel notches or touchpad pixels, plus keyboard.
    let notches = match scroll.unit {
        MouseScrollUnit::Line => scroll.delta.y,
        MouseScrollUnit::Pixel => scroll.delta.y / PIXELS_PER_LINE,
    };
    let key_zoom = axis(&keys, &[KeyCode::Minus, KeyCode::NumpadSubtract], &[KeyCode::Equal, KeyCode::NumpadAdd]);
    next.distance = (next.distance - notches * ZOOM_SPEED - key_zoom * KEY_ZOOM_SPEED * dt)
        .clamp(MIN_DISTANCE, MAX_DISTANCE);

    target.0 = next;

    // Ease the real camera toward the target so moves feel smooth.
    let blend = 1.0 - (-SMOOTHING * dt).exp();
    let eased = OrbitCamera {
        focus: orbit.focus.lerp(next.focus, blend),
        yaw: orbit.yaw + (next.yaw - orbit.yaw) * blend,
        pitch: orbit.pitch + (next.pitch - orbit.pitch) * blend,
        distance: orbit.distance + (next.distance - orbit.distance) * blend,
    };
    *transform = orbit_transform(&eased);
    *orbit = eased;
}

/// -1, 0 or 1 depending on which group of keys is held.
fn axis(keys: &ButtonInput<KeyCode>, negative: &[KeyCode], positive: &[KeyCode]) -> f32 {
    let held = |group: &[KeyCode]| if keys.any_pressed(group.iter().copied()) { 1.0 } else { 0.0 };
    held(positive) - held(negative)
}

/// Screen-right and screen-forward directions projected onto the table.
fn ground_axes(yaw: f32) -> (Vec3, Vec3) {
    let rotation = Quat::from_rotation_y(yaw);
    (rotation * Vec3::X, rotation * Vec3::NEG_Z)
}

fn orbit_transform(orbit: &OrbitCamera) -> Transform {
    let offset = Quat::from_euler(EulerRot::YXZ, orbit.yaw, -orbit.pitch, 0.0) * Vec3::Z;
    Transform::from_translation(orbit.focus + offset * orbit.distance).looking_at(orbit.focus, Vec3::Y)
}
