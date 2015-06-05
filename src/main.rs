extern crate libc;
extern crate arrayfire;

// include!(concat!(env!("OUT_DIR"), "/arrayfire.rs"));


fn main() {
	// Let's define a NULL var: 
	// Rust does not like initializing dangling pointers
	let mut NULL = 0;

	unsafe {
    arrayfire::af_set_device(0);
    arrayfire::af_info();

    println!("Create a 5-by-3 matrix of random floats on the GPU");	
          let A: *mut arrayfire::af_array = &mut NULL as *mut _ as *mut arrayfire::af_array;
          let dims: [arrayfire::dim_t; 2] = [5,3];
          arrayfire::af_randu(A, 2, &dims[0], arrayfire::f32);
          arrayfire::af_print_array(*A);

          println!("Element-wise arithmetic");
          let B: *mut arrayfire::af_array = &mut NULL as *mut _ as *mut arrayfire::af_array;
          arrayfire::af_sin(B, *A);
          arrayfire::af_print_array(*B);

          // TODO: work through implementations of below
          // printf("Negate the first three elements of second column\n");
          // B(seq(0, 2), 1) = B(seq(0, 2), 1) * -1;
          // af_print(B);

          // printf("Fourier transform the result\n");
          // array C = fft(B);
          // af_print(C);

          // printf("Grab last row\n");
          // array c = C.row(end);
          // af_print(c);

          // printf("Create 2-by-3 matrix from host data\n");
          // float d[] = { 1, 2, 3, 4, 5, 6 };
          // array D(2, 3, d, af::afHost);
          // af_print(D);

          // printf("Copy last column onto first\n");
          // D.col(0) = D.col(end);
          // af_print(D);

          // // Sort A
          // printf("Sort A and print sorted array and corresponding indices\n");
          // array vals, inds;
          // sort(vals, inds, A);
          // af_print(vals);
          // af_print(inds);
	}
}
