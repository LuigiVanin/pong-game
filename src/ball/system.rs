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
    // println!("{:?}", query);
    for (transform, mut velocity) in query.iter_mut() {
        print!("y: {:?}", transform.translation.y);
        println!("x: {:?}", transform.translation.x);

        if transform.translation.y + BALL_RADIUS > WINDOW_HEIGHT / 2.
            || transform.translation.y - BALL_RADIUS < -(WINDOW_HEIGHT / 2.)
        {
            println!("Ball colision with top");
            velocity.y = -velocity.y;
        }

        if transform.translation.x + BALL_RADIUS > WINDOW_WIDTH / 2.
            || transform.translation.x - BALL_RADIUS < -(WINDOW_WIDTH / 2.)
        {
            velocity.x = -velocity.x;
        }
    }
}
