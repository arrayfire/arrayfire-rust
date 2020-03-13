extern crate libc;

use self::libc::{c_int, c_uint, c_void};
use crate::array::Array;
use crate::defines::{AfError, CublasMathMode, MatProp};
use crate::error::HANDLE_ERROR;
use crate::util::{to_u32, AfArray, MutAfArray};
use crate::util::{FloatingPoint, HasAfEnum};
use std::vec::Vec;

#[allow(dead_code)]
extern "C" {
    fn af_gemm(
        out: MutAfArray,
        optlhs: c_uint,
        optrhs: c_uint,
        alpha: *const c_void,
        lhs: AfArray,
        rhs: AfArray,
        beta: *const c_void,
    ) -> c_int;

    fn af_matmul(
        out: MutAfArray,
        lhs: AfArray,
        rhs: AfArray,
        optlhs: c_uint,
        optrhs: c_uint,
    ) -> c_int;

    fn af_dot(out: MutAfArray, lhs: AfArray, rhs: AfArray, optlhs: c_uint, optrhs: c_uint)
        -> c_int;

    fn af_transpose(out: MutAfArray, arr: AfArray, conjugate: c_int) -> c_int;
    fn af_transpose_inplace(arr: AfArray, conjugate: c_int) -> c_int;

    fn afcu_cublasSetMathMode(mode: c_int) -> c_int;
}

/// BLAS general matrix multiply (GEMM) of two Array objects
///
///
/// This provides a general interface to the BLAS level 3 general matrix multiply (GEMM),
/// which is generally defined as:
///
/// \begin{equation}
///     C = \alpha * opA(A)opB(B) + \beta * C
/// \end{equation}
///
///   where $\alpha$ (**alpha**) and $\beta$ (**beta**) are both scalars; $A$ and $B$ are the matrix
///   multiply operands; and $opA$ and $opB$ are noop
///   (if optLhs is [MatProp::NONE](./enum.MatProp.html)) or transpose
///   (if optLhs is [MatProp::TRANS](./enum.MatProp.html)) operations on $A$ or $B$ before the
///   actual GEMM operation. Batched GEMM is supported if at least either $A$ or $B$ have more than
///   two dimensions (see [af::matmul](http://arrayfire.org/docs/group__blas__func__matmul.htm#ga63306b6ed967bd1055086db862fe885b)
///   for more details on broadcasting). However, only one **alpha** and one **beta** can be used
///   for all of the batched matrix operands.
///
///   The `output` Array can be used both as an input and output. An allocation will be performed
///   if you pass an empty Array (i.e. `let c: Array<f32> = (0 as i64).into();`). If a valid Array
///   is passed as $C$, the operation will be performed on that Array itself. The C Array must be
///   the correct type and shape; otherwise, an error will be thrown.
///
///   Note: Passing an Array that has not been initialized to the C array
///   will cause undefined behavior.
///
/// # Examples
///
/// Given below is an example of using gemm API with existing Arrays
///
/// ```rust
/// use arrayfire::{Array, Dim4, print, randu, gemm};
///
/// let dims = Dim4::new(&[5, 5, 1, 1]);
///
/// let alpha = vec![1.0 as f32];
/// let  beta = vec![2.0 as f32];
///
/// let lhs = randu::<f32>(dims);
/// let rhs = randu::<f32>(dims);
///
/// let mut result = Array::new_empty(dims);
/// gemm(&mut result, arrayfire::MatProp::NONE, arrayfire::MatProp::NONE,
///      alpha, &lhs, &rhs, beta);
/// ```
///
/// If you don't have an existing Array, you can also use gemm in the following fashion.
/// However, if there is no existing Array that you need to fill and your use case doesn't
/// deal with alpha and beta from gemm equation, it is recommended to use
/// [matmul](./fn.matmul.html) for more terse code.
///
/// ```rust
/// use arrayfire::{Array, Dim4, print, randu, gemm};
///
/// let dims = Dim4::new(&[5, 5, 1, 1]);
///
/// let alpha = vec![1.0 as f32];
/// let  beta = vec![2.0 as f32];
///
/// let lhs = randu::<f32>(dims);
/// let rhs = randu::<f32>(dims);
///
/// let mut result: Array::<f32> = (0 as i64).into();
///
/// gemm(&mut result, arrayfire::MatProp::NONE, arrayfire::MatProp::NONE,
///      alpha, &lhs, &rhs, beta);
/// ```
///
/// # Parameters
///
/// - `optlhs` - Transpose left hand side before the function is performed, uses one of the values of [MatProp](./enum.MatProp.html)
/// - `optrhs` - Transpose right hand side before the function is performed, uses one of the values of [MatProp](./enum.MatProp.html)
/// - `alpha` is alpha value;
/// - `lhs` is the Array on left hand side
/// - `rhs` is the Array on right hand side
/// - `beta` is beta value;
///
/// # Return Values
///
/// Array, result of gemm operation
pub fn gemm<T>(
    output: &mut Array<T>,
    optlhs: MatProp,
    optrhs: MatProp,
    alpha: Vec<T>,
    lhs: &Array<T>,
    rhs: &Array<T>,
    beta: Vec<T>,
) where
    T: HasAfEnum + FloatingPoint,
{
    let mut out = output.get();
    unsafe {
        let err_val = af_gemm(
            &mut out as MutAfArray,
            to_u32(optlhs) as c_uint,
            to_u32(optrhs) as c_uint,
            alpha.as_ptr() as *const c_void,
            lhs.get() as AfArray,
            rhs.get() as AfArray,
            beta.as_ptr() as *const c_void,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    output.set(out);
}

/// Matrix multiple of two Arrays
///
/// # Parameters
///
/// - `lhs` is the Array on left hand side
/// - `rhs` is the Array on right hand side
/// - `optlhs` - Transpose left hand side before the function is performed, uses one of the values of [MatProp](./enum.MatProp.html)
/// - `optrhs` - Transpose right hand side before the function is performed, uses one of the values of [MatProp](./enum.MatProp.html)
///
/// # Return Values
///
/// The result Array of matrix multiplication
#[allow(unused_mut)]
pub fn matmul<T>(lhs: &Array<T>, rhs: &Array<T>, optlhs: MatProp, optrhs: MatProp) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_matmul(
            &mut temp as MutAfArray,
            lhs.get() as AfArray,
            rhs.get() as AfArray,
            to_u32(optlhs) as c_uint,
            to_u32(optrhs) as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Calculate the dot product of vectors.
///
/// Scalar dot product between two vectors. Also referred to as the inner product. This function returns the scalar product of two equal sized vectors.
///
/// # Parameters
///
/// - `lhs` - Left hand side of dot operation
/// - `rhs` - Right hand side of dot operation
/// - `optlhs` - Options for lhs. Currently only NONE value from [MatProp](./enum.MatProp.html) is supported.
/// - `optrhs` - Options for rhs. Currently only NONE value from [MatProp](./enum.MatProp.html) is supported.
///
/// # Return Values
///
/// The result of dot product.
#[allow(unused_mut)]
pub fn dot<T>(lhs: &Array<T>, rhs: &Array<T>, optlhs: MatProp, optrhs: MatProp) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_dot(
            &mut temp as MutAfArray,
            lhs.get() as AfArray,
            rhs.get() as AfArray,
            to_u32(optlhs) as c_uint,
            to_u32(optrhs) as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Transpose of a matrix.
///
/// # Parameters
///
/// - `arr` is the input Array
/// - `conjugate` is a boolean that indicates if the transpose operation needs to be a conjugate
/// transpose
///
/// # Return Values
///
/// Transposed Array.
#[allow(unused_mut)]
pub fn transpose<T: HasAfEnum>(arr: &Array<T>, conjugate: bool) -> Array<T> {
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_transpose(
            &mut temp as MutAfArray,
            arr.get() as AfArray,
            conjugate as c_int,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Inplace transpose of a matrix.
///
/// # Parameters
///
/// - `arr` is the input Array that has to be transposed
/// - `conjugate` is a boolean that indicates if the transpose operation needs to be a conjugate
/// transpose
#[allow(unused_mut)]
pub fn transpose_inplace<T: HasAfEnum>(arr: &mut Array<T>, conjugate: bool) {
    unsafe {
        let err_val = af_transpose_inplace(arr.get() as AfArray, conjugate as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Sets the cuBLAS math mode for the internal handle.
///
/// See the cuBLAS documentation for additional details
///
/// # Parameters
///
/// - `mode` takes a value of [CublasMathMode](./enum.CublasMathMode.html) enum
pub fn set_cublas_mode(mode: CublasMathMode) {
    unsafe {
        afcu_cublasSetMathMode(mode as c_int);
        //let err_val = afcu_cublasSetMathMode(mode as c_int);
        // FIXME(wonder if this something to throw off,
        // the program state is not invalid or anything
        // HANDLE_ERROR(AfError::from(err_val));
    }
}
