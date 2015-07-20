extern crate libc;

use array::Array;
use defines::AfError;
use self::libc::{c_int};

type MutAfArray = *mut self::libc::c_longlong;
type MutDouble  = *mut self::libc::c_double;
type AfArray    = self::libc::c_longlong;
type DimT       = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_mean(out: MutAfArray, arr: AfArray, dim: DimT) -> c_int;
    fn af_stdev(out: MutAfArray, arr: AfArray, dim: DimT) -> c_int;
    fn af_median(out: MutAfArray, arr: AfArray, dim: DimT) -> c_int;

    fn af_mean_weighted(out: MutAfArray, arr: AfArray, wts: AfArray, dim: DimT) -> c_int;
    fn af_var_weighted(out: MutAfArray, arr: AfArray, wts: AfArray, dim: DimT) -> c_int;

    fn af_var(out: MutAfArray, arr: AfArray, isbiased: c_int, dim: DimT) -> c_int;
    fn af_cov(out: MutAfArray, X: AfArray, Y: AfArray, isbiased: c_int) -> c_int;
    fn af_var_all(real: MutDouble, imag: MutDouble, arr: AfArray, isbiased: c_int) -> c_int;

    fn af_mean_all(real: MutDouble, imag: MutDouble, arr: AfArray) -> c_int;
    fn af_stdev_all(real: MutDouble, imag: MutDouble, arr: AfArray) -> c_int;
    fn af_median_all(real: MutDouble, imag: MutDouble, arr: AfArray) -> c_int;

    fn af_mean_all_weighted(real: MutDouble, imag: MutDouble, arr: AfArray, wts: AfArray) -> c_int;
    fn af_var_all_weighted(real: MutDouble, imag: MutDouble, arr: AfArray, wts: AfArray) -> c_int;

    fn af_corrcoef(real: MutDouble, imag: MutDouble, X: AfArray, Y: AfArray) -> c_int;
}

macro_rules! stat_func_def {
    ($fn_name: ident, $ffi_fn: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, dim: i64) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_fn(&mut temp as MutAfArray, input.get() as AfArray, dim as DimT);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

stat_func_def!(mean, af_mean);
stat_func_def!(stdev, af_stdev);
stat_func_def!(median, af_median);

macro_rules! stat_wtd_func_def {
    ($fn_name: ident, $ffi_fn: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, weights: &Array, dim: i64) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_fn(&mut temp as MutAfArray, input.get() as AfArray,
                                      weights.get() as AfArray, dim as DimT);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

stat_wtd_func_def!(mean_weighted, af_mean_weighted);
stat_wtd_func_def!(var_weighted, af_var_weighted);

#[allow(unused_mut)]
pub fn var(arr: &Array, isbiased: bool, dim: i64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_var(&mut temp as MutAfArray, arr.get() as AfArray,
                             isbiased as c_int, dim as DimT);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn cov(x: &Array, y: &Array, isbiased: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_cov(&mut temp as MutAfArray, x.get() as AfArray,
                             y.get() as AfArray, isbiased as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn var_all(input: &Array, isbiased: bool) -> Result<(f64, f64), AfError> {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        let err_val = af_var_all(&mut real as MutDouble, &mut imag as MutDouble,
                                 input.get() as AfArray, isbiased as c_int);
        match err_val {
            0 => Ok((real, imag)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

macro_rules! stat_all_func_def {
    ($fn_name: ident, $ffi_fn: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array) -> Result<(f64, f64), AfError> {
            unsafe {
                let mut real: f64 = 0.0;
                let mut imag: f64 = 0.0;
                let err_val = $ffi_fn(&mut real as MutDouble, &mut imag as MutDouble,
                                      input.get() as AfArray);
                match err_val {
                    0 => Ok((real, imag)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

stat_all_func_def!(mean_all, af_mean_all);
stat_all_func_def!(stdev_all, af_stdev_all);
stat_all_func_def!(median_all, af_median_all);

macro_rules! stat_wtd_all_func_def {
    ($fn_name: ident, $ffi_fn: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, weights: &Array) -> Result<(f64, f64), AfError> {
            unsafe {
                let mut real: f64 = 0.0;
                let mut imag: f64 = 0.0;
                let err_val = $ffi_fn(&mut real as MutDouble, &mut imag as MutDouble,
                                      input.get() as AfArray, weights.get() as AfArray);
                match err_val {
                    0 => Ok((real, imag)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

stat_wtd_all_func_def!(mean_all_weighted, af_mean_all_weighted);
stat_wtd_all_func_def!(var_all_weighted, af_var_all_weighted);

#[allow(unused_mut)]
pub fn corrcoef(x: &Array, y: &Array) -> Result<(f64, f64), AfError> {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        let err_val = af_corrcoef(&mut real as MutDouble, &mut imag as MutDouble,
                                  x.get() as AfArray, y.get() as AfArray);
        match err_val {
            0 => Ok((real, imag)),
            _ => Err(AfError::from(err_val)),
        }
    }
}
