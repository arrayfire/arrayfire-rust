extern crate libc;

use crate::array::Array;
use crate::defines::{AfError, TopkFn};
use crate::error::HANDLE_ERROR;
use self::libc::{c_int, c_uint};
use crate::util::{AfArray, DimT, MutAfArray, MutDouble};
use crate::util::{RealNumber, CovarianceComputable};
use crate::util::{HasAfEnum, MedianComputable, RealFloating};

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
    fn af_topk(vals: MutAfArray, idxs: MutAfArray, arr: AfArray, k: c_int,
               dim: c_int, order: c_uint) -> c_int;
}

/// Find the median along a given dimension
///
///# Parameters
///
/// - `input` is the input Array
/// - `dim` is dimension along which median has to be found
///
///# Return Values
///
/// An Array whose size is equal to input except along the dimension which
/// median needs to be found. Array size along `dim` will be reduced to one.
#[allow(unused_mut)]
pub fn median<T>(input: &Array<T>, dim: i64) -> Array<T>
    where T: HasAfEnum + MedianComputable
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_median(&mut temp as MutAfArray,
                                input.get() as AfArray, dim as DimT);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

macro_rules! stat_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => (
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `input` is the input Array
        /// - `dim` is dimension along which the current stat has to be computed
        ///
        ///# Return Values
        ///
        /// An Array whose size is equal to input except along the dimension which
        /// the stat operation is performed. Array size along `dim` will be reduced to one.
        #[allow(unused_mut)]
        pub fn $fn_name<T>(input: &Array<T>, dim: i64) -> Array< T::MeanOutType >
            where T: HasAfEnum, T::MeanOutType: HasAfEnum
        {
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_fn(&mut temp as MutAfArray,
                                      input.get() as AfArray, dim as DimT);
                HANDLE_ERROR(AfError::from(err_val));
            }
            temp.into()
        }
    )
}

stat_func_def!("Mean along specified dimension", mean, af_mean);
stat_func_def!("Standard deviation along specified dimension", stdev, af_stdev);

macro_rules! stat_wtd_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => (
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `input` is the input Array
        /// - `weights` Array has the weights to be used during the stat computation
        /// - `dim` is dimension along which the current stat has to be computed
        ///
        ///# Return Values
        ///
        /// An Array whose size is equal to input except along the dimension which
        /// the stat operation is performed. Array size along `dim` will be reduced to one.
        #[allow(unused_mut)]
        pub fn $fn_name<T, W>(input: &Array<T>,
                              weights: &Array<W>,
                              dim: i64) -> Array< T::MeanOutType >
            where T: HasAfEnum,
                  T::MeanOutType: HasAfEnum,
                  W: HasAfEnum + RealFloating
        {
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_fn(&mut temp as MutAfArray, input.get() as AfArray,
                                      weights.get() as AfArray, dim as DimT);
                HANDLE_ERROR(AfError::from(err_val));
            }
            temp.into()
        }
    )
}

stat_wtd_func_def!("Weighted mean along specified dimension", mean_weighted, af_mean_weighted);
stat_wtd_func_def!("Weight variance along specified dimension", var_weighted, af_var_weighted);

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
pub fn var<T>(arr: &Array<T>, isbiased: bool, dim: i64) -> Array< T::MeanOutType >
    where T: HasAfEnum, T::MeanOutType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_var(&mut temp as MutAfArray, arr.get() as AfArray,
                             isbiased as c_int, dim as DimT);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
pub fn cov<T>(x: &Array<T>, y: &Array<T>, isbiased: bool) -> Array< T::MeanOutType >
    where T: HasAfEnum + CovarianceComputable,
          T::MeanOutType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_cov(&mut temp as MutAfArray, x.get() as AfArray,
                             y.get() as AfArray, isbiased as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
pub fn var_all<T:HasAfEnum>(input: &Array<T>, isbiased: bool) -> (f64, f64)
{
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_var_all(&mut real as MutDouble, &mut imag as MutDouble,
                                 input.get() as AfArray, isbiased as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    (real, imag)
}

macro_rules! stat_all_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => (
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `input` is the input Array
        ///
        ///# Return Values
        ///
        /// A tuple of 64-bit floating point values with the stat values.
        #[allow(unused_mut)]
        pub fn $fn_name<T:HasAfEnum>(input: &Array<T>) -> (f64, f64) {
            let mut real: f64 = 0.0;
            let mut imag: f64 = 0.0;
            unsafe {
                let err_val = $ffi_fn(&mut real as MutDouble, &mut imag as MutDouble,
                                      input.get() as AfArray);
                HANDLE_ERROR(AfError::from(err_val));
            }
            (real, imag)
        }
    )
}

stat_all_func_def!("Compute mean of all data", mean_all, af_mean_all);
stat_all_func_def!("Compute standard deviation of all data", stdev_all, af_stdev_all);

/// Compute median of all data
///
///# Parameters
///
/// - `input` is the input Array
///
///# Return Values
///
/// A tuple of 64-bit floating point values with the median
#[allow(unused_mut)]
pub fn median_all<T>(input: &Array<T>) -> (f64, f64)
    where T: HasAfEnum + MedianComputable
{
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_median_all(&mut real as MutDouble,
                                    &mut imag as MutDouble,
                                    input.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
    }
    (real, imag)
}

macro_rules! stat_wtd_all_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => (
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `input` is the input Array
        /// - `weights` Array has the weights
        ///
        ///# Return Values
        ///
        /// A tuple of 64-bit floating point values with the stat values.
        #[allow(unused_mut)]
        pub fn $fn_name<T, W>(input: &Array<T>, weights: &Array<W>) -> (f64, f64)
            where T: HasAfEnum,
                  W: HasAfEnum + RealFloating
        {
            let mut real: f64 = 0.0;
            let mut imag: f64 = 0.0;
            unsafe {
                let err_val = $ffi_fn(&mut real as MutDouble, &mut imag as MutDouble,
                                      input.get() as AfArray, weights.get() as AfArray);
                HANDLE_ERROR(AfError::from(err_val));
            }
            (real, imag)
        }
    )
}

stat_wtd_all_func_def!("Compute weighted mean of all data",
                       mean_all_weighted, af_mean_all_weighted);
stat_wtd_all_func_def!("Compute weighted variance of all data",
                       var_all_weighted, af_var_all_weighted);

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
pub fn corrcoef<T>(x: &Array<T>, y: &Array<T>) -> (f64, f64)
    where T: HasAfEnum + RealNumber
{
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_corrcoef(&mut real as MutDouble, &mut imag as MutDouble,
                                  x.get() as AfArray, y.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
    }
    (real, imag)
}

/// Find top k elements along a given dimension
///
/// This function returns the top k values along a given dimension of the input
/// array. The indices along with their values are returned.
///
/// If the input is a multi-dimensional array, the indices will be the index of
/// the value in that dimension. Order of duplicate values are not preserved.
///
/// This function is optimized for small values of k. Currently, topk elements
/// can be found only along dimension 0.
///
/// # Parameters
///
/// - `input` is the values from which top k elements are to be retrieved
/// - `k` is the number of top elements to be retrieve
/// - `dim` is the dimension along which the retrieval operation has to performed
/// - `order` is an enum that can take values of type [TopkFn](./enum.TopkFn.html)
///
/// # Return Values
///
/// A tuple(couple) of Array's with the first Array containing the topk values
/// with the second Array containing the indices of the topk values in the input
/// data.
pub fn topk<T>(input: &Array<T>, k: u32, dim: i32, order: TopkFn) -> (Array<T>, Array<u32>)
    where T: HasAfEnum
{
    let mut t0: i64 = 0;
    let mut t1: i64 = 0;
    unsafe {
        let err_val = af_topk(&mut t0 as MutAfArray, &mut t1 as MutAfArray,
                              input.get() as AfArray, k as c_int, dim as c_int,
                              order as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
    }
    (t0.into(), t1.into())
}
