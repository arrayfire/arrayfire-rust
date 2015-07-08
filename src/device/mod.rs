extern crate libc;

use self::libc::c_int;

extern {
    fn af_info() -> c_int;

    fn af_set_device(device: c_int) -> c_int;
}

pub fn info() {
    unsafe { af_info(); }
}

pub fn set_device(device: i32) {
    unsafe { af_set_device(device as c_int); }
}
