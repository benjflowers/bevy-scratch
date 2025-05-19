use bevy::prelude::*;

mod ui;
mod audio_player;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(ui::UiPlugin)
    .add_plugins(audio_player::AudioPlayerPlugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2d);
}