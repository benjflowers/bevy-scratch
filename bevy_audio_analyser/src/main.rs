use bevy::prelude::*;

mod ui;
mod audio_player;
mod audio_controller;
mod audio_analysis;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(ui::UiPlugin)
    .add_plugins(audio_player::AudioPlayerPlugin)
    .add_plugins(audio_controller::AudioControllerPlugin)
    .add_plugins(audio_analysis::AudioAnalysisPlugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, mut audio_controller: ResMut<audio_controller::AudioController>,) {
  commands.spawn(Camera2d);

  if let Err(e) = audio_controller.load_sound("assets/music/apr_13_agn.wav") {
    error!("Failed to load audio: {}", e);
  } else {
    info!("Audio file loaded in setup");
  }
}