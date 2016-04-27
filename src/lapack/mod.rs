extern crate libc;

use array::Array;
use defines::{AfError, MatProp, NormType};
use error::HANDLE_ERROR;
use util::to_u32;
use self::libc::{uint8_t, c_int, c_uint, c_double};

type MutAfArray = *mut self::libc::c_longlong;
type MutDouble  = *mut self::libc::c_double;
type AfArray    = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_svd(u: MutAfArray, s: MutAfArray, vt: MutAfArray, input: AfArray) -> c_int;
    fn af_svd_inplace(u: MutAfArray, s: MutAfArray, vt: MutAfArray, input: AfArray) -> c_int;
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
    fn af_is_lapack_available(out: *mut c_int) -> c_int;
}

/// Perform Singular Value Decomposition
///
/// This function factorizes a matrix A into two unitary matrices U and Vt, and a diagonal matrix S
/// such that
///
/// A = U∗S∗Vt
///
/// If A has M rows and N columns, U is of the size M x M , V is of size N x N, and S is of size M
/// x N
///
/// # Parameters
///
/// - `in` is the input matrix
///
/// # Return Values
///
/// A triplet of Arrays.
///
/// The first Array is the output array containing U
///
/// The second Array is the output array containing the diagonal values of sigma, (singular values of the input matrix))
///
/// The third Array is the output array containing V ^ H
#[allow(unused_mut)]
pub fn svd(input: &Array) -> (Array, Array, Array) {
    unsafe {
        let mut u: i64 = 0;
        let mut s: i64 = 0;
        let mut vt: i64 = 0;
        let err_val = af_svd(&mut u as MutAfArray, &mut s as MutAfArray, &mut vt as MutAfArray,
                             input.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        (Array::from(u), Array::from(s), Array::from(vt))
    }
}

/// Perform Singular Value Decomposition inplace
///
/// This function factorizes a matrix A into two unitary matrices U and Vt, and a diagonal matrix S
/// such that
///
/// A = U∗S∗Vt
///
/// If A has M rows and N columns, U is of the size M x M , V is of size N x N, and S is of size M
/// x N
///
/// # Parameters
///
/// - `in` is the input/output matrix. This will contain random data after the function call is
/// complete.
///
/// # Return Values
///
/// A triplet of Arrays.
///
/// The first Array is the output array containing U
///
/// The second Array is the output array containing the diagonal values of sigma, (singular values of the input matrix))
///
/// The third Array is the output array containing V ^ H
#[allow(unused_mut)]
pub fn svd_inplace(input: &mut Array) -> (Array, Array, Array) {
    unsafe {
        let mut u: i64 = 0;
        let mut s: i64 = 0;
        let mut vt: i64 = 0;
        let err_val = af_svd_inplace(&mut u as MutAfArray, &mut s as MutAfArray,
                                     &mut vt as MutAfArray, input.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        (Array::from(u), Array::from(s), Array::from(vt))
    }
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
pub fn lu(input: &Array) -> (Array, Array, Array) {
    unsafe {
        let mut lower: i64 = 0;
        let mut upper: i64 = 0;
        let mut pivot: i64 = 0;
        let err_val = af_lu(&mut lower as MutAfArray, &mut upper as MutAfArray,
                            &mut pivot as MutAfArray, input.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        (Array::from(lower), Array::from(upper), Array::from(pivot))
    }
}

/// Perform inplace LU decomposition
///
/// # Parameters
///
/// - `input` contains the input matrix on entry and packed LU decomposition on exit
/// - `is_lapack_pic` specified if the pivot is returned in original LAPACK compliant format
///
/// # Return Values
///
/// An Array with permutation indices to map the input to the decomposition. Since, the input
/// matrix is modified in place, only pivot values are returned.
#[allow(unused_mut)]
pub fn lu_inplace(input: &mut Array, is_lapack_piv: bool) -> Array {
    unsafe {
        let mut pivot: i64 = 0;
        let err_val = af_lu_inplace(&mut pivot as MutAfArray, input.get() as AfArray,
                                    is_lapack_piv as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(pivot)
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
pub fn qr(input: &Array) -> (Array, Array, Array) {
    unsafe {
        let mut q: i64 = 0;
        let mut r: i64 = 0;
        let mut tau: i64 = 0;
        let err_val = af_qr(&mut q as MutAfArray, &mut r as MutAfArray,
                            &mut tau as MutAfArray, input.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        (Array::from(q), Array::from(r), Array::from(tau))
    }
}

/// Perform inplace QR decomposition
///
/// # Parameters
///
/// - `input` contains the input matrix on entry, and packed QR decomposition on exit
///
/// # Return Values
///
/// An Array with additional information needed for unpacking the data.
#[allow(unused_mut)]
pub fn qr_inplace(input: &mut Array) -> Array {
    unsafe {
        let mut tau: i64 = 0;
        let err_val = af_qr_inplace(&mut tau as MutAfArray, input.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(tau)
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
pub fn cholesky(input: &Array, is_upper: bool) -> (Array, i32) {
    unsafe {
        let mut temp: i64 = 0;
        let mut info: i32 = 0;
        let err_val = af_cholesky(&mut temp as MutAfArray, &mut info as *mut c_int,
                                  input.get() as AfArray, is_upper as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        (Array::from(temp), info)
    }
}

/// Perform inplace Cholesky decomposition
///
/// # Parameters
///
/// - `input` contains the input matrix on entry, and triangular matrix on exit.
/// - `is_upper` is a boolean to indicate if the output has to be upper or lower triangular matrix
///
/// # Return Values
///
/// A signed 32-bit integer. If the integer is 0, it means the cholesky decomposition passed. Otherwise,
/// it will contain the rank at which the decomposition failed.
#[allow(unused_mut)]
pub fn cholesky_inplace(input: &mut Array, is_upper: bool) -> i32 {
    unsafe {
        let mut info: i32 = 0;
        let err_val = af_cholesky_inplace(&mut info as *mut c_int, input.get() as AfArray,
                                          is_upper as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        info
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
pub fn solve(a: &Array, b: &Array, options: MatProp) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_solve(&mut temp as MutAfArray, a.get() as AfArray,
                               b.get() as AfArray, to_u32(options) as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
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
                options: MatProp) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_solve_lu(&mut temp as MutAfArray, a.get() as AfArray, piv.get() as AfArray,
                    b.get() as AfArray, to_u32(options) as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
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
pub fn inverse(input: &Array, options: MatProp) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_inverse(&mut temp as MutAfArray, input.get() as AfArray, to_u32(options) as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
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
pub fn rank(input: &Array, tol: f64) -> u32 {
    unsafe {
        let mut temp: u32 = 0;
        let err_val = af_rank(&mut temp as *mut c_uint, input.get() as AfArray, tol as c_double);
        HANDLE_ERROR(AfError::from(err_val));
        temp
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
pub fn det(input: &Array) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        let err_val = af_det(&mut real as MutDouble, &mut imag as MutDouble, input.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        (real, imag)
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
pub fn norm(input: &Array, ntype: NormType, p: f64, q: f64) -> f64 {
    unsafe {
        let mut out: f64 = 0.0;
        let err_val = af_norm(&mut out as MutDouble, input.get() as AfArray, ntype as uint8_t,
                              p as c_double, q as c_double);
        HANDLE_ERROR(AfError::from(err_val));
        out
    }
}

/// Function to check if lapack support is available
///
/// # Parameters
///
/// None
///
/// # Return Values
///
/// Return a boolean indicating if ArrayFire was compiled with lapack support
pub fn is_lapack_available() -> bool {
    unsafe {
        let mut temp: i32 = 0;
        af_is_lapack_available(&mut temp as *mut c_int);
        temp > 0 // Return boolean fla
    }
}