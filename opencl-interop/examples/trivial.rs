//! A trivial example.
//!
//! Copied from ocl.

extern crate ocl_core as core;

use crate::core::{ArgVal, ContextProperties, Event};
use std::ffi::CString;

#[allow(dead_code, unused_variables, unused_mut)]
fn main() {
    let src = r#"
        __kernel void add(__global float* buffer, float scalar) {
            buffer[get_global_id(0)] += scalar;
        }
    "#;

    // (1) Define which platform and device(s) to use. Create a context,
    // queue, and program then define some dims..
    let platform_id = core::default_platform().unwrap();
    let device_ids = core::get_device_ids(&platform_id, None, None).unwrap();
    let device_id = device_ids[0];
    let context_properties = ContextProperties::new().platform(platform_id);
    let context =
        core::create_context(Some(&context_properties), &[device_id], None, None).unwrap();
    let src_cstring = CString::new(src).unwrap();
    let program = core::create_program_with_source(&context, &[src_cstring]).unwrap();
    core::build_program(
        &program,
        None::<&[()]>,
        &CString::new("").unwrap(),
        None,
        None,
    )
    .unwrap();
    let queue = core::create_command_queue(&context, &device_id, None).unwrap();
    let dims = [1 << 20, 1, 1];

    // (2) Create a `Buffer`:
    let mut vec = vec![0.0f32; dims[0]];
    let buffer = unsafe {
        core::create_buffer(
            &context,
            core::MEM_READ_WRITE | core::MEM_COPY_HOST_PTR,
            dims[0],
            Some(&vec),
        )
        .unwrap()
    };

    // (3) Create a kernel with arguments matching those in the source above:
    let kernel = core::create_kernel(&program, "add").unwrap();
    core::set_kernel_arg(&kernel, 0, ArgVal::mem(&buffer)).unwrap();
    core::set_kernel_arg(&kernel, 1, ArgVal::scalar(&10.0f32)).unwrap();

    unsafe {
        // (4) Run the kernel:
        core::enqueue_kernel(
            &queue,
            &kernel,
            1,
            None,
            &dims,
            None,
            None::<Event>,
            None::<&mut Event>,
        )
        .unwrap();

        // (5) Read results from the device into a vector:
        core::enqueue_read_buffer(
            &queue,
            &buffer,
            true,
            0,
            &mut vec,
            None::<Event>,
            None::<&mut Event>,
        )
        .unwrap();
    }

    // Print an element:
    println!("The value at index [{}] is now '{}'!", 200007, vec[200007]);
}
