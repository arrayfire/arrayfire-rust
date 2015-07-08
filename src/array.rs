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

    fn af_get_data_ptr(data: *mut c_void, arr: AfArray) -> c_int;

    fn af_eval(arr: AfArray) -> c_int;

    fn af_release_array(arr: AfArray) -> c_int;

    fn af_print_array(arr: AfArray) -> c_int;
}

impl Array {
    #[allow(unused_mut)]
    pub fn new<T>(dims: Dim4, slice: &[T], aftype: Aftype) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            af_create_array(&mut temp as *mut c_longlong,
                            slice.as_ptr() as *const c_void,
                            dims.ndims() as c_uint,
                            dims.get().as_ptr() as * const c_longlong,
                            get_ffi_type(aftype.clone()) as c_int);
            Array { handle: temp }
        }
    }

    pub fn elements(&self) -> i64 {
        unsafe {
            let mut ret_val: i64 = 0;
            af_get_elements(&mut ret_val as *mut c_longlong,
                            self.handle as c_longlong);
            ret_val
        }
    }

    pub fn get_type(&self) -> Aftype {
        unsafe {
            let mut ret_val: i32 = 0;
            af_get_type(&mut ret_val as *mut c_int,
                        self.handle as c_longlong);
            get_af_type(ret_val)
        }
    }

    pub fn dims(&self) -> Dim4 {
        unsafe {
            let mut ret0: i64 = 0;
            let mut ret1: i64 = 0;
            let mut ret2: i64 = 0;
            let mut ret3: i64 = 0;
            af_get_dims(&mut ret0 as *mut c_longlong,
                        &mut ret1 as *mut c_longlong,
                        &mut ret2 as *mut c_longlong,
                        &mut ret3 as *mut c_longlong,
                        self.handle as c_longlong);
            Dim4 { dims: [ret0 as u64, ret1 as u64, ret2 as u64, ret3 as u64] }
        }
    }

    pub fn numdims(&self) -> u32 {
        unsafe {
            let mut ret_val: u32 = 0;
            af_get_numdims(&mut ret_val as *mut c_uint,
                           self.handle as c_longlong);
            ret_val
        }
    }

    pub fn get(&self) -> i64 {
        self.handle
    }

    pub fn host(&self, data:&mut [f64]) {
        unsafe {
            af_get_data_ptr(data.as_mut_ptr() as *mut c_void,
                            self.handle as c_longlong);
        }
    }

    pub fn eval(&self) {
        unsafe {
            af_eval(self.handle as c_longlong);
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
