extern crate libc;

use defines::{AfError, DType};
use error::HANDLE_ERROR;
use self::libc::{c_int, size_t, c_char, c_void};
use std::ffi::{CStr, CString};
use util::free_host;

extern {
    fn af_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int) -> c_int;
    fn af_info() -> c_int;
    fn af_info_string(str: *mut *mut c_char, verbose: bool) -> c_int;
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
pub fn get_version() -> (i32, i32, i32) {
    unsafe {
        let mut maj: i32 = 0;
        let mut min: i32 = 0;
        let mut pat: i32 = 0;
        let err_val = af_get_version(&mut maj as *mut c_int,
                                     &mut min as *mut c_int, &mut pat as *mut c_int);
        HANDLE_ERROR(AfError::from(err_val));
        (maj, min, pat)
    }
}

/// Print library meta-info
///
/// # Examples
///
/// An example output of `af::info` call looks like below
///
/// ```text
/// ArrayFire v3.0.0 (CUDA, 64-bit Mac OSX, build d8d4b38)
/// Platform: CUDA Toolkit 7, Driver: CUDA Driver Version: 7000
/// [0] GeForce GT 750M, 2048 MB, CUDA Compute 3.0
/// ```
pub fn info() {
    unsafe {
        let err_val = af_info();
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Return library meta-info as `String`
///
/// # Examples
///
/// An example output of `af::info_string` call looks like below
///
/// ```text
/// ArrayFire v3.0.0 (CUDA, 64-bit Mac OSX, build d8d4b38)
/// Platform: CUDA Toolkit 7, Driver: CUDA Driver Version: 7000
/// [0] GeForce GT 750M, 2048 MB, CUDA Compute 3.0
/// ```
pub fn info_string(verbose: bool) -> String {
    let result: String;
    unsafe {
        let mut tmp: *mut c_char = ::std::ptr::null_mut();
        let err_val = af_info_string(&mut tmp, verbose);
        HANDLE_ERROR(AfError::from(err_val));
        result = CStr::from_ptr(tmp).to_string_lossy().into_owned();
        free_host(tmp);
    }
    result
}

/// Initialize ArrayFire library
///
/// 0th device will be the default device unless init call
/// is followed by set_device
pub fn init() {
    unsafe {
        let err_val = af_init();
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Get total number of available devices
pub fn device_count() -> i32 {
    unsafe {
        let mut temp: i32 = 0;
        let err_val = af_get_device_count(&mut temp as *mut c_int);
        HANDLE_ERROR(AfError::from(err_val));
        temp
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
pub fn is_double_available(device: i32) -> bool {
    unsafe {
        let mut temp: i32 = 0;
        let err_val = af_get_dbl_support(&mut temp as *mut c_int, device as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        temp > 0
    }
}

/// Set active device
///
/// # Parameters
///
/// - `device` is the value of the device identifier which has to be set as active
pub fn set_device(device: i32) {
    unsafe {
        let err_val = af_set_device(device as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Get the current active device id
pub fn get_device() -> i32 {
    unsafe {
        let mut temp: i32 = 0;
        let err_val = af_get_device(&mut temp as *mut c_int);
        HANDLE_ERROR(AfError::from(err_val));
        temp
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
pub fn device_mem_info() -> (usize, usize, usize, usize) {
    unsafe {
        let mut o0: usize = 0;
        let mut o1: usize = 0;
        let mut o2: usize = 0;
        let mut o3: usize = 0;
        let err_val = af_device_mem_info(&mut o0 as *mut size_t,
                                         &mut o1 as *mut size_t,
                                         &mut o2 as *mut size_t,
                                         &mut o3 as *mut size_t);
        HANDLE_ERROR(AfError::from(err_val));
        (o0, o1, o2, o3)
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
pub fn print_mem_info(msg: String, device: i32) {
    unsafe {
        let cmsg = CString::new(msg.as_bytes());
        match cmsg {
            Ok(v) => {
                let err_val = af_print_mem_info(v.to_bytes_with_nul().as_ptr() as * const c_char,
                                                device as c_int);
                HANDLE_ERROR(AfError::from(err_val));
            },
            Err(_) => HANDLE_ERROR(AfError::ERR_INTERNAL),
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
pub fn set_mem_step_size(step_bytes: usize) {
    unsafe {
        let err_val = af_set_mem_step_size(step_bytes as size_t);
        HANDLE_ERROR(AfError::from(err_val));
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
pub fn get_mem_step_size() -> usize {
    unsafe {
        let mut temp: usize = 0;
        let err_val = af_get_mem_step_size(&mut temp as *mut size_t);
        HANDLE_ERROR(AfError::from(err_val));
        temp
    }
}

/// Call the garbage collection routine
pub fn device_gc() {
    unsafe {
        let err_val = af_device_gc();
        HANDLE_ERROR(AfError::from(err_val));
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
pub fn sync(device: i32) {
    unsafe {
        let err_val = af_sync(device as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
}
