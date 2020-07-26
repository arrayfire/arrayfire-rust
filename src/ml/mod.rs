use super::core::{
    af_array, dim_t, AfError, Array, ConvGradientType, Dim4, HasAfEnum, RealFloating, HANDLE_ERROR,
};

use libc::{c_int, c_uint};

extern "C" {
    fn af_convolve2_nn(
        out: *mut af_array,
        signal: af_array,
        filter: af_array,
        stride_dims: c_uint,
        strides: *const dim_t,
        padding_dim: c_uint,
        paddings: *const dim_t,
        dilation_dim: c_uint,
        dilations: *const dim_t,
    ) -> c_int;

    fn af_convolve2_gradient_nn(
        out: *mut af_array,
        incoming_gradient: af_array,
        original_signal: af_array,
        original_filter: af_array,
        convolved_output: af_array,
        stride_dims: c_uint,
        strides: *const dim_t,
        padding_dims: c_uint,
        paddings: *const dim_t,
        dilation_dims: c_uint,
        dilations: *const dim_t,
        grad_type: c_uint,
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
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_convolve2_nn(
            &mut temp as *mut af_array,
            signal.get(),
            filter.get(),
            strides.ndims() as c_uint,
            strides.get().as_ptr() as *const dim_t,
            padding.ndims() as c_uint,
            padding.get().as_ptr() as *const dim_t,
            dilation.ndims() as c_uint,
            dilation.get().as_ptr() as *const dim_t,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
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
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_convolve2_gradient_nn(
            &mut temp as *mut af_array,
            incoming_grad.get(),
            original_signal.get(),
            original_filter.get(),
            convolved_output.get(),
            strides.ndims() as c_uint,
            strides.get().as_ptr() as *const dim_t,
            padding.ndims() as c_uint,
            padding.get().as_ptr() as *const dim_t,
            dilation.ndims() as c_uint,
            dilation.get().as_ptr() as *const dim_t,
            grad_type as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}
