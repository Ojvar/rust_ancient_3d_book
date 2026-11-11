//! A box with rounded corners and softly rounded edges, like a real board or
//! a cast metal fitting, instead of a razor-sharp cuboid.

use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology};

/// Arc segments per outline corner and per rounded edge.
const CORNER_SEGMENTS: usize = 8;
const EDGE_SEGMENTS: usize = 4;

/// `size` is (width x, thickness y, height z), centred on the origin.
/// `corner` rounds the outline seen from above; `edge` rounds the rims.
pub fn rounded_slab(size: Vec3, corner: f32, edge: f32) -> Mesh {
    let half = size / 2.0;
    let corner = corner.min(half.x).min(half.z);
    let edge = edge.min(half.y).min(corner);
    let outline = outline_directions();
    let ring_len = outline.len();

    // Profile around the rim: bottom edge (-90..0 deg), then top edge (0..90).
    let profile: Vec<(f32, f32)> = (0..=EDGE_SEGMENTS)
        .map(|i| -FRAC_PI_2 + FRAC_PI_2 * i as f32 / EDGE_SEGMENTS as f32)
        .map(|phi| (phi, -(half.y - edge) + edge * phi.sin()))
        .chain(
            (0..=EDGE_SEGMENTS)
                .map(|i| FRAC_PI_2 * i as f32 / EDGE_SEGMENTS as f32)
                .map(|phi| (phi, (half.y - edge) + edge * phi.sin())),
        )
        .collect();

    let (mut positions, mut normals) = (Vec::new(), Vec::new());
    for &(phi, y) in &profile {
        let inset = edge * (1.0 - phi.cos());
        for &(center, dir) in &outline {
            let c = Vec2::new(center.x * (half.x - corner), center.y * (half.z - corner));
            let p = c + dir * (corner - inset);
            positions.push([p.x, y, p.y]);
            normals.push([dir.x * phi.cos(), phi.sin(), dir.y * phi.cos()]);
        }
    }
    let mut indices = Vec::new();
    for ring in 0..profile.len() - 1 {
        for i in 0..ring_len {
            let j = (i + 1) % ring_len;
            let (a, b) = ((ring * ring_len + i) as u32, (ring * ring_len + j) as u32);
            let (c, d) = (a + ring_len as u32, b + ring_len as u32);
            indices.extend([a, b, c, b, d, c]);
        }
    }
    // Flat caps: a fan from the centre to the innermost bottom and top rings.
    for (ring, y, ny) in [(0, -half.y, -1.0), (profile.len() - 1, half.y, 1.0)] {
        let center = positions.len() as u32;
        positions.push([0.0, y, 0.0]);
        normals.push([0.0, ny, 0.0]);
        for i in 0..ring_len {
            let j = (i + 1) % ring_len;
            indices.extend([center, (ring * ring_len + i) as u32, (ring * ring_len + j) as u32]);
        }
    }
    face_outward(&positions, &normals, &mut indices);

    let uvs: Vec<[f32; 2]> = positions.iter().map(|p| [p[0] / size.x + 0.5, p[2] / size.z + 0.5]).collect();
    let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices));
    match mesh.clone().with_generated_tangents() {
        Ok(with_tangents) => with_tangents,
        Err(error) => {
            warn!("rounded slab tangents: {error}");
            mesh
        }
    }
}

/// Points around a rounded rectangle: (which corner as +-1 signs, outward direction).
fn outline_directions() -> Vec<(Vec2, Vec2)> {
    let corners = [(Vec2::new(1.0, 1.0), 0.0), (Vec2::new(-1.0, 1.0), FRAC_PI_2), (Vec2::new(-1.0, -1.0), PI), (Vec2::new(1.0, -1.0), 1.5 * PI)];
    corners
        .iter()
        .flat_map(|&(center, start)| {
            (0..=CORNER_SEGMENTS).map(move |i| {
                let theta = start + FRAC_PI_2 * i as f32 / CORNER_SEGMENTS as f32;
                (center, Vec2::new(theta.cos(), theta.sin()))
            })
        })
        .collect()
}

/// Makes every triangle wind counter-clockwise when seen from outside.
fn face_outward(positions: &[[f32; 3]], normals: &[[f32; 3]], indices: &mut [u32]) {
    for tri in indices.chunks_mut(3) {
        let [a, b, c] = [tri[0], tri[1], tri[2]].map(|i| Vec3::from(positions[i as usize]));
        let normal = [tri[0], tri[1], tri[2]].map(|i| Vec3::from(normals[i as usize])).iter().sum::<Vec3>();
        if (b - a).cross(c - a).dot(normal) < 0.0 {
            tri.swap(1, 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::mesh::VertexAttributeValues;

    #[test]
    fn slab_fits_its_size() {
        let mesh = rounded_slab(Vec3::new(2.0, 0.1, 3.0), 0.05, 0.03);
        let Some(VertexAttributeValues::Float32x3(p)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else { panic!() };
        let max = p.iter().fold(Vec3::splat(f32::MIN), |m, v| m.max(Vec3::from(*v)));
        assert!((max - Vec3::new(1.0, 0.05, 1.5)).abs().max_element() < 1e-4);
    }
}
