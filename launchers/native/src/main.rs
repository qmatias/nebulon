use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use std::io::Cursor;
use winit::window::Icon;

fn set_window_icon(
    primary_query: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let entity = primary_query.single();
    let window = windows
        .get_window(entity)
        .expect("should have primary window");

    let buffer = Cursor::new(include_bytes!("../static/appicon.png"));
    let img = image::load(buffer, image::ImageFormat::Png)
        .expect("icon path should be valid")
        .into_rgba8();

    let (width, height) = img.dimensions();
    let rgba = img.into_raw();
    let icon = Icon::from_rgba(rgba, width, height)
        .expect("icon should have valid rgba");

    window.set_window_icon(Some(icon));
}

fn main() {
    let mut app = game::app(false);

    info!("Starting launcher: Native");
    app.add_startup_system(set_window_icon);
    app.run();
}
