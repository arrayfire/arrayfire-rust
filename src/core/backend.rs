use super::defines::{AfError, Backend};
use super::error::HANDLE_ERROR;

use libc::{c_int, c_uint};

extern "C" {
    fn af_set_backend(bknd: u8) -> c_int;
    fn af_get_backend_count(num_backends: *mut c_uint) -> c_int;
    fn af_get_available_backends(backends: *mut c_int) -> c_int;
    fn af_get_active_backend(backend: *mut c_int) -> c_int;
}

/// Toggle backends between cuda, opencl or cpu
///
/// # Parameters
///
/// - `backend` to which to switch to
pub fn set_backend(backend: Backend) {
    unsafe {
        let err_val = af_set_backend(backend as u8);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Get the available backend count
pub fn get_backend_count() -> u32 {
    unsafe {
        let mut temp: u32 = 0;
        let err_val = af_get_backend_count(&mut temp as *mut c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        temp
    }
}

/// Get the available backends
pub fn get_available_backends() -> Vec<Backend> {
    unsafe {
        let mut temp: i32 = 0;
        let err_val = af_get_available_backends(&mut temp as *mut c_int);
        HANDLE_ERROR(AfError::from(err_val));

        let mut b = Vec::new();
        if temp & 0b0100 == 0b0100 {
            b.push(Backend::OPENCL);
        }
        if temp & 0b0010 == 0b0010 {
            b.push(Backend::CUDA);
        }
        if temp & 0b0001 == 0b0001 {
            b.push(Backend::CPU);
        }

        b
    }
}

/// Get current active backend
pub fn get_active_backend() -> Backend {
    unsafe {
        let mut temp: i32 = 0;
        let err_val = af_get_active_backend(&mut temp as *mut c_int);
        HANDLE_ERROR(AfError::from(err_val));
        match (err_val, temp) {
            (0, 0) => Backend::DEFAULT,
            (0, 1) => Backend::CPU,
            (0, 2) => Backend::CUDA,
            (0, 4) => Backend::OPENCL,
            _ => panic!("Invalid backend retrieved, undefined behavior."),
        }
    }
}
