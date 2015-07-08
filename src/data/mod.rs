extern crate libc;
extern crate num;

use super::Array as Array;
use super::Dim4 as Dim4;
use super::Aftype as Aftype;
use util::get_ffi_type;
use self::libc::{c_int, c_uint, c_double};
use self::num::Complex;

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
                seq_dims: c_int, afdtype: c_int) -> c_int;

    fn af_iota(out: MutAfArray, ndims: c_uint, dims: *const DimT,
               t_ndims: c_uint, tdims: *const DimT, afdtype: c_int) -> c_int;

    fn af_randu(out: MutAfArray, ndims: c_uint, dims: *const DimT, afdtype: c_int) -> c_int;

    fn af_randn(out: MutAfArray, ndims: c_uint, dims: *const DimT, afdtype: c_int) -> c_int;

    fn af_set_seed(seed: Uintl);

    fn af_get_seed(seed: *mut Uintl);

}

pub trait ConstGenerator {
    fn generate(&self, dims: Dim4) -> Array;
}

#[allow(unused_mut)]
impl ConstGenerator for i64 {
    fn generate(&self, dims: Dim4) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            af_constant_long(&mut temp as MutAfArray, *self as Intl,
                             dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT);
            Array {handle: temp}
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for u64 {
    fn generate(&self, dims: Dim4) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            af_constant_ulong(&mut temp as MutAfArray, *self as Uintl,
                              dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT);
            Array {handle: temp}
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for Complex<f32> {
    fn generate(&self, dims: Dim4) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            af_constant_complex(&mut temp as MutAfArray,
                                (*self).re as c_double, (*self).im as c_double,
                                dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT, 1);
            Array {handle: temp}
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for Complex<f64> {
    fn generate(&self, dims: Dim4) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            af_constant_complex(&mut temp as MutAfArray,
                                (*self).re as c_double, (*self).im as c_double,
                                dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT, 3);
            Array {handle: temp}
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
                    af_constant(&mut temp as MutAfArray, *self as c_double,
                                dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                                $ffi_type);
                    Array {handle: temp}
                }
            }
        }
    )
}

cnst!(f32 , 0);
cnst!(f64 , 2);
cnst!(bool, 4);
cnst!(i32 , 5);
cnst!(u32 , 6);
cnst!(u8  , 7);


pub fn constant<T : ConstGenerator>(cnst: T, dims: Dim4) -> Array {
    cnst.generate(dims)
}

#[allow(unused_mut)]
pub fn range(dims: Dim4, seq_dim: i32, aftype: Aftype) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_range(&mut temp as MutAfArray,
                 dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                 seq_dim as c_int,
                 get_ffi_type(aftype.clone()) as c_int);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn iota(dims: Dim4, tdims: Dim4, aftype: Aftype) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_iota(&mut temp as MutAfArray,
                dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                tdims.ndims() as c_uint, tdims.get().as_ptr() as *const DimT,
                get_ffi_type(aftype.clone()) as c_int);
        Array {handle: temp}
    }
}

pub fn set_seed(seed: u64) {
    unsafe { af_set_seed(seed as Uintl); }
}

#[allow(unused_mut)]
pub fn get_seed() -> u64 {
    unsafe {
        let mut temp: u64 = 0;
        af_get_seed(&mut temp as *mut Uintl);
        temp
    }
}

#[allow(unused_mut)]
pub fn randu(dims: Dim4, aftype: Aftype) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_randu(&mut temp as MutAfArray,
                 dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                 get_ffi_type(aftype.clone()) as c_int);
        Array {handle: temp}
    }
}

#[allow(unused_mut)]
pub fn randn(dims: Dim4, aftype: Aftype) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_randn(&mut temp as MutAfArray,
                 dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                 get_ffi_type(aftype.clone()) as c_int);
        Array {handle: temp}
    }
}
