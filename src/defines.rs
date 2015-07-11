#[derive(Copy, Clone)]
pub enum Aftype {
    F32 = 0,
    C32 = 1,
    F64 = 2,
    C64 = 3,
    B8  = 4,
    S32 = 5,
    U32 = 6,
    U8  = 7,
    S64 = 8,
    U64 = 9,
}

#[derive(Copy, Clone)]
pub enum InterpType {
    NEAREST = 0,
    LINEAR  = 1,
    BILINEAR= 2,
    CUBIC   = 3,
}

#[derive(Copy, Clone)]
pub enum BorderType {
    ZERO = 0,
    SYMMETRIC = 1,
}

#[derive(Copy, Clone)]
pub enum Connectivity {
    FOUR = 4,
    EIGHT = 8
}

#[derive(Copy, Clone)]
pub enum ConvMode {
    DEFAULT = 0,
    EXPAND  = 1,
}

#[derive(Copy, Clone)]
pub enum ConvDomain {
    AUTO     = 0,
    SPATIAL  = 1,
    FREQUENCY= 2,
}

#[derive(Copy, Clone)]
pub enum MatchType {
    SAD = 0,
    ZSAD= 1,
    LSAD= 2,
    SSD = 3,
    ZSSD= 4,
    LSSD= 5,
    NCC = 6,
    ZNCC= 7,
    SHD = 8,
}

#[derive(Copy, Clone)]
pub enum ColorSpace {
    GRAY = 0,
    RGB  = 1,
    HSV  = 2,
}

#[derive(Copy, Clone)]
pub enum MatProp {
    NONE,
    TRANS,
    CTRANS,
    UPPER,
    LOWER,
    DIAGUNIT,
    SYM,
    POSDEF,
    ORTHOG,
    TRIDIAG,
    BLOCKDIAG,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum NormType {
    VECTOR_1    = 0,
    VECTOR_INF  = 1,
    VECTOR_2    = 2,
    VECTOR_P    = 3,
    MATRIX_1    = 4,
    MATRIX_INF  = 5,
    MATRIX_2    = 6,
    MATRIX_L_PQ = 7,
}
