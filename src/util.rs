use super::Aftype as Aftype;
use super::InterpType as InterpType;
use super::ConvMode as ConvMode;
use super::ConvDomain as ConvDomain;
use std::mem;

impl From<u8> for Aftype {
    fn from(t: u8) -> Aftype {
        assert!(Aftype::F32 as u8 <= t && t <= Aftype::U64 as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for InterpType {
    fn from(t: u8) -> InterpType {
        assert!(InterpType::Nearest as u8 <= t && t <= InterpType::Cubic as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for ConvMode {
    fn from(t: u8) -> ConvMode {
        assert!(ConvMode::Default as u8 <= t && t <= ConvMode::Expand as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for ConvDomain {
    fn from(t: u8) -> ConvDomain {
        assert!(ConvDomain::Auto as u8 <= t && t <= ConvDomain::Frequency as u8);
        unsafe { mem::transmute(t) }
    }
}
