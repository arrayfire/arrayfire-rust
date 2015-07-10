use super::Aftype as Aftype;
use super::InterpType as InterpType;
use super::ConvMode as ConvMode;
use super::ConvDomain as ConvDomain;
use super::MatProp as MatProp;
use std::mem;

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
