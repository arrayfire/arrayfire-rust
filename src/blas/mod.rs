extern crate libc;

use self::libc::{c_int, c_uint};
use crate::array::Array;
use crate::defines::AfError;
use crate::defines::MatProp;
use crate::error::HANDLE_ERROR;
use crate::util::{to_u32, AfArray, MutAfArray};
use crate::util::{FloatingPoint, HasAfEnum};

#[allow(dead_code)]
extern "C" {
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
