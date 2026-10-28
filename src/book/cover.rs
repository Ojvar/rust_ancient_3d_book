//! Leather boards, rounded spine and the gold ornaments of the front cover.

use bevy::prelude::*;

use super::leaf_height;
use crate::constants::{COVER_INDEX, cover::*};
use crate::types::{BookMaterials, Leaf, LeafKind, LeafMode, LeafState};

/// One gold or leather box: size and centre, in board space.
struct Part {
    size: Vec3,
    center: Vec3,
}

pub fn spawn_cover(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<BookMaterials>,
) {
    let leaf = Leaf { index: COVER_INDEX, kind: LeafKind::Cover };
    let board = meshes.add(board_mesh());
    commands
        .spawn((
            leaf,
            closed_cover(),
            Transform::from_xyz(0.0, leaf_height(&leaf, 0.0), 0.0),
            Visibility::default(),
        ))
        .with_children(|cover| {
            cover.spawn((
                Mesh3d(board.clone()),
                MeshMaterial3d(materials.leather.clone()),
                Transform::from_translation(board_center(1.0)),
            ));
            cover.spawn(endpaper(&mut meshes, &materials));
            cover.spawn(emblem(&mut meshes, &materials));
            for part in ornaments() {
                cover.spawn(gold_part(&mut meshes, &materials, part));
            }
        });
    commands.spawn((
        Mesh3d(board),
        MeshMaterial3d(materials.leather.clone()),
        Transform::from_translation(board_center(-1.0)),
    ));
    commands.spawn(spine(&mut meshes, &materials));
}

fn closed_cover() -> LeafState {
    LeafState { angle: 0.0, velocity: 0.0, bend: 0.0, grab_z: 0.0, mode: LeafMode::Resting }
}

/// Boards need tangents for the leather normal map.
fn board_mesh() -> Mesh {
    Mesh::from(Cuboid::new(BOARD_WIDTH, THICKNESS, BOARD_HEIGHT))
        .with_generated_tangents()
        .expect("cuboid has normals and UVs")
}

/// `side` = 1 puts the board above its hinge (front), -1 below it (back).
fn board_center(side: f32) -> Vec3 {
    Vec3::new(BOARD_WIDTH / 2.0, side * THICKNESS / 2.0, 0.0)
}

/// Parchment glued inside the front board; seen when the cover is open.
fn endpaper(meshes: &mut Assets<Mesh>, materials: &BookMaterials) -> impl Bundle {
    let half = Vec2::new(BOARD_WIDTH - OVERHANG, BOARD_HEIGHT - 2.0 * OVERHANG) / 2.0;
    (
        Mesh3d(meshes.add(Plane3d::new(Vec3::NEG_Y, half))),
        MeshMaterial3d(materials.endpaper.clone()),
        Transform::from_xyz(half.x + OVERHANG * 0.5, -0.001, 0.0),
    )
}

fn spine(meshes: &mut Assets<Mesh>, materials: &BookMaterials) -> impl Bundle {
    (
        Mesh3d(meshes.add(Cylinder::new(SPINE_RADIUS, BOARD_HEIGHT))),
        MeshMaterial3d(materials.leather.clone()),
        Transform::from_xyz(0.0, -THICKNESS * 0.75, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    )
}

fn gold_part(meshes: &mut Assets<Mesh>, materials: &BookMaterials, part: Part) -> impl Bundle {
    (
        Mesh3d(meshes.add(Cuboid::from_size(part.size))),
        MeshMaterial3d(materials.gold.clone()),
        Transform::from_translation(part.center),
    )
}

/// Frame, corner guards, cross and clasps, raised on the outer face.
fn ornaments() -> Vec<Part> {
    [frame(), corners(), cross(), clasps()].into_iter().flatten().collect()
}

/// Height of a relief part's centre above the outer face.
fn relief_y(height: f32) -> f32 {
    THICKNESS + height / 2.0
}

fn frame() -> Vec<Part> {
    let (w, h) = (BOARD_WIDTH - 2.0 * FRAME_INSET, BOARD_HEIGHT - 2.0 * FRAME_INSET);
    let cx = BOARD_WIDTH / 2.0;
    let y = relief_y(RELIEF);
    vec![
        Part { size: Vec3::new(w, RELIEF, FRAME_WIDTH), center: Vec3::new(cx, y, -h / 2.0) },
        Part { size: Vec3::new(w, RELIEF, FRAME_WIDTH), center: Vec3::new(cx, y, h / 2.0) },
        Part { size: Vec3::new(FRAME_WIDTH, RELIEF, h), center: Vec3::new(cx - w / 2.0, y, 0.0) },
        Part { size: Vec3::new(FRAME_WIDTH, RELIEF, h), center: Vec3::new(cx + w / 2.0, y, 0.0) },
    ]
}

/// Metal corner protectors on the two free corners of the board.
fn corners() -> Vec<Part> {
    let x = BOARD_WIDTH - CORNER_SIZE / 2.0;
    let z = BOARD_HEIGHT / 2.0 - CORNER_SIZE / 2.0;
    let size = Vec3::new(CORNER_SIZE, RELIEF, CORNER_SIZE);
    [-z, z].map(|z| Part { size, center: Vec3::new(x, relief_y(RELIEF), z) }).into()
}

fn cross() -> Vec<Part> {
    let center = Vec3::new(BOARD_WIDTH / 2.0, relief_y(RELIEF * 1.5), 0.0);
    vec![
        Part { size: Vec3::new(CROSS_WIDTH, RELIEF * 1.5, CROSS_LENGTH), center },
        Part {
            size: Vec3::new(CROSS_LENGTH * 0.65, RELIEF * 1.5, CROSS_WIDTH),
            center: center - Vec3::Z * CROSS_LENGTH * 0.15,
        },
    ]
}

/// Clasps wrap around the free edge of the board.
fn clasps() -> Vec<Part> {
    let size = Vec3::new(CLASP_LENGTH, THICKNESS + 2.0 * RELIEF, CLASP_WIDTH);
    let x = BOARD_WIDTH - CLASP_LENGTH / 2.0 + RELIEF;
    CLASP_OFFSETS.map(|z| Part { size, center: Vec3::new(x, THICKNESS / 2.0, z) }).into()
}

/// The ring of the central emblem, around the cross.
fn emblem(meshes: &mut Assets<Mesh>, materials: &BookMaterials) -> impl Bundle {
    let ring = Torus { minor_radius: EMBLEM_TUBE, major_radius: EMBLEM_RADIUS };
    (
        Mesh3d(meshes.add(ring)),
        MeshMaterial3d(materials.gold.clone()),
        Transform::from_xyz(BOARD_WIDTH / 2.0, THICKNESS + EMBLEM_TUBE * 0.4, 0.0),
    )
}
