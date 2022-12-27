use bevy::prelude::*;

use crate::constants::{CELL_SIZE, GRID_SIZE};

#[derive(Component)]
pub struct Cell;

#[derive(Component)]
pub enum CellType {
    Empty,
    Wall,
    Pheromone,
}

#[derive(Bundle)]
struct CellBundle {
    _cell: Cell,
    cell_type: CellType,
    #[bundle]
    sprite: SpriteBundle,
}

impl CellBundle {
    fn new(position: Vec2, cell_type: CellType) -> CellBundle {
        CellBundle {
            _cell: Cell,
            cell_type,
            sprite: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec3 {
                        x: CELL_SIZE,
                        y: CELL_SIZE,
                        z: 1.0,
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.3, 0.3, 0.3),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn spawn_grid(mut commands: Commands) {
    let mut x = -0.5 * GRID_SIZE;
    while x < 0.5 * GRID_SIZE {
        let mut y = -0.5 * GRID_SIZE;
        while y < 0.5 * GRID_SIZE {
            commands.spawn_bundle(CellBundle::new(
                Vec2 {
                    x: x as f32,
                    y: y as f32,
                },
                CellType::Empty,
            ));
            y += CELL_SIZE;
        }
        x += CELL_SIZE;
    }
}
