use arrayfire::*;
use half::f16;

fn main() {
    set_device(0);
    info();

    let values: Vec<_> = (1u8..101).map(std::convert::From::from).collect();

    let half_values = values.iter().map(|&x| f16::from_f32(x)).collect::<Vec<_>>();

    let hvals = Array::new(&half_values, Dim4::new(&[10, 10, 1, 1]));

    print(&hvals);
}
