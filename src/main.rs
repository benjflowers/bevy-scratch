use bevy::{
    prelude::*,
    render::mesh::Mesh,
};

use rand::Rng;

#[derive(Component)]
struct Crawler;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, spawn_circles))
        .add_systems(Update, crawl)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_circles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mut i = 0;
    while i < 10 {
        commands.spawn((
            Crawler,
            Mesh2d(meshes.add(Circle::new(2.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(40.0, 90.0, 0.5))))
        ));
        i = i + 1
    }
}

fn crawl(
    mut existing_meshes: Query<&mut Transform, With<Crawler>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mut rng = rand::rng();

    for mut transform in &mut existing_meshes {
        let x_pos = transform.translation.x;
        let y_pos = transform.translation.y;

        transform.translation.x += rng.random_range(-1.0..1.0);
        transform.translation.y += rng.random_range(-1.0..1.0);

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(1.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(20.0, 90.0, 0.5)))),
            Transform::from_xyz(x_pos, y_pos, 0.0)
        ));
    }
}
