extern crate libc;
extern crate arrayfire as af;

use af::Dim4;
use af::Array;

#[test]
fn main() {
    af::set_device(0);
    af::info();

    let dims: Dim4 = Dim4::new(&[5, 3, 1, 1]);

    println!("Create a 5-by-3 matrix of random floats on the GPU");
    let a: Array = af::randu(dims, af::Aftype::F32);
    af::print(&a);

    println!("Element-wise arithmetic");
    let b: Array = af::sin(&a) + 1.5;
    af::print(&b);

    // printf("Negate the first three elements of second column\n");
    // B(seq(0, 2), 1) = B(seq(0, 2), 1) * -1;
    // af_print(B);

    println!("Fourier transform the result");
    let c: Array = af::fft(&b, 1.0, 0);
    af::print(&c);

    // printf("Grab last row\n");
    // array c = C.row(end);
    // af_print(c);

    println!("Create 2-by-3 matrix from host data");
    let d_dims: Dim4 = Dim4::new(&[2, 3, 1, 1]);
    let d_input: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let d: Array = Array::new(d_dims, &d_input, af::Aftype::S32);
    af::print(&d);

    // printf("Copy last column onto first\n");
    // D.col(0) = D.col(end);
    // af_print(D);

    // // Sort A
    println!("Sort A and print sorted array and corresponding indices");
    let (vals, inds) = af::sort_index(&a, 0, true);
    af::print(&vals);
    af::print(&inds);

    println!("u8 constant array");
    let u8_cnst = af::constant(1 as u8, Dim4::new(&[4,4,1,1]));
    af::print(&u8_cnst);
}
