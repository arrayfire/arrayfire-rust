extern crate libc;

use defines::AfError;
use self::libc::{c_int, size_t, c_char};
use std::ffi::CString;

extern {
    fn af_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int) -> c_int;
    fn af_info() -> c_int;
    fn af_init() -> c_int;
    fn af_get_device_count(nDevices: *mut c_int) -> c_int;
    fn af_get_dbl_support(available: *mut c_int, device: c_int) -> c_int;
    fn af_set_device(device: c_int) -> c_int;
    fn af_get_device(device: *mut c_int) -> c_int;
    fn af_device_mem_info(alloc_bytes: *mut size_t, alloc_buffers: *mut size_t,
                          lock_bytes: *mut size_t, lock_buffers: *mut size_t) -> c_int;
    fn af_print_mem_info(msg: *const c_char, device_id: c_int) -> c_int;
    fn af_set_mem_step_size(step_bytes: size_t) -> c_int;
    fn af_get_mem_step_size(step_bytes: *mut size_t) -> c_int;
    fn af_device_gc() -> c_int;
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
/// ```ignore
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

/// Initialize ArrayFire library
///
/// 0th device will be the default device unless init call
/// is followed by set_device
pub fn init() -> Result<(), AfError> {
    unsafe {
        let err_val = af_init();
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

/// Get memory information from the memory manager for the current active device
///
/// # Parameters
///
/// This function doesn't take any input parameters
///
/// # Return Values
///
/// A quadruple of values regarding the following information.
///
/// * Number of bytes allocated
/// * Number of buffers allocated
/// * Number of bytes locked
/// * Number of buffers locked
pub fn device_mem_info() -> Result<(u64, u64, u64, u64), AfError> {
    unsafe {
        let mut o0: u64 = 0;
        let mut o1: u64 = 0;
        let mut o2: u64 = 0;
        let mut o3: u64 = 0;
        let err_val = af_device_mem_info(&mut o0 as *mut size_t,
                                         &mut o1 as *mut size_t,
                                         &mut o2 as *mut size_t,
                                         &mut o3 as *mut size_t);
        match err_val {
            0 => Ok((o0, o1, o2, o3)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Print buffer details from the ArrayFire device manager
///
/// This information is printed in the form of a table.
///
/// # Parameters
///
/// - `msg` is a message to print before the table
/// - `device` is the id of the device for which buffer details are to be printed
///
/// # Return Values
///
/// None
pub fn print_mem_info(msg: String, device: i32) -> Result<(), AfError> {
    unsafe {
        let cmsg = CString::new(msg.as_bytes());
        match cmsg {
            Ok(v) => {
                let err_val = af_print_mem_info(v.to_bytes_with_nul().as_ptr() as * const c_char,
                                                device as c_int);
                match err_val {
                    0 => Ok(()),
                    _ => Err(AfError::from(err_val)),
                }
            },
            Err(_) => Err(AfError::from(AfError::ERR_INTERNAL)),
        }
    }
}

/// Set the minimum memory chunk size
///
/// # Parameters
///
/// - `step_bytes` is the size of minimum memory chunk in bytes
///
/// # Return Values
///
/// None
pub fn set_mem_step_size(step_bytes: u64) -> Result<(), AfError> {
    unsafe {
        let err_val = af_set_mem_step_size(step_bytes as size_t);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Get the minimum memory chunk size
///
/// # Parameters
///
/// None
///
/// # Return Values
///
/// Returns is the size of minimum memory chunk in bytes
pub fn get_mem_step_size() -> Result<u64, AfError> {
    unsafe {
        let mut temp: u64 = 0;
        let err_val = af_get_mem_step_size(&mut temp as *mut size_t);
        match err_val {
            0 => Ok(temp),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Call the garbage collection routine
pub fn device_gc() -> Result<(), AfError> {
    unsafe {
        let err_val = af_device_gc();
        match err_val {
            0 => Ok(()),
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
