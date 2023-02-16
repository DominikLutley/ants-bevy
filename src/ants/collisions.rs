use bevy::prelude::*;

use crate::{
    ants::Ant,
    world::{LocationState, WorldState},
};

use super::{move_ants::translate_ant, HasFood};

pub fn resolve_collisions(
    mut commands: Commands,
    mut query_ants: Query<(&mut Transform, Entity, Without<HasFood>), With<Ant>>,
    mut query_world: Query<&mut WorldState>,
) {
    for (mut transform, entity, _) in query_ants.iter_mut() {
        let location_state = query_world
            .single()
            .get_location_state(transform.translation.truncate());

        match location_state {
            None | Some(LocationState::Wall) => {
                transform.rotate_z(std::f32::consts::PI);
                translate_ant(&mut transform);
            }
            Some(LocationState::Food) => {
                commands.entity(entity).insert(HasFood);
                query_world
                    .single_mut()
                    .remove_food(transform.translation.truncate());
                transform.rotate_z(std::f32::consts::PI);
                translate_ant(&mut transform);
            }
            _ => {}
        }
    }
}
