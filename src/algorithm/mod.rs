extern crate libc;

use crate::array::Array;
use crate::defines::{AfError, BinaryOp};
use crate::error::HANDLE_ERROR;
use self::libc::{c_int, c_uint, c_double};
use crate::util::{AfArray, MutAfArray, MutDouble, MutUint};
use crate::util::{HasAfEnum, Scanable, RealNumber};

#[allow(dead_code)]
extern {
    fn af_sum(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_sum_nan(out: MutAfArray, input: AfArray, dim: c_int, nanval: c_double) -> c_int;
    fn af_product(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_product_nan(out: MutAfArray, input: AfArray, dim: c_int, val: c_double) -> c_int;
    fn af_min(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_max(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_all_true(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_any_true(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_count(out: MutAfArray, input: AfArray, dim: c_int) -> c_int;
    fn af_sum_all(r: MutDouble, i: MutDouble, input: AfArray) -> c_int;
    fn af_sum_nan_all(r: MutDouble, i: MutDouble, input: AfArray, val: c_double) -> c_int;
    fn af_product_all(r: MutDouble, i: MutDouble, input: AfArray) -> c_int;
    fn af_product_nan_all(r: MutDouble, i: MutDouble, input: AfArray, val: c_double) -> c_int;
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

    fn af_scan(out: MutAfArray, inp: AfArray, dim: c_int, op: c_uint, inclusive: c_int) -> c_int;
    fn af_scan_by_key(out: MutAfArray, key: AfArray, inp: AfArray,
                      dim: c_int, op: c_uint, inclusive: c_int) -> c_int;
}

macro_rules! dim_reduce_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident, $out_type: ty) => (
        #[doc=$doc_str]
        #[allow(unused_mut)]
        pub fn $fn_name<T>(input: &Array<T>, dim: i32) -> Array< $out_type >
            where T: HasAfEnum, $out_type: HasAfEnum
        {
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                        input.get() as AfArray, dim as c_int);
                HANDLE_ERROR(AfError::from(err_val));
            }
            temp.into()
        }
    )
}

dim_reduce_func_def!("
    Sum elements along a given dimension

    # Parameters

    - `input` - Input Array
    - `dim`   - Dimension along which the input Array will be reduced

    # Return Values

    Result Array after summing all elements along given dimension

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, sum};
    let dims = Dim4::new(&[5, 3, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    let b = sum(&a, 0);
    print(&b);
    let c = sum(&a, 1);
    print(&c);
    ```
    ",
    sum, af_sum, T::AggregateOutType);

dim_reduce_func_def!("
    Compute product of elements along a given dimension

    # Parameters

    - `input` - Input Array
    - `dim`   - Dimension along which the input Array will be reduced

    # Return Values

    Result Array after multiplying all elements along given dimension

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, product};
    let dims = Dim4::new(&[5, 3, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    let b = product(&a, 0);
    print(&b);
    let c = product(&a, 1);
    print(&c);
    ```
    ", product, af_product, T::AggregateOutType);

dim_reduce_func_def!("
    Find minimum among elements of given dimension

    # Parameters

    - `input` - Input Array
    - `dim`   - Dimension along which the input Array will be reduced

    # Return Values

    Result Array after finding minimum among elements along a given dimension

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, min};
    let dims = Dim4::new(&[5, 3, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    let b = min(&a, 0);
    print(&b);
    let c = min(&a, 1);
    print(&c);
    ```
    ", min, af_min, T::InType);

dim_reduce_func_def!("
    Find maximum among elements of given dimension

    # Parameters

    - `input` - Input Array
    - `dim`   - Dimension along which the input Array will be reduced

    # Return Values

    Result Array after finding maximum among elements along a given dimension

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, max};
    let dims = Dim4::new(&[5, 3, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    let b = max(&a, 0);
    print(&b);
    let c = max(&a, 1);
    print(&c);
    ```
    ", max, af_max, T::InType);

dim_reduce_func_def!("
    Find if all of the values along a given dimension in the Array are true

    # Parameters

    - `input` - Input Array
    - `dim`   - Dimension along which the predicate is evaluated

    # Return Values

    Result Array that contains the result of `AND` operation of all elements along given dimension

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, all_true};
    let dims = Dim4::new(&[5, 3, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    let b = all_true(&a, 0);
    print(&b);
    let c = all_true(&a, 1);
    print(&c);
    ```
    ", all_true, af_all_true, bool);

dim_reduce_func_def!("
    Find if any of the values along a given dimension in the Array are true

    # Parameters

    - `input` - Input Array
    - `dim`   - Dimension along which the predicate is evaluated

    # Return Values

    Result Array that contains the result of `OR` operation of all elements along given dimension

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, any_true};
    let dims = Dim4::new(&[5, 3, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    let b = any_true(&a, 0);
    print(&b);
    let c = any_true(&a, 1);
    print(&c);
    ```
    ", any_true, af_any_true, bool);

dim_reduce_func_def!("
    Count number of non-zero elements along a given dimension

    # Parameters

    - `input` - Input Array
    - `dim`   - Dimension along which the non-zero elements are counted

    # Return Values

    Result Array with number of non-zero elements along a given dimension

    # Examples

    ```rust
    use arrayfire::{Dim4, gt, print, randu, count};
    let dims = Dim4::new(&[5, 3, 1, 1]);
    let cnst: f32 = 0.5;
    let a = gt(&randu::<f32>(dims), &cnst, false);
    print(&a);
    let b = count(&a, 0);
    print(&b);
    let c = count(&a, 1);
    print(&c);
    ```
    ", count, af_count, u32);

dim_reduce_func_def!("
    Perform exclusive sum of elements along a given dimension

    # Parameters

    - `input` - Input Array
    - `dim`   - Dimension along which the exclusive scan operation is carried out

    # Return Values

    Result Array with exclusive sums of input Array elements along a given dimension

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, accum};
    let dims = Dim4::new(&[5, 3, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    let b = accum(&a, 0);
    print(&b);
    let c = accum(&a, 1);
    print(&c);
    ```
    ", accum, af_accum, T::AggregateOutType);

dim_reduce_func_def!("
    Calculate first order numerical difference along a given dimension

    # Parameters

    - `input` - Input Array
    - `dim`   - Dimension along which first order difference is calculated

    # Return Values

    Result Array with first order difference values

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, diff1};
    let dims = Dim4::new(&[5, 3, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    let b = diff1(&a, 0);
    print(&b);
    let c = diff1(&a, 1);
    print(&c);
    ```
    ", diff1, af_diff1, T::InType);

dim_reduce_func_def!("
    Calculate second order numerical difference along a given dimension

    # Parameters

    - `input` - Input Array
    - `dim`   - Dimension along which second order difference is calculated

    # Return Values

    Result Array with second order difference values

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, diff2};
    let dims = Dim4::new(&[5, 3, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    let b = diff2(&a, 0);
    print(&b);
    let c = diff2(&a, 1);
    print(&c);
    ```
    ", diff2, af_diff2, T::InType);

/// Sum along specific dimension using user specified value instead of `NAN` values
///
/// Sum values of the `input` Array along `dim` dimension after replacing any `NAN` values in the
/// Array with the value of the parameter `nanval`.
///
/// # Parameters
///
/// - `input` is the input Array
/// - `dim` is reduction dimension
/// - `nanval` is value with which all the `NAN` values of Array are replaced with
///
/// # Return Values
///
/// Array that is reduced along given dimension via addition operation
pub fn sum_nan<T>(input: &Array<T>,
                  dim: i32, nanval: f64) -> Array< T::AggregateOutType >
    where T: HasAfEnum,
          T::AggregateOutType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_sum_nan(&mut temp as MutAfArray, input.get() as AfArray,
                                 dim as c_int, nanval as c_double);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Product of elements along specific dimension using user specified value instead of `NAN` values
///
/// Compute product of the values of the `input` Array along `dim` dimension after replacing any `NAN` values in the Array with `nanval` value.
///
/// # Parameters
///
/// - `input` is the input Array
/// - `dim` is reduction dimension
/// - `nanval` is value with which all the `NAN` values of Array are replaced with
///
/// # Return Values
///
/// Array that is reduced along given dimension via multiplication operation
pub fn product_nan<T>(input: &Array<T>,
                      dim: i32, nanval: f64) -> Array< T::AggregateOutType >
    where T: HasAfEnum,
          T::AggregateOutType: HasAfEnum
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_product_nan(&mut temp as MutAfArray, input.get() as AfArray,
                                     dim as c_int, nanval as c_double);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

macro_rules! all_reduce_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident) => (
        #[doc=$doc_str]
        #[allow(unused_mut)]
        pub fn $fn_name<T:HasAfEnum>(input: &Array<T>) -> (f64, f64) {
            let mut real: f64 = 0.0;
            let mut imag: f64 = 0.0;
            unsafe {
                let err_val = $ffi_name(&mut real as MutDouble,
                                        &mut imag as MutDouble,
                                        input.get() as AfArray);
                HANDLE_ERROR(AfError::from(err_val));
            }
            (real, imag)
        }
    )
}

all_reduce_func_def!("
    Sum all values of the Array

    # Parameters

    - `input` is the input Array

    # Return Values

    A tuple containing the summation result.

    Note: For non-complex data type Arrays, second value of tuple is zero.

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, sum_all};
    let dims = Dim4::new(&[5, 5, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    println!(\"Result : {:?}\", sum_all(&a));
    ```
    ", sum_all, af_sum_all);

all_reduce_func_def!("
    Product of all values of the Array

    # Parameters

    - `input` is the input Array

    # Return Values

    A tuple containing the product result.

    Note: For non-complex data type Arrays, second value of tuple is zero.

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, product_all};
    let dims = Dim4::new(&[5, 5, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    println!(\"Result : {:?}\", product_all(&a));
    ```
    ", product_all, af_product_all);

all_reduce_func_def!("
    Find minimum among all values of the Array

    # Parameters

    - `input` is the input Array

    # Return Values

    A tuple containing the minimum value.

    Note: For non-complex data type Arrays, second value of tuple is zero.

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, min_all};
    let dims = Dim4::new(&[5, 5, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    println!(\"Result : {:?}\", min_all(&a));
    ```
    ", min_all, af_min_all);

all_reduce_func_def!("
    Find maximum among all values of the Array

    # Parameters

    - `input` is the input Array

    # Return Values

    A tuple containing the maximum value.

    Note: For non-complex data type Arrays, second value of tuple is zero.

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, max_all};
    let dims = Dim4::new(&[5, 5, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    println!(\"Result : {:?}\", max_all(&a));
    ```
    ", max_all, af_max_all);

all_reduce_func_def!("
    Find if all values of Array are non-zero

    # Parameters

    - `input` is the input Array

    # Return Values

    A tuple containing the result of `AND` operation on all values of Array.

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, all_true_all};
    let dims = Dim4::new(&[5, 5, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    println!(\"Result : {:?}\", all_true_all(&a));
    ```
    ", all_true_all, af_all_true_all);

all_reduce_func_def!("
    Find if any value of Array is non-zero

    # Parameters

    - `input` is the input Array

    # Return Values

    A tuple containing the result of `OR` operation on all values of Array.

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, any_true_all};
    let dims = Dim4::new(&[5, 5, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    println!(\"Result : {:?}\", any_true_all(&a));
    ```
    ", any_true_all, af_any_true_all);

all_reduce_func_def!("
    Count number of non-zero values in the Array

    # Parameters

    - `input` is the input Array

    # Return Values

    A tuple containing the count of non-zero values in the Array.

    # Examples

    ```rust
    use arrayfire::{Dim4, print, randu, count_all};
    let dims = Dim4::new(&[5, 5, 1, 1]);
    let a = randu::<f32>(dims);
    print(&a);
    println!(\"Result : {:?}\", count_all(&a));
    ```
    ", count_all, af_count_all);

/// Sum all values using user provided value for `NAN`
///
/// Sum all the values of the `input` Array after replacing any `NAN` values with `val`.
///
/// # Parameters
///
/// - `input` is the input Array
/// - `val` is the val that replaces all `NAN` values of the Array before reduction operation is
/// performed.
///
/// # Return Values
///
/// A tuple of summation result.
///
/// Note: For non-complex data type Arrays, second value of tuple is zero.
pub fn sum_nan_all<T: HasAfEnum>(input: &Array<T>, val: f64) -> (f64, f64) {
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_sum_nan_all(&mut real as MutDouble, &mut imag as MutDouble,
                                     input.get() as AfArray, val as c_double);
        HANDLE_ERROR(AfError::from(err_val));
    }
    (real, imag)
}

/// Product of all values using user provided value for `NAN`
///
/// Compute the product of all the values of the `input` Array after replacing any `NAN` values with `val`
///
/// # Parameters
///
/// - `input` is the input Array
/// - `val` is the val that replaces all `NAN` values of the Array before reduction operation is
/// performed.
///
/// # Return Values
///
/// A tuple of product result.
///
/// Note: For non-complex data type Arrays, second value of tuple is zero.
pub fn product_nan_all<T: HasAfEnum>(input: &Array<T>, val: f64) -> (f64, f64) {
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_product_nan_all(&mut real as MutDouble, &mut imag as MutDouble,
                                         input.get() as AfArray, val as c_double);
        HANDLE_ERROR(AfError::from(err_val));
    }
    (real, imag)
}

macro_rules! dim_ireduce_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident, $out_type: ident) => (
        #[doc=$doc_str]
        #[allow(unused_mut)]
        pub fn $fn_name<T>(input: &Array<T>, dim: i32) -> (Array< T::$out_type >, Array<u32>)
            where T: HasAfEnum,
                  T::$out_type: HasAfEnum
        {
            let mut temp: i64 = 0;
            let mut idx: i64 = 0;
            unsafe {
                let err_val = $ffi_name(&mut temp as MutAfArray, &mut idx as MutAfArray,
                                        input.get() as AfArray, dim as c_int);
                HANDLE_ERROR(AfError::from(err_val));
            }
            (temp.into(), idx.into())
        }
    )
}

dim_ireduce_func_def!("
    Find minimum value along given dimension and their corresponding indices

    # Parameters

    - `input` - Input Array
    - `dim` - Dimension along which the input Array will be reduced

    # Return Values

    A tuple of Arrays: Array minimum values and Array containing their index along the reduced dimension.
    ", imin, af_imin, InType);

dim_ireduce_func_def!("
    Find maximum value along given dimension and their corresponding indices

    # Parameters

    - `input` - Input Array
    - `dim` - Dimension along which the input Array will be reduced

    # Return Values

    A tuple of Arrays: Array maximum values and Array containing their index along the reduced dimension.
    ", imax, af_imax, InType);

macro_rules! all_ireduce_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident) => (
        #[doc=$doc_str]
        #[allow(unused_mut)]
        pub fn $fn_name<T:HasAfEnum>(input: &Array<T>) -> (f64, f64, u32) {
            let mut real: f64 = 0.0;
            let mut imag: f64 = 0.0;
            let mut temp: u32 = 0;
            unsafe {
                let err_val = $ffi_name(&mut real as MutDouble, &mut imag as MutDouble,
                                        &mut temp as MutUint, input.get() as AfArray);
                HANDLE_ERROR(AfError::from(err_val));
            }
            (real, imag, temp)
        }
    )
}

all_ireduce_func_def!("
    Find minimum and it's index in the whole Array

    # Parameters

    `input` - Input Array

    # Return Values

    A triplet with

      * minimum element of Array in the first component.
      * second component of value zero if Array is of non-complex type.
      * index of minimum element in the third component.
    ", imin_all, af_imin_all);
all_ireduce_func_def!("
    Find maximum and it's index in the whole Array

    # Parameters

    `input` - Input Array

    # Return Values

    A triplet with

      - maximum element of Array in the first component.
      - second component of value zero if Array is of non-complex type.
      - index of maximum element in the third component.
    ", imax_all, af_imax_all);

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
pub fn locate<T:HasAfEnum>(input: &Array<T>) -> Array<u32> {
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_where(&mut temp as MutAfArray, input.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Sort the values in input Arrays
///
/// Sort an multidimensional Array along a given dimension
///
/// # Parameters
///
/// - `input` - Input Array
/// - `dim` - Dimension along which to sort
/// - `ascending` - Sorted output will have ascending values if
///                 ```True``` and descending order otherwise.
///
/// # Return Values
///
/// Sorted Array.
#[allow(unused_mut)]
pub fn sort<T>(input: &Array<T>, dim: u32, ascending: bool) -> Array<T>
    where T: HasAfEnum + RealNumber
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_sort(&mut temp as MutAfArray, input.get() as AfArray,
                              dim as c_uint, ascending as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Sort the values in input Arrays
///
/// # Parameters
///
/// - `input` - Input Array
/// - `dim` - Dimension along which to sort
/// - `ascending` - Sorted output will have ascending values if
///                 ```True``` and descending order otherwise.
///
/// # Return Values
///
/// A tuple of Arrays.
///
/// The first Array contains the keys based on sorted values.
///
/// The second Array contains the original indices of the sorted values.
#[allow(unused_mut)]
pub fn sort_index<T>(input: &Array<T>, dim: u32, ascending: bool) -> (Array<T>, Array<u32>)
    where T: HasAfEnum + RealNumber
{
    let mut temp: i64 = 0;
    let mut idx: i64 = 0;
    unsafe {
        let err_val = af_sort_index(&mut temp as MutAfArray,
                                    &mut idx as MutAfArray,
                                    input.get() as AfArray,
                                    dim as c_uint, ascending as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    (temp.into(), idx.into())
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
pub fn sort_by_key<K, V>(keys: &Array<K>, vals: &Array<V>, dim: u32,
                         ascending: bool) -> (Array<K>, Array<V>)
    where K: HasAfEnum + RealNumber,
          V: HasAfEnum
{
    let mut temp: i64 = 0;
    let mut temp2: i64 = 0;
    unsafe {
        let err_val = af_sort_by_key(&mut temp as MutAfArray,
                                     &mut temp2 as MutAfArray,
                                     keys.get() as AfArray,
                                     vals.get() as AfArray,
                                     dim as c_uint, ascending as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    (temp.into(), temp2.into())
}

/// Find unique values from a Set
///
/// # Parameters
///
/// - `input` - Input Array
/// - `is_sorted` - is a boolean variable. If ```True``
///                 indicates, the `input` Array is sorted.
///
/// # Return Values
///
/// An Array of unique values from the input Array.
#[allow(unused_mut)]
pub fn set_unique<T>(input: &Array<T>, is_sorted: bool) -> Array<T>
    where T: HasAfEnum + RealNumber
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_set_unique(&mut temp as MutAfArray,
                                    input.get() as AfArray,
                                    is_sorted as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
pub fn set_union<T>(first: &Array<T>, second: &Array<T>, is_unique: bool) -> Array<T>
    where T: HasAfEnum + RealNumber
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_set_union(&mut temp as MutAfArray, first.get() as AfArray,
                                   second.get() as AfArray, is_unique as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
pub fn set_intersect<T>(first: &Array<T>, second: &Array<T>, is_unique: bool) -> Array<T>
    where T: HasAfEnum + RealNumber
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_set_intersect(&mut temp as MutAfArray, first.get() as AfArray,
                                       second.get() as AfArray, is_unique as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Generalized scan
///
/// # Parameters
///
/// - `input` is the data on which scan is to be performed
/// - `dim` is the dimension along which scan operation is to be performed
/// - `op` takes value of [BinaryOp](./enum.BinaryOp.html) enum indicating
///    the type of scan operation
/// - `inclusive` says if inclusive/exclusive scan is to be performed
///
/// # Return Values
///
/// Output Array of scanned input
pub fn scan<T>(input: &Array<T>, dim: i32,
               op: BinaryOp, inclusive: bool) -> Array< T::AggregateOutType >
    where T: HasAfEnum,
          T::AggregateOutType: HasAfEnum
{
    let mut temp : i64 = 0;
    unsafe {
        let err_val = af_scan(&mut temp as MutAfArray, input.get() as AfArray,
                              dim as c_int, op as c_uint, inclusive as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Generalized scan by key
///
/// # Parameters
///
/// - `key` is the key Array
/// - `input` is the data on which scan is to be performed
/// - `dim` is the dimension along which scan operation is to be performed
/// - `op` takes value of [BinaryOp](./enum.BinaryOp.html) enum indicating
///    the type of scan operation
/// - `inclusive` says if inclusive/exclusive scan is to be performed
///
/// # Return Values
///
/// Output Array of scanned input
pub fn scan_by_key<K, V>(key: &Array<K>, input: &Array<V>,
                         dim: i32, op: BinaryOp,
                         inclusive: bool) -> Array< V::AggregateOutType >
    where V: HasAfEnum,
          V::AggregateOutType: HasAfEnum,
          K: HasAfEnum + Scanable
{
    let mut temp : i64 = 0;
    unsafe {
        let err_val = af_scan_by_key(&mut temp as MutAfArray, key.get() as AfArray,
                                     input.get() as AfArray, dim as c_int,
                                     op as c_uint, inclusive as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}
