use super::core::{
    af_array, AfError, Array, BinaryOp, Fromf64, HasAfEnum, RealNumber, ReduceByKeyInput, Scanable,
    HANDLE_ERROR,
};

use libc::{c_double, c_int, c_uint};

extern "C" {
    fn af_sum(out: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_sum_nan(out: *mut af_array, input: af_array, dim: c_int, nanval: c_double) -> c_int;
    fn af_product(out: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_product_nan(out: *mut af_array, input: af_array, dim: c_int, val: c_double) -> c_int;
    fn af_min(out: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_max(out: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_all_true(out: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_any_true(out: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_count(out: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_sum_all(r: *mut c_double, i: *mut c_double, input: af_array) -> c_int;
    fn af_sum_nan_all(r: *mut c_double, i: *mut c_double, input: af_array, val: c_double) -> c_int;
    fn af_product_all(r: *mut c_double, i: *mut c_double, input: af_array) -> c_int;
    fn af_product_nan_all(
        r: *mut c_double,
        i: *mut c_double,
        input: af_array,
        val: c_double,
    ) -> c_int;
    fn af_min_all(r: *mut c_double, i: *mut c_double, input: af_array) -> c_int;
    fn af_max_all(r: *mut c_double, i: *mut c_double, input: af_array) -> c_int;
    fn af_all_true_all(r: *mut c_double, i: *mut c_double, input: af_array) -> c_int;
    fn af_any_true_all(r: *mut c_double, i: *mut c_double, input: af_array) -> c_int;
    fn af_count_all(r: *mut c_double, i: *mut c_double, input: af_array) -> c_int;
    fn af_imin(out: *mut af_array, idx: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_imax(out: *mut af_array, idx: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_imin_all(r: *mut c_double, i: *mut c_double, idx: *mut c_uint, input: af_array) -> c_int;
    fn af_imax_all(r: *mut c_double, i: *mut c_double, idx: *mut c_uint, input: af_array) -> c_int;
    fn af_accum(out: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_where(out: *mut af_array, input: af_array) -> c_int;
    fn af_diff1(out: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_diff2(out: *mut af_array, input: af_array, dim: c_int) -> c_int;
    fn af_sort(out: *mut af_array, input: af_array, dim: c_uint, ascend: bool) -> c_int;
    fn af_sort_index(
        o: *mut af_array,
        i: *mut af_array,
        inp: af_array,
        d: c_uint,
        a: bool,
    ) -> c_int;
    fn af_set_unique(out: *mut af_array, input: af_array, is_sorted: bool) -> c_int;
    fn af_set_union(out: *mut af_array, first: af_array, second: af_array, is_unq: bool) -> c_int;
    fn af_set_intersect(out: *mut af_array, one: af_array, two: af_array, is_unq: bool) -> c_int;

    fn af_sort_by_key(
        out_keys: *mut af_array,
        out_vals: *mut af_array,
        in_keys: af_array,
        in_vals: af_array,
        dim: c_uint,
        ascend: bool,
    ) -> c_int;

    fn af_scan(out: *mut af_array, inp: af_array, dim: c_int, op: c_uint, inclusive: bool)
        -> c_int;
    fn af_scan_by_key(
        out: *mut af_array,
        key: af_array,
        inp: af_array,
        dim: c_int,
        op: c_uint,
        inclusive: bool,
    ) -> c_int;
    fn af_all_true_by_key(
        keys_out: *mut af_array,
        vals_out: *mut af_array,
        keys: af_array,
        vals: af_array,
        dim: c_int,
    ) -> c_int;
    fn af_any_true_by_key(
        keys_out: *mut af_array,
        vals_out: *mut af_array,
        keys: af_array,
        vals: af_array,
        dim: c_int,
    ) -> c_int;
    fn af_count_by_key(
        keys_out: *mut af_array,
        vals_out: *mut af_array,
        keys: af_array,
        vals: af_array,
        dim: c_int,
    ) -> c_int;
    fn af_max_by_key(
        keys_out: *mut af_array,
        vals_out: *mut af_array,
        keys: af_array,
        vals: af_array,
        dim: c_int,
    ) -> c_int;
    fn af_min_by_key(
        keys_out: *mut af_array,
        vals_out: *mut af_array,
        keys: af_array,
        vals: af_array,
        dim: c_int,
    ) -> c_int;
    fn af_product_by_key(
        keys_out: *mut af_array,
        vals_out: *mut af_array,
        keys: af_array,
        vals: af_array,
        dim: c_int,
    ) -> c_int;
    fn af_product_by_key_nan(
        keys_out: *mut af_array,
        vals_out: *mut af_array,
        keys: af_array,
        vals: af_array,
        dim: c_int,
        nan_val: c_double,
    ) -> c_int;
    fn af_sum_by_key(
        keys_out: *mut af_array,
        vals_out: *mut af_array,
        keys: af_array,
        vals: af_array,
        dim: c_int,
    ) -> c_int;
    fn af_sum_by_key_nan(
        keys_out: *mut af_array,
        vals_out: *mut af_array,
        keys: af_array,
        vals: af_array,
        dim: c_int,
        nan_val: c_double,
    ) -> c_int;
    fn af_max_ragged(
        val_out: *mut af_array,
        idx_out: *mut af_array,
        input: af_array,
        ragged_len: af_array,
        dim: c_int,
    ) -> c_int;
}

macro_rules! dim_reduce_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident, $out_type: ty) => {
        #[doc=$doc_str]
        pub fn $fn_name<T>(input: &Array<T>, dim: i32) -> Array<$out_type>
        where
            T: HasAfEnum,
            $out_type: HasAfEnum,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(&mut temp as *mut af_array, input.get(), dim);
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    };
}

dim_reduce_func_def!(
    "
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
    sum,
    af_sum,
    T::AggregateOutType
);

dim_reduce_func_def!(
    "
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
    ",
    product,
    af_product,
    T::ProductOutType
);

dim_reduce_func_def!(
    "
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
    ",
    min,
    af_min,
    T::InType
);

dim_reduce_func_def!(
    "
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
    ",
    max,
    af_max,
    T::InType
);

dim_reduce_func_def!(
    "
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
    ",
    all_true,
    af_all_true,
    bool
);

dim_reduce_func_def!(
    "
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
    ",
    any_true,
    af_any_true,
    bool
);

dim_reduce_func_def!(
    "
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
    ",
    count,
    af_count,
    u32
);

dim_reduce_func_def!(
    "
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
    ",
    accum,
    af_accum,
    T::AggregateOutType
);

dim_reduce_func_def!(
    "
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
    ",
    diff1,
    af_diff1,
    T::InType
);

dim_reduce_func_def!(
    "
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
    ",
    diff2,
    af_diff2,
    T::InType
);

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
pub fn sum_nan<T>(input: &Array<T>, dim: i32, nanval: f64) -> Array<T::AggregateOutType>
where
    T: HasAfEnum,
    T::AggregateOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_sum_nan(&mut temp as *mut af_array, input.get(), dim, nanval);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
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
pub fn product_nan<T>(input: &Array<T>, dim: i32, nanval: f64) -> Array<T::ProductOutType>
where
    T: HasAfEnum,
    T::ProductOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_product_nan(&mut temp as *mut af_array, input.get(), dim, nanval);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

macro_rules! all_reduce_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident, $assoc_type:ident) => {
        #[doc=$doc_str]
        pub fn $fn_name<T>(
            input: &Array<T>,
        ) -> (
            <<T as HasAfEnum>::$assoc_type as HasAfEnum>::BaseType,
            <<T as HasAfEnum>::$assoc_type as HasAfEnum>::BaseType,
        )
        where
            T: HasAfEnum,
            <T as HasAfEnum>::$assoc_type: HasAfEnum,
            <<T as HasAfEnum>::$assoc_type as HasAfEnum>::BaseType: HasAfEnum + Fromf64,
        {
            let mut real: f64 = 0.0;
            let mut imag: f64 = 0.0;
            unsafe {
                let err_val = $ffi_name(
                    &mut real as *mut c_double,
                    &mut imag as *mut c_double,
                    input.get(),
                );
                HANDLE_ERROR(AfError::from(err_val));
            }
            (
                <<T as HasAfEnum>::$assoc_type as HasAfEnum>::BaseType::fromf64(real),
                <<T as HasAfEnum>::$assoc_type as HasAfEnum>::BaseType::fromf64(imag),
            )
        }
    };
}

all_reduce_func_def!(
    "
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
    ",
    sum_all,
    af_sum_all,
    AggregateOutType
);

all_reduce_func_def!(
    "
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
    let res = product_all(&a);
    println!(\"Result : {:?}\", res);
    ```
    ",
    product_all,
    af_product_all,
    ProductOutType
);

all_reduce_func_def!(
    "
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
    ",
    min_all,
    af_min_all,
    InType
);

all_reduce_func_def!(
    "
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
    ",
    max_all,
    af_max_all,
    InType
);

macro_rules! all_reduce_func_def2 {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident, $out_type:ty) => {
        #[doc=$doc_str]
        pub fn $fn_name<T>(input: &Array<T>) -> ($out_type, $out_type)
        where
            T: HasAfEnum,
            $out_type: HasAfEnum + Fromf64,
        {
            let mut real: f64 = 0.0;
            let mut imag: f64 = 0.0;
            unsafe {
                let err_val = $ffi_name(
                    &mut real as *mut c_double,
                    &mut imag as *mut c_double,
                    input.get(),
                );
                HANDLE_ERROR(AfError::from(err_val));
            }
            (<$out_type>::fromf64(real), <$out_type>::fromf64(imag))
        }
    };
}

all_reduce_func_def2!(
    "
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
    ",
    all_true_all,
    af_all_true_all,
    bool
);

all_reduce_func_def2!(
    "
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
    ",
    any_true_all,
    af_any_true_all,
    bool
);

all_reduce_func_def2!(
    "
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
    ",
    count_all,
    af_count_all,
    u64
);

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
pub fn sum_nan_all<T>(
    input: &Array<T>,
    val: f64,
) -> (
    <<T as HasAfEnum>::AggregateOutType as HasAfEnum>::BaseType,
    <<T as HasAfEnum>::AggregateOutType as HasAfEnum>::BaseType,
)
where
    T: HasAfEnum,
    <T as HasAfEnum>::AggregateOutType: HasAfEnum,
    <<T as HasAfEnum>::AggregateOutType as HasAfEnum>::BaseType: HasAfEnum + Fromf64,
{
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_sum_nan_all(
            &mut real as *mut c_double,
            &mut imag as *mut c_double,
            input.get(),
            val,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    (
        <<T as HasAfEnum>::AggregateOutType as HasAfEnum>::BaseType::fromf64(real),
        <<T as HasAfEnum>::AggregateOutType as HasAfEnum>::BaseType::fromf64(imag),
    )
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
pub fn product_nan_all<T>(
    input: &Array<T>,
    val: f64,
) -> (
    <<T as HasAfEnum>::ProductOutType as HasAfEnum>::BaseType,
    <<T as HasAfEnum>::ProductOutType as HasAfEnum>::BaseType,
)
where
    T: HasAfEnum,
    <T as HasAfEnum>::ProductOutType: HasAfEnum,
    <<T as HasAfEnum>::ProductOutType as HasAfEnum>::BaseType: HasAfEnum + Fromf64,
{
    let mut real: f64 = 0.0;
    let mut imag: f64 = 0.0;
    unsafe {
        let err_val = af_product_nan_all(
            &mut real as *mut c_double,
            &mut imag as *mut c_double,
            input.get(),
            val,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    (
        <<T as HasAfEnum>::ProductOutType as HasAfEnum>::BaseType::fromf64(real),
        <<T as HasAfEnum>::ProductOutType as HasAfEnum>::BaseType::fromf64(imag),
    )
}

macro_rules! dim_ireduce_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident, $out_type: ident) => {
        #[doc=$doc_str]
        pub fn $fn_name<T>(input: &Array<T>, dim: i32) -> (Array<T::$out_type>, Array<u32>)
        where
            T: HasAfEnum,
            T::$out_type: HasAfEnum,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let mut idx: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(
                    &mut temp as *mut af_array,
                    &mut idx as *mut af_array,
                    input.get(),
                    dim,
                );
                HANDLE_ERROR(AfError::from(err_val));
                (temp.into(), idx.into())
            }
        }
    };
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
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident, $assoc_type:ident) => {
        #[doc=$doc_str]
        pub fn $fn_name<T>(
            input: &Array<T>,
        ) -> (
            <<T as HasAfEnum>::$assoc_type as HasAfEnum>::BaseType,
            <<T as HasAfEnum>::$assoc_type as HasAfEnum>::BaseType,
            u32,
        )
        where
            T: HasAfEnum,
            <T as HasAfEnum>::$assoc_type: HasAfEnum,
            <<T as HasAfEnum>::$assoc_type as HasAfEnum>::BaseType: HasAfEnum + Fromf64,
        {
            let mut real: f64 = 0.0;
            let mut imag: f64 = 0.0;
            let mut temp: u32 = 0;
            unsafe {
                let err_val = $ffi_name(
                    &mut real as *mut c_double,
                    &mut imag as *mut c_double,
                    &mut temp as *mut c_uint,
                    input.get(),
                );
                HANDLE_ERROR(AfError::from(err_val));
            }
            (
                <<T as HasAfEnum>::$assoc_type as HasAfEnum>::BaseType::fromf64(real),
                <<T as HasAfEnum>::$assoc_type as HasAfEnum>::BaseType::fromf64(imag),
                temp,
            )
        }
    };
}

all_ireduce_func_def!(
    "
    Find minimum and it's index in the whole Array

    # Parameters

    `input` - Input Array

    # Return Values

    A triplet with

      * minimum element of Array in the first component.
      * second component of value zero if Array is of non-complex type.
      * index of minimum element in the third component.
    ",
    imin_all,
    af_imin_all,
    InType
);
all_ireduce_func_def!(
    "
    Find maximum and it's index in the whole Array

    # Parameters

    `input` - Input Array

    # Return Values

    A triplet with

      - maximum element of Array in the first component.
      - second component of value zero if Array is of non-complex type.
      - index of maximum element in the third component.
    ",
    imax_all,
    af_imax_all,
    InType
);

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
pub fn locate<T: HasAfEnum>(input: &Array<T>) -> Array<u32> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_where(&mut temp as *mut af_array, input.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
/// - `ascending` - Sorted output will have ascending values if
///                 ```True``` and descending order otherwise.
///
/// # Return Values
///
/// Sorted Array.
pub fn sort<T>(input: &Array<T>, dim: u32, ascending: bool) -> Array<T>
where
    T: HasAfEnum + RealNumber,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_sort(&mut temp as *mut af_array, input.get(), dim, ascending);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
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
pub fn sort_index<T>(input: &Array<T>, dim: u32, ascending: bool) -> (Array<T>, Array<u32>)
where
    T: HasAfEnum + RealNumber,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let mut idx: af_array = std::ptr::null_mut();
        let err_val = af_sort_index(
            &mut temp as *mut af_array,
            &mut idx as *mut af_array,
            input.get(),
            dim,
            ascending,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (temp.into(), idx.into())
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
pub fn sort_by_key<K, V>(
    keys: &Array<K>,
    vals: &Array<V>,
    dim: u32,
    ascending: bool,
) -> (Array<K>, Array<V>)
where
    K: HasAfEnum + RealNumber,
    V: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let mut temp2: af_array = std::ptr::null_mut();
        let err_val = af_sort_by_key(
            &mut temp as *mut af_array,
            &mut temp2 as *mut af_array,
            keys.get(),
            vals.get(),
            dim,
            ascending,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (temp.into(), temp2.into())
    }
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
pub fn set_unique<T>(input: &Array<T>, is_sorted: bool) -> Array<T>
where
    T: HasAfEnum + RealNumber,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_set_unique(&mut temp as *mut af_array, input.get(), is_sorted);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn set_union<T>(first: &Array<T>, second: &Array<T>, is_unique: bool) -> Array<T>
where
    T: HasAfEnum + RealNumber,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_set_union(
            &mut temp as *mut af_array,
            first.get(),
            second.get(),
            is_unique,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn set_intersect<T>(first: &Array<T>, second: &Array<T>, is_unique: bool) -> Array<T>
where
    T: HasAfEnum + RealNumber,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_set_intersect(
            &mut temp as *mut af_array,
            first.get(),
            second.get(),
            is_unique,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
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
pub fn scan<T>(
    input: &Array<T>,
    dim: i32,
    op: BinaryOp,
    inclusive: bool,
) -> Array<T::AggregateOutType>
where
    T: HasAfEnum,
    T::AggregateOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_scan(
            &mut temp as *mut af_array,
            input.get(),
            dim,
            op as u32,
            inclusive,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
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
pub fn scan_by_key<K, V>(
    key: &Array<K>,
    input: &Array<V>,
    dim: i32,
    op: BinaryOp,
    inclusive: bool,
) -> Array<V::AggregateOutType>
where
    V: HasAfEnum,
    V::AggregateOutType: HasAfEnum,
    K: HasAfEnum + Scanable,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_scan_by_key(
            &mut temp as *mut af_array,
            key.get(),
            input.get(),
            dim,
            op as u32,
            inclusive,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

macro_rules! dim_reduce_by_key_func_def {
    ($brief_str: expr, $ex_str: expr, $fn_name: ident, $ffi_name: ident, $out_type: ty) => {
        #[doc=$brief_str]
        /// # Parameters
        ///
        /// - `keys` - key Array
        /// - `vals` - value Array
        /// - `dim`   - Dimension along which the input Array is reduced
        ///
        /// # Return Values
        ///
        /// Tuple of Arrays, with output keys and values after reduction
        ///
        #[doc=$ex_str]
        pub fn $fn_name<KeyType, ValueType>(
            keys: &Array<KeyType>,
            vals: &Array<ValueType>,
            dim: i32,
        ) -> (Array<KeyType>, Array<$out_type>)
        where
            KeyType: ReduceByKeyInput,
            ValueType: HasAfEnum,
            $out_type: HasAfEnum,
        {
            unsafe {
                let mut out_keys: af_array = std::ptr::null_mut();
                let mut out_vals: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(
                    &mut out_keys as *mut af_array,
                    &mut out_vals as *mut af_array,
                    keys.get(),
                    vals.get(),
                    dim,
                );
                HANDLE_ERROR(AfError::from(err_val));
                (out_keys.into(), out_vals.into())
            }
        }
    };
}

dim_reduce_by_key_func_def!(
    "
Key based AND of elements along a given dimension

All positive non-zero values are considered true, while negative and zero
values are considered as false.
",
    "
# Examples
```rust
use arrayfire::{Dim4, print, randu, all_true_by_key};
let dims = Dim4::new(&[5, 3, 1, 1]);
let vals = randu::<f32>(dims);
let keys = randu::<u32>(Dim4::new(&[5, 1, 1, 1]));
print(&vals);
print(&keys);
let (out_keys, out_vals) = all_true_by_key(&keys, &vals, 0);
print(&out_keys);
print(&out_vals);
```
",
    all_true_by_key,
    af_all_true_by_key,
    ValueType::AggregateOutType
);

dim_reduce_by_key_func_def!(
    "
Key based OR of elements along a given dimension

All positive non-zero values are considered true, while negative and zero
values are considered as false.
",
    "
# Examples
```rust
use arrayfire::{Dim4, print, randu, any_true_by_key};
let dims = Dim4::new(&[5, 3, 1, 1]);
let vals = randu::<f32>(dims);
let keys = randu::<u32>(Dim4::new(&[5, 1, 1, 1]));
print(&vals);
print(&keys);
let (out_keys, out_vals) = any_true_by_key(&keys, &vals, 0);
print(&out_keys);
print(&out_vals);
```
",
    any_true_by_key,
    af_any_true_by_key,
    ValueType::AggregateOutType
);

dim_reduce_by_key_func_def!(
    "Find total count of elements with similar keys along a given dimension",
    "",
    count_by_key,
    af_count_by_key,
    ValueType::AggregateOutType
);

dim_reduce_by_key_func_def!(
    "Find maximum among values of similar keys along a given dimension",
    "",
    max_by_key,
    af_max_by_key,
    ValueType::AggregateOutType
);

dim_reduce_by_key_func_def!(
    "Find minimum among values of similar keys along a given dimension",
    "",
    min_by_key,
    af_min_by_key,
    ValueType::AggregateOutType
);

dim_reduce_by_key_func_def!(
    "Find product of all values with similar keys along a given dimension",
    "",
    product_by_key,
    af_product_by_key,
    ValueType::ProductOutType
);

dim_reduce_by_key_func_def!(
    "Find sum of all values with similar keys along a given dimension",
    "",
    sum_by_key,
    af_sum_by_key,
    ValueType::AggregateOutType
);

macro_rules! dim_reduce_by_key_nan_func_def {
    ($brief_str: expr, $ex_str: expr, $fn_name: ident, $ffi_name: ident, $out_type: ty) => {
        #[doc=$brief_str]
        ///
        /// This version of sum by key can replaced all NaN values in the input
        /// with a user provided value before performing the reduction operation.
        /// # Parameters
        ///
        /// - `keys` - key Array
        /// - `vals` - value Array
        /// - `dim`   - Dimension along which the input Array is reduced
        ///
        /// # Return Values
        ///
        /// Tuple of Arrays, with output keys and values after reduction
        ///
        #[doc=$ex_str]
        pub fn $fn_name<KeyType, ValueType>(
            keys: &Array<KeyType>,
            vals: &Array<ValueType>,
            dim: i32,
            replace_value: f64,
        ) -> (Array<KeyType>, Array<$out_type>)
        where
            KeyType: ReduceByKeyInput,
            ValueType: HasAfEnum,
            $out_type: HasAfEnum,
        {
            unsafe {
                let mut out_keys: af_array = std::ptr::null_mut();
                let mut out_vals: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(
                    &mut out_keys as *mut af_array,
                    &mut out_vals as *mut af_array,
                    keys.get(),
                    vals.get(),
                    dim,
                    replace_value,
                );
                HANDLE_ERROR(AfError::from(err_val));
                (out_keys.into(), out_vals.into())
            }
        }
    };
}

dim_reduce_by_key_nan_func_def!(
    "Compute sum of all values with similar keys along a given dimension",
    "",
    sum_by_key_nan,
    af_sum_by_key_nan,
    ValueType::AggregateOutType
);

dim_reduce_by_key_nan_func_def!(
    "Compute product of all values with similar keys along a given dimension",
    "",
    product_by_key_nan,
    af_product_by_key_nan,
    ValueType::ProductOutType
);

/// Max reduction along given axis as per ragged lengths provided
///
/// # Parameters
///
/// - `input` contains the input values to be reduced
/// - `ragged_len` array containing number of elements to use when reducing along `dim`
/// - `dim` is the dimension along which the max operation occurs
///
/// # Return Values
///
/// Tuple of Arrays:
/// - First element: An Array containing the maximum ragged values in `input` along `dim`
///                  according to `ragged_len`
/// - Second Element: An Array containing the locations of the maximum ragged values in
///                   `input` along `dim` according to `ragged_len`
///
/// # Examples
/// ```rust
/// use arrayfire::{Array, dim4, print, randu, max_ragged};
/// let vals: [f32; 6] = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0];
/// let rlens: [u32; 2] = [9, 2];
/// let varr = Array::new(&vals, dim4![3, 2]);
/// let rarr = Array::new(&rlens, dim4![1, 2]);
/// print(&varr);
/// // 1 4
/// // 2 5
/// // 3 6
/// print(&rarr); // numbers of elements to participate in reduction along given axis
/// // 9 2
/// let (out, idx) = max_ragged(&varr, &rarr, 0);
/// print(&out);
/// // 3 5
/// print(&idx);
/// // 2 1 //Since 3 is max element for given length 9 along first column
///        //Since 5 is max element for given length 2 along second column
/// ```
pub fn max_ragged<T>(
    input: &Array<T>,
    ragged_len: &Array<u32>,
    dim: i32,
) -> (Array<T::InType>, Array<u32>)
where
    T: HasAfEnum,
    T::InType: HasAfEnum,
{
    unsafe {
        let mut out_vals: af_array = std::ptr::null_mut();
        let mut out_idxs: af_array = std::ptr::null_mut();
        let err_val = af_max_ragged(
            &mut out_vals as *mut af_array,
            &mut out_idxs as *mut af_array,
            input.get(),
            ragged_len.get(),
            dim,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (out_vals.into(), out_idxs.into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::core::c32;
    use super::{imax_all, imin_all, product_nan_all, sum_all, sum_nan_all};
    use crate::core::set_device;
    use crate::randu;

    #[test]
    fn all_reduce_api() {
        set_device(0);
        let a = randu!(c32; 10, 10);
        println!("Reduction of complex f32 matrix: {:?}", sum_all(&a));

        let b = randu!(bool; 10, 10);
        println!("reduction of bool matrix: {:?}", sum_all(&b));

        println!(
            "reduction of complex f32 matrix after replacing nan with 1.0: {:?}",
            product_nan_all(&a, 1.0)
        );

        println!(
            "reduction of bool matrix after replacing nan with 0.0: {:?}",
            sum_nan_all(&b, 0.0)
        );
    }

    #[test]
    fn all_ireduce_api() {
        set_device(0);
        let a = randu!(c32; 10);
        println!("Reduction of complex f32 matrix: {:?}", imin_all(&a));

        let b = randu!(u32; 10);
        println!("reduction of bool matrix: {:?}", imax_all(&b));
    }
}
