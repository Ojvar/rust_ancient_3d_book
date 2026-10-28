//! A dark wooden desk and a candle that lights the scene.

use bevy::prelude::*;

use crate::constants::{cover::THICKNESS, environment::*, materials as palette, textures};

pub fn spawn_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn(table(&mut meshes, &mut materials, &assets));
    commands.spawn(candle(&mut meshes, &mut materials));
    commands.spawn(flame(&mut meshes, &mut materials));
    commands.spawn(candle_light());
    commands.spawn(fill_light());
    commands.insert_resource(GlobalAmbientLight {
        color: srgb(LIGHT_COLOR),
        brightness: AMBIENT_BRIGHTNESS,
        ..default()
    });
}

fn srgb((r, g, b): (f32, f32, f32)) -> Color {
    Color::srgb(r, g, b)
}

/// The desk top sits right under the book's back board.
fn table_top() -> f32 {
    -THICKNESS
}

fn table(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>, assets: &AssetServer) -> impl Bundle {
    let (w, h, d) = TABLE_SIZE;
    let wood = StandardMaterial {
        base_color_texture: Some(assets.load(textures::WOOD)),
        perceptual_roughness: palette::WOOD_ROUGHNESS,
        ..default()
    };
    (
        Mesh3d(meshes.add(Cuboid::new(w, h, d))),
        MeshMaterial3d(materials.add(wood)),
        Transform::from_xyz(0.0, table_top() - h / 2.0, 0.0),
    )
}

fn candle_base() -> Vec3 {
    let (x, z) = CANDLE_POSITION;
    Vec3::new(x, table_top(), z)
}

fn flame_position() -> Vec3 {
    candle_base() + Vec3::Y * (CANDLE_HEIGHT + FLAME_RADIUS * 1.5)
}

fn candle(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
    let wax = StandardMaterial { base_color: srgb(palette::WAX), perceptual_roughness: 0.6, ..default() };
    (
        Mesh3d(meshes.add(Cylinder::new(CANDLE_RADIUS, CANDLE_HEIGHT))),
        MeshMaterial3d(materials.add(wax)),
        Transform::from_translation(candle_base() + Vec3::Y * CANDLE_HEIGHT / 2.0),
    )
}

/// A bright emissive drop; with HDR and bloom it glows.
fn flame(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
    let (r, g, b) = palette::FLAME;
    let glow = StandardMaterial { emissive: LinearRgba::rgb(r, g, b), unlit: true, ..default() };
    (
        Mesh3d(meshes.add(Sphere::new(FLAME_RADIUS))),
        MeshMaterial3d(materials.add(glow)),
        Transform::from_translation(flame_position()).with_scale(Vec3::new(1.0, 1.8, 1.0)),
    )
}

/// Warm point light with a radius, which gives soft shadow edges.
fn candle_light() -> impl Bundle {
    (
        PointLight {
            color: srgb(LIGHT_COLOR),
            intensity: LIGHT_INTENSITY,
            radius: LIGHT_RADIUS,
            range: 30.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_translation(flame_position() + Vec3::Y * 0.15),
    )
}

/// Very weak cool light from the other side, so shadows are not pitch black.
fn fill_light() -> impl Bundle {
    (
        DirectionalLight {
            color: Color::srgb(0.6, 0.65, 0.8),
            illuminance: FILL_ILLUMINANCE,
            ..default()
        },
        Transform::from_xyz(4.0, 5.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    )
}
