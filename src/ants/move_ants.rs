use crate::ants::Ant;
use crate::constants::ANT_STEP;
use crate::constants::ANT_WANDER_PERCENT;
use crate::world::WorldLocation;
use bevy::prelude::*;
use bevy::transform;
use rand::Rng;

use super::HasFood;
use super::PathToHome;

const MAX_ROTATION: f32 = ANT_WANDER_PERCENT / 100.0 * std::f32::consts::PI;

pub fn rotate_ants(mut query: Query<(&mut Transform, With<Ant>, Without<HasFood>)>) {
    if MAX_ROTATION <= 0.0 {
        return;
    }
    let mut rng = rand::thread_rng();
    for (mut transform, _, _) in query.iter_mut() {
        let angle = rng.gen_range(-1.0 * MAX_ROTATION..MAX_ROTATION);
        transform.rotate_z(angle);
    }
}

pub fn translate_ants(mut query: Query<(&mut Transform, With<Ant>, Without<HasFood>)>) {
    for (mut transform, _, _) in query.iter_mut() {
        translate_ant(&mut transform);
    }
}

pub fn translate_ant(transform: &mut Transform) {
    let local_x = transform.local_x();
    transform.translation += local_x * ANT_STEP;
}

pub fn save_location(mut query: Query<(&mut PathToHome, &Transform, Without<HasFood>, With<Ant>)>) {
    for (mut path_to_home, transform, _, _) in query.iter_mut() {
        path_to_home
            .0
            .push(WorldLocation::from(transform.translation));
    }
}

pub fn return_to_nest(
    mut commands: Commands,
    mut query: Query<(
        &mut PathToHome,
        &mut Transform,
        Entity,
        With<HasFood>,
        With<Ant>,
    )>,
) {
    for (mut path_to_home, mut transform, entity, _, _) in query.iter_mut() {
        let next_location = path_to_home.0.pop();
        match next_location {
            Some(value) => {
                let location: Vec3 = value.into();
                let diff = location - transform.translation;
                transform.rotation = Quat::from_rotation_z(
                    -1.0 * diff.truncate().angle_between(Vec2 { x: 1.0, y: 0.0 }),
                );
                transform.translation = location;
            }
            None => {
                commands.entity(entity).remove::<HasFood>();
                transform.rotate_z(std::f32::consts::PI);
            }
        }
    }
}
