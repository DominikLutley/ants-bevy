use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod ants;
use ants::{detect_walls, move_ants, spawn_ants};
use collisions::handle_collisions;
use walls::spawn_walls;
mod collisions;
mod constants;
mod walls;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_camera_2d)
        .add_startup_system(spawn_ants)
        .add_startup_system(spawn_walls)
        .add_system(move_ants)
        .add_system(detect_walls)
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

// fn display_events(
//     mut collision_events: EventReader<CollisionEvent>,
//     mut contact_force_events: EventReader<ContactForceEvent>,
// ) {
//     for collision_event in collision_events.iter() {
//         println!("Received collision event: {:?}", collision_event);
//     }

//     for contact_force_event in contact_force_events.iter() {
//         println!("Received contact force event: {:?}", contact_force_event);
//     }
// }
