use rustfft::{
    FftPlanner,
    num_complex::{Complex, ComplexFloat},
    num_traits::Zero,
};

//ai helped with using the rustfft crate and normalisation
pub fn mono_inverse(gram: &mut Vec<Vec<Complex<f32>>>) -> Vec<f32> {
    let win_size = 1024;
    let overlap = 512;

    let op_samples = (gram.len() - 1) * overlap + win_size;
    //preinit
    let mut op_audio = vec![0.0f32; op_samples];
    let window: Vec<f32> = window(win_size);
    let mut window_sum = vec![0.0f32; op_samples]; //norm purposes

    let mut planner = FftPlanner::new();
    let ifft = planner.plan_fft_inverse(win_size);

    // Create a scratch buffer for FFT computations
    let mut scratch = vec![Complex::zero(); ifft.get_inplace_scratch_len()];

    for (i, fr) in gram.iter_mut().enumerate() {
        let start = i * overlap;
        // Perform the IFFT on this frame
        // Note: The input might need Hermitian symmetry if your gram contains only positive frequencies
        ifft.process_with_scratch(fr.as_mut_slice(), &mut scratch);
        // Scale the IFFT output (IFFT typically needs to be scaled by 1/N)
        let scale = 1.0 / (win_size as f32);

        for ((j, c), w) in fr.iter().enumerate().zip(&window) {
            op_audio[start + j] += c.re * w * scale;
            window_sum[start + j] += w;
        }
    }
    //normalise
    for i in 0..op_audio.len() {
        if window_sum[i] > 1e-6 {
            op_audio[i] /= window_sum[i];
        }
    }
    op_audio
}

pub fn window(size: usize) -> Vec<f32> {
    //w[n] = 1/2 * cos(1- 2npi/(N-1))
    (0..size)
        .map(|n| 0.5 * (1.0 - (2.0 * std::f32::consts::PI * n as f32 / (size - 1) as f32)).cos())
        .collect()
}

// let mut first: Vec<f32> = gram
//     .iter()
//     .map(|frame| {
//         //frame * Hann window function
//         frame
//             .iter()
//             .zip(window.iter())
//             .map(|(c, w)| c.re * w)
//             .collect::<Vec<f32>>()
//     })
//     .into_iter()
//     .flatten()
//     .collect();
// // shift this to the right 512
// let mut second = first.clone();
// second.splice(..0, std::iter::repeat(0.0).take(overlap));
// //and the other one too
// first.extend(std::iter::repeat(0.0).take(overlap));
