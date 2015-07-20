extern crate libc;

use array::Array;
use defines::AfError;
use defines::MatProp;
use self::libc::{c_uint, c_int};
use util::to_u32;

type MutAfArray = *mut self::libc::c_longlong;
type AfArray    = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_matmul(out: MutAfArray, lhs: AfArray, rhs: AfArray,
                 optlhs: c_uint, optrhs: c_uint) -> c_int;

    fn af_dot(out: MutAfArray, lhs: AfArray, rhs: AfArray,
              optlhs: c_uint, optrhs: c_uint) -> c_int;

    fn af_transpose(out: MutAfArray, arr: AfArray, conjugate: c_int) -> c_int;
    fn af_transpose_inplace(arr: AfArray, conjugate: c_int) -> c_int;
}

#[allow(unused_mut)]
pub fn matmul(lhs: &Array, rhs: &Array,
              optlhs: MatProp, optrhs: MatProp) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_matmul(&mut temp as MutAfArray,
                                lhs.get() as AfArray, rhs.get() as AfArray,
                                to_u32(optlhs) as c_uint, to_u32(optrhs) as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn dot(lhs: &Array, rhs: &Array,
           optlhs: MatProp, optrhs: MatProp) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_dot(&mut temp as MutAfArray,
                             lhs.get() as AfArray, rhs.get() as AfArray,
                             to_u32(optlhs) as c_uint, to_u32(optrhs) as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn transpose(arr: &Array, conjugate: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_transpose(&mut temp as MutAfArray,
                                   arr.get() as AfArray, conjugate as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn transpose_inplace(arr: &mut Array, conjugate: bool) -> Result<(), AfError> {
    unsafe {
        let err_val = af_transpose_inplace(arr.get() as AfArray, conjugate as c_int);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}
