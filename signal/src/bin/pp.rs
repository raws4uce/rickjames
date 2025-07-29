//pp - preprocessing, 124 x 100 matrices npy
use anyhow::{Ok, Result};
use ndarray::Array2;
use ndarray_npy::write_npy;
use rand::Rng;
use rustfft::num_complex::Complex;
use signal::{fft, normal};
fn main() -> Result<()> {
    for a in 0..=4 {
        let genre = match a {
            0 => "rk",
            1 => "grm",
            2 => "jazz",
            3 => "ele",
            4 => "dnb",
            _ => "",
        };
        for b in 1..=8 {
            println!("{}{}", genre, b);
            let path = format!("/home/raws4uce/Downloads/samples_wav/{}{}.wav", genre, b);

            let reader_ = hound::WavReader::open(&path)?;
            let spect = reader_.spec();
            let sample_len = spect.sample_rate * 10;

            //need random sample 10s
            let sample: Vec<Vec<f32>> = match spect.channels {
                1 => {
                    let n_f32: Result<Vec<f32>> = normal::monowav_to_f32(path.as_str());
                    let spectrogram = fft::mono_stft(n_f32.expect("FAIL"));

                    let n_samples = spectrogram.len() * 128;
                    let range = n_samples - sample_len as usize;

                    //random 10s sample
                    let sample = m_random_sample(&spectrogram, sample_len, range as u32)
                        .expect("sample fn err");
                    let mag: Vec<Vec<f32>> = sample
                        .iter()
                        .map(|frame| frame.iter().map(|c| c.norm()).collect())
                        .collect();
                    mag
                }
                2 => {
                    let n_f32: Result<Vec<(f32, f32)>> = normal::stereowav_to_f32(path.as_str());
                    let spectrogram = fft::stereo_stft(n_f32.expect("FAIL"));

                    let n_samples = spectrogram.len() * 128;
                    let range = n_samples - sample_len as usize;

                    //random 10s sample
                    let sample = s_random_sample(&spectrogram, sample_len, range as u32)
                        .expect("sample fn err");
                    let mag: Vec<Vec<f32>> = sample
                        .iter()
                        .map(|frame| {
                            frame
                                .iter()
                                .map(|(c1, c2)| (c1.norm() + c2.norm()) / 2.0)
                                .collect()
                        })
                        .collect();

                    mag
                }
                _ => panic!("we dont do 5.1 surroundsoundblud"),
            };

            //128 x 100 matrix (reduced)
            let x = 100; //n frames
            let red_sample: Vec<Vec<f32>> = (0..x)
                .map(|i| {
                    //condencing frames by finding average (indexing)
                    let frames = sample.len();
                    let start = (i * frames) / x;
                    let end = if (((i + 1) * frames) / x) > sample.len() {
                        sample.len()
                    } else {
                        ((i + 1) * frames) / x
                    };

                    (start..end)
                        .map(|k| sample[k].clone())
                        .reduce(|acc, x| acc.iter().zip(x).map(|(a, b)| a + b).collect())
                        .expect("indexing err")
                        .iter()
                        .map(|&x| x / (end - start) as f32)
                        .collect()
                })
                .collect();

            //parse samplevec into .npy
            let arr: Array2<f32> =
                Array2::from_shape_vec((100, 128), red_sample.into_iter().flatten().collect())?;
            write_npy(
                format!(
                    "/home/raws4uce/rickjames/signal/p_samples/{}/{}{}.npy",
                    genre, genre, b
                ),
                &arr,
            )?;
        }
    }

    //store in different genres

    Ok(())
}
fn s_random_sample(
    spectrogram: &Vec<Vec<(Complex<f32>, Complex<f32>)>>,
    sample_len: u32,
    range: u32,
) -> Result<Vec<Vec<(Complex<f32>, Complex<f32>)>>> {
    let mut rng = rand::rng();
    let r_ind: u32 = rng.random_range(0..range);

    let win_size: u32 = 128;
    let n_frames: usize = (sample_len / win_size) as usize;

    let mut result: Vec<Vec<(Complex<f32>, Complex<f32>)>> =
        vec![vec![(Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)); win_size as usize]; n_frames];

    for i in r_ind as usize..n_frames {
        result[i].copy_from_slice(&spectrogram[i]);
    }
    Ok(result)
}
fn m_random_sample(
    spectrogram: &Vec<Vec<Complex<f32>>>,
    sample_len: u32,
    range: u32,
) -> Result<Vec<Vec<Complex<f32>>>> {
    let mut rng = rand::rng();
    let r_ind: u32 = rng.random_range(0..range);

    let win_size: u32 = 128;
    let n_frames: usize = (sample_len / win_size) as usize;

    let mut result: Vec<Vec<Complex<f32>>> =
        vec![vec![Complex::new(0.0, 0.0); win_size as usize]; n_frames];

    for i in r_ind as usize..n_frames {
        result[i].copy_from_slice(&spectrogram[i]);
    }
    Ok(result)
}
