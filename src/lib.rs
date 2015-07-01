extern crate libc;

use libc::c_void;
use libc::c_int;
use libc::c_uint;
use libc::c_longlong;
use std::ops::Index;

#[link(name="afopencl")]
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

#[allow(unused_mut)]
pub fn randu(dims: &Dim4) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_randu(temp as *mut c_longlong,
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
        af_sin(temp as *mut c_longlong, input.get());
        Array { handle: temp }
    }
}

pub fn print(input: &Array) {
    unsafe {
        af_print_array(input.get() as c_longlong);
    }
}
