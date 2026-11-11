//! Pure bend deformation of a subdivided page.
//!
//! A page is a strip of columns hinged at the spine (x = 0). Each column's
//! direction is the leaf angle minus a bend that grows toward the free edge,
//! so the paper curls behind the spine. Positions are integrated column by
//! column, which keeps the paper length constant (no stretching).

use bevy::prelude::*;

use std::f32::consts::{PI, TAU};

use crate::constants::{
    CORNER_CURL, GUTTER_ANGLE, GUTTER_SPAN, PAGE_HEIGHT, PAGE_WIDTH, PAPER_RIPPLE, SEGMENTS_X, SEGMENTS_Z,
};
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
        let direction = column_angle(params.angle, bend, column) + paper_shape(params.angle, column, z);
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

/// Extra tilt that gives a resting sheet its natural shape: an arch out of
/// the gutter, a gentle wave, and outer corners that lift a little. Every
/// term integrates to a height >= 0, so the sheet never dips into the block.
/// It is strongest when the leaf lies flat and fades out while it stands up
/// (cos of the angle is 1 on the right, -1 on the left, 0 when vertical).
fn paper_shape(angle: f32, column: usize, z: f32) -> f32 {
    let t = (column as f32 - 0.5) / SEGMENTS_X as f32;
    let across = 2.0 * z / PAGE_HEIGHT; // -1 at the top edge, 1 at the bottom
    let gutter = if t < GUTTER_SPAN { GUTTER_ANGLE * (PI * t / GUTTER_SPAN).cos() } else { 0.0 };
    let ripple = PAPER_RIPPLE * (0.65 + 0.35 * (1.7 * z + 0.4).sin()) * (TAU * 1.4 * t).sin();
    let curl = CORNER_CURL * t.powi(5) * (0.35 + across * across);
    angle.cos() * (gutter + ripple + curl)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row_length(p: &[[f32; 3]]) -> f32 {
        p.windows(2).map(|w| Vec3::from(w[0]).distance(Vec3::from(w[1]))).sum()
    }

    #[test]
    fn resting_page_keeps_its_width_and_stays_above_the_block() {
        let p = deform_page(BendParams { angle: 0.0, bend: 0.0, grab_z: 0.0 });
        for row in p.chunks(COLUMNS) {
            assert!((row_length(row) - PAGE_WIDTH).abs() < 1e-3);
            assert!(row.iter().all(|v| v[1] >= 0.0 && v[1] < 0.12));
            assert!(row[COLUMNS - 1][0] > PAGE_WIDTH * 0.97);
        }
    }

    #[test]
    fn shape_mirrors_on_the_left() {
        let right = deform_page(BendParams { angle: 0.0, bend: 0.0, grab_z: 0.0 });
        let left = deform_page(BendParams { angle: PI, bend: 0.0, grab_z: 0.0 });
        for (r, l) in right.iter().zip(&left) {
            assert!((r[0] + l[0]).abs() < 1e-3 && (r[1] - l[1]).abs() < 1e-3);
        }
    }
}
