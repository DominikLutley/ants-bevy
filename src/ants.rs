use crate::{constants::ANT_VIEW_ARC, walls::Wall};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::constants::{ANT_COUNT, ANT_SIZE, ANT_SPEED, ANT_VIEW_RADIUS, ANT_WANDER_PERCENT};

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

struct WallsDetected {
    left: bool,
    middle: bool,
    right: bool,
}

impl WallsDetected {
    pub fn new() -> WallsDetected {
        WallsDetected {
            left: false,
            middle: false,
            right: false,
        }
    }
}

pub fn detect_walls(
    mut query_ants: Query<(&mut Transform, &WallsInRange), With<Ant>>,
    query_walls: Query<(&Transform, With<Wall>, Without<Ant>)>,
) {
    for (mut ant_transform, walls_in_range) in query_ants.iter_mut() {
        let walls_detected = filter_walls(&ant_transform, walls_in_range, &query_walls);
        avoid_walls(walls_detected, &mut ant_transform);
    }
}

fn filter_walls(
    transform: &Transform,
    walls_in_range: &WallsInRange,
    query_walls: &Query<(&Transform, With<Wall>, Without<Ant>)>,
) -> WallsDetected {
    let mut walls_detected = WallsDetected::new();
    for wall_entity in &walls_in_range.0 {
        let wall = query_walls
            .get_component::<Transform>(*wall_entity)
            .unwrap();
        let diff_vector = wall.translation - transform.translation;
        let angle = diff_vector
            .truncate()
            .angle_between(transform.local_x().truncate());
        match angle {
            a if a.abs() < ANT_VIEW_ARC / 2. => {
                walls_detected.middle = true;
            }
            a if a.abs() < ANT_VIEW_ARC => {
                if a.is_sign_positive() {
                    walls_detected.right = true;
                } else {
                    walls_detected.left = true;
                }
            }
            _ => {}
        }
    }
    walls_detected
}

fn avoid_walls(walls_detected: WallsDetected, transform: &mut Transform) {
    match (
        walls_detected.left,
        walls_detected.middle,
        walls_detected.right,
    ) {
        (true, true, false) => {
            transform.rotate_z(-0.2);
        }
        (false, true, true) => {
            transform.rotate_z(0.2);
        }
        (_, true, _) => {
            transform.rotate_z(std::f32::consts::PI);
        }
        _ => {}
    }
}

pub fn move_ants(time: Res<Time>, mut query: Query<&mut Transform, With<Ant>>) {
    for mut transform in query.iter_mut() {
        rotate_ant(&mut transform);
        translate_ant(&mut transform, time.delta_seconds());
    }
}

fn rotate_ant(transform: &mut Transform) {
    let mut rng = rand::thread_rng();
    let max = ANT_WANDER_PERCENT / 100. * std::f32::consts::PI;
    if max <= 0. {
        return;
    }
    let diff = rng.gen_range(-1. * max..max);
    transform.rotate_z(diff);
}

fn translate_ant(transform: &mut Transform, delta_seconds: f32) {
    let movement_direction = transform.rotation * Vec3::X;
    transform.translation += movement_direction * ANT_SPEED * delta_seconds;
}
