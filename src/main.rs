use bevy::{
    prelude::*,
    render::mesh::Mesh,
};

use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, spawn_circle))
        .add_systems(Update, crawl)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mut i = 0;
    while i < 100 {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(2.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(40.0, 90.0, 0.5))))
        ));
        i = i + 1
    }
}

fn crawl(mut meshes: Query<&mut Transform, With<Mesh2d>>) {
    let mut rng = rand::rng();

    for mut transform in &mut meshes {
        transform.translation.x += rng.random_range(-1.0..1.0);
        transform.translation.y += rng.random_range(-1.0..1.0);
    }
}
