extern crate num;

use defines::AfError;
use defines::Aftype;
use defines::InterpType;
use defines::ConvMode;
use defines::ConvDomain;
use defines::MatProp;
use defines::MatchType;
use defines::ColorMap;
use std::mem;
use self::num::Complex;

impl From<i32> for AfError {
    fn from(t: i32) -> AfError {
        assert!(AfError::SUCCESS as i32 <= t && t <= AfError::ERR_UNKNOWN as i32);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for Aftype {
    fn from(t: u8) -> Aftype {
        assert!(Aftype::F32 as u8 <= t && t <= Aftype::U64 as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for InterpType {
    fn from(t: u8) -> InterpType {
        assert!(InterpType::NEAREST as u8 <= t && t <= InterpType::CUBIC as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for ConvMode {
    fn from(t: u8) -> ConvMode {
        assert!(ConvMode::DEFAULT as u8 <= t && t <= ConvMode::EXPAND as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for ConvDomain {
    fn from(t: u8) -> ConvDomain {
        assert!(ConvDomain::AUTO as u8 <= t && t <= ConvDomain::FREQUENCY as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for MatchType {
    fn from(t: u8) -> MatchType {
        assert!(MatchType::SAD as u8 <= t && t <= MatchType::SHD as u8);
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
    fn get_af_dtype() -> Aftype;
}

macro_rules! impl_has_af_enum {
    ($rust_t: ty, $af_dtype: expr) => (
        impl HasAfEnum for $rust_t {
            fn get_af_dtype() -> Aftype {
                $af_dtype
            }
        }
    )
}

impl_has_af_enum!(f32, Aftype::F32);
impl_has_af_enum!(Complex<f32>, Aftype::C32);
impl_has_af_enum!(f64, Aftype::F64);
impl_has_af_enum!(Complex<f64>, Aftype::C64);
// FIXME: Rust bool may become incompatible in memory layout with C-ABI
// Currently, it is of size 1-byte
impl_has_af_enum!(bool, Aftype::B8);
impl_has_af_enum!(i32, Aftype::S32);
impl_has_af_enum!(u32, Aftype::U32);
impl_has_af_enum!(u8, Aftype::U8);
impl_has_af_enum!(i64, Aftype::S64);
impl_has_af_enum!(u64, Aftype::U64);
impl_has_af_enum!(i16, Aftype::S16);
impl_has_af_enum!(u16, Aftype::U16);
