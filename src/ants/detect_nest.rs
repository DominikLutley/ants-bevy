use crate::ants::HasFood;
use crate::ants::NestInRange;
use crate::constants::ANT_SIZE;
use crate::nest::Nest;
use bevy::prelude::*;

use crate::ants::Ant;

pub fn detect_nest(
    mut query_ants: Query<(&mut HasFood, &mut Transform, &NestInRange), With<Ant>>,
    query_nest: Query<(&Transform, With<Nest>, Without<Ant>)>,
) {
    for (mut has_food, mut ant_transform, nest_in_range) in query_ants.iter_mut() {
        if !has_food.get() {
            continue;
        }
        let nest_entity = match nest_in_range.0 {
            Some(entity) => entity,
            None => {
                continue;
            }
        };
        let nest_transform = query_nest.get_component::<Transform>(nest_entity).unwrap();
        let diff_vector = nest_transform.translation - ant_transform.translation;
        if diff_vector.length() < ANT_SIZE * 2. {
            has_food.unset();
            ant_transform.rotate_z(std::f32::consts::PI);
            continue;
        }

        let angle_diff = diff_vector
            .truncate()
            .angle_between(ant_transform.local_x().truncate());
        ant_transform.rotate_z(-1. * angle_diff);
    }
}
