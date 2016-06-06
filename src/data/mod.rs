extern crate libc;
extern crate num;

use array::Array;
use dim4::Dim4;
use defines::AfError;
use error::HANDLE_ERROR;
use self::libc::{uint8_t, c_int, c_uint, c_double};
use self::num::Complex;
use util::HasAfEnum;
use std::vec::Vec;

type MutAfArray = *mut self::libc::c_longlong;
type MutDouble  = *mut self::libc::c_double;
type MutUint    = *mut self::libc::c_uint;
type AfArray    = self::libc::c_longlong;
type DimT       = self::libc::c_longlong;
type Intl       = self::libc::c_longlong;
type Uintl      = self::libc::c_ulonglong;

#[allow(dead_code)]
extern {
    fn af_constant(out: MutAfArray, val: c_double,
                   ndims: c_uint, dims: *const DimT, afdtype: c_int) -> c_int;

    fn af_constant_complex(out: MutAfArray, real: c_double, imag: c_double,
                           ndims: c_uint, dims: *const DimT, afdtype: c_int) -> c_int;

    fn af_constant_long(out: MutAfArray, val: Intl,
                        ndims: c_uint, dims: *const DimT) -> c_int;

    fn af_constant_ulong(out: MutAfArray, val: Uintl,
                         ndims: c_uint, dims: *const DimT) -> c_int;

    fn af_range(out: MutAfArray, ndims: c_uint, dims: *const DimT,
                seq_dims: c_int, afdtype: uint8_t) -> c_int;

    fn af_iota(out: MutAfArray, ndims: c_uint, dims: *const DimT,
               t_ndims: c_uint, tdims: *const DimT, afdtype: uint8_t) -> c_int;

    fn af_randu(out: MutAfArray, ndims: c_uint, dims: *const DimT, afdtype: uint8_t) -> c_int;
    fn af_randn(out: MutAfArray, ndims: c_uint, dims: *const DimT, afdtype: uint8_t) -> c_int;

    fn af_set_seed(seed: Uintl) -> c_int;
    fn af_get_seed(seed: *mut Uintl) -> c_int;

    fn af_identity(out: MutAfArray, ndims: c_uint, dims: *const DimT, afdtype: uint8_t) -> c_int;
    fn af_diag_create(out: MutAfArray, arr: AfArray, num: c_int) -> c_int;
    fn af_diag_extract(out: MutAfArray, arr: AfArray, num: c_int) -> c_int;
    fn af_join(out: MutAfArray, dim: c_int, first: AfArray, second: AfArray) -> c_int;
    fn af_join_many(out: MutAfArray, dim: c_int, n_arrays: c_uint, inpts: *const AfArray) -> c_int;

    fn af_tile(out: MutAfArray, arr: AfArray, x: c_uint, y: c_uint, z: c_uint, w: c_uint) -> c_int;
    fn af_reorder(o: MutAfArray, a: AfArray, x: c_uint, y: c_uint, z: c_uint, w: c_uint) -> c_int;
    fn af_shift(o: MutAfArray, a: AfArray, x: c_uint, y: c_uint, z: c_uint, w: c_uint) -> c_int;
    fn af_moddims(out: MutAfArray, arr: AfArray, ndims: c_uint, dims: *const DimT) -> c_int;

    fn af_flat(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_flip(out: MutAfArray, arr: AfArray, dim: c_uint) -> c_int;
    fn af_lower(out: MutAfArray, arr: AfArray, is_unit_diag: c_int) -> c_int;
    fn af_upper(out: MutAfArray, arr: AfArray, is_unit_diag: c_int) -> c_int;

    fn af_select(out: MutAfArray, cond: AfArray, a: AfArray, b: AfArray) -> c_int;
    fn af_select_scalar_l(out: MutAfArray, cond: AfArray, a: c_double, b: AfArray) -> c_int;
    fn af_select_scalar_r(out: MutAfArray, cond: AfArray, a: AfArray, b: c_double) -> c_int;

    fn af_replace(a: AfArray, cond: AfArray, b: AfArray) -> c_int;
    fn af_replace_scalar(a: AfArray, cond: AfArray, b: c_double) -> c_int;
}

pub trait ConstGenerator {
    fn generate(&self, dims: Dim4) -> Array;
}

#[allow(unused_mut)]
impl ConstGenerator for i64 {
    fn generate(&self, dims: Dim4) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_constant_long(&mut temp as MutAfArray, *self as Intl,
                                           dims.ndims() as c_uint,
                                           dims.get().as_ptr() as *const DimT);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for u64 {
    fn generate(&self, dims: Dim4) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_constant_ulong(&mut temp as MutAfArray, *self as Uintl,
                                            dims.ndims() as c_uint,
                                            dims.get().as_ptr() as *const DimT);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for Complex<f32> {
    fn generate(&self, dims: Dim4) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_constant_complex(&mut temp as MutAfArray,
                                              (*self).re as c_double, (*self).im as c_double,
                                              dims.ndims() as c_uint,
                                              dims.get().as_ptr() as *const DimT, 1);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for Complex<f64> {
    fn generate(&self, dims: Dim4) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_constant_complex(&mut temp as MutAfArray,
                                              (*self).re as c_double, (*self).im as c_double,
                                              dims.ndims() as c_uint,
                                              dims.get().as_ptr() as *const DimT, 3);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for bool {
    fn generate(&self, dims: Dim4) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_constant(&mut temp as MutAfArray, *self as c_int as c_double,
                                      dims.ndims() as c_uint,
                                      dims.get().as_ptr() as *const DimT, 4);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    }
}

macro_rules! cnst {
    ($rust_type:ty, $ffi_type:expr) => (
        #[allow(unused_mut)]
        impl ConstGenerator for $rust_type {
            fn generate(&self, dims: Dim4) -> Array {
                unsafe {
                    let mut temp: i64 = 0;
                    let err_val = af_constant(&mut temp as MutAfArray, *self as c_double,
                                              dims.ndims() as c_uint,
                                              dims.get().as_ptr() as *const DimT, $ffi_type);
                    HANDLE_ERROR(AfError::from(err_val));
                    Array::from(temp)
                }
            }
        }
    )
}

cnst!(f32 ,  0);
cnst!(f64 ,  2);
cnst!(i32 ,  5);
cnst!(u32 ,  6);
cnst!(u8  ,  7);
cnst!(i16 , 10);
cnst!(u16 , 11);


/// Create an Array with constant value
///
/// The trait ConstGenerator has been defined internally for the following types:
///
/// - i64
/// - u64
/// - num::Complex\<f32\>
/// - num::Complex\<f64\>
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
pub fn constant<T : ConstGenerator>(cnst: T, dims: Dim4) -> Array {
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
#[allow(unused_mut)]
pub fn range<T: HasAfEnum>(dims: Dim4, seq_dim: i32) -> Array {
    unsafe {
        let aftype = T::get_af_dtype();
        let mut temp: i64 = 0;
        let err_val = af_range(&mut temp as MutAfArray,
                              dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                              seq_dim as c_int, aftype as uint8_t);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
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
#[allow(unused_mut)]
pub fn iota<T: HasAfEnum>(dims: Dim4, tdims: Dim4) -> Array {
    unsafe {
        let aftype = T::get_af_dtype();
        let mut temp: i64 = 0;
        let err_val =af_iota(&mut temp as MutAfArray,
                             dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                             tdims.ndims() as c_uint, tdims.get().as_ptr() as *const DimT,
                             aftype as uint8_t);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Set seed for random number generation
pub fn set_seed(seed: u64) {
    unsafe {
        let err_val = af_set_seed(seed as Uintl);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Get the seed of random number generator
#[allow(unused_mut)]
pub fn get_seed() -> u64 {
    unsafe {
        let mut temp: u64 = 0;
        let err_val = af_get_seed(&mut temp as *mut Uintl);
        HANDLE_ERROR(AfError::from(err_val));
        temp
    }
}

macro_rules! data_gen_def {
    ($doc_str: expr, $fn_name:ident, $ffi_name: ident) => (
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `dims` is the output dimensions
        ///
        ///# Return Values
        ///
        /// An Array with modified data.
        #[allow(unused_mut)]
        pub fn $fn_name<T: HasAfEnum>(dims: Dim4) -> Array {
            unsafe {
                let aftype = T::get_af_dtype();
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                        dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                                        aftype as uint8_t);
                HANDLE_ERROR(AfError::from(err_val));
                Array::from(temp)
            }
        }
    )
}

data_gen_def!("Create random numbers from uniform distribution", randu, af_randu);
data_gen_def!("Create random numbers from normal distribution", randn, af_randn);
data_gen_def!("Create an identity array with 1's in diagonal", identity, af_identity);

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
#[allow(unused_mut)]
pub fn diag_create(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_diag_create(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
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
#[allow(unused_mut)]
pub fn diag_extract(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_diag_extract(&mut temp as MutAfArray,
                                      input.get() as AfArray, dim as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
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
#[allow(unused_mut)]
pub fn join(dim: i32, first: &Array, second: &Array) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_join(&mut temp as MutAfArray, dim as c_int,
                              first.get() as AfArray, second.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
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
#[allow(unused_mut)]
pub fn join_many(dim: i32, inputs: Vec<&Array>) -> Array {
    unsafe {
        let mut v = Vec::new();
        for i in inputs {
            v.push(i.get());
        }
        let mut temp: i64 = 0;
        let err_val = af_join_many(&mut temp as MutAfArray, dim as c_int,
                                   v.len() as c_uint, v.as_ptr() as *const AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

macro_rules! data_func_def {
    ($doc_str: expr, $fn_name:ident, $ffi_name: ident) => (
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `input` is the input Array
        /// - `dims` is the target(output) dimensions
        ///
        ///# Return Values
        ///
        /// An Array with modified data.
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, dims: Dim4) -> Array {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray, input.get() as AfArray,
                                        dims[0] as c_uint, dims[1] as c_uint,
                                        dims[2] as c_uint, dims[3] as c_uint);
                HANDLE_ERROR(AfError::from(err_val));
                Array::from(temp)
            }
        }
    )
}

data_func_def!("Tile the input array along specified dimension", tile, af_tile);
data_func_def!("Reorder the array in specified order", reorder, af_reorder);
data_func_def!("Circular shift of values along specified dimension", shift, af_shift);

/// Change the shape of the Array
///
/// # Parameters
///
/// - `input` is the input Array
/// - `dims` is the new dimensions to which the input Array is reshaped to
///
/// # Return Values
/// Reshaped Array
#[allow(unused_mut)]
pub fn moddims(input: &Array, dims: Dim4) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_moddims(&mut temp as MutAfArray, input.get() as AfArray,
                                 dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Flatten the multidimensional Array to an 1D Array
#[allow(unused_mut)]
pub fn flat(input: &Array) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_flat(&mut temp as MutAfArray, input.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
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
#[allow(unused_mut)]
pub fn flip(input: &Array, dim: u32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_flip(&mut temp as MutAfArray, input.get() as AfArray, dim as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
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
#[allow(unused_mut)]
pub fn lower(input: &Array, is_unit_diag: bool) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_lower(&mut temp as MutAfArray,
                               input.get() as AfArray, is_unit_diag as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
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
#[allow(unused_mut)]
pub fn upper(input: &Array, is_unit_diag: bool) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_upper(&mut temp as MutAfArray,
                               input.get() as AfArray, is_unit_diag as c_int);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Element wise conditional operator for Arrays
///
/// This function does the C-equivalent of the following statement, except that the operation
/// happens on a GPU for all elements simultaneously.
///
/// ```ignore
/// c = cond ? a : b; /// where cond, a & b are all objects of type Array
/// ```
///
/// # Parameters
///
/// - `a` is the Array whose element will be assigned to output if corresponding element in `cond` Array is
/// `True`
/// - `cond` is the Array with conditional values
/// - `b` is the Array whose element will be assigned to output if corresponding element in `cond` Array is
/// `False`
///
/// # Return Values
///
/// An Array
#[allow(unused_mut)]
pub fn select(a: &Array, cond: &Array, b: &Array) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_select(&mut temp as MutAfArray, cond.get() as AfArray,
                                a.get() as AfArray, b.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Element wise conditional operator for Arrays
///
/// This function does the C-equivalent of the following statement, except that the operation
/// happens on a GPU for all elements simultaneously.
///
/// ```ignore
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
#[allow(unused_mut)]
pub fn selectl(a: f64, cond: &Array, b: &Array) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_select_scalar_l(&mut temp as MutAfArray, cond.get() as AfArray,
        a as c_double, b.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Element wise conditional operator for Arrays
///
/// This function does the C-equivalent of the following statement, except that the operation
/// happens on a GPU for all elements simultaneously.
///
/// ```ignore
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
#[allow(unused_mut)]
pub fn selectr(a: &Array, cond: &Array, b: f64) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_select_scalar_r(&mut temp as MutAfArray, cond.get() as AfArray,
                                         a.get() as AfArray, b as c_double);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Inplace replace in Array based on a condition
///
/// This function does the C-equivalent of the following statement, except that the operation
/// happens on a GPU for all elements simultaneously.
///
/// ```ignore
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
#[allow(unused_mut)]
pub fn replace(a: &mut Array, cond: &Array, b: &Array) {
    unsafe {
        let err_val = af_replace(a.get() as AfArray, cond.get() as AfArray, b.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Inplace replace in Array based on a condition
///
/// This function does the C-equivalent of the following statement, except that the operation
/// happens on a GPU for all elements simultaneously.
///
/// ```ignore
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
#[allow(unused_mut)]
pub fn replace_scalar(a: &mut Array, cond: &Array, b: f64) {
    unsafe {
        let err_val = af_replace_scalar(a.get() as AfArray, cond.get() as AfArray, b as c_double);
        HANDLE_ERROR(AfError::from(err_val));
    }
}
