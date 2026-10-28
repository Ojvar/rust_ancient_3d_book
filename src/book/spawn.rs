//! Spawns the cover and the leaves with their page images.

use bevy::prelude::*;

use super::{mesh::build_face_mesh, stack_height};
use crate::constants::{LEAF_COUNT, LEAF_GAP, PAGE_HEIGHT, PAGE_IMAGE_DIR, PAGE_WIDTH};
use crate::types::{Leaf, LeafFace, LeafMode, LeafState};

const COVER_MARGIN: f32 = 0.08;
const COVER_THICKNESS: f32 = 0.06;

pub fn spawn_book(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    spawn_cover(&mut commands, &mut meshes, &mut materials);
    for index in 0..LEAF_COUNT {
        let mut face = |face: LeafFace| {
            let image = assets.load(page_image_path(page_number(index, face)));
            (
                Mesh3d(meshes.add(build_face_mesh(face))),
                MeshMaterial3d(materials.add(page_material(image))),
                face,
            )
        };
        let (front, back) = (face(LeafFace::Front), face(LeafFace::Back));
        commands.spawn((
            Leaf { index },
            resting_leaf(),
            Transform::from_xyz(0.0, stack_height(index, 0.0), 0.0),
            Visibility::default(),
            children![front, back],
        ));
    }
}

/// Page numbers start at 1: leaf 0 holds pages 1 (front) and 2 (back).
fn page_number(leaf: usize, face: LeafFace) -> usize {
    match face {
        LeafFace::Front => leaf * 2 + 1,
        LeafFace::Back => leaf * 2 + 2,
    }
}

fn page_image_path(page: usize) -> String {
    format!("{PAGE_IMAGE_DIR}/page-{page:02}.png")
}

fn page_material(image: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(image),
        perceptual_roughness: 0.9,
        ..default()
    }
}

fn resting_leaf() -> LeafState {
    LeafState { angle: 0.0, velocity: 0.0, bend: 0.0, grab_z: 0.0, mode: LeafMode::Resting }
}

fn spawn_cover(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let size = Vec3::new(
        2.0 * (PAGE_WIDTH + COVER_MARGIN),
        COVER_THICKNESS,
        PAGE_HEIGHT + 2.0 * COVER_MARGIN,
    );
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(size))),
        MeshMaterial3d(materials.add(Color::srgb(0.45, 0.12, 0.1))),
        Transform::from_xyz(0.0, -COVER_THICKNESS / 2.0 - LEAF_GAP * 0.5, 0.0),
    ));
}
