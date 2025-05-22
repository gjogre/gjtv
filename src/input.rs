use crate::app_config::AppConfig;
use crate::components::AppTab;
use crate::launcher::launch_selected_app;
use crate::ui::setup_ui;
use bevy::prelude::*;

pub fn input_handler(
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
