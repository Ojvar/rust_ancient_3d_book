//! A 3D book with ten pages you can turn with the mouse.

mod book;
mod constants;
mod input;
mod physics;
mod scene;
mod types;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Book 3D".into(), ..default() }),
            ..default()
        }))
        .add_plugins((
            scene::ScenePlugin,
            book::BookPlugin,
            physics::PhysicsPlugin,
            input::InputPlugin,
        ))
        .run();
}
