use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::FOOD_SIZE;

#[derive(Component)]
pub struct Food;

#[derive(Bundle)]
struct FoodBundle {
    _food: Food,
    // Collisions
    collider: Collider,
    collision_groups: CollisionGroups,
    collision_types: ActiveCollisionTypes,
    rigid_body: RigidBody,
    active_events: ActiveEvents,
    #[bundle]
    sprite: SpriteBundle,
}

impl FoodBundle {
    fn new(point: Vec2) -> FoodBundle {
        FoodBundle {
            _food: Food,
            collider: Collider::cuboid(0.5, 0.5),
            collision_groups: CollisionGroups::new(
                Group::from_bits_truncate(0b0010),
                Group::from_bits_truncate(0b0001),
            ),
            collision_types: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::KINEMATIC_STATIC,
            rigid_body: RigidBody::Fixed,
            active_events: ActiveEvents::COLLISION_EVENTS,
            sprite: SpriteBundle {
                transform: Transform {
                    translation: point.extend(0.),
                    scale: Vec3 {
                        x: FOOD_SIZE,
                        y: FOOD_SIZE,
                        z: 1.,
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.4, 1.0, 0.4),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_food(mut commands: Commands) {
    for x in (100..140).step_by(FOOD_SIZE as usize) {
        for y in (100..140).step_by(FOOD_SIZE as usize) {
            commands.spawn_bundle(FoodBundle::new(Vec2 {
                x: x as f32,
                y: y as f32,
            }));
        }
    }
}
