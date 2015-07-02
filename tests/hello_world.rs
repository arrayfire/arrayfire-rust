extern crate libc;
extern crate arrayfire;

use arrayfire::Dim4;
use arrayfire::Array;

#[test]
fn main() {
    arrayfire::set_device(0);
    arrayfire::info();

    let dims: Dim4 = Dim4::new(&[5, 3, 1, 1]);

    println!("Create a 5-by-3 matrix of random floats on the GPU");
	let a: Array = arrayfire::randu(&dims);
	arrayfire::print(&a);

    println!("Element-wise arithmetic");
	let b: Array = arrayfire::sin(&a);
	arrayfire::print(&b);

    // printf("Negate the first three elements of second column\n");
    // B(seq(0, 2), 1) = B(seq(0, 2), 1) * -1;
    // af_print(B);

    println!("Fourier transform the result");
    let c: Array = arrayfire::fft(&b, 1.0, 0);
    arrayfire::print(&c);

    // printf("Grab last row\n");
    // array c = C.row(end);
    // af_print(c);

    println!("Create 2-by-3 matrix from host data");
    let d_dims: Dim4 = Dim4::new(&[2, 3, 1, 1]);
    let d_input: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let d: Array = Array::new(&d_dims, &d_input);
    arrayfire::print(&d);

    // printf("Copy last column onto first\n");
    // D.col(0) = D.col(end);
    // af_print(D);

    // // Sort A
    println!("Sort A and print sorted array and corresponding indices");
    let (vals, inds) = arrayfire::sort(&a, 0, true);
    arrayfire::print(&vals);
    arrayfire::print(&inds);
}
