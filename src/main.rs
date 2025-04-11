use bevy::{ prelude::*, input::mouse::MouseMotion, render::mesh::Mesh };
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup).chain())
        .add_systems(Update, mouse_button_input)
        .add_systems(Update, mouse_motion)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(40.0, 90.0, 0.5)))),
    ));
}

fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        println!("Left mouse button pressed");
    }
    if buttons.just_released(MouseButton::Left) {
        println!("Left mouse button released");
    }
}

fn mouse_motion(
    mut event_reader: EventReader<MouseMotion>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut clear: ResMut<ClearColor>,
) {
    for event in event_reader.read() {
        println!("Mouse moved: X: {} px, Y: {} px", event.delta.x, event.delta.y);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(50.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(40.0, 90.0, 0.5)))),
            Transform::default().with_translation(Vec3::new(event.delta.x, event.delta.y, 0.0))
        ));
        clear.0 = Color::hsl(10.0, 10.0, 0.5);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
