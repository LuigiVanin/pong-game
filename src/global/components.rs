use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource)]
pub struct Scoreboard(pub [u8; 2]);
