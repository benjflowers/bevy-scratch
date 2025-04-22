use bevy::{
    prelude::*,
    render::mesh::Mesh,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, spawn_circle))
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
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(40.0, 90.0, 0.5))))
    ));
}
