use bevy::prelude::*;

use crate::{
    config::{window::WINDOW_HEIGHT, PADDLE_HEIGHT, PADDLE_SPEED},
    globals::Velocity,
    paddle::components::Position,
};

use super::components::{Paddle, PlayerPosition};

pub fn executing_movement(
    kb: &Res<Input<KeyCode>>,
    query: &mut Query<(&mut Transform, &mut Velocity, &PlayerPosition), With<Paddle>>,
    key: KeyCode,
    position: Position,
    update_paddle_velocity: fn(&mut Velocity),
) {
    if kb.pressed(key) {
        let paddle = query.iter_mut().find(move |(_, _, pos)| match pos {
            PlayerPosition(p) if *p == position => true,
            _ => false,
        });
        let (_, mut vel, _) = paddle.unwrap();
        update_paddle_velocity(&mut vel);
    }
}

// Movement on the right side
pub fn paddle_movement_ketboard_event(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity, &PlayerPosition), With<Paddle>>,
) {
    executing_movement(&kb, &mut query, KeyCode::Up, Position::Right, |vel| {
        vel.y = PADDLE_SPEED;
    });
    executing_movement(&kb, &mut query, KeyCode::Down, Position::Right, |vel| {
        vel.y = -PADDLE_SPEED;
    });
    if kb.just_released(KeyCode::Up) || kb.just_released(KeyCode::Down) {
        let paddle = query.iter_mut().find(|(_, _, pos)| match pos {
            PlayerPosition(Position::Right) => true,
            _ => false,
        });
        let (_, mut vel, _) = paddle.unwrap();
        vel.y = 0.;
    }

    // Movement on the left side
    executing_movement(&kb, &mut query, KeyCode::W, Position::Left, |vel| {
        vel.y = PADDLE_SPEED;
    });
    executing_movement(&kb, &mut query, KeyCode::S, Position::Left, |vel| {
        vel.y = -PADDLE_SPEED;
    });

    if kb.just_released(KeyCode::W) || kb.just_released(KeyCode::S) {
        let paddle = query.iter_mut().find(|(_, _, pos)| match pos {
            PlayerPosition(Position::Left) => true,
            _ => false,
        });
        let (_, mut vel, _) = paddle.unwrap();
        vel.y = 0.;
    }
}

pub fn paddle_boundery_check(
    mut query: Query<(&mut Transform, &mut Velocity, &PlayerPosition), With<Paddle>>,
) {
    for (mut transform, mut velocity, pos) in query.iter_mut() {
        let paddle_limit = (WINDOW_HEIGHT / 2.) - (PADDLE_HEIGHT / 2.) - 20.;
        if transform.translation.y > paddle_limit {
            transform.translation.y = paddle_limit;
            velocity.y = 0.;
        }
        if transform.translation.y < -paddle_limit {
            transform.translation.y = -paddle_limit;
            velocity.y = 0.;
        }
    }
}
