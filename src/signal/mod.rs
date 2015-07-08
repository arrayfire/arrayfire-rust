extern crate libc;

use super::Array as Array;
use self::libc::{c_int, c_double, c_longlong};

type MutAfArray = *mut self::libc::c_longlong;
type AfArray    = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_fft(out: MutAfArray, arr: AfArray,
              nfac: c_double, odim0: c_longlong) -> c_int;

    fn af_fft2(out: MutAfArray, arr: AfArray, nfac: c_double,
               odim0: c_longlong, odim1: c_longlong) -> c_int;

    fn af_fft3(out: MutAfArray, arr: AfArray, nfac: c_double,
               odim0: c_longlong, odim1: c_longlong, odim2: c_longlong) -> c_int;
}

#[allow(unused_mut)]
pub fn fft(input: &Array, norm_factor: f64, odim0: i64) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_fft(&mut temp as MutAfArray, input.get() as AfArray,
               norm_factor as c_double, odim0 as c_longlong);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn fft2(input: &Array, norm_factor: f64, odim0: i64, odim1: i64) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_fft2(&mut temp as MutAfArray, input.get() as AfArray,
                norm_factor as c_double, odim0 as c_longlong, odim1 as c_longlong);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn fft3(input: &Array, norm_factor: f64, odim0: i64, odim1: i64, odim2: i64) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_fft3(&mut temp as MutAfArray, input.get() as AfArray, norm_factor as c_double,
                odim0 as c_longlong, odim1 as c_longlong, odim2 as c_longlong);
        Array {handle: temp}
    }
}
