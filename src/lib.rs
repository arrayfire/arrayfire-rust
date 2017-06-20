//! ArrayFire is a high performance software library for parallel computing with
//! an easy-to-use API. ArrayFire abstracts away much of the details of
//! programming parallel architectures by providing a high-level container object,
//! the [Array](./struct.Array.html), that represents data stored on a CPU, GPU, FPGA,
//! or other type of accelerator. This abstraction permits developers to write
//! massively parallel applications in a high-level language where they need
//! not be concerned about low-level optimizations that are frequently required to
//! achieve high throughput on most parallel architectures.

//! This crate provides Rust bindings for the ArrayFire library. Given below table shows the rust bindings compatability with ArrayFire upstream.  If you find any bugs, please report them on [github](https://github.com/arrayfire/arrayfire-rust/issues).
//!
//!
//! | ArrayFire Upstream | Rust Crate |
//! |:------------------:|:---------------:|
//! | 3.3.x | 3.3.x |
//! | 3.4.x | 3.4.x |
//!
//! Only, Major & Minor version numbers need to match.
//!
//! ## Tutorials
//!
//! - [Getting Started with ArrayFire](./getting_started.html)
//! - [Introduction to Vectorization](./vectorization.html)
//! - [Array and Matrix Manipulation](./array_and_matrix_manipulation.html)
//! - [Indexing](./indexing.html)
//! - [Configure ArrayFire Environment](./configuring_arrayfire_environment.html)

#![doc(html_logo_url = "http://www.arrayfire.com/logos/arrayfire_logo_symbol.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico",
       html_root_url = "http://arrayfire.com/docs/rust")]
#![warn(missing_docs)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;

pub use array::*;
mod array;

#[cfg(feature="algorithm")]
pub use algorithm::*;
#[cfg(feature="algorithm")]
mod algorithm;

#[cfg(feature="arithmetic")]
pub use arith::*;
#[cfg(feature="arithmetic")]
mod arith;

pub use backend::*;
mod backend;

#[cfg(feature="blas")]
pub use blas::*;
#[cfg(feature="blas")]
mod blas;

#[cfg(feature="data")]
pub use data::*;
#[cfg(feature="data")]
mod data;

pub use device::*;
mod device;

pub use defines::*;
mod defines;

pub use dim4::Dim4;
mod dim4;

pub use error::{Callback, ErrorCallback, register_error_handler, handle_error_general};
mod error;

#[cfg(feature="indexing")]
pub use index::*;
#[cfg(feature="indexing")]
mod index;

pub use seq::Seq;
mod seq;

#[cfg(feature="graphics")]
pub use graphics::Window;
#[cfg(feature="graphics")]
mod graphics;

#[cfg(feature="image")]
pub use image::*;
#[cfg(feature="image")]
mod image;

#[cfg(feature="lapack")]
pub use lapack::*;
#[cfg(feature="lapack")]
mod lapack;

#[cfg(feature="macros")]
mod macros;
mod num;

#[cfg(feature="random")]
pub use random::*;
#[cfg(feature="random")]
mod random;

#[cfg(feature="signal")]
pub use signal::*;
#[cfg(feature="signal")]
mod signal;

#[cfg(feature="sparse")]
pub use sparse::*;
#[cfg(feature="sparse")]
mod sparse;

#[cfg(feature="statistics")]
pub use statistics::*;
#[cfg(feature="statistics")]
mod statistics;

pub use util::{HasAfEnum, get_size};
mod util;

#[cfg(feature="vision")]
pub use vision::*;
#[cfg(feature="vision")]
mod vision;

// headers that are not exposed through rust wrapper are given follows:
// compatible.h
// constants.h
// complex.h
// cuda.h
// opencl.h
