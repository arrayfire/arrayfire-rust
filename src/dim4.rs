use super::Dim4 as Dim4;
use std::fmt;
use std::ops::Index;

impl Default for Dim4 {
    fn default() -> Dim4 {
        Dim4 { dims:[1, 1, 1, 1] }
    }
}

impl Index<usize> for Dim4 {
    type Output = u64;

    fn index<'a>(&'a self, _index: usize) ->&'a u64 {
        &self.dims[_index]
    }
}

impl fmt::Display for Dim4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {} {} {}]", self.dims[0], self.dims[1], self.dims[2], self.dims[3])
    }
}

impl Dim4 {
    pub fn new(dims: &[u64; 4]) -> Dim4 {
        Dim4 { dims: dims.clone(), }
    }

    pub fn elements(&self) -> u64 {
        self.dims[0]*self.dims[1]*self.dims[2]*self.dims[3]
    }

    pub fn ndims(&self) -> usize {
        let nelems = self.elements();
        match nelems {
            0 => 0,
            1 => 0,
            _ => {
                if self.dims[3] != 1 { 4 }
                else if self.dims[2] != 1 { 3 }
                else if self.dims[1] != 1 { 2 }
                else { 1 }
            },
        }
    }

    pub fn get(&self) -> &[u64; 4] {
        &self.dims
    }
}
