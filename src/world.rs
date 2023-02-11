use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::constants::WORLD_SIZE;

#[derive(Component)]
pub struct World;

#[derive(Component)]
pub struct WorldImage(Handle<Image>);

#[derive(Bundle)]
struct WorldBundle {
    _world: World,
    world_image: WorldImage,
    #[bundle]
    sprite: SpriteBundle,
}

impl WorldBundle {
    fn new(image_handle: Handle<Image>) -> WorldBundle {
        WorldBundle {
            _world: World,
            world_image: WorldImage(image_handle.clone()),
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
    let image = Image::new(
        Extent3d {
            width: WORLD_SIZE.into(),
            height: WORLD_SIZE.into(),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![20; usize::from(WORLD_SIZE).pow(2) * 4],
        TextureFormat::Rgba8Unorm,
    );
    let handle = images.add(image);
    commands.spawn_bundle(WorldBundle::new(handle));
}
