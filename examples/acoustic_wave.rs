use arrayfire::*;
use std::f64::consts::*;

fn main() {
    set_device(0);
    info();
    acoustic_wave_simulation();
}

fn normalise(a: &Array<f32>) -> Array<f32> {
    (a / (max_all(&abs(a)).0 as f32 * 2.0f32)) + 0.5f32
}

fn acoustic_wave_simulation() {
    // Speed of sound
    let c: f32 = 0.1;
    // Distance step
    let dx: f32 = 0.5;
    // Time step
    let dt: f32 = 1.0;

    // Grid size.
    let nx: u64 = 1500;
    let ny: u64 = 1500;

    // Grid dimensions.
    let dims = Dim4::new(&[nx, ny, 1, 1]);

    // Pressure field
    let mut p = constant::<f32>(0.0, dims);
    // d(pressure)/dt field
    let mut p_dot = p.clone();

    // Laplacian (Del^2) convolution kernel.
    let laplacian_values: [f32; 9] = [0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0];
    let laplacian_kernel = Array::new(&laplacian_values, Dim4::new(&[3, 3, 1, 1])) / (dx * dx);

    // Create a window to show the waves.
    let mut win = Window::new(1000, 1000, "Waves".to_string());

    // Hann windowed pulse.
    let pulse_time: f32 = 100.0;
    let centre_freq: f32 = 0.05;
    let twopi = PI as f32 * 2.0;

    // Number of samples in pulse.
    let pulse_n = (pulse_time / dt).floor() as u64;

    let i = range::<f32>(Dim4::new(&[pulse_n, 1, 1, 1]), 0);
    let t = i.clone() * dt;
    let hmg_wnd = cos(&(i * (twopi / pulse_n as f32))) * -0.46f32 + 0.54f32;
    let wave = sin(&(&t * centre_freq * twopi));
    let pulse = wave * hmg_wnd;

    // Iteration count.
    let mut it = 0;

    while !win.is_closed() {
        // Convole with laplacian to get spacial second derivative.
        let lap_p = convolve2(
            &p,
            &laplacian_kernel,
            ConvMode::DEFAULT,
            ConvDomain::SPATIAL,
        );
        // Calculate the updated pressure and d(pressure)/dt fields.
        p_dot += lap_p * (c * dt);
        p += &p_dot * dt;

        if it < pulse_n {
            // Location of the source.
            let seqs = &[Seq::new(700.0, 800.0, 1.0), Seq::new(800.0, 800.0, 1.0)];
            // Set the pressure there.
            assign_seq(
                &mut p,
                seqs,
                &index(&pulse, &[Seq::new(it as f64, it as f64, 1.0)]),
            );
        }

        // Draw the image.
        win.set_colormap(ColorMap::BLUE);
        win.draw_image(&normalise(&p), None);

        it += 1;
    }
}
