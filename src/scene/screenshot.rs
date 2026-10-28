//! Optional screenshot mode for docs: `BOOK3D_SCREENSHOT=docs/screenshot.png cargo run`.
//! It flicks the cover open, saves one frame and quits.

use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};

use crate::constants::{COVER_INDEX, screenshot::*};
use crate::types::{Leaf, LeafMode, LeafState};

#[derive(Resource)]
struct ScreenshotPath(String);

pub struct ScreenshotPlugin;

impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        if let Ok(path) = std::env::var(ENV_VAR) {
            app.insert_resource(ScreenshotPath(path)).add_systems(Update, run_screenshot);
        }
    }
}

fn run_screenshot(
    mut frame: Local<u32>,
    path: Res<ScreenshotPath>,
    mut commands: Commands,
    mut leaves: Query<(&Leaf, &mut LeafState)>,
    mut exit: MessageWriter<AppExit>,
) {
    *frame += 1;
    match *frame {
        2 => open_cover(&mut leaves),
        SCREENSHOT_FRAME => {
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path.0.clone()));
        }
        EXIT_FRAME => {
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}

fn open_cover(leaves: &mut Query<(&Leaf, &mut LeafState)>) {
    for (_, mut state) in leaves.iter_mut().filter(|(leaf, _)| leaf.index == COVER_INDEX) {
        state.velocity = OPEN_COVER_SPEED;
        state.mode = LeafMode::Free;
    }
}
