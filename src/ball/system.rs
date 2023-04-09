use bevy::prelude::*;
use rand::Rng;

use crate::{
    config::{
        window::{WINDOW_HEIGHT, WINDOW_WIDTH},
        BALL_RADIUS,
    },
    global::components::{Scoreboard, Velocity},
};

use super::components::{Ball, SpawnTimer};

pub fn ball_collision(
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    mut scoreboard: ResMut<Scoreboard>,
    mut time: ResMut<SpawnTimer>,
    _time: Res<Time>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        if transform.translation.y + BALL_RADIUS > WINDOW_HEIGHT / 2.
            || transform.translation.y - BALL_RADIUS < -(WINDOW_HEIGHT / 2.)
        {
            velocity.y = -velocity.y;
        }

        let has_passed_left = transform.translation.x + BALL_RADIUS > WINDOW_WIDTH / 2.;
        let has_passed_right = transform.translation.x - BALL_RADIUS < -(WINDOW_WIDTH / 2.);

        let [mut left_player, mut right_player] = scoreboard.0;
        if !(has_passed_left || has_passed_right) {
            continue;
        }
        if has_passed_left {
            left_player += 1;
        } else if has_passed_right {
            right_player += 1;
        }
        scoreboard.0 = [left_player, right_player];
        velocity.x = -velocity.x;
        reset_ball_position(&mut transform);
        println!("Scoreboard| {:?}", scoreboard.0);
        time.timer.reset();
    }
}

fn reset_ball_position(transform: &mut Transform) {
    let mut rnd = rand::thread_rng();

    let y_pos = rnd.gen_range(-200..200);
    transform.translation.x = 0.;
    transform.translation.y = y_pos as f32;
}
