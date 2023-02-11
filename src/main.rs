mod ants;
mod constants;
mod world;

use crate::ants::spawn_ants;
use ants::collisions::resolve_collisions;
use ants::move_ants::{rotate_ants, translate_ants};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use world::spawn_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera_2d)
        .add_startup_system(spawn_ants)
        .add_startup_system(spawn_world)
        .add_system(rotate_ants)
        .add_system(translate_ants.after(rotate_ants))
        .add_system(resolve_collisions.after(translate_ants))
        .run();
}

fn setup_camera_2d(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.1, 0.1, 0.1)),
        },
        ..Default::default()
    });
}
