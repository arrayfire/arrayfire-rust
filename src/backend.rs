extern crate libc;

use defines::{AfError, Backend};
use self::libc::{c_int, c_uint, uint8_t};

extern {
    fn af_set_backend(bknd: uint8_t) -> c_int;
    fn af_get_backend_count(num_backends: *mut c_uint) -> c_int;
    fn af_get_available_backends(backends: *mut c_int) -> c_int;
}

/// Toggle backends between cuda, opencl or cpu
///
/// # Parameters
///
/// - `backend` to which to switch to
pub fn set_backend(backend: Backend) -> Result<(), AfError> {
    unsafe {
        let err_val = af_set_backend(backend as uint8_t);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Get the available backend count
#[allow(unused_mut)]
pub fn get_backend_count() -> Result<u32, AfError> {
    unsafe {
        let mut temp: u32 = 0;
        let err_val = af_get_backend_count(&mut temp as *mut c_uint);
        match err_val {
            0 => Ok(temp),
            _ => Err(AfError::from(err_val)),
        }
    }
}


/// Get the available backends
#[allow(unused_mut)]
pub fn get_available_backends() -> Result<Vec<Backend>, AfError> {
    unsafe {
        let mut temp: i32 = 0;
        let err_val = af_get_available_backends(&mut temp as *mut c_int);
        match err_val {
            0 => {
                let mut b = Vec::new();
                if temp & 0b0100 == 0b0100 { b.push(Backend::AF_BACKEND_OPENCL); }
                if temp & 0b0010 == 0b0010 { b.push(Backend::AF_BACKEND_CUDA); }
                if temp & 0b0001 == 0b0001 { b.push(Backend::AF_BACKEND_CPU); }
                Ok(b)
            },
            _ => Err(AfError::from(err_val)),
        }
    }
}
