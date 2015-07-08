use super::Aftype as Aftype;

pub fn get_ffi_type(t: Aftype) -> i32 {
    match t {
        Aftype::F32 => 0,
        Aftype::C32 => 1,
        Aftype::F64 => 2,
        Aftype::C64 => 3,
        Aftype::B8  => 4,
        Aftype::S32 => 5,
        Aftype::U32 => 6,
        Aftype::U8  => 7,
        Aftype::S64 => 8,
        Aftype::U64 => 9,
    }
}

pub fn get_af_type(t: i32) -> Aftype {
    match t {
        0 => Aftype::F32,
        1 => Aftype::C32,
        2 => Aftype::F64,
        3 => Aftype::C64,
        4 => Aftype::B8 ,
        5 => Aftype::S32,
        6 => Aftype::U32,
        7 => Aftype::U8 ,
        8 => Aftype::S64,
        9 => Aftype::U64,
        _ => Aftype::F32,
    }
}
