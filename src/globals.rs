use bevy::prelude::*;

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
