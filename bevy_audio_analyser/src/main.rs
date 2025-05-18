use bevy::prelude::*;

mod ui;
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(ui::UiPlugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);
}