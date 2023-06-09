use crate::config::text::{
    FONT_COLOR, FONT_PATH, FONT_SIZE, TEXT_LEFT_POSITION, TEXT_TOP_POSITION,
};
use crate::config::*;
use crate::paddle::components::{Paddle, PaddleBundle, PlayerPosition, Position};
use crate::{ball::components::*, global::components::Velocity};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
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

    commands.spawn((PaddleBundle {
        mesh: MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.).into())
                .into(),
            material: materials.add(ColorMaterial::from(PADDLE_COLOR)),
            transform: Transform::from_translation(Vec3::new(
                PADDLE_LEFT_START_X,
                PADDLE_LEFT_START_Y,
                0.,
            )),
            ..default()
        },
        paddle: Paddle,
        velocity: Velocity { x: 0., y: 0. },
        player_position: PlayerPosition(Position::Left),
    },));

    commands.spawn((PaddleBundle {
        mesh: MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.).into())
                .into(),
            material: materials.add(ColorMaterial::from(PADDLE_COLOR)),
            transform: Transform::from_translation(Vec3::new(
                PADDLE_RIGHT_START_X,
                PADDLE_RIGHT_START_Y,
                0.,
            )),
            ..default()
        },
        paddle: Paddle,
        velocity: Velocity { x: 0., y: 0. },
        player_position: PlayerPosition(Position::Right),
    },));

    commands.spawn(
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: FONT_SIZE,
            color: FONT_COLOR,
            font: asset_server.load(FONT_PATH),
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: TEXT_TOP_POSITION,
                left: TEXT_LEFT_POSITION,
                ..default()
            },
            ..default()
        }),
    );
}
