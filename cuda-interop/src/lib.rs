//! af-cuda-interop package is to used only when the application intends to mix
//! arrayfire code with raw CUDA code.

use arrayfire::{handle_error_general, AfError};
use cuda_runtime_sys::cudaStream_t;
use libc::c_int;

extern "C" {
    fn afcu_get_native_id(native_id: *mut c_int, id: c_int) -> c_int;
    fn afcu_set_native_id(native_id: c_int) -> c_int;
    fn afcu_get_stream(out: *mut cudaStream_t, id: c_int) -> c_int;
}

/// Get active device's id in CUDA context
///
/// # Parameters
///
/// - `id` is the integer identifier of concerned CUDA device as per ArrayFire context
///
/// # Return Values
///
/// Integer identifier of device in CUDA context
pub fn get_device_native_id(id: i32) -> i32 {
    unsafe {
        let mut temp: i32 = 0;
        let err_val = afcu_get_native_id(&mut temp as *mut c_int, id);
        handle_error_general(AfError::from(err_val));
        temp
    }
}

/// Set active device using CUDA context's id
///
/// # Parameters
///
/// - `id` is the identifier of GPU in CUDA context
pub fn set_device_native_id(native_id: i32) {
    unsafe {
        let err_val = afcu_set_native_id(native_id);
        handle_error_general(AfError::from(err_val));
    }
}

/// Get CUDA stream of active CUDA device
///
/// # Parameters
///
/// - `id` is the identifier of device in ArrayFire context
///
/// # Return Values
///
/// [cudaStream_t](https://docs.rs/cuda-runtime-sys/0.3.0-alpha.1/cuda_runtime_sys/type.cudaStream_t.html) handle.
pub fn get_stream(native_id: i32) -> cudaStream_t {
    unsafe {
        let mut ret_val: cudaStream_t = std::ptr::null_mut();
        let err_val = afcu_get_stream(&mut ret_val as *mut cudaStream_t, native_id);
        handle_error_general(AfError::from(err_val));
        ret_val
    }
}
