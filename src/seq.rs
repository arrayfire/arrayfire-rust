extern crate libc;

use std::fmt;
use std::default::Default;
use self::libc::{c_double};

/// Sequences are used for indexing Arrays
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Seq {
    begin: c_double,
    end: c_double,
    step: c_double,
}

/// Default `Seq` spans all the elements along a dimension
impl Default for Seq {
    fn default() -> Seq {
        Seq { begin: 1.0, end: 1.0, step: 0.0, }
    }
}

/// Enables use of `Seq` with `{}` format in print statements
impl fmt::Display for Seq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[begin: {}, end: {}, step: {}]", self.begin, self.end, self.step)
    }
}

impl Seq {
    /// Create a `Seq` that goes from `begin` to `end` at a step size of `step`
    pub fn new(begin: f64, end: f64, step: f64) -> Seq {
        Seq { begin: begin, end: end, step: step, }
    }

    /// Get begin index of Seq
    pub fn begin(&self) -> f64 {
        self.begin as f64
    }

    /// Get begin index of Seq
    pub fn end(&self) -> f64 {
        self.end as f64
    }

    /// Get step size of Seq
    pub fn step(&self) -> f64 {
        self.step as f64
    }
}
