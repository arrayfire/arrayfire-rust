extern crate libc;

use super::Array as Array;
use self::libc::{c_int};
use data::constant;

type MutAfArray = *mut self::libc::c_longlong;
type MutDouble  = *mut self::libc::c_double;
type MutUint    = *mut self::libc::c_uint;
type AfArray    = self::libc::c_longlong;

use std::ops::Add;

#[allow(dead_code)]
extern {
    fn af_add(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_sin(out: MutAfArray, arr: AfArray) -> c_int;

}

impl Add<f64> for Array {
    type Output = Array;

    fn add(self, rhs: f64) -> Array {
        let cnst_arr = constant(rhs, self.dims());
        unsafe {
            let mut temp: i64 = 0;
            af_add(&mut temp as MutAfArray, self.get() as AfArray, cnst_arr.get() as AfArray, 0);
            Array {handle: temp}
        }
    }
}

#[allow(unused_mut)]
pub fn sin(input: &Array) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_sin(&mut temp as MutAfArray, input.get() as AfArray);
        Array {handle: temp}
    }
}
