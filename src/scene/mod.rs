//! Camera, lights and a small orbit control (right mouse + wheel).

mod environment;
mod screenshot;

use bevy::{
    camera::Hdr,
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
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
            .add_systems(Update, orbit_camera);
    }
}

/// HDR camera with filmic tone mapping and a soft bloom for the flame.
fn spawn_camera(mut commands: Commands) {
    let orbit = OrbitCamera { yaw: YAW, pitch: PITCH, distance: DISTANCE };
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        orbit_transform(&orbit),
        orbit,
    ));
}

/// Right drag orbits, the wheel zooms. Disabled while a page is held.
fn orbit_camera(
    buttons: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    drag: Res<DragState>,
    camera: Single<(&mut OrbitCamera, &mut Transform)>,
) {
    if drag.leaf.is_some() {
        return;
    }
    let (mut orbit, mut transform) = camera.into_inner();
    let delta = if buttons.pressed(MouseButton::Right) { motion.delta } else { Vec2::ZERO };
    let next = OrbitCamera {
        yaw: orbit.yaw - delta.x * ORBIT_SPEED,
        pitch: (orbit.pitch + delta.y * ORBIT_SPEED).clamp(MIN_PITCH, MAX_PITCH),
        distance: (orbit.distance - scroll.delta.y * ZOOM_SPEED).clamp(MIN_DISTANCE, MAX_DISTANCE),
    };
    *transform = orbit_transform(&next);
    *orbit = next;
}

fn orbit_transform(orbit: &OrbitCamera) -> Transform {
    let offset = Quat::from_euler(EulerRot::YXZ, orbit.yaw, -orbit.pitch, 0.0) * Vec3::Z;
    let (x, y, z) = FOCUS;
    let focus = Vec3::new(x, y, z);
    Transform::from_translation(focus + offset * orbit.distance).looking_at(focus, Vec3::Y)
}
