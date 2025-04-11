use bevy::{ prelude::*, input::mouse::MouseMotion, render::mesh::Mesh, window::PrimaryWindow };
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera).chain())
        .add_systems(Update, mouse_button_input)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}


// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn((
//         Mesh2d(meshes.add(Circle::new(50.0))),
//         MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(40.0, 90.0, 0.5)))),
//     ));
// }

fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>
) {
    if buttons.just_pressed(MouseButton::Left) {
        current_pos(q_windows);
    }
}

fn current_pos(
    q_windows: Query<&Window, With<PrimaryWindow>>,
) -> Vec2 {
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Current Position {:?}", position);
        return position;
    } else {
        println!("Click was outside of window");
        return Vec2::ZERO;
    }
}

// fn mouse_motion(
//     mut event_reader: EventReader<MouseMotion>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut clear: ResMut<ClearColor>,
// ) {
//     for event in event_reader.read() {
//         println!("Mouse moved: X: {} px, Y: {} px", event.delta.x, event.delta.y);
//         commands.spawn((
//             Mesh2d(meshes.add(Circle::new(50.0))),
//             MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(40.0, 90.0, 0.5)))),
//             Transform::default().with_translation(Vec3::new(event.delta.x, event.delta.y, 0.0))
//         ));
//         clear.0 = Color::hsl(10.0, 10.0, 0.5);
//     }
// }

fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
) -> (Vec2, ()) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
        return (position,());
    } else {
        println!("Cursor is not in the game window.");
        return (Vec2::ZERO,());
    }
}

// fn cursor_events(
//     mut evr_cursor: EventReader<CursorMoved>,
// ) {
//     for ev in evr_cursor.read() {
//         println!(
//             "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
//             ev.position.x, ev.position.y, ev.window
//         );
//     }
// }
