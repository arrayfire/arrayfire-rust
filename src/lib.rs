//! ArrayFire is a high performance software library for parallel computing with an easy-to-use API.
//! ArrayFire abstracts away much of the details of programming parallel architectures by providing
//! a high-level container object, the [Array](./struct.Array.html), that represents data stored on
//! a CPU, GPU, FPGA, or other type of accelerator. This abstraction permits developers to write
//! massively parallel applications in a high-level language where they need not be concerned about
//! low-level optimizations that are frequently required to achieve high throughput on most parallel
//! architectures.

//! This crate provides Rust bindings for the ArrayFire library. Given below table shows the rust
//! bindings compatability with ArrayFire upstream.  If you find any bugs, please report them on
//! [github](https://github.com/arrayfire/arrayfire-rust/issues).
//!
//! | arrayfire-rust crate | ArrayFire Upstream |
//! |:--------------------:|:------------------:|
//! |         M.m.p1       |      M.m.p2        |
//!
//! Only, Major(M) & Minor(m) version numbers need to match. *p1* and *p2* are patch/fix updates
//! for `arrayfire-rust` & `ArrayFire` respectively, and they don't need to match.
//!
//! Please go through our [tutorials](http://arrayfire.org/arrayfire-rust/book/index.html) book for
//! more explanations on how to use ArrayFire to speedup your code.
//!
//! Note that the public traits on arrayfire-rust crate aren't meant to be implemented for user
//! defined types. If attempted, rust compiler will throw an error.

#![doc(
    html_logo_url = "https://www.arrayfire.com/logos/arrayfire_logo_symbol.png",
    html_favicon_url = "https://www.arrayfire.com/logos/arrayfire.ico",
    html_root_url = "https://arrayfire.org/arrayfire-rust/arrayfire/index.html"
)]
#![warn(missing_docs)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;

pub use crate::core::*;
mod core;

#[cfg(feature = "algorithm")]
pub use crate::algorithm::*;
#[cfg(feature = "algorithm")]
mod algorithm;

#[cfg(feature = "blas")]
pub use crate::blas::*;
#[cfg(feature = "blas")]
mod blas;

#[cfg(feature = "graphics")]
pub use crate::graphics::Window;
#[cfg(feature = "graphics")]
mod graphics;

#[cfg(feature = "image")]
pub use crate::image::*;
#[cfg(feature = "image")]
mod image;

#[cfg(feature = "lapack")]
pub use crate::lapack::*;
#[cfg(feature = "lapack")]
mod lapack;

#[cfg(feature = "ml")]
pub use crate::ml::*;
#[cfg(feature = "ml")]
mod ml;

#[cfg(feature = "signal")]
pub use crate::signal::*;
#[cfg(feature = "signal")]
mod signal;

#[cfg(feature = "sparse")]
pub use crate::sparse::*;
#[cfg(feature = "sparse")]
mod sparse;

#[cfg(feature = "statistics")]
pub use crate::statistics::*;
#[cfg(feature = "statistics")]
mod statistics;

#[cfg(feature = "vision")]
pub use crate::vision::*;
#[cfg(feature = "vision")]
mod vision;

// headers that are not exposed through rust wrapper are given follows:
// compatible.h
// constants.h
// complex.h
// cuda.h
// opencl.h
