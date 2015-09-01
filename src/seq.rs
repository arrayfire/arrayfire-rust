extern crate libc;

use std::fmt;
use std::default::Default;
use self::libc::{c_double};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Seq {
    begin: c_double,
    end: c_double,
    step: c_double,
}

impl Default for Seq {
    fn default() -> Seq {
        Seq { begin: 1.0, end: 1.0, step: 0.0, }
    }
}

impl fmt::Display for Seq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[begin: {}, end: {}, step: {}]", self.begin, self.end, self.step)
    }
}

impl Seq {
    pub fn new(begin: f64, end: f64, step: f64) -> Seq {
        Seq { begin: begin, end: end, step: step, }
    }

    pub fn begin(&self) -> f64 {
        self.begin as f64
    }

    pub fn end(&self) -> f64 {
        self.end as f64
    }

    pub fn step(&self) -> f64 {
        self.step as f64
    }
}
