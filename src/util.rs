extern crate libc;
extern crate num;

use defines::{AfError, ColorMap, ConvDomain, ConvMode, DType, InterpType, MatProp, MatchType};
use defines::{SparseFormat, BinaryOp, RandomEngineType};
use error::HANDLE_ERROR;
use std::mem;
use self::num::Complex;
use self::libc::{uint8_t, c_int, size_t, c_void};

pub type AfArray       = self::libc::c_longlong;
pub type CellPtr       = *const self::libc::c_void;
pub type Complex32     = Complex<f32>;
pub type Complex64     = Complex<f64>;
pub type DimT          = self::libc::c_longlong;
pub type Feat          = *const self::libc::c_void;
pub type IndexT        = self::libc::c_longlong;
pub type Intl          = self::libc::c_longlong;
pub type MutAfArray    = *mut self::libc::c_longlong;
pub type MutAfIndex    = *mut self::libc::c_longlong;
pub type MutDimT       = *mut self::libc::c_longlong;
pub type MutDouble     = *mut self::libc::c_double;
pub type MutFeat       = *mut *mut self::libc::c_void;
pub type MutRandEngine = *mut self::libc::c_longlong;
pub type MutUint       = *mut self::libc::c_uint;
pub type MutVoidPtr    = *mut self::libc::c_ulonglong;
pub type MutWndHandle  = *mut self::libc::c_ulonglong;
pub type RandEngine    = self::libc::c_longlong;
pub type Uintl         = self::libc::c_ulonglong;
pub type WndHandle     = self::libc::c_ulonglong;

#[allow(dead_code)]
extern {
    fn af_get_size_of(size: *mut size_t, aftype: uint8_t) -> c_int;

    fn af_alloc_host(ptr: *mut *const c_void, bytes: DimT) -> c_int;
    fn af_free_host(ptr: *mut c_void) -> c_int;
}

/// Get size, in bytes, of the arrayfire native type
pub fn get_size(value: DType) -> usize {
    unsafe {
        let mut ret_val: usize = 0;
        let err_val = af_get_size_of(&mut ret_val as *mut size_t, value as uint8_t);
        HANDLE_ERROR(AfError::from(err_val));
        ret_val
    }
}

/// Allocates space using Arrayfire allocator in host memory
#[allow(dead_code)]
pub fn alloc_host<T>(elements: usize, _type: DType) -> *const T {
    let ptr: *const T = ::std::ptr::null();
    let bytes = (elements * get_size(_type)) as DimT;
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
    fn from(t: i32) -> AfError {
        assert!(AfError::SUCCESS as i32 <= t && t <= AfError::ERR_UNKNOWN as i32);
        unsafe { mem::transmute(t) }
    }
}

impl From<i32> for DType {
    fn from(t: i32) -> DType {
        assert!(DType::F32 as i32 <= t && t <= DType::U64 as i32);
        unsafe { mem::transmute(t) }
    }
}

impl From<i32> for InterpType {
    fn from(t: i32) -> InterpType {
        assert!(InterpType::NEAREST as i32 <= t && t <= InterpType::BICUBIC_SPLINE as i32);
        unsafe { mem::transmute(t) }
    }
}

impl From<i32> for ConvMode {
    fn from(t: i32) -> ConvMode {
        assert!(ConvMode::DEFAULT as i32 <= t && t <= ConvMode::EXPAND as i32);
        unsafe { mem::transmute(t) }
    }
}

impl From<i32> for ConvDomain {
    fn from(t: i32) -> ConvDomain {
        assert!(ConvDomain::AUTO as i32 <= t && t <= ConvDomain::FREQUENCY as i32);
        unsafe { mem::transmute(t) }
    }
}

impl From<i32> for MatchType {
    fn from(t: i32) -> MatchType {
        assert!(MatchType::SAD as i32 <= t && t <= MatchType::SHD as i32);
        unsafe { mem::transmute(t) }
    }
}

pub fn to_u32(t: MatProp) -> u32 {
    match t {
        MatProp::NONE       =>  0,
        MatProp::TRANS      =>  1,
        MatProp::CTRANS     =>  2,
        MatProp::UPPER      =>  32,
        MatProp::LOWER      =>  64,
        MatProp::DIAGUNIT  =>  128,
        MatProp::SYM        =>  512,
        MatProp::POSDEF     =>  1024,
        MatProp::ORTHOG     =>  2048,
        MatProp::TRIDIAG   =>  4096,
        MatProp::BLOCKDIAG =>  8192,
    }
}

impl From<i32> for ColorMap {
    fn from(t: i32) -> ColorMap {
        assert!(ColorMap::DEFAULT as i32 <= t && t <= ColorMap::BLUE as i32);
        unsafe { mem::transmute(t) }
    }
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
pub trait HasAfEnum {
    /// Return trait implmentors corresponding [DType](./enum.DType.html)
    fn get_af_dtype() -> DType;
}

macro_rules! impl_has_af_enum {
    ($rust_t: ty, $af_dtype: expr) => (
        impl HasAfEnum for $rust_t {
            fn get_af_dtype() -> DType {
                $af_dtype
            }
        }
    )
}

impl_has_af_enum!(f32, DType::F32);
impl_has_af_enum!(Complex<f32>, DType::C32);
impl_has_af_enum!(f64, DType::F64);
impl_has_af_enum!(Complex<f64>, DType::C64);
// FIXME: Rust bool may become incompatible in memory layout with C-ABI
// Currently, it is of size 1-byte
impl_has_af_enum!(bool, DType::B8);
impl_has_af_enum!(i32, DType::S32);
impl_has_af_enum!(u32, DType::U32);
impl_has_af_enum!(u8, DType::U8);
impl_has_af_enum!(i64, DType::S64);
impl_has_af_enum!(u64, DType::U64);
impl_has_af_enum!(i16, DType::S16);
impl_has_af_enum!(u16, DType::U16);

impl From<i32> for SparseFormat {
    fn from(t: i32) -> SparseFormat {
        assert!(SparseFormat::DENSE as i32 <= t && t <= SparseFormat::COO as i32);
        unsafe { mem::transmute(t) }
    }
}

impl From<i32> for BinaryOp {
    fn from(t: i32) -> BinaryOp {
        assert!(BinaryOp::ADD as i32 <= t && t <= BinaryOp::MAX as i32);
        unsafe { mem::transmute(t) }
    }
}

impl From<i32> for RandomEngineType {
    fn from(t: i32) -> RandomEngineType {
        assert!(RandomEngineType::PHILOX_4X32_10 as i32 <= t && t <= RandomEngineType::MERSENNE_GP11213 as i32);
        unsafe { mem::transmute(t) }
    }
}
