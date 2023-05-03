use bevy::prelude::*;

use crate::assets::GameAssets;

pub struct MinerPlugin;

impl Plugin for MinerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_miner);
    }
}

#[derive(Component, Default)]
pub struct Miner;

#[derive(Bundle, Default)]
pub struct MinerBundle {
    pub miner: Miner,

    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

fn spawn_miner(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(MinerBundle {
        sprite_bundle: SpriteBundle {
            texture: assets.miner.clone(),
            ..default()
        },
        ..default()
    });
}
