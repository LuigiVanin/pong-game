use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::{ball::components::Ball, paddle::components::Paddle};

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
    paddle_query: Query<(&mut Transform,), Without<Ball>>,
) {
    let ball = ball_query.single_mut();

    for (transform,) in paddle_query.iter() {
        if collide(
            transform.translation,
            Vec2::new(100., 20.),
            ball.1.translation,
            Vec2::new(20., 20.),
        )
        .is_some()
        {
            println!("Collision!")
        }
    }
}
