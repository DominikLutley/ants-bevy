use crate::ants::FoodInRange;
use crate::ants::{Ant, WallsInRange};
use crate::food::Food;
use crate::walls::Wall;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const EXPECT_WALLS_IN_RANGE_MESSAGE: &str = "Ant should contain WallsInRange component";
const EXPECT_FOOD_IN_RANGE_MESSAGE: &str = "Ant should contain FoodInRange component";

pub fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut query_ants: Query<(&mut WallsInRange, &mut FoodInRange), With<Ant>>,
    query_walls: Query<With<Wall>>,
    query_food: Query<With<Food>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(first, second, _) => {
                let (ant, other) = get_ant_from_collision_event(&query_ants, first, second);
                if query_walls.contains(*other) {
                    query_ants
                        .get_component_mut::<WallsInRange>(*ant)
                        .expect(EXPECT_WALLS_IN_RANGE_MESSAGE)
                        .into_inner()
                        .add_wall(*other);
                    continue;
                }
                if query_food.contains(*other) {
                    query_ants
                        .get_component_mut::<FoodInRange>(*ant)
                        .expect(EXPECT_FOOD_IN_RANGE_MESSAGE)
                        .into_inner()
                        .set_if_none(*other);
                    continue;
                }
            }
            CollisionEvent::Stopped(first, second, _) => {
                let (ant, other) = get_ant_from_collision_event(&query_ants, first, second);
                if query_walls.contains(*other) {
                    query_ants
                        .get_component_mut::<WallsInRange>(*ant)
                        .expect(EXPECT_WALLS_IN_RANGE_MESSAGE)
                        .into_inner()
                        .remove_wall(*other);
                    continue;
                }
                if query_food.contains(*other) {
                    query_ants
                        .get_component_mut::<FoodInRange>(*ant)
                        .expect(EXPECT_FOOD_IN_RANGE_MESSAGE)
                        .into_inner()
                        .unset();
                    continue;
                }
            }
        }
    }
}

fn get_ant_from_collision_event<'a>(
    query_ants: &Query<(&mut WallsInRange, &mut FoodInRange), With<Ant>>,
    first: &'a Entity,
    second: &'a Entity,
) -> (&'a Entity, &'a Entity) {
    match (query_ants.contains(*first), query_ants.contains(*second)) {
        (true, false) => (first, second),
        (false, true) => (second, first),
        (false, false) => {
            panic!("Only ants should collide with objects")
        }
        (true, true) => {
            panic!("Ants shouldn't collide")
        }
    }
}
