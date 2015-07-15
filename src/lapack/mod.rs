extern crate libc;

use array::Array;
use defines::NormType;
use util::to_u32;
use self::libc::{uint8_t, c_int, c_uint, c_double};

type MutAfArray = *mut self::libc::c_longlong;
type MutDouble  = *mut self::libc::c_double;
type AfArray    = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_lu(lower: MutAfArray, upper: MutAfArray, pivot: MutAfArray, input: AfArray) -> c_int;
    fn af_lu_inplace(pivot: MutAfArray, input: AfArray, is_lapack_piv: c_int) -> c_int;
    fn af_qr(q: MutAfArray, r: MutAfArray, tau: MutAfArray, input: AfArray) -> c_int;
    fn af_qr_inplace(tau: MutAfArray, input: AfArray) -> c_int;
    fn af_cholesky(out: MutAfArray, info: *mut c_int, input: AfArray, is_upper: c_int) -> c_int;
    fn af_cholesky_inplace(info: *mut c_int, input: AfArray, is_upper: c_int) -> c_int;
    fn af_solve(x: MutAfArray, a: AfArray, b: AfArray, options: c_uint) -> c_int;
    fn af_solve_lu(x: MutAfArray, a: AfArray, piv: AfArray, b: AfArray, options: c_uint) -> c_int;
    fn af_inverse(out: MutAfArray, input: AfArray, options: c_uint) -> c_int;
    fn af_rank(rank: *mut c_uint, input: AfArray, tol: c_double) -> c_int;
    fn af_det(det_real: MutDouble, det_imag: MutDouble, input: AfArray) -> c_int;
    fn af_norm(out: MutDouble, input: AfArray, ntype: uint8_t, p: c_double, q: c_double) -> c_int;
}

#[allow(unused_mut)]
pub fn lu(input: &Array) -> (Array, Array, Array) {
    unsafe {
        let mut lower: i64 = 0;
        let mut upper: i64 = 0;
        let mut pivot: i64 = 0;
        af_lu(&mut lower as MutAfArray, &mut upper as MutAfArray,
              &mut pivot as MutAfArray, input.get() as AfArray);
        (Array::from(lower), Array::from(upper), Array::from(pivot))
    }
}

#[allow(unused_mut)]
pub fn lu_inplace(input: &mut Array, is_lapack_piv: bool) -> Array {
    unsafe {
        let mut pivot: i64 = 0;
        af_lu_inplace(&mut pivot as MutAfArray, input.get() as AfArray,
                      is_lapack_piv as c_int);
        Array::from(pivot)
    }
}

#[allow(unused_mut)]
pub fn qr(input: &Array) -> (Array, Array, Array) {
    unsafe {
        let mut q: i64 = 0;
        let mut r: i64 = 0;
        let mut tau: i64 = 0;
        af_qr(&mut q as MutAfArray, &mut r as MutAfArray,
              &mut tau as MutAfArray, input.get() as AfArray);
        (Array::from(q), Array::from(r), Array::from(tau))
    }
}

#[allow(unused_mut)]
pub fn qr_inplace(input: &mut Array) -> Array {
    unsafe {
        let mut tau: i64 = 0;
        af_lu_inplace(&mut tau as MutAfArray, input.get() as AfArray);
        Array::from(tau)
    }
}

#[allow(unused_mut)]
pub fn cholesky(input: &Array, is_upper: bool) -> (Array, i32) {
    unsafe {
        let mut temp: i64 = 0;
        let mut info: i32 = 0;
        af_cholesky(&mut temp as MutAfArray, &mut info as *mut c_int,
                    input.get() as AfArray, is_upper as c_int);
        (Array::from(temp), info)
    }
}

#[allow(unused_mut)]
pub fn cholesky_inplace(input: &mut Array, is_upper: bool) -> i32 {
    unsafe {
        let mut info: i32 = 0;
        af_cholesky_inplace(&mut info as *mut c_int, input.get() as AfArray,
                            is_upper as c_int);
        info
    }
}

#[allow(unused_mut)]
pub fn solve(a: &Array, b: &Array, options: MatProp) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_solve(&mut temp as MutAfArray, a.get() as AfArray,
                 b.get() as AfArray, to_u32(options) as c_uint);
        Array::from(temp)
    }
}

#[allow(unused_mut)]
pub fn solve_lu(a: &Array, piv: &Array, b: &Array, options: MatProp) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_solve_lu(&mut temp as MutAfArray, a.get() as AfArray, piv.get() as AfArray,
                    b.get() as AfArray, to_u32(options) as c_uint);
        Array::from(temp)
    }
}

#[allow(unused_mut)]
pub fn inverse(input: &Array, options: MatProp) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_solve(&mut temp as MutAfArray, input.get() as AfArray, to_u32(options) as c_uint);
        Array::from(temp)
    }
}

#[allow(unused_mut)]
pub fn rank(input: &Array, tol: f64) -> u32 {
    unsafe {
        let mut temp: u32 = 0;
        af_rank(&mut temp as *mut c_uint, input.get() as AfArray, tol as c_double);
        temp
    }
}

#[allow(unused_mut)]
pub fn det(input: &Array) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        af_det(&mut real as MutDouble, &mut imag as MutDouble, input.get() as AfArray);
        (real, imag)
    }
}

#[allow(unused_mut)]
pub fn norm(input: &Array, ntype: NormType, p: f64, q: f64) -> f64 {
    unsafe {
        let mut out: f64 = 0.0;
        af_norm(&mut out as MutDouble, input.get() as AfArray, ntype as uint8_t,
                p as c_double, q as c_double);
        out
    }
}
