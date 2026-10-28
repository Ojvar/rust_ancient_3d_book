//! Tunable values for the book, its pages and the page physics.

use std::f32::consts::PI;

/// Number of printed pages. Each leaf (sheet) carries two pages.
pub const PAGE_COUNT: usize = 10;
pub const LEAF_COUNT: usize = PAGE_COUNT / 2;

/// Page size in world units (x = width from the spine, z = height).
pub const PAGE_WIDTH: f32 = 2.0;
pub const PAGE_HEIGHT: f32 = 2.8;

/// Mesh subdivision. More columns = smoother bend.
pub const SEGMENTS_X: usize = 32;
pub const SEGMENTS_Z: usize = 12;

/// Vertical gap between stacked leaves, avoids z-fighting.
pub const LEAF_GAP: f32 = 0.006;

/// Angles: a leaf lies on the right at 0 and on the left at PI.
pub const ANGLE_RIGHT: f32 = 0.0;
pub const ANGLE_LEFT: f32 = PI;

/// Folder (under `assets/`) and file pattern of the page images.
pub const PAGE_IMAGE_DIR: &str = "pages";

pub mod physics {
    /// Largest frame time accepted, so a hiccup cannot explode the simulation.
    pub const MAX_DT: f32 = 1.0 / 30.0;
    /// Fixed sub-step size for the integrator.
    pub const SUB_STEP: f32 = 1.0 / 240.0;
    /// Gravity-like pull (rad/s²) toward the side the page leans to.
    pub const GRAVITY: f32 = 9.0;
    /// Air damping (1/s) applied to the angular velocity.
    pub const DAMPING: f32 = 1.6;
    /// Spring stiffness (1/s²) pulling a dragged page toward the cursor.
    pub const DRAG_STIFFNESS: f32 = 260.0;
    /// Critical-ish damping of the drag spring (1/s).
    pub const DRAG_DAMPING: f32 = 26.0;
    /// Fraction of speed kept when a page hits the stack.
    pub const RESTITUTION: f32 = 0.18;
    /// Below this speed (rad/s) a page on the stack comes to rest.
    pub const REST_SPEED: f32 = 0.15;
    /// How much the paper lags behind (bends) per rad/s of angular speed.
    pub const BEND_PER_SPEED: f32 = 0.09;
    /// Sag of a lifted page under its own weight (rad at 45°).
    pub const DROOP: f32 = 0.3;
    /// Largest bend in radians.
    pub const MAX_BEND: f32 = 1.2;
    /// How fast the bend follows its target (1/s).
    pub const BEND_RESPONSE: f32 = 14.0;
    /// Extra lag of rows far from the grab point (corner curl).
    pub const CORNER_TWIST: f32 = 0.8;
}

pub mod camera {
    pub const DISTANCE: f32 = 6.5;
    pub const PITCH: f32 = 1.05;
    pub const ORBIT_SPEED: f32 = 0.005;
    pub const ZOOM_SPEED: f32 = 0.4;
    pub const MIN_DISTANCE: f32 = 3.0;
    pub const MAX_DISTANCE: f32 = 12.0;
    pub const MIN_PITCH: f32 = 0.2;
    pub const MAX_PITCH: f32 = 1.5;
}
