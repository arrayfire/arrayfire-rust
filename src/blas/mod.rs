extern crate libc;

use array::Array;
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
pub fn matmul(lhs: &Array, rhs: &Array, optlhs: MatProp, optrhs: MatProp) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_matmul(&mut temp as MutAfArray, lhs.get() as AfArray, rhs.get() as AfArray,
                  to_u32(optlhs) as c_uint, to_u32(optrhs) as c_uint);
        Array::from(temp)
    }
}

#[allow(unused_mut)]
pub fn dot(lhs: &Array, rhs: &Array, optlhs: MatProp, optrhs: MatProp) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_dot(&mut temp as MutAfArray, lhs.get() as AfArray, rhs.get() as AfArray,
               to_u32(optlhs) as c_uint, to_u32(optrhs) as c_uint);
        Array::from(temp)
    }
}

#[allow(unused_mut)]
pub fn transpose(arr: &Array, conjugate: bool) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_transpose(&mut temp as MutAfArray, arr.get() as AfArray, conjugate as c_int);
        Array::from(temp)
    }
}

#[allow(unused_mut)]
pub fn transpose_inplace(arr: &Array, conjugate: bool) {
    unsafe {
        af_transpose_inplace(arr.get() as AfArray, conjugate as c_int);
    }
}
