extern crate arrayfire as af;
extern crate time;

use time::PreciseTime;
use af::*;

#[allow(unused_must_use)]
#[allow(unused_variables)]
fn main() {
    set_device(0);
    info();
    let samples = 20_000_000;
    let dims = Dim4::new(&[samples, 1, 1, 1]);

    let x = &randu(dims, Aftype::F32).unwrap();
    let y = &randu(dims, Aftype::F32).unwrap();

    let start = PreciseTime::now();

    for bench_iter in 0..100 {
        let pi_val = add(&mul(x, x, false).unwrap(), &mul(y, y, false).unwrap(), false)
            .and_then( |z| sqrt(&z) )
            .and_then( |z| le(&z, &constant(1, dims).unwrap(), false) )
            .and_then( |z| sum_all(&z) )
            .map( |z| z.0 * 4.0/(samples as  f64) )
            .unwrap();
    }

    let end = PreciseTime::now();

    println!("Estimated Pi Value in {} seconds", start.to(end) / 100);
}
