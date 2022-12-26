use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::NEST_SIZE;

#[derive(Component)]
pub struct Nest;

#[derive(Bundle)]
struct NestBundle {
    _nest: Nest,
    // Collisions
    collider: Collider,
    collision_groups: CollisionGroups,
    collision_types: ActiveCollisionTypes,
    rigid_body: RigidBody,
    active_events: ActiveEvents,
    #[bundle]
    sprite: SpriteBundle,
}

impl NestBundle {
    fn new(point: Vec2) -> NestBundle {
        NestBundle {
            _nest: Nest,
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
                        x: NEST_SIZE,
                        y: NEST_SIZE,
                        z: 1.,
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.6, 0.4, 0.4),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_nest(mut commands: Commands) {
    commands.spawn_bundle(NestBundle::new(Vec2 { x: 0., y: 0. }));
}
