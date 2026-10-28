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

/// A leaf of the book in turning order; `index` 0 is the front cover.
#[derive(Component, Clone, Copy, Debug)]
pub struct Leaf {
    pub index: usize,
    pub kind: LeafKind,
}

/// A rigid leather board or a bending sheet of paper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeafKind {
    Cover,
    Paper,
}

/// Per-kind physical constants of a leaf.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicsParams {
    /// Gravity-like pull (rad/s²) toward the side the leaf leans to.
    pub gravity: f32,
    /// Air damping (1/s).
    pub damping: f32,
    /// Spring (1/s²) and damping (1/s) pulling a held leaf to the cursor.
    pub drag_stiffness: f32,
    pub drag_damping: f32,
    /// Bend lag per rad/s of speed, sag when lifted, and the bend limit.
    pub bend_per_speed: f32,
    pub droop: f32,
    pub max_bend: f32,
}

/// One half of the page block (the stacked paper on each side).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageBlock {
    Right,
    Left,
}

/// Shared materials of the book.
#[derive(Resource, Clone, Debug)]
pub struct BookMaterials {
    pub leather: Handle<StandardMaterial>,
    pub gold: Handle<StandardMaterial>,
    pub page_edge: Handle<StandardMaterial>,
    pub endpaper: Handle<StandardMaterial>,
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
