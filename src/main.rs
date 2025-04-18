use bevy::{ prelude::*, input::mouse::{ MouseButton, MouseMotion}, render::mesh::Mesh, window::PrimaryWindow };
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
    circle_rad: Option<f32>,
}

fn mouse_input_system(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut circle_event_w: EventWriter<SpawnCircleEvent>,
    mut cursor_event_r: EventReader<CursorMoved>,
) {
    let window = q_windows.single();
    // get window dimenstions
    let window_size = Vec2::new(window.width(), window.height());

    if let Some(cursor_position) = window.cursor_position() {
        let world_position = calculate_world_position(cursor_position, window_size);

        if buttons.just_pressed(MouseButton::Left) {
            circle_event_w.send(SpawnCircleEvent {
                window_position: cursor_position,
                world_position,
                window_size,
                circle_rad: None,
            });
        }

        for _event in cursor_event_r.read() {
            // println!("Cursor moved to: {:?}", event.position);
            circle_event_w.send(SpawnCircleEvent {
                window_position: cursor_position,
                world_position,
                window_size,
                circle_rad: Some(5.0),
            });
        }
    }
}

fn spawn_circle_system(
    mut circle_events_r: EventReader<SpawnCircleEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in circle_events_r.read() {
        println!("Spawn circle at: {:?}", event.world_position);
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(event.circle_rad.unwrap_or(10.0)))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::hsl(40.0, 90.0, 0.5)))),
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
