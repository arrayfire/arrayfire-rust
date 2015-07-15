extern crate libc;

use self::libc::c_int;

extern {
    fn af_get_version(major: *mut c_int, minor: *mut c_int, patch: *mut c_int) -> c_int;

    fn af_info() -> c_int;

    fn af_set_device(device: c_int) -> c_int;
}

pub fn get_version() -> (i32, i32, i32) {
    unsafe {
        let mut maj: i32 = 0;
        let mut min: i32 = 0;
        let mut pat: i32 = 0;
        af_get_version(&mut maj as *mut c_int, &mut min as *mut c_int, &mut pat as *mut c_int);
        (maj, min, pat)
    }
}

pub fn info() {
    unsafe { af_info(); }
}

pub fn set_device(device: i32) {
    unsafe { af_set_device(device as c_int); }
}
