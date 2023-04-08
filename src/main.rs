mod ball;
mod config;
mod globals;
mod paddle;
mod setup;
use crate::ball::components::SpawnTimer;
use crate::ball::system::ball_collision;
use crate::config::window::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::config::TIME_STEP;
use crate::globals::{apply_velocity_system, collision_effect_system, Scoreboard};
use crate::paddle::system::{paddle_boundery_check, paddle_movement_ketboard_event};
use crate::setup::setup_system;
use bevy::prelude::*;

fn update_test_system(time: Res<Time>) {
    println!("Time: {}", time.delta_seconds());
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".to_string(),
                resizable: false,
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup_system)
        .add_systems(
            (
                collision_effect_system,
                apply_velocity_system,
                ball_collision,
                paddle_movement_ketboard_event,
                paddle_boundery_check,
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .insert_resource(Scoreboard([0, 0]))
        .insert_resource(SpawnTimer {
            timer: Timer::from_seconds(2., TimerMode::Once),
        })
        .run();
}
