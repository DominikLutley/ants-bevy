use crate::ants::Ant;
use crate::constants::ANT_SPEED;
use crate::constants::ANT_WANDER_PERCENT;
use bevy::prelude::*;
use rand::Rng;

const MAX_ROTATION: f32 = ANT_WANDER_PERCENT / 100.0 * std::f32::consts::PI;

pub fn rotate_ants(mut query: Query<&mut Transform, With<Ant>>) {
    let mut rng = rand::thread_rng();
    if MAX_ROTATION <= 0.0 {
        return;
    }
    for mut transform in query.iter_mut() {
        let angle = rng.gen_range(-1.0 * MAX_ROTATION..MAX_ROTATION);
        transform.rotate_z(angle);
    }
}

pub fn translate_ants(time: Res<Time>, mut query: Query<&mut Transform, With<Ant>>) {
    for mut transform in query.iter_mut() {
        let local_x = transform.local_x();
        transform.translation += local_x * ANT_SPEED * time.delta_seconds();
    }
}
