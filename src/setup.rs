use crate::config::*;
use crate::{ball::components::*, globals::Velocity};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((BallBundle {
        mesh: MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(Vec3::new(BALL_START_X, BALL_START_Y, 0.)),
            ..default()
        },
        velocity: Velocity {
            x: BALL_START_X_VELOCITY,
            y: BALL_START_Y_VELOCITY,
        },
        ball: Ball,
    },));
}
