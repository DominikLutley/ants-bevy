use crate::constants::ANT_SIZE;
use crate::food::Food;
use bevy::prelude::*;

use crate::ants::Ant;
use crate::ants::FoodInRange;

use super::HasFood;

pub fn detect_food(
    mut query_ants: Query<(&mut HasFood, &mut Transform, &mut FoodInRange), With<Ant>>,
    query_food: Query<(&Transform, With<Food>, Without<Ant>)>,
    mut commands: Commands,
) {
    for (mut has_food, mut ant_transform, mut food_in_range) in query_ants.iter_mut() {
        if has_food.get() {
            continue;
        }
        let food_entity = match food_in_range.0 {
            Some(entity) => entity,
            None => {
                continue;
            }
        };
        let food_transform = query_food.get_component::<Transform>(food_entity);
        let food_transform = match food_transform {
            Ok(transform) => transform,
            Err(_) => {
                food_in_range.unset();
                continue;
            }
        };
        let diff_vector = food_transform.translation - ant_transform.translation;
        if diff_vector.length() < ANT_SIZE * 2. {
            has_food.set();
            ant_transform.rotate_z(std::f32::consts::PI);
            commands.entity(food_entity).despawn();
            continue;
        }

        let angle_diff = diff_vector
            .truncate()
            .angle_between(ant_transform.local_x().truncate());
        ant_transform.rotate_z(-1. * angle_diff);
    }
}
