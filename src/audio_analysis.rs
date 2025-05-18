use std::i16;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

use hound;

#[derive(Debug, Clone)]
pub struct FrequencyAnalysis {
  pub bass: f32,
  pub mids: f32,
  pub highs: f32,
}

pub fn read_audio_file() -> Result<Vec<f32>, Box<dyn std::error::Error>> {
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

pub fn generate_spectrogram(audio_data: &[f32]) -> Vec<Vec<f32>> {
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

pub fn analyze_frequency_bands(spectrogram: &Vec<Vec<f32>>, _sample_rate: f32) -> Vec<FrequencyAnalysis> {
  let mut results = Vec::new();
  
  // For each time window
  for window_data in spectrogram {
      // Define frequency ranges (in bin indices)
      // Example: for 44.1kHz and 1024-sample window
      let bass_range = 1..12;        // ~43Hz to ~516Hz
      let mid_range = 12..93;        // ~516Hz to ~4,000Hz
      let high_range = 93..232;      // ~4,000Hz to ~10,000Hz
      
      // Calculate energy in each band
      let bass_energy = calculate_band_energy(window_data, &bass_range);
      let mid_energy = calculate_band_energy(window_data, &mid_range);
      let high_energy = calculate_band_energy(window_data, &high_range);
      
      results.push(FrequencyAnalysis {
          bass: bass_energy,
          mids: mid_energy,
          highs: high_energy,
          // You could add more analysis here
      });
  }
  
  results
}

fn calculate_band_energy(window_data: &Vec<f32>, range: &std::ops::Range<usize>) -> f32 {
  let mut sum = 0.0;
  for i in range.clone() {
      if i < window_data.len() {
          sum += window_data[i];
      }
  }
  
  // Average and normalize
  if range.end > range.start {
      sum / (range.end - range.start) as f32
  } else {
      0.0
  }
}