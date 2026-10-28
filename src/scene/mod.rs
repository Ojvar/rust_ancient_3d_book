//! Camera, lights and a small orbit control (right mouse + wheel).

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

use crate::constants::camera::*;
use crate::types::{DragState, OrbitCamera};

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.12, 0.13, 0.16)))
            .add_systems(Startup, (spawn_camera, spawn_lights))
            .add_systems(Update, orbit_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let orbit = OrbitCamera { yaw: 0.0, pitch: PITCH, distance: DISTANCE };
    commands.spawn((Camera3d::default(), orbit_transform(&orbit), orbit));
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        DirectionalLight { illuminance: 6_000.0, shadow_maps_enabled: true, ..default() },
        Transform::from_xyz(2.0, 6.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(GlobalAmbientLight { brightness: 400.0, ..default() });
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
    Transform::from_translation(offset * orbit.distance).looking_at(Vec3::ZERO, Vec3::Y)
}
