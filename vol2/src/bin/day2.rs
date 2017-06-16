extern crate hound;
extern crate num;
extern crate rustfft;

use std::f32::consts::PI;
use hound::{SampleFormat, WavReader, WavSamples, WavSpec, WavWriter};
use num::complex::Complex;
use rustfft::FFTplanner;


trait Signal {
    fn energy(self) -> f64;
}

impl<'a, R> Signal for WavSamples<'a, R, i16>
where
    R: std::io::Read,
{
    fn energy(self) -> f64 {
        self.map(|x| {
            let sample = x.unwrap() as f64;
            sample * sample
        }).sum()
    }
}


fn generate_sine(filename: &str, frequency: f32, duration: u32) {
    let header = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create(filename, header).expect("Failed to created WAV writer");
    let num_samples = duration * header.sample_rate;
    let signal_amplitude = 16384f32;
    for n in 0..num_samples {
        let t: f32 = n as f32 / header.sample_rate as f32;
        let x = signal_amplitude * (t * frequency * 2.0 * PI).sin();
        writer.write_sample(x as i16).unwrap();
    }
}

fn find_spectral_peak(filename: &str) -> Option<f32> {
    let mut reader = WavReader::open(filename).expect("Failed to open WAV file");
    let num_samples = reader.len() as usize;
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(num_samples);
    let mut signal = reader
        .samples::<i16>()
        .map(|x| Complex::new(x.unwrap() as f32, 0f32))
        .collect::<Vec<_>>();
    let mut spectrum = signal.clone();
    fft.process(&mut signal[..], &mut spectrum[..]);
    let max_peak = spectrum
        .iter()
        .take(num_samples / 2)
        .enumerate()
        .max_by_key(|&(_, freq)| freq.norm() as u32);
    if let Some((i, _)) = max_peak {
        let bin = 44100f32 / num_samples as f32;
        Some(i as f32 * bin)
    } else {
        None
    }
}

fn main() {
    println!("24 Days of Rust vol. 2 - hound");
    generate_sine("test.wav", 1000f32, 5);
    let mut reader = WavReader::open("test.wav").expect("Failed to open WAV file");
    let samples = reader.samples::<i16>();
    println!("Signal energy: {}", samples.energy());

    if let Some(peak) = find_spectral_peak("test.wav") {
        println!("Max frequency: {} Hz", peak);
    }
}
