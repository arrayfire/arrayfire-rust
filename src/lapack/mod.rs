use super::core::{
    af_array, AfError, Array, FloatingPoint, HasAfEnum, MatProp, NormType, HANDLE_ERROR,
};

use libc::{c_double, c_int, c_uint};

extern "C" {
    fn af_svd(u: *mut af_array, s: *mut af_array, vt: *mut af_array, input: af_array) -> c_int;
    fn af_svd_inplace(
        u: *mut af_array,
        s: *mut af_array,
        vt: *mut af_array,
        input: af_array,
    ) -> c_int;
    fn af_lu(
        lower: *mut af_array,
        upper: *mut af_array,
        pivot: *mut af_array,
        input: af_array,
    ) -> c_int;
    fn af_lu_inplace(pivot: *mut af_array, input: af_array, is_lapack_piv: bool) -> c_int;
    fn af_qr(q: *mut af_array, r: *mut af_array, tau: *mut af_array, input: af_array) -> c_int;
    fn af_qr_inplace(tau: *mut af_array, input: af_array) -> c_int;
    fn af_cholesky(out: *mut af_array, info: *mut c_int, input: af_array, is_upper: bool) -> c_int;
    fn af_cholesky_inplace(info: *mut c_int, input: af_array, is_upper: bool) -> c_int;
    fn af_solve(x: *mut af_array, a: af_array, b: af_array, options: c_uint) -> c_int;
    fn af_solve_lu(
        x: *mut af_array,
        a: af_array,
        piv: af_array,
        b: af_array,
        options: c_uint,
    ) -> c_int;
    fn af_inverse(out: *mut af_array, input: af_array, options: c_uint) -> c_int;
    fn af_rank(rank: *mut c_uint, input: af_array, tol: c_double) -> c_int;
    fn af_det(det_real: *mut c_double, det_imag: *mut c_double, input: af_array) -> c_int;
    fn af_norm(
        out: *mut c_double,
        input: af_array,
        ntype: c_uint,
        p: c_double,
        q: c_double,
    ) -> c_int;
    fn af_is_lapack_available(out: *mut bool) -> c_int;
    fn af_pinverse(out: *mut af_array, input: af_array, tol: c_double, options: c_uint) -> c_int;
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
pub fn svd<T>(input: &Array<T>) -> (Array<T>, Array<T::BaseType>, Array<T>)
where
    T: HasAfEnum + FloatingPoint,
    T::BaseType: HasAfEnum,
{
    unsafe {
        let mut u: af_array = std::ptr::null_mut();
        let mut s: af_array = std::ptr::null_mut();
        let mut vt: af_array = std::ptr::null_mut();
        let err_val = af_svd(
            &mut u as *mut af_array,
            &mut s as *mut af_array,
            &mut vt as *mut af_array,
            input.get(),
        );
        HANDLE_ERROR(AfError::from(err_val));
        (u.into(), s.into(), vt.into())
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
pub fn svd_inplace<T>(input: &mut Array<T>) -> (Array<T>, Array<T::BaseType>, Array<T>)
where
    T: HasAfEnum + FloatingPoint,
    T::BaseType: HasAfEnum,
{
    unsafe {
        let mut u: af_array = std::ptr::null_mut();
        let mut s: af_array = std::ptr::null_mut();
        let mut vt: af_array = std::ptr::null_mut();
        let err_val = af_svd_inplace(
            &mut u as *mut af_array,
            &mut s as *mut af_array,
            &mut vt as *mut af_array,
            input.get(),
        );
        HANDLE_ERROR(AfError::from(err_val));
        (u.into(), s.into(), vt.into())
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
pub fn lu<T>(input: &Array<T>) -> (Array<T>, Array<T>, Array<i32>)
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut lower: af_array = std::ptr::null_mut();
        let mut upper: af_array = std::ptr::null_mut();
        let mut pivot: af_array = std::ptr::null_mut();
        let err_val = af_lu(
            &mut lower as *mut af_array,
            &mut upper as *mut af_array,
            &mut pivot as *mut af_array,
            input.get(),
        );
        HANDLE_ERROR(AfError::from(err_val));
        (lower.into(), upper.into(), pivot.into())
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
pub fn lu_inplace<T>(input: &mut Array<T>, is_lapack_piv: bool) -> Array<i32>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut pivot: af_array = std::ptr::null_mut();
        let err_val = af_lu_inplace(&mut pivot as *mut af_array, input.get(), is_lapack_piv);
        HANDLE_ERROR(AfError::from(err_val));
        pivot.into()
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
pub fn qr<T>(input: &Array<T>) -> (Array<T>, Array<T>, Array<T>)
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut q: af_array = std::ptr::null_mut();
        let mut r: af_array = std::ptr::null_mut();
        let mut tau: af_array = std::ptr::null_mut();
        let err_val = af_qr(
            &mut q as *mut af_array,
            &mut r as *mut af_array,
            &mut tau as *mut af_array,
            input.get(),
        );
        HANDLE_ERROR(AfError::from(err_val));
        (q.into(), r.into(), tau.into())
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
pub fn qr_inplace<T>(input: &mut Array<T>) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut tau: af_array = std::ptr::null_mut();
        let err_val = af_qr_inplace(&mut tau as *mut af_array, input.get());
        HANDLE_ERROR(AfError::from(err_val));
        tau.into()
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
pub fn cholesky<T>(input: &Array<T>, is_upper: bool) -> (Array<T>, i32)
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let mut info: i32 = 0;
        let err_val = af_cholesky(
            &mut temp as *mut af_array,
            &mut info as *mut c_int,
            input.get(),
            is_upper,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (temp.into(), info)
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
pub fn cholesky_inplace<T>(input: &mut Array<T>, is_upper: bool) -> i32
where
    T: HasAfEnum + FloatingPoint,
{
    let mut info: i32 = 0;
    unsafe {
        let err_val = af_cholesky_inplace(&mut info as *mut c_int, input.get(), is_upper);
        HANDLE_ERROR(AfError::from(err_val));
    }
    info
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
pub fn solve<T>(a: &Array<T>, b: &Array<T>, options: MatProp) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_solve(
            &mut temp as *mut af_array,
            a.get(),
            b.get(),
            options as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn solve_lu<T>(a: &Array<T>, piv: &Array<i32>, b: &Array<T>, options: MatProp) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_solve_lu(
            &mut temp as *mut af_array,
            a.get(),
            piv.get(),
            b.get(),
            options as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn inverse<T>(input: &Array<T>, options: MatProp) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_inverse(&mut temp as *mut af_array, input.get(), options as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn rank<T>(input: &Array<T>, tol: f64) -> u32
where
    T: HasAfEnum + FloatingPoint,
{
    let mut temp: u32 = 0;
    unsafe {
        let err_val = af_rank(&mut temp as *mut c_uint, input.get(), tol);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp
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
pub fn det<T>(input: &Array<T>) -> (f64, f64)
where
    T: HasAfEnum + FloatingPoint,
{
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_det(
            &mut real as *mut c_double,
            &mut imag as *mut c_double,
            input.get(),
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    (real, imag)
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
pub fn norm<T>(input: &Array<T>, ntype: NormType, p: f64, q: f64) -> f64
where
    T: HasAfEnum + FloatingPoint,
{
    let mut out: f64 = 0.0;
    unsafe {
        let err_val = af_norm(
            &mut out as *mut c_double,
            input.get(),
            ntype as c_uint,
            p,
            q,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    out
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
    let mut temp: bool = false;
    unsafe {
        af_is_lapack_available(&mut temp as *mut bool);
    }
    temp
}

/// Psuedo Inverse of Matrix
///
/// # Parameters
///
/// - `input` is input matrix
/// - `tolerance` defines the lower threshold for singular values from SVD
/// - `option` must be [MatProp::NONE](./enum.MatProp.html) (more options might be supported in the future)
///
/// Notes:
///
/// - Tolerance is not the actual lower threshold, but it is passed in as a
///   parameter to the calculation of the actual threshold relative to the shape and contents of input.
/// - First, try setting tolerance to 1e-6 for single precision and 1e-12 for double.
///
/// # Return
///
/// Pseudo Inverse matrix for the input matrix
pub fn pinverse<T>(input: &Array<T>, tolerance: f64, option: MatProp) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut out: af_array = std::ptr::null_mut();
        let err_val = af_pinverse(
            &mut out as *mut af_array,
            input.get(),
            tolerance,
            option as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        out.into()
    }
}
