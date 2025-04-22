use bevy::{
    prelude::*,
    render::mesh::Mesh,
};

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
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(2.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(40.0, 90.0, 0.5))))
    ));
}

fn crawl(mut meshes: Query<&mut Transform, With<Mesh2d>>) {
    for mut transform in &mut meshes {
        transform.translation.x += 0.5;
        transform.translation.y += 0.5;
    }
}
