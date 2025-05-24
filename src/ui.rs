use crate::app_config::AppConfig;
use crate::components::AppTab;
use bevy::prelude::*;
use freedesktop_icons::lookup;
use std::fs;
use std::path::{Path, PathBuf};

pub fn setup_ui(mut commands: Commands, config: ResMut<AppConfig>, asset_server: Res<AssetServer>) {
    let current_tab = &config.tab_names[config.current_tab];
    let apps = &config.categories[current_tab];

    let icon = lookup("firefox").find();
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                row_gap: Val::Px(20.0),
                margin: UiRect::top(Val::Px(20.0)),
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
                            image: if app.icon.is_empty() {
                                load_icon(&asset_server, &app.exec)
                            } else {
                                asset_server.load(&app.icon)
                            },
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

fn load_icon(asset_server: &AssetServer, icon_name: &str) -> Handle<Image> {
    if let Some(path) = lookup(icon_name).find() {
        if let Some(asset_path) = copy_icon_to_assets(&path, icon_name) {
            return asset_server.load(&asset_path);
        }
    }

    asset_server.load("icons/testcat.png")
}

fn copy_icon_to_assets(system_icon_path: &PathBuf, icon_name: &str) -> Option<String> {
    let ext = system_icon_path.extension()?.to_str()?;
    let dest_dir = Path::new("assets/icons");
    let dest_file = dest_dir.join(format!("{}.{}", icon_name, ext));

    if dest_file.exists() {
        return Some(format!("icons/{}.{}", icon_name, ext));
    }

    std::fs::create_dir_all(dest_dir).ok()?;

    std::fs::copy(system_icon_path, &dest_file).ok()?;

    Some(format!("icons/{}.{}", icon_name, ext))
}
