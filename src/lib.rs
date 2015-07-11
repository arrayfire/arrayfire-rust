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

#[derive(Copy, Clone)]
pub struct Dim4 {
    dims: [u64; 4],
}

pub struct Array {
    handle: i64,
}

pub use array::{print};
mod array;

//pub use algorithm::{sum_nan, product_nan, sum_nan_all, product_nan_all};
pub use algorithm::{sum, product, min, max, all_true, any_true, count};
pub use algorithm::{sum_all, product_all, min_all, max_all};
pub use algorithm::{all_true_all, any_true_all, count_all, imin, imax, imin_all, imax_all};
pub use algorithm::{accum, locate, diff1, diff2, sort, sort_index, sort_by_key};
pub use algorithm::{set_unique, set_union, set_intersect};
mod algorithm;

pub use arith::{lt, gt, le, ge, eq, neq, and, or, minof, maxof};
pub use arith::{abs, sign, round, trunc, floor, ceil, modulo};
pub use arith::{sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh};
pub use arith::{atan2, cplx2, arg, cplx, real, imag, conjg, hypot};
pub use arith::{sqrt, log, log1p, log10, log2, pow2, exp, expm1, erf, erfc, root, pow};
pub use arith::{cbrt, factorial, tgamma, lgamma, iszero, isinf, isnan};
mod arith;

pub use blas::{matmul, dot, transpose, transpose_inplace};
mod blas;

pub use data::{constant, range, iota};
pub use data::{set_seed, get_seed, randu, randn};
pub use data::{identity, diag_create, diag_extract, lower, upper};
pub use data::{join, join_many, tile};
pub use data::{reorder, shift, moddims, flat, flip};
mod data;

pub use device::{get_version, info, set_device};
mod device;

mod dim4;

pub use signal::{approx1, approx2};
pub use signal::{fft, fft2, fft3, ifft, ifft2, ifft3};
pub use signal::{convolve1, convolve2, convolve3, convolve2_sep};
pub use signal::{fft_convolve1, fft_convolve2, fft_convolve3};
pub use signal::{fir, iir};
mod signal;

pub use statistics::{mean, stdev, median, var, cov, corrcoef};
pub use statistics::{mean_weighted, var_weighted};
pub use statistics::{var_all, mean_all, stdev_all, median_all};
pub use statistics::{mean_all_weighted, var_all_weighted};
mod statistics;

mod util;

pub use vision::Features;
pub use vision::{fast, orb, hamming_matcher, match_template};
mod vision;
