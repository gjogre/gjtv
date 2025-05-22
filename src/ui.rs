use crate::app_config::AppConfig;
use crate::components::AppTab;
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, config: ResMut<AppConfig>, asset_server: Res<AssetServer>) {
    let current_tab = &config.tab_names[config.current_tab];
    let apps = &config.categories[current_tab];

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
