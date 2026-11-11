//! Spawns the cover and the leaves with their page images.

use bevy::prelude::*;

use super::{leaf_height, materials::paper_grain, mesh::build_face_mesh};
use crate::constants::{
    COVER_INDEX, PAPER_GRAIN, PAPER_GRAIN_SIZE, PAPER_LEAF_COUNT, PAPER_TRANSLUCENCY, materials::PAPER_ROUGHNESS,
    textures::PAGE_DIR,
};
use crate::types::{Leaf, LeafFace, LeafKind, LeafMode, LeafState};

/// Spawns the paper leaves, each with a front and a back page.
pub fn spawn_pages(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    assets: Res<AssetServer>,
) {
    let grain = images.add(paper_grain(PAPER_GRAIN_SIZE, PAPER_GRAIN));
    for sheet in 0..PAPER_LEAF_COUNT {
        let leaf = Leaf { index: COVER_INDEX + 1 + sheet, kind: LeafKind::Paper };
        let mut face = |face: LeafFace| {
            let image = assets.load(page_image_path(page_number(sheet, face)));
            (
                Mesh3d(meshes.add(build_face_mesh(face))),
                MeshMaterial3d(materials.add(page_material(image, grain.clone()))),
                face,
            )
        };
        let (front, back) = (face(LeafFace::Front), face(LeafFace::Back));
        commands.spawn((
            leaf,
            resting_leaf(),
            Transform::from_xyz(0.0, leaf_height(&leaf, 0.0), 0.0),
            Visibility::default(),
            children![front, back],
        ));
    }
}

/// Page numbers start at 1: sheet 0 holds pages 1 (front) and 2 (back).
fn page_number(sheet: usize, face: LeafFace) -> usize {
    match face {
        LeafFace::Front => sheet * 2 + 1,
        LeafFace::Back => sheet * 2 + 2,
    }
}

fn page_image_path(page: usize) -> String {
    format!("{PAGE_DIR}/page-{page:02}.png")
}

/// Matte paper with a fibre grain; a little light shines through the sheet.
fn page_material(image: Handle<Image>, grain: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(image),
        normal_map_texture: Some(grain),
        perceptual_roughness: PAPER_ROUGHNESS,
        reflectance: 0.3,
        diffuse_transmission: PAPER_TRANSLUCENCY,
        thickness: 0.002,
        ..default()
    }
}

fn resting_leaf() -> LeafState {
    LeafState { angle: 0.0, velocity: 0.0, bend: 0.0, grab_z: 0.0, mode: LeafMode::Resting }
}
