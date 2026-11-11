//! A dark wooden desk and three candles that light the scene.

use bevy::{light::NotShadowCaster, prelude::*};

use crate::constants::{cover::THICKNESS, environment::*, materials as palette, textures};

pub fn spawn_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn(table(&mut meshes, &mut materials, &assets));
    commands.spawn(candle(&assets));
    for (i, at) in flame_positions().into_iter().enumerate() {
        // Each flame flickers on its own; only the first (tallest) casts
        // shadows, which keeps the shadow pass over the heavy model cheap.
        let phase = i as f32 * 17.3;
        commands.spawn(flame(&mut meshes, &mut materials, at, phase));
        commands.spawn(flame_core(&mut meshes, &mut materials, at, phase));
        commands.spawn(candle_light(at, phase, i == 0));
    }
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

/// The glTF model is Z-up; its root node turns it Y-up, so a model point
/// (x, y, z) lands at (x, z, -y) before scaling.
fn model_to_world(x: f32, y: f32, z: f32) -> Vec3 {
    candle_base() + Vec3::new(x, z, -y) * CANDLE_SCALE
}

/// Where each flame burns: just above its wick or the wax rim around it.
fn flame_positions() -> Vec<Vec3> {
    WICKS
        .iter()
        .map(|&(x, y, wick, rim)| model_to_world(x, y, wick.max(rim - 0.02)) + Vec3::Y * FLAME_RADIUS * 1.6)
        .collect()
}

/// Three wax candles from `assets/candle` ("Three Wax Candles" by Rudy, CC-BY-4.0).
fn candle(assets: &AssetServer) -> impl Bundle {
    (
        WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(CANDLE_MODEL))),
        Transform::from_translation(candle_base()).with_scale(Vec3::splat(CANDLE_SCALE)),
    )
}

/// Outer flame: an emissive teardrop; with HDR and bloom it glows.
fn flame(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>, at: Vec3, phase: f32) -> impl Bundle {
    let (r, g, b) = palette::FLAME;
    let glow = StandardMaterial { emissive: LinearRgba::rgb(r, g, b), unlit: true, ..default() };
    let scale = Vec3::new(1.0, 2.2, 1.0);
    (
        Flame { base: at, scale, phase },
        Mesh3d(meshes.add(Sphere::new(FLAME_RADIUS).mesh().uv(24, 16))),
        MeshMaterial3d(materials.add(glow)),
        Transform::from_translation(at).with_scale(scale),
        NotShadowCaster,
    )
}

/// Hot, almost white core low in the flame.
fn flame_core(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>, at: Vec3, phase: f32) -> impl Bundle {
    let (r, g, b) = FLAME_CORE;
    let core = StandardMaterial { emissive: LinearRgba::rgb(r, g, b), unlit: true, ..default() };
    let base = at - Vec3::Y * FLAME_RADIUS * 0.5;
    let scale = Vec3::new(1.0, 1.6, 1.0) * FLAME_CORE_SCALE;
    (
        Flame { base, scale, phase },
        Mesh3d(meshes.add(Sphere::new(FLAME_RADIUS).mesh().uv(16, 12))),
        MeshMaterial3d(materials.add(core)),
        Transform::from_translation(base).with_scale(scale),
        NotShadowCaster,
    )
}

/// A flame mesh that sways and stretches with the flicker.
#[derive(Component, Clone, Copy, Debug)]
pub struct Flame {
    base: Vec3,
    scale: Vec3,
    phase: f32,
}

/// The candle light, remembered so it can flicker around its base values.
#[derive(Component, Clone, Copy, Debug)]
pub struct CandleLight {
    base: Vec3,
    phase: f32,
    intensity: f32,
}

/// Smooth pseudo-random flicker in about [-1, 1]: a few unrelated sines.
fn flicker(t: f32) -> f32 {
    let t = t * FLICKER_SPEED;
    0.5 * (t * 7.3).sin() + 0.3 * (t * 13.1 + 1.7).sin() + 0.2 * (t * 23.7 + 4.1).sin()
}

/// Makes the candle light waver in brightness and sway a little, and moves
/// the flame meshes with it, like a real flame in still air.
pub fn flicker_candle(
    time: Res<Time>,
    mut lights: Query<(&CandleLight, &mut PointLight, &mut Transform), Without<Flame>>,
    mut flames: Query<(&Flame, &mut Transform), Without<CandleLight>>,
) {
    let t = time.elapsed_secs();
    let sway = |t: f32| Vec3::new(flicker(t * 0.7 + 11.0), 0.0, flicker(t * 0.6 + 23.0)) * FLICKER_SWAY;
    for (light, mut point, mut transform) in &mut lights {
        let t = t + light.phase;
        point.intensity = light.intensity * (1.0 + FLICKER_AMOUNT * flicker(t));
        transform.translation = light.base + sway(t);
    }
    for (flame, mut transform) in &mut flames {
        let t = t + flame.phase;
        let stretch = 1.0 + 0.12 * flicker(t);
        transform.translation = flame.base + sway(t) * 1.5;
        transform.scale = flame.scale * Vec3::new(1.0 / stretch.sqrt(), stretch, 1.0 / stretch.sqrt());
    }
}

/// Warm point light with a radius, which gives soft shadow edges. The three
/// flames share the old single-candle brightness, the main one a bit more.
fn candle_light(at: Vec3, phase: f32, main: bool) -> impl Bundle {
    let intensity = LIGHT_INTENSITY * if main { 0.55 } else { 0.3 };
    let base = at + Vec3::Y * 0.05;
    (
        PointLight {
            color: srgb(LIGHT_COLOR),
            intensity,
            radius: LIGHT_RADIUS,
            range: 30.0,
            shadow_maps_enabled: main,
            ..default()
        },
        CandleLight { base, phase, intensity },
        Transform::from_translation(base),
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
