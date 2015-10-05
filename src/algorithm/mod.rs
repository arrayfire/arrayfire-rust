extern crate libc;

use array::Array;
use defines::AfError;
use self::libc::{c_int, c_uint};

type MutAfArray = *mut self::libc::c_longlong;
type MutDouble  = *mut self::libc::c_double;
type MutUint    = *mut self::libc::c_uint;
type AfArray    = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_sum(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    //fn af_sum_nan(out: MutAfArray, input: AfArray, dim: c_int, nanval: c_double) -> c_int;
    fn af_product(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    //fn af_product_nan(out: MutAfArray, input: AfArray, dim: c_int, val: c_double) -> c_int;
    fn af_min(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_max(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_all_true(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_any_true(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_count(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_sum_all(r: MutDouble, i: MutDouble, input: AfArray) -> c_int;
    //fn af_sum_nan_all(r: MutDouble, i: MutDouble, input: AfArray, val: c_double) -> c_int;
    fn af_product_all(r: MutDouble, i: MutDouble, input: AfArray) -> c_int;
    //fn af_product_nan_all(r: MutDouble, i: MutDouble, input: AfArray, val: c_double) -> c_int;
    fn af_min_all(r: MutDouble, i: MutDouble, input: AfArray) -> c_int;
    fn af_max_all(r: MutDouble, i: MutDouble, input: AfArray) -> c_int;
    fn af_all_true_all(r: MutDouble, i: MutDouble, input: AfArray) -> c_int;
    fn af_any_true_all(r: MutDouble, i: MutDouble, input: AfArray) -> c_int;
    fn af_count_all(r: MutDouble, i: MutDouble, input: AfArray) -> c_int;
    fn af_imin(out: MutAfArray, idx: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_imax(out: MutAfArray, idx: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_imin_all(r: MutDouble, i: MutDouble, idx: MutUint, input: AfArray) -> c_int;
    fn af_imax_all(r: MutDouble, i: MutDouble, idx: MutUint, input: AfArray) -> c_int;
    fn af_accum(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_where(out: MutAfArray, input: AfArray) -> c_int;
    fn af_diff1(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_diff2(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_sort(out: MutAfArray, input: AfArray, dim: c_uint, ascend: c_int) -> c_int;
    fn af_sort_index(o: MutAfArray, i: MutAfArray, inp: AfArray, d: c_uint, a: c_int) -> c_int;
    fn af_set_unique(out: MutAfArray, input: AfArray, is_sorted: c_int) -> c_int;
    fn af_set_union(out: MutAfArray, first: AfArray, second: AfArray, is_unq: c_int) -> c_int;
    fn af_set_intersect(out: MutAfArray, one: AfArray, two: AfArray, is_unq: c_int) -> c_int;

    fn af_sort_by_key(out_keys: MutAfArray, out_vals: MutAfArray,
                      in_keys: AfArray, in_vals: AfArray, dim: c_uint, ascend: c_int) -> c_int;
}

macro_rules! dim_reduce_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        /// Reduction operation along specific dimension
        ///
        /// # Parameters
        ///
        /// - `input` - Input Array
        /// - `dim` - Dimension along which the input Array will be reduced
        ///
        /// # Return Values
        ///
        /// Reduced Array
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, dim: i32) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                        input.get() as AfArray, dim as c_int);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

dim_reduce_func_def!(sum, af_sum);
dim_reduce_func_def!(product, af_product);
dim_reduce_func_def!(min, af_min);
dim_reduce_func_def!(max, af_max);
dim_reduce_func_def!(all_true, af_all_true);
dim_reduce_func_def!(any_true, af_any_true);
dim_reduce_func_def!(count, af_count);
dim_reduce_func_def!(accum, af_accum);
dim_reduce_func_def!(diff1, af_diff1);
dim_reduce_func_def!(diff2, af_diff2);

//pub fn sum_nan(input: &Array, dim: i32, nanval: f64) -> Array {
//    unsafe {
//        let mut temp: i64 = 0;
//        af_sum_nan(&mut temp as MutAfArray, input.get() as AfArray,
//                   dim as c_int, nanval as c_double);
//        Array {handle: temp}
//    }
//}

//pub fn product_nan(input: &Array, dim: i32, nanval: f64) -> Array {
//    unsafe {
//        let mut temp: i64 = 0;
//        af_product_nan(&mut temp as MutAfArray, input.get() as AfArray,
//                       dim as c_int, nanval as c_double);
//        Array {handle: temp}
//    }
//}

macro_rules! all_reduce_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        /// Reduction operation of all values
        ///
        /// # Parameters
        ///
        /// `input` - Input Array
        ///
        /// # Return Values
        ///
        /// A tuple of reduction result. For non-complex data type Arrays, second value of tuple is
        /// zero.
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array) -> Result<(f64, f64), AfError> {
            unsafe {
                let mut real: f64 = 0.0;
                let mut imag: f64 = 0.0;
                let err_val = $ffi_name(&mut real as MutDouble, &mut imag as MutDouble,
                                        input.get() as AfArray);
                match err_val {
                    0 => Ok((real, imag)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

all_reduce_func_def!(sum_all, af_sum_all);
all_reduce_func_def!(product_all, af_product_all);
all_reduce_func_def!(min_all, af_min_all);
all_reduce_func_def!(max_all, af_max_all);
all_reduce_func_def!(all_true_all, af_all_true_all);
all_reduce_func_def!(any_true_all, af_any_true_all);
all_reduce_func_def!(count_all, af_count_all);

//pub fn sum_nan_all(input: &Array, val: f64) -> (f64, f64) {
//    unsafe {
//        let mut real: f64 = 0.0;
//        let mut imag: f64 = 0.0;
//        af_sum_nan_all(&mut real as MutDouble, &mut imag as MutDouble,
//                       input.get() as AfArray, val as c_double);
//        (real, imag)
//    }
//}

//pub fn product_nan_all(input: &Array, val: f64) -> (f64, f64) {
//    unsafe {
//        let mut real: f64 = 0.0;
//        let mut imag: f64 = 0.0;
//        af_product_nan_all(&mut real as MutDouble, &mut imag as MutDouble,
//                           input.get() as AfArray, val as c_double);
//        (real, imag)
//    }
//}

macro_rules! dim_ireduce_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        /// Reduction operation along specific dimension
        ///
        /// # Parameters
        ///
        /// - `input` - Input Array
        /// - `dim` - Dimension along which the input Array will be reduced
        ///
        /// # Return Values
        ///
        /// A tuple of Arrays: Reduced Array and Indices Array.
        ///
        /// The indices Array has the index of the result element along the reduction dimension.
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, dim: i32) -> Result<(Array, Array), AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let mut idx: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray, &mut idx as MutAfArray,
                                        input.get() as AfArray, dim as c_int);
                match err_val {
                    0 => Ok((Array::from(temp), Array::from(idx))),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

dim_ireduce_func_def!(imin, af_imin);
dim_ireduce_func_def!(imax, af_imax);

macro_rules! all_ireduce_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        /// Reduction operation of all values
        ///
        /// # Parameters
        ///
        /// `input` - Input Array
        ///
        /// # Return Values
        ///
        /// A triplet of reduction result.
        ///
        /// The second value of the tuple is zero for non-complex data type Arrays.
        ///
        /// The third value of triplet is the index of result element from reduction operation.
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array) -> Result<(f64, f64, u32), AfError> {
            unsafe {
                let mut real: f64 = 0.0;
                let mut imag: f64 = 0.0;
                let mut temp: u32 = 0;
                let err_val = $ffi_name(&mut real as MutDouble, &mut imag as MutDouble,
                                        &mut temp as MutUint, input.get() as AfArray);
                match err_val {
                    0 => Ok((real, imag, temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

all_ireduce_func_def!(imin_all, af_imin_all);
all_ireduce_func_def!(imax_all, af_imax_all);

/// Locate the indices of non-zero elements.
///
/// The locations are provided by flattening the input into a linear array.
///
/// # Parameters
///
/// - `input` - Input Array
///
/// # Return Values
///
/// Array of indices where the input Array has non-zero values.
#[allow(unused_mut)]
pub fn locate(input: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_where(&mut temp as MutAfArray, input.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Sort the values in input Arrays
///
/// Sort an multidimensional Array along a given dimension
///
/// # Parameters
///
/// - `input` - Input Array
/// - `dim` - Dimension along which to sort
/// - `ascending` - Sorted output will have ascending values if ```True``` and descending order otherwise.
///
/// # Return Values
///
/// Sorted Array.
#[allow(unused_mut)]
pub fn sort(input: &Array, dim: u32, ascending: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_sort(&mut temp as MutAfArray, input.get() as AfArray,
                dim as c_uint, ascending as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Sort the values in input Arrays
///
/// # Parameters
///
/// - `input` - Input Array
/// - `dim` - Dimension along which to sort
/// - `ascending` - Sorted output will have ascending values if ```True``` and descending order otherwise.
///
/// # Return Values
///
/// A tuple of Arrays.
///
/// The first Array contains the keys based on sorted values.
///
/// The second Array contains the original indices of the sorted values.
#[allow(unused_mut)]
pub fn sort_index(input: &Array, dim: u32, ascending: bool) -> Result<(Array, Array), AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let mut idx: i64 = 0;
        let err_val = af_sort_index(&mut temp as MutAfArray, &mut idx as MutAfArray,
                                    input.get() as AfArray,
                                    dim as c_uint, ascending as c_int);
        match err_val {
            0 => Ok((Array::from(temp), Array::from(idx))),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Sort the values in input Arrays
///
/// Sort an multidimensional Array based on keys
///
/// # Parameters
///
/// - `keys` - Array with key values
/// - `vals` - Array with input values
/// - `dim` - Dimension along which to sort
/// - `ascending` - Sorted output will have ascending values if ```True``` and descending order otherwise.
///
/// # Return Values
///
/// A tuple of Arrays.
///
/// The first Array contains the keys based on sorted values.
///
/// The second Array contains the sorted values.
#[allow(unused_mut)]
pub fn sort_by_key(keys: &Array, vals: &Array, dim: u32,
                   ascending: bool) -> Result<(Array, Array), AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let mut temp2: i64 = 0;
        let err_val = af_sort_by_key(&mut temp as MutAfArray, &mut temp2 as MutAfArray,
                                     keys.get() as AfArray, vals.get() as AfArray,
                                     dim as c_uint, ascending as c_int);
        match err_val {
            0 => Ok((Array::from(temp), Array::from(temp2))),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Find unique values from a Set
///
/// # Parameters
///
/// - `input` - Input Array
/// - `is_sorted` - is a boolean variable. If ```True`` indicates, the `input` Array is sorted.
///
/// # Return Values
///
/// An Array of unique values from the input Array.
#[allow(unused_mut)]
pub fn set_unique(input: &Array, is_sorted: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_set_unique(&mut temp as MutAfArray, input.get() as AfArray,
                                    is_sorted as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Find union of two sets
///
/// # Parameters
///
/// - `first` is one of the input sets
/// - `second` is the other of the input sets
/// - `is_unique` is a boolean value indicates if the input sets are unique
///
/// # Return Values
///
/// An Array with union of the input sets
#[allow(unused_mut)]
pub fn set_union(first: &Array, second: &Array, is_unique: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_set_union(&mut temp as MutAfArray, first.get() as AfArray,
                     second.get() as AfArray, is_unique as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Find intersection of two sets
///
/// # Parameters
///
/// - `first` is one of the input sets
/// - `second` is the other of the input sets
/// - `is_unique` is a boolean value indicates if the input sets are unique
///
/// # Return Values
///
/// An Array with intersection of the input sets
#[allow(unused_mut)]
pub fn set_intersect(first: &Array, second: &Array, is_unique: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_set_intersect(&mut temp as MutAfArray, first.get() as AfArray,
                         second.get() as AfArray, is_unique as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}
