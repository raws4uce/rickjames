mod fft;
mod normal;
use anyhow::{Ok, Result};
fn main() -> Result<()> {
    let path: &str = "/home/raws4uce/rickjames/signal/samples/test2mono.wav";
    let n_f32: Result<Vec<f32>> = normal::monowav_to_f32(path);
    let spectrogram = fft::mono_stft(n_f32.expect("FAIL"));

    Ok(())
}
