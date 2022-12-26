use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod ants;
use ants::detect_food::detect_food;
use ants::detect_walls::detect_walls;
use ants::move_ants::move_ants;
use ants::spawn_ants;

mod collisions;
use collisions::handle_collisions;

mod constants;

mod walls;
use walls::spawn_walls;

mod food;
use food::spawn_food;

mod nest;
use nest::spawn_nest;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_camera_2d)
        .add_startup_system(spawn_ants)
        .add_startup_system(spawn_walls)
        .add_startup_system(spawn_food)
        .add_startup_system(spawn_nest)
        .add_system(move_ants)
        .add_system(detect_walls)
        .add_system(detect_food)
        .add_system(handle_collisions)
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
