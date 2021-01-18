use super::array::Array;
use super::defines::{AfError, BorderType};
use super::dim4::Dim4;
use super::error::HANDLE_ERROR;
use super::util::{af_array, c32, c64, dim_t, u64_t, HasAfEnum};

use libc::{c_double, c_int, c_uint};
use std::option::Option;
use std::vec::Vec;

extern "C" {
    fn af_constant(
        out: *mut af_array,
        val: c_double,
        ndims: c_uint,
        dims: *const dim_t,
        afdtype: c_uint,
    ) -> c_int;

    fn af_constant_complex(
        out: *mut af_array,
        real: c_double,
        imag: c_double,
        ndims: c_uint,
        dims: *const dim_t,
        afdtype: c_uint,
    ) -> c_int;

    fn af_constant_long(out: *mut af_array, val: dim_t, ndims: c_uint, dims: *const dim_t)
        -> c_int;

    fn af_constant_ulong(
        out: *mut af_array,
        val: u64_t,
        ndims: c_uint,
        dims: *const dim_t,
    ) -> c_int;

    fn af_range(
        out: *mut af_array,
        ndims: c_uint,
        dims: *const dim_t,
        seq_dim: c_int,
        afdtype: c_uint,
    ) -> c_int;

    fn af_iota(
        out: *mut af_array,
        ndims: c_uint,
        dims: *const dim_t,
        t_ndims: c_uint,
        tdims: *const dim_t,
        afdtype: c_uint,
    ) -> c_int;

    fn af_identity(out: *mut af_array, ndims: c_uint, dims: *const dim_t, afdtype: c_uint)
        -> c_int;
    fn af_diag_create(out: *mut af_array, arr: af_array, num: c_int) -> c_int;
    fn af_diag_extract(out: *mut af_array, arr: af_array, num: c_int) -> c_int;
    fn af_join(out: *mut af_array, dim: c_int, first: af_array, second: af_array) -> c_int;
    fn af_join_many(
        out: *mut af_array,
        dim: c_int,
        n_arrays: c_uint,
        inpts: *const af_array,
    ) -> c_int;

    fn af_tile(
        out: *mut af_array,
        arr: af_array,
        x: c_uint,
        y: c_uint,
        z: c_uint,
        w: c_uint,
    ) -> c_int;
    fn af_reorder(
        o: *mut af_array,
        a: af_array,
        x: c_uint,
        y: c_uint,
        z: c_uint,
        w: c_uint,
    ) -> c_int;
    fn af_shift(o: *mut af_array, a: af_array, x: c_int, y: c_int, z: c_int, w: c_int) -> c_int;
    fn af_moddims(out: *mut af_array, arr: af_array, ndims: c_uint, dims: *const dim_t) -> c_int;

    fn af_flat(out: *mut af_array, arr: af_array) -> c_int;
    fn af_flip(out: *mut af_array, arr: af_array, dim: c_uint) -> c_int;
    fn af_lower(out: *mut af_array, arr: af_array, is_unit_diag: bool) -> c_int;
    fn af_upper(out: *mut af_array, arr: af_array, is_unit_diag: bool) -> c_int;

    fn af_select(out: *mut af_array, cond: af_array, a: af_array, b: af_array) -> c_int;
    fn af_select_scalar_l(out: *mut af_array, cond: af_array, a: c_double, b: af_array) -> c_int;
    fn af_select_scalar_r(out: *mut af_array, cond: af_array, a: af_array, b: c_double) -> c_int;

    fn af_replace(a: *mut af_array, cond: af_array, b: af_array) -> c_int;
    fn af_replace_scalar(a: *mut af_array, cond: af_array, b: c_double) -> c_int;

    fn af_pad(
        out: *mut af_array,
        input: af_array,
        begin_ndims: c_uint,
        begin_dims: *const dim_t,
        end_ndims: c_uint,
        end_dims: *const dim_t,
        pad_fill_type: c_uint,
    ) -> c_int;
}

/// Type Trait to generate a constant [Array](./struct.Array.html) of given size
///
/// Internally, ConstGenerator trait is implemented by following types.
///
/// - f32
/// - f64
/// - num::Complex\<f32\>
/// - num::Complex\<f64\>
/// - bool
/// - i32
/// - u32
/// - u8
/// - i64
/// - u64
/// - i16
/// - u16
///
pub trait ConstGenerator: HasAfEnum {
    /// The type of Array<T> object returned by generate function
    type OutType: HasAfEnum;

    /// Create an Array of `dims` size from scalar value `self`.
    ///
    /// # Parameters
    ///
    /// - `dims` are the dimensions of the output constant [Array](./struct.Array.html)
    fn generate(&self, dims: Dim4) -> Array<Self::OutType>;
}

impl ConstGenerator for i64 {
    type OutType = i64;

    fn generate(&self, dims: Dim4) -> Array<Self::OutType> {
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_constant_long(
                &mut temp as *mut af_array,
                *self,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as *const dim_t,
            );
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }
}

impl ConstGenerator for u64 {
    type OutType = u64;

    fn generate(&self, dims: Dim4) -> Array<Self::OutType> {
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_constant_ulong(
                &mut temp as *mut af_array,
                *self,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as *const dim_t,
            );
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }
}

impl ConstGenerator for c32 {
    type OutType = c32;

    fn generate(&self, dims: Dim4) -> Array<Self::OutType> {
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_constant_complex(
                &mut temp as *mut af_array,
                (*self).re as c_double,
                (*self).im as c_double,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as *const dim_t,
                1,
            );
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }
}

impl ConstGenerator for c64 {
    type OutType = c64;

    fn generate(&self, dims: Dim4) -> Array<Self::OutType> {
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_constant_complex(
                &mut temp as *mut af_array,
                (*self).re as c_double,
                (*self).im as c_double,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as *const dim_t,
                3,
            );
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }
}

impl ConstGenerator for bool {
    type OutType = bool;

    fn generate(&self, dims: Dim4) -> Array<Self::OutType> {
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_constant(
                &mut temp as *mut af_array,
                *self as c_int as c_double,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as *const dim_t,
                4,
            );
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }
}

macro_rules! cnst {
    ($rust_type:ty, $ffi_type:expr) => {
        impl ConstGenerator for $rust_type {
            type OutType = $rust_type;

            fn generate(&self, dims: Dim4) -> Array<Self::OutType> {
                unsafe {
                    let mut temp: af_array = std::ptr::null_mut();
                    let err_val = af_constant(
                        &mut temp as *mut af_array,
                        *self as c_double,
                        dims.ndims() as c_uint,
                        dims.get().as_ptr() as *const dim_t,
                        $ffi_type,
                    );
                    HANDLE_ERROR(AfError::from(err_val));
                    temp.into()
                }
            }
        }
    };
}

cnst!(f32, 0);
cnst!(f64, 2);
cnst!(i32, 5);
cnst!(u32, 6);
cnst!(u8, 7);
cnst!(i16, 10);
cnst!(u16, 11);

/// Create an Array with constant value
///
/// The trait ConstGenerator has been defined internally for the following types:
///
/// - i64
/// - u64
/// - num::Complex\<f32\> a.k.a c32
/// - num::Complex\<f64\> a.k.a c64
/// - f32
/// - f64
/// - i32
/// - u32
/// - u8
/// - i16
/// - u16
///
/// # Parameters
///
/// - `cnst` is the constant value to be filled in the Array
/// - `dims` is the size of the constant Array
///
/// # Return Values
///
/// An Array of given dimensions with constant value
pub fn constant<T>(cnst: T, dims: Dim4) -> Array<T>
where
    T: ConstGenerator<OutType = T>,
{
    cnst.generate(dims)
}

/// Create a Range of values
///
/// Creates an array with [0, n] values along the `seq_dim` which is tiled across other dimensions.
///
/// # Parameters
///
/// - `dims` is the size of Array
/// - `seq_dim` is the dimension along which range values are populated, all values along other
/// dimensions are just repeated
///
/// # Return Values
/// Array
pub fn range<T: HasAfEnum>(dims: Dim4, seq_dim: i32) -> Array<T> {
    let aftype = T::get_af_dtype();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_range(
            &mut temp as *mut af_array,
            dims.ndims() as c_uint,
            dims.get().as_ptr() as *const dim_t,
            seq_dim as c_int,
            aftype as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Create a range of values
///
/// Create an sequence [0, dims.elements() - 1] and modify to specified dimensions dims and then tile it according to tile_dims.
///
/// # Parameters
///
/// - `dims` is the dimensions of the sequence to be generated
/// - `tdims` is the number of repitions of the unit dimensions
///
/// # Return Values
///
/// Array
pub fn iota<T: HasAfEnum>(dims: Dim4, tdims: Dim4) -> Array<T> {
    let aftype = T::get_af_dtype();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_iota(
            &mut temp as *mut af_array,
            dims.ndims() as c_uint,
            dims.get().as_ptr() as *const dim_t,
            tdims.ndims() as c_uint,
            tdims.get().as_ptr() as *const dim_t,
            aftype as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Create an identity array with 1's in diagonal
///
/// # Parameters
///
/// - `dims` is the output Array dimensions
///
/// # Return Values
///
/// Identity matrix
pub fn identity<T: HasAfEnum>(dims: Dim4) -> Array<T> {
    let aftype = T::get_af_dtype();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_identity(
            &mut temp as *mut af_array,
            dims.ndims() as c_uint,
            dims.get().as_ptr() as *const dim_t,
            aftype as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Create a diagonal matrix
///
/// # Parameters
///
/// - `input` is the input Array
/// - `dim` is the diagonal index relative to principal diagonal where values from input Array are
/// to be placed
///
/// # Return Values
///
/// An Array with values as a diagonal Matrix
pub fn diag_create<T>(input: &Array<T>, dim: i32) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_diag_create(&mut temp as *mut af_array, input.get(), dim);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Extract diagonal from a given Matrix
///
/// # Parameters
///
/// - `input` is the input Matrix
/// - `dim` is the index of the diagonal that has to be extracted from the input Matrix
///
/// # Return Values
///
/// An Array with values of the diagonal from input Array
pub fn diag_extract<T>(input: &Array<T>, dim: i32) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_diag_extract(&mut temp as *mut af_array, input.get(), dim);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Join two arrays
///
/// # Parameters
///
/// - `dim` is the dimension along which the concatenation has to be done
/// - `first` is the Array that comes first in the concatenation
/// - `second` is the Array that comes last in the concatenation
///
/// # Return Values
///
/// Concatenated Array
pub fn join<T>(dim: i32, first: &Array<T>, second: &Array<T>) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_join(&mut temp as *mut af_array, dim, first.get(), second.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Join multiple arrays
///
/// # Parameters
///
/// - `dim` is the dimension along which the concatenation has to be done
/// - `inputs` is the vector of Arrays that has to be concatenated
///
/// # Return Values
///
/// Concatenated Array
pub fn join_many<T>(dim: i32, inputs: Vec<&Array<T>>) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut v = Vec::new();
        for i in inputs {
            v.push(i.get());
        }
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_join_many(
            &mut temp as *mut af_array,
            dim,
            v.len() as u32,
            v.as_ptr() as *const af_array,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Tile the input array along specified dimension
///
/// Tile essentially creates copies of data along each dimension.
/// The number of copies created is provided by the user on per
/// axis basis using [Dim4](./struct.dim4.html)
///
///# Parameters
///
/// - `input` is the input Array
/// - `dims` is the target(output) dimensions
///
///# Return Values
///
/// Tiled input array as per the tiling dimensions provided
pub fn tile<T>(input: &Array<T>, dims: Dim4) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_tile(
            &mut temp as *mut af_array,
            input.get() as af_array,
            dims[0] as c_uint,
            dims[1] as c_uint,
            dims[2] as c_uint,
            dims[3] as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Reorder the array according to the new specified axes
///
/// Exchanges data within an array such that the requested change in axes is
/// satisfied. The linear ordering of data within the array is preserved.
///
/// The default order of axes in ArrayFire is [0 1 2 3] i.e. axis with smallest
/// distance between adjacent elements followed by next smallest distance axis and
/// so on. See [examples](#examples) to have a basic idea of how data is re-ordered.
///
///# Parameters
///
/// - `input` is the input Array
/// - `new_axis0` is the new first axis for output
/// - `new_axis1` is the new second axis for output
/// - `next_axes` is the new axes order for output
///
///# Return Values
///
/// Array with data reordered as per the new axes order
///
///# Examples
///
/// ```rust
/// use arrayfire::{Array, Dim4, print, randu, reorder_v2};
/// let a  = randu::<f32>(Dim4::new(&[5, 3, 1, 1]));
/// let b  = reorder_v2(&a, 1, 0, None);
/// print(&a);
///
/// // [5 3 1 1]
/// //  0.8104     0.2990     0.3014
/// //  0.6913     0.2802     0.6938
/// //  0.7821     0.1480     0.3513
/// //  0.3054     0.1330     0.7176
/// //  0.1673     0.4696     0.1181
///
/// print(&b);
/// // [3 5 1 1]
/// //     0.8104     0.6913     0.7821     0.3054     0.1673
/// //     0.2990     0.2802     0.1480     0.1330     0.4696
/// //     0.3014     0.6938     0.3513     0.7176     0.1181
///
/// let c  = reorder_v2(&a, 2, 0, Some(vec![1]));
/// print(&c);
///
/// // [1 5 3 1]
/// //  0.8104     0.6913     0.7821     0.3054     0.1673
/// //
/// //  0.2990     0.2802     0.1480     0.1330     0.4696
/// //
/// //  0.3014     0.6938     0.3513     0.7176     0.1181
/// ```
pub fn reorder_v2<T>(
    input: &Array<T>,
    new_axis0: u64,
    new_axis1: u64,
    next_axes: Option<Vec<u64>>,
) -> Array<T>
where
    T: HasAfEnum,
{
    let mut new_axes = [0, 1, 2, 3];
    new_axes[0] = new_axis0;
    new_axes[1] = new_axis1;
    match next_axes {
        Some(left_over_new_axes) => {
            // At the moment of writing this comment, ArrayFire could
            // handle only a maximum of 4 dimensions. Hence, excluding
            // the two explicit axes arguments to this function, a maximum
            // of only two more axes can be provided. Hence the below condition.
            assert!(left_over_new_axes.len() <= 2);

            new_axes[2..(left_over_new_axes.len() + 2)].clone_from_slice(&left_over_new_axes[..]);
        }
        None => {
            let left_over_indices: Vec<usize> = (2..4).collect();
            for a_idx in left_over_indices {
                new_axes[a_idx] = a_idx as u64;
            }
        }
    };

    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_reorder(
            &mut temp as *mut af_array,
            input.get() as af_array,
            new_axes[0] as c_uint,
            new_axes[1] as c_uint,
            new_axes[2] as c_uint,
            new_axes[3] as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Reorder the array in specified order
///
/// The default order of axes in ArrayFire is axis with smallest distance
/// between adjacent elements towards an axis with highest distance between
/// adjacent elements.
///
///# Parameters
///
/// - `input` is the input Array
/// - `dims` is the target(output) dimensions
///
///# Return Values
///
/// Array with data reordered as per the new axes order
#[deprecated(since = "3.6.3", note = "Please use new reorder API")]
pub fn reorder<T>(input: &Array<T>, dims: Dim4) -> Array<T>
where
    T: HasAfEnum,
{
    reorder_v2(input, dims[0], dims[1], Some(vec![dims[2], dims[3]]))
}

///"Circular shift of values along specified dimension
///
///# Parameters
///
/// - `input` is the input Array
/// - `offsets` is 4-value tuple that specifies the shift along respective dimension
///
///# Return Values
///
/// An Array with shifted data.
///
///# Examples
///
/// ```rust
/// use arrayfire::{Array, Dim4, print, randu, shift};
/// let a  = randu::<f32>(Dim4::new(&[5, 1, 1, 1]));
/// let _a = shift(&a, &[-1i32, 1 , 1, 1]); //shift data one step backward
/// let a_ = shift(&a, &[ 1i32, 1 , 1, 1]); //shift data one step forward
/// print(& a);
/// print(&_a);
/// print(&a_);
/// ```
pub fn shift<T>(input: &Array<T>, offsets: &[i32; 4]) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_shift(
            &mut temp as *mut af_array,
            input.get(),
            offsets[0],
            offsets[1],
            offsets[2],
            offsets[3],
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Change the shape of the Array
///
/// # Parameters
///
/// - `input` is the input Array
/// - `dims` is the new dimensions to which the input Array is reshaped to
///
/// # Return Values
/// Reshaped Array
pub fn moddims<T>(input: &Array<T>, dims: Dim4) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_moddims(
            &mut temp as *mut af_array,
            input.get(),
            dims.ndims() as c_uint,
            dims.get().as_ptr() as *const dim_t,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Flatten the multidimensional Array to an 1D Array
pub fn flat<T>(input: &Array<T>) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_flat(&mut temp as *mut af_array, input.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Flip the Array
///
/// # Parameters
///
/// - `input` is the Array to be flipped
/// - `dim` is the dimension along which the flip has to happen
///
/// # Return Values
///
/// Flipped Array
pub fn flip<T>(input: &Array<T>, dim: u32) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_flip(&mut temp as *mut af_array, input.get(), dim);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Create lower triangular matrix
///
/// # Parameters
///
/// - `input` is the input Array
/// - `is_unit_diag` dictates if the output Array should have 1's along diagonal
///
/// # Return Values
/// Array
pub fn lower<T>(input: &Array<T>, is_unit_diag: bool) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_lower(&mut temp as *mut af_array, input.get(), is_unit_diag);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Create upper triangular matrix
///
/// # Parameters
///
/// - `input` is the input Array
/// - `is_unit_diag` dictates if the output Array should have 1's along diagonal
///
/// # Return Values
/// Array
pub fn upper<T>(input: &Array<T>, is_unit_diag: bool) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_upper(&mut temp as *mut af_array, input.get(), is_unit_diag);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Element wise conditional operator for Arrays
///
/// This function does the C-equivalent of the following statement, except that the operation
/// happens on a GPU for all elements simultaneously.
///
/// ```text
/// c = cond ? a : b; /// where cond, a & b are all objects of type Array
/// ```
///
/// # Parameters
///
/// - `a` is the Array whose element will be assigned to output if corresponding element in `cond` Array is
/// `True`
/// - `cond` is the Array with boolean values
/// - `b` is the Array whose element will be assigned to output if corresponding element in `cond` Array is
/// `False`
///
/// # Return Values
///
/// An Array
pub fn select<T>(a: &Array<T>, cond: &Array<bool>, b: &Array<T>) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_select(&mut temp as *mut af_array, cond.get(), a.get(), b.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Element wise conditional operator for Arrays
///
/// This function does the C-equivalent of the following statement, except that the operation
/// happens on a GPU for all elements simultaneously.
///
/// ```text
/// c = cond ? a : b; /// where  a is a scalar(f64) and b is Array
/// ```
///
/// # Parameters
///
/// - `a` is the scalar that is assigned to output if corresponding element in `cond` Array is
/// `True`
/// - `cond` is the Array with conditional values
/// - `b` is the Array whose element will be assigned to output if corresponding element in `cond` Array is
/// `False`
///
/// # Return Values
///
/// An Array
pub fn selectl<T>(a: f64, cond: &Array<bool>, b: &Array<T>) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_select_scalar_l(&mut temp as *mut af_array, cond.get(), a, b.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Element wise conditional operator for Arrays
///
/// This function does the C-equivalent of the following statement, except that the operation
/// happens on a GPU for all elements simultaneously.
///
/// ```text
/// c = cond ? a : b; /// where a is Array and b is a scalar(f64)
/// ```
///
/// # Parameters
///
/// - `a` is the Array whose element will be assigned to output if corresponding element in `cond` Array is
/// `True`
/// - `cond` is the Array with conditional values
/// - `b` is the scalar that is assigned to output if corresponding element in `cond` Array is
/// `False`
///
/// # Return Values
///
/// An Array
pub fn selectr<T>(a: &Array<T>, cond: &Array<bool>, b: f64) -> Array<T>
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_select_scalar_r(&mut temp as *mut af_array, cond.get(), a.get(), b);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Inplace replace in Array based on a condition
///
/// This function does the C-equivalent of the following statement, except that the operation
/// happens on a GPU for all elements simultaneously.
///
/// ```text
/// a = cond ? a : b; /// where cond, a & b are all objects of type Array
/// ```
///
/// # Parameters
///
/// - `a` is the Array whose element will be replaced with element from `b` if corresponding element in `cond` Array is `True`
/// - `cond` is the Array with conditional values
/// - `b` is the Array whose element will replace the element in output if corresponding element in `cond` Array is
/// `False`
///
/// # Return Values
///
/// None
pub fn replace<T>(a: &mut Array<T>, cond: &Array<bool>, b: &Array<T>)
where
    T: HasAfEnum,
{
    unsafe {
        let err_val = af_replace(a.get() as *mut af_array, cond.get(), b.get());
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Inplace replace in Array based on a condition
///
/// This function does the C-equivalent of the following statement, except that the operation
/// happens on a GPU for all elements simultaneously.
///
/// ```text
/// a = cond ? a : b; /// where cond, a are Arrays and b is scalar(f64)
/// ```
///
/// # Parameters
///
/// - `a` is the Array whose element will be replaced with element from `b` if corresponding element in `cond` Array is `True`
/// - `cond` is the Array with conditional values
/// - `b` is the scalar that will replace the element in output if corresponding element in `cond` Array is
/// `False`
///
/// # Return Values
///
/// None
pub fn replace_scalar<T>(a: &mut Array<T>, cond: &Array<bool>, b: f64)
where
    T: HasAfEnum,
{
    unsafe {
        let err_val = af_replace_scalar(a.get() as *mut af_array, cond.get(), b);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Pad input Array along borders
///
/// # Parameters
///
/// - `input` is the input array to be padded
/// - `begin` is padding size before first element along a given dimension
/// - `end` is padding size after the last element along a given dimension
/// - `fill_type` indicates what values should be used to fill padded regions
///
/// # Return Values
///
/// Padded Array
pub fn pad<T: HasAfEnum>(
    input: &Array<T>,
    begin: Dim4,
    end: Dim4,
    fill_type: BorderType,
) -> Array<T> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_pad(
            &mut temp as *mut af_array,
            input.get(),
            4,
            begin.get().as_ptr() as *const dim_t,
            4,
            end.get().as_ptr() as *const dim_t,
            fill_type as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

#[cfg(test)]
mod tests {
    use super::reorder_v2;

    use super::super::defines::BorderType;
    use super::super::device::set_device;
    use super::super::random::randu;
    use super::pad;

    use crate::dim4;

    #[test]
    fn check_reorder_api() {
        set_device(0);
        let a = randu::<f32>(dim4!(4, 5, 2, 3));

        let _transposed = reorder_v2(&a, 1, 0, None);
        let _swap_0_2 = reorder_v2(&a, 2, 1, Some(vec![0]));
        let _swap_1_2 = reorder_v2(&a, 0, 2, Some(vec![1]));
        let _swap_0_3 = reorder_v2(&a, 3, 1, Some(vec![2, 0]));
    }

    #[test]
    fn check_pad_api() {
        set_device(0);
        let a = randu::<f32>(dim4![3, 3]);
        let begin_dims = dim4!(0, 0, 0, 0);
        let end_dims = dim4!(2, 2, 0, 0);
        let _padded = pad(&a, begin_dims, end_dims, BorderType::ZERO);
    }
}
