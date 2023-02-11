use bevy::prelude::*;

use crate::ants::Ant;

pub fn resolve_collisions(mut query: Query<&mut Transform, With<Ant>>) {}
