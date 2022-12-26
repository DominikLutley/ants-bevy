use bevy::prelude::*;

use crate::ants::Ant;
use crate::ants::WallsInRange;
use crate::constants::ANT_VIEW_ARC;
use crate::walls::Wall;

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
