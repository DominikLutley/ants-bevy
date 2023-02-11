use bevy::prelude::*;

use crate::{
    ants::Ant,
    world::{LocationState, WorldState},
};

use super::move_ants::translate_ant;

pub fn resolve_collisions(
    mut query_ants: Query<&mut Transform, With<Ant>>,
    query_world: Query<&WorldState>,
) {
    for mut transform in query_ants.iter_mut() {
        let location_state = query_world
            .single()
            .get_location_state(transform.translation.truncate());

        match location_state {
            None | Some(LocationState::Wall) => {
                transform.rotate_z(std::f32::consts::PI);
                translate_ant(&mut transform);
            }
            _ => {}
        }
    }
}
