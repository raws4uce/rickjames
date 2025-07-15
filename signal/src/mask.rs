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
                    let factor = if mag >= a {
                        1.0
                    } else {
                        mag / a // smooth falloff
                    };
                    *c * factor
                })
                .collect()
        })
        .collect();
    masked_gram
}
pub fn stero_mask(
    gram: Vec<Vec<(Complex<f32>, Complex<f32>)>>,
    degree: f32,
) -> Vec<Vec<(Complex<f32>, Complex<f32>)>> {
    let mut l_r_min = f32::MAX;
    let mut l_r_max = f32::MIN;
    let mut r_r_min = f32::MAX;
    let mut r_r_max = f32::MIN;

    for frame in gram.iter() {
        for c in frame {
            let l_mag = (c.0.re * c.0.re + c.0.im * c.0.im).sqrt();
            let r_mag = (c.1.re * c.1.re + c.1.im * c.1.im).sqrt();
            if l_mag > l_r_max {
                l_r_max = l_mag
            } else if l_mag < l_r_min {
                l_r_min = l_mag
            }
            if r_mag > r_r_max {
                r_r_max = r_mag
            } else if r_mag < r_r_min {
                r_r_min = r_mag
            }
        }
    }
    let l_range = l_r_max - l_r_min;
    let r_range = r_r_max - r_r_min;
    //smaller reigon, where upper reigon is r_max
    let l_a = l_range * (1.0 - degree) + l_r_min;
    let r_a = r_range * (1.0 - degree) + r_r_min;
    let masked_gram: Vec<Vec<(Complex<f32>, Complex<f32>)>> = gram
        .iter()
        .map(|f| {
            f.iter()
                .map(|(l_c, r_c)| {
                    let l_mag = (l_c.re * l_c.re + l_c.im * l_c.im).sqrt();
                    let r_mag = (r_c.re * r_c.re + r_c.im * r_c.im).sqrt();
                    let l = if l_a <= l_mag {
                        *l_c // Directly clone the complex number k
                    } else {
                        Complex { re: 0.0, im: 0.0 }
                    };
                    let r = if r_a <= r_mag {
                        *r_c // Directly clone the complex number k
                    } else {
                        Complex { re: 0.0, im: 0.0 }
                    };
                    (l, r)
                })
                .collect()
        })
        .collect();
    masked_gram
}
