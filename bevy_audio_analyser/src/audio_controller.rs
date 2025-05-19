use bevy::prelude::*;
use kira::{
  manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
  CommandError,
};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::thread;

enum AudioCommand {
  LoadSound(String),
  Play,
  Stop,
  Shutdown,
}

#[derive(Resource, Default)]
pub struct AudioController {
    sender: Option<Sender<AudioCommand>>,
    is_playing: bool,
    current_position: f64, // in seconds
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

  pub fn play(&mut self) -> Result<(), String> {
    if let Some(sender) = &self.sender {
      self.is_playing = true;
      self.current_position = 0.0;
      sender.send(AudioCommand::Play)
        .map_err(|_| "Failed to send play command".to_string())
    } else {
      Err("Audio controller not initialized".to_string())
    }
  }
}

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<AudioController>()
      .add_systems(Startup, setup_audio_controller);
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
                      } else {
                        warn!("Attempted to play, but no sound is loaded");
                      }
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