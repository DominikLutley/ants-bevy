use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

mod ants;
use ants::move_ants::{rotate_ants, translate_ants};
use ants::spawn_ants;

mod grid;
use grid::spawn_grid;

mod constants;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera_2d)
        .add_startup_system(spawn_ants)
        .add_startup_system(spawn_grid)
        .add_system(translate_ants)
        .add_system(rotate_ants)
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
