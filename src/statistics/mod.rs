extern crate libc;

use array::Array;
use defines::AfError;
use error::HANDLE_ERROR;
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
        /// Compute a statistic along given dimension for an Array
        ///
        /// # Parameters
        ///
        /// - `input` is the input Array
        /// - `dim` is dimension along which the current stat has to be computed
        ///
        /// # Return Values
        ///
        /// An Array whose size is equal to input except along the dimension which the stat
        /// operation is performed. Array size along `dim` will be reduced to one.
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, dim: i64) -> Array {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_fn(&mut temp as MutAfArray, input.get() as AfArray, dim as DimT);
                HANDLE_ERROR(AfError::from(err_val));
                Array::from(temp)
            }
        }
    )
}

stat_func_def!(mean, af_mean);
stat_func_def!(stdev, af_stdev);
stat_func_def!(median, af_median);

macro_rules! stat_wtd_func_def {
    ($fn_name: ident, $ffi_fn: ident) => (
        /// Compute a weighted statistic along given dimension for an Array
        ///
        /// # Parameters
        ///
        /// - `input` is the input Array
        /// - `weights` Array has the weights to be used during the stat computation
        /// - `dim` is dimension along which the current stat has to be computed
        ///
        /// # Return Values
        ///
        /// An Array whose size is equal to input except along the dimension which the stat
        /// operation is performed. Array size along `dim` will be reduced to one.
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, weights: &Array, dim: i64) -> Array {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_fn(&mut temp as MutAfArray, input.get() as AfArray,
                                      weights.get() as AfArray, dim as DimT);
                HANDLE_ERROR(AfError::from(err_val));
                Array::from(temp)
            }
        }
    )
}

stat_wtd_func_def!(mean_weighted, af_mean_weighted);
stat_wtd_func_def!(var_weighted, af_var_weighted);

/// Compute Variance along a specific dimension
///
/// # Parameters
///
/// - `arr` is the input Array
/// - `isbiased` is boolean denoting population variance(False) or Sample variance(True)
/// - `dim` is the dimension along which the variance is extracted
///
/// # Return Values
///
/// Array with variance of input Array `arr` along dimension `dim`.
#[allow(unused_mut)]
pub fn var(arr: &Array, isbiased: bool, dim: i64) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_var(&mut temp as MutAfArray, arr.get() as AfArray,
                             isbiased as c_int, dim as DimT);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Compute covariance of two Arrays
///
/// # Parameters
///
/// - `x` is the first Array
/// - `y` is the second Array
/// - `isbiased` is boolean denoting if biased estimate should be taken(default: False)
///
/// # Return Values
///
/// An Array with Covariance values
#[allow(unused_mut)]
pub fn cov(x: &Array, y: &Array, isbiased: bool) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_cov(&mut temp as MutAfArray, x.get() as AfArray,
                             y.get() as AfArray, isbiased as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Compute Variance of all elements
///
/// # Parameters
///
/// - `input` is the input Array
/// - `isbiased` is boolean denoting population variance(False) or sample variance(True)
///
/// # Return Values
///
/// A tuple of 64-bit floating point values that has the variance of `input` Array.
#[allow(unused_mut)]
pub fn var_all(input: &Array, isbiased: bool) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        let err_val = af_var_all(&mut real as MutDouble, &mut imag as MutDouble,
                                 input.get() as AfArray, isbiased as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        (real, imag)
    }
}

macro_rules! stat_all_func_def {
    ($fn_name: ident, $ffi_fn: ident) => (
        /// Compute statistic for all elements of Array
        ///
        /// # Parameters
        ///
        /// - `input` is the input Array
        ///
        /// # Return Values
        ///
        /// A tuple of 64-bit floating point values with the stat values.
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array) -> (f64, f64) {
            unsafe {
                let mut real: f64 = 0.0;
                let mut imag: f64 = 0.0;
                let err_val = $ffi_fn(&mut real as MutDouble, &mut imag as MutDouble,
                                      input.get() as AfArray);
                HANDLE_ERROR(AfError::from(err_val));
                (real, imag)
            }
        }
    )
}

stat_all_func_def!(mean_all, af_mean_all);
stat_all_func_def!(stdev_all, af_stdev_all);
stat_all_func_def!(median_all, af_median_all);

macro_rules! stat_wtd_all_func_def {
    ($fn_name: ident, $ffi_fn: ident) => (
        /// Compute weighted statistic for all elements of Array
        ///
        /// # Parameters
        ///
        /// - `input` is the input Array
        /// - `weights` Array has the weights
        ///
        /// # Return Values
        ///
        /// A tuple of 64-bit floating point values with the stat values.
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, weights: &Array) -> (f64, f64) {
            unsafe {
                let mut real: f64 = 0.0;
                let mut imag: f64 = 0.0;
                let err_val = $ffi_fn(&mut real as MutDouble, &mut imag as MutDouble,
                                      input.get() as AfArray, weights.get() as AfArray);
                HANDLE_ERROR(AfError::from(err_val));
                (real, imag)
            }
        }
    )
}

stat_wtd_all_func_def!(mean_all_weighted, af_mean_all_weighted);
stat_wtd_all_func_def!(var_all_weighted, af_var_all_weighted);

/// Compute correlation coefficient
///
/// # Parameters
///
/// - `x` is the first Array
/// - `y` isthe second Array
///
/// # Return Values
/// A tuple of 64-bit floating point values with the coefficients.
#[allow(unused_mut)]
pub fn corrcoef(x: &Array, y: &Array) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        let err_val = af_corrcoef(&mut real as MutDouble, &mut imag as MutDouble,
                                  x.get() as AfArray, y.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        (real, imag)
    }
}