use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::hashbrown::HashMap,
};

use crate::constants::{
    ANT_DETECTION_ANGLE, ANT_DETECTION_DISTANCE, ANT_DETECTION_RADIUS, WORLD_SIZE,
};

#[derive(Component)]
pub struct World;

#[derive(Hash, Eq, PartialEq)]
pub struct WorldLocation((i32, i32));

impl From<Vec3> for WorldLocation {
    fn from(value: Vec3) -> Self {
        WorldLocation((value.x as i32, value.y as i32))
    }
}

impl From<WorldLocation> for Vec3 {
    fn from(value: WorldLocation) -> Self {
        let location = value.0;
        Vec3 {
            x: location.0 as f32,
            y: location.1 as f32,
            z: 1.0,
        }
    }
}

#[derive(Debug)]
pub struct PheromoneState(u8);

#[derive(Debug)]
pub enum LocationState {
    Wall,
    Food,
    Nest,
    Ground(PheromoneState),
}

pub enum TurnDirection {
    Left,
    Center,
    Right,
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
                } else if distance < 10.0 {
                    LocationState::Nest
                } else {
                    LocationState::Ground(PheromoneState(0))
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

    pub fn place_pheromone(&mut self, position: Vec2) {
        let x = position.x.round() as i32;
        let y = position.y.round() as i32;
        match self.0.get(&WorldLocation((x, y))) {
            Some(LocationState::Ground(_)) => {
                self.0.insert(
                    WorldLocation((x, y)),
                    LocationState::Ground(PheromoneState(255)),
                );
            }
            _ => {}
        }
    }

    pub fn remove_food(&mut self, position: Vec2) {
        let x = position.x.round() as i32;
        let y = position.y.round() as i32;
        match self.0.get(&WorldLocation((x, y))) {
            Some(LocationState::Food) => {
                self.0.insert(
                    WorldLocation((x, y)),
                    LocationState::Ground(PheromoneState(0)),
                );
            }
            _ => {}
        }
    }

    fn pheromone_zone(&self, location: WorldLocation) -> i32 {
        let mut sum: i32 = 0;
        for i in -1 * ANT_DETECTION_RADIUS..=ANT_DETECTION_RADIUS {
            for j in -1 * ANT_DETECTION_RADIUS..=ANT_DETECTION_RADIUS {
                let state = self
                    .0
                    .get(&WorldLocation((location.0 .0 + i, location.0 .1 + j)))
                    .unwrap_or(&LocationState::Ground(PheromoneState(0)));
                match state {
                    LocationState::Ground(PheromoneState(val)) => {
                        sum += *val as i32;
                    }
                    _ => {}
                }
            }
        }
        sum
    }

    pub fn query_pheromones(&self, transform: &Transform) -> TurnDirection {
        let center_dir = transform.local_x().truncate().normalize().extend(0.0);
        let center_location = transform.translation + center_dir * ANT_DETECTION_DISTANCE;
        let center_angle = -1.0 * center_dir.truncate().angle_between(Vec2 { x: 1.0, y: 0.0 });
        let left_dir = transform
            .with_rotation(Quat::from_rotation_z(center_angle + ANT_DETECTION_ANGLE))
            .local_x()
            .truncate()
            .normalize()
            .extend(0.0);
        let left_location = transform.translation + left_dir * ANT_DETECTION_DISTANCE;
        let right_dir = transform
            .with_rotation(Quat::from_rotation_z(center_angle - ANT_DETECTION_ANGLE))
            .local_x()
            .truncate()
            .normalize()
            .extend(0.0);
        let right_location = transform.translation + right_dir * ANT_DETECTION_DISTANCE;
        let center_sum = self.pheromone_zone(WorldLocation((
            center_location.x.round() as i32,
            center_location.y.round() as i32,
        )));
        let left_sum = self.pheromone_zone(WorldLocation((
            left_location.x.round() as i32,
            left_location.y.round() as i32,
        )));
        let right_sum = self.pheromone_zone(WorldLocation((
            right_location.x.round() as i32,
            right_location.y.round() as i32,
        )));
        if center_sum >= left_sum && center_sum >= right_sum {
            TurnDirection::Center
        } else if left_sum >= right_sum {
            TurnDirection::Left
        } else {
            TurnDirection::Right
        }
    }

    pub fn update_image(&mut self) -> Image {
        let mut data: Vec<u8> = Vec::with_capacity(usize::from(WORLD_SIZE).pow(2) * 4);
        for y in -1 * i32::from(WORLD_SIZE / 2)..=i32::from(WORLD_SIZE / 2) {
            for x in -1 * i32::from(WORLD_SIZE / 2)..=i32::from(WORLD_SIZE / 2) {
                match self.0.get(&WorldLocation((x, -1 * y))) {
                    Some(LocationState::Food) => {
                        data.push(18);
                        data.push(58);
                        data.push(18);
                        data.push(255);
                    }
                    Some(LocationState::Ground(PheromoneState(strength))) => {
                        data.push(8);
                        data.push(54);
                        data.push(34);
                        data.push(*strength);
                        if *strength > 0 {
                            self.0.insert(
                                WorldLocation((x, -1 * y)),
                                LocationState::Ground(PheromoneState(strength - 1)),
                            );
                        }
                    }
                    Some(LocationState::Wall) => {
                        data.push(24);
                        data.push(24);
                        data.push(24);
                        data.push(255);
                    }
                    Some(LocationState::Nest) => {
                        data.push(12);
                        data.push(12);
                        data.push(68);
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

#[derive(Component)]
pub struct WorldImage(Handle<Image>);

#[derive(Bundle)]
struct WorldBundle {
    _world: World,
    state: WorldState,
    image: WorldImage,
    #[bundle]
    sprite: SpriteBundle,
}

impl WorldBundle {
    fn new(image_handle: Handle<Image>, state: WorldState) -> WorldBundle {
        WorldBundle {
            _world: World,
            state,
            image: WorldImage(image_handle.clone()),
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
    let mut world_state = WorldState::new();
    let image = world_state.update_image();
    let handle = images.add(image);
    commands.spawn_bundle(WorldBundle::new(handle, world_state));
}

pub fn update_world_image(
    mut images: ResMut<Assets<Image>>,
    mut query_world: Query<(&mut WorldState, &mut WorldImage)>,
) {
    let (mut state, mut image) = query_world.single_mut();
    image.0 = images.set(image.0.to_owned(), state.update_image());
}
