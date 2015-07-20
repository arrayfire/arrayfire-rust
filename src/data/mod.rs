extern crate libc;
extern crate num;

use array::Array;
use dim4::Dim4;
use defines::AfError;
use defines::Aftype;
use self::libc::{uint8_t, c_int, c_uint, c_double};
use self::num::Complex;

use std::vec::Vec;

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
                seq_dims: c_int, afdtype: uint8_t) -> c_int;

    fn af_iota(out: MutAfArray, ndims: c_uint, dims: *const DimT,
               t_ndims: c_uint, tdims: *const DimT, afdtype: uint8_t) -> c_int;

    fn af_randu(out: MutAfArray, ndims: c_uint, dims: *const DimT, afdtype: uint8_t) -> c_int;
    fn af_randn(out: MutAfArray, ndims: c_uint, dims: *const DimT, afdtype: uint8_t) -> c_int;

    fn af_set_seed(seed: Uintl) -> c_int;
    fn af_get_seed(seed: *mut Uintl) -> c_int;

    fn af_identity(out: MutAfArray, ndims: c_uint, dims: *const DimT, afdtype: uint8_t) -> c_int;
    fn af_diag_create(out: MutAfArray, arr: AfArray, num: c_int) -> c_int;
    fn af_diag_extract(out: MutAfArray, arr: AfArray, num: c_int) -> c_int;
    fn af_join(out: MutAfArray, dim: c_int, first: AfArray, second: AfArray) -> c_int;
    fn af_join_many(out: MutAfArray, dim: c_int, n_arrays: c_uint, inpts: *const AfArray) -> c_int;

    fn af_tile(out: MutAfArray, arr: AfArray, x: c_uint, y: c_uint, z: c_uint, w: c_uint) -> c_int;
    fn af_reorder(o: MutAfArray, a: AfArray, x: c_uint, y: c_uint, z: c_uint, w: c_uint) -> c_int;
    fn af_shift(o: MutAfArray, a: AfArray, x: c_uint, y: c_uint, z: c_uint, w: c_uint) -> c_int;
    fn af_moddims(out: MutAfArray, arr: AfArray, ndims: c_uint, dims: *const DimT) -> c_int;

    fn af_flat(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_flip(out: MutAfArray, arr: AfArray, dim: c_uint) -> c_int;
    fn af_lower(out: MutAfArray, arr: AfArray, is_unit_diag: c_int) -> c_int;
    fn af_upper(out: MutAfArray, arr: AfArray, is_unit_diag: c_int) -> c_int;
}

pub trait ConstGenerator {
    fn generate(&self, dims: Dim4) -> Result<Array, AfError>;
}

#[allow(unused_mut)]
impl ConstGenerator for i64 {
    fn generate(&self, dims: Dim4) -> Result<Array, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_constant_long(&mut temp as MutAfArray, *self as Intl,
                                           dims.ndims() as c_uint,
                                           dims.get().as_ptr() as *const DimT);
            match err_val {
                0 => Ok(Array::from(temp)),
                _ => Err(AfError::from(err_val)),
            }
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for u64 {
    fn generate(&self, dims: Dim4) -> Result<Array, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_constant_ulong(&mut temp as MutAfArray, *self as Uintl,
                                            dims.ndims() as c_uint,
                                            dims.get().as_ptr() as *const DimT);
            match err_val {
                0 => Ok(Array::from(temp)),
                _ => Err(AfError::from(err_val)),
            }
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for Complex<f32> {
    fn generate(&self, dims: Dim4) -> Result<Array, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_constant_complex(&mut temp as MutAfArray,
                                              (*self).re as c_double, (*self).im as c_double,
                                              dims.ndims() as c_uint,
                                              dims.get().as_ptr() as *const DimT, 1);
            match err_val {
                0 => Ok(Array::from(temp)),
                _ => Err(AfError::from(err_val)),
            }
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for Complex<f64> {
    fn generate(&self, dims: Dim4) -> Result<Array, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_constant_complex(&mut temp as MutAfArray,
                                              (*self).re as c_double, (*self).im as c_double,
                                              dims.ndims() as c_uint,
                                              dims.get().as_ptr() as *const DimT, 3);
            match err_val {
                0 => Ok(Array::from(temp)),
                _ => Err(AfError::from(err_val)),
            }
        }
    }
}

#[allow(unused_mut)]
impl ConstGenerator for bool {
    fn generate(&self, dims: Dim4) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            af_constant(&mut temp as MutAfArray, *self as c_int as c_double,
                              dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT, 4);
            Array::from(temp)
        }
    }
}

macro_rules! cnst {
    ($rust_type:ty, $ffi_type:expr) => (
        #[allow(unused_mut)]
        impl ConstGenerator for $rust_type {
            fn generate(&self, dims: Dim4) -> Result<Array, AfError> {
                unsafe {
                    let mut temp: i64 = 0;
                    let err_val = af_constant(&mut temp as MutAfArray, *self as c_double,
                                              dims.ndims() as c_uint,
                                              dims.get().as_ptr() as *const DimT, $ffi_type);
                    match err_val {
                        0 => Ok(Array::from(temp)),
                        _ => Err(AfError::from(err_val)),
                    }
                }
            }
        }
    )
}

cnst!(f32 , 0);
cnst!(f64 , 2);
cnst!(i32 , 5);
cnst!(u32 , 6);
cnst!(u8  , 7);


pub fn constant<T : ConstGenerator>(cnst: T, dims: Dim4) -> Result<Array, AfError> {
    cnst.generate(dims)
}

#[allow(unused_mut)]
pub fn range(dims: Dim4, seq_dim: i32, aftype: Aftype) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val =af_range(&mut temp as MutAfArray,
                              dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                              seq_dim as c_int, aftype as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn iota(dims: Dim4, tdims: Dim4, aftype: Aftype) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val =af_iota(&mut temp as MutAfArray,
                             dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                             tdims.ndims() as c_uint, tdims.get().as_ptr() as *const DimT,
                             aftype as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

pub fn set_seed(seed: u64) -> Result<(), AfError> {
    unsafe {
        let err_val = af_set_seed(seed as Uintl);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn get_seed() -> Result<u64, AfError> {
    unsafe {
        let mut temp: u64 = 0;
        let err_val = af_get_seed(&mut temp as *mut Uintl);
        match err_val {
            0 => Ok(temp),
            _ => Err(AfError::from(err_val)),
        }
    }
}

macro_rules! data_gen_def {
    ($fn_name:ident, $ffi_name: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(dims: Dim4, aftype: Aftype) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                        dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                                        aftype as uint8_t);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

data_gen_def!(randu, af_randu);
data_gen_def!(randn, af_randn);
data_gen_def!(identity, af_identity);

#[allow(unused_mut)]
pub fn diag_create(input: &Array, dim: i32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_diag_create(&mut temp as MutAfArray, input.get() as AfArray, dim as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn diag_extract(input: &Array, dim: i32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_diag_extract(&mut temp as MutAfArray,
                                      input.get() as AfArray, dim as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn join(dim: i32, first: &Array, second: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_join(&mut temp as MutAfArray, dim as c_int,
                              first.get() as AfArray, second.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn join_many(dim: i32, inputs: Vec<&Array>) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_join_many(&mut temp as MutAfArray, dim as c_int,
                                   inputs.len() as c_uint, inputs.as_ptr() as *const AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

// Using macro to implement join many wrapper
#[macro_export]
macro_rules! join_many {
    [$dim: expr; $($x:ident),+] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
             )*
            let mut temp: i64 = 0;
            unsafe {
                let mut temp: i64 = 0;
                af_join_many(&mut temp as MutAfArray, $dim as c_int,
                             temp_vec.len() as c_uint, temp_vec.as_ptr() as *const AfArray);
                Array::from(temp)
            }
        }
    };
}

macro_rules! data_func_def {
    ($fn_name:ident, $ffi_name: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, dims: Dim4) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray, input.get() as AfArray,
                                        dims[0] as c_uint, dims[1] as c_uint,
                                        dims[2] as c_uint, dims[3] as c_uint);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

data_func_def!(tile, af_tile);
data_func_def!(reorder, af_reorder);
data_func_def!(shift, af_shift);

#[allow(unused_mut)]
pub fn moddims(input: &Array, dims: Dim4) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_moddims(&mut temp as MutAfArray, input.get() as AfArray,
                                 dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn flat(input: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_flat(&mut temp as MutAfArray, input.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn flip(input: &Array, dim: u32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_flip(&mut temp as MutAfArray, input.get() as AfArray, dim as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn lower(input: &Array, is_unit_diag: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_lower(&mut temp as MutAfArray,
                               input.get() as AfArray, is_unit_diag as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn upper(input: &Array, is_unit_diag: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_upper(&mut temp as MutAfArray,
                               input.get() as AfArray, is_unit_diag as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}
