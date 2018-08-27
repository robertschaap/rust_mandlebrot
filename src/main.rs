extern crate num;

use num::Complex;
use std::str::FromStr;

#[allow(dead_code)]
fn main() {
    fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
        let mut z = Complex { re: 0.0, im: 0.0 };

        for i in 0..limit {
            z = z * z + c;

            if z.norm_sqr() > 4.0 {
                return Some(i);
            }

            None
        }
    }

    fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
        match s.find(separator) {
            None => None,
            Some(index) => {
                match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                    (Ok(l), Ok(r)) => Some((l, r))),
                    _ => None
                }
            }
        }
    }

    fn parse_complex(s: &str) -> Option<Complex<f64>> {
        match parse_pair(s, ',') {
            Some((re, im)) => Some(Complex { re, im }),
            None => None
        }
    }

    fn pixel_to_point(bounds: (usize, usize),
                      pixel: (usize, usize),
                      upper_left: Complex<f64>,
                      lower_right: Complex<f64>)
        -> Complex<f64>
    {
        let (width, height) = (lower_right.re - upper_left.re,
                               upper_left.im - lower_right.im);

        Complex {
            re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
            im: upper_left.im + pixel.1 as f64 * width / bounds.1 as f64
        }
    }
}
