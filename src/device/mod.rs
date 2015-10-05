extern crate libc;

use defines::AfError;
use self::libc::c_int;

extern {
    fn af_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int) -> c_int;

    fn af_info() -> c_int;

    fn af_set_device(device: c_int) -> c_int;
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
pub fn info() -> Result<(), AfError> {
    unsafe {
        let err_val = af_info();
        match err_val {
            0 => Ok(()),
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
