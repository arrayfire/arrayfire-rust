use super::core::{
    af_array, dim_t, AfError, Array, CovarianceComputable, HasAfEnum, MedianComputable,
    RealFloating, RealNumber, TopkFn, VarianceBias, HANDLE_ERROR,
};

use libc::{c_double, c_int, c_uint};

extern "C" {
    fn af_mean(out: *mut af_array, arr: af_array, dim: dim_t) -> c_int;
    fn af_median(out: *mut af_array, arr: af_array, dim: dim_t) -> c_int;

    fn af_mean_weighted(out: *mut af_array, arr: af_array, wts: af_array, dim: dim_t) -> c_int;
    fn af_var_weighted(out: *mut af_array, arr: af_array, wts: af_array, dim: dim_t) -> c_int;

    fn af_mean_all(real: *mut c_double, imag: *mut c_double, arr: af_array) -> c_int;
    fn af_median_all(real: *mut c_double, imag: *mut c_double, arr: af_array) -> c_int;

    fn af_mean_all_weighted(
        real: *mut c_double,
        imag: *mut c_double,
        arr: af_array,
        wts: af_array,
    ) -> c_int;
    fn af_var_all_weighted(
        real: *mut c_double,
        imag: *mut c_double,
        arr: af_array,
        wts: af_array,
    ) -> c_int;

    fn af_corrcoef(real: *mut c_double, imag: *mut c_double, X: af_array, Y: af_array) -> c_int;
    fn af_topk(
        vals: *mut af_array,
        idxs: *mut af_array,
        arr: af_array,
        k: c_int,
        dim: c_int,
        order: c_uint,
    ) -> c_int;

    fn af_meanvar(
        mean: *mut af_array,
        var: *mut af_array,
        input: af_array,
        weights: af_array,
        bias: c_uint,
        dim: dim_t,
    ) -> c_int;
    fn af_var_v2(out: *mut af_array, arr: af_array, bias_kind: c_uint, dim: dim_t) -> c_int;
    fn af_cov_v2(out: *mut af_array, X: af_array, Y: af_array, bias_kind: c_uint) -> c_int;
    fn af_stdev_v2(out: *mut af_array, arr: af_array, bias_kind: c_uint, dim: dim_t) -> c_int;
    fn af_var_all_v2(
        real: *mut c_double,
        imag: *mut c_double,
        arr: af_array,
        bias_kind: c_uint,
    ) -> c_int;
    fn af_stdev_all_v2(
        real: *mut c_double,
        imag: *mut c_double,
        arr: af_array,
        bias_kind: c_uint,
    ) -> c_int;
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
pub fn median<T>(input: &Array<T>, dim: i64) -> Array<T>
where
    T: HasAfEnum + MedianComputable,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_median(&mut temp as *mut af_array, input.get(), dim);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

macro_rules! stat_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => {
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
        pub fn $fn_name<T>(input: &Array<T>, dim: i64) -> Array<T::MeanOutType>
        where
            T: HasAfEnum,
            T::MeanOutType: HasAfEnum,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_fn(&mut temp as *mut af_array, input.get(), dim);
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    };
}

stat_func_def!("Mean along specified dimension", mean, af_mean);

macro_rules! stat_wtd_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => {
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
        pub fn $fn_name<T, W>(
            input: &Array<T>,
            weights: &Array<W>,
            dim: i64,
        ) -> Array<T::MeanOutType>
        where
            T: HasAfEnum,
            T::MeanOutType: HasAfEnum,
            W: HasAfEnum + RealFloating,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_fn(&mut temp as *mut af_array, input.get(), weights.get(), dim);
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    };
}

stat_wtd_func_def!(
    "Weighted mean along specified dimension",
    mean_weighted,
    af_mean_weighted
);
stat_wtd_func_def!(
    "Weight variance along specified dimension",
    var_weighted,
    af_var_weighted
);

/// Compute Variance along a specific dimension
///
/// # Parameters
///
/// - `arr` is the input Array
/// - `bias_kind` of type [VarianceBias][1] denotes the type of variane to be computed
/// - `dim` is the dimension along which the variance is extracted
///
/// # Return Values
///
/// Array with variance of input Array `arr` along dimension `dim`.
///
/// [1]: ./enum.VarianceBias.html
pub fn var_v2<T>(arr: &Array<T>, bias_kind: VarianceBias, dim: i64) -> Array<T::MeanOutType>
where
    T: HasAfEnum,
    T::MeanOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_var_v2(
            &mut temp as *mut af_array,
            arr.get(),
            bias_kind as c_uint,
            dim,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

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
#[deprecated(since = "3.8.0", note = "Please use var_v2 API")]
pub fn var<T>(arr: &Array<T>, isbiased: bool, dim: i64) -> Array<T::MeanOutType>
where
    T: HasAfEnum,
    T::MeanOutType: HasAfEnum,
{
    var_v2(
        arr,
        if isbiased {
            VarianceBias::SAMPLE
        } else {
            VarianceBias::POPULATION
        },
        dim,
    )
}

/// Compute covariance of two Arrays
///
/// # Parameters
///
/// - `x` is the first Array
/// - `y` is the second Array
/// - `bias_kind` of type [VarianceBias][1] denotes the type of variane to be computed
///
/// # Return Values
///
/// An Array with Covariance values
///
/// [1]: ./enum.VarianceBias.html
pub fn cov_v2<T>(x: &Array<T>, y: &Array<T>, bias_kind: VarianceBias) -> Array<T::MeanOutType>
where
    T: HasAfEnum + CovarianceComputable,
    T::MeanOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_cov_v2(
            &mut temp as *mut af_array,
            x.get(),
            y.get(),
            bias_kind as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
#[deprecated(since = "3.8.0", note = "Please use cov_v2 API")]
pub fn cov<T>(x: &Array<T>, y: &Array<T>, isbiased: bool) -> Array<T::MeanOutType>
where
    T: HasAfEnum + CovarianceComputable,
    T::MeanOutType: HasAfEnum,
{
    cov_v2(
        x,
        y,
        if isbiased {
            VarianceBias::SAMPLE
        } else {
            VarianceBias::POPULATION
        },
    )
}

/// Compute Variance of all elements
///
/// # Parameters
///
/// - `input` is the input Array
/// - `bias_kind` of type [VarianceBias][1] denotes the type of variane to be computed
///
/// # Return Values
///
/// A tuple of 64-bit floating point values that has the variance of `input` Array.
///
/// [1]: ./enum.VarianceBias.html
pub fn var_all_v2<T: HasAfEnum>(input: &Array<T>, bias_kind: VarianceBias) -> (f64, f64) {
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_var_all_v2(
            &mut real as *mut c_double,
            &mut imag as *mut c_double,
            input.get(),
            bias_kind as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    (real, imag)
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
#[deprecated(since = "3.8.0", note = "Please use var_all_v2 API")]
pub fn var_all<T: HasAfEnum>(input: &Array<T>, isbiased: bool) -> (f64, f64) {
    var_all_v2(
        input,
        if isbiased {
            VarianceBias::SAMPLE
        } else {
            VarianceBias::POPULATION
        },
    )
}

macro_rules! stat_all_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => {
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `input` is the input Array
        ///
        ///# Return Values
        ///
        /// A tuple of 64-bit floating point values with the stat values.
        pub fn $fn_name<T: HasAfEnum>(input: &Array<T>) -> (f64, f64) {
            let mut real: f64 = 0.0;
            let mut imag: f64 = 0.0;
            unsafe {
                let err_val = $ffi_fn(
                    &mut real as *mut c_double,
                    &mut imag as *mut c_double,
                    input.get(),
                );
                HANDLE_ERROR(AfError::from(err_val));
            }
            (real, imag)
        }
    };
}

stat_all_func_def!("Compute mean of all data", mean_all, af_mean_all);

/// Compute median of all data
///
///# Parameters
///
/// - `input` is the input Array
///
///# Return Values
///
/// A tuple of 64-bit floating point values with the median
pub fn median_all<T>(input: &Array<T>) -> (f64, f64)
where
    T: HasAfEnum + MedianComputable,
{
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_median_all(
            &mut real as *mut c_double,
            &mut imag as *mut c_double,
            input.get(),
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    (real, imag)
}

macro_rules! stat_wtd_all_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => {
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
        pub fn $fn_name<T, W>(input: &Array<T>, weights: &Array<W>) -> (f64, f64)
        where
            T: HasAfEnum,
            W: HasAfEnum + RealFloating,
        {
            let mut real: f64 = 0.0;
            let mut imag: f64 = 0.0;
            unsafe {
                let err_val = $ffi_fn(
                    &mut real as *mut c_double,
                    &mut imag as *mut c_double,
                    input.get(),
                    weights.get(),
                );
                HANDLE_ERROR(AfError::from(err_val));
            }
            (real, imag)
        }
    };
}

stat_wtd_all_func_def!(
    "Compute weighted mean of all data",
    mean_all_weighted,
    af_mean_all_weighted
);
stat_wtd_all_func_def!(
    "Compute weighted variance of all data",
    var_all_weighted,
    af_var_all_weighted
);

/// Compute correlation coefficient
///
/// # Parameters
///
/// - `x` is the first Array
/// - `y` isthe second Array
///
/// # Return Values
/// A tuple of 64-bit floating point values with the coefficients.
pub fn corrcoef<T>(x: &Array<T>, y: &Array<T>) -> (f64, f64)
where
    T: HasAfEnum + RealNumber,
{
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_corrcoef(
            &mut real as *mut c_double,
            &mut imag as *mut c_double,
            x.get(),
            y.get(),
        );
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
where
    T: HasAfEnum,
{
    unsafe {
        let mut t0: af_array = std::ptr::null_mut();
        let mut t1: af_array = std::ptr::null_mut();
        let err_val = af_topk(
            &mut t0 as *mut af_array,
            &mut t1 as *mut af_array,
            input.get(),
            k as c_int,
            dim as c_int,
            order as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (t0.into(), t1.into())
    }
}

/// Calculate mean and variance in single API call
///
///# Parameters
///
/// - `input` is the input Array
/// - `weights` Array has the weights to be used during the stat computation
/// - `bias` is type of bias used for variance calculation
/// - `dim` is dimension along which the current stat has to be computed
///
///# Return Values
///
/// A tuple of Arrays, whose size is equal to input except along the dimension which
/// the stat operation is performed. Array size along `dim` will be reduced to one.
///
/// - First Array contains mean values
/// - Second Array contains variance values
pub fn meanvar<T, W>(
    input: &Array<T>,
    weights: &Array<W>,
    bias: VarianceBias,
    dim: i64,
) -> (Array<T::MeanOutType>, Array<T::MeanOutType>)
where
    T: HasAfEnum,
    T::MeanOutType: HasAfEnum,
    W: HasAfEnum + RealFloating,
{
    unsafe {
        let mut mean: af_array = std::ptr::null_mut();
        let mut var: af_array = std::ptr::null_mut();
        let err_val = af_meanvar(
            &mut mean as *mut af_array,
            &mut var as *mut af_array,
            input.get(),
            weights.get(),
            bias as c_uint,
            dim,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (mean.into(), var.into())
    }
}

/// Standard deviation along given axis
///
///# Parameters
///
/// - `input` is the input Array
/// - `bias_kind` of type [VarianceBias][1] denotes the type of variane to be computed
/// - `dim` is dimension along which the current stat has to be computed
///
///# Return Values
///
/// An Array whose size is equal to input except along the dimension which
/// the stat operation is performed. Array size along `dim` will be reduced to one.
///
/// [1]: ./enum.VarianceBias.html
pub fn stdev_v2<T>(input: &Array<T>, bias_kind: VarianceBias, dim: i64) -> Array<T::MeanOutType>
where
    T: HasAfEnum,
    T::MeanOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_stdev_v2(
            &mut temp as *mut af_array,
            input.get(),
            bias_kind as c_uint,
            dim,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Standard deviation along specified axis
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
#[deprecated(since = "3.8.0", note = "Please use stdev_v2 API")]
pub fn stdev<T>(input: &Array<T>, dim: i64) -> Array<T::MeanOutType>
where
    T: HasAfEnum,
    T::MeanOutType: HasAfEnum,
{
    stdev_v2(input, VarianceBias::POPULATION, dim)
}

/// Compute standard deviation of all data
///
///# Parameters
///
/// - `input` is the input Array
/// - `bias_kind` of type [VarianceBias][1] denotes the type of variane to be computed
///
///# Return Values
///
/// A tuple of 64-bit floating point values with the stat values.
///
/// [1]: ./enum.VarianceBias.html
pub fn stdev_all_v2<T: HasAfEnum>(input: &Array<T>, bias_kind: VarianceBias) -> (f64, f64) {
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_stdev_all_v2(
            &mut real as *mut c_double,
            &mut imag as *mut c_double,
            input.get(),
            bias_kind as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    (real, imag)
}

/// Compute standard deviation of all data
///
///# Parameters
///
/// - `input` is the input Array
///
///# Return Values
///
/// A tuple of 64-bit floating point values with the stat values.
pub fn stdev_all<T: HasAfEnum>(input: &Array<T>) -> (f64, f64) {
    stdev_all_v2(input, VarianceBias::POPULATION)
}
