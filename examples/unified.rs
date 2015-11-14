extern crate arrayfire as af;

use af::*;

#[allow(unused_must_use)]
fn test_backend(){
  info();

  let num_rows: u64 = 10;
  let num_cols: u64 = 10;
  let dims = Dim4::new(&[num_rows, num_cols, 1, 1]);

  println!("Create a 10-by-10 matrix of random floats on the compute device");
  let a = match randu(dims, Aftype::F32) {
    Ok(value) => value,
    Err(error) => panic!("{}", error),
  };
  print(&a);
}


#[allow(unused_must_use)]
fn main() {
  println!("There are {:?} available backends", get_backend_count().unwrap());
  let available = get_available_backends().unwrap();

  if available.contains(&Backend::AF_BACKEND_CPU){
    println!("Evaluating CPU Backend...");
    let err = set_backend(Backend::AF_BACKEND_CPU);
    println!("There are {} CPU compute devices", device_count().unwrap());
      match err {
        Ok(_)  => test_backend(),
        Err(e) => println!("CPU backend error: {}", e),
    };
  }

  if available.contains(&Backend::AF_BACKEND_CUDA){
    println!("Evaluating CUDA Backend...");
    let err = set_backend(Backend::AF_BACKEND_CUDA);
    println!("There are {} CUDA compute devices", device_count().unwrap());
      match err {
        Ok(_)  => test_backend(),
        Err(e) => println!("CUDA backend error: {}", e),
    };
  }

  if available.contains(&Backend::AF_BACKEND_OPENCL){
    println!("Evaluating OpenCL Backend...");
    let err = set_backend(Backend::AF_BACKEND_OPENCL);
    println!("There are {} OpenCL compute devices", device_count().unwrap());
      match err {
        Ok(_)  => test_backend(),
        Err(e) => println!("OpenCL backend error: {}", e),
    };
  }
}
