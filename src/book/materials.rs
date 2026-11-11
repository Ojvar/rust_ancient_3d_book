//! PBR materials of the tome, loaded once into [`BookMaterials`].

use bevy::{
    asset::RenderAssetUsages,
    image::ImageLoaderSettings,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::constants::{materials::*, textures};
use crate::types::BookMaterials;

pub fn create_materials(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(BookMaterials {
        leather: materials.add(leather(&assets)),
        gold: materials.add(gold()),
        page_edge: materials.add(textured(&assets, textures::PAGE_EDGE, PAPER_ROUGHNESS)),
        endpaper: materials.add(textured(&assets, textures::ENDPAPER, PAPER_ROUGHNESS)),
    });
}

/// Dark worn leather with a wrinkle normal map and a roughness map.
fn leather(assets: &AssetServer) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(assets.load(textures::LEATHER)),
        normal_map_texture: Some(load_linear(assets, textures::LEATHER_NORMAL)),
        metallic_roughness_texture: Some(load_linear(assets, textures::LEATHER_ROUGHNESS)),
        // The roughness map averages ~0.8; the factor just scales it.
        perceptual_roughness: 1.0,
        metallic: 0.0,
        clearcoat: LEATHER_CLEARCOAT,
        clearcoat_perceptual_roughness: LEATHER_CLEARCOAT_ROUGHNESS,
        ..default()
    }
}

fn gold() -> StandardMaterial {
    let (r, g, b) = GOLD;
    StandardMaterial {
        base_color: Color::srgb(r, g, b),
        metallic: 1.0,
        perceptual_roughness: GOLD_ROUGHNESS,
        ..default()
    }
}

pub fn textured(assets: &AssetServer, path: &'static str, roughness: f32) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(assets.load(path)),
        perceptual_roughness: roughness,
        ..default()
    }
}

/// Data textures (normals, roughness) must not be treated as sRGB colour.
fn load_linear(assets: &AssetServer, path: &'static str) -> Handle<Image> {
    assets
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
        .load(path)
}

/// A tangent-space normal map of paper fibres and fine tooth, made from
/// value noise so no extra asset is needed. Long thin streaks along the width
/// read as fibres; a fine octave gives the surface its grain.
pub fn paper_grain(size: u32, strength: f32) -> Image {
    let n = size as usize;
    let height: Vec<f32> = (0..n * n)
        .map(|i| {
            let (x, y) = ((i % n) as f32, (i / n) as f32);
            0.45 * value_noise(x / 3.0, y / 14.0, 1) // fibres: stretched along v
                + 0.35 * value_noise(x / 2.0, y / 2.0, 2) // tooth
                + 0.20 * value_noise(x / 9.0, y / 9.0, 3) // soft blotches
        })
        .collect();
    let at = |x: usize, y: usize| height[(y % n) * n + (x % n)];
    let data = (0..n * n)
        .flat_map(|i| {
            let (x, y) = (i % n, i / n);
            let dx = (at(x + 1, y) - at(x + n - 1, y)) * strength;
            let dy = (at(x, y + 1) - at(x, y + n - 1)) * strength;
            let normal = Vec3::new(-dx, -dy, 1.0).normalize();
            let byte = |v: f32| ((v * 0.5 + 0.5) * 255.0).round() as u8;
            [byte(normal.x), byte(normal.y), byte(normal.z), 255]
        })
        .collect();
    Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    )
}

/// Smooth 2D value noise in [0, 1].
fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
    let (xi, yi) = (x.floor() as i32, y.floor() as i32);
    let (fx, fy) = (x - xi as f32, y - yi as f32);
    let smooth = |t: f32| t * t * (3.0 - 2.0 * t);
    let (sx, sy) = (smooth(fx), smooth(fy));
    let h = |dx: i32, dy: i32| hash(xi + dx, yi + dy, seed);
    let top = h(0, 0) + (h(1, 0) - h(0, 0)) * sx;
    let bottom = h(0, 1) + (h(1, 1) - h(0, 1)) * sx;
    top + (bottom - top) * sy
}

fn hash(x: i32, y: i32, seed: u32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x8da6_b343)
        ^ (y as u32).wrapping_mul(0xd816_3841)
        ^ seed.wrapping_mul(0xcb1a_b31f);
    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1_e995);
    h ^= h >> 15;
    (h & 0xffff) as f32 / 65535.0
}
