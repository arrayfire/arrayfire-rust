extern crate libc;

use array::Array;
use defines::AfError;
use defines::{MatProp, NormType};
use util::to_u32;
use self::libc::{uint8_t, c_int, c_uint, c_double};

type MutAfArray = *mut self::libc::c_longlong;
type MutDouble  = *mut self::libc::c_double;
type AfArray    = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_lu(lower: MutAfArray, upper: MutAfArray, pivot: MutAfArray, input: AfArray) -> c_int;
    fn af_lu_inplace(pivot: MutAfArray, input: AfArray, is_lapack_piv: c_int) -> c_int;
    fn af_qr(q: MutAfArray, r: MutAfArray, tau: MutAfArray, input: AfArray) -> c_int;
    fn af_qr_inplace(tau: MutAfArray, input: AfArray) -> c_int;
    fn af_cholesky(out: MutAfArray, info: *mut c_int, input: AfArray, is_upper: c_int) -> c_int;
    fn af_cholesky_inplace(info: *mut c_int, input: AfArray, is_upper: c_int) -> c_int;
    fn af_solve(x: MutAfArray, a: AfArray, b: AfArray, options: c_uint) -> c_int;
    fn af_solve_lu(x: MutAfArray, a: AfArray, piv: AfArray, b: AfArray, options: c_uint) -> c_int;
    fn af_inverse(out: MutAfArray, input: AfArray, options: c_uint) -> c_int;
    fn af_rank(rank: *mut c_uint, input: AfArray, tol: c_double) -> c_int;
    fn af_det(det_real: MutDouble, det_imag: MutDouble, input: AfArray) -> c_int;
    fn af_norm(out: MutDouble, input: AfArray, ntype: uint8_t, p: c_double, q: c_double) -> c_int;
}

/// Perform LU decomposition
///
/// # Parameters
///
/// - `input` is the input matrix
///
/// # Return Values
///
/// A triplet of Arrays.
///
/// The first Array will contain the lower triangular matrix of the LU decomposition.
///
/// The second Array will contain the lower triangular matrix of the LU decomposition.
///
/// The third Array will contain the permutation indices to map the input to the decomposition.
#[allow(unused_mut)]
pub fn lu(input: &Array) -> Result<(Array, Array, Array), AfError> {
    unsafe {
        let mut lower: i64 = 0;
        let mut upper: i64 = 0;
        let mut pivot: i64 = 0;
        let err_val = af_lu(&mut lower as MutAfArray, &mut upper as MutAfArray,
                            &mut pivot as MutAfArray, input.get() as AfArray);
        match err_val {
            0 => Ok((Array::from(lower), Array::from(upper), Array::from(pivot))),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Perform inplace LU decomposition
///
/// # Parameters
///
/// - `input` is the input matrix
/// - `is_lapack_pic` specified if the pivot is returned in original LAPACK compliant format
///
/// # Return Values
///
/// An Array with permutation indices to map the input to the decomposition. Since, the input
/// matrix is modified in place, only pivot values are returned.
#[allow(unused_mut)]
pub fn lu_inplace(input: &mut Array, is_lapack_piv: bool) -> Result<Array, AfError> {
    unsafe {
        let mut pivot: i64 = 0;
        let err_val = af_lu_inplace(&mut pivot as MutAfArray, input.get() as AfArray,
                                    is_lapack_piv as c_int);
        match err_val {
            0 => Ok(Array::from(pivot)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Perform QR decomposition
///
/// # Parameters
///
/// - `input` is the input matrix
///
/// # Return Values
///
/// A triplet of Arrays.
///
/// The first Array is the orthogonal matrix from QR decomposition
///
/// The second Array is the upper triangular matrix from QR decomposition
///
/// The third Array will contain additional information needed for solving a least squares problem
/// using q and r
#[allow(unused_mut)]
pub fn qr(input: &Array) -> Result<(Array, Array, Array), AfError> {
    unsafe {
        let mut q: i64 = 0;
        let mut r: i64 = 0;
        let mut tau: i64 = 0;
        let err_val = af_qr(&mut q as MutAfArray, &mut r as MutAfArray,
                            &mut tau as MutAfArray, input.get() as AfArray);
        match err_val {
            0 => Ok((Array::from(q), Array::from(r), Array::from(tau))),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Perform inplace QR decomposition
///
/// # Parameters
///
/// - `input` is the input matrix
///
/// # Return Values
///
/// An Array with additional information needed for unpacking the data.
#[allow(unused_mut)]
pub fn qr_inplace(input: &mut Array) -> Result<Array, AfError> {
    unsafe {
        let mut tau: i64 = 0;
        let err_val = af_qr_inplace(&mut tau as MutAfArray, input.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(tau)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Perform Cholesky decomposition
///
/// # Parameters
///
/// - `input` is the input matrix
/// - `is_upper` is a boolean to indicate if the output has to be upper or lower triangular matrix
///
/// # Return Values
///
/// A tuple of an Array and signed 32-bit integer.
///
/// The Array contains the triangular matrix (multiply it with conjugate transpose to reproduce the input).
///
/// If the integer is 0, it means the cholesky decomposition passed. Otherwise, it will contain the rank at
/// which the decomposition failed.
#[allow(unused_mut)]
pub fn cholesky(input: &Array, is_upper: bool) -> Result<(Array, i32), AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let mut info: i32 = 0;
        let err_val = af_cholesky(&mut temp as MutAfArray, &mut info as *mut c_int,
                                  input.get() as AfArray, is_upper as c_int);
        match err_val {
            0 => Ok((Array::from(temp), info)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Perform inplace Cholesky decomposition
///
/// # Parameters
///
/// - `input` is the input matrix
/// - `is_upper` is a boolean to indicate if the output has to be upper or lower triangular matrix
///
/// # Return Values
///
/// A signed 32-bit integer. If the integer is 0, it means the cholesky decomposition passed. Otherwise,
/// it will contain the rank at which the decomposition failed.
#[allow(unused_mut)]
pub fn cholesky_inplace(input: &mut Array, is_upper: bool) -> Result<i32, AfError> {
    unsafe {
        let mut info: i32 = 0;
        let err_val = af_cholesky_inplace(&mut info as *mut c_int, input.get() as AfArray,
                                          is_upper as c_int);
        match err_val {
            0 => Ok(info),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Solve a system of equations
///
/// # Parameters
///
/// - `a` is the coefficient matrix
/// - `b` has the measured values
/// - `options` determine the various properties of matrix a
///
/// The `options` parameter currently needs to be either `NONE`, `LOWER` or `UPPER`, other values are not supported yet.
///
/// # Return Values
///
/// An Array which is the matrix of unknown variables
#[allow(unused_mut)]
pub fn solve(a: &Array, b: &Array, options: MatProp) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_solve(&mut temp as MutAfArray, a.get() as AfArray,
                               b.get() as AfArray, to_u32(options) as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Solve a system of equations
///
/// # Parameters
///
/// - `a` is the output matrix from packed LU decomposition of the coefficient matrix
/// - `piv` is the pivot array from packed LU decomposition of the coefficient matrix
/// - `b` has the measured values
/// - `options` determine the various properties of matrix a
///
/// The `options` parameter currently needs to be `NONE`, other values are not supported yet.
///
/// # Return Values
///
/// An Array which is the matrix of unknown variables
#[allow(unused_mut)]
pub fn solve_lu(a: &Array, piv: &Array, b: &Array,
                options: MatProp) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_solve_lu(&mut temp as MutAfArray, a.get() as AfArray, piv.get() as AfArray,
                    b.get() as AfArray, to_u32(options) as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Compute inverse of a matrix
///
/// # Parameters
///
/// - `input` is the input matrix
/// - `options` determine various properties of input matrix
///
/// The parameter `options` currently take only the value `NONE`.
///
/// # Return Values
///
/// An Array with values of the inverse of input matrix.
#[allow(unused_mut)]
pub fn inverse(input: &Array, options: MatProp) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_inverse(&mut temp as MutAfArray, input.get() as AfArray, to_u32(options) as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Find rank of a matrix
///
/// # Parameters
///
/// - `input` is the input matrix
/// - `tol` is the tolerance value
///
/// # Return Values
///
/// An unsigned 32-bit integer which is the rank of the input matrix.
#[allow(unused_mut)]
pub fn rank(input: &Array, tol: f64) -> Result<u32, AfError> {
    unsafe {
        let mut temp: u32 = 0;
        let err_val = af_rank(&mut temp as *mut c_uint, input.get() as AfArray, tol as c_double);
        match err_val {
            0 => Ok(temp),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Find the determinant of the matrix
///
/// # Parameters
///
/// - `input` is the input matrix
///
/// # Return Values
///
/// A tuple of 32-bit floating point values.
///
/// If the input matrix is non-complex type, only first values of tuple contains the result.
#[allow(unused_mut)]
pub fn det(input: &Array) -> Result<(f64, f64), AfError> {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        let err_val = af_det(&mut real as MutDouble, &mut imag as MutDouble, input.get() as AfArray);
        match err_val {
            0 => Ok((real, imag)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Find the norm of a matrix
///
/// # Parameters
///
/// - `input` is the input matrix
/// - `ntype` is specifies the required norm type using enum [NormType](./enum.NormType.html)
/// - `p` specifies the value of *P* when `ntype` is one of VECTOR_P, MATRIX_L_PQ. It is ignored
/// for other values of `ntype`
/// - `q` specifies the value of *Q* when `ntype` is MATRIX_L_PQ. This parameter is ignored if
/// `ntype` is anything else.
///
/// # Return Values
///
/// A 64-bit floating point value that contains the norm of input matrix.
#[allow(unused_mut)]
pub fn norm(input: &Array, ntype: NormType, p: f64, q: f64) -> Result<f64, AfError> {
    unsafe {
        let mut out: f64 = 0.0;
        let err_val = af_norm(&mut out as MutDouble, input.get() as AfArray, ntype as uint8_t,
                              p as c_double, q as c_double);
        match err_val {
            0 => Ok(out),
            _ => Err(AfError::from(err_val)),
        }
    }
}
