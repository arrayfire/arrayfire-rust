extern crate libc;

use std::fmt;
use std::default::Default;
use crate::num::{One, Zero};

/// Sequences are used for indexing Arrays
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Seq<T> {
    begin: T,
    end: T,
    step: T,
}

/// Default `Seq` spans all the elements along a dimension
impl<T: One+Zero> Default for Seq<T> {
    fn default() -> Self {
        Seq { begin: One::one(), end: One::one(), step: Zero::zero() }
    }
}

/// Enables use of `Seq` with `{}` format in print statements
impl<T: fmt::Display> fmt::Display for Seq<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[begin: {}, end: {}, step: {}]", self.begin, self.end, self.step)
    }
}

impl<T: Copy> Seq<T> {
    /// Create a `Seq` that goes from `begin` to `end` at a step size of `step`
    pub fn new(begin: T, end: T, step: T) -> Self {
        Seq { begin: begin, end: end, step: step, }
    }

    /// Get begin index of Seq
    pub fn begin(&self) -> T {
        self.begin
    }

    /// Get end index of Seq
    pub fn end(&self) -> T {
        self.end
    }

    /// Get step size of Seq
    pub fn step(&self) -> T {
        self.step
    }
}
