use crate::ants::Ant;
use crate::ants::HasFood;
use crate::world::WorldState;
use bevy::prelude::*;

pub fn drop_pheromones(
    query_ants: Query<(&Transform, &HasFood), With<Ant>>,
    mut query_world: Query<&mut WorldState>,
) {
    for (transform, has_food) in query_ants.iter() {
        if !has_food.get() {
            continue;
        }
        query_world
            .single_mut()
            .place_pheromone(transform.translation.truncate());
    }
}
