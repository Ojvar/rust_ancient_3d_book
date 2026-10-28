//! Builds and updates the grid meshes of a leaf's two faces.

use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, VertexAttributeValues},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};

use super::deformation::{COLUMNS, ROWS, deform_page};
use crate::types::{BendParams, LeafFace};

/// A flat page mesh for one face; kept in the main world so it can bend.
pub fn build_face_mesh(face: LeafFace) -> Mesh {
    let flat = BendParams { angle: 0.0, bend: 0.0, grab_z: 0.0 };
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, deform_page(flat))
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, face_uvs(face))
    .with_inserted_indices(Indices::U32(face_indices(face)));
    mesh.compute_normals();
    mesh
}

/// Writes new positions into a face mesh and refreshes its normals.
pub fn apply_positions(mesh: &mut Mesh, positions: Vec<[f32; 3]>) {
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.compute_normals();
}

/// The back is seen from below, so its image is mirrored along the width.
fn face_uvs(face: LeafFace) -> VertexAttributeValues {
    let uv = |row: usize, column: usize| {
        let u = column as f32 / (COLUMNS - 1) as f32;
        let v = row as f32 / (ROWS - 1) as f32;
        match face {
            LeafFace::Front => [u, v],
            LeafFace::Back => [1.0 - u, v],
        }
    };
    let uvs: Vec<[f32; 2]> = (0..ROWS)
        .flat_map(|row| (0..COLUMNS).map(move |column| uv(row, column)))
        .collect();
    uvs.into()
}

/// Two triangles per grid cell, facing up (front) or down (back).
fn face_indices(face: LeafFace) -> Vec<u32> {
    let cells = (0..ROWS - 1).flat_map(|row| (0..COLUMNS - 1).map(move |column| (row, column)));
    cells
        .flat_map(|(row, column)| cell_triangles(row, column))
        .flat_map(|[a, b, c]| match face {
            LeafFace::Front => [a, b, c],
            LeafFace::Back => [a, c, b],
        })
        .collect()
}

fn cell_triangles(row: usize, column: usize) -> [[u32; 3]; 2] {
    let at = |r: usize, c: usize| (r * COLUMNS + c) as u32;
    let (a, right) = (at(row, column), at(row, column + 1));
    let (down, diagonal) = (at(row + 1, column), at(row + 1, column + 1));
    [[a, down, right], [right, down, diagonal]]
}
