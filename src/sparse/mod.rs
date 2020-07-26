use super::core::{
    af_array, dim_t, AfError, Array, FloatingPoint, HasAfEnum, SparseFormat, HANDLE_ERROR,
};

use libc::{c_int, c_uint, c_void};

extern "C" {
    fn af_create_sparse_array(
        out: *mut af_array,
        nRows: dim_t,
        nCols: dim_t,
        vals: af_array,
        rowIdx: af_array,
        colIdx: af_array,
        stype: c_uint,
    ) -> c_int;

    fn af_create_sparse_array_from_ptr(
        out: *mut af_array,
        nRows: dim_t,
        nCols: dim_t,
        nNZ: dim_t,
        values: *const c_void,
        rowIdx: *const c_int,
        colIdx: *const c_int,
        aftype: c_uint,
        stype: c_uint,
        src: c_uint,
    ) -> c_int;

    fn af_create_sparse_array_from_dense(
        out: *mut af_array,
        dense: af_array,
        stype: c_uint,
    ) -> c_int;

    fn af_sparse_convert_to(out: *mut af_array, input: af_array, dstStrge: c_uint) -> c_int;

    fn af_sparse_to_dense(out: *mut af_array, sparse: af_array) -> c_int;

    fn af_sparse_get_info(
        vals: *mut af_array,
        rIdx: *mut af_array,
        cIdx: *mut af_array,
        stype: *mut c_uint,
        input: af_array,
    ) -> c_int;

    fn af_sparse_get_values(out: *mut af_array, input: af_array) -> c_int;

    fn af_sparse_get_row_idx(out: *mut af_array, input: af_array) -> c_int;

    fn af_sparse_get_col_idx(out: *mut af_array, input: af_array) -> c_int;

    fn af_sparse_get_nnz(out: *mut dim_t, input: af_array) -> c_int;

    fn af_sparse_get_storage(out: *mut c_uint, input: af_array) -> c_int;
}

/// Create sprase matrix from arrays
///
/// This function converts [Array](./struct.Array.html) of `values` into sparse array
/// of `format` sparse format using arrays `row_indices` and `col_indices`.
///
/// # Parameters
///
/// - `rows` is the number of rows in the dense matrix
/// - `cols` is the number of columns in the dense matrix
/// - `values` is the \ref af::array containing the non-zero elements
///   `of the matrix
/// - `row_indices` is the row indices for the sparse array
/// - `col_indices` is the column indices for the sparse array
/// - `format` is the storage format of the sparse array
///
/// # Return Values
///
/// Array with data in given sparse format
///
/// # Note
///
/// This function only uses references of the input arrays to create the
/// sparse data structure and does not perform deep copies.
pub fn sparse<T>(
    rows: u64,
    cols: u64,
    values: &Array<T>,
    row_indices: &Array<i32>,
    col_indices: &Array<i32>,
    format: SparseFormat,
) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_create_sparse_array(
            &mut temp as *mut af_array,
            rows as dim_t,
            cols as dim_t,
            values.get(),
            row_indices.get(),
            col_indices.get(),
            format as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Create sprase matrix from data on host memory
///
/// This function converts host array `values` into sparse array of `format` sparse
/// format using host arrays `row_indices` and `col_indices`.
///
/// # Parameters
///
/// - `rows` is the number of rows in the dense matrix
/// - `cols` is the number of columns in the dense matrix
/// - `nzz` is the number of non zero elements in the dense matrix
/// - `values` is the \ref af::array containing the non-zero elements
///   `of the matrix
/// - `row_indices` is the row indices for the sparse array
/// - `col_indices` is the column indices for the sparse array
/// - `format` is the storage format of the sparse array
///
/// # Return Values
///
/// Array with data in given sparse format
///
/// # Note
///
/// The rules for deep copy/shallow copy/reference are the same as for creating a
/// regular [Array](./struct.Array.html).
pub fn sparse_from_host<T>(
    rows: u64,
    cols: u64,
    nzz: u64,
    values: &[T],
    row_indices: &[i32],
    col_indices: &[i32],
    format: SparseFormat,
) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    let aftype = T::get_af_dtype();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_create_sparse_array_from_ptr(
            &mut temp as *mut af_array,
            rows as dim_t,
            cols as dim_t,
            nzz as dim_t,
            values.as_ptr() as *const c_void,
            row_indices.as_ptr() as *const c_int,
            col_indices.as_ptr() as *const c_int,
            aftype as c_uint,
            format as c_uint,
            1,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Convert dense array to sparse array
///
/// # Parameters
///
/// - `dense` is the dense format array
/// - `format` is the target sparse format
///
/// # Return Values
///
/// Sparse Array
pub fn sparse_from_dense<T>(dense: &Array<T>, format: SparseFormat) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_create_sparse_array_from_dense(
            &mut temp as *mut af_array,
            dense.get(),
            format as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Convert between sparse formats
///
/// # Parameters
///
/// - `input` is the input sparse array
/// - `format` is the target sparse format
///
/// # Return Values
///
/// Sparse Array in targe sparse format.
pub fn sparse_convert_to<T>(input: &Array<T>, format: SparseFormat) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val =
            af_sparse_convert_to(&mut temp as *mut af_array, input.get(), format as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Convert sparse array to dense array
///
/// # Parameters
///
/// - `input` is the sparse array
///
/// # Return Values
///
/// Dense Array
pub fn sparse_to_dense<T>(input: &Array<T>) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_sparse_to_dense(&mut temp as *mut af_array, input.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Get sparse Array information
///
/// # Parameters
///
/// - `input` is the sparse array
///
/// # Return Values
///
/// A tuple of values, row indices, column indices Arrays and SparseFormat enum.
pub fn sparse_get_info<T>(input: &Array<T>) -> (Array<T>, Array<i32>, Array<i32>, SparseFormat)
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut val: af_array = std::ptr::null_mut();
        let mut row: af_array = std::ptr::null_mut();
        let mut col: af_array = std::ptr::null_mut();
        let mut stype: u32 = 0;
        let err_val = af_sparse_get_info(
            &mut val as *mut af_array,
            &mut row as *mut af_array,
            &mut col as *mut af_array,
            &mut stype as *mut c_uint,
            input.get(),
        );
        HANDLE_ERROR(AfError::from(err_val));
        (
            val.into(),
            row.into(),
            col.into(),
            SparseFormat::from(stype),
        )
    }
}

/// Get values of sparse Array
///
/// # Parameters
///
/// - `input` is the sparse array
///
/// # Return Values
///
/// Sparse array values
pub fn sparse_get_values<T>(input: &Array<T>) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut val: af_array = std::ptr::null_mut();
        let err_val = af_sparse_get_values(&mut val as *mut af_array, input.get());
        HANDLE_ERROR(AfError::from(err_val));
        val.into()
    }
}

/// Get row indices Array
///
/// # Parameters
///
/// - `input` is the sparse array
///
/// # Return Values
///
/// Array with row indices values of sparse Array
pub fn sparse_get_row_indices<T>(input: &Array<T>) -> Array<i32>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut val: af_array = std::ptr::null_mut();
        let err_val = af_sparse_get_row_idx(&mut val as *mut af_array, input.get());
        HANDLE_ERROR(AfError::from(err_val));
        val.into()
    }
}

/// Get cololumn indices Array
///
/// # Parameters
///
/// - `input` is the sparse array
///
/// # Return Values
///
/// Array with coloumn indices values of sparse Array
pub fn sparse_get_col_indices<T>(input: &Array<T>) -> Array<i32>
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut val: af_array = std::ptr::null_mut();
        let err_val = af_sparse_get_col_idx(&mut val as *mut af_array, input.get());
        HANDLE_ERROR(AfError::from(err_val));
        val.into()
    }
}

/// Get number of non-zero elements in sparse array
///
/// # Parameters
///
/// - `input` is the sparse array
///
/// # Return Values
///
/// Number of non-zero elements of sparse Array
pub fn sparse_get_nnz<T: HasAfEnum>(input: &Array<T>) -> i64 {
    let mut count: i64 = 0;
    unsafe {
        let err_val = af_sparse_get_nnz(&mut count as *mut dim_t, input.get());
        HANDLE_ERROR(AfError::from(err_val));
    }
    count
}

/// Get sparse format
///
/// # Parameters
///
/// - `input` is the sparse array
///
/// # Return Values
///
/// Sparse array format
pub fn sparse_get_format<T: HasAfEnum>(input: &Array<T>) -> SparseFormat {
    let mut stype: u32 = 0;
    unsafe {
        let err_val = af_sparse_get_storage(&mut stype as *mut c_uint, input.get());
        HANDLE_ERROR(AfError::from(err_val));
    }
    SparseFormat::from(stype)
}
