use bevy::core_pipeline::core_2d::Camera2d;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::prelude::{EventReader, Local};
use bevy::window::WindowFocused;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize, Clone)]
pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub category: String,
}

#[derive(Resource)]
pub struct AppConfig {
    pub categories: HashMap<String, Vec<AppEntry>>,
    pub current_tab: usize,
    pub current_selection: usize,
    pub tab_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AppList {
    apps: Vec<AppEntry>,
}

pub fn load_app_config(path: &str) -> AppConfig {
    let content = fs::read_to_string(path).expect("Failed to read config.toml");
    let app_list: AppList = toml::from_str(&content).expect("Invalid TOML");

    let mut categories: HashMap<String, Vec<AppEntry>> = HashMap::new();
    for app in app_list.apps {
        categories
            .entry(app.category.clone())
            .or_default()
            .push(app);
    }

    let mut tab_names: Vec<String> = categories.keys().cloned().collect();
    tab_names.sort();

    AppConfig {
        categories,
        current_tab: 0,
        current_selection: 0,
        tab_names,
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
pub fn move_on_focus(mut events: EventReader<WindowFocused>, mut done: Local<bool>) {
    if *done {
        return;
    }

    for event in events.read() {
        if event.focused {
            let _ = Command::new("hyprctl")
                .args(["dispatch", "movetoworkspace", "special:z"])
                .status();
            *done = true;
        }
    }
}
