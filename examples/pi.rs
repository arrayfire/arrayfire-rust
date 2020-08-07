use arrayfire::*;
use std::time::Instant;

#[allow(unused_must_use)]
#[allow(unused_variables)]
fn main() {
    set_device(0);
    info();
    let samples = 20_000_000;
    let dims = Dim4::new(&[samples, 1, 1, 1]);

    let x = &randu::<f32>(dims);
    let y = &randu::<f32>(dims);

    let start = Instant::now();

    mem_info!("Before benchmark");

    for bench_iter in 0..100 {
        let xsqrd = &mul(x, x, false);
        let ysqrd = &mul(y, y, false);
        let xplusy = &add(xsqrd, ysqrd, false);
        let root = &sqrt(xplusy);
        let cnst = &constant(1, dims);
        let (real, imag) = sum_all(&le(root, cnst, false));
        let pi_val = (real as f64) * 4.0 / (samples as f64);
    }

    println!("Estimated Pi Value in {:?}", start.elapsed());

    mem_info!("After benchmark");
}
