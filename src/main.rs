use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, mouse_button_input)
        .run();
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
