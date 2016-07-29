extern crate libc;
extern crate num;

use dim4::Dim4;
use array::Array;
use defines::AfError;
use error::HANDLE_ERROR;
use self::libc::{c_int};
use data::{constant, tile};
use self::num::Complex;

type MutAfArray = *mut self::libc::c_longlong;
type MutDouble  = *mut self::libc::c_double;
type MutUint    = *mut self::libc::c_uint;
type AfArray    = self::libc::c_longlong;

use std::ops::{Add, Sub, Div, Mul, BitAnd, BitOr, BitXor, Not, Rem, Shl, Shr};

#[allow(dead_code)]
extern {
    fn af_add(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_sub(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_mul(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_div(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_lt(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_gt(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_le(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_ge(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_eq(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_or(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_neq(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_and(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_rem(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_mod(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_bitand(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_bitor(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_bitxor(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_bitshiftl(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_bitshiftr(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_minof(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_maxof(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_not(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_abs(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_arg(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_sign(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_ceil(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_round(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_trunc(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_floor(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_hypot(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_sin(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_cos(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_tan(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_asin(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_acos(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_atan(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_atan2(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_cplx2(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_root(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_pow(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_cplx(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_real(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_imag(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_conjg(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_sinh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_cosh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_tanh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_asinh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_acosh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_atanh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_pow2(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_exp(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_sigmoid(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_expm1(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_erf(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_erfc(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_log(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_log1p(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_log10(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_log2(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_sqrt(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_cbrt(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_factorial(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_tgamma(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_lgamma(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_iszero(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_isinf(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_isnan(out: MutAfArray, arr: AfArray) -> c_int;
}

/// Enables use of `!` on objects of type [Array](./struct.Array.html)
impl<'f> Not for &'f Array {
    type Output = Array;

    fn not(self) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_not(&mut temp as MutAfArray, self.get() as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    }
}

macro_rules! unary_func {
    ($fn_name: ident, $ffi_fn: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array) -> Array {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_fn(&mut temp as MutAfArray, input.get() as AfArray);
                HANDLE_ERROR(AfError::from(err_val));
                Array::from(temp)
            }
        }
    )
}

unary_func!(abs, af_abs);
unary_func!(arg, af_arg);
unary_func!(sign, af_sign);
unary_func!(round, af_round);
unary_func!(trunc, af_trunc);
unary_func!(floor, af_floor);
unary_func!(ceil, af_ceil);
unary_func!(sin, af_sin);
unary_func!(cos, af_cos);
unary_func!(tan, af_tan);
unary_func!(asin, af_asin);
unary_func!(acos, af_acos);
unary_func!(atan, af_atan);
unary_func!(cplx, af_cplx);
unary_func!(real, af_real);
unary_func!(imag, af_imag);
unary_func!(conjg, af_conjg);
unary_func!(sinh, af_sinh);
unary_func!(cosh, af_cosh);
unary_func!(tanh, af_tanh);
unary_func!(asinh, af_asinh);
unary_func!(acosh, af_acosh);
unary_func!(atanh, af_atanh);
unary_func!(pow2, af_pow2);
unary_func!(exp, af_exp);
unary_func!(sigmoid, af_sigmoid);
unary_func!(expm1, af_expm1);
unary_func!(erf, af_erf);
unary_func!(erfc, af_erfc);
unary_func!(log, af_log);
unary_func!(log1p, af_log1p);
unary_func!(log10, af_log10);
unary_func!(log2, af_log2);
unary_func!(sqrt, af_sqrt);
unary_func!(cbrt, af_cbrt);
unary_func!(factorial, af_factorial);
unary_func!(tgamma, af_tgamma);
unary_func!(lgamma, af_lgamma);
unary_func!(iszero, af_iszero);
unary_func!(isinf, af_isinf);
unary_func!(isnan, af_isnan);

macro_rules! binary_func {
    ($fn_name: ident, $ffi_fn: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(lhs: &Array, rhs: &Array) -> Array {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_fn(&mut temp as MutAfArray,
                                      lhs.get() as AfArray, rhs.get() as AfArray,
                                      0);
                HANDLE_ERROR(AfError::from(err_val));
                Array::from(temp)
            }
        }
    )
}

binary_func!(bitand, af_bitand);
binary_func!(bitor, af_bitor);
binary_func!(bitxor, af_bitxor);
binary_func!(neq, af_neq);
binary_func!(and, af_and);
binary_func!(or, af_or);
binary_func!(minof, af_minof);
binary_func!(maxof, af_maxof);
binary_func!(hypot, af_hypot);

pub trait Convertable {
    fn convert(&self) -> Array;
}

macro_rules! convertable_type_def {
    ($rust_type: ty) => (
        impl Convertable for $rust_type {
            fn convert(&self) -> Array {
                constant(*self, Dim4::new(&[1,1,1,1]))
            }
        }
    )
}

convertable_type_def!(u64);
convertable_type_def!(i64);
convertable_type_def!(f64);
convertable_type_def!(f32);
convertable_type_def!(i32);
convertable_type_def!(u32);
convertable_type_def!(u8);

impl Convertable for Array {
    fn convert(&self) -> Array {
        self.clone()
    }
}

macro_rules! overloaded_binary_func {
    ($fn_name: ident, $help_name: ident, $ffi_name: ident) => (
        fn $help_name(lhs: &Array, rhs: &Array, batch: bool) -> Array {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                     lhs.get() as AfArray, rhs.get() as AfArray,
                                     batch as c_int);
                HANDLE_ERROR(AfError::from(err_val));
                Array::from(temp)
            }
        }

        pub fn $fn_name<T, U> (arg1: &T, arg2: &U, batch: bool) -> Array where T: Convertable, U: Convertable {
            let lhs = arg1.convert();
            let rhs = arg2.convert();
            match (lhs.is_scalar(), rhs.is_scalar()) {
                ( true, false) => {
                    let l = tile(&lhs, rhs.dims());
                    $help_name(&l, &rhs, batch)
                },
                (false,  true) => {
                    let r = tile(&rhs, lhs.dims());
                    $help_name(&lhs, &r, batch)
                },
                _ => $help_name(&lhs, &rhs, batch),
            }
        }
    )
}

// thanks to Umar Arshad for the idea on how to
// implement overloaded function
overloaded_binary_func!(add, add_helper, af_add);
overloaded_binary_func!(sub, sub_helper, af_sub);
overloaded_binary_func!(mul, mul_helper, af_mul);
overloaded_binary_func!(div, div_helper, af_div);
overloaded_binary_func!(rem, rem_helper, af_rem);
overloaded_binary_func!(shiftl, shiftl_helper, af_bitshiftl);
overloaded_binary_func!(shiftr, shiftr_helper, af_bitshiftr);
overloaded_binary_func!(lt, lt_helper, af_lt);
overloaded_binary_func!(gt, gt_helper, af_gt);
overloaded_binary_func!(le, le_helper, af_le);
overloaded_binary_func!(ge, ge_helper, af_ge);
overloaded_binary_func!(eq, eq_helper, af_eq);
overloaded_binary_func!(modulo, modulo_helper, af_mod);
overloaded_binary_func!(atan2, atan2_helper, af_atan2);
overloaded_binary_func!(cplx2, cplx2_helper, af_cplx2);
overloaded_binary_func!(root, root_helper, af_root);
overloaded_binary_func!(pow, pow_helper, af_pow);

macro_rules! arith_scalar_func {
    ($rust_type: ty, $op_name:ident, $fn_name: ident, $ffi_fn: ident) => (
        impl<'f> $op_name<$rust_type> for &'f Array {
            type Output = Array;

            fn $fn_name(self, rhs: $rust_type) -> Array {
                let cnst_arr = constant(rhs, self.dims());
                unsafe {
                    let mut temp: i64 = 0;
                    let err_val = $ffi_fn(&mut temp as MutAfArray, self.get() as AfArray,
                                          cnst_arr.get() as AfArray, 0); 
                    HANDLE_ERROR(AfError::from(err_val));
                    Array::from(temp)
                }
            }
        }
    )
}

macro_rules! arith_scalar_spec {
    ($ty_name:ty) => (
        arith_scalar_func!($ty_name, Add, add, af_add);
        arith_scalar_func!($ty_name, Sub, sub, af_sub);
        arith_scalar_func!($ty_name, Mul, mul, af_mul);
        arith_scalar_func!($ty_name, Div, div, af_div);
    )
}

arith_scalar_spec!(Complex<f64>);
arith_scalar_spec!(Complex<f32>);
arith_scalar_spec!(f64);
arith_scalar_spec!(f32);
arith_scalar_spec!(u64);
arith_scalar_spec!(i64);
arith_scalar_spec!(u32);
arith_scalar_spec!(i32);
arith_scalar_spec!(u8);

macro_rules! arith_func {
    ($op_name:ident, $fn_name:ident, $ffi_fn: ident) => (
        impl $op_name<Array> for Array {
            type Output = Array;

            fn $fn_name(self, rhs: Array) -> Array {
                unsafe {
                    let mut temp: i64 = 0;
                    let err_val = $ffi_fn(&mut temp as MutAfArray,
                                          self.get() as AfArray, rhs.get() as AfArray, 0);
                    HANDLE_ERROR(AfError::from(err_val));
                    Array::from(temp)
                }
            }
        }
    )
}

arith_func!(Add, add, af_add);
arith_func!(Sub, sub, af_sub);
arith_func!(Mul, mul, af_mul);
arith_func!(Div, div, af_div);
arith_func!(Rem, rem, af_rem);
arith_func!(BitAnd, bitand, af_bitand);
arith_func!(BitOr, bitor, af_bitor);
arith_func!(BitXor, bitxor, af_bitxor);
arith_func!(Shl, shl, af_bitshiftl);
arith_func!(Shr, shr, af_bitshiftr);

#[cfg(op_assign)]
mod op_assign {

use array::Array;
use super::*;
use index::{Indexer, assign_gen};
use seq::Seq;
use std::mem;
use std::ops::{AddAssign, SubAssign, DivAssign, MulAssign, RemAssign};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};


macro_rules! arith_assign_func {
    ($op_name:ident, $fn_name:ident, $func: ident) => (
        impl $op_name<Array> for Array {

            #[allow(unused_variables)]
            fn $fn_name(&mut self, rhs: Array) {
                let mut idxrs = Indexer::new();
                idxrs.set_index(&Seq::<f32>::default(), 0, Some(false));
                idxrs.set_index(&Seq::<f32>::default(), 1, Some(false));
                let tmp = assign_gen(self as &Array, &idxrs,
                                     & $func(self as &Array, &rhs, false));
                mem::replace(self, tmp);
            }
        }
    )
}

arith_assign_func!(AddAssign, add_assign, add);
arith_assign_func!(SubAssign, sub_assign, sub);
arith_assign_func!(MulAssign, mul_assign, mul);
arith_assign_func!(DivAssign, div_assign, div);
arith_assign_func!(RemAssign, rem_assign, rem);
arith_assign_func!(ShlAssign, shl_assign, shiftl);
arith_assign_func!(ShrAssign, shr_assign, shiftr);

macro_rules! bit_assign_func {
    ($op_name:ident, $fn_name:ident, $func: ident) => (
        impl $op_name<Array> for Array {

            #[allow(unused_variables)]
            fn $fn_name(&mut self, rhs: Array) {
                let mut idxrs = Indexer::new();
                idxrs.set_index(&Seq::<f32>::default(), 0, Some(false));
                idxrs.set_index(&Seq::<f32>::default(), 1, Some(false));
                let tmp = assign_gen(self as &Array, &idxrs, & $func(self as &Array, &rhs));
                mem::replace(self, tmp);
            }
        }
    )
}

bit_assign_func!(BitAndAssign, bitand_assign, bitand);
bit_assign_func!(BitOrAssign, bitor_assign, bitor);
bit_assign_func!(BitXorAssign, bitxor_assign, bitxor);

}
