use arrayfire::*;

fn main() {
    set_device(0);
    info();
    conways_game_of_life();
}

fn normalise(a: &Array<f32>) -> Array<f32> {
    a / (max_all(&abs(a)).0 as f32)
}

fn conways_game_of_life() {
    let h_kernel = [1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let kernel = Array::new(&h_kernel, Dim4::new(&[3, 3, 1, 1]));
    let s = constant::<f32>(0.5, Dim4::new(&[1, 1, 1, 1]));
    let mut state = gt(&randu::<f32>(Dim4::new(&[256, 256, 3, 1])), &s, false);
    let c0 = constant::<f32>(2.0, Dim4::new(&[1, 1, 1, 1]));
    let c1 = constant::<f32>(3.0, Dim4::new(&[1, 1, 1, 1]));

    let win = Window::new(512, 512, "Game of Life".to_string());
    while !win.is_closed() {
        let n_hood = convolve2(&state, &kernel, ConvMode::DEFAULT, ConvDomain::SPATIAL);
        let c0 = &eq(&n_hood, &c0, false);
        let c1 = &eq(&n_hood, &c1, false);
        state = state * c0 + c1;
        win.draw_image(&normalise(&state.cast::<f32>()), None);
    }
}
