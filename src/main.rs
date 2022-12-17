use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod ants;
use ants::{move_ants, spawn_ants};
mod constants;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_startup_system(setup_camera_2d)
        .add_startup_system(spawn_ants)
        .add_system(move_ants)
        .run();
}

fn setup_camera_2d(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.1, 0.1, 0.1)),
        },
        ..Default::default()
    });
    commands
        .spawn()
        .insert(Collider::cuboid(500.0, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}
