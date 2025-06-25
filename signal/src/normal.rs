use anyhow::{self, Result};
use std::i16;
pub fn monowav_to_f32(path: &str) -> Result<Vec<f32>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    assert_eq!(spec.channels, 1, "supposed to be mono file");

    //normalise [-1,1]
    let samples: Vec<f32> = match spec.bits_per_sample {
        16 => reader
            .samples::<i16>()
            .map(|s| s.map(|v| v as f32 / i16::MAX as f32))
            .collect::<Result<_, _>>()?,
        24 => reader
            .samples::<i32>()
            .map(|s| s.map(|v| v as f32 / 8_388_608.0))
            .collect::<Result<_, _>>()?,
        32 if spec.sample_format == hound::SampleFormat::Float => {
            //already normalised
            reader.samples::<f32>().collect::<Result<_, _>>()?
        }
        32 => reader
            .samples::<i32>()
            .map(|s| s.map(|v| v as f32 / i32::MAX as f32))
            .collect::<Result<_, _>>()?,
        _ => panic!("mono bit depth out of range"),
    };

    Ok(samples)
}
pub fn stereowav_to_f32(path: &str) -> Result<Vec<(f32, f32)>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    assert_eq!(spec.channels, 2, "supposed to be stereo file");

    let samples: Vec<(f32, f32)> = match spec.bits_per_sample {
        16 => {
            let temp: Vec<f32> = reader
                .samples::<i16>()
                .map(|s| s.map(|v| v as f32 / i16::MAX as f32))
                .collect::<Result<_, _>>()?;
            temp.chunks_exact(2).map(|c| (c[0], c[1])).collect()
        }
        24 => {
            let temp: Vec<f32> = reader
                .samples::<i32>()
                .map(|s| s.map(|v| v as f32 / 8_388_608.0))
                .collect::<Result<_, _>>()?;

            temp.chunks_exact(2).map(|c| (c[0], c[1])).collect()
        }
        32 if spec.sample_format == hound::SampleFormat::Float => {
            //already normalised
            let mut temp = reader.samples::<f32>().collect::<Result<Vec<(_)>, _>>()?;
            temp.chunks_exact(2).map(|c| (c[0], c[1])).collect()
        }
        32 => {
            let temp: Vec<f32> = reader
                .samples::<i32>()
                .map(|s| s.map(|v| v as f32 / i32::MAX as f32))
                .collect::<Result<_, _>>()?;
            temp.chunks_exact(2).map(|c| (c[0], c[1])).collect()
        }
        _ => panic!("stero bit depth out of range"),
    };
    Ok(samples)
}
