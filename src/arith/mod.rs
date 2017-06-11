extern crate libc;
extern crate num;

use dim4::Dim4;
use array::Array;
use defines::{AfError, DType, Scalar};
use error::HANDLE_ERROR;
use self::libc::{c_int};
use data::{constant, constant_t, tile};
use self::num::Complex;
use util::{AfArray, Complex32, Complex64, MutAfArray};
use std::ops::Neg;
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
    fn af_clamp(out: MutAfArray, inp: AfArray, lo: AfArray, hi: AfArray, batch: c_int) -> c_int;

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
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => (
        #[doc=$doc_str]
        ///
        /// This is an element wise unary operation.
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

unary_func!("Computes absolute value", abs, af_abs);
unary_func!("Computes phase value", arg, af_arg);
unary_func!("Computes the sign of input Array values", sign, af_sign);
unary_func!("Round the values in an Array", round, af_round);
unary_func!("Truncate the values in an Array", trunc, af_trunc);
unary_func!("Floor the values in an Array", floor, af_floor);
unary_func!("Ceil the values in an Array", ceil, af_ceil);
unary_func!("Compute sin", sin, af_sin);
unary_func!("Compute cos", cos, af_cos);
unary_func!("Compute tan", tan, af_tan);
unary_func!("Compute asin", asin, af_asin);
unary_func!("Compute acos", acos, af_acos);
unary_func!("Compute atan", atan, af_atan);
unary_func!("Create a complex Array from real Array", cplx, af_cplx);
unary_func!("Extract real values from a complex Array", real, af_real);
unary_func!("Extract imaginary values from a complex Array", imag, af_imag);
unary_func!("Compute the complex conjugate", conjg, af_conjg);
unary_func!("Compute sinh", sinh, af_sinh);
unary_func!("Compute cosh", cosh, af_cosh);
unary_func!("Compute tanh", tanh, af_tanh);
unary_func!("Compute asinh", asinh, af_asinh);
unary_func!("Compute acosh", acosh, af_acosh);
unary_func!("Compute atanh", atanh, af_atanh);
unary_func!("Compute two raised to the power of value", pow2, af_pow2);
unary_func!("Compute e raised to the power of value", exp, af_exp);
unary_func!("Compute sigmoid function", sigmoid, af_sigmoid);
unary_func!("Compute e raised to the power of value -1", expm1, af_expm1);
unary_func!("Compute error function value", erf, af_erf);
unary_func!("Compute the complementary error function value", erfc, af_erfc);
unary_func!("Compute the natural logarithm", log, af_log);
unary_func!("Compute the logarithm of input Array + 1", log1p, af_log1p);
unary_func!("Compute logarithm base 10", log10, af_log10);
unary_func!("Compute logarithm base 2", log2, af_log2);
unary_func!("Compute the square root", sqrt, af_sqrt);
unary_func!("Compute the cube root", cbrt, af_cbrt);
unary_func!("Compute the factorial", factorial, af_factorial);
unary_func!("Compute gamma function", tgamma, af_tgamma);
unary_func!("Compute the logarithm of absolute values of gamma function", lgamma, af_lgamma);
unary_func!("Check if values are zero", iszero, af_iszero);
unary_func!("Check if values are infinity", isinf, af_isinf);
unary_func!("Check if values are NaN", isnan, af_isnan);

macro_rules! binary_func {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => (
        #[doc=$doc_str]
        ///
        /// This is an element wise binary operation.
        #[allow(unused_mut)]
        pub fn $fn_name(lhs: &Array, rhs: &Array, batch: bool) -> Array {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_fn(&mut temp as MutAfArray,
                                      lhs.get() as AfArray, rhs.get() as AfArray,
                                      batch as c_int);
                HANDLE_ERROR(AfError::from(err_val));
                Array::from(temp)
            }
        }
    )
}

binary_func!("Elementwise AND(bit) operation of two Arrays", bitand, af_bitand);
binary_func!("Elementwise OR(bit) operation of two Arrays", bitor, af_bitor);
binary_func!("Elementwise XOR(bit) operation of two Arrays", bitxor, af_bitxor);
binary_func!("Elementwise not equals comparison of two Arrays", neq, af_neq);
binary_func!("Elementwise logical and operation of two Arrays", and, af_and);
binary_func!("Elementwise logical or operation of two Arrays", or, af_or);
binary_func!("Elementwise minimum operation of two Arrays", minof, af_minof);
binary_func!("Elementwise maximum operation of two Arrays", maxof, af_maxof);
binary_func!("Compute length of hypotenuse of two Arrays", hypot, af_hypot);

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

convertable_type_def!(Complex<f64>);
convertable_type_def!(Complex<f32>);
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
    ($doc_str: expr, $fn_name: ident, $help_name: ident, $ffi_name: ident) => (
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

        #[doc=$doc_str]
        ///
        /// This is a binary elementwise operation.
        ///
        ///# Parameters
        ///
        /// - `arg1`is an argument that implements an internal trait `Convertable`.
        /// - `arg2`is an argument that implements an internal trait `Convertable`.
        /// - `batch` is an boolean that indicates if the current operation is an batch operation.
        ///
        /// Both parameters `arg1` and `arg2` can be either an Array or a value of rust integral
        /// type.
        ///
        ///# Return Values
        ///
        /// An Array with results of the binary operation.
        ///
        /// In the case of comparison operations such as the following, the type of output
        /// Array is [DType::B8](./enum.DType.html). To retrieve the results of such boolean output
        /// to host, an array of 8-bit wide types(eg. u8, i8) should be used since ArrayFire's internal
        /// implementation uses char for boolean.
        ///
        /// * [gt](./fn.gt.html)
        /// * [lt](./fn.lt.html)
        /// * [ge](./fn.ge.html)
        /// * [le](./fn.le.html)
        /// * [eq](./fn.eq.html)
        ///
        ///# Note
        ///
        /// The trait `Convertable` essentially translates to a scalar native type on rust or Array.
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
overloaded_binary_func!("Addition of two Arrays", add, add_helper, af_add);
overloaded_binary_func!("Subtraction of two Arrays", sub, sub_helper, af_sub);
overloaded_binary_func!("Multiplication of two Arrays", mul, mul_helper, af_mul);
overloaded_binary_func!("Division of two Arrays", div, div_helper, af_div);
overloaded_binary_func!("Compute remainder from two Arrays", rem, rem_helper, af_rem);
overloaded_binary_func!("Compute left shift", shiftl, shiftl_helper, af_bitshiftl);
overloaded_binary_func!("Compute right shift", shiftr, shiftr_helper, af_bitshiftr);
overloaded_binary_func!("Perform `less than` comparison operation", lt, lt_helper, af_lt);
overloaded_binary_func!("Perform `greater than` comparison operation", gt, gt_helper, af_gt);
overloaded_binary_func!("Perform `less than equals` comparison operation", le, le_helper, af_le);
overloaded_binary_func!("Perform `greater than equals` comparison operation", ge, ge_helper, af_ge);
overloaded_binary_func!("Perform `equals` comparison operation", eq, eq_helper, af_eq);
overloaded_binary_func!("Compute modulo of two Arrays", modulo, modulo_helper, af_mod);
overloaded_binary_func!("Calculate atan2 of two Arrays", atan2, atan2_helper, af_atan2);
overloaded_binary_func!("Create complex array from two Arrays", cplx2, cplx2_helper, af_cplx2);
overloaded_binary_func!("Compute root", root, root_helper, af_root);
overloaded_binary_func!("Computer power", pow, pow_helper, af_pow);

/// Clamp the values of Array
///
/// # Parameters
///
/// - `arg1`is an argument that implements an internal trait `Convertable`.
/// - `arg2`is an argument that implements an internal trait `Convertable`.
/// - `batch` is an boolean that indicates if the current operation is an batch operation.
///
/// Both parameters `arg1` and `arg2` can be either an Array or a value of rust integral
/// type.
///
/// # Return Values
///
/// An Array with results of the binary operation.
///
/// # Note
///
/// The trait `Convertable` essentially translates to a scalar native type on rust or Array.
pub fn clamp<T, U> (input: &Array, arg1: &T, arg2: &U, batch: bool) -> Array
    where T: Convertable, U: Convertable
{
    let clamp_helper = |lo: &Array, hi: &Array| {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_clamp(&mut temp as MutAfArray, input.get() as AfArray,
                                   lo.get() as AfArray, hi.get() as AfArray,
                                   batch as c_int);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    };

    let lo = arg1.convert();
    let hi = arg2.convert();
    match (lo.is_scalar(), hi.is_scalar()) {
        ( true, false) => {
            let l = tile(&lo, hi.dims());
            clamp_helper(&l, &hi)
        },
        (false,  true) => {
            let r = tile(&hi, lo.dims());
            clamp_helper(&lo, &r)
        },
        _ => clamp_helper(&lo, &hi),
    }
}

macro_rules! arith_scalar_func {
    ($rust_type: ty, $op_name:ident, $fn_name: ident) => (
        impl<'f> $op_name<$rust_type> for &'f Array {
            type Output = Array;

            fn $fn_name(self, rhs: $rust_type) -> Array {
                let temp = rhs.clone();
                $fn_name(self, &temp, false)
            }
        }

        impl $op_name<$rust_type> for Array {
            type Output = Array;

            fn $fn_name(self, rhs: $rust_type) -> Array {
                let temp = rhs.clone();
                $fn_name(&self, &temp, false)
            }
        }
    )
}

macro_rules! arith_scalar_spec {
    ($ty_name:ty) => (
        arith_scalar_func!($ty_name, Add, add);
        arith_scalar_func!($ty_name, Sub, sub);
        arith_scalar_func!($ty_name, Mul, mul);
        arith_scalar_func!($ty_name, Div, div);
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
    ($op_name:ident, $fn_name:ident, $delegate:ident) => (
        impl $op_name<Array> for Array {
            type Output = Array;

            fn $fn_name(self, rhs: Array) -> Array {
                $delegate(&self, &rhs, false)
            }
        }

        impl<'a> $op_name<&'a Array> for Array {
            type Output = Array;

            fn $fn_name(self, rhs: &'a Array) -> Array {
                $delegate(&self, rhs, false)
            }
        }

        impl<'a> $op_name<Array> for &'a Array {
            type Output = Array;

            fn $fn_name(self, rhs: Array) -> Array {
                $delegate(self, &rhs, false)
            }
        }

        impl<'a, 'b> $op_name<&'a Array> for &'b Array {
            type Output = Array;

            fn $fn_name(self, rhs: &'a Array) -> Array {
                $delegate(self, rhs, false)
            }
        }
    )
}

arith_func!(Add   , add   , add   );
arith_func!(Sub   , sub   , sub   );
arith_func!(Mul   , mul   , mul   );
arith_func!(Div   , div   , div   );
arith_func!(Rem   , rem   , rem   );
arith_func!(Shl   , shl   , shiftl);
arith_func!(Shr   , shr   , shiftr);
arith_func!(BitAnd, bitand, bitand);
arith_func!(BitOr , bitor , bitor );
arith_func!(BitXor, bitxor, bitxor);

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
                let tmp_seq   = Seq::<f32>::default();
                let mut idxrs = Indexer::new();
                idxrs.set_index(&tmp_seq, 0, Some(false));
                idxrs.set_index(&tmp_seq, 1, Some(false));
                idxrs.set_index(&tmp_seq, 2, Some(false));
                idxrs.set_index(&tmp_seq, 3, Some(false));
                let tmp = assign_gen(self as &Array, &idxrs,
                                     & $func(self as &Array, &rhs, false));
                let old = mem::replace(self, tmp);
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
                let tmp_seq   = Seq::<f32>::default();
                let mut idxrs = Indexer::new();
                idxrs.set_index(&tmp_seq, 0, Some(false));
                idxrs.set_index(&tmp_seq, 1, Some(false));
                idxrs.set_index(&tmp_seq, 2, Some(false));
                idxrs.set_index(&tmp_seq, 3, Some(false));
                let tmp = assign_gen(self as &Array, &idxrs, & $func(self as &Array, &rhs, false));
                let old = mem::replace(self, tmp);
            }
        }
    )
}

bit_assign_func!(BitAndAssign, bitand_assign, bitand);
bit_assign_func!(BitOrAssign, bitor_assign, bitor);
bit_assign_func!(BitXorAssign, bitxor_assign, bitxor);

}

///Implement negation trait for Array
impl Neg for Array {
    type Output = Array;

    fn neg(self) -> Self::Output {
        match self.get_type() {
            DType::S64 => (constant_t(Scalar::S64(0 as i64), self.dims(), DType::S64) - self),
            DType::U64 => (constant_t(Scalar::U64(0 as u64), self.dims(), DType::U64) - self),
            DType::C32 => (constant_t(Scalar::C32(Complex32::new(0.0, 0.0)), self.dims(), DType::C32) - self),
            DType::C64 => (constant_t(Scalar::C64(Complex64::new(0.0, 0.0)), self.dims(), DType::C64) - self),
            DType::F32 => (constant_t(Scalar::F32(0 as f32), self.dims(), DType::F32) - self),
            DType::F64 => (constant_t(Scalar::F64(0 as f64), self.dims(), DType::F64) - self),
            DType::B8  => (constant_t(Scalar::B8 (false   ), self.dims(), DType::B8 ) - self),
            DType::S32 => (constant_t(Scalar::S32(0 as i32), self.dims(), DType::S32) - self),
            DType::U32 => (constant_t(Scalar::U32(0 as u32), self.dims(), DType::U32) - self),
            DType::U8  => (constant_t(Scalar::U8 (0 as u8 ), self.dims(), DType::U8 ) - self),
            DType::S16 => (constant_t(Scalar::S16(0 as i16), self.dims(), DType::S16) - self),
            DType::U16 => (constant_t(Scalar::U16(0 as u16), self.dims(), DType::U16) - self),
        }
    }
}
