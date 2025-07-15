mod fft;
mod ifft;
mod mask;
mod normal;
use core::f32;

use anyhow::{Ok, Result};
use hound::{WavSpec, WavWriter};
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use rustfft::num_complex::Complex;

// struct MonoSpectrogram {
//     spectrogram: Vec<Vec<Complex<f32>>>,
//     mask: f32,
//     max_mag: f32,
// }
//
//do when bored
// impl MonoSpectrogram {
//     fn new() -> MonoSpectrogram {
//         MonoSpectrogram {
//             spectrogram: vec![],
//             mask: 1.0,
//             max_mag: f32::MIN,
//         }
//     }
// }

fn main() -> Result<()> {
    let path: &str = "/home/raws4uce/rickjames/signal/samples/stereo16.wav";
    let mut reader_ = hound::WavReader::open(path)?;
    let spect = reader_.spec();

    let root = BitMapBackend::new(
        "/home/raws4uce/rickjames/signal/photo/stereo16.png",
        (1024, 720),
    )
    .into_drawing_area();
    root.fill(&WHITE);

    match spect.channels {
        1 => {
            let n_f32: Result<Vec<f32>> = normal::monowav_to_f32(path);
            let mut spectrogram = fft::mono_stft(n_f32.expect("FAIL"));
            //(n_frame -> spectrogram.len() ,magnitude -> (2)^1/2)

            // mask audio (in terms of magnitude where 0.4 would mean top 40% of samples)
            // let mut spectrogram = mask::mono_mask(&spectrogram, 0.2);
            //output wav
            let mut op_vec: Vec<f32> = ifft::mono_inverse(&mut spectrogram);

            let spec = WavSpec {
                channels: 1,
                sample_rate: 44100,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };

            let mut writer =
                WavWriter::create("/home/raws4uce/rickjames/signal/output/mono24.wav", spec)?;

            // Normalize to [-1.0, 1.0]
            let max_val = op_vec.iter().fold(0.0f32, |acc, &x| acc.max(x.abs()));
            if max_val > 0.0 {
                op_vec.iter_mut().for_each(|x| *x /= max_val);
            }

            // Write as-is (f32 format expects float samples in [-1.0, 1.0])
            for &sample in op_vec.iter() {
                writer.write_sample(sample)?;
            }

            writer.finalize()?;
            let mut ctx = ChartBuilder::on(&root)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption("no mask", ("sans-serif", 40))
                .build_cartesian_2d(0..spectrogram.len(), 0.0..150.0)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            let coordinates = mono_coordinates(&spectrogram);

            ctx.draw_series(coordinates.iter().map(|point| Circle::new(*point, 5, &RED)))
                .unwrap();
        }
        2 => {
            let n_f32: Result<Vec<(f32, f32)>> = normal::stereowav_to_f32(path);
            let mut spectrogram = fft::stereo_stft(n_f32.expect("FAIL"));

            // mask audio
            // let spectrogram = mask::stero_mask(spectrogram, 0.8);

            let mut op_vec: Vec<(f32, f32)> = ifft::stereo_inverse(&mut spectrogram);

            let spec = WavSpec {
                channels: 2,
                sample_rate: 44100,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };

            let mut writer =
                WavWriter::create("/home/raws4uce/rickjames/signal/output/stereo16.wav", spec)?;

            // Normalize to [-1.0, 1.0]
            let l_max = op_vec.iter().fold(0.0f32, |acc, &x| acc.max(x.0.abs()));
            let r_max = op_vec.iter().fold(0.0f32, |acc, &x| acc.max(x.1.abs()));

            if l_max > 0.0 {
                op_vec.iter_mut().for_each(|x| x.0 /= l_max);
            }
            if r_max > 0.0 {
                op_vec.iter_mut().for_each(|x| x.1 /= l_max);
            }

            // Write as-is (f32 format expects float samples in [-1.0, 1.0])
            for &sample in op_vec.iter() {
                writer.write_sample(sample.0)?;
                writer.write_sample(sample.1)?;
            }

            writer.finalize()?;
            let mut ctx = ChartBuilder::on(&root)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption("magnitude by time", ("sans-serif", 40))
                .build_cartesian_2d(0..spectrogram.len(), 0.0..500.0)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();
            let coordinates = stereo_coordinates(&spectrogram);
            ctx.draw_series(
                coordinates
                    .0
                    .iter()
                    .map(|point| Circle::new(*point, 1, &RED)),
            )
            .unwrap();
            ctx.draw_series(
                coordinates
                    .1
                    .iter()
                    .map(|point| Circle::new(*point, 1, &BLACK)),
            )
            .unwrap();
        }
        _ => panic!("can only pattern mono/stero,nun ov vat 5:1surroundsound biznes"),
    }

    Ok(())
}
fn mono_coordinates(spectrogram: &Vec<Vec<Complex<f32>>>) -> Vec<(usize, f64)> {
    let mut coordinates: Vec<(usize, f64)> = vec![];

    //probably very redundant
    for (i, frame) in spectrogram.iter().enumerate() {
        for comp in frame {
            // println!("{}, real : {}, imaginary {}", i, comp.re, comp.im);
            let mag = ((comp.re * comp.re + comp.im * comp.im).sqrt()).max(1e-10);
            let log_mag = 20.0 * mag.log10();
            coordinates.push((i, log_mag as f64));
        }
    }
    coordinates
}
fn stereo_coordinates(
    spectrogram: &Vec<Vec<(Complex<f32>, Complex<f32>)>>,
) -> (Vec<(usize, f64)>, Vec<(usize, f64)>) {
    let mut l_coodinates: Vec<(usize, f64)> = vec![];
    let mut r_coodinates: Vec<(usize, f64)> = vec![];

    //probably very redundant
    for (i, frame) in spectrogram.iter().enumerate() {
        for comp in frame {
            // println!("{}, real : {}, imaginary {}", i, comp.re, comp.im);

            let l_mag = ((comp.0.re * comp.0.re + comp.0.im * comp.0.im).sqrt()).max(1e-10);
            let r_mag = ((comp.1.re * comp.1.re + comp.1.im * comp.1.im).sqrt()).max(1e-10);

            let l_log_mag = 20.0 * l_mag.log10();
            let r_log_mag = 20.0 * r_mag.log10();

            l_coodinates.push((i, l_log_mag as f64));
            r_coodinates.push((i, r_log_mag as f64));
        }
    }
    (l_coodinates, r_coodinates)
}
