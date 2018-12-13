use arrayfire::*;

#[cfg(op_assign)]
fn helper(dims: Dim4) {
    let mut a = randu::<f32>(dims);
    let b = randu::<f32>(dims);
    print(&a);
    print(&b);
    a += b;
    print(&a);
}

#[cfg(not(op_assign))]
fn helper(dims: Dim4) {
    let b = randu::<f32>(dims);
    print(&b);
}

#[allow(unused_must_use)]
fn test_backend() {
    info();

    println!("Create a 10-by-10 matrix of random floats on the compute device");
    let num_rows: u64 = 10;
    let num_cols: u64 = 10;
    let dims = Dim4::new(&[num_rows, num_cols, 1, 1]);

    helper(dims)
}

#[allow(unused_must_use)]
fn main() {
    println!("There are {:?} available backends", get_backend_count());
    let available = get_available_backends();

    if available.contains(&Backend::CPU) {
        println!("Evaluating CPU Backend...");
        set_backend(Backend::CPU);
        println!("There are {} CPU compute devices", device_count());
        test_backend();
    }

    if available.contains(&Backend::CUDA) {
        println!("Evaluating CUDA Backend...");
        set_backend(Backend::CUDA);
        println!("There are {} CUDA compute devices", device_count());
        test_backend();
    }

    if available.contains(&Backend::OPENCL) {
        println!("Evaluating OpenCL Backend...");
        set_backend(Backend::OPENCL);
        println!("There are {} OpenCL compute devices", device_count());
        test_backend();
    }
}
