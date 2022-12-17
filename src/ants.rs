use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::constants::{ANT_COUNT, ANT_SIZE, ANT_SPEED, ANT_WANDER_PERCENT};

#[derive(Component)]
pub struct Ant;

#[derive(Bundle)]
struct AntBundle {
    _a: Ant,
    #[bundle]
    sprite: SpriteBundle,
}

impl AntBundle {
    fn new(position: Vec2, rotation: f32) -> AntBundle {
        AntBundle {
            _a: Ant,
            sprite: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    rotation: Quat::from_rotation_z(rotation),
                    scale: Vec3::new(ANT_SIZE / 2.0, ANT_SIZE, 1.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_ants(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _i in 0..ANT_COUNT {
        let angle = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
        commands.spawn_bundle(AntBundle::new(Vec2::new(0., 0.), angle));
        commands.spawn().insert(Collider::ball(ANT_SIZE / 2.0));
    }
}

pub fn move_ants(time: Res<Time>, mut query: Query<&mut Transform, With<Ant>>) {
    for mut transform in query.iter_mut() {
        rotate_ant(&mut transform);
        translate_ant(&mut transform, time.delta_seconds());
    }
}

fn rotate_ant(transform: &mut Transform) {
    let mut rng = rand::thread_rng();
    let max = ANT_WANDER_PERCENT / 100. * std::f32::consts::PI;
    let diff = rng.gen_range(-1. * max..max);
    transform.rotate_z(diff);
}

fn translate_ant(transform: &mut Transform, delta_seconds: f32) {
    let movement_direction = transform.rotation * Vec3::Y;
    transform.translation += movement_direction * ANT_SPEED * delta_seconds;
}
