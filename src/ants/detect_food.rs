use crate::constants::ANT_SIZE;
use crate::food::Food;
use bevy::prelude::*;

use crate::ants::Ant;
use crate::ants::FoodInRange;

use super::HasFood;

pub fn detect_food(
    mut query_ants: Query<(&mut HasFood, &mut Transform, &FoodInRange), With<Ant>>,
    query_food: Query<(&Transform, With<Food>, Without<Ant>)>,
) {
    for (mut has_food, mut ant_transform, food_in_range) in query_ants.iter_mut() {
        if has_food.get() {
            continue;
        }
        let food_transform = match food_in_range.0 {
            Some(entity) => query_food.get_component::<Transform>(entity).unwrap(),
            None => {
                continue;
            }
        };
        let diff_vector = food_transform.translation - ant_transform.translation;
        if diff_vector.length() < ANT_SIZE * 2. {
            has_food.set();
            ant_transform.rotate_z(std::f32::consts::PI);
            continue;
        }

        let angle_diff = diff_vector
            .truncate()
            .angle_between(ant_transform.local_x().truncate());
        ant_transform.rotate_z(-1. * angle_diff);
    }
}
