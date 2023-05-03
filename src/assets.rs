use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(
        paths(
            "sprites/meteorBrown_big1.png",
            "sprites/meteorBrown_big2.png",
            "sprites/meteorBrown_big3.png",
            "sprites/meteorBrown_big4.png"
        ),
        collection(typed)
    )]
    pub asteroids: Vec<Handle<Image>>,

    #[asset(path = "sprites/miner.png")]
    pub miner: Handle<Image>,
}
