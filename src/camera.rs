use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_plugin(PanCamPlugin::default());
    }
}

fn spawn_camera(mut commands: Commands) {
    // TODO use leafwing_input_manager or similar for input handling
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        grab_buttons: vec![MouseButton::Left],
        min_scale: 0.01,
        ..default()
    });
}
