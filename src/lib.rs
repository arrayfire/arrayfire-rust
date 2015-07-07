extern crate libc;

use libc::{c_void, c_int, c_uint, c_double, c_longlong};

use std::fmt;
use std::ops::Index;
use std::ops::Add;

extern {
    fn af_set_device(device: c_int) -> c_int;

    fn af_info() -> c_int;

    fn af_create_array(out: *mut c_longlong,
                        data: *const c_void,
                        ndims: c_uint,
                        dims: *const c_longlong,
                        af_type: c_int) -> c_int;

    fn af_get_elements(out: *mut c_longlong,
                       arr: c_longlong) -> c_int;

    fn af_get_type(out: *mut c_int,
                   arr: c_longlong) -> c_int;

    fn af_get_dims(dim0: *mut c_longlong,
                   dim1: *mut c_longlong,
                   dim2: *mut c_longlong,
                   dim3: *mut c_longlong,
                   arr: c_longlong) -> c_int;

    fn af_get_numdims(result: *mut c_uint,
                      arr: c_longlong) -> c_int;

    fn af_get_data_ptr(data: *mut c_void,
                        arr: c_longlong) -> c_int;

    fn af_eval(arr: c_longlong) -> c_int;

    fn af_release_array(arr: c_longlong) -> c_int;

    fn af_print_array(arr: c_longlong) -> c_int;

    fn af_constant(out: *mut c_longlong,
                   cnst: c_double,
                   ndims: c_uint,
                   dims: *const c_longlong,
                   af_type: c_int) -> c_int;

    fn af_randu(out: *mut c_longlong,
                ndims: c_uint,
                dims: *const c_longlong,
                af_type: c_int) -> c_int;

    fn af_add(out: *mut c_longlong,
              lhs: c_longlong,
              rhs: c_longlong,
              batch: c_int) -> c_int;

    fn af_sin(out: *mut c_longlong,
                arr: c_longlong) -> c_int;

    fn af_fft(out: *mut c_longlong,
              arr: c_longlong,
              nfac: c_double,
              odim0: c_longlong) -> c_int;

    fn af_fft2(out: *mut c_longlong,
               arr: c_longlong,
               nfac: c_double,
               odim0: c_longlong,
               odim1: c_longlong) -> c_int;

    fn af_fft3(out: *mut c_longlong,
               arr: c_longlong,
               nfac: c_double,
               odim0: c_longlong,
               odim1: c_longlong,
               odim2: c_longlong) -> c_int;
}

#[derive(Clone)]
pub enum Aftype {
    F32,
    C32,
    F64,
    C64,
    B8,
    S32,
    U32,
    U8,
    S64,
    U64,
}

fn get_ffi_type(t: Aftype) -> i32 {
    match t {
        Aftype::F32 => 0,
        Aftype::C32 => 1,
        Aftype::F64 => 2,
        Aftype::C64 => 3,
        Aftype::B8  => 4,
        Aftype::S32 => 5,
        Aftype::U32 => 6,
        Aftype::U8  => 7,
        Aftype::S64 => 8,
        Aftype::U64 => 9,
    }
}

fn get_af_type(t: i32) -> Aftype {
    match t {
        0 => Aftype::F32,
        1 => Aftype::C32,
        2 => Aftype::F64,
        3 => Aftype::C64,
        4 => Aftype::B8 ,
        5 => Aftype::S32,
        6 => Aftype::U32,
        7 => Aftype::U8 ,
        8 => Aftype::S64,
        9 => Aftype::U64,
        _ => Aftype::F32,
    }
}

#[derive(Clone)]
pub struct Dim4 {
    dims: [u64; 4],
}

impl Default for Dim4 {
    fn default() -> Dim4 {
        Dim4 { dims:[1, 1, 1, 1] }
    }
}

impl Index<usize> for Dim4 {
    type Output = u64;

    fn index<'a>(&'a self, _index: usize) ->&'a u64 {
        &self.dims[_index]
    }
}

impl fmt::Display for Dim4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {} {} {}]", self.dims[0], self.dims[1], self.dims[2], self.dims[3])
    }
}

impl Dim4 {
    pub fn new(dims: &[u64; 4]) -> Dim4 {
        Dim4 { dims: dims.clone(), }
    }

    pub fn elements(&self) -> u64 {
        self.dims[0]*self.dims[1]*self.dims[2]*self.dims[3]
    }

    pub fn ndims(&self) -> usize {
        let nelems = self.elements();
        match nelems {
            0 => 0,
            1 => 0,
            _ => {
                if self.dims[3] != 1 { 4 }
                else if self.dims[2] != 1 { 3 }
                else if self.dims[1] != 1 { 2 }
                else { 1 }
            },
        }
    }

    pub fn get(&self) -> &[u64; 4] {
        &self.dims
    }
}

pub struct Array {
    handle: i64,
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

impl Add<f64> for Array {
    type Output = Array;

    fn add(self, rhs: f64) -> Array {
        let cnst_arr = constant(rhs, self.dims(), self.get_type().clone());
        unsafe {
            let mut temp: i64 = 0;
            af_add(&mut temp as *mut c_longlong,
                   self.get() as c_longlong,
                   cnst_arr.get() as c_longlong,
                   0);
            Array { handle: temp }
        }
    }
}

pub fn set_device(device: i32) {
    unsafe {
        af_set_device(device as c_int);
    }
}

pub fn info() {
    unsafe {
        af_info();
    }
}

pub fn print(input: &Array) {
    unsafe {
        af_print_array(input.get() as c_longlong);
    }
}

#[allow(unused_mut)]
pub fn randu(dims: Dim4, aftype: Aftype) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_randu(&mut temp as *mut c_longlong,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as * const c_longlong,
                get_ffi_type(aftype.clone()) as c_int);
        Array { handle: temp }
    }
}

#[allow(unused_mut)]
pub fn constant(cnst: f64, dims: Dim4, aftype: Aftype) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_constant(&mut temp as *mut c_longlong,
                 cnst as c_double,
                 dims.ndims() as c_uint,
                 dims.get().as_ptr() as * const c_longlong,
                 get_ffi_type(aftype.clone()) as c_int);
        Array { handle: temp }
    }
}

#[allow(unused_mut)]
pub fn sin(input: &Array) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_sin(&mut temp as *mut c_longlong, input.get() as c_longlong);
        Array { handle: temp }
    }
}

#[allow(unused_mut)]
pub fn fft(input: &Array, norm_factor: f64, odim0: i64) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_fft(&mut temp as *mut c_longlong,
               input.get() as c_longlong,
               norm_factor as c_double,
               odim0 as c_longlong);
        Array { handle: temp }
    }
}

#[allow(unused_mut)]
pub fn fft2(input: &Array, norm_factor: f64, odim0: i64, odim1: i64) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_fft2(&mut temp as *mut c_longlong,
                input.get() as c_longlong,
                norm_factor as c_double,
                odim0 as c_longlong,
                odim1 as c_longlong);
        Array { handle: temp }
    }
}

#[allow(unused_mut)]
pub fn fft3(input: &Array, norm_factor: f64, odim0: i64, odim1: i64, odim2: i64) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_fft3(&mut temp as *mut c_longlong,
                input.get() as c_longlong,
                norm_factor as c_double,
                odim0 as c_longlong,
                odim1 as c_longlong,
                odim2 as c_longlong);
        Array { handle: temp }
    }
}

pub use algorithm::{sum, sum_nan, product, product_nan, min, max, all_true, any_true, count};
pub use algorithm::{sum_all, sum_nan_all, product_all, product_nan_all, min_all, max_all};
pub use algorithm::{all_true_all, any_true_all, count_all, imin, imax, imin_all, imax_all};
pub use algorithm::{accum, locate, diff1, diff2, sort, sort_index, sort_by_key};
pub use algorithm::{set_unique, set_union, set_intersect};
mod algorithm;
