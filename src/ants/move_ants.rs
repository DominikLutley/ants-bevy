use crate::ants::Ant;
use crate::constants::ANT_SPEED;
use crate::constants::ANT_WANDER_PERCENT;
use bevy::prelude::*;
use rand::Rng;

pub fn move_ants(time: Res<Time>, mut query: Query<&mut Transform, With<Ant>>) {
    for mut transform in query.iter_mut() {
        rotate_ant(&mut transform);
        translate_ant(&mut transform, time.delta_seconds());
    }
}

fn rotate_ant(transform: &mut Transform) {
    let mut rng = rand::thread_rng();
    let max = ANT_WANDER_PERCENT / 100. * std::f32::consts::PI;
    if max <= 0. {
        return;
    }
    let diff = rng.gen_range(-1. * max..max);
    transform.rotate_z(diff);
}

fn translate_ant(transform: &mut Transform, delta_seconds: f32) {
    let movement_direction = transform.rotation * Vec3::X;
    transform.translation += movement_direction * ANT_SPEED * delta_seconds;
}
