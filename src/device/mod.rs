extern crate libc;

use defines::{AfError, AfBackend};
use self::libc::{c_int, c_uint, uint8_t};


extern {
    fn af_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int) -> c_int;
    fn af_info() -> c_int;
    fn af_get_device_count(nDevices: *mut c_int) -> c_int;
    fn af_get_dbl_support(available: *mut c_int, device: c_int) -> c_int;
    fn af_set_device(device: c_int) -> c_int;
    fn af_set_backend(bknd: uint8_t) -> c_int;
    fn af_get_backend_count(num_backends: *mut c_uint) -> c_int;
    fn af_get_device(device: *mut c_int) -> c_int;
    fn af_sync(device: c_int) -> c_int;
}

/// Get ArrayFire Version Number
///
/// # Return Values
/// A triplet of integers indicating major, minor & fix release version numbers.
pub fn get_version() -> Result<(i32, i32, i32), AfError> {
    unsafe {
        let mut maj: i32 = 0;
        let mut min: i32 = 0;
        let mut pat: i32 = 0;
        let err_val = af_get_version(&mut maj as *mut c_int,
                                     &mut min as *mut c_int, &mut pat as *mut c_int);
        match err_val {
            0 => Ok((maj, min, pat)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Print library meta-info
///
/// # Examples
///
/// An example output of `af::info` call looks like below
///
/// ```
/// ArrayFire v3.0.0 (CUDA, 64-bit Mac OSX, build d8d4b38)
/// Platform: CUDA Toolkit 7, Driver: CUDA Driver Version: 7000
/// [0] GeForce GT 750M, 2048 MB, CUDA Compute 3.0
/// ```
pub fn info() -> Result<(), AfError> {
    unsafe {
        let err_val = af_info();
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Get total number of available devices
pub fn device_count() -> Result<i32, AfError> {
    unsafe {
        let mut temp: i32 = 0;
        let err_val = af_get_device_count(&mut temp as *mut c_int);
        match err_val {
            0 => Ok(temp),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Check if a device has double support
///
/// # Parameters
///
/// - `device` is the device for which double support is checked for
///
/// # Return Values
///
/// `True` if `device` device has double support, `False` otherwise.
pub fn is_double_available(device: i32) -> Result<bool, AfError> {
    unsafe {
        let mut temp: i32 = 0;
        let err_val = af_get_dbl_support(&mut temp as *mut c_int, device as c_int);
        match err_val {
            0 => Ok(temp > 0),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Set active device
///
/// # Parameters
///
/// - `device` is the value of the device identifier which has to be set as active
pub fn set_device(device: i32) -> Result<(), AfError> {
    unsafe {
        let err_val = af_set_device(device as c_int);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Get the current active device id
pub fn get_device() -> Result<i32, AfError> {
    unsafe {
        let mut temp: i32 = 0;
        let err_val = af_get_device(&mut temp as *mut c_int);
        match err_val {
            0 => Ok(temp),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Sync all operations on given device
///
/// # Parameters
///
/// - `device` on which the operations are to be synced
///
/// # Return Values
///
/// None
pub fn sync(device: i32) -> Result<(), AfError> {
    unsafe {
        let err_val = af_sync(device as c_int);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Toggle backends between cuda, opencl or cpu
///
/// # Parameters
///
/// - `backend` to which to switch to
pub fn set_backend(backend: AfBackend) -> Result<(), AfError> {
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
