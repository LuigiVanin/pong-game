use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    ball::components::{Ball, SpawnTimer},
    config::{PADDLE_HEIGHT, PADDLE_WIDTH},
    global::components::Scoreboard,
    paddle::components::{PlayerPosition, Position},
};

use super::components::Velocity;

pub fn update_scoreboard(score: Res<Scoreboard>, mut score_text: Query<(&mut Text,)>) {
    let (mut text,) = score_text.single_mut();
    let [left_score, right_score] = score.0;
    text.sections[0].value = format!("{} | {}", left_score, right_score);
}

pub fn apply_velocity_system(
    mut spawn_timer: ResMut<SpawnTimer>,
    mut ball_query: Query<(&mut Transform, &Velocity), With<Ball>>,
    mut paddle_query: Query<(&mut Transform, &Velocity, &PlayerPosition), Without<Ball>>,
    time: Res<Time>,
) {
    for (mut transform, velocity, _) in paddle_query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }

    if !spawn_timer.timer.tick(time.delta()).finished() {
        return;
    }

    for (mut transform, velocity) in ball_query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

pub fn collision_effect_system(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    paddle_query: Query<(&mut Transform, &PlayerPosition), Without<Ball>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();

    for (transform, pos) in paddle_query.iter() {
        if collide(
            transform.translation,
            Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            ball_transform.translation,
            Vec2::new(20., 20.),
        )
        .is_some()
        {
            println!(
                "collision | ball_x: {:?}; paddle_x: {:?}; paddle_y: {:?}",
                ball_transform.translation.x, transform.translation.x, transform.translation.y,
            );
            if ball_transform.translation.x < -25. || ball_transform.translation.x > 25. {
                match pos {
                    PlayerPosition(Position::Left) => ball_velocity.x = ball_velocity.x.abs(),
                    PlayerPosition(Position::Right) => ball_velocity.x = -(ball_velocity.x.abs()),
                }
            }
        }
    }
}
