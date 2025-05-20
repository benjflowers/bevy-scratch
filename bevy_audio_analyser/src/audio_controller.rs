use bevy::prelude::*;
use kira::{
  manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
  CommandError,
};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use crate::ui::AudioPlaybackEvent;

enum AudioCommand {
  LoadSound(String),
  Play,
  Stop,
  Shutdown,
}

#[derive(Resource, Default)]
pub struct AudioController {
    sender: Option<Sender<AudioCommand>>,
    pub is_playing: bool,
    pub current_position: f64, // in seconds
    start_time: f64
}

impl AudioController {
  pub fn load_sound(&mut self, path: &str) -> Result<(), String> {
    if let Some(sender) = &self.sender {
      sender.send(AudioCommand::LoadSound(path.to_string()))
        .map_err(|_| "failed to send load command".to_string())
    } else {
      Err("Audio controller to initialized".to_string())
    }
  }

  pub fn play(&mut self, time: &Time) -> Result<(), String> {
    if let Some(sender) = &self.sender {
      self.start_time = time.elapsed_secs_f64();
      self.is_playing = true;
      self.current_position = 0.0;
      sender.send(AudioCommand::Play)
        .map_err(|_| "Failed to send play command".to_string())
    } else {
      Err("Audio controller not initialized".to_string())
    }
  }

  pub fn update_position(&mut self, current_time: f64) {
    if self.is_playing {
      self.current_position = current_time - self.start_time;
    }
  }
}

fn handle_ui_events(
  mut events: EventReader<AudioPlaybackEvent>,
  mut controller: ResMut<AudioController>,
  time: Res<Time>
) {
  for _ in events.read() {
    info!("Play button clicked, starting playback");
    if let Err(e) = controller.play(&time) {
      error!("Failed to play audio: {}", e);
    }
  }
}

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<AudioController>()
      .add_systems(Startup, setup_audio_controller)
      .add_systems(Update, (handle_ui_events, update_audio_position));
  }
}

fn update_audio_position(
  time: Res<Time>,
  mut controller: ResMut<AudioController>,
) {
  if controller.is_playing {
    let current_time = time.elapsed_secs_f64();
    controller.update_position(current_time);
  }
}

fn debug_audio_position(controller: Res<AudioController>) {
  if controller.is_playing {
      // Only print once per second to avoid flooding the console
      if (controller.current_position * 10.0).round() % 10.0 == 0.0 {
          info!("Audio position: {:.1}s", controller.current_position);
      }
  }
}

fn setup_audio_controller(mut controller: ResMut<AudioController>) {
  let (sender, receiver) = channel::<AudioCommand>();
  controller.sender = Some(sender);

  thread::spawn(move || {
    match AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()) {
      Ok(mut manager) => {
          info!("Audio controller initialized in separate thread");
          
          let mut current_sound: Option<StaticSoundData> = None;
          // Process audio commands
          while let Ok(command) = receiver.recv() {
              match command {
                  AudioCommand::LoadSound(path) => {
                      info!("Loading sound from {}", path);
                      match StaticSoundData::from_file(path, StaticSoundSettings::default()) {
                        Ok(sound) => {
                          current_sound = Some(sound);
                          info!("Sound loaded Succesfully");
                        },
                        Err(e) => error!("Failed to load sound: {:?}", e),
                      }
                  },
                  AudioCommand::Play => {
                    if let Some(sound) = &current_sound {
                      match manager.play(sound.clone()) {
                        Ok(_) => info!("Sound started playing"),
                        Err(e) => error!("Failed to play sound: {:?}", e),
                      }
                    } else {
                      warn!("Attempted to play, but no sound is loaded");
                    }
                  },
                  AudioCommand::Stop => {
                    // We'll implement this later
                    info!("Stop command received");
                  },
                  AudioCommand::Shutdown => {
                    info!("Shutting down audio thread");
                    break;
                  },
              }
          }
      },
      Err(e) => {
          error!("Failed to initialize audio manager: {:?}", e);
      }
  }
  });
}