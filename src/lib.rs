#[derive(Copy, Clone)]
pub enum Aftype {
    F32,
    C32,
    F64,
    C64,
    B8,
    S32,
    U32,
    U8,
    S64,
    U64,
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

mod dim4;

mod util;

pub use device::{get_version, info, set_device};
mod device;

//pub use algorithm::{sum_nan, product_nan, sum_nan_all, product_nan_all};
pub use algorithm::{sum, product, min, max, all_true, any_true, count};
pub use algorithm::{sum_all, product_all, min_all, max_all};
pub use algorithm::{all_true_all, any_true_all, count_all, imin, imax, imin_all, imax_all};
pub use algorithm::{accum, locate, diff1, diff2, sort, sort_index, sort_by_key};
pub use algorithm::{set_unique, set_union, set_intersect};
mod algorithm;

pub use arith::{sin};
mod arith;

pub use data::{constant, range, iota};
pub use data::{set_seed, get_seed, randu, randn};
//pub use data::{identity, diag_create, diag_extract, lower, upper};
//pub use data::{join, join_many, tile};
//pub use data::{reorder, shift, moddims, flat, flip};
mod data;

pub use signal::{fft, fft2, fft3};
mod signal;
