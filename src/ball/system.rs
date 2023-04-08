use bevy::prelude::*;

use crate::{
    config::{
        window::{WINDOW_HEIGHT, WINDOW_WIDTH},
        BALL_RADIUS,
    },
    globals::Velocity,
};

use super::components::Ball;

pub fn ball_collision(
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    _time: Res<Time>,
) {
    for (transform, mut velocity) in query.iter_mut() {
        if transform.translation.y + BALL_RADIUS > WINDOW_HEIGHT / 2.
            || transform.translation.y - BALL_RADIUS < -(WINDOW_HEIGHT / 2.)
        {
            velocity.y = -velocity.y;
        }

        // NOTE: This will be removed when we add the paddle
        // if transform.translation.x + BALL_RADIUS > WINDOW_WIDTH / 2.
        //     || transform.translation.x - BALL_RADIUS < -(WINDOW_WIDTH / 2.)
        // {
        //     velocity.x = -velocity.x;
        // }
    }
}
