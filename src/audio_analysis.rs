use std::i16;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

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

// choosing a size that is a power of 2 is more efficient for the algo
// larger window = better freq resolution but worse time resolution
// smaller window = better time resolution but worse frequency resolution
const WINDOW_SIZE: usize = 1024;
// hop size covers 50% of the window
// can captcher smaller moments missed
// apparently 'standard practice'
const HOP_SIZE: usize = 512;

fn generate_spectrogram(audio_data: &[f32]) -> Vec<Vec<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(WINDOW_SIZE);
    let mut spectrogram = Vec::new();
    for start in (0..audio_data.len() - WINDOW_SIZE).step_by(HOP_SIZE) {
        let window = &audio_data[start..start + WINDOW_SIZE];
        let mut buffer: Vec<Complex<f32>> = window.iter().map(|&x| Complex::new(x, 0.0)).collect();
        fft.process(&mut buffer);
        
        let magnitudes: Vec<f32> = buffer.iter().map(|c| c.norm()).collect();
        spectrogram.push(magnitudes);
    }
    spectrogram
}

pub fn main() {
  let audio_data = read_audio_file().unwrap();
  let spectrogram = generate_spectrogram(&audio_data);
  println!("spectrogram legnth {}", spectrogram.len());
}