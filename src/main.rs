use bevy::prelude::*;

mod app_config;
mod components;
mod input;
mod launcher;
mod ui;

use app_config::{load_app_config, move_on_focus, setup_camera};
use input::input_handler;
use ui::setup_ui;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GJTV".to_string(),
                resolution: (1920., 1080.).into(),
                decorations: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(load_app_config("config.toml"))
        .add_systems(Startup, (setup_camera, setup_ui))
        .add_systems(Update, (input_handler, move_on_focus))
        .run();
}
