use crate::ants::Ant;
use crate::ants::HasFood;
use crate::constants::ANT_PHEROMONE_TURN_ANGLE;
use crate::world::TurnDirection;
use crate::world::WorldState;
use bevy::prelude::*;

pub fn drop_pheromones(
    query_ants: Query<(&Transform, With<HasFood>), With<Ant>>,
    mut query_world: Query<&mut WorldState>,
) {
    for (transform, _) in query_ants.iter() {
        query_world
            .single_mut()
            .place_pheromone(transform.translation.truncate());
    }
}

pub fn detect_surroundings(
    mut query_ants: Query<(&mut Transform, Without<HasFood>), With<Ant>>,
    query_world: Query<&WorldState>,
) {
    for (mut transform, _) in query_ants.iter_mut() {
        match query_world.single().query_surroundings(&transform) {
            TurnDirection::Left => {
                transform.rotate_z(ANT_PHEROMONE_TURN_ANGLE);
            }
            TurnDirection::Right => {
                transform.rotate_z(ANT_PHEROMONE_TURN_ANGLE * -1.0);
            }
            TurnDirection::Center => {}
        }
    }
}
