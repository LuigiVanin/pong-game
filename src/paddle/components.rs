use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::config::window::WINDOW_WIDTH;
use crate::config::{PADDLE_LEFT_START_X, PADDLE_RIGHT_START_X};
use crate::global::components::Velocity;

#[derive(Component)]
pub struct Paddle;

#[derive(Debug, PartialEq, Eq)]

pub enum Position {
    Right,
    Left,
}

#[derive(Component, Debug)]
pub struct PlayerPosition(pub Position);

#[derive(Bundle)]
pub struct PaddleBundle {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub velocity: Velocity,
    pub paddle: Paddle,
    pub player_position: PlayerPosition,
}

impl Position {
    #[allow(dead_code)]
    pub fn get_starting_x(&self) -> f32 {
        match self {
            Position::Right => PADDLE_RIGHT_START_X,
            Position::Left => PADDLE_LEFT_START_X,
        }
    }

    #[allow(dead_code)]
    pub fn get_paddle_wall(&self) -> f32 {
        match self {
            Position::Right => WINDOW_WIDTH / 2.,
            Position::Left => -WINDOW_WIDTH / 2.,
        }
    }
}
