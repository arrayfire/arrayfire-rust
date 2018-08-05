extern crate libc;
extern crate num;

use array::Array;
use defines::{AfError, ConvDomain, ConvMode, InterpType};
use error::HANDLE_ERROR;
use self::num::Complex;
use self::libc::{uint8_t, c_int, c_float, c_double, c_longlong, size_t};
use util::{ComplexFloating, FloatingPoint, HasAfEnum, RealFloating};
use util::{AfArray, MutAfArray};

#[allow(dead_code)]
extern {
    fn af_approx1(out: MutAfArray, inp: AfArray, pos: AfArray,
                  method: c_int, off_grid: c_float) -> c_int;

    fn af_approx2(out: MutAfArray, inp: AfArray, pos0: AfArray, pos1: AfArray,
                  method: c_int, off_grid: c_float) -> c_int;

    fn af_set_fft_plan_cache_size(cache_size: size_t) -> c_int;

    fn af_fft(out: MutAfArray, arr: AfArray,
              nfac: c_double, odim0: c_longlong) -> c_int;

    fn af_fft2(out: MutAfArray, arr: AfArray, nfac: c_double,
               odim0: c_longlong, odim1: c_longlong) -> c_int;

    fn af_fft3(out: MutAfArray, arr: AfArray, nfac: c_double,
               odim0: c_longlong, odim1: c_longlong, odim2: c_longlong) -> c_int;

    fn af_ifft(out: MutAfArray, arr: AfArray,
               nfac: c_double, odim0: c_longlong) -> c_int;

    fn af_ifft2(out: MutAfArray, arr: AfArray, nfac: c_double,
                odim0: c_longlong, odim1: c_longlong) -> c_int;

    fn af_ifft3(out: MutAfArray, arr: AfArray, nfac: c_double,
                odim0: c_longlong, odim1: c_longlong, odim2: c_longlong) -> c_int;

    fn af_fft_inplace(arr: AfArray, nfac: c_double) -> c_int;
    fn af_fft2_inplace(arr: AfArray, nfac: c_double) -> c_int;
    fn af_fft3_inplace(arr: AfArray, nfac: c_double) -> c_int;
    fn af_ifft_inplace(arr: AfArray, nfac: c_double) -> c_int;
    fn af_ifft2_inplace(arr: AfArray, nfac: c_double) -> c_int;
    fn af_ifft3_inplace(arr: AfArray, nfac: c_double) -> c_int;

    fn af_fft_r2c(out: MutAfArray, arr: AfArray,
                  nfac: c_double, pad0: c_longlong) -> c_int;
    fn af_fft2_r2c(out: MutAfArray, arr: AfArray,
                   nfac: c_double, pad0: c_longlong, pad1: c_longlong) -> c_int;
    fn af_fft3_r2c(out: MutAfArray, arr: AfArray,
                   nfac: c_double, pad0: c_longlong, pad1: c_longlong, pad2: c_longlong) -> c_int;

    fn af_fft_c2r(out: MutAfArray, input: AfArray, nfac: c_double, is_odd: c_int) -> c_int;
    fn af_fft2_c2r(out: MutAfArray, input: AfArray, nfac: c_double, is_odd: c_int) -> c_int;
    fn af_fft3_c2r(out: MutAfArray, input: AfArray, nfac: c_double, is_odd: c_int) -> c_int;

    fn af_convolve1(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t, d: uint8_t) -> c_int;
    fn af_convolve2(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t, d: uint8_t) -> c_int;
    fn af_convolve3(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t, d: uint8_t) -> c_int;
    fn af_convolve2_sep(o: MutAfArray, c: AfArray, r: AfArray, s: AfArray, m: uint8_t) -> c_int;
    fn af_fft_convolve1(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t) -> c_int;
    fn af_fft_convolve2(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t) -> c_int;
    fn af_fft_convolve3(out: MutAfArray, s: AfArray, f: AfArray, m: uint8_t) -> c_int;
    fn af_fir(out: MutAfArray, b: AfArray, x: AfArray) -> c_int;
    fn af_iir(out: MutAfArray, b: AfArray, a: AfArray, x: AfArray) -> c_int;
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
#[allow(unused_mut)]
pub fn approx1<T, P>(input: &Array<T>, pos: &Array<P>,
                     method: InterpType, off_grid: f32) -> Array<T>
    where T: HasAfEnum + FloatingPoint,
          P: HasAfEnum + RealFloating
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_approx1(&mut temp as MutAfArray, input.get() as AfArray,
                                 pos.get() as AfArray, method as c_int, off_grid as c_float);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

#[allow(unused_mut)]
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
pub fn approx2<T, P>(input: &Array<T>, pos0: &Array<P>, pos1: &Array<P>,
                     method: InterpType, off_grid: f32) -> Array<T>
    where T: HasAfEnum + FloatingPoint,
          P: HasAfEnum + RealFloating
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_approx2(&mut temp as MutAfArray, input.get() as AfArray,
                                 pos0.get() as AfArray, pos1.get() as AfArray,
                                 method as c_int, off_grid as c_float);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
#[allow(unused_mut)]
pub fn fft<T>(input: &Array<T>, norm_factor: f64,
              odim0: i64) -> Array< T::ComplexOutType >
    where T: HasAfEnum + FloatingPoint,
          <T as HasAfEnum>::ComplexOutType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_fft(&mut temp as MutAfArray,
                             input.get() as AfArray, norm_factor as c_double,
                             odim0 as c_longlong);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
#[allow(unused_mut)]
pub fn fft2<T>(input: &Array<T>, norm_factor: f64,
               odim0: i64, odim1: i64) -> Array< T::ComplexOutType >
    where T: HasAfEnum + FloatingPoint,
          <T as HasAfEnum>::ComplexOutType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_fft2(&mut temp as MutAfArray,
                              input.get() as AfArray, norm_factor as c_double,
                              odim0 as c_longlong, odim1 as c_longlong);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
#[allow(unused_mut)]
pub fn fft3<T>(input: &Array<T>, norm_factor: f64,
               odim0: i64, odim1: i64, odim2: i64) -> Array< T::ComplexOutType >
    where T: HasAfEnum + FloatingPoint,
          <T as HasAfEnum>::ComplexOutType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_fft3(&mut temp as MutAfArray,
                              input.get() as AfArray, norm_factor as c_double,
                              odim0 as c_longlong, odim1 as c_longlong, odim2 as c_longlong);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
#[allow(unused_mut)]
pub fn ifft<T>(input: &Array<T>, norm_factor: f64, odim0: i64) -> Array< T::ComplexOutType >
    where T: HasAfEnum + FloatingPoint,
          <T as HasAfEnum>::ComplexOutType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_ifft(&mut temp as MutAfArray,
                              input.get() as AfArray, norm_factor as c_double,
                              odim0 as c_longlong);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
#[allow(unused_mut)]
pub fn ifft2<T>(input: &Array<T>, norm_factor: f64,
                odim0: i64, odim1: i64) -> Array< T::ComplexOutType >
    where T: HasAfEnum + FloatingPoint,
          <T as HasAfEnum>::ComplexOutType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_ifft2(&mut temp as MutAfArray,
                               input.get() as AfArray, norm_factor as c_double,
                               odim0 as c_longlong, odim1 as c_longlong);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
#[allow(unused_mut)]
pub fn ifft3<T>(input: &Array<T>, norm_factor: f64,
                odim0: i64, odim1: i64, odim2: i64) -> Array< T::ComplexOutType >
    where T: HasAfEnum + FloatingPoint,
          <T as HasAfEnum>::ComplexOutType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_ifft3(&mut temp as MutAfArray,
                               input.get() as AfArray, norm_factor as c_double,
                               odim0 as c_longlong, odim1 as c_longlong, odim2 as c_longlong);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

macro_rules! conv_func_def {
    ($doc_str: expr, $fn_name:ident, $ffi_name: ident) => (
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
        #[allow(unused_mut)]
        pub fn $fn_name<T, F>(signal: &Array<T>, filter: &Array<F>,
                              mode: ConvMode, domain: ConvDomain) -> Array<T>
            where T: HasAfEnum,
                  F: HasAfEnum
        {
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                        signal.get() as AfArray, filter.get() as AfArray,
                                        mode as uint8_t, domain as uint8_t);
                HANDLE_ERROR(AfError::from(err_val));
            }
            temp.into()
        }
    )
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
#[allow(unused_mut)]
pub fn convolve2_sep<T, F>(cfilt: &Array<F>, rfilt: &Array<F>, signal: &Array<T>,
                           mode: ConvMode) -> Array<T>
    where T: HasAfEnum,
          F: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_convolve2_sep(&mut temp as MutAfArray,
                                       cfilt.get() as AfArray, rfilt.get() as AfArray,
                                       signal.get() as AfArray, mode as uint8_t);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

macro_rules! fft_conv_func_def {
    ($doc_str: expr, $fn_name:ident, $ffi_name: ident) => (
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
        #[allow(unused_mut)]
        pub fn $fn_name<T, F>(signal: &Array<T>, filter: &Array<F>,
                              mode: ConvMode) -> Array<T>
            where T: HasAfEnum,
                  F: HasAfEnum
        {
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_name(&mut temp as MutAfArray, signal.get() as AfArray,
                                        filter.get() as AfArray, mode as uint8_t);
                HANDLE_ERROR(AfError::from(err_val));
            }
            temp.into()
        }
    )
}

fft_conv_func_def!("1d convolution using fast-fourier transform", fft_convolve1, af_fft_convolve1);
fft_conv_func_def!("2d convolution using fast-fourier transform", fft_convolve2, af_fft_convolve2);
fft_conv_func_def!("3d convolution using fast-fourier transform", fft_convolve3, af_fft_convolve3);

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
#[allow(unused_mut)]
pub fn fir<B, X>(b: &Array<B>, x: &Array<X>) -> Array<X>
    where B: HasAfEnum,
          X: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_fir(&mut temp as MutAfArray, b.get() as AfArray, x.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
#[allow(unused_mut)]
pub fn iir<T: HasAfEnum>(b: &Array<T>, a: &Array<T>, x: &Array<T>) -> Array<T> {
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_iir(&mut temp as MutAfArray,
                             b.get() as AfArray, a.get() as AfArray, x.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// In place 1d dimensional Fast fourier transform
///
/// # Parameters
///
/// - `input` is the input Array
/// - `norm_factor` is the normalization factor
pub fn fft_inplace<T>(input: &mut Array<T>, norm_factor: f64)
    where T: HasAfEnum + ComplexFloating
{
    unsafe {
        let err_val = af_fft_inplace(input.get() as AfArray, norm_factor as c_double);
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
    where T: HasAfEnum + ComplexFloating
{
    unsafe {
        let err_val = af_fft2_inplace(input.get() as AfArray, norm_factor as c_double);
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
    where T: HasAfEnum + ComplexFloating
{
    unsafe {
        let err_val = af_fft3_inplace(input.get() as AfArray, norm_factor as c_double);
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
    where T: HasAfEnum + ComplexFloating
{
    unsafe {
        let err_val = af_ifft_inplace(input.get() as AfArray, norm_factor as c_double);
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
    where T: HasAfEnum + ComplexFloating
{
    unsafe {
        let err_val = af_ifft2_inplace(input.get() as AfArray, norm_factor as c_double);
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
    where T: HasAfEnum + ComplexFloating
{
    unsafe {
        let err_val = af_ifft3_inplace(input.get() as AfArray, norm_factor as c_double);
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
pub fn fft_r2c<T>(input: &Array<T>, norm_factor: f64, pad0: i64) -> Array< Complex<T> >
    where T: HasAfEnum + RealFloating,
          Complex<T>: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_fft_r2c(&mut temp as MutAfArray, input.get() as AfArray,
                                 norm_factor as c_double, pad0 as c_longlong);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
pub fn fft2_r2c<T>(input: &Array<T>, norm_factor: f64,
                   pad0: i64, pad1: i64) -> Array< Complex<T> >
    where T: HasAfEnum + RealFloating,
          Complex<T>: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_fft2_r2c(&mut temp as MutAfArray, input.get() as AfArray,
                                  norm_factor as c_double, pad0 as c_longlong, pad1 as c_longlong);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
pub fn fft3_r2c<T>(input: &Array<T>, norm_factor: f64,
                   pad0: i64, pad1: i64, pad2: i64) -> Array< Complex<T> >
    where T: HasAfEnum + RealFloating,
          Complex<T>: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_fft3_r2c(&mut temp as MutAfArray, input.get() as AfArray,
                                  norm_factor as c_double, pad0 as c_longlong,
                                  pad1 as c_longlong, pad2 as c_longlong);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
pub fn fft_c2r<T>(input: &Array<T>,
                  norm_factor: f64, is_odd: bool) -> Array< T::BaseType >
    where T: HasAfEnum + ComplexFloating,
          <T as HasAfEnum>::BaseType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_fft_c2r(&mut temp as MutAfArray, input.get() as AfArray,
                                 norm_factor as c_double, is_odd as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
pub fn fft2_c2r<T>(input: &Array<T>,
                   norm_factor: f64, is_odd: bool) -> Array< T::BaseType >
    where T: HasAfEnum + ComplexFloating,
          <T as HasAfEnum>::BaseType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_fft2_c2r(&mut temp as MutAfArray, input.get() as AfArray,
                                  norm_factor as c_double, is_odd as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
pub fn fft3_c2r<T>(input: &Array<T>,
                   norm_factor: f64, is_odd: bool) -> Array< T::BaseType >
    where T: HasAfEnum + ComplexFloating,
          <T as HasAfEnum>::BaseType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_fft3_c2r(&mut temp as MutAfArray, input.get() as AfArray,
                                  norm_factor as c_double, is_odd as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}
