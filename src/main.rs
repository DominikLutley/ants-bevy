mod ants;
mod constants;
mod world;

use crate::ants::pheromones::drop_pheromones;
use crate::ants::spawn_ants;
use ants::collisions::resolve_collisions;
use ants::move_ants::{return_to_nest, rotate_ants, save_location, translate_ants};
use ants::pheromones::detect_pheromones;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use world::{spawn_world, update_world_image};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera_2d)
        .add_startup_system(spawn_ants)
        .add_startup_system(spawn_world)
        .add_system(rotate_ants)
        .add_system(detect_pheromones.after(rotate_ants))
        .add_system(translate_ants.after(detect_pheromones))
        .add_system(resolve_collisions.after(translate_ants))
        .add_system(save_location.after(resolve_collisions))
        .add_system(drop_pheromones.after(resolve_collisions))
        .add_system(return_to_nest)
        .add_system(update_world_image)
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
