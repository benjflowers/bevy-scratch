use bevy::{prelude::*, audio::{AudioSource}};
use crate::ui::AudioPlaybackEvent;

pub struct AudioPlayerPlugin;

impl Plugin for AudioPlayerPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<AudioAssets>()
      .add_systems(Startup, load_audio);
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

