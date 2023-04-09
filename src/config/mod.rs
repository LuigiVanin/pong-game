pub mod text;
pub mod window;

use bevy::prelude::Color;

use self::window::WINDOW_WIDTH;

pub const BALL_RADIUS: f32 = 10.;
pub const BALL_COLOR: Color = Color::WHITE;
pub const BALL_START_X: f32 = 0.;
pub const BALL_START_Y: f32 = 0.;
pub const BALL_START_X_VELOCITY: f32 = 5.;
pub const BALL_START_Y_VELOCITY: f32 = 3.;
pub const PADDLE_WIDTH: f32 = 10.;
pub const PADDLE_HEIGHT: f32 = 70.;
pub const PADDLE_COLOR: Color = Color::WHITE;
pub const PADDLE_LEFT_START_X: f32 = -WINDOW_WIDTH / 2. + 30.;
pub const PADDLE_LEFT_START_Y: f32 = -0.;
pub const PADDLE_RIGHT_START_X: f32 = WINDOW_WIDTH / 2. - 30.;
pub const PADDLE_RIGHT_START_Y: f32 = 0.;
pub const PADDLE_SPEED: f32 = 5.;
pub const TIME_STEP: f32 = 1. / 60.;
