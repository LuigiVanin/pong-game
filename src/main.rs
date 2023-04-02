mod ball;
mod config;
mod globals;
mod setup;
use crate::config::window::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::config::TIME_STEP;
use crate::globals::apply_velocity_system;
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
        .add_systems((apply_velocity_system,).in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .run();
}