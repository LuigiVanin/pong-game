use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::globals::Velocity;

#[derive(Component)]
pub struct Paddle;

#[derive(Bundle)]
pub struct PaddleBundle {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub velocity: Velocity,
    pub paddle: Paddle,
}
