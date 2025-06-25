mod fft;
mod normal;
use anyhow::{Ok, Result};
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
fn main() -> Result<()> {
    let path: &str = "/home/raws4uce/rickjames/signal/samples/test3.wav";
    let n_f32: Result<Vec<f32>> = normal::monowav_to_f32(path);
    let spectrogram = fft::mono_stft(n_f32.expect("FAIL"));

    // for (i, frame) in spectrogram.iter().enumerate() {
    //     for comp in frame {
    //         println!("{}, real : {}, imaginary {}", i, comp.re, comp.im);
    //         let mag = (comp.re * comp.re + comp.im * comp.im).sqrt();
    //     }
    // }

    let root = BitMapBackend::new(
        "/home/raws4uce/rickjames/signal/photo/3_thuds.png",
        (1024, 720),
    )
    .into_drawing_area();
    root.fill(&WHITE);

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("magnitude by time", ("sans-serif", 40))
        .build_cartesian_2d(0..spectrogram.len(), 0.0..1.414213562373)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
    //(n_frame -> spectrogram.len() ,magnitude -> (2)^1/2)
    let mut coodinates: Vec<(usize, f64)> = vec![];

    //probably very redundant
    for (i, frame) in spectrogram.iter().enumerate() {
        for comp in frame {
            // println!("{}, real : {}, imaginary {}", i, comp.re, comp.im);
            let mag = (comp.re * comp.re + comp.im * comp.im).sqrt();

            coodinates.push((i, mag as f64));
        }
    }

    ctx.draw_series(coodinates.iter().map(|point| Circle::new(*point, 5, &RED)))
        .unwrap();
    for (i, frame) in spectrogram.iter().enumerate() {
        for comp in frame {
            // println!("{}, real : {}, imaginary {}", i, comp.re, comp.im);
            let mag = (comp.re * comp.re + comp.im * comp.im).sqrt();
        }
    }

    Ok(())
}
