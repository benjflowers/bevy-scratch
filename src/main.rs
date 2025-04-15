use bevy::{ prelude::*, input::mouse::MouseButton, render::mesh::Mesh, window::PrimaryWindow };
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<SpawnCircleEvent>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (mouse_input_system, spawn_circle_system))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Event)]
struct SpawnCircleEvent {
    window_position: Vec2, // Raw cursor position
    world_position: Vec2, // Transformed position for rendering
    window_size: Vec2, // Current Window size
}

fn mouse_input_system(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut circle_events: EventWriter<SpawnCircleEvent>
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = q_windows.single();
        if let Some(click_position) = window.cursor_position() {
            // get window dimenstions
            let window_size = Vec2::new(window.width(), window.height());

            // calculated world position to pass to transform
            let world_position = calculate_world_position(click_position, window_size);
            circle_events.send(SpawnCircleEvent {
                window_position: click_position,
                world_position,
                window_size,
            });
        } 
    }
}

fn spawn_circle_system(
    mut circle_events: EventReader<SpawnCircleEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in circle_events.read() {
        let color_augment = event.world_position.x * 0.002;
        println!("Spawn circle at: {:?}", event.world_position);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(50.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(40.0 + color_augment, 90.0, 0.5)))),
            Transform::default().with_translation(Vec3::new(event.world_position.x, event.world_position.y, 0.0)),
        ));
    }
}

fn calculate_world_position(click_position: Vec2, window_size: Vec2) -> Vec2 {
    Vec2::new(
        click_position.x - window_size.x / 2.0,
        window_size.y / 2.0 - click_position.y
    )
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

// fn mouse_button_input(
//     buttons: Res<ButtonInput<MouseButton>>,
//     q_windows: Query<&Window, With<PrimaryWindow>>,
//     commands: Commands,
//     meshes: ResMut<Assets<Mesh>>,
//     materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     if buttons.just_pressed(MouseButton::Left) {
//         let pos = current_pos(q_windows);
//         let windowSize: Vec2 = get_resolution(Query<&Window, With<PrimaryWindow>>);
//         let transform_calc = calc_pos_from_origin(pos, windowSize);
//         spawn_circle(commands, meshes, materials,pos);
//     }
// }

// fn calc_pos_from_origin(pos: Vec2, windowSize: Vec2) {
//     // window pos - top left is 0,0
//     // origin pos - center is 0,0

//     // if click pos is 50, 50
//     // calculate window width and height
//     // originPos is 1/2  of height and width always
//     // clickPos - originPos = newPos
//     println!("Window Size: {:?}", windowSize);
//     println!("Click Position: {:?}", pos);
// }

// fn get_resolution(q_windows: Query<&Window, With<PrimaryWindow>>) -> Vec2 {
//     let result;
//     let window = q_windows.single();
//     result = Vec2::new(window.width(), window.height());
//     // println!("Window Size: {:?}", result);
//     return result;
// }

// fn current_pos(
//     q_windows: Query<&Window, With<PrimaryWindow>>,
// ) -> Vec2 {
//     if let Some(position) = q_windows.single().cursor_position() {
//         // println!("Current Position {:?
//         // }", position);
//         return position;
//     } else {
//         println!("Click was outside of window");
//         return Vec2::ZERO;
//     }
// }

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

// fn cursor_position(
//     q_windows: Query<&Window, With<PrimaryWindow>>,
// ) -> (Vec2, ()) {
//     // Games typically only have one window (the primary window)
//     if let Some(position) = q_windows.single().cursor_position() {
//         println!("Cursor is inside the primary window, at {:?}", position);
//         return (position,());
//     } else {
//         println!("Cursor is not in the game window.");
//         return (Vec2::ZERO,());
//     }
// }

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
