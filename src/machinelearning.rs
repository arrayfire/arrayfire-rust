extern crate libc;

use self::libc::{c_int, c_longlong, c_uint};
use crate::array::Array;
use crate::defines::{AfError, ConvGradientType};
use crate::dim4::Dim4;
use crate::error::HANDLE_ERROR;
use crate::util::{AfArray, DimT, HasAfEnum, MutAfArray, RealFloating};

#[allow(dead_code)]
extern "C" {
    fn af_convolve2_nn(
        out: MutAfArray,
        signal: AfArray,
        filter: AfArray,
        stride_dims: c_uint,
        strides: *const DimT,
        padding_dim: c_uint,
        paddings: *const DimT,
        dilation_dim: c_uint,
        dilations: *const DimT,
    ) -> c_int;

    fn af_convolve2_gradient_nn(
        out: MutAfArray,
        incoming_gradient: AfArray,
        original_signal: AfArray,
        original_filter: AfArray,
        convolved_output: AfArray,
        stride_dims: c_uint,
        strides: *const DimT,
        padding_dims: c_uint,
        paddings: *const DimT,
        dilation_dims: c_uint,
        dilations: *const DimT,
        grad_type: c_int,
    ) -> c_int;
}

/// Convolution Integral for two dimensional data
///
/// This version of convolution is consistent with the machine learning formulation
/// that will spatially convolve a filter on 2-dimensions against a signal. Multiple
/// signals and filters can be batched against each other. Furthermore, the signals
/// and filters can be multi-dimensional however their dimensions must match. Usually,
/// this is the forward pass convolution in ML
///
/// Example:
///
/// Signals with dimensions: d0 x d1 x d2 x Ns
///
/// Filters with dimensions: d0 x d1 x d2 x Nf
///
/// Resulting Convolution: d0 x d1 x Nf x Ns
///
/// # Parameters
///
/// - `signal` is the input signal
/// - `filter` is convolution filter
/// - `strides` are distance between consecutive elements along each dimension for original convolution
/// - `padding` specifies padding width along each dimension for original convolution
/// - `dilation` specifies filter dilation along each dimension for original convolution
///
/// # Return Values
///
/// Convolved Array
pub fn convolve2_nn<T>(
    signal: &Array<T>,
    filter: &Array<T>,
    strides: Dim4,
    padding: Dim4,
    dilation: Dim4,
) -> Array<T>
where
    T: HasAfEnum + RealFloating,
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_convolve2_nn(
            &mut temp as MutAfArray,
            signal.get() as AfArray,
            filter.get() as AfArray,
            strides.ndims() as c_uint,
            strides.get().as_ptr() as *const c_longlong,
            padding.ndims() as c_uint,
            padding.get().as_ptr() as *const c_longlong,
            dilation.ndims() as c_uint,
            dilation.get().as_ptr() as *const c_longlong,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Backward pass gradient of 2D convolution
///
/// # Parameters
///
/// - `incoming_gradient` gradients to be distributed in backwards pass
/// - `original_signal` input signal to forward pass of convolution assumed structure of input is ( d0 x d1 x d2 x N )
/// - `original_filter` input filter to forward pass of convolution assumed structure of input is ( d0 x d1 x d2 x N )
/// - `convolved_output` output from forward pass of convolution
/// - `strides` are distance between consecutive elements along each dimension for original convolution
/// - `padding` specifies padding width along each dimension for original convolution
/// - `dilation` specifies filter dilation along each dimension for original convolution
/// - `grad_type` specifies which gradient to return
///
/// # Return Values
///
/// Gradient Array w.r.t input generated from [convolve2_nn](./fn.convolve2_nn.html)
#[allow(clippy::too_many_arguments)]
pub fn convolve2_gradient_nn<T>(
    incoming_grad: &Array<T>,
    original_signal: &Array<T>,
    original_filter: &Array<T>,
    convolved_output: &Array<T>,
    strides: Dim4,
    padding: Dim4,
    dilation: Dim4,
    grad_type: ConvGradientType,
) -> Array<T>
where
    T: HasAfEnum + RealFloating,
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_convolve2_gradient_nn(
            &mut temp as MutAfArray,
            incoming_grad.get() as AfArray,
            original_signal.get() as AfArray,
            original_filter.get() as AfArray,
            convolved_output.get() as AfArray,
            strides.ndims() as c_uint,
            strides.get().as_ptr() as *const c_longlong,
            padding.ndims() as c_uint,
            padding.get().as_ptr() as *const c_longlong,
            dilation.ndims() as c_uint,
            dilation.get().as_ptr() as *const c_longlong,
            grad_type as c_int,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}
