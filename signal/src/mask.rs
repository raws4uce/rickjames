use rustfft::num_complex::Complex;
//top X% bins (found by looking for magnitude)
pub fn mono_mask(gram: &Vec<Vec<Complex<f32>>>, degree: f32) -> Vec<Vec<Complex<f32>>> {
    let mut r_min = f32::MAX;
    let mut r_max = f32::MIN;

    for frame in gram.iter() {
        for c in frame {
            let mag = (c.re * c.re + c.im * c.im).sqrt();
            if mag > r_max {
                r_max = mag
            } else if mag < r_min {
                r_min = mag
            }
        }
    }
    let range = r_max - r_min;
    //smaller reigon, where upper reigon is r_max
    let a = range * (1.0 - degree) + r_min;
    let masked_gram: Vec<Vec<Complex<f32>>> = gram
        .iter()
        .map(|f| {
            f.iter()
                .map(|c| {
                    let mag = (c.re * c.re + c.im * c.im).sqrt();
                    if a <= mag {
                        *c // Directly clone the complex number
                    } else {
                        Complex { re: 0.0, im: 0.0 }
                    }
                })
                .collect()
        })
        .collect();
    masked_gram
}
