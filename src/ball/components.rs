use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::global::components::Velocity;

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    pub velocity: Velocity,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub ball: Ball,
}
