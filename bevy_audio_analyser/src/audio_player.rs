use bevy::{prelude::*, audio::{PlaybackSettings, AudioSource}};
use crate::ui::AudioPlaybackEvent;

pub struct AudioPlayerPlugin;

impl Plugin for AudioPlayerPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<AudioAssets>()
      .add_systems(Startup, load_audio)
      .add_systems(Update, handle_play_button);
  }
}

#[derive(Resource, Default)]
pub struct AudioAssets {
  music: Option<Handle<AudioSource>>
}

fn load_audio(mut audio_assets: ResMut<AudioAssets>, asset_server: Res<AssetServer>) {
  audio_assets.music = Some(asset_server.load("music/apr_13_agn.wav"));
  println!("audio loaded!");
}

fn handle_play_button(
  mut commands: Commands,
  audio_assets: Res<AudioAssets>,
  mut events: EventReader<AudioPlaybackEvent>,
) {
  for _ in events.read() {
    if let Some(music) = &audio_assets.music {
      commands.spawn(AudioPlayer::new(music.clone()));

      info!("Playing audio!");
    } else {
      warn!("No audio loaded!");
    }
  }
}

