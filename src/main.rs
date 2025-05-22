use bevy::prelude::*;
use bevy::window::WindowMode;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize, Clone)]
struct AppEntry {
    name: String,
    exec: String,
    icon: String,
    category: String,
}

#[derive(Resource)]
struct AppConfig {
    categories: HashMap<String, Vec<AppEntry>>,
    current_tab: usize,
    current_selection: usize,
    tab_names: Vec<String>,
}

#[derive(Component)]
struct AppTab;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GJTV".to_string(),
                resolution: (1920., 1080.).into(),
                resizable: false,
                decorations: false,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(load_app_config("config.toml"))
        .add_systems(Startup, (setup_camera, setup_ui))
        .add_systems(Update, (input_handler))
        .run();
}
#[derive(Debug, Deserialize)]
struct AppList {
    apps: Vec<AppEntry>,
}

fn load_app_config(path: &str) -> AppConfig {
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

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_ui(mut commands: Commands, config: ResMut<AppConfig>, asset_server: Res<AssetServer>) {
    let current_tab = &config.tab_names[config.current_tab];
    let apps = &config.categories[current_tab];

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            AppTab,
        ))
        .with_children(|parent| {
            for (i, app) in apps.iter().enumerate() {
                let color = if i == config.current_selection {
                    Color::linear_rgb(1., 1., 0.)
                } else {
                    Color::WHITE
                };
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    children![
                        ImageNode {
                            color: Color::WHITE,
                            image: asset_server.load(&app.icon),
                            texture_atlas: None,
                            flip_x: false,
                            flip_y: false,
                            rect: None,
                            image_mode: NodeImageMode::default(),
                        },
                        (Text::new(app.name.clone()), TextColor(color.into()))
                    ],
                ));
            }
        });
}

fn input_handler(
    keys: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<AppConfig>,
    mut commands: Commands,
    query: Query<Entity, With<AppTab>>,
    asset_server: Res<AssetServer>,
) {
    let mut needs_redraw = false;

    if keys.just_pressed(KeyCode::ArrowLeft) {
        config.current_tab =
            (config.current_tab + config.tab_names.len() - 1) % config.tab_names.len();
        config.current_selection = 0;
        needs_redraw = true;
    }

    if keys.just_pressed(KeyCode::ArrowRight) {
        config.current_tab = (config.current_tab + 1) % config.tab_names.len();
        config.current_selection = 0;
        needs_redraw = true;
    }

    if keys.just_pressed(KeyCode::ArrowDown) {
        config.current_selection += 1;
        needs_redraw = true;
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        if config.current_selection > 0 {
            config.current_selection -= 1;
            needs_redraw = true;
        }
    }

    if keys.just_pressed(KeyCode::Enter) {
        launch_selected_app(&config);
    }

    if needs_redraw {
        for e in query.iter() {
            commands.entity(e).despawn();
        }
        setup_ui(commands, config, asset_server);
    }
}

fn launch_selected_app(config: &AppConfig) {
    let current_tab = &config.tab_names[config.current_tab];
    if let Some(apps) = config.categories.get(current_tab) {
        if let Some(app) = apps.get(config.current_selection) {
            let _ = Command::new("sh").arg("-c").arg(&app.exec).spawn();
        }
    }
}

// fn gamepad_navigation(
//     mut events: EventReader<GamepadEvent>,
//     mut config: ResMut<AppConfig>,
//     mut commands: Commands,
//     query: Query<Entity, With<AppTab>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let mut needs_redraw = false;

//     for event in events.iter() {
//         match &event.event_type {
//             GamepadEventType::ButtonChanged(GamepadButtonType::DPadDown, v) if *v > 0.5 => {
//                 config.current_selection += 1;
//                 needs_redraw = true;
//             }
//             GamepadEventType::ButtonChanged(GamepadButtonType::DPadUp, v) if *v > 0.5 => {
//                 if config.current_selection > 0 {
//                     config.current_selection -= 1;
//                     needs_redraw = true;
//                 }
//             }
//             GamepadEventType::ButtonChanged(GamepadButtonType::DPadLeft, v) if *v > 0.5 => {
//                 config.current_tab =
//                     (config.current_tab + config.tab_names.len() - 1) % config.tab_names.len();
//                 config.current_selection = 0;
//                 needs_redraw = true;
//             }
//             GamepadEventType::ButtonChanged(GamepadButtonType::DPadRight, v) if *v > 0.5 => {
//                 config.current_tab = (config.current_tab + 1) % config.tab_names.len();
//                 config.current_selection = 0;
//                 needs_redraw = true;
//             }
//             GamepadEventType::ButtonChanged(GamepadButtonType::South, v) if *v > 0.5 => {
//                 launch_selected_app(&config);
//             }
//             _ => {}
//         }
//     }

//     if needs_redraw {
//         for e in query.iter() {
//             commands.entity(e).despawn_recursive();
//         }
//         setup_ui(commands, config, asset_server);
//     }
// }
