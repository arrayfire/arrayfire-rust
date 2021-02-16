//! A trivial example. Copied from ocl-core crate repository.
use af_opencl_interop as afcl;
use arrayfire as af;

use ocl_core::{retain_mem_object, ContextProperties, Event};

fn main() {
    // Set the arrayfire backend to use OpenCL first,
    // because CUDA is the automatically preferred if available
    af::set_backend(af::Backend::OPENCL);

    // Choose platform & device(s) to use. Create a context, queue,
    let platform_id = ocl_core::default_platform().unwrap();
    let device_ids = ocl_core::get_device_ids(&platform_id, None, None).unwrap();
    let device_id = device_ids[0];
    let context_properties = ContextProperties::new().platform(platform_id);
    let context =
        ocl_core::create_context(Some(&context_properties), &[device_id], None, None).unwrap();
    let queue = ocl_core::create_command_queue(&context, &device_id, None).unwrap();
    let dims = [8, 1, 1];

    // Create a `Buffer`:
    let mut vec = vec![1.0f32; dims[0]];
    let buffer = unsafe {
        ocl_core::create_buffer(
            &context,
            ocl_core::MEM_READ_WRITE | ocl_core::MEM_COPY_HOST_PTR,
            dims[0],
            Some(&vec),
        )
        .unwrap()
    };
    ocl_core::finish(&queue).unwrap(); //sync up before switching to arrayfire

    // Add custom device, context and associated queue to ArrayFire
    afcl::add_device_context(device_id.as_raw(), context.as_ptr(), queue.as_ptr());
    afcl::set_device_context(device_id.as_raw(), context.as_ptr());
    af::info();

    unsafe {
        retain_mem_object(&buffer).unwrap();
    }
    let mut af_buffer = af::Array::new_from_device_ptr(
        buffer.as_ptr() as *mut f32,
        af::Dim4::new(&[dims[0] as u64, 1, 1, 1]),
    );

    af::af_print!("GPU Buffer before modification:", af_buffer);

    af_buffer = af_buffer + 10f32;

    af::sync(af::get_device());
    unsafe {
        let ptr = af_buffer.device_ptr();
        let obuf = ocl_core::Mem::from_raw_copied_ptr(ptr);

        // Read results from the device into a vector:
        ocl_core::enqueue_read_buffer(
            &queue,
            &obuf,
            true,
            0,
            &mut vec,
            None::<Event>,
            None::<&mut Event>,
        )
        .unwrap();
    }
    println!("GPU buffer on host after ArrayFire operation: {:?}", vec);

    // Remove device from ArrayFire management towards Application Exit
    af::set_device(0); // Cannot pop when in Use, hence switch to another device
    afcl::delete_device_context(device_id.as_raw(), context.as_ptr());
}
