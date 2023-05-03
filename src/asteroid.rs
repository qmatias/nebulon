use bevy::prelude::*;
use rand::prelude::*;

use crate::assets::GameAssets;

#[derive(Bundle, Default)]
pub struct AsteroidBundle {
    pub asteroid: Asteroid,

    #[bundle]
    pub(crate) sprite_bundle: SpriteBundle,
}

#[derive(Component, Default)]
pub struct Asteroid;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_asteroids);
    }
}

fn spawn_asteroids(mut commands: Commands, assets: Res<GameAssets>) {
    let count = 50;
    let max_dist = 500.0;

    let mut rng = thread_rng();

    for _ in 0..count {
        // choose random texture, position, and size
        let texture = assets.asteroids.choose(&mut rng).unwrap();
        let x = rng.gen_range(-max_dist..max_dist);
        let y = rng.gen_range(-max_dist..max_dist);
        let scl = rng.gen_range(0.1..1.0);

        // create the asteroid sprite
        commands.spawn(AsteroidBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0)
                    .with_scale(Vec3::new(scl, scl, scl)),
                texture: texture.clone(),
                ..default()
            },
            ..default()
        });
    }
}
