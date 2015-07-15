extern crate libc;

use super::Array as Array;
use super::Dim4 as Dim4;
use super::Aftype as Aftype;
use util::get_ffi_type;
use util::get_af_type;
use self::libc::{c_void, c_int, c_uint, c_longlong};

type MutAfArray = *mut self::libc::c_longlong;
type MutDouble  = *mut self::libc::c_double;
type MutUint    = *mut self::libc::c_uint;
type AfArray    = self::libc::c_longlong;
type DimT       = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_create_array(out: MutAfArray, data: *const c_void,
                       ndims: c_uint, dims: *const DimT, aftype: c_int) -> c_int;

    fn af_get_elements(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_get_type(out: *mut c_int, arr: AfArray) -> c_int;

    fn af_get_dims(dim0: *mut c_longlong, dim1: *mut c_longlong, dim2: *mut c_longlong,
                   dim3: *mut c_longlong, arr: AfArray) -> c_int;

    fn af_get_numdims(result: *mut c_uint, arr: AfArray) -> c_int;

    fn af_is_empty(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_scalar(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_row(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_column(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_vector(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_complex(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_real(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_double(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_single(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_realfloating(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_floating(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_integer(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_bool(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_get_data_ptr(data: *mut c_void, arr: AfArray) -> c_int;

    fn af_eval(arr: AfArray) -> c_int;

    fn af_retain_array(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_release_array(arr: AfArray) -> c_int;

    fn af_print_array(arr: AfArray) -> c_int;
}

macro_rules! is_func {
    ($fn_name: ident, $ffi_fn: ident) => (
        pub fn $fn_name(&self) -> bool {
            unsafe {
                let mut ret_val: i32 = 0;
                $ffi_fn(&mut ret_val as *mut c_int, self.handle as AfArray);
                ret_val > 0
            }
        }
    )
}

impl Array {
    #[allow(unused_mut)]
    pub fn new<T>(dims: Dim4, slice: &[T], aftype: Aftype) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            af_create_array(&mut temp as MutAfArray, slice.as_ptr() as *const c_void,
                            dims.ndims() as c_uint, dims.get().as_ptr() as * const c_longlong,
                            get_ffi_type(aftype) as c_int);
            Array {handle: temp}
        }
    }

    pub fn elements(&self) -> i64 {
        unsafe {
            let mut ret_val: i64 = 0;
            af_get_elements(&mut ret_val as MutAfArray, self.handle as AfArray);
            ret_val
        }
    }

    pub fn get_type(&self) -> Aftype {
        unsafe {
            let mut ret_val: i32 = 0;
            af_get_type(&mut ret_val as *mut c_int, self.handle as AfArray);
            get_af_type(ret_val)
        }
    }

    pub fn dims(&self) -> Dim4 {
        unsafe {
            let mut ret0: i64 = 0;
            let mut ret1: i64 = 0;
            let mut ret2: i64 = 0;
            let mut ret3: i64 = 0;
            af_get_dims(&mut ret0 as *mut c_longlong, &mut ret1 as *mut c_longlong,
                        &mut ret2 as *mut c_longlong, &mut ret3 as *mut c_longlong,
                        self.handle as AfArray);
            Dim4 {dims: [ret0 as u64, ret1 as u64, ret2 as u64, ret3 as u64]}
        }
    }

    pub fn numdims(&self) -> u32 {
        unsafe {
            let mut ret_val: u32 = 0;
            af_get_numdims(&mut ret_val as *mut c_uint, self.handle as AfArray);
            ret_val
        }
    }

    pub fn get(&self) -> i64 {
        self.handle
    }

    pub fn host(&self, data:&mut [f64]) {
        unsafe {
            af_get_data_ptr(data.as_mut_ptr() as *mut c_void, self.handle as AfArray);
        }
    }

    pub fn eval(&self) {
        unsafe {
            af_eval(self.handle as AfArray);
        }
    }

    is_func!(is_empty, af_is_empty);
    is_func!(is_scalar, af_is_scalar);
    is_func!(is_row, af_is_row);
    is_func!(is_column, af_is_column);
    is_func!(is_vector, af_is_vector);
    is_func!(is_complex, af_is_complex);
    is_func!(is_double, af_is_double);
    is_func!(is_single, af_is_single);
    is_func!(is_real, af_is_real);
    is_func!(is_floating, af_is_floating);
    is_func!(is_integer, af_is_integer);
    is_func!(is_bool, af_is_bool);
}

impl Clone for Array {
    fn clone(&self) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            af_retain_array(&mut temp as MutAfArray, self.handle as AfArray);
            Array {handle: temp}
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            af_release_array(self.handle);
        }
    }
}

pub fn print(input: &Array) {
    unsafe {
        af_print_array(input.get() as c_longlong);
    }
}
