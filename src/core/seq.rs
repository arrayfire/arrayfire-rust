use num::{One, Zero};

#[cfg(feature = "afserde")]
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fmt;

use super::util::IndexableType;

/// Sequences are used for indexing Arrays
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "afserde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Seq<T: IndexableType> {
    begin: T,
    end: T,
    step: T,
}

/// Default `Seq` spans all the elements along a dimension
impl<T> Default for Seq<T>
where
    T: One + Zero + IndexableType,
{
    fn default() -> Self {
        Self {
            begin: One::one(),
            end: One::one(),
            step: Zero::zero(),
        }
    }
}

/// Enables use of `Seq` with `{}` format in print statements
impl<T> fmt::Display for Seq<T>
where
    T: fmt::Display + IndexableType,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[begin: {}, end: {}, step: {}]",
            self.begin, self.end, self.step
        )
    }
}

impl<T> Seq<T>
where
    T: Copy + IndexableType,
{
    /// Create a `Seq` that goes from `begin` to `end` at a step size of `step`
    pub fn new(begin: T, end: T, step: T) -> Self {
        Self { begin, end, step }
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

#[cfg(test)]
mod tests {
    #[cfg(feature = "afserde")]
    #[test]
    fn seq_serde() {
        use super::Seq;
        use crate::seq;

        // ANCHOR: seq_json_serde_snippet
        let original = seq!(1:2:1);
        let serd = match serde_json::to_string(&original) {
            Ok(serialized_str) => serialized_str,
            Err(e) => e.to_string(),
        };

        let deserd: Seq<i32> = serde_json::from_str(&serd).unwrap();
        assert_eq!(deserd, original);
        // ANCHOR_END: seq_json_serde_snippet
    }
}
