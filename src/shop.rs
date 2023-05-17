use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shop_system);
    }
}

#[derive(Component, Default)]
pub struct Shop;

fn shop_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
