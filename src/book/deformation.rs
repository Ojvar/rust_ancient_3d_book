//! Pure bend deformation of a subdivided page.
//!
//! A page is a strip of columns hinged at the spine (x = 0). Each column's
//! direction is the leaf angle minus a bend that grows toward the free edge,
//! so the paper curls behind the spine. Positions are integrated column by
//! column, which keeps the paper length constant (no stretching).

use bevy::prelude::*;

use crate::constants::{PAGE_HEIGHT, PAGE_WIDTH, SEGMENTS_X, SEGMENTS_Z};
use crate::constants::physics::CORNER_TWIST;
use crate::types::BendParams;

/// Number of vertices in one row and in one column of the grid.
pub const COLUMNS: usize = SEGMENTS_X + 1;
pub const ROWS: usize = SEGMENTS_Z + 1;

/// Vertex positions (row-major: row = z, column = distance from spine).
pub fn deform_page(params: BendParams) -> Vec<[f32; 3]> {
    (0..ROWS).flat_map(|row| deform_row(params, row_z(row))).collect()
}

/// z coordinate of a row, from the top edge (-H/2) to the bottom (+H/2).
fn row_z(row: usize) -> f32 {
    (row as f32 / SEGMENTS_Z as f32 - 0.5) * PAGE_HEIGHT
}

fn deform_row(params: BendParams, z: f32) -> Vec<[f32; 3]> {
    let bend = params.bend * corner_factor(z, params.grab_z);
    let step = PAGE_WIDTH / SEGMENTS_X as f32;
    let points = (1..COLUMNS).scan(Vec2::ZERO, |point, column| {
        let direction = column_angle(params.angle, bend, column);
        *point += Vec2::from_angle(direction) * step;
        Some(*point)
    });
    std::iter::once(Vec2::ZERO)
        .chain(points)
        .map(|p| [p.x, p.y.max(0.0), z])
        .collect()
}

/// Rows far from the grab point lag more, which curls the free corner.
fn corner_factor(z: f32, grab_z: f32) -> f32 {
    1.0 + CORNER_TWIST * (z - grab_z).abs() / PAGE_HEIGHT
}

/// Direction of one segment: quadratic falloff of the bend from the spine.
fn column_angle(angle: f32, bend: f32, column: usize) -> f32 {
    let t = (column as f32 - 0.5) / SEGMENTS_X as f32;
    angle - bend * t * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_page_has_full_width() {
        let p = deform_page(BendParams { angle: 0.0, bend: 0.0, grab_z: 0.0 });
        let edge = p[COLUMNS - 1];
        assert!((edge[0] - PAGE_WIDTH).abs() < 1e-4);
        assert!(edge[1].abs() < 1e-6);
    }
}
