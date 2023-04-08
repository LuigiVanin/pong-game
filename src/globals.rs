use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::{
    ball::components::Ball,
    config::{PADDLE_HEIGHT, PADDLE_WIDTH},
    paddle::components::{PlayerPosition, Position},
};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub fn apply_velocity_system(mut query: Query<(&mut Transform, &Velocity)>, _time: Res<Time>) {
    // println!("{:?}", query);
    for (mut transform, velocity) in query.iter_mut() {
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
