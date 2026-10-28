//! Tunable values: sizes, physics, materials, lights and camera.

use std::f32::consts::PI;

use crate::types::PhysicsParams;

/// Number of printed pages. Each paper leaf carries two pages.
pub const PAGE_COUNT: usize = 10;
pub const PAPER_LEAF_COUNT: usize = PAGE_COUNT / 2;
/// Leaf index of the front cover; paper leaves follow it (1..=5).
pub const COVER_INDEX: usize = 0;

/// Page size in world units (x = width from the spine, z = height).
pub const PAGE_WIDTH: f32 = 2.0;
pub const PAGE_HEIGHT: f32 = 2.8;

/// Mesh subdivision. More columns = smoother bend.
pub const SEGMENTS_X: usize = 32;
pub const SEGMENTS_Z: usize = 12;

/// Height of a loose leaf above the page block below it (avoids z-fighting).
pub const LEAF_GAP: f32 = 0.004;
/// Total thickness of the page block (all the paper of the tome).
pub const BLOCK_THICKNESS: f32 = 0.32;
/// The block is slightly smaller than a page, so the loose leaf covers it.
pub const BLOCK_INSET: f32 = 0.985;

/// Angles: a leaf lies on the right at 0 and on the left at PI.
pub const ANGLE_RIGHT: f32 = 0.0;
pub const ANGLE_LEFT: f32 = PI;

pub mod cover {
    use super::{PAGE_HEIGHT, PAGE_WIDTH};

    pub const THICKNESS: f32 = 0.07;
    /// How far the boards stick out past the pages.
    pub const OVERHANG: f32 = 0.09;
    pub const BOARD_WIDTH: f32 = PAGE_WIDTH + OVERHANG;
    pub const BOARD_HEIGHT: f32 = PAGE_HEIGHT + 2.0 * OVERHANG;
    pub const SPINE_RADIUS: f32 = 0.14;

    /// Raised gold frame on the front board.
    pub const FRAME_INSET: f32 = 0.14;
    pub const FRAME_WIDTH: f32 = 0.035;
    pub const RELIEF: f32 = 0.014;
    pub const CORNER_SIZE: f32 = 0.24;
    pub const EMBLEM_RADIUS: f32 = 0.36;
    pub const EMBLEM_TUBE: f32 = 0.028;
    pub const CROSS_LENGTH: f32 = 0.62;
    pub const CROSS_WIDTH: f32 = 0.07;
    pub const CLASP_OFFSETS: [f32; 2] = [-0.75, 0.75];
    pub const CLASP_LENGTH: f32 = 0.2;
    pub const CLASP_WIDTH: f32 = 0.13;
}

pub mod textures {
    pub const PAGE_DIR: &str = "pages";
    pub const LEATHER: &str = "textures/leather.png";
    pub const LEATHER_NORMAL: &str = "textures/leather-normal.png";
    pub const LEATHER_ROUGHNESS: &str = "textures/leather-roughness.png";
    pub const WOOD: &str = "textures/wood.png";
    pub const PAGE_EDGE: &str = "textures/page-edge.png";
    pub const ENDPAPER: &str = "textures/endpaper.png";
}

pub mod materials {
    /// sRGB colours.
    pub const GOLD: (f32, f32, f32) = (0.86, 0.64, 0.27);
    pub const GOLD_ROUGHNESS: f32 = 0.32;
    pub const PAPER_ROUGHNESS: f32 = 0.95;
    pub const WOOD_ROUGHNESS: f32 = 0.7;
    pub const WAX: (f32, f32, f32) = (0.93, 0.88, 0.74);
    /// Linear emissive colour of the flame (bright, so it blooms).
    pub const FLAME: (f32, f32, f32) = (18.0, 7.0, 1.5);
}

pub mod environment {
    pub const TABLE_SIZE: (f32, f32, f32) = (14.0, 0.3, 9.0);
    pub const CANDLE_POSITION: (f32, f32) = (-3.4, -1.9);
    pub const CANDLE_HEIGHT: f32 = 1.0;
    pub const CANDLE_RADIUS: f32 = 0.13;
    pub const FLAME_RADIUS: f32 = 0.05;
    pub const LIGHT_COLOR: (f32, f32, f32) = (1.0, 0.68, 0.38);
    pub const LIGHT_INTENSITY: f32 = 40_000.0;
    pub const LIGHT_RADIUS: f32 = 0.12;
    pub const FILL_ILLUMINANCE: f32 = 450.0;
    pub const AMBIENT_BRIGHTNESS: f32 = 60.0;
    pub const BACKGROUND: (f32, f32, f32) = (0.02, 0.015, 0.01);
}

pub mod physics {
    /// Largest frame time accepted, so a hiccup cannot explode the simulation.
    pub const MAX_DT: f32 = 1.0 / 30.0;
    /// Fixed sub-step size for the integrator.
    pub const SUB_STEP: f32 = 1.0 / 240.0;
    /// Fraction of speed kept when a leaf hits the stack.
    pub const RESTITUTION: f32 = 0.18;
    /// Below this speed (rad/s) a leaf on the stack comes to rest.
    pub const REST_SPEED: f32 = 0.15;
    /// How fast the bend follows its target (1/s).
    pub const BEND_RESPONSE: f32 = 14.0;
    /// Extra lag of rows far from the grab point (corner curl).
    pub const CORNER_TWIST: f32 = 0.6;
}

/// Old, slightly stiff paper: follows the hand, lags and sags a little.
pub const PAPER_PHYSICS: PhysicsParams = PhysicsParams {
    gravity: 9.0,
    damping: 1.8,
    drag_stiffness: 260.0,
    drag_damping: 26.0,
    bend_per_speed: 0.06,
    droop: 0.16,
    max_bend: 0.9,
};

/// Heavy board: slower to follow, stiffer, does not bend.
pub const COVER_PHYSICS: PhysicsParams = PhysicsParams {
    gravity: 6.0,
    damping: 2.6,
    drag_stiffness: 90.0,
    drag_damping: 18.0,
    bend_per_speed: 0.0,
    droop: 0.0,
    max_bend: 0.0,
};

pub mod camera {
    pub const DISTANCE: f32 = 6.4;
    pub const PITCH: f32 = 0.95;
    pub const YAW: f32 = 0.22;
    /// Point the camera looks at (slightly above the table).
    pub const FOCUS: (f32, f32, f32) = (0.0, 0.1, 0.15);
    pub const ORBIT_SPEED: f32 = 0.005;
    pub const ZOOM_SPEED: f32 = 0.4;
    pub const MIN_DISTANCE: f32 = 3.0;
    pub const MAX_DISTANCE: f32 = 12.0;
    pub const MIN_PITCH: f32 = 0.2;
    pub const MAX_PITCH: f32 = 1.5;
}

/// Screenshot mode (`BOOK3D_SCREENSHOT=path cargo run`): open the cover,
/// capture after `SCREENSHOT_FRAME` frames and quit.
pub mod screenshot {
    pub const ENV_VAR: &str = "BOOK3D_SCREENSHOT";
    pub const OPEN_COVER_SPEED: f32 = 7.0;
    pub const SCREENSHOT_FRAME: u32 = 300;
    pub const EXIT_FRAME: u32 = 360;
}
