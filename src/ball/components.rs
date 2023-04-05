use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::globals::Velocity;

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    pub velocity: Velocity,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub ball: Ball,
}
