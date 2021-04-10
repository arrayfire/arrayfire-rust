use super::core::{
    af_array, dim_t, AfError, Array, ComplexFloating, ConvDomain, ConvMode, FloatingPoint,
    HasAfEnum, InterpType, RealFloating, HANDLE_ERROR,
};

use libc::{c_double, c_float, c_int, c_uint, size_t};
use num::Complex;

extern "C" {
    fn af_approx1(
        out: *mut af_array,
        inp: af_array,
        pos: af_array,
        method: c_uint,
        off_grid: c_float,
    ) -> c_int;

    fn af_approx1_v2(
        out: *mut af_array,
        inp: af_array,
        pos: af_array,
        method: c_uint,
        off_grid: c_float,
    ) -> c_int;

    fn af_approx1_uniform(
        out: *mut af_array,
        inp: af_array,
        pos: af_array,
        interp_dim: c_int,
        idx_start: c_double,
        idx_step: c_double,
        method: c_uint,
        off_grid: c_float,
    ) -> c_int;

    fn af_approx1_uniform_v2(
        out: *mut af_array,
        inp: af_array,
        pos: af_array,
        interp_dim: c_int,
        idx_start: c_double,
        idx_step: c_double,
        method: c_uint,
        off_grid: c_float,
    ) -> c_int;

    fn af_approx2(
        out: *mut af_array,
        inp: af_array,
        pos0: af_array,
        pos1: af_array,
        method: c_uint,
        off_grid: c_float,
    ) -> c_int;

    fn af_approx2_v2(
        out: *mut af_array,
        inp: af_array,
        pos0: af_array,
        pos1: af_array,
        method: c_uint,
        off_grid: c_float,
    ) -> c_int;

    fn af_approx2_uniform(
        out: *mut af_array,
        inp: af_array,
        pos0: af_array,
        interp_dim0: c_int,
        idx_start_dim0: c_double,
        idx_step_dim0: c_double,
        pos1: af_array,
        interp_dim1: c_int,
        idx_start_dim1: c_double,
        idx_step_dim1: c_double,
        method: c_uint,
        off_grid: c_float,
    ) -> c_int;

    fn af_approx2_uniform_v2(
        out: *mut af_array,
        inp: af_array,
        pos0: af_array,
        interp_dim0: c_int,
        idx_start_dim0: c_double,
        idx_step_dim0: c_double,
        pos1: af_array,
        interp_dim1: c_int,
        idx_start_dim1: c_double,
        idx_step_dim1: c_double,
        method: c_uint,
        off_grid: c_float,
    ) -> c_int;

    fn af_set_fft_plan_cache_size(cache_size: size_t) -> c_int;

    fn af_fft(out: *mut af_array, arr: af_array, nfac: c_double, odim0: dim_t) -> c_int;

    fn af_fft2(
        out: *mut af_array,
        arr: af_array,
        nfac: c_double,
        odim0: dim_t,
        odim1: dim_t,
    ) -> c_int;

    fn af_fft3(
        out: *mut af_array,
        arr: af_array,
        nfac: c_double,
        odim0: dim_t,
        odim1: dim_t,
        odim2: dim_t,
    ) -> c_int;

    fn af_ifft(out: *mut af_array, arr: af_array, nfac: c_double, odim0: dim_t) -> c_int;

    fn af_ifft2(
        out: *mut af_array,
        arr: af_array,
        nfac: c_double,
        odim0: dim_t,
        odim1: dim_t,
    ) -> c_int;

    fn af_ifft3(
        out: *mut af_array,
        arr: af_array,
        nfac: c_double,
        odim0: dim_t,
        odim1: dim_t,
        odim2: dim_t,
    ) -> c_int;

    fn af_fft_inplace(arr: *mut af_array, nfac: c_double) -> c_int;
    fn af_fft2_inplace(arr: *mut af_array, nfac: c_double) -> c_int;
    fn af_fft3_inplace(arr: *mut af_array, nfac: c_double) -> c_int;
    fn af_ifft_inplace(arr: *mut af_array, nfac: c_double) -> c_int;
    fn af_ifft2_inplace(arr: *mut af_array, nfac: c_double) -> c_int;
    fn af_ifft3_inplace(arr: *mut af_array, nfac: c_double) -> c_int;

    fn af_fft_r2c(out: *mut af_array, arr: af_array, nfac: c_double, pad0: dim_t) -> c_int;
    fn af_fft2_r2c(
        out: *mut af_array,
        arr: af_array,
        nfac: c_double,
        pad0: dim_t,
        pad1: dim_t,
    ) -> c_int;
    fn af_fft3_r2c(
        out: *mut af_array,
        arr: af_array,
        nfac: c_double,
        pad0: dim_t,
        pad1: dim_t,
        pad2: dim_t,
    ) -> c_int;

    fn af_fft_c2r(out: *mut af_array, input: af_array, nfac: c_double, is_odd: bool) -> c_int;
    fn af_fft2_c2r(out: *mut af_array, input: af_array, nfac: c_double, is_odd: bool) -> c_int;
    fn af_fft3_c2r(out: *mut af_array, input: af_array, nfac: c_double, is_odd: bool) -> c_int;

    fn af_convolve1(out: *mut af_array, s: af_array, f: af_array, m: c_uint, d: c_uint) -> c_int;
    fn af_convolve2(out: *mut af_array, s: af_array, f: af_array, m: c_uint, d: c_uint) -> c_int;
    fn af_convolve3(out: *mut af_array, s: af_array, f: af_array, m: c_uint, d: c_uint) -> c_int;
    fn af_convolve2_sep(
        o: *mut af_array,
        c: af_array,
        r: af_array,
        s: af_array,
        m: c_uint,
    ) -> c_int;
    fn af_fft_convolve1(out: *mut af_array, s: af_array, f: af_array, m: c_uint) -> c_int;
    fn af_fft_convolve2(out: *mut af_array, s: af_array, f: af_array, m: c_uint) -> c_int;
    fn af_fft_convolve3(out: *mut af_array, s: af_array, f: af_array, m: c_uint) -> c_int;
    fn af_fir(out: *mut af_array, b: af_array, x: af_array) -> c_int;
    fn af_iir(out: *mut af_array, b: af_array, a: af_array, x: af_array) -> c_int;
}

/// Perform signal interpolation for 1d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `pos` Array contains the interpolation locations
/// - `method` indicates the type of interpolation method that be used. It is of type enum
/// [InterpType](./enum.InterpType.html)
/// - `off_grid` is the value that will set in the output Array when certain index is out of bounds
///
/// # Return Values
///
/// An Array with interpolated values
pub fn approx1<T, P>(
    input: &Array<T>,
    pos: &Array<P>,
    method: InterpType,
    off_grid: f32,
) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
    P: HasAfEnum + RealFloating,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_approx1(
            &mut temp as *mut af_array,
            input.get(),
            pos.get(),
            method as c_uint,
            off_grid,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Same as [approx1](./fn.approx1.html) but uses existing Array as output
pub fn approx1_v2<T, P>(
    output: &mut Array<T>,
    input: &Array<T>,
    pos: &Array<P>,
    method: InterpType,
    off_grid: f32,
) where
    T: HasAfEnum + FloatingPoint,
    P: HasAfEnum + RealFloating,
{
    unsafe {
        let err_val = af_approx1_v2(
            output.get() as *mut af_array,
            input.get(),
            pos.get(),
            method as c_uint,
            off_grid,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Perform signal interpolation for 1d signals along specified dimension
///
/// # Parameters
///
/// - `input` is the input Array
/// - `pos` Array contains the interpolation locations
/// - `interp_dim` is the dimension along which interpolation is performed
/// - `start` is the first index along `interp_dim`
/// - `step` is the uniform spacing value between subsequent indices along `interp_dim`
/// - `method` indicates the type of interpolation method that be used. It is of type enum
/// [InterpType](./enum.InterpType.html)
/// - `off_grid` is the value that will set in the output Array when certain index is out of bounds
///
/// # Return Values
///
/// An Array with interpolated values
pub fn approx1_uniform<T, P>(
    input: &Array<T>,
    pos: &Array<P>,
    interp_dim: i32,
    start: f64,
    step: f64,
    method: InterpType,
    off_grid: f32,
) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
    P: HasAfEnum + RealFloating,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_approx1_uniform(
            &mut temp as *mut af_array,
            input.get(),
            pos.get(),
            interp_dim,
            start,
            step,
            method as c_uint,
            off_grid,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Same as [approx1_uniform](./fn.approx1_uniform.html) but uses existing Array as output
#[allow(clippy::too_many_arguments)]
pub fn approx1_uniform_v2<T, P>(
    output: &mut Array<T>,
    input: &Array<T>,
    pos: &Array<P>,
    interp_dim: i32,
    start: f64,
    step: f64,
    method: InterpType,
    off_grid: f32,
) where
    T: HasAfEnum + FloatingPoint,
    P: HasAfEnum + RealFloating,
{
    unsafe {
        let err_val = af_approx1_uniform_v2(
            output.get() as *mut af_array,
            input.get(),
            pos.get(),
            interp_dim,
            start,
            step,
            method as c_uint,
            off_grid,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Perform signal interpolation for 2d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `pos0` Array contains the interpolation locations for first dimension
/// - `pos1` Array contains the interpolation locations for second dimension
/// - `method` indicates the type of interpolation method that be used. It is of type enum
/// [InterpType](./enum.InterpType.html)
/// - `off_grid` is the value that will set in the output Array when certain index is out of bounds
///
/// # Return Values
///
/// An Array with interpolated values
pub fn approx2<T, P>(
    input: &Array<T>,
    pos0: &Array<P>,
    pos1: &Array<P>,
    method: InterpType,
    off_grid: f32,
) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
    P: HasAfEnum + RealFloating,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_approx2(
            &mut temp as *mut af_array,
            input.get(),
            pos0.get(),
            pos1.get(),
            method as c_uint,
            off_grid,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Same as [approx2](./fn.approx2.html) but uses existing Array as output
pub fn approx2_v2<T, P>(
    output: &mut Array<T>,
    input: &Array<T>,
    pos0: &Array<P>,
    pos1: &Array<P>,
    method: InterpType,
    off_grid: f32,
) where
    T: HasAfEnum + FloatingPoint,
    P: HasAfEnum + RealFloating,
{
    unsafe {
        let err_val = af_approx2_v2(
            output.get() as *mut af_array,
            input.get(),
            pos0.get(),
            pos1.get(),
            method as c_uint,
            off_grid,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Perform signal interpolation for 2d signals along a specified dimension
///
/// # Parameters
///
/// - `input` is the input Array
/// - `pos0` Array contains the interpolation locations for first dimension
/// - `interp_dim0` is the dimension along which interpolation is performed
/// - `start0` is the first index along `interp_dim0`
/// - `step0` is the uniform spacing value between subsequent indices along `interp_dim0`
/// - `pos1` Array contains the interpolation locations for second dimension
/// - `interp_dim0` is the dimension along which interpolation is performed
/// - `start0` is the first index along `interp_dim1`
/// - `step0` is the uniform spacing value between subsequent indices along `interp_dim1`
/// - `method` indicates the type of interpolation method that be used. It is of type enum
/// [InterpType](./enum.InterpType.html)
/// - `off_grid` is the value that will set in the output Array when certain index is out of bounds
///
/// # Return Values
///
/// An Array with interpolated values
#[allow(clippy::too_many_arguments)]
pub fn approx2_uniform<T, P>(
    input: &Array<T>,
    pos0: &Array<P>,
    interp_dim0: i32,
    start0: f64,
    step0: f64,
    pos1: &Array<P>,
    interp_dim1: i32,
    start1: f64,
    step1: f64,
    method: InterpType,
    off_grid: f32,
) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
    P: HasAfEnum + RealFloating,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_approx2_uniform(
            &mut temp as *mut af_array,
            input.get(),
            pos0.get(),
            interp_dim0,
            start0,
            step0,
            pos1.get(),
            interp_dim1,
            start1,
            step1,
            method as c_uint,
            off_grid,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Same as [approx2_uniform](./fn.approx2_uniform.html) but uses existing Array as output
#[allow(clippy::too_many_arguments)]
pub fn approx2_uniform_v2<T, P>(
    output: &mut Array<T>,
    input: &Array<T>,
    pos0: &Array<P>,
    interp_dim0: i32,
    start0: f64,
    step0: f64,
    pos1: &Array<P>,
    interp_dim1: i32,
    start1: f64,
    step1: f64,
    method: InterpType,
    off_grid: f32,
) where
    T: HasAfEnum + FloatingPoint,
    P: HasAfEnum + RealFloating,
{
    unsafe {
        let err_val = af_approx2_uniform_v2(
            output.get() as *mut af_array,
            input.get(),
            pos0.get(),
            interp_dim0,
            start0,
            step0,
            pos1.get(),
            interp_dim1,
            start1,
            step1,
            method as c_uint,
            off_grid,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Set fft plan cache size
///
/// Though this is a low overhead function, it is advised not to change
/// the fft plan cache size a mid program execution unless that is what
/// you intend to do.
pub fn set_fft_plan_cache_size(cache_size: usize) {
    unsafe {
        let err_val = af_set_fft_plan_cache_size(cache_size as size_t);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Fast fourier transform for 1d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signals - used for either truncating or padding the input
/// signals
///
/// # Return Values
///
/// Transformed Array
pub fn fft<T>(input: &Array<T>, norm_factor: f64, odim0: i64) -> Array<T::ComplexOutType>
where
    T: HasAfEnum + FloatingPoint,
    <T as HasAfEnum>::ComplexOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_fft(&mut temp as *mut af_array, input.get(), norm_factor, odim0);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Fast fourier transform for 2d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signal first dimension - used for either truncating or padding the input
/// - `odim1` is the length of output signal second dimension - used for either truncating or padding the input
///
/// # Return Values
///
/// Transformed Array
pub fn fft2<T>(
    input: &Array<T>,
    norm_factor: f64,
    odim0: i64,
    odim1: i64,
) -> Array<T::ComplexOutType>
where
    T: HasAfEnum + FloatingPoint,
    <T as HasAfEnum>::ComplexOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_fft2(
            &mut temp as *mut af_array,
            input.get(),
            norm_factor,
            odim0,
            odim1,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Fast fourier transform for 3d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signal first dimension - used for either truncating or padding the input
/// - `odim1` is the length of output signal second dimension - used for either truncating or padding the input
/// - `odim2` is the length of output signal third dimension - used for either truncating or padding the input
///
/// # Return Values
///
/// Transformed Array
pub fn fft3<T>(
    input: &Array<T>,
    norm_factor: f64,
    odim0: i64,
    odim1: i64,
    odim2: i64,
) -> Array<T::ComplexOutType>
where
    T: HasAfEnum + FloatingPoint,
    <T as HasAfEnum>::ComplexOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_fft3(
            &mut temp as *mut af_array,
            input.get(),
            norm_factor,
            odim0,
            odim1,
            odim2,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Inverse fast fourier transform for 1d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signals - used for either truncating or padding the input
/// signals
///
/// # Return Values
///
/// Transformed Array
pub fn ifft<T>(input: &Array<T>, norm_factor: f64, odim0: i64) -> Array<T::ComplexOutType>
where
    T: HasAfEnum + FloatingPoint,
    <T as HasAfEnum>::ComplexOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_ifft(&mut temp as *mut af_array, input.get(), norm_factor, odim0);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Inverse fast fourier transform for 2d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signal first dimension - used for either truncating or padding the input
/// - `odim1` is the length of output signal second dimension - used for either truncating or padding the input
///
/// # Return Values
///
/// Transformed Array
pub fn ifft2<T>(
    input: &Array<T>,
    norm_factor: f64,
    odim0: i64,
    odim1: i64,
) -> Array<T::ComplexOutType>
where
    T: HasAfEnum + FloatingPoint,
    <T as HasAfEnum>::ComplexOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_ifft2(
            &mut temp as *mut af_array,
            input.get(),
            norm_factor,
            odim0,
            odim1,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Inverse fast fourier transform for 3d signals
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor with which the input is scaled before the
/// transformation is applied
/// - `odim0` is the length of output signal first dimension - used for either truncating or padding the input
/// - `odim1` is the length of output signal second dimension - used for either truncating or padding the input
/// - `odim2` is the length of output signal third dimension - used for either truncating or padding the input
///
/// # Return Values
///
/// Transformed Array
pub fn ifft3<T>(
    input: &Array<T>,
    norm_factor: f64,
    odim0: i64,
    odim1: i64,
    odim2: i64,
) -> Array<T::ComplexOutType>
where
    T: HasAfEnum + FloatingPoint,
    <T as HasAfEnum>::ComplexOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_ifft3(
            &mut temp as *mut af_array,
            input.get(),
            norm_factor,
            odim0,
            odim1,
            odim2,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

macro_rules! conv_func_def {
    ($doc_str: expr, $fn_name:ident, $ffi_name: ident) => {
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `signal` is the input signal
        /// - `filter` is the signal that shall be flipped for convolution operation
        /// - `mode` indicates if the convolution should be expanded or not(where output size
        /// equals input). It takes a value of type [ConvMode](./enum.ConvMode.html)
        /// - `domain` indicates if the convolution should be performed in frequencey or spatial
        /// domain. It takes a value of type [ConvDomain](./enum.ConvDomain.html)
        ///
        ///# Return Values
        ///
        /// Convolved Array
        pub fn $fn_name<T, F>(
            signal: &Array<T>,
            filter: &Array<F>,
            mode: ConvMode,
            domain: ConvDomain,
        ) -> Array<T>
        where
            T: HasAfEnum,
            F: HasAfEnum,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(
                    &mut temp as *mut af_array,
                    signal.get(),
                    filter.get(),
                    mode as c_uint,
                    domain as c_uint,
                );
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    };
}

conv_func_def!("1d convolution", convolve1, af_convolve1);
conv_func_def!("2d convolution", convolve2, af_convolve2);
conv_func_def!("3d convolution", convolve3, af_convolve3);

/// Separable convolution for 2d signals
///
/// # Parameters
///
/// - `cfilt` is the filter to be applied along coloumns
/// - `rfilt` is the filter to be applied along rows
/// - `signal` is the input signal
/// - `mode` indicates if the convolution should be expanded or not(where output size equals input)
///
/// # Return Values
///
/// The convolved Array
pub fn convolve2_sep<T, F>(
    cfilt: &Array<F>,
    rfilt: &Array<F>,
    signal: &Array<T>,
    mode: ConvMode,
) -> Array<T>
where
    T: HasAfEnum,
    F: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_convolve2_sep(
            &mut temp as *mut af_array,
            cfilt.get(),
            rfilt.get(),
            signal.get(),
            mode as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

macro_rules! fft_conv_func_def {
    ($doc_str: expr, $fn_name:ident, $ffi_name: ident) => {
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `signal` is the input signal
        /// - `filter` is the signal that shall be used for convolution operation
        /// - `mode` indicates if the convolution should be expanded or not(where output size
        /// equals input). It takes values of type [ConvMode](./enum.ConvMode.html)
        ///
        ///# Return Values
        ///
        /// Convolved Array
        pub fn $fn_name<T, F>(signal: &Array<T>, filter: &Array<F>, mode: ConvMode) -> Array<T>
        where
            T: HasAfEnum,
            F: HasAfEnum,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(
                    &mut temp as *mut af_array,
                    signal.get(),
                    filter.get(),
                    mode as c_uint,
                );
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    };
}

fft_conv_func_def!(
    "1d convolution using fast-fourier transform",
    fft_convolve1,
    af_fft_convolve1
);
fft_conv_func_def!(
    "2d convolution using fast-fourier transform",
    fft_convolve2,
    af_fft_convolve2
);
fft_conv_func_def!(
    "3d convolution using fast-fourier transform",
    fft_convolve3,
    af_fft_convolve3
);

/// Finite impulse filter
///
/// # Parameters
///
/// - `b` is the Array containing the coefficients of the filter
/// - `x` is the input signal to filter
///
/// # Return Values
///
/// Filtered Array
pub fn fir<B, X>(b: &Array<B>, x: &Array<X>) -> Array<X>
where
    B: HasAfEnum,
    X: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_fir(&mut temp as *mut af_array, b.get(), x.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Infinite impulse response filter
///
/// # Parameters
///
/// - `b` is the Array containing the feedforward coefficients
/// - `a` is the Array containing the feedback coefficients
/// - `x` is the input signal to filter
///
/// # Return Values
///
/// Filtered Array
pub fn iir<T: HasAfEnum>(b: &Array<T>, a: &Array<T>, x: &Array<T>) -> Array<T> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_iir(&mut temp as *mut af_array, b.get(), a.get(), x.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// In place 1d dimensional Fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn fft_inplace<T>(input: &mut Array<T>, norm_factor: f64)
where
    T: HasAfEnum + ComplexFloating,
{
    unsafe {
        let err_val = af_fft_inplace(input.get() as *mut af_array, norm_factor);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// In place 2d dimensional Fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn fft2_inplace<T>(input: &mut Array<T>, norm_factor: f64)
where
    T: HasAfEnum + ComplexFloating,
{
    unsafe {
        let err_val = af_fft2_inplace(input.get() as *mut af_array, norm_factor);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// In place 3d dimensional Fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn fft3_inplace<T>(input: &mut Array<T>, norm_factor: f64)
where
    T: HasAfEnum + ComplexFloating,
{
    unsafe {
        let err_val = af_fft3_inplace(input.get() as *mut af_array, norm_factor);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// In place 1d dimensional inverse fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn ifft_inplace<T>(input: &mut Array<T>, norm_factor: f64)
where
    T: HasAfEnum + ComplexFloating,
{
    unsafe {
        let err_val = af_ifft_inplace(input.get() as *mut af_array, norm_factor);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// In place 2d dimensional inverse fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn ifft2_inplace<T>(input: &mut Array<T>, norm_factor: f64)
where
    T: HasAfEnum + ComplexFloating,
{
    unsafe {
        let err_val = af_ifft2_inplace(input.get() as *mut af_array, norm_factor);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// In place 3d dimensional inverse fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn ifft3_inplace<T>(input: &mut Array<T>, norm_factor: f64)
where
    T: HasAfEnum + ComplexFloating,
{
    unsafe {
        let err_val = af_ifft3_inplace(input.get() as *mut af_array, norm_factor);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// 1d Real to Complex fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `pad0` is the padding along 0th dimension of Array
///
/// # Return Values
///
/// Complex Array
pub fn fft_r2c<T>(input: &Array<T>, norm_factor: f64, pad0: i64) -> Array<Complex<T>>
where
    T: HasAfEnum + RealFloating,
    Complex<T>: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_fft_r2c(&mut temp as *mut af_array, input.get(), norm_factor, pad0);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// 2d Real to Complex fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `pad0` is the padding along 0th dimension of Array
/// - `pad1` is the padding along 1st dimension of Array
///
/// # Return Values
///
/// Complex Array
pub fn fft2_r2c<T>(input: &Array<T>, norm_factor: f64, pad0: i64, pad1: i64) -> Array<Complex<T>>
where
    T: HasAfEnum + RealFloating,
    Complex<T>: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_fft2_r2c(
            &mut temp as *mut af_array,
            input.get(),
            norm_factor,
            pad0,
            pad1,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// 3d Real to Complex fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `pad0` is the padding along 0th dimension of Array
/// - `pad1` is the padding along 1st dimension of Array
/// - `pad2` is the padding along 2nd dimension of Array
///
/// # Return Values
///
/// Complex Array
pub fn fft3_r2c<T>(
    input: &Array<T>,
    norm_factor: f64,
    pad0: i64,
    pad1: i64,
    pad2: i64,
) -> Array<Complex<T>>
where
    T: HasAfEnum + RealFloating,
    Complex<T>: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_fft3_r2c(
            &mut temp as *mut af_array,
            input.get(),
            norm_factor,
            pad0,
            pad1,
            pad2,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// 1d Complex to Real fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `is_odd` signifies if the output should be even or odd size
///
/// # Return Values
///
/// Complex Array
pub fn fft_c2r<T>(input: &Array<T>, norm_factor: f64, is_odd: bool) -> Array<T::BaseType>
where
    T: HasAfEnum + ComplexFloating,
    <T as HasAfEnum>::BaseType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_fft_c2r(&mut temp as *mut af_array, input.get(), norm_factor, is_odd);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// 2d Complex to Real fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `is_odd` signifies if the output should be even or odd size
///
/// # Return Values
///
/// Complex Array
pub fn fft2_c2r<T>(input: &Array<T>, norm_factor: f64, is_odd: bool) -> Array<T::BaseType>
where
    T: HasAfEnum + ComplexFloating,
    <T as HasAfEnum>::BaseType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_fft2_c2r(&mut temp as *mut af_array, input.get(), norm_factor, is_odd);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// 3d Complex to Real fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor to be applied before fft is applied
/// - `is_odd` signifies if the output should be even or odd size
///
/// # Return Values
///
/// Complex Array
pub fn fft3_c2r<T>(input: &Array<T>, norm_factor: f64, is_odd: bool) -> Array<T::BaseType>
where
    T: HasAfEnum + ComplexFloating,
    <T as HasAfEnum>::BaseType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_fft3_c2r(&mut temp as *mut af_array, input.get(), norm_factor, is_odd);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}
