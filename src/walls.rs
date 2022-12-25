use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::{FIELD_SIZE, WALL_SIZE};

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
struct WallBundle {
    _wall: Wall,
    // Collisions
    collider: Collider,
    collision_groups: CollisionGroups,
    collision_types: ActiveCollisionTypes,
    rigid_body: RigidBody,
    active_events: ActiveEvents,
    #[bundle]
    sprite: SpriteBundle,
}

impl WallBundle {
    fn new(point: Vec2) -> WallBundle {
        WallBundle {
            _wall: Wall,
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
                        x: WALL_SIZE,
                        y: WALL_SIZE,
                        z: 1.,
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.4, 0.4, 0.4),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_walls(mut commands: Commands) {
    let mut x = -1. * FIELD_SIZE;
    while x <= FIELD_SIZE {
        commands.spawn_bundle(WallBundle::new(Vec2 { x, y: FIELD_SIZE }));
        commands.spawn_bundle(WallBundle::new(Vec2 {
            x,
            y: -1. * FIELD_SIZE,
        }));
        x += WALL_SIZE;
    }

    let mut y = -1. * FIELD_SIZE + WALL_SIZE;
    while y <= FIELD_SIZE - WALL_SIZE {
        commands.spawn_bundle(WallBundle::new(Vec2 { x: FIELD_SIZE, y }));
        commands.spawn_bundle(WallBundle::new(Vec2 {
            x: -1. * FIELD_SIZE,
            y,
        }));
        y += WALL_SIZE;
    }
}
