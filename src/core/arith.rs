use super::array::Array;
use super::data::{constant, tile, ConstGenerator};
use super::defines::AfError;
use super::dim4::Dim4;
use super::error::HANDLE_ERROR;
use super::util::{af_array, HasAfEnum, ImplicitPromote, IntegralType};
use num::Zero;

use libc::c_int;
use num::Complex;
use std::mem;
use std::ops::Neg;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

extern "C" {
    fn af_add(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_sub(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_mul(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_div(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;

    fn af_lt(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_gt(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_le(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_ge(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_eq(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_or(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;

    fn af_neq(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_and(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_rem(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_mod(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;

    fn af_bitand(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_bitor(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_bitxor(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_bitshiftl(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_bitshiftr(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_minof(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_maxof(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_clamp(
        out: *mut af_array,
        inp: af_array,
        lo: af_array,
        hi: af_array,
        batch: bool,
    ) -> c_int;

    fn af_not(out: *mut af_array, arr: af_array) -> c_int;
    fn af_abs(out: *mut af_array, arr: af_array) -> c_int;
    fn af_arg(out: *mut af_array, arr: af_array) -> c_int;
    fn af_sign(out: *mut af_array, arr: af_array) -> c_int;
    fn af_ceil(out: *mut af_array, arr: af_array) -> c_int;
    fn af_round(out: *mut af_array, arr: af_array) -> c_int;
    fn af_trunc(out: *mut af_array, arr: af_array) -> c_int;
    fn af_floor(out: *mut af_array, arr: af_array) -> c_int;

    fn af_hypot(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;

    fn af_sin(out: *mut af_array, arr: af_array) -> c_int;
    fn af_cos(out: *mut af_array, arr: af_array) -> c_int;
    fn af_tan(out: *mut af_array, arr: af_array) -> c_int;
    fn af_asin(out: *mut af_array, arr: af_array) -> c_int;
    fn af_acos(out: *mut af_array, arr: af_array) -> c_int;
    fn af_atan(out: *mut af_array, arr: af_array) -> c_int;

    fn af_atan2(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_cplx2(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_root(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;
    fn af_pow(out: *mut af_array, lhs: af_array, rhs: af_array, batch: bool) -> c_int;

    fn af_cplx(out: *mut af_array, arr: af_array) -> c_int;
    fn af_real(out: *mut af_array, arr: af_array) -> c_int;
    fn af_imag(out: *mut af_array, arr: af_array) -> c_int;
    fn af_conjg(out: *mut af_array, arr: af_array) -> c_int;
    fn af_sinh(out: *mut af_array, arr: af_array) -> c_int;
    fn af_cosh(out: *mut af_array, arr: af_array) -> c_int;
    fn af_tanh(out: *mut af_array, arr: af_array) -> c_int;
    fn af_asinh(out: *mut af_array, arr: af_array) -> c_int;
    fn af_acosh(out: *mut af_array, arr: af_array) -> c_int;
    fn af_atanh(out: *mut af_array, arr: af_array) -> c_int;
    fn af_pow2(out: *mut af_array, arr: af_array) -> c_int;
    fn af_exp(out: *mut af_array, arr: af_array) -> c_int;
    fn af_sigmoid(out: *mut af_array, arr: af_array) -> c_int;
    fn af_expm1(out: *mut af_array, arr: af_array) -> c_int;
    fn af_erf(out: *mut af_array, arr: af_array) -> c_int;
    fn af_erfc(out: *mut af_array, arr: af_array) -> c_int;
    fn af_log(out: *mut af_array, arr: af_array) -> c_int;
    fn af_log1p(out: *mut af_array, arr: af_array) -> c_int;
    fn af_log10(out: *mut af_array, arr: af_array) -> c_int;
    fn af_log2(out: *mut af_array, arr: af_array) -> c_int;
    fn af_sqrt(out: *mut af_array, arr: af_array) -> c_int;
    fn af_rsqrt(out: *mut af_array, arr: af_array) -> c_int;
    fn af_cbrt(out: *mut af_array, arr: af_array) -> c_int;
    fn af_factorial(out: *mut af_array, arr: af_array) -> c_int;
    fn af_tgamma(out: *mut af_array, arr: af_array) -> c_int;
    fn af_lgamma(out: *mut af_array, arr: af_array) -> c_int;
    fn af_iszero(out: *mut af_array, arr: af_array) -> c_int;
    fn af_isinf(out: *mut af_array, arr: af_array) -> c_int;
    fn af_isnan(out: *mut af_array, arr: af_array) -> c_int;
    fn af_bitnot(out: *mut af_array, arr: af_array) -> c_int;
}

/// Enables use of `!` on objects of type [Array](./struct.Array.html)
impl<'f, T> Not for &'f Array<T>
where
    T: HasAfEnum,
{
    type Output = Array<T>;

    fn not(self) -> Self::Output {
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_not(&mut temp as *mut af_array, self.get());
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }
}

macro_rules! unary_func {
    [$doc_str: expr, $fn_name: ident, $ffi_fn: ident, $out_type: ident] => (
        #[doc=$doc_str]
        ///
        /// This is an element wise unary operation.
        pub fn $fn_name<T: HasAfEnum>(input: &Array<T>) -> Array< T::$out_type >
        where T::$out_type: HasAfEnum {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_fn(&mut temp as *mut af_array, input.get());
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    )
}

unary_func!("Computes absolute value", abs, af_abs, AbsOutType);
unary_func!("Computes phase value", arg, af_arg, ArgOutType);

unary_func!(
    "Truncate the values in an Array",
    trunc,
    af_trunc,
    AbsOutType
);
unary_func!(
    "Computes the sign of input Array values",
    sign,
    af_sign,
    AbsOutType
);
unary_func!("Round the values in an Array", round, af_round, AbsOutType);
unary_func!("Floor the values in an Array", floor, af_floor, AbsOutType);
unary_func!("Ceil the values in an Array", ceil, af_ceil, AbsOutType);

unary_func!("Compute sigmoid function", sigmoid, af_sigmoid, AbsOutType);
unary_func!(
    "Compute e raised to the power of value -1",
    expm1,
    af_expm1,
    AbsOutType
);
unary_func!("Compute error function value", erf, af_erf, AbsOutType);
unary_func!(
    "Compute the complementary error function value",
    erfc,
    af_erfc,
    AbsOutType
);

unary_func!("Compute logarithm base 10", log10, af_log10, AbsOutType);
unary_func!(
    "Compute the logarithm of input Array + 1",
    log1p,
    af_log1p,
    AbsOutType
);
unary_func!("Compute logarithm base 2", log2, af_log2, AbsOutType);

unary_func!("Compute the cube root", cbrt, af_cbrt, AbsOutType);
unary_func!("Compute gamma function", tgamma, af_tgamma, AbsOutType);
unary_func!(
    "Compute the logarithm of absolute values of gamma function",
    lgamma,
    af_lgamma,
    AbsOutType
);

unary_func!("Compute acosh", acosh, af_acosh, UnaryOutType);
unary_func!("Compute acos", acos, af_acos, UnaryOutType);
unary_func!("Compute asin", asin, af_asin, UnaryOutType);
unary_func!("Compute asinh", asinh, af_asinh, UnaryOutType);
unary_func!("Compute atan", atan, af_atan, UnaryOutType);
unary_func!("Compute atanh", atanh, af_atanh, UnaryOutType);
unary_func!("Compute cos", cos, af_cos, UnaryOutType);
unary_func!("Compute cosh", cosh, af_cosh, UnaryOutType);
unary_func!(
    "Compute e raised to the power of value",
    exp,
    af_exp,
    UnaryOutType
);
unary_func!("Compute the natural logarithm", log, af_log, UnaryOutType);
unary_func!("Compute sin", sin, af_sin, UnaryOutType);
unary_func!("Compute sinh", sinh, af_sinh, UnaryOutType);
unary_func!("Compute the square root", sqrt, af_sqrt, UnaryOutType);
unary_func!(
    "Compute the reciprocal square root",
    rsqrt,
    af_rsqrt,
    UnaryOutType
);
unary_func!("Compute tan", tan, af_tan, UnaryOutType);
unary_func!("Compute tanh", tanh, af_tanh, UnaryOutType);

unary_func!(
    "Extract real values from a complex Array",
    real,
    af_real,
    AbsOutType
);
unary_func!(
    "Extract imaginary values from a complex Array",
    imag,
    af_imag,
    AbsOutType
);
unary_func!(
    "Create a complex Array from real Array",
    cplx,
    af_cplx,
    ComplexOutType
);
unary_func!(
    "Compute the complex conjugate",
    conjg,
    af_conjg,
    ComplexOutType
);
unary_func!(
    "Compute two raised to the power of value",
    pow2,
    af_pow2,
    UnaryOutType
);
unary_func!(
    "Compute the factorial",
    factorial,
    af_factorial,
    UnaryOutType
);

macro_rules! unary_boolean_func {
    [$doc_str: expr, $fn_name: ident, $ffi_fn: ident] => (
        #[doc=$doc_str]
        ///
        /// This is an element wise unary operation.
        pub fn $fn_name<T: HasAfEnum>(input: &Array<T>) -> Array<bool> {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_fn(&mut temp as *mut af_array, input.get());
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    )
}

unary_boolean_func!("Check if values are zero", iszero, af_iszero);
unary_boolean_func!("Check if values are infinity", isinf, af_isinf);
unary_boolean_func!("Check if values are NaN", isnan, af_isnan);

macro_rules! binary_func {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => {
        #[doc=$doc_str]
        ///
        /// This is an element wise binary operation.
        ///
        /// # Important Notes
        ///
        /// - If shape/dimensions of `lhs` and `rhs` are same, the value of `batch` parameter
        ///   has no effect.
        ///
        /// - If shape/dimensions of `lhs` and `rhs` are different, the value of `batch` has
        ///   to be set to `true`. In this case, the shapes of `lhs` and `rhs` have to satisfy the
        ///   following criteria:
        ///   - Same number of elements in `lhs` and `rhs` along a given dimension/axis
        ///   - Only one element in `lhs` or `rhs` along a given dimension/axis
        pub fn $fn_name<A, B>(lhs: &Array<A>, rhs: &Array<B>, batch: bool) -> Array<A::Output>
        where
            A: ImplicitPromote<B>,
            B: ImplicitPromote<A>,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_fn(&mut temp as *mut af_array, lhs.get(), rhs.get(), batch);
                HANDLE_ERROR(AfError::from(err_val));
                Into::<Array<A::Output>>::into(temp)
            }
        }
    };
}

binary_func!(
    "Elementwise AND(bit) operation of two Arrays",
    bitand,
    af_bitand
);
binary_func!(
    "Elementwise OR(bit) operation of two Arrays",
    bitor,
    af_bitor
);
binary_func!(
    "Elementwise XOR(bit) operation of two Arrays",
    bitxor,
    af_bitxor
);
binary_func!(
    "Elementwise minimum operation of two Arrays",
    minof,
    af_minof
);
binary_func!(
    "Elementwise maximum operation of two Arrays",
    maxof,
    af_maxof
);
binary_func!(
    "Compute length of hypotenuse of two Arrays",
    hypot,
    af_hypot
);

/// Type Trait to convert to an [Array](./struct.Array.html)
///
/// Generic functions that overload the binary operations such as add, div, mul, rem, ge etc. are
/// bound by this trait to allow combinations of scalar values and Array objects as parameters
/// to those functions.
///
/// Internally, Convertable trait is implemented by following types.
///
/// - f32
/// - f64
/// - num::Complex\<f32\>
/// - num::Complex\<f64\>
/// - bool
/// - i32
/// - u32
/// - u8
/// - i64
/// - u64
/// - i16
/// - u16
///
pub trait Convertable {
    /// This type alias always points to `Self` which is the
    /// type of [Array](./struct.Array.html) returned by the
    /// trait method [convert](./trait.Convertable.html#tymethod.convert).
    type OutType: HasAfEnum;

    /// Get an Array of implementors type
    fn convert(&self) -> Array<Self::OutType>;
}

impl<T> Convertable for T
where
    T: Clone + ConstGenerator<OutType = T>,
{
    type OutType = T;

    fn convert(&self) -> Array<Self::OutType> {
        constant(*self, Dim4::new(&[1, 1, 1, 1]))
    }
}

impl<T: HasAfEnum> Convertable for Array<T> {
    type OutType = T;

    fn convert(&self) -> Array<Self::OutType> {
        self.clone()
    }
}

macro_rules! overloaded_binary_func {
    ($doc_str: expr, $fn_name: ident, $help_name: ident, $ffi_name: ident) => {
        fn $help_name<A, B>(lhs: &Array<A>, rhs: &Array<B>, batch: bool) -> Array<A::Output>
        where
            A: ImplicitPromote<B>,
            B: ImplicitPromote<A>,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(&mut temp as *mut af_array, lhs.get(), rhs.get(), batch);
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
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
        ///# Important Notes
        ///
        /// - If shape/dimensions of `arg1` and `arg2` are same, the value of `batch` parameter
        ///   has no effect.
        ///
        /// - If shape/dimensions of `arg1` and `arg2` are different, the value of `batch` has
        ///   to be set to `true`. In this case, the shapes of `arg1` and `arg2` have to satisfy the
        ///   following criteria:
        ///   - Same number of elements in `arg1` and `arg2` along a given dimension/axis
        ///   - Only one element in `arg1` or `arg2` along a given dimension/axis
        ///
        /// - The trait `Convertable` essentially translates to a scalar native type on rust or Array.
        pub fn $fn_name<T, U>(
            arg1: &T,
            arg2: &U,
            batch: bool,
        ) -> Array<
            <<T as Convertable>::OutType as ImplicitPromote<<U as Convertable>::OutType>>::Output,
        >
        where
            T: Convertable,
            U: Convertable,
            <T as Convertable>::OutType: ImplicitPromote<<U as Convertable>::OutType>,
            <U as Convertable>::OutType: ImplicitPromote<<T as Convertable>::OutType>,
        {
            let lhs = arg1.convert(); // Convert to Array<T>
            let rhs = arg2.convert(); // Convert to Array<T>
            match (lhs.is_scalar(), rhs.is_scalar()) {
                (true, false) => {
                    let l = tile(&lhs, rhs.dims());
                    $help_name(&l, &rhs, batch)
                }
                (false, true) => {
                    let r = tile(&rhs, lhs.dims());
                    $help_name(&lhs, &r, batch)
                }
                _ => $help_name(&lhs, &rhs, batch),
            }
        }
    };
}

overloaded_binary_func!("Addition of two Arrays", add, add_helper, af_add);
overloaded_binary_func!("Subtraction of two Arrays", sub, sub_helper, af_sub);
overloaded_binary_func!("Multiplication of two Arrays", mul, mul_helper, af_mul);
overloaded_binary_func!("Division of two Arrays", div, div_helper, af_div);
overloaded_binary_func!("Compute remainder from two Arrays", rem, rem_helper, af_rem);
overloaded_binary_func!("Compute left shift", shiftl, shiftl_helper, af_bitshiftl);
overloaded_binary_func!("Compute right shift", shiftr, shiftr_helper, af_bitshiftr);
overloaded_binary_func!(
    "Compute modulo of two Arrays",
    modulo,
    modulo_helper,
    af_mod
);
overloaded_binary_func!(
    "Calculate atan2 of two Arrays",
    atan2,
    atan2_helper,
    af_atan2
);
overloaded_binary_func!(
    "Create complex array from two Arrays",
    cplx2,
    cplx2_helper,
    af_cplx2
);
overloaded_binary_func!("Compute root", root, root_helper, af_root);
overloaded_binary_func!("Computer power", pow, pow_helper, af_pow);

macro_rules! overloaded_logic_func {
    ($doc_str: expr, $fn_name: ident, $help_name: ident, $ffi_name: ident) => {
        fn $help_name<A, B>(lhs: &Array<A>, rhs: &Array<B>, batch: bool) -> Array<bool>
        where
            A: ImplicitPromote<B>,
            B: ImplicitPromote<A>,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(&mut temp as *mut af_array, lhs.get(), rhs.get(), batch);
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }

        #[doc=$doc_str]
        ///
        /// This is a comparison operation.
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
        /// An Array with results of the comparison operation a.k.a an Array of boolean values.
        ///
        ///# Important Notes
        ///
        /// - If shape/dimensions of `arg1` and `arg2` are same, the value of `batch` parameter
        ///   has no effect.
        ///
        /// - If shape/dimensions of `arg1` and `arg2` are different, the value of `batch` has
        ///   to be set to `true`. In this case, the shapes of `arg1` and `arg2` have to satisfy the
        ///   following criteria:
        ///   - Same number of elements in `arg1` and `arg2` along a given dimension/axis
        ///   - Only one element in `arg1` or `arg2` along a given dimension/axis
        ///
        /// - The trait `Convertable` essentially translates to a scalar native type on rust or Array.
        pub fn $fn_name<T, U>(arg1: &T, arg2: &U, batch: bool) -> Array<bool>
        where
            T: Convertable,
            U: Convertable,
            <T as Convertable>::OutType: ImplicitPromote<<U as Convertable>::OutType>,
            <U as Convertable>::OutType: ImplicitPromote<<T as Convertable>::OutType>,
        {
            let lhs = arg1.convert(); // Convert to Array<T>
            let rhs = arg2.convert(); // Convert to Array<T>
            match (lhs.is_scalar(), rhs.is_scalar()) {
                (true, false) => {
                    let l = tile(&lhs, rhs.dims());
                    $help_name(&l, &rhs, batch)
                }
                (false, true) => {
                    let r = tile(&rhs, lhs.dims());
                    $help_name(&lhs, &r, batch)
                }
                _ => $help_name(&lhs, &rhs, batch),
            }
        }
    };
}

overloaded_logic_func!(
    "Perform `less than` comparison operation",
    lt,
    lt_helper,
    af_lt
);
overloaded_logic_func!(
    "Perform `greater than` comparison operation",
    gt,
    gt_helper,
    af_gt
);
overloaded_logic_func!(
    "Perform `less than equals` comparison operation",
    le,
    le_helper,
    af_le
);
overloaded_logic_func!(
    "Perform `greater than equals` comparison operation",
    ge,
    ge_helper,
    af_ge
);
overloaded_logic_func!(
    "Perform `equals` comparison operation",
    eq,
    eq_helper,
    af_eq
);
overloaded_logic_func!(
    "Elementwise `not equals` comparison of two Arrays",
    neq,
    neq_helper,
    af_neq
);
overloaded_logic_func!(
    "Elementwise logical AND operation of two Arrays",
    and,
    and_helper,
    af_and
);
overloaded_logic_func!(
    "Elementwise logical OR operation of two Arrays",
    or,
    or_helper,
    af_or
);

fn clamp_helper<X, Y>(
    inp: &Array<X>,
    lo: &Array<Y>,
    hi: &Array<Y>,
    batch: bool,
) -> Array<<X as ImplicitPromote<Y>>::Output>
where
    X: ImplicitPromote<Y>,
    Y: ImplicitPromote<X>,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_clamp(
            &mut temp as *mut af_array,
            inp.get(),
            lo.get(),
            hi.get(),
            batch,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

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
/// # Important Notes
///
/// - If shape/dimensions of `arg1` and `arg2` are same, the value of `batch` parameter
///   has no effect.
///
/// - If shape/dimensions of `arg1` and `arg2` are different, the value of `batch` has
///   to be set to `true`. In this case, the shapes of `arg1` and `arg2` have to satisfy the
///   following criteria:
///   - Same number of elements in `arg1` and `arg2` along a given dimension/axis
///   - Only one element in `arg1` or `arg2` along a given dimension/axis
///
/// - The trait `Convertable` essentially translates to a scalar native type on rust or Array.
pub fn clamp<T, C>(
    input: &Array<T>,
    arg1: &C,
    arg2: &C,
    batch: bool,
) -> Array<<T as ImplicitPromote<<C as Convertable>::OutType>>::Output>
where
    T: ImplicitPromote<<C as Convertable>::OutType>,
    C: Convertable,
    <C as Convertable>::OutType: ImplicitPromote<T>,
{
    let lo = arg1.convert(); // Convert to Array<T>
    let hi = arg2.convert(); // Convert to Array<T>
    match (lo.is_scalar(), hi.is_scalar()) {
        (true, false) => {
            let l = tile(&lo, hi.dims());
            clamp_helper(input, &l, &hi, batch)
        }
        (false, true) => {
            let r = tile(&hi, lo.dims());
            clamp_helper(input, &lo, &r, batch)
        }
        (true, true) => {
            let l = tile(&lo, input.dims());
            let r = tile(&hi, input.dims());
            clamp_helper(input, &l, &r, batch)
        }
        _ => clamp_helper(input, &lo, &hi, batch),
    }
}

macro_rules! arith_rhs_scalar_func {
    ($op_name:ident, $fn_name: ident) => {
        // Implement (&Array<T> op_name rust_type)
        impl<'f, T, U> $op_name<U> for &'f Array<T>
        where
            T: ImplicitPromote<U>,
            U: ImplicitPromote<T> + Clone + ConstGenerator<OutType = U>,
        {
            type Output = Array<<T as ImplicitPromote<U>>::Output>;

            fn $fn_name(self, rhs: U) -> Self::Output {
                let temp = rhs.clone();
                $fn_name(self, &temp, false)
            }
        }

        // Implement (Array<T> op_name rust_type)
        impl<T, U> $op_name<U> for Array<T>
        where
            T: ImplicitPromote<U>,
            U: ImplicitPromote<T> + Clone + ConstGenerator<OutType = U>,
        {
            type Output = Array<<T as ImplicitPromote<U>>::Output>;

            fn $fn_name(self, rhs: U) -> Self::Output {
                let temp = rhs.clone();
                $fn_name(&self, &temp, false)
            }
        }
    };
}

macro_rules! arith_lhs_scalar_func {
    ($rust_type: ty, $op_name: ident, $fn_name: ident) => {
        // Implement (rust_type op_name &Array<T>)
        impl<'f, T> $op_name<&'f Array<T>> for $rust_type
        where
            T: ImplicitPromote<$rust_type>,
            $rust_type: ImplicitPromote<T>,
        {
            type Output = Array<<$rust_type as ImplicitPromote<T>>::Output>;

            fn $fn_name(self, rhs: &'f Array<T>) -> Self::Output {
                $fn_name(&self, rhs, false)
            }
        }

        // Implement (rust_type op_name Array<T>)
        impl<T> $op_name<Array<T>> for $rust_type
        where
            T: ImplicitPromote<$rust_type>,
            $rust_type: ImplicitPromote<T>,
        {
            type Output = Array<<$rust_type as ImplicitPromote<T>>::Output>;

            fn $fn_name(self, rhs: Array<T>) -> Self::Output {
                $fn_name(&self, &rhs, false)
            }
        }
    };
}

arith_rhs_scalar_func!(Add, add);
arith_rhs_scalar_func!(Sub, sub);
arith_rhs_scalar_func!(Mul, mul);
arith_rhs_scalar_func!(Div, div);

macro_rules! arith_scalar_spec {
    ($ty_name:ty) => {
        arith_lhs_scalar_func!($ty_name, Add, add);
        arith_lhs_scalar_func!($ty_name, Sub, sub);
        arith_lhs_scalar_func!($ty_name, Mul, mul);
        arith_lhs_scalar_func!($ty_name, Div, div);
    };
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
    ($op_name:ident, $fn_name:ident, $delegate:ident) => {
        impl<A, B> $op_name<Array<B>> for Array<A>
        where
            A: ImplicitPromote<B>,
            B: ImplicitPromote<A>,
        {
            type Output = Array<<A as ImplicitPromote<B>>::Output>;

            fn $fn_name(self, rhs: Array<B>) -> Self::Output {
                $delegate(&self, &rhs, false)
            }
        }

        impl<'a, A, B> $op_name<&'a Array<B>> for Array<A>
        where
            A: ImplicitPromote<B>,
            B: ImplicitPromote<A>,
        {
            type Output = Array<<A as ImplicitPromote<B>>::Output>;

            fn $fn_name(self, rhs: &'a Array<B>) -> Self::Output {
                $delegate(&self, rhs, false)
            }
        }

        impl<'a, A, B> $op_name<Array<B>> for &'a Array<A>
        where
            A: ImplicitPromote<B>,
            B: ImplicitPromote<A>,
        {
            type Output = Array<<A as ImplicitPromote<B>>::Output>;

            fn $fn_name(self, rhs: Array<B>) -> Self::Output {
                $delegate(self, &rhs, false)
            }
        }

        impl<'a, 'b, A, B> $op_name<&'a Array<B>> for &'b Array<A>
        where
            A: ImplicitPromote<B>,
            B: ImplicitPromote<A>,
        {
            type Output = Array<<A as ImplicitPromote<B>>::Output>;

            fn $fn_name(self, rhs: &'a Array<B>) -> Self::Output {
                $delegate(self, rhs, false)
            }
        }
    };
}

arith_func!(Add, add, add);
arith_func!(Sub, sub, sub);
arith_func!(Mul, mul, mul);
arith_func!(Div, div, div);
arith_func!(Rem, rem, rem);
arith_func!(Shl, shl, shiftl);
arith_func!(Shr, shr, shiftr);
arith_func!(BitAnd, bitand, bitand);
arith_func!(BitOr, bitor, bitor);
arith_func!(BitXor, bitxor, bitxor);

macro_rules! bitshift_scalar_func {
    ($rust_type: ty, $trait_name: ident, $op_name: ident) => {
        impl<T> $trait_name<$rust_type> for Array<T>
        where
            T: ImplicitPromote<$rust_type>,
            $rust_type: ImplicitPromote<T>,
        {
            type Output = Array<<T as ImplicitPromote<$rust_type>>::Output>;

            fn $op_name(self, rhs: $rust_type) -> Self::Output {
                let op2 = constant(rhs, self.dims());
                self.$op_name(op2)
            }
        }
        impl<'f, T> $trait_name<$rust_type> for &'f Array<T>
        where
            T: ImplicitPromote<$rust_type>,
            $rust_type: ImplicitPromote<T>,
        {
            type Output = Array<<T as ImplicitPromote<$rust_type>>::Output>;

            fn $op_name(self, rhs: $rust_type) -> Self::Output {
                let op2 = constant(rhs, self.dims());
                self.$op_name(op2)
            }
        }
    };
}

macro_rules! shift_spec {
    ($trait_name: ident, $op_name: ident) => {
        bitshift_scalar_func!(u64, $trait_name, $op_name);
        bitshift_scalar_func!(u32, $trait_name, $op_name);
        bitshift_scalar_func!(u16, $trait_name, $op_name);
        bitshift_scalar_func!(u8, $trait_name, $op_name);
    };
}

shift_spec!(Shl, shl);
shift_spec!(Shr, shr);

#[cfg(op_assign)]
mod op_assign {

    use super::*;
    use crate::core::{assign_gen, Array, Indexer, Seq};
    use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};

    macro_rules! arith_assign_func {
        ($op_name:ident, $fn_name:ident, $func: ident) => {
            impl<A, B> $op_name<Array<B>> for Array<A>
            where
                A: ImplicitPromote<B>,
                B: ImplicitPromote<A>,
            {
                fn $fn_name(&mut self, rhs: Array<B>) {
                    let tmp_seq = Seq::<f32>::default();
                    let mut idxrs = Indexer::default();
                    for n in 0..self.numdims() {
                        idxrs.set_index(&tmp_seq, n, Some(false));
                    }
                    let opres = $func(self as &Array<A>, &rhs, false).cast::<A>();
                    assign_gen(self, &idxrs, &opres);
                }
            }
        };
    }

    arith_assign_func!(AddAssign, add_assign, add);
    arith_assign_func!(SubAssign, sub_assign, sub);
    arith_assign_func!(MulAssign, mul_assign, mul);
    arith_assign_func!(DivAssign, div_assign, div);
    arith_assign_func!(RemAssign, rem_assign, rem);
    arith_assign_func!(ShlAssign, shl_assign, shiftl);
    arith_assign_func!(ShrAssign, shr_assign, shiftr);

    macro_rules! shift_assign_func {
        ($rust_type:ty, $trait_name:ident, $op_name:ident, $func:ident) => {
            impl<T> $trait_name<$rust_type> for Array<T>
            where
                $rust_type: ImplicitPromote<T>,
                T: ImplicitPromote<$rust_type, Output = T>,
            {
                fn $op_name(&mut self, rhs: $rust_type) {
                    let mut temp = $func(self, &rhs, false);
                    mem::swap(self, &mut temp);
                }
            }
        };
    }

    macro_rules! shift_assign_spec {
        ($trait_name: ident, $op_name: ident, $func:ident) => {
            shift_assign_func!(u64, $trait_name, $op_name, $func);
            shift_assign_func!(u32, $trait_name, $op_name, $func);
            shift_assign_func!(u16, $trait_name, $op_name, $func);
            shift_assign_func!(u8, $trait_name, $op_name, $func);
        };
    }

    shift_assign_spec!(ShlAssign, shl_assign, shiftl);
    shift_assign_spec!(ShrAssign, shr_assign, shiftr);

    macro_rules! bit_assign_func {
        ($op_name:ident, $fn_name:ident, $func: ident) => {
            impl<A, B> $op_name<Array<B>> for Array<A>
            where
                A: ImplicitPromote<B>,
                B: ImplicitPromote<A>,
            {
                fn $fn_name(&mut self, rhs: Array<B>) {
                    let tmp_seq = Seq::<f32>::default();
                    let mut idxrs = Indexer::default();
                    for n in 0..self.numdims() {
                        idxrs.set_index(&tmp_seq, n, Some(false));
                    }
                    let opres = $func(self as &Array<A>, &rhs, false).cast::<A>();
                    assign_gen(self, &idxrs, &opres);
                }
            }
        };
    }

    bit_assign_func!(BitAndAssign, bitand_assign, bitand);
    bit_assign_func!(BitOrAssign, bitor_assign, bitor);
    bit_assign_func!(BitXorAssign, bitxor_assign, bitxor);
}

///Implement negation trait for Array
impl<T> Neg for Array<T>
where
    T: Zero + ConstGenerator<OutType = T>,
{
    type Output = Array<T>;

    fn neg(self) -> Self::Output {
        let cnst = constant(T::zero(), self.dims());
        sub(&cnst, &self, true)
    }
}

/// Perform bitwise complement on all values of Array
pub fn bitnot<T: HasAfEnum>(input: &Array<T>) -> Array<T>
where
    T: HasAfEnum + IntegralType,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_bitnot(&mut temp as *mut af_array, input.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}
