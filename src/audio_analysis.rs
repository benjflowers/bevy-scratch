use std::i16;

use bevy::render::render_resource::encase::private::Length;
use hound;

fn read_audio_file() -> Result<Vec<f32>, Box<dyn std::error::Error>> {
  let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
  .join("src")
  .join("music")
  .join("apr_13_agn.wav");

  let mut reader = hound::WavReader::open(path)?;
  let _spec = reader.spec();
  let samples: Vec<f32> = reader.samples::<i16>()
    .map(|s| s.unwrap() as f32 / i16::MAX as f32)
    .collect();
  Ok(samples)
}

pub fn main() {
  let audio_data = read_audio_file().unwrap();
  println!("audio file length {}", audio_data.length());
}