# Day 2 - hound

My engineering diploma involved some digital signal processing (DSP),
in particular sound generation and recognition. Throughout my studies
I went through a ton of audio files, usually using C++ to process them.
I've written a custom `.wav` file loader, of course missed a few edge
cases and it crashed upon receiving new training data from my supervisor...

A few months later I discovered that Python
[suports WAV](https://docs.python.org/3.5/library/wave.html) in the standard
library. Since then I try not to reinvent the wheel when it comes to processing
`.wav` files. Luckily there's a great library for doing that in Rust -
[hound](https://crates.io/crates/hound).

Writing to .wav file
--------------------

The canonical *Hello World* program in the land of digital signal processing is
one that generates a sine wave.

```rust
extern crate hound;

use std::f32::consts::PI;
use hound::{SampleFormat, WavSpec, WavWriter};

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

generate_sine("test.wav", 1000f32, 5);
```

Our `generate_sine` function takes three arguments - path to output file,
desired frequency of the sine wave and a total duration (in seconds). We start
by creating a `WavSpec` struct, which is essentially a higher-level view of the
WAV header. Then we open the file to write samples according to the spec. We
need to cast calculated sample to type consistent with `sample_format` field
of the spec, otherwise `write_sample()` panics at runtime.

If we run that code and open the generated file with an audio player, we should
hear a 5 second beep at 1 kHz.

Calculating signal energy
-------------------------

Signal energy in DSP is understood as a sum of squared norms of all the
(discrete) signal's samples. To quote the SP4COMM book:

> This definition is consistent with the idea that, if the values of the
> sequence represent a time-varying voltage, the above sum would express
> the total energy (in joules) dissipated over a 1Ω-resistor.

Since signals can come from different sources, let's abstract the concept as a
Rust trait:

```rust
trait Signal {
    fn energy(self) -> f64;
}
```

We can imagine implementing this trait for any signal source: microphone,
sonar, synthesizer or a WAV file. Let's implement it for `WavSamples`,
which is the actual iterator returning sample values.

```rust
impl<'a, R> Signal for WavSamples<'a, R, i16>
    where R: std::io::Read
{
    fn energy(self) -> f64 {
        self.map(|x| {
                let sample = x.unwrap() as f64;
                sample * sample
            })
            .sum()
    }
}
```

The `WavSamples` type is parametrized by a lifetime, a type implementing
[`Read`](https://doc.rust-lang.org/stable/std/io/trait.Read.html) and a sample
type. For this example I decided fixing sample type to `i16` to avoid non-scalar
casts with generic types. The actual calculation is very simple - for every
sample, calculate it's square and sum it all up.

Finding spectral peaks
----------------------

A common task in DSP is to find the most dominant frequency in the signal. To
do that, we need to calculate the frequency spectrum of our signal and then
find peaks in the spectrum. Moving from time domain to frequency domain
involves a calculation known as
[Fourier transform](https://en.wikipedia.org/wiki/Fourier_transform). FFT is
a family of fast algorithms to compute Fourier transforms and there is a ton of
packages for FFT in many programming languages. I chose
[rustfft](https://crates.io/crates/rustfft) since it's written purely in Rust.
(Other crates may be faster, but they're usually bindings to C or C++
libraries.)

```rust
extern crate num;
extern crate rustfft;

use num::complex::Complex;
use rustfft::FFT;

fn find_spectral_peak(filename: &str) -> Option<f32> {
    let mut reader = WavReader::open(filename).expect("Failed to open WAV file");
    let num_samples = reader.len() as usize;
    let mut fft = FFT::new(num_samples, false);
    let signal = reader.samples::<i16>()
        .map(|x| Complex::new(x.unwrap() as f32, 0f32))
        .collect::<Vec<_>>();
    let mut spectrum = signal.clone();
    fft.process(&signal[..], &mut spectrum[..]);
    let max_peak = spectrum.iter()
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

```

This function takes a filename as an argument and (possibly) returns the
strongest frequency in the signal. As the function is slightly complex (pun
totally intended), let's go through some of the important parts.

```rust
let num_samples = reader.len() as usize;
let mut fft = FFT::new(num_samples, false);
let signal = reader.samples::<i16>()
    .map(|x| Complex::new(x.unwrap() as f32, 0f32))
    .collect::<Vec<_>>();
let mut spectrum = signal.clone();
fft.process(&signal[..], &mut spectrum[..]);
```

We need to know up front the length of the transform. Next we collect our
data from WAV file into a complex vector (the FFT in `rustfft` can process
only complex signals). The output of the FFT is also a complex vector of
the same length, so it's easiest to clone it and declare as mutable.

```rust
let max_peak = spectrum.iter()
    .take(num_samples / 2)
    .enumerate()
    .max_by_key(|&(_, freq)| freq.norm() as u32);
```

The FFT spectrum is symmetrical, so we're interested only in the first half of
it. Using `enumerate()`, we pair up each value in the spectrum with its index.
Later in the call to `max_by_key()` we ignore the index, but use the norm
of current value for comparison.

```rust
if let Some((i, _)) = max_peak {
    let bin = 44100f32 / num_samples as f32;
    Some(i as f32 * bin)
} else {
    None
}
```

Finally we pattern match using `if let` to extract just the index of the
maximum value. `bin` is the width of a single frequency bin in spectrum,
so if we multiply the index with bin width, we get the frequency at that index.

```rust
if let Some(peak) = find_spectral_peak("test.wav") {
    println!("Max frequency: {} Hz", peak);
}
```

If we run this function on the WAV file generated earlier (remember the beep?),
we can confirm that was really a 1 kHz sine wave.

```text
$ cargo run
Max frequency: 1000 Hz
```

Further reading
---------------

 - [A Coursera course on DSP](https://www.coursera.org/learn/dsp)
 - [SP4COMM - Signal Processing for Communications](http://www.sp4comm.org/)
