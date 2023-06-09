mod assets;
mod asteroid;
mod camera;
mod miner;
mod shop;

use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_asset_loader::prelude::*;
use bevy_egui::EguiPlugin;

use crate::assets::GameAssets;
use crate::asteroid::AsteroidPlugin;
use crate::camera::GameCameraPlugin;
use crate::miner::MinerPlugin;
use crate::shop::ShopPlugin;

pub const LAUNCHER_TITLE: &str = "Nebulon";

pub fn app(fullscreen: bool) -> App {
    let mode = match fullscreen {
        true => WindowMode::BorderlessFullscreen,
        false => WindowMode::Windowed,
    };
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: LAUNCHER_TITLE.to_string(),
            // canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            mode,
            ..default()
        }),
        ..default()
    }))
    .add_plugin(EguiPlugin)
    .init_collection::<GameAssets>()
    .add_plugin(ShopPlugin)
    .add_plugin(GameCameraPlugin)
    .add_plugin(MinerPlugin)
    .add_plugin(AsteroidPlugin);
    app
}
