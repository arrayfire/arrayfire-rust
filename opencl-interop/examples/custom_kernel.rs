//! A trivial example. Copied from ocl-core crate repository.
use af_opencl_interop as afcl;
use arrayfire as af;

use ocl_core::{ArgVal, Event};

use std::ffi::CString;

fn main() {
    // Set the arrayfire backend to use OpenCL first,
    // because CUDA is the automatically preferred if available
    af::set_backend(af::Backend::OPENCL);

    af::info();
    let dims = af::dim4!(8);
    let af_buffer = af::constant(0f32, dims.clone());
    af::af_print!("af_buffer", af_buffer);

    let src = r#"
        __kernel void add(__global float* buffer, float scalar) {
            buffer[get_global_id(0)] += scalar;
        }
    "#;

    let af_did = afcl::get_device_id();
    let af_ctx = afcl::get_context(false);
    let af_que = afcl::get_queue(false);

    let _devid = unsafe { ocl_core::DeviceId::from_raw(af_did) };
    let contx = unsafe { ocl_core::Context::from_raw_copied_ptr(af_ctx) };
    let queue = unsafe { ocl_core::CommandQueue::from_raw_copied_ptr(af_que) };

    // Define which platform and device(s) to use. Create a context,
    // queue, and program then define some dims..
    let src_cstring = CString::new(src).unwrap();
    let program = ocl_core::create_program_with_source(&contx, &[src_cstring]).unwrap();
    ocl_core::build_program(
        &program,
        None::<&[()]>,
        &CString::new("").unwrap(),
        None,
        None,
    )
    .unwrap();

    // Fetch cl_mem from ArrayFire Array
    let ptr = unsafe { af_buffer.device_ptr() };
    let buffer = unsafe { ocl_core::Mem::from_raw_copied_ptr(ptr) };

    // Create a kernel with arguments matching those in the source above:
    let kernel = ocl_core::create_kernel(&program, "add").unwrap();
    ocl_core::set_kernel_arg(&kernel, 0, ArgVal::mem(&buffer)).unwrap();
    ocl_core::set_kernel_arg(&kernel, 1, ArgVal::scalar(&10.0f32)).unwrap();

    let ocl_dims: [usize; 3] = [dims[0] as usize, dims[1] as usize, dims[2] as usize];
    unsafe {
        ocl_core::enqueue_kernel(
            &queue,
            &kernel,
            1,
            None,
            &ocl_dims,
            None,
            None::<Event>,
            None::<&mut Event>,
        )
        .unwrap();
    }
    ocl_core::finish(&queue).unwrap();
    af_buffer.unlock(); //Give back control of cl_mem to ArrayFire memory manager

    af::af_print!("af_buffer after running Custom Kernel on it", af_buffer);
}
