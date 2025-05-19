use bevy::prelude::*;
use kira::{
  manager::{backend::DefaultBackend, AudioManger, AudioManagerSettings},
  CommandError,
};

#[derive(Resource, Default)]
pub struct AudioController {
  manager: Option<AudioManager>,
  is_playing: bool,
  current_position: f64, // in seconds
}