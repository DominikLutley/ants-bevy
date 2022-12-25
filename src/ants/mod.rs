use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
pub mod detect_walls;
pub mod move_ants;

use crate::constants::{ANT_COUNT, ANT_SIZE, ANT_VIEW_RADIUS};

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
pub struct WallsInRange(pub Vec<Entity>);

impl WallsInRange {
    pub fn add_wall(&mut self, wall: Entity) {
        self.0.push(wall);
    }

    pub fn remove_wall(&mut self, wall: Entity) {
        self.0.retain(|x| wall.id() != x.id());
    }
}

#[derive(Bundle)]
struct AntBundle {
    _ant: Ant,
    walls_in_range: WallsInRange,
    // Collisions
    collider: Collider,
    collision_groups: CollisionGroups,
    rigid_body: RigidBody,
    #[bundle]
    sprite: SpriteBundle,
}

impl AntBundle {
    fn new(position: Vec2, rotation: f32) -> AntBundle {
        AntBundle {
            _ant: Ant,
            walls_in_range: WallsInRange(Vec::new()),
            // Collisions
            collider: Collider::ball(ANT_VIEW_RADIUS / ANT_SIZE),
            collision_groups: CollisionGroups::new(
                Group::from_bits_truncate(0b0001),
                Group::from_bits_truncate(0b0010),
            ),
            rigid_body: RigidBody::KinematicPositionBased,
            sprite: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    rotation: Quat::from_rotation_z(rotation),
                    scale: Vec3::new(ANT_SIZE, ANT_SIZE, 1.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1., 1., 1.),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_ants(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _i in 0..ANT_COUNT {
        let angle = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
        commands.spawn_bundle(AntBundle::new(Vec2::new(0., 0.), angle));
    }
}
