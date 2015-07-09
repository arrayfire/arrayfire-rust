extern crate libc;

use super::Array as Array;
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

#[allow(unused_mut)]
pub fn sum(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_sum(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        Array {handle: temp}
    }
}

//pub fn sum_nan(input: &Array, dim: i32, nanval: f64) -> Array {
//    unsafe {
//        let mut temp: i64 = 0;
//        af_sum_nan(&mut temp as MutAfArray, input.get() as AfArray,
//                   dim as c_int, nanval as c_double);
//        Array {handle: temp}
//    }
//}

#[allow(unused_mut)]
pub fn product(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_product(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        Array {handle: temp}
    }
}

//pub fn product_nan(input: &Array, dim: i32, nanval: f64) -> Array {
//    unsafe {
//        let mut temp: i64 = 0;
//        af_product_nan(&mut temp as MutAfArray, input.get() as AfArray,
//                       dim as c_int, nanval as c_double);
//        Array {handle: temp}
//    }
//}

#[allow(unused_mut)]
pub fn min(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_min(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn max(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_max(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn all_true(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_all_true(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn any_true(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_any_true(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn count(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_count(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn sum_all(input: &Array) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        af_sum_all(&mut real as MutDouble, &mut imag as MutDouble,
                   input.get() as AfArray);
        (real, imag)
    }
}

//pub fn sum_nan_all(input: &Array, val: f64) -> (f64, f64) {
//    unsafe {
//        let mut real: f64 = 0.0;
//        let mut imag: f64 = 0.0;
//        af_sum_nan_all(&mut real as MutDouble, &mut imag as MutDouble,
//                       input.get() as AfArray, val as c_double);
//        (real, imag)
//    }
//}

pub fn product_all(input: &Array) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        af_product_all(&mut real as MutDouble, &mut imag as MutDouble,
                       input.get() as AfArray);
        (real, imag)
    }
}

//pub fn product_nan_all(input: &Array, val: f64) -> (f64, f64) {
//    unsafe {
//        let mut real: f64 = 0.0;
//        let mut imag: f64 = 0.0;
//        af_product_nan_all(&mut real as MutDouble, &mut imag as MutDouble,
//                           input.get() as AfArray, val as c_double);
//        (real, imag)
//    }
//}

#[allow(unused_mut)]
pub fn min_all(input: &Array) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        af_min_all(&mut real as MutDouble, &mut imag as MutDouble,
                   input.get() as AfArray);
        (real, imag)
    }
}

#[allow(unused_mut)]
pub fn max_all(input: &Array) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        af_max_all(&mut real as MutDouble, &mut imag as MutDouble,
                   input.get() as AfArray);
        (real, imag)
    }
}

#[allow(unused_mut)]
pub fn all_true_all(input: &Array) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        af_all_true_all(&mut real as MutDouble, &mut imag as MutDouble,
                        input.get() as AfArray);
        (real, imag)
    }
}

#[allow(unused_mut)]
pub fn any_true_all(input: &Array) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        af_any_true_all(&mut real as MutDouble, &mut imag as MutDouble,
                        input.get() as AfArray);
        (real, imag)
    }
}

#[allow(unused_mut)]
pub fn count_all(input: &Array) -> (f64, f64) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        af_count_all(&mut real as MutDouble, &mut imag as MutDouble,
                     input.get() as AfArray);
        (real, imag)
    }
}

#[allow(unused_mut)]
pub fn imin(input: &Array, dim: i32) -> (Array, Array) {
    unsafe {
        let mut temp: i64 = 0;
        let mut idx: i64 = 0;
        af_imin(&mut temp as MutAfArray, &mut idx as MutAfArray,
                input.get() as AfArray, dim as c_int);
        (Array{handle: temp}, Array{handle: idx})
    }
}

#[allow(unused_mut)]
pub fn imax(input: &Array, dim: i32) -> (Array, Array) {
    unsafe {
        let mut temp: i64 = 0;
        let mut idx: i64 = 0;
        af_imax(&mut temp as MutAfArray, &mut idx as MutAfArray,
                input.get() as AfArray, dim as c_int);
        (Array{handle: temp}, Array{handle: idx})
    }
}

#[allow(unused_mut)]
pub fn imin_all(input: &Array) -> (f64, f64, u32) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        let mut temp: u32 = 0;
        af_imin_all(&mut real as MutDouble, &mut imag as MutDouble,
                    &mut temp as MutUint, input.get() as AfArray);
        (real, imag, temp)
    }
}

#[allow(unused_mut)]
pub fn imax_all(input: &Array) -> (f64, f64, u32) {
    unsafe {
        let mut real: f64 = 0.0;
        let mut imag: f64 = 0.0;
        let mut temp: u32 = 0;
        af_imax_all(&mut real as MutDouble, &mut imag as MutDouble,
                    &mut temp as MutUint, input.get() as AfArray);
        (real, imag, temp)
    }
}

#[allow(unused_mut)]
pub fn accum(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_accum(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn locate(input: &Array) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_where(&mut temp as MutAfArray, input.get() as AfArray);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn diff1(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_diff1(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn diff2(input: &Array, dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_diff2(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn sort(input: &Array, dim: u32, ascending: bool) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_sort(&mut temp as MutAfArray, input.get() as AfArray,
                dim as c_uint, ascending as c_int);
        Array{handle: temp}
    }
}

#[allow(unused_mut)]
pub fn sort_index(input: &Array, dim: u32, ascending: bool) -> (Array, Array) {
    unsafe {
        let mut temp: i64 = 0;
        let mut idx: i64 = 0;
        af_sort_index(&mut temp as MutAfArray, &mut idx as MutAfArray,
                      input.get() as AfArray,
                      dim as c_uint, ascending as c_int);
        (Array {handle: temp}, Array {handle: idx})
    }
}

#[allow(unused_mut)]
pub fn sort_by_key(keys: &Array, vals: &Array, dim: u32, ascending: bool) -> (Array, Array) {
    unsafe {
        let mut temp: i64 = 0;
        let mut temp2: i64 = 0;
        af_sort_by_key(&mut temp as MutAfArray, &mut temp2 as MutAfArray,
                       keys.get() as AfArray, vals.get() as AfArray,
                      dim as c_uint, ascending as c_int);
        (Array {handle: temp}, Array {handle: temp2})
    }
}

#[allow(unused_mut)]
pub fn set_unique(input: &Array, is_sorted: bool) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_set_unique(&mut temp as MutAfArray, input.get() as AfArray, is_sorted as c_int);
        Array{handle: temp}
    }
}

#[allow(unused_mut)]
pub fn set_union(first: &Array, second: &Array, is_unique: bool) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_set_union(&mut temp as MutAfArray, first.get() as AfArray,
                     second.get() as AfArray, is_unique as c_int);
        Array{handle: temp}
    }
}

#[allow(unused_mut)]
pub fn set_intersect(first: &Array, second: &Array, is_unique: bool) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_set_intersect(&mut temp as MutAfArray, first.get() as AfArray,
                         second.get() as AfArray, is_unique as c_int);
        Array{handle: temp}
    }
}
