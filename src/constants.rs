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

/// Resting shape of real paper (radians of extra tilt, fractions of width).
/// Near the spine the page arches up out of the gutter and settles again
/// within `GUTTER_SPAN` of the width.
pub const GUTTER_ANGLE: f32 = 0.24;
pub const GUTTER_SPAN: f32 = 0.32;
/// A soft, low wave across the sheet and a lift of the outer corners.
pub const PAPER_RIPPLE: f32 = 0.045;
pub const CORNER_CURL: f32 = 0.11;
/// How much light passes through a sheet (candle glow seen through paper).
pub const PAPER_TRANSLUCENCY: f32 = 0.22;
/// Strength and resolution of the procedural paper-fibre normal map.
pub const PAPER_GRAIN: f32 = 0.35;
pub const PAPER_GRAIN_SIZE: u32 = 512;

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
    /// How far the rounded spine bulges out past the hinge.
    pub const SPINE_RADIUS: f32 = 0.14;
    /// Extra height so the spine just covers the board rims.
    pub const SPINE_MARGIN: f32 = 0.004;
    /// Rounding of the board outline and of its rims.
    pub const BOARD_CORNER: f32 = 0.045;
    pub const BOARD_EDGE: f32 = 0.022;
    /// Raised bands across the spine (as on a hand-bound book), with gold
    /// tooling lines beside each one.
    pub const SPINE_BANDS: [f32; 4] = [-0.95, -0.32, 0.32, 0.95];
    pub const SPINE_BAND_RADIUS: f32 = 0.022;
    pub const SPINE_LINE_RADIUS: f32 = 0.006;
    pub const SPINE_LINE_OFFSET: f32 = 0.06;

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
    /// Waxed leather: a soft sheen layer over the rough hide.
    pub const LEATHER_CLEARCOAT: f32 = 0.4;
    pub const LEATHER_CLEARCOAT_ROUGHNESS: f32 = 0.5;
    pub const PAPER_ROUGHNESS: f32 = 0.95;
    pub const WOOD_ROUGHNESS: f32 = 0.7;
    /// Linear emissive colour of the flame (bright, so it blooms).
    pub const FLAME: (f32, f32, f32) = (18.0, 7.0, 1.5);
}

pub mod environment {
    pub const TABLE_SIZE: (f32, f32, f32) = (14.0, 0.3, 9.0);
    pub const CANDLE_POSITION: (f32, f32) = (-3.4, -1.9);
    /// glTF candle model (three wax candles) and its scale in world units.
    pub const CANDLE_MODEL: &str = "candle/scene.gltf";
    pub const CANDLE_SCALE: f32 = 1.2;
    /// Each candle in model space (Z-up): wick x, y, wick top z, wax rim top z.
    /// Tallest first; it is the one that casts shadows.
    pub const WICKS: [(f32, f32, f32, f32); 3] =
        [(0.259, 0.067, 0.832, 0.901), (-0.285, 0.184, 0.66, 0.73), (0.009, -0.089, 0.486, 0.466)];
    pub const FLAME_RADIUS: f32 = 0.05;
    pub const LIGHT_COLOR: (f32, f32, f32) = (1.0, 0.58, 0.24);
    pub const LIGHT_INTENSITY: f32 = 40_000.0;
    pub const LIGHT_RADIUS: f32 = 0.12;
    pub const FILL_ILLUMINANCE: f32 = 750.0;
    pub const AMBIENT_BRIGHTNESS: f32 = 110.0;
    pub const BACKGROUND: (f32, f32, f32) = (0.02, 0.015, 0.01);
    /// Candle flicker: how much the brightness wavers (fraction) and how far
    /// the light and flame sway (world units), plus the base flicker speed.
    pub const FLICKER_AMOUNT: f32 = 0.18;
    pub const FLICKER_SWAY: f32 = 0.012;
    pub const FLICKER_SPEED: f32 = 1.0;
    /// Bright inner core of the flame (linear emissive) and its size.
    pub const FLAME_CORE: (f32, f32, f32) = (40.0, 30.0, 14.0);
    pub const FLAME_CORE_SCALE: f32 = 0.45;
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
    pub const DISTANCE: f32 = 4.2;
    pub const PITCH: f32 = 0.5;
    pub const YAW: f32 = -1.1;
    /// Point the camera looks at (slightly above the table).
    pub const FOCUS: (f32, f32, f32) = (0.0, 0.1, 0.15);
    pub const ORBIT_SPEED: f32 = 0.005;
    pub const ZOOM_SPEED: f32 = 0.4;
    pub const MIN_DISTANCE: f32 = 3.0;
    pub const MAX_DISTANCE: f32 = 12.0;
    pub const MIN_PITCH: f32 = 0.2;
    pub const MAX_PITCH: f32 = 1.5;
    /// Pan speed per pixel of mouse motion, scaled by the camera distance.
    pub const PAN_SPEED: f32 = 0.0012;
    /// Keyboard pan speed (fraction of the camera distance per second).
    pub const KEY_PAN_SPEED: f32 = 0.6;
    /// Keyboard orbit speed (rad/s) and zoom speed (units/s).
    pub const KEY_ORBIT_SPEED: f32 = 1.4;
    pub const KEY_ZOOM_SPEED: f32 = 4.0;
    /// Touchpads report scrolling in pixels; this many pixels equal one wheel notch.
    pub const PIXELS_PER_LINE: f32 = 40.0;
    /// How far the focus may be panned away from the book (x, z).
    pub const PAN_LIMIT: (f32, f32) = (5.0, 3.5);
    /// How fast the camera eases toward its target (1/s); higher is snappier.
    pub const SMOOTHING: f32 = 14.0;
}

/// Screenshot mode (`BOOK3D_SCREENSHOT=path cargo run`): open the cover,
/// capture after `SCREENSHOT_FRAME` frames and quit.
pub mod screenshot {
    pub const ENV_VAR: &str = "BOOK3D_SCREENSHOT";
    /// Set this too to keep the book closed (shows the front cover).
    pub const CLOSED_ENV_VAR: &str = "BOOK3D_SCREENSHOT_CLOSED";
    pub const OPEN_COVER_SPEED: f32 = 7.0;
    pub const SCREENSHOT_FRAME: u32 = 300;
    pub const EXIT_FRAME: u32 = 360;
}
