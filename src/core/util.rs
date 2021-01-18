use super::defines::{
    AfError, BinaryOp, ColorMap, ConvDomain, ConvMode, DType, InterpType, MatProp, MatchType,
    RandomEngineType, SparseFormat,
};
use super::error::HANDLE_ERROR;

use half::f16;
use libc::{c_int, c_uint, c_void, size_t};
use num::Complex;
use std::convert::From;
use std::mem;
use std::ops::BitOr;

/// Short type alias for Complex single precision type
pub type c32 = Complex<f32>;
/// Short type alias for Complex double precision type
pub type c64 = Complex<f64>;
/// ArrayFire FFI Type alias for libc's signed long long
pub type dim_t = libc::c_longlong;
/// ArrayFire FFI Type alias for libc's unsigned long long
pub type u64_t = libc::c_ulonglong;
/// ArrayFire FFI Type alias for libc's void*
pub type void_ptr = *mut libc::c_void;

/// ArrayFire FFI Type alias for af_array
pub type af_array = *mut libc::c_void;
/// ArrayFire FFI Type alias for af_event
pub type af_event = *mut libc::c_void;
/// ArrayFire FFI Type alias for af_indexers_t
pub type af_index_t = *mut libc::c_void;
/// ArrayFire FFI Type alias for af_features
pub type af_features = *const libc::c_void;
/// ArrayFire FFI Type alias for af_random_engine
pub type af_random_engine = *mut libc::c_void;
/// ArrayFire FFI Type alias for af_window
pub type af_window = *mut libc::c_void;

extern "C" {
    fn af_get_size_of(size: *mut size_t, aftype: c_uint) -> c_int;
    fn af_alloc_host(ptr: *mut *const c_void, bytes: dim_t) -> c_int;
    fn af_free_host(ptr: *mut c_void) -> c_int;
}

/// Get size, in bytes, of the arrayfire native type
pub fn get_size(value: DType) -> usize {
    unsafe {
        let mut ret_val: usize = 0;
        let err_val = af_get_size_of(&mut ret_val as *mut size_t, value as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        ret_val
    }
}

/// Allocates space using Arrayfire allocator in host memory
pub fn alloc_host<T>(elements: usize, _type: DType) -> *const T {
    let ptr: *const T = ::std::ptr::null();
    let bytes = (elements * get_size(_type)) as dim_t;
    unsafe {
        let err_val = af_alloc_host(&mut (ptr as *const c_void), bytes);
        HANDLE_ERROR(AfError::from(err_val));
    }
    ptr
}

/// Frees memory allocated by Arrayfire allocator in host memory
pub fn free_host<T>(ptr: *mut T) {
    unsafe {
        let err_val = af_free_host(ptr as *mut c_void);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

impl From<i32> for AfError {
    fn from(t: i32) -> Self {
        assert!(AfError::SUCCESS as i32 <= t && t <= AfError::ERR_UNKNOWN as i32);
        unsafe { mem::transmute(t) }
    }
}

impl From<u32> for DType {
    fn from(t: u32) -> Self {
        assert!(DType::F32 as u32 <= t && t <= DType::U64 as u32);
        unsafe { mem::transmute(t) }
    }
}

impl From<u32> for InterpType {
    fn from(t: u32) -> Self {
        assert!(InterpType::NEAREST as u32 <= t && t <= InterpType::BICUBIC_SPLINE as u32);
        unsafe { mem::transmute(t) }
    }
}

impl From<u32> for ConvMode {
    fn from(t: u32) -> Self {
        assert!(ConvMode::DEFAULT as u32 <= t && t <= ConvMode::EXPAND as u32);
        unsafe { mem::transmute(t) }
    }
}

impl From<u32> for ConvDomain {
    fn from(t: u32) -> Self {
        assert!(ConvDomain::AUTO as u32 <= t && t <= ConvDomain::FREQUENCY as u32);
        unsafe { mem::transmute(t) }
    }
}

impl From<u32> for MatchType {
    fn from(t: u32) -> Self {
        assert!(MatchType::SAD as u32 <= t && t <= MatchType::SHD as u32);
        unsafe { mem::transmute(t) }
    }
}

impl From<u32> for ColorMap {
    fn from(t: u32) -> Self {
        assert!(ColorMap::DEFAULT as u32 <= t && t <= ColorMap::BLUE as u32);
        unsafe { mem::transmute(t) }
    }
}

mod private {
    pub trait Sealed: std::fmt::Debug + std::marker::Copy + std::default::Default {}

    impl Sealed for num::complex::Complex<f32> {}
    impl Sealed for num::complex::Complex<f64> {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
    impl Sealed for bool {}
    impl Sealed for u8 {}
    impl Sealed for i16 {}
    impl Sealed for u16 {}
    impl Sealed for half::f16 {}
    impl Sealed for i32 {}
    impl Sealed for u32 {}
    impl Sealed for i64 {}
    impl Sealed for u64 {}
    impl Sealed for usize {}
}

/// Types of the data that can be generated using ArrayFire data generation functions.
///
/// The trait HasAfEnum has been defined internally for the following types. We strongly suggest
/// not to implement this trait in your program for user defined types because ArrayFire functions
/// will only work for the following data types currently. Any such trait implementation for types
/// other than the ones listed below will result in undefined behavior.
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
pub trait HasAfEnum: private::Sealed {
    /// This type alias points to `Self` always.
    type InType: HasAfEnum;
    /// This type alias points to the data type used to hold real part of a
    /// complex number. For real valued numbers, this points to `Self`.
    type BaseType: HasAfEnum;
    /// This type alias points to `f32` for all 32 bit size types and `f64` for
    /// larger 64-bit size types.
    type AbsOutType: HasAfEnum;
    /// This type alias points to `f64`/`f32` for floating point types and
    /// `Self` otherwise.
    type ArgOutType: HasAfEnum;
    /// This type alias is used to define the output Array type for unary
    /// operations. It points to `Self` for floating point types, either
    /// real or complex. It points to `f32` for rest of the input types.
    type UnaryOutType: HasAfEnum;
    /// This type alias points to complex type created from a given input type.
    /// This alias always points to either `std::Complex<f32>` or `std::Complex<f64>`
    type ComplexOutType;
    /// This type alias points to a data type that can store the mean value for
    /// a given input type. This alias points to `f32`/`Complex<f32>` for all 32
    /// bit size types and `f64`/`Complex<f64>` for larger 64-bit size types.
    type MeanOutType: HasAfEnum;
    /// This type alias points to a data type that can store the result of
    /// aggregation of set of values for a given input type. Aggregate type
    /// alias points to below types for given input types:
    /// - `Self` for input types: `Complex<64>`, `Complex<f32>`, `f64`, `f32`, `i64`, `u64`
    /// - `u32` for input types: `bool`
    /// - `u32` for input types: `u8`
    /// - `i32` for input types: `i16`
    /// - `u32` for input types: `u16`
    /// - `i32` for input types: `i32`
    /// - `u32` for input types: `u32`
    type AggregateOutType: HasAfEnum;
    /// This type is different for b8 input type
    type ProductOutType: HasAfEnum;
    /// This type alias points to the output type for given input type of
    /// sobel filter operation. Sobel filter output alias points to below
    /// types for given input types:
    /// - `f32` for input types: `Complex<f32>`, `f32`
    /// - `f64` for input types: `Complex<f64>`, `f64`
    /// - `i32` for input types: `bool`, `u8`, `i16`, `u16`, `i32`, `u32`
    /// - `i64` for input types: `i64`, `u64`
    type SobelOutType: HasAfEnum;

    /// Return trait implmentors corresponding [DType](./enum.DType.html)
    fn get_af_dtype() -> DType;
}

impl HasAfEnum for Complex<f32> {
    type InType = Self;
    type BaseType = f32;
    type AbsOutType = f32;
    type ArgOutType = f32;
    type UnaryOutType = Self;
    type ComplexOutType = Self;
    type MeanOutType = Self;
    type AggregateOutType = Self;
    type ProductOutType = Self;
    type SobelOutType = Self;

    fn get_af_dtype() -> DType {
        DType::C32
    }
}
impl HasAfEnum for Complex<f64> {
    type InType = Self;
    type BaseType = f64;
    type AbsOutType = f64;
    type ArgOutType = f64;
    type UnaryOutType = Self;
    type ComplexOutType = Self;
    type MeanOutType = Self;
    type AggregateOutType = Self;
    type ProductOutType = Self;
    type SobelOutType = Self;

    fn get_af_dtype() -> DType {
        DType::C64
    }
}
impl HasAfEnum for f32 {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = f32;
    type ArgOutType = f32;
    type UnaryOutType = Self;
    type ComplexOutType = Complex<f32>;
    type MeanOutType = Self;
    type AggregateOutType = Self;
    type ProductOutType = Self;
    type SobelOutType = Self;

    fn get_af_dtype() -> DType {
        DType::F32
    }
}
impl HasAfEnum for f64 {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = f64;
    type ArgOutType = f64;
    type UnaryOutType = Self;
    type ComplexOutType = Complex<f64>;
    type MeanOutType = Self;
    type AggregateOutType = Self;
    type ProductOutType = Self;
    type SobelOutType = Self;

    fn get_af_dtype() -> DType {
        DType::F64
    }
}
impl HasAfEnum for bool {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = f32;
    type ArgOutType = bool;
    type UnaryOutType = f32;
    type ComplexOutType = Complex<f32>;
    type MeanOutType = f32;
    type AggregateOutType = u32;
    type ProductOutType = bool;
    type SobelOutType = i32;

    fn get_af_dtype() -> DType {
        DType::B8
    }
}
impl HasAfEnum for u8 {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = f32;
    type ArgOutType = u8;
    type UnaryOutType = f32;
    type ComplexOutType = Complex<f32>;
    type MeanOutType = f32;
    type AggregateOutType = u32;
    type ProductOutType = u32;
    type SobelOutType = i32;

    fn get_af_dtype() -> DType {
        DType::U8
    }
}
impl HasAfEnum for i16 {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = f32;
    type ArgOutType = i16;
    type UnaryOutType = f32;
    type ComplexOutType = Complex<f32>;
    type MeanOutType = f32;
    type AggregateOutType = i32;
    type ProductOutType = i32;
    type SobelOutType = i32;

    fn get_af_dtype() -> DType {
        DType::S16
    }
}
impl HasAfEnum for u16 {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = f32;
    type ArgOutType = u16;
    type UnaryOutType = f32;
    type ComplexOutType = Complex<f32>;
    type MeanOutType = f32;
    type AggregateOutType = u32;
    type ProductOutType = u32;
    type SobelOutType = i32;

    fn get_af_dtype() -> DType {
        DType::U16
    }
}
impl HasAfEnum for f16 {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = Self;
    type ArgOutType = Self;
    type UnaryOutType = Self;
    type ComplexOutType = Complex<f16>;
    type MeanOutType = Self;
    type AggregateOutType = f32;
    type ProductOutType = f32;
    type SobelOutType = Self;

    fn get_af_dtype() -> DType {
        DType::F16
    }
}
impl HasAfEnum for i32 {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = f32;
    type ArgOutType = i32;
    type UnaryOutType = f32;
    type ComplexOutType = Complex<f32>;
    type MeanOutType = f32;
    type AggregateOutType = i32;
    type ProductOutType = i32;
    type SobelOutType = i32;

    fn get_af_dtype() -> DType {
        DType::S32
    }
}
impl HasAfEnum for u32 {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = f32;
    type ArgOutType = u32;
    type UnaryOutType = f32;
    type ComplexOutType = Complex<f32>;
    type MeanOutType = f32;
    type AggregateOutType = u32;
    type ProductOutType = u32;
    type SobelOutType = i32;

    fn get_af_dtype() -> DType {
        DType::U32
    }
}
impl HasAfEnum for i64 {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = f64;
    type ArgOutType = i64;
    type UnaryOutType = f64;
    type ComplexOutType = Complex<f64>;
    type MeanOutType = f64;
    type AggregateOutType = Self;
    type ProductOutType = Self;
    type SobelOutType = i64;

    fn get_af_dtype() -> DType {
        DType::S64
    }
}
impl HasAfEnum for u64 {
    type InType = Self;
    type BaseType = Self;
    type AbsOutType = f64;
    type ArgOutType = u64;
    type UnaryOutType = f64;
    type ComplexOutType = Complex<f64>;
    type MeanOutType = f64;
    type AggregateOutType = Self;
    type ProductOutType = Self;
    type SobelOutType = i64;

    fn get_af_dtype() -> DType {
        DType::U64
    }
}

impl From<u32> for SparseFormat {
    fn from(t: u32) -> Self {
        assert!(SparseFormat::DENSE as u32 <= t && t <= SparseFormat::COO as u32);
        unsafe { mem::transmute(t) }
    }
}

impl From<u32> for BinaryOp {
    fn from(t: u32) -> Self {
        assert!(BinaryOp::ADD as u32 <= t && t <= BinaryOp::MAX as u32);
        unsafe { mem::transmute(t) }
    }
}

impl From<u32> for RandomEngineType {
    fn from(t: u32) -> Self {
        assert!(
            RandomEngineType::PHILOX_4X32_10 as u32 <= t
                && t <= RandomEngineType::MERSENNE_GP11213 as u32
        );
        unsafe { mem::transmute(t) }
    }
}

/// This is an internal trait defined and implemented by ArrayFire
/// create for rust's built-in types to figure out the data type
/// binary operation's results.
pub trait ImplicitPromote<RHS>: HasAfEnum {
    /// This type alias points to the type of the result obtained
    /// by performing a given binary option on given type and `RHS`.
    type Output: HasAfEnum;
}

impl<T> ImplicitPromote<T> for T
where
    T: HasAfEnum,
{
    type Output = T;
}

macro_rules! implicit {
    [$implType: ident, $rhsType: ident => $outType: ident] => (
        impl ImplicitPromote< $rhsType > for $implType {
            type Output = $outType;
        }
    )
}

//
//implicit(implementation type, RHS type, output type)
//

//LHS is Complex double
implicit!(c64, c32 => c64);
implicit!(c64, f64       => c64);
implicit!(c64, f32       => c64);
implicit!(c64, i64       => c64);
implicit!(c64, u64       => c64);
implicit!(c64, i32       => c64);
implicit!(c64, u32       => c64);
implicit!(c64, i16       => c64);
implicit!(c64, u16       => c64);
implicit!(c64, bool      => c64);
implicit!(c64, u8        => c64);

//LHS is Complex float
implicit!(c32, c64 => c64);
implicit!(c32, f64       => c64);
implicit!(c32, f32       => c32);
implicit!(c32, i64       => c32);
implicit!(c32, u64       => c32);
implicit!(c32, i32       => c32);
implicit!(c32, u32       => c32);
implicit!(c32, i16       => c32);
implicit!(c32, u16       => c32);
implicit!(c32, bool      => c32);
implicit!(c32, u8        => c32);

//LHS is 64-bit floating point
implicit!(f64, c64 => c64);
implicit!(f64, c32 => c64);
implicit!(f64, f32       =>       f64);
implicit!(f64, i64       =>       f64);
implicit!(f64, u64       =>       f64);
implicit!(f64, i32       =>       f64);
implicit!(f64, u32       =>       f64);
implicit!(f64, i16       =>       f64);
implicit!(f64, u16       =>       f64);
implicit!(f64, bool      =>       f64);
implicit!(f64, u8        =>       f64);

//LHS is 32-bit floating point
implicit!(f32, c64 => c64);
implicit!(f32, c32 => c32);
implicit!(f32, f64       =>       f64);
implicit!(f32, i64       =>       f32);
implicit!(f32, u64       =>       f32);
implicit!(f32, i32       =>       f32);
implicit!(f32, u32       =>       f32);
implicit!(f32, i16       =>       f32);
implicit!(f32, u16       =>       f32);
implicit!(f32, bool      =>       f32);
implicit!(f32, u8        =>       f32);

//LHS is 64-bit signed integer
implicit!(i64, c64 => c64);
implicit!(i64, c32 => c32);
implicit!(i64, f64       =>       f64);
implicit!(i64, f32       =>       f32);
implicit!(i64, u64       =>       u64);
implicit!(i64, i32       =>       i64);
implicit!(i64, u32       =>       i64);
implicit!(i64, i16       =>       i64);
implicit!(i64, u16       =>       i64);
implicit!(i64, bool      =>       i64);
implicit!(i64, u8        =>       i64);

//LHS is 64-bit unsigned integer
implicit!(u64, c64 => c64);
implicit!(u64, c32 => c32);
implicit!(u64, f64       =>       f64);
implicit!(u64, f32       =>       f32);
implicit!(u64, i64       =>       u64);
implicit!(u64, i32       =>       u64);
implicit!(u64, u32       =>       u64);
implicit!(u64, i16       =>       u64);
implicit!(u64, u16       =>       u64);
implicit!(u64, bool      =>       u64);
implicit!(u64, u8        =>       u64);

//LHS is 32-bit signed integer
implicit!(i32, c64 => c64);
implicit!(i32, c32 => c32);
implicit!(i32, f64       =>       f64);
implicit!(i32, f32       =>       f32);
implicit!(i32, i64       =>       i64);
implicit!(i32, u64       =>       u64);
implicit!(i32, u32       =>       u32);
implicit!(i32, i16       =>       i32);
implicit!(i32, u16       =>       i32);
implicit!(i32, bool      =>       i32);
implicit!(i32, u8        =>       i32);

//LHS is 32-bit unsigned integer
implicit!(u32, c64 => c64);
implicit!(u32, c32 => c32);
implicit!(u32, f64       =>       f64);
implicit!(u32, f32       =>       f32);
implicit!(u32, i64       =>       i64);
implicit!(u32, u64       =>       u64);
implicit!(u32, i32       =>       u32);
implicit!(u32, i16       =>       u32);
implicit!(u32, u16       =>       u32);
implicit!(u32, bool      =>       u32);
implicit!(u32, u8        =>       u32);

//LHS is 16-bit signed integer
implicit!(i16, c64 => c64);
implicit!(i16, c32 => c32);
implicit!(i16, f64       =>       f64);
implicit!(i16, f32       =>       f32);
implicit!(i16, i64       =>       i64);
implicit!(i16, u64       =>       u64);
implicit!(i16, i32       =>       i32);
implicit!(i16, u32       =>       u32);
implicit!(i16, u16       =>       u16);
implicit!(i16, bool      =>       u16);
implicit!(i16, u8        =>       u16);

//LHS is 16-bit unsigned integer
implicit!(u16, c64 => c64);
implicit!(u16, c32 => c32);
implicit!(u16, f64       =>       f64);
implicit!(u16, f32       =>       f32);
implicit!(u16, i64       =>       i64);
implicit!(u16, u64       =>       u64);
implicit!(u16, i32       =>       i32);
implicit!(u16, u32       =>       u32);
implicit!(u16, i16       =>       u16);
implicit!(u16, bool      =>       u16);
implicit!(u16, u8        =>       u16);

//LHS is 8-bit unsigned integer
implicit!(u8, c64 => c64);
implicit!(u8, c32 => c32);
implicit!(u8, f64       =>       f64);
implicit!(u8, f32       =>       f32);
implicit!(u8, i64       =>       i64);
implicit!(u8, u64       =>       u64);
implicit!(u8, i32       =>       i32);
implicit!(u8, u32       =>       u32);
implicit!(u8, i16       =>       i16);
implicit!(u8, u16       =>       u16);
implicit!(u8, bool      =>        u8);

//LHS is bool(af::s8)
implicit!(bool, c64 => c64);
implicit!(bool, c32 => c32);
implicit!(bool, f64       =>       f64);
implicit!(bool, f32       =>       f32);
implicit!(bool, i64       =>       i64);
implicit!(bool, u64       =>       u64);
implicit!(bool, i32       =>       i32);
implicit!(bool, u32       =>       u32);
implicit!(bool, i16       =>       i16);
implicit!(bool, u16       =>       u16);
implicit!(bool, u8        =>        u8);

///Trait qualifier to accept either real or complex typed data
pub trait FloatingPoint: HasAfEnum {
    /// Use to check if trait implementor is real number
    fn is_real() -> bool {
        false
    }
    /// Use to check if trait implementor is complex number
    fn is_complex() -> bool {
        false
    }
}

impl FloatingPoint for Complex<f64> {
    fn is_complex() -> bool {
        true
    }
}
impl FloatingPoint for Complex<f32> {
    fn is_complex() -> bool {
        true
    }
}
impl FloatingPoint for f64 {
    fn is_real() -> bool {
        true
    }
}
impl FloatingPoint for f32 {
    fn is_real() -> bool {
        true
    }
}

///Trait qualifier to accept real data(numbers)
pub trait RealFloating: HasAfEnum {}

impl RealFloating for f64 {}
impl RealFloating for f32 {}

///Trait qualifier to accept complex data(numbers)
pub trait ComplexFloating: HasAfEnum {}

impl ComplexFloating for c64 {}
impl ComplexFloating for c32 {}

///Trait qualifier indicating it can hold real numbers only
pub trait RealNumber: HasAfEnum {}

impl RealNumber for f64 {}
impl RealNumber for f32 {}
impl RealNumber for i32 {}
impl RealNumber for u32 {}
impl RealNumber for i16 {}
impl RealNumber for u16 {}
impl RealNumber for u8 {}
impl RealNumber for bool {}
impl RealNumber for u64 {}
impl RealNumber for i64 {}

///Trait qualifier for the type of Arrays accepted by scan operations
pub trait Scanable: HasAfEnum {}

impl Scanable for i32 {}
impl Scanable for u32 {}
impl Scanable for u64 {}
impl Scanable for i64 {}

/// Trait qualifier for type of Array's that are accepted
/// by native image load/save functions.
pub trait ImageNativeType: HasAfEnum {}

impl ImageNativeType for f32 {}
impl ImageNativeType for u16 {}
impl ImageNativeType for u8 {}

/// Trait qualifier for type of Array's that are accepted
/// by image processing functions especially filtering algorithms
pub trait ImageFilterType: HasAfEnum {}

impl ImageFilterType for f64 {}
impl ImageFilterType for f32 {}
impl ImageFilterType for i32 {}
impl ImageFilterType for u32 {}
impl ImageFilterType for i16 {}
impl ImageFilterType for u16 {}
impl ImageFilterType for u8 {}
impl ImageFilterType for bool {}

// TODO Rust haven't stabilized trait aliases yet
/// Trait qualifier for given type indicating conversion capability between
/// grayscale and RGB triplets of data
pub trait GrayRGBConvertible: HasAfEnum {}

impl GrayRGBConvertible for f64 {}
impl GrayRGBConvertible for f32 {}
impl GrayRGBConvertible for i32 {}
impl GrayRGBConvertible for u32 {}
impl GrayRGBConvertible for i16 {}
impl GrayRGBConvertible for u16 {}
impl GrayRGBConvertible for u8 {}

// TODO Rust haven't stabilized trait aliases yet
/// Trait qualifier for given type indicating computability of Moments
pub trait MomentsComputable: HasAfEnum {}

impl MomentsComputable for f64 {}
impl MomentsComputable for f32 {}
impl MomentsComputable for i32 {}
impl MomentsComputable for u32 {}
impl MomentsComputable for i16 {}
impl MomentsComputable for u16 {}
impl MomentsComputable for u8 {}

// TODO Rust haven't stabilized trait aliases yet
/// Trait qualifier for given type indicating computability of Median
pub trait MedianComputable: HasAfEnum {}

impl MedianComputable for f64 {}
impl MedianComputable for f32 {}
impl MedianComputable for i32 {}
impl MedianComputable for u32 {}
impl MedianComputable for i16 {}
impl MedianComputable for u16 {}
impl MedianComputable for u8 {}

// TODO Rust haven't stabilized trait aliases yet
/// Trait qualifier for given type indicating if edge calculations such as
/// derivates etc. can be performed
pub trait EdgeComputable: HasAfEnum {}

impl EdgeComputable for f64 {}
impl EdgeComputable for f32 {}
impl EdgeComputable for i32 {}
impl EdgeComputable for u32 {}
impl EdgeComputable for i16 {}
impl EdgeComputable for u16 {}
impl EdgeComputable for u8 {}

/// Trait qualifier for given type indicating computability of covariance
pub trait CovarianceComputable: HasAfEnum {}

impl CovarianceComputable for f64 {}
impl CovarianceComputable for f32 {}
impl CovarianceComputable for i32 {}
impl CovarianceComputable for u32 {}
impl CovarianceComputable for i16 {}
impl CovarianceComputable for u16 {}
impl CovarianceComputable for u8 {}
impl CovarianceComputable for u64 {}
impl CovarianceComputable for i64 {}

/// Trait qualifier for confidence connected components input
pub trait ConfidenceCCInput: HasAfEnum {}

impl ConfidenceCCInput for f32 {}
impl ConfidenceCCInput for u32 {}
impl ConfidenceCCInput for u16 {}
impl ConfidenceCCInput for u8 {}

/// Trait qualifier for confidence connected components input
pub trait DeconvInput: HasAfEnum {}

impl DeconvInput for f32 {}
impl DeconvInput for i16 {}
impl DeconvInput for u16 {}
impl DeconvInput for u8 {}

/// Trait qualifier for Reduction Key type
pub trait ReduceByKeyInput: HasAfEnum {}

impl ReduceByKeyInput for i32 {}
impl ReduceByKeyInput for u32 {}

impl From<u32> for MatProp {
    fn from(t: u32) -> Self {
        unsafe { mem::transmute(t) }
    }
}

impl BitOr for MatProp {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self::from(self as u32 | rhs as u32)
    }
}

/// Trait to convert reduction's scalar output to appropriate output type
///
/// This is an internal trait and ideally of no use to user usecases.
pub trait Fromf64: private::Sealed {
    /// Convert to target type from a double precision value
    fn fromf64(value: f64) -> Self;
}

#[rustfmt::skip]
impl Fromf64 for usize{ fn fromf64(value: f64) -> Self { value as Self }}
#[rustfmt::skip]
impl Fromf64 for f64  { fn fromf64(value: f64) -> Self { value as Self }}
#[rustfmt::skip]
impl Fromf64 for u64  { fn fromf64(value: f64) -> Self { value as Self }}
#[rustfmt::skip]
impl Fromf64 for i64  { fn fromf64(value: f64) -> Self { value as Self }}
#[rustfmt::skip]
impl Fromf64 for f32  { fn fromf64(value: f64) -> Self { value as Self }}
#[rustfmt::skip]
impl Fromf64 for u32  { fn fromf64(value: f64) -> Self { value as Self }}
#[rustfmt::skip]
impl Fromf64 for i32  { fn fromf64(value: f64) -> Self { value as Self }}
#[rustfmt::skip]
impl Fromf64 for u16  { fn fromf64(value: f64) -> Self { value as Self }}
#[rustfmt::skip]
impl Fromf64 for i16  { fn fromf64(value: f64) -> Self { value as Self }}
#[rustfmt::skip]
impl Fromf64 for u8   { fn fromf64(value: f64) -> Self { value as Self }}
#[rustfmt::skip]
impl Fromf64 for bool { fn fromf64(value: f64) -> Self { value > 0.0   }}

///Trait qualifier for the type of Arrays accepted by scan operations
pub trait IndexableType: HasAfEnum {}

impl IndexableType for f64 {}
impl IndexableType for i64 {}
impl IndexableType for u64 {}
impl IndexableType for f32 {}
impl IndexableType for i32 {}
impl IndexableType for u32 {}
impl IndexableType for i16 {}
impl IndexableType for u16 {}
impl IndexableType for u8 {}

/// Trait qualifier for given type indicating computability of covariance
pub trait IntegralType: HasAfEnum {}

impl IntegralType for i64 {}
impl IntegralType for u64 {}
impl IntegralType for i32 {}
impl IntegralType for u32 {}
impl IntegralType for i16 {}
impl IntegralType for u16 {}
impl IntegralType for u8 {}
impl IntegralType for bool {}
