use std::path::Path;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiSettings};
use project::{Project, ProjectMgr, ProjectPlugin};

mod color;
mod image;
mod keybinds;
mod project;
mod tools;
mod ui;
mod view;

fn main() {
    dotenvy::dotenv().ok();

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                title: env!("CARGO_PKG_NAME").to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(ProjectPlugin)
        .add_plugin(view::Plugin)
        .add_plugin(ui::Plugin)
        .add_plugin(keybinds::Plugin)
        .add_plugin(tools::Plugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut egui_settings: ResMut<EguiSettings>,
    mut project_mgr: ResMut<ProjectMgr>,
) {
    commands.spawn(Camera2dBundle::default());
    egui_settings.scale_factor = 1.5;

    if let Ok(path) = std::env::var("NEW_PROJECT_INPUT_PATH") {
        project_mgr.current = Project::new_from_input_path(Path::new(&path)).ok()
    }
}
