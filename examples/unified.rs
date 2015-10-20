extern crate arrayfire as af;

use af::*;

#[allow(unused_must_use)]
fn test_backend(){
  info();

  let num_rows: u64 = 10;
  let num_cols: u64 = 10;
  let dims = Dim4::new(&[num_rows, num_cols, 1, 1]);

  println!("Create a 5-by-3 matrix of random floats on the compute device");
  let a = match randu(dims, Aftype::F32) {
    Ok(value) => value,
    Err(error) => panic!("{}", error),
  };
  print(&a);
}


#[allow(unused_must_use)]
fn main() {
  println!("There are {} available backends", get_backend_count().unwrap());
  let err = set_backend(AfBackend::AF_BACKEND_CPU);
  match err {
    Ok(_)  => test_backend(),
    Err(e) => println!("CPU backend not available: {}", e),
  };

  let err = set_backend(AfBackend::AF_BACKEND_CUDA);
  match err {
    Ok(_)  => test_backend(),
    Err(e) => println!("CUDA backend not available: {}", e),
  };

  let err = set_backend(AfBackend::AF_BACKEND_OPENCL);
  match err {
    Ok(_)  => test_backend(),
    Err(e) => println!("OpenCL backend not available: {}", e),
  };
}
