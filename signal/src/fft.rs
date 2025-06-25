use anyhow::{Ok, Result};
use rustfft::{
    FftPlanner,
    num_complex::{Complex, ComplexFloat},
};
pub fn mono_stft(samples: Vec<f32>) -> Vec<Vec<Complex<f32>>> {
    //into frames
    let win_size = 1024;
    let hop = 512;
    let mut frames: Vec<Vec<f32>> = Vec::new();
    for i in (0..samples.len()).step_by(hop) {
        let win_end = i + win_size;
        if win_end <= samples.len() {
            frames.push(Vec::from(&samples[i..win_end]));
        } else {
            break;
        }
    }

    //frame * Hann window function
    let window: Vec<f32> = window(win_size);

    let frames = frames
        .iter()
        .map(|frame| {
            frame
                .iter()
                .zip(window.iter())
                .map(|(sample, w)| sample * w)
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>();

    //fft
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(win_size);

    let mut spectrogram: Vec<Vec<Complex<f32>>> = Vec::with_capacity(frames.len());

    for frame in frames {
        let mut buffer: Vec<Complex<f32>> =
            frame.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();

        //zero padding
        buffer.resize(win_size, Complex { re: 0.0, im: 0.0 });
        fft.process(&mut buffer);
        spectrogram.push(buffer);
    }
    spectrogram
}
pub fn stereo_stft(samples: Vec<(f32, f32)>) -> Vec<Vec<(Complex<f32>, Complex<f32>)>> {
    //into frames
    let win_size = 1024;
    let hop = 512;
    let mut frames: Vec<Vec<(f32, f32)>> = Vec::new();
    for i in (0..samples.len()).step_by(hop) {
        let win_end = i + win_size;
        if win_end <= samples.len() {
            frames.push(Vec::from(&samples[i..win_end]));
        } else {
            break;
        }
    }

    //frame * Hann window function
    let window: Vec<f32> = window(win_size);

    let frames = frames
        .iter()
        .map(|frame| {
            frame
                .iter()
                .zip(window.iter())
                .map(|((l, r), w)| ((l * w), (r * w)))
                .collect::<Vec<(f32, f32)>>()
        })
        .collect::<Vec<Vec<(f32, f32)>>>();

    //fft
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(win_size);

    let mut spectrogram: Vec<Vec<(Complex<f32>, Complex<f32>)>> = Vec::with_capacity(frames.len());

    for frame in frames {
        let mut frame_inp: Vec<(Complex<f32>, Complex<f32>)> = frame
            .iter()
            .map(|&x| (Complex { re: x.0, im: 0.0 }, Complex { re: x.1, im: 0.0 }))
            .collect();

        //zero padding
        // frame_inp.resize(
        //     win_size,
        //     (Complex { re: 0.0, im: 0.0 }, Complex { re: 0.0, im: 0.0 }),
        // );
        // split buffer into 2 vector
        let (mut l_buf, mut r_buf): (Vec<_>, Vec<_>) =
            frame_inp.into_iter().map(|(l, r)| (l, r)).unzip();

        fft.process(&mut l_buf);
        fft.process(&mut r_buf);

        let new_buf = l_buf.into_iter().zip(r_buf).collect();

        spectrogram.push(new_buf);
    }
    spectrogram
}
pub fn window(size: usize) -> Vec<f32> {
    //w[n] = 1/2 * cos(1- 2npi/(N-1))
    (0..size)
        .map(|n| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * n as f32 / (size - 1) as f32)).cos())
        .collect()
}
