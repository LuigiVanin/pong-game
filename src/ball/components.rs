use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    pub velocity: Velocity,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub ball: Ball,
}
