//! PBR materials of the tome, loaded once into [`BookMaterials`].

use bevy::{image::ImageLoaderSettings, prelude::*};

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
