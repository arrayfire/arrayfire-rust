#![doc(html_logo_url = "http://www.arrayfire.com/logos/arrayfire_logo_symbol.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://arrayfire.com/docs/rust")]

pub use array::Array;
pub use array::{print};
mod array;

//pub use algorithm::{sum_nan, product_nan, sum_nan_all, product_nan_all};
pub use algorithm::{sum, product, min, max, all_true, any_true, count, sum_nan, product_nan};
pub use algorithm::{sum_all, product_all, min_all, max_all, sum_nan_all, product_nan_all};
pub use algorithm::{all_true_all, any_true_all, count_all, imin, imax, imin_all, imax_all};
pub use algorithm::{accum, locate, diff1, diff2, sort, sort_index, sort_by_key};
pub use algorithm::{set_unique, set_union, set_intersect};
mod algorithm;

pub use arith::{add, sub, div, mul, lt, gt, le, ge, eq, neq, and, or, minof, maxof, rem};
pub use arith::{bitand, bitor, bitxor, shiftl, shiftr};
pub use arith::{abs, sign, round, trunc, floor, ceil, modulo, sigmoid};
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
pub use data::{select, selectl, selectr, replace, replace_scalar};
mod data;

pub use device::{get_version, info, device_count, is_double_available, set_device, get_device, sync};
mod device;

pub use defines::{Aftype, AfError, ColorMap, YCCStd};
pub use defines::{InterpType, BorderType, MatchType, NormType};
pub use defines::{Connectivity, ConvMode, ConvDomain, ColorSpace, MatProp};
mod defines;

pub use dim4::Dim4;
mod dim4;

pub use index::{Indexer, index, row, rows, col, cols, slice, slices
                , set_row, set_rows, set_col, set_cols, set_slice, set_slices
                , lookup, assign_seq, index_gen, assign_gen};
mod index;

pub use seq::Seq;
mod seq;

pub use graphics::Window;
mod graphics;

pub use image::{gaussian_kernel, load_image, save_image};
pub use image::{resize, transform, rotate, translate, scale, skew};
pub use image::{dilate, dilate3, erode, erode3, minfilt, maxfilt};
pub use image::{gradient, histogram, hist_equal, regions};
pub use image::{gray2rgb, rgb2gray, hsv2rgb, rgb2hsv, color_space};
pub use image::{bilateral, mean_shift, medfilt, sobel};
pub use image::{unwrap, wrap, sat, rgb2ycbcr, ycbcr2rgb};
mod image;

pub use lapack::{svd, lu, qr, cholesky, solve, solve_lu, inverse, det, rank, norm};
pub use lapack::{svd_inplace, lu_inplace, qr_inplace, cholesky_inplace};
mod lapack;

pub use signal::{approx1, approx2};
pub use signal::{fft, fft2, fft3, ifft, ifft2, ifft3};
pub use signal::{fft_r2c, fft2_r2c, fft3_r2c, fft_c2r, fft2_c2r, fft3_c2r};
pub use signal::{fft_inplace, fft2_inplace, fft3_inplace};
pub use signal::{ifft_inplace, ifft2_inplace, ifft3_inplace};
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
