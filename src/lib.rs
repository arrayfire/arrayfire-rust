extern crate libc;

use std::fmt;
use libc::c_void;
use libc::c_int;
use libc::c_uint;
use libc::c_double;
use libc::c_longlong;
use std::ops::Index;

#[link(name="afcpu")]
extern {
    fn af_set_device(device: c_int) -> c_int;

    fn af_info() -> c_int;

    fn af_create_array(out: *mut c_longlong,
                        data: *const c_void,
                        ndims: c_uint,
                        dims: *const c_longlong,
                        af_type: c_int) -> c_int;

    fn af_get_data_ptr(data: *mut c_void,
                        arr: c_longlong) -> c_int;

    fn af_eval(arr: c_longlong) -> c_int;

    fn af_release_array(arr: c_longlong) -> c_int;

    fn af_print_array(arr: c_longlong) -> c_int;

    fn af_randu(out: *mut c_longlong,
                ndims: c_uint,
                dims: *const c_longlong,
                af_type: c_int) -> c_int;

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

    fn af_sort_index(out: *mut c_longlong,
                     indices: *mut c_longlong,
                     input: c_longlong,
                     dim: c_uint,
                     ascending: c_int) -> c_int;
}

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
            }
        }
    }

    pub fn get(&self) -> &[u64; 4] {
        &self.dims
    }
}

pub struct Array {
    handle: i64,
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            af_release_array(self.handle);
        }
    }
}

impl Array {
    #[allow(unused_mut)]
    pub fn new(dims: &Dim4, data: &[f64]) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            af_create_array(temp as *mut c_longlong,
                            data.as_ptr() as *const c_void,
                            dims.ndims() as c_uint,
                            dims.get().as_ptr() as * const c_longlong,
                            0);
            Array { handle: temp }
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
pub fn randu(dims: &Dim4) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_randu(&mut temp as *mut c_longlong,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as * const c_longlong,
                0);
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

#[allow(unused_mut)]
pub fn sort(input: &Array, dim: u32, ascending: bool) -> (Array, Array) {
    unsafe {
        let mut temp: i64 = 0;
        let mut idx: i64 = 0;
        af_sort_index(&mut temp as *mut c_longlong,
                      &mut idx as *mut c_longlong,
                      input.get() as c_longlong,
                      dim as c_uint,
                      ascending as c_int);
        (Array {handle: temp}, Array {handle: idx})
    }
}
