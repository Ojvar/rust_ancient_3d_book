//! Shared named types (components, resources and plain data).

use bevy::prelude::*;

/// What is currently moving a leaf.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeafMode {
    /// Lying on one of the stacks.
    Resting,
    /// Held by the mouse. `target` is the angle under the cursor,
    /// `grab_z` is where (along the page height) it was grabbed.
    Dragged { target: f32, grab_z: f32 },
    /// Released; gravity, damping and momentum move it.
    Free,
}

/// Physical state of one leaf rotating around the spine.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct LeafState {
    pub angle: f32,
    pub velocity: f32,
    /// Current paper bend in radians (0 = flat).
    pub bend: f32,
    /// Grab position along z, kept after release for the corner curl.
    pub grab_z: f32,
    pub mode: LeafMode,
}

/// A leaf (sheet) of the book; `index` 0 is the first sheet.
#[derive(Component, Clone, Copy, Debug)]
pub struct Leaf {
    pub index: usize,
}

/// Which side of a leaf a child mesh renders.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeafFace {
    Front,
    Back,
}

/// The leaf currently held by the mouse, plus the page fraction grabbed.
#[derive(Resource, Default, Debug)]
pub struct DragState {
    pub leaf: Option<Entity>,
    /// Distance from the spine of the grab point, as a fraction of page width.
    pub grab_fraction: f32,
}

/// Orbit camera parameters.
#[derive(Component, Clone, Copy, Debug)]
pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

/// Inputs to the bend deformation of one leaf.
#[derive(Clone, Copy, Debug)]
pub struct BendParams {
    pub angle: f32,
    pub bend: f32,
    pub grab_z: f32,
}
