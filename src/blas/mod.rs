extern crate libc;

use array::Array;
use defines::AfError;
use defines::MatProp;
use error::HANDLE_ERROR;
use self::libc::{c_uint, c_int};
use util::to_u32;

type MutAfArray = *mut self::libc::c_longlong;
type AfArray    = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_matmul(out: MutAfArray, lhs: AfArray, rhs: AfArray,
                 optlhs: c_uint, optrhs: c_uint) -> c_int;

    fn af_dot(out: MutAfArray, lhs: AfArray, rhs: AfArray,
              optlhs: c_uint, optrhs: c_uint) -> c_int;

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
pub fn matmul(lhs: &Array, rhs: &Array,
              optlhs: MatProp, optrhs: MatProp) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_matmul(&mut temp as MutAfArray,
                                lhs.get() as AfArray, rhs.get() as AfArray,
                                to_u32(optlhs) as c_uint, to_u32(optrhs) as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Calculate the dot product of vectors.
///
/// Scalar dot product between two vectors. Also referred to as the inner product. This function returns the scalar product of two equal sized vectors or between a matrix and a vector. The second operand needs to be a vector in either case.
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
pub fn dot(lhs: &Array, rhs: &Array,
           optlhs: MatProp, optrhs: MatProp) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_dot(&mut temp as MutAfArray,
                             lhs.get() as AfArray, rhs.get() as AfArray,
                             to_u32(optlhs) as c_uint, to_u32(optrhs) as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
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
pub fn transpose(arr: &Array, conjugate: bool) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_transpose(&mut temp as MutAfArray,
                                   arr.get() as AfArray, conjugate as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Inplace transpose of a matrix.
///
/// # Parameters
///
/// - `arr` is the input Array that has to be transposed
/// - `conjugate` is a boolean that indicates if the transpose operation needs to be a conjugate
/// transpose
#[allow(unused_mut)]
pub fn transpose_inplace(arr: &mut Array, conjugate: bool) {
    unsafe {
        let err_val = af_transpose_inplace(arr.get() as AfArray, conjugate as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
}
