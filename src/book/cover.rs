//! Leather boards, rounded spine and the gold ornaments of the front cover.

use bevy::prelude::*;

use super::{leaf_height, rounded::rounded_slab};
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
    for bundle in spine_bands(&mut meshes, &materials) {
        commands.spawn(bundle);
    }
}

fn closed_cover() -> LeafState {
    LeafState { angle: 0.0, velocity: 0.0, bend: 0.0, grab_z: 0.0, mode: LeafMode::Resting }
}

/// A board with rounded corners and worn, rounded rims (with tangents for
/// the leather normal map).
fn board_mesh() -> Mesh {
    rounded_slab(Vec3::new(BOARD_WIDTH, THICKNESS, BOARD_HEIGHT), BOARD_CORNER, BOARD_EDGE)
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

/// One piece of the spine: the leather body or a band/gold line at `z`.
/// The spine always wraps the book's back edge, from under the back board to
/// the top of the front board, so it is re-fitted whenever the cover moves.
#[derive(Component, Clone, Copy, Debug)]
pub struct SpinePart {
    z: f32,
    body: bool,
}

fn spine(meshes: &mut Assets<Mesh>, materials: &BookMaterials) -> impl Bundle {
    let part = SpinePart { z: 0.0, body: true };
    (
        part,
        Mesh3d(meshes.add(Cylinder::new(1.0, BOARD_HEIGHT))),
        MeshMaterial3d(materials.leather.clone()),
        spine_transform(part, closed_cover_height()),
    )
}

/// Raised leather bands around the spine, each framed by two gold lines.
fn spine_bands(meshes: &mut Assets<Mesh>, materials: &BookMaterials) -> Vec<impl Bundle> {
    // Unit tori, scaled to the spine's oval; the tube is sized for the
    // spine's depth so it stays round where it shows.
    let band = meshes.add(Torus { minor_radius: SPINE_BAND_RADIUS / SPINE_RADIUS, major_radius: 1.0 });
    let line = meshes.add(Torus { minor_radius: SPINE_LINE_RADIUS / SPINE_RADIUS, major_radius: 1.0 });
    let piece = |mesh: &Handle<Mesh>, material: &Handle<StandardMaterial>, z: f32| {
        let part = SpinePart { z, body: false };
        (part, Mesh3d(mesh.clone()), MeshMaterial3d(material.clone()), spine_transform(part, closed_cover_height()))
    };
    SPINE_BANDS
        .iter()
        .flat_map(|&z| {
            [
                piece(&band, &materials.leather, z),
                piece(&line, &materials.gold, z - SPINE_LINE_OFFSET),
                piece(&line, &materials.gold, z + SPINE_LINE_OFFSET),
            ]
        })
        .collect()
}

fn closed_cover_height() -> f32 {
    leaf_height(&Leaf { index: COVER_INDEX, kind: LeafKind::Cover }, 0.0)
}

/// An oval tube along z at the hinge (x = 0), reaching from the bottom of the
/// back board to the top of the front board, `SPINE_RADIUS` deep.
fn spine_transform(part: SpinePart, cover_height: f32) -> Transform {
    let (bottom, top) = (-THICKNESS, cover_height + THICKNESS);
    let half = (top - bottom) / 2.0 + SPINE_MARGIN;
    // The tube's axis is local y; after the quarter turn local z points down.
    let scale = if part.body {
        Vec3::new(SPINE_RADIUS, 1.0, half)
    } else {
        Vec3::new(SPINE_RADIUS, SPINE_RADIUS, half)
    };
    Transform::from_xyz(0.0, (top + bottom) / 2.0, part.z)
        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
        .with_scale(scale)
}

/// Keeps the spine wrapped around the book as the front cover opens or closes.
pub fn fit_spine(
    covers: Query<(&Leaf, &LeafState), Changed<LeafState>>,
    mut parts: Query<(&SpinePart, &mut Transform)>,
) {
    let Some((leaf, state)) = covers.iter().find(|(leaf, _)| leaf.kind == LeafKind::Cover) else { return };
    let height = leaf_height(leaf, state.angle);
    for (part, mut transform) in &mut parts {
        *transform = spine_transform(*part, height);
    }
}

/// Gold fittings with softened edges, like cast and polished brass.
fn gold_part(meshes: &mut Assets<Mesh>, materials: &BookMaterials, part: Part) -> impl Bundle {
    let corner = part.size.x.min(part.size.z) * 0.3;
    let edge = (part.size.y * 0.45).min(corner);
    (
        Mesh3d(meshes.add(rounded_slab(part.size, corner, edge))),
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
