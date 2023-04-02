pub mod window;

use bevy::prelude::Color;

pub const BALL_RADIUS: f32 = 10.;
pub const BALL_COLOR: Color = Color::WHITE;
pub const BALL_START_X: f32 = 100.;
pub const BALL_START_Y: f32 = 100.;
pub const BALL_START_X_VELOCITY: f32 = 3.;
pub const BALL_START_Y_VELOCITY: f32 = 3.;
pub const PADDLE_WIDTH: f32 = 10.;
pub const PADDLE_HEIGHT: f32 = 100.;
pub const PADDLE_COLOR: Color = Color::WHITE;
pub const PADDLE_START_X: f32 = 100.;
pub const PADDLE_START_Y: f32 = 100.;
pub const PADDLE_SPEED: f32 = 1.;
pub const PADDLE_START_X_VELOCITY: f32 = 0.;
pub const PADDLE_START_Y_VELOCITY: f32 = 1.;
pub const TIME_STEP: f32 = 1. / 60.;
