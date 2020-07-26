use std::fmt;
use std::ops::{Index, IndexMut};

/// Dim4 is used to store [Array](./struct.Array.html) dimensions
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Dim4 {
    dims: [u64; 4],
}

/// Default trait for Dim4 returns an Array of dimensions [1, 1, 1, 1]
impl Default for Dim4 {
    fn default() -> Self {
        Self { dims: [1, 1, 1, 1] }
    }
}

/// Enables index operation for Dim4
///
/// # Examples
///
/// ```rust
/// use arrayfire::Dim4;
///
/// let dims = Dim4::new(&[4, 4, 2, 1]);
/// println!("0th Dimension length is {}", dims[0]); // -> 4
/// println!("1th Dimension length is {}", dims[1]); // -> 4
/// println!("2th Dimension length is {}", dims[2]); // -> 2
/// println!("3th Dimension length is {}", dims[3]); // -> 1
/// ```
impl Index<usize> for Dim4 {
    type Output = u64;

    fn index(&self, _index: usize) -> &u64 {
        &self.dims[_index]
    }
}

/// Enables index operation for Dim4 to modify dimensions
///
/// # Examples
///
/// ```rust
/// use arrayfire::Dim4;
///
/// let mut dims = Dim4::new(&[4, 4, 2, 1]);
/// dims[2] = 4;
/// println!("Dimensions: {}", dims); // note that third dimension changed to 4
/// ```
impl IndexMut<usize> for Dim4 {
    fn index_mut(&mut self, _index: usize) -> &mut Self::Output {
        &mut self.dims[_index]
    }
}

/// Enables use of Dim4 objects for printing it to display
///
/// # Examples
///
/// ```rust
/// use arrayfire::Dim4;
///
/// let dims = Dim4::new(&[4, 4, 2, 1]);
/// println!("0th Dimension length is {}", dims[0]); // -> 4
/// ```
impl fmt::Display for Dim4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{} {} {} {}]",
            self.dims[0], self.dims[1], self.dims[2], self.dims[3]
        )
    }
}

impl Dim4 {
    /// Create Dim4 object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arrayfire::Dim4;
    /// let dims = Dim4::new(&[4, 4, 2, 1]);
    /// ```
    pub fn new(dims: &[u64; 4]) -> Self {
        Self {
            dims: [dims[0], dims[1], dims[2], dims[3]],
        }
    }

    /// Get the number of elements represented by Dim4 object
    pub fn elements(&self) -> u64 {
        self.dims[0] * self.dims[1] * self.dims[2] * self.dims[3]
    }

    /// Get the number of dimensions of Dim4
    pub fn ndims(&self) -> usize {
        let nelems = self.elements();
        match nelems {
            0 => 0,
            1 => 1,
            _ => {
                if self.dims[3] != 1 {
                    4
                } else if self.dims[2] != 1 {
                    3
                } else if self.dims[1] != 1 {
                    2
                } else {
                    1
                }
            }
        }
    }

    /// Get the dimensions as a slice of 4 values
    pub fn get(&self) -> &[u64; 4] {
        &self.dims
    }
}
