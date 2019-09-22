use arrayfire::*;
use num::Complex;

fn main() {
    set_device(0);
    info();
    let samples = 10;
    let dims = Dim4::new(&[samples, 1, 1, 1]);

    let values = vec![
        Complex::new(0.0, 2.0),
        Complex::new(0.0, 2.0),
        Complex::new(0.0, 2.0),
        Complex::new(0.0, 2.0),
        Complex::new(0.0, 2.0),
        Complex::new(0.0, 2.0),
        Complex::new(0.0, 2.0),
        Complex::new(0.0, 2.0),
        Complex::new(0.0, 2.0),
        Complex::new(0.0, 2.0),
    ];

    let signal = Array::new(&values, dims);

    af_print!("signal", signal);

    // Used length of input signal as norm_factor
    let output = fft(&signal, 0.1, samples as i64);

    af_print!("Output", output);
}
