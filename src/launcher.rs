use crate::app_config::AppConfig;

use std::process::Command;
use std::str;
use std::thread;
use std::time::Duration;
pub fn launch_selected_app(config: &AppConfig) {
    let current_tab = &config.tab_names[config.current_tab];
    if let Some(apps) = config.categories.get(current_tab) {
        if let Some(app) = apps.get(config.current_selection) {
            let workspace = find_next_empty_workspace();
            launch_app_in_workspace(&app.exec, workspace);
        }
    }
}

fn find_next_empty_workspace() -> i32 {
    let output = Command::new("hyprctl")
        .args(["workspaces", "-j"])
        .output()
        .expect("Failed to execute hyprctl command");

    if !output.status.success() {
        return 1; // Default to workspace 1 if command fails
    }

    let workspaces_json = str::from_utf8(&output.stdout).unwrap_or("");

    let mut occupied_ids = Vec::new();
    for line in workspaces_json.lines() {
        if line.contains("\"id\":") {
            if let Some(id_str) = line.split(':').nth(1) {
                if let Some(id_clean) = id_str
                    .trim()
                    .trim_matches(',')
                    .trim_matches('"')
                    .parse::<i32>()
                    .ok()
                {
                    occupied_ids.push(id_clean);
                }
            }
        }
    }

    let mut next_id = 1;
    while occupied_ids.contains(&next_id) {
        next_id += 1;
    }

    next_id
}

fn launch_app_in_workspace(app_cmd: &str, workspace: i32) {
    // Toggle special off
    let _ = Command::new("hyprctl")
        .args(["dispatch", "togglespecialworkspace", "z"])
        .status()
        .expect("Failed to toggle special workspace");

    // Switch to the target workspace
    let _ = Command::new("hyprctl")
        .args(["dispatch", "workspace", &workspace.to_string()])
        .status()
        .expect("Failed to switch workspace");

    thread::sleep(Duration::from_millis(100));

    let _ = Command::new("sh")
        .arg("-c")
        .arg(app_cmd)
        .spawn()
        .expect("Failed to launch application");
}
