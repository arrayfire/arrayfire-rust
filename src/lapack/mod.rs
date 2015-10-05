extern crate libc;

use array::Array;
use defines::AfError;
use defines::{MatProp, NormType};
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
pub fn lu(input: &Array) -> Result<(Array, Array, Array), AfError> {
    unsafe {
        let mut lower: i64 = 0;
        let mut upper: i64 = 0;
        let mut pivot: i64 = 0;
        let err_val = af_lu(&mut lower as MutAfArray, &mut upper as MutAfArray,
                            &mut pivot as MutAfArray, input.get() as AfArray);
        match err_val {
            0 => Ok((Array::from(lower), Array::from(upper), Array::from(pivot))),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn lu_inplace(input: &mut Array, is_lapack_piv: bool) -> Result<Array, AfError> {
    unsafe {
        let mut pivot: i64 = 0;
        let err_val = af_lu_inplace(&mut pivot as MutAfArray, input.get() as AfArray,
                                    is_lapack_piv as c_int);
        match err_val {
            0 => Ok(Array::from(pivot)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn qr(input: &Array) -> Result<(Array, Array, Array), AfError> {
    unsafe {
        let mut q: i64 = 0;
        let mut r: i64 = 0;
        let mut tau: i64 = 0;
        let err_val = af_qr(&mut q as MutAfArray, &mut r as MutAfArray,
                            &mut tau as MutAfArray, input.get() as AfArray);
        match err_val {
            0 => Ok((Array::from(q), Array::from(r), Array::from(tau))),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn qr_inplace(input: &mut Array) -> Result<Array, AfError> {
    unsafe {
        let mut tau: i64 = 0;
        let err_val = af_qr_inplace(&mut tau as MutAfArray, input.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(tau)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn cholesky(input: &Array, is_upper: bool) -> Result<(Array, i32), AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let mut info: i32 = 0;
        let err_val = af_cholesky(&mut temp as MutAfArray, &mut info as *mut c_int,
                                  input.get() as AfArray, is_upper as c_int);
        match err_val {
            0 => Ok((Array::from(temp), info)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn cholesky_inplace(input: &mut Array, is_upper: bool) -> Result<i32, AfError> {
    unsafe {
        let mut info: i32 = 0;
        let err_val = af_cholesky_inplace(&mut info as *mut c_int, input.get() as AfArray,
                                          is_upper as c_int);
        match err_val {
            0 => Ok(info),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn solve(a: &Array, b: &Array, options: MatProp) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_solve(&mut temp as MutAfArray, a.get() as AfArray,
                               b.get() as AfArray, to_u32(options) as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn solve_lu(a: &Array, piv: &Array, b: &Array,
                options: MatProp) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_solve_lu(&mut temp as MutAfArray, a.get() as AfArray, piv.get() as AfArray,
                    b.get() as AfArray, to_u32(options) as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn inverse(input: &Array, options: MatProp) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_inverse(&mut temp as MutAfArray, input.get() as AfArray, to_u32(options) as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn rank(input: &Array, tol: f64) -> Result<u32, AfError> {
    unsafe {
        let mut temp: u32 = 0;
        let err_val = af_rank(&mut temp as *mut c_uint, input.get() as AfArray, tol as c_double);
        match err_val {
            0 => Ok(temp),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn det(input: &Array) -> Result<(f64, f64), AfError> {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        let err_val = af_det(&mut real as MutDouble, &mut imag as MutDouble, input.get() as AfArray);
        match err_val {
            0 => Ok((real, imag)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn norm(input: &Array, ntype: NormType, p: f64, q: f64) -> Result<f64, AfError> {
    unsafe {
        let mut out: f64 = 0.0;
        let err_val = af_norm(&mut out as MutDouble, input.get() as AfArray, ntype as uint8_t,
                              p as c_double, q as c_double);
        match err_val {
            0 => Ok(out),
            _ => Err(AfError::from(err_val)),
        }
    }
}
