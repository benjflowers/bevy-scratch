use bevy::{
    prelude::*,
    render::mesh::Mesh,
};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup))
        .run();
}

#[derive(Component)]

struct Crawler{
    pos: Vec2,
    size: f32
}

impl Crawler {
    fn random(rand: &mut ChaCha8Rng) -> Self {
        Crawler {
            pos: Vec2::new(
                (rand.random::<f32>()),
                (rand.random::<f32>()),
            ),
            size: 4.0
        }
    }
}

fn setup(
    mut commands: Commands
) {
    // spawn camera
    commands.spawn(Camera2d);
}