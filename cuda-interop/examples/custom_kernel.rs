use arrayfire as af;
use rustacuda::prelude::*;
use rustacuda::*;

use std::ffi::CString;

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
    let ptx = CString::new(include_str!("./resources/add.ptx")).unwrap();
    let module = match Module::load_from_string(&ptx) {
        Ok(m) => m,
        Err(e) => panic!("Failed to load module from string: {:?}", e),
    };
    let stream = match Stream::new(StreamFlags::NON_BLOCKING, None) {
        Ok(s) => s,
        Err(e) => panic!("Failed to create stream: {:?}", e),
    };

    af::set_device(0);
    af::info();

    let num: i32 = 10;
    let x = af::constant(1f32, af::dim4!(10));
    let y = af::constant(2f32, af::dim4!(10));
    let out = af::constant(0f32, af::dim4!(10));

    af::af_print!("x", x);
    af::af_print!("y", y);
    af::af_print!("out(init)", out);

    //TODO Figure out how to use Stream returned by ArrayFire with Rustacuda
    // let af_id = get_device();
    // let cuda_id = get_device_native_id(af_id);
    // let af_cuda_stream = get_stream(cuda_id);

    //TODO Figure out how to use Stream returned by ArrayFire with Rustacuda
    // let stream = Stream {inner: mem::transmute(af_cuda_stream)};

    // Run a custom CUDA kernel in the ArrayFire CUDA stream
    unsafe {
        // Obtain device pointers from ArrayFire using Array::device() method
        let d_x: *mut f32 = x.device_ptr() as *mut f32;
        let d_y: *mut f32 = y.device_ptr() as *mut f32;
        let d_o: *mut f32 = out.device_ptr() as *mut f32;

        match launch!(module.sum<<<1, 1, 0, stream>>>(
        memory::DevicePointer::wrap(d_x),
        memory::DevicePointer::wrap(d_y),
        memory::DevicePointer::wrap(d_o),
        num
        )) {
            Ok(()) => {}
            Err(e) => panic!("Kernel Launch failure: {:?}", e),
        }

        // wait for the kernel to finish as it is async call
        match stream.synchronize() {
            Ok(()) => {}
            Err(e) => panic!("Stream sync failure: {:?}", e),
        };

        // Return control of Array memory to ArrayFire using unlock
        x.unlock();
        y.unlock();
        out.unlock();
    }
    af::af_print!("sum after kernel launch", out);
}
