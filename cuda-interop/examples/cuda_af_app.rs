use arrayfire::{af_print, dim4, info, set_device, Array};
use rustacuda::prelude::*;

fn main() {
    // MAKE SURE to do all rustacuda initilization before arrayfire API's
    // first call. It seems like some CUDA context state is getting messed up
    // if we mix CUDA context init(device, context, module, stream) with ArrayFire API
    match rustacuda::init(CudaFlags::empty()) {
        Ok(()) => {}
        Err(e) => panic!("rustacuda init failure: {:?}", e),
    }
    let device = match Device::get_device(0) {
        Ok(d) => d,
        Err(e) => panic!("Failed to get device: {:?}", e),
    };
    let _context =
        match Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device) {
            Ok(c) => c,
            Err(e) => panic!("Failed to create context: {:?}", e),
        };
    let stream = match Stream::new(StreamFlags::NON_BLOCKING, None) {
        Ok(s) => s,
        Err(e) => panic!("Failed to create stream: {:?}", e),
    };

    let mut in_x = DeviceBuffer::from_slice(&[1.0f32; 10]).unwrap();
    let mut in_y = DeviceBuffer::from_slice(&[2.0f32; 10]).unwrap();

    // wait for any prior kernels to finish before passing
    // the device pointers to ArrayFire
    match stream.synchronize() {
        Ok(()) => {}
        Err(e) => panic!("Stream sync failure: {:?}", e),
    };

    set_device(0);
    info();

    let x = Array::new_from_device_ptr(in_x.as_device_ptr().as_raw_mut(), dim4!(10));
    let y = Array::new_from_device_ptr(in_y.as_device_ptr().as_raw_mut(), dim4!(10));

    // Lock so that ArrayFire doesn't free pointers from RustaCUDA
    // But we have to make sure these pointers stay in valid scope
    // as long as the associated ArrayFire Array objects are valid
    x.lock();
    y.lock();

    af_print!("x", x);
    af_print!("y", y);

    let o = x + y;
    af_print!("out", o);

    let _o_dptr = unsafe { o.device_ptr() }; // Calls an implicit lock

    // User has to call unlock if they want to relenquish control to ArrayFire

    // Once the non-arrayfire operations are done, call unlock.
    o.unlock(); // After this, there is no guarantee that value of o_dptr is valid
}
