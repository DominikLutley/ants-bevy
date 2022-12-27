use bevy::prelude::*;
use rand::Rng;

pub mod move_ants;

use crate::constants::{ANT_COUNT, ANT_SIZE};

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
pub struct HasFood(bool);

impl HasFood {
    pub fn set(&mut self) {
        self.0 = true;
    }
    pub fn unset(&mut self) {
        self.0 = false;
    }
    pub fn get(&self) -> bool {
        self.0
    }
}

#[derive(Bundle)]
struct AntBundle {
    _ant: Ant,
    has_food: HasFood,
    #[bundle]
    sprite: SpriteBundle,
}

impl AntBundle {
    fn new(position: Vec2, rotation: f32) -> AntBundle {
        AntBundle {
            _ant: Ant,
            has_food: HasFood(false),
            sprite: SpriteBundle {
                transform: Transform {
                    translation: position.extend(1.0),
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
