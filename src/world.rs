use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::hashbrown::HashMap,
};

use crate::constants::WORLD_SIZE;

#[derive(Component)]
pub struct World;

#[derive(Hash, Eq, PartialEq)]
pub struct WorldLocation((i32, i32));

pub struct PheromoneState(u8);

pub enum LocationState {
    Wall,
    Food,
    Nest,
    Empty(PheromoneState),
}

#[derive(Component)]
pub struct WorldState(HashMap<WorldLocation, LocationState>);

impl WorldState {
    fn new() -> WorldState {
        if WORLD_SIZE % 2 == 0 {
            panic!("WORLD_SIZE must be odd");
        }

        let mut world_state = HashMap::with_capacity(usize::from(WORLD_SIZE).pow(2));

        for i in -1 * i32::from(WORLD_SIZE / 2)..=i32::from(WORLD_SIZE / 2) {
            for j in -1 * i32::from(WORLD_SIZE / 2)..=i32::from(WORLD_SIZE / 2) {
                let distance = Vec2::from_array([i as f32, j as f32]).length();
                let location_state = if distance > WORLD_SIZE as f32 / 1.7 {
                    LocationState::Food
                } else {
                    LocationState::Empty(PheromoneState(0))
                };
                world_state.insert(WorldLocation((i as i32, j as i32)), location_state);
            }
        }
        WorldState(world_state)
    }

    pub fn get_location_state(&self, position: Vec2) -> Option<&LocationState> {
        let x = position.x.round() as i32;
        let y = position.y.round() as i32;
        self.0.get(&WorldLocation((x, y)))
    }

    pub fn to_image(&self) -> Image {
        let mut data: Vec<u8> = Vec::with_capacity(usize::from(WORLD_SIZE).pow(2) * 4);
        let mut count = 0;
        for y in -1 * i32::from(WORLD_SIZE / 2)..=i32::from(WORLD_SIZE / 2) {
            for x in -1 * i32::from(WORLD_SIZE / 2)..=i32::from(WORLD_SIZE / 2) {
                count += 4;
                match self.0.get(&WorldLocation((x, y))) {
                    Some(LocationState::Food) => {
                        data.push(12);
                        data.push(31);
                        data.push(12);
                        data.push(255);
                    }
                    Some(LocationState::Empty(_)) => {
                        data.push(2);
                        data.push(2);
                        data.push(2);
                        data.push(255);
                    }
                    Some(_) => {
                        data.push(255);
                        data.push(0);
                        data.push(0);
                        data.push(255);
                    }
                    None => {
                        panic!("This location doesn't exist");
                    }
                }
            }
        }
        Image::new(
            Extent3d {
                width: WORLD_SIZE.into(),
                height: WORLD_SIZE.into(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8Unorm,
        )
    }
}

#[derive(Bundle)]
struct WorldBundle {
    _world: World,
    state: WorldState,
    #[bundle]
    sprite: SpriteBundle,
}

impl WorldBundle {
    fn new(image_handle: Handle<Image>, state: WorldState) -> WorldBundle {
        WorldBundle {
            _world: World,
            state,
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    ..default()
                },
                texture: image_handle,
                ..default()
            },
        }
    }
}

pub fn spawn_world(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let world_state = WorldState::new();
    let image = world_state.to_image();
    let handle = images.add(image);
    commands.spawn_bundle(WorldBundle::new(handle, world_state));
}
