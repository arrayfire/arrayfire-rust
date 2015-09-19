extern crate libc;

use array::Array;
use defines::AfError;
use seq::Seq;
use self::libc::{c_int, c_uint, c_longlong};

type MutAfIndex    = *mut self::libc::c_longlong;
type MutAfArray    = *mut self::libc::c_longlong;
type AfArray       = self::libc::c_longlong;
type DimT          = self::libc::c_longlong;
type IndexT        = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_create_indexers(indexers: MutAfIndex) -> c_int;
    fn af_set_array_indexer(indexer: MutAfIndex, idx: AfArray, dim: DimT) -> c_int;
    fn af_set_seq_indexer(indexer: MutAfIndex, idx: *const Seq, dim: DimT, is_batch: c_int) -> c_int;
    fn af_release_indexers(indexers: MutAfIndex) -> c_int;

    fn af_index(out: MutAfArray, input: AfArray, ndims: c_uint, index: *const Seq) -> c_int;
    fn af_lookup(out: MutAfArray, arr: AfArray, indices: AfArray, dim: c_uint) -> c_int;
    fn af_assign_seq(out: MutAfArray, lhs: AfArray, ndims: c_uint, indices: *const Seq, rhs: AfArray) -> c_int;
    fn af_index_gen(out: MutAfArray, input: AfArray, ndims: DimT, indices: *const IndexT) -> c_int;
    fn af_assign_gen(out: MutAfArray, lhs: AfArray, ndims: DimT, indices: *const IndexT, rhs: AfArray) -> c_int;
}

pub struct Indexer {
    handle: i64,
    count: u32,
}

pub trait Indexable {
    fn set(&self, idxr: &Indexer, dim: u32, is_batch: Option<bool>) -> Result<(), AfError>;
}

impl Indexable for Array {
    #[allow(unused_variables)]
    fn set(&self, idxr: &Indexer, dim: u32, is_batch: Option<bool>) -> Result<(), AfError> {
        unsafe {
            let err_val = af_set_array_indexer(idxr.clone().get() as MutAfIndex,
                                             self.get() as AfArray,
                                             dim as DimT);
            match err_val {
                0 => Ok(()),
                _ => Err(AfError::from(err_val)),
            }
        }
    }
}

impl Indexable for Seq {
    fn set(&self, idxr: &Indexer, dim: u32, is_batch: Option<bool>) -> Result<(), AfError> {
        unsafe {
            let err_val = af_set_seq_indexer(idxr.clone().get() as MutAfIndex, self as *const Seq,
                                           dim as DimT, is_batch.unwrap() as c_int);
            match err_val {
                0 => Ok(()),
                _ => Err(AfError::from(err_val)),
            }
        }
    }
}

impl Indexer {
    #[allow(unused_mut)]
    pub fn new() -> Result<Indexer, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_create_indexers(&mut temp as MutAfIndex);
            match err_val {
                0 => Ok(Indexer{handle: temp, count: 0}),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    pub fn set_index<T: Indexable>(&mut self, idx: &T, dim: u32, is_batch: Option<bool>) -> Result<(), AfError> {
        self.count = self.count + 1;
        idx.set(self, dim, is_batch)
    }

    pub fn get(&self) -> i64 {
        self.handle
    }

    pub fn len(&self) -> u32 {
        self.count
    }
}

impl Drop for Indexer {
    fn drop(&mut self) {
        unsafe {
            let ret_val = af_release_indexers(self.handle as MutAfIndex);
            match ret_val {
                0 => (),
                _ => panic!("Failed to release indexers resource: {}", ret_val),
            }
        }
    }
}

pub fn index(input: &Array, seqs: &[Seq]) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_index(&mut temp as MutAfArray
                               , input.get() as AfArray, seqs.len() as u32
                               , seqs.as_ptr() as *const Seq);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

pub fn row(input: &Array, row_num: u64) -> Result<Array, AfError> {
    let dims_err = input.dims();
    let dims = match dims_err {
        Ok(dim) =>  dim.clone(),
        Err(e)  =>  panic!("Error unwrapping dims in row(): {}", e),
    };

    index(input, &[Seq::new(row_num as f64, row_num as f64, 1.0)
                    ,Seq::new(0.0, dims[1] as f64 - 1.0, 1.0)])
}

pub fn rows(input: &Array, first: u64, last: u64) -> Result<Array, AfError> {
    let dims_err = input.dims();
    let dims = match dims_err {
        Ok(dim) =>  dim.clone(),
        Err(e)  =>  panic!("Error unwrapping dims in row(): {}", e),
    };

    index(input, &[Seq::new(first as f64, last as f64, 1.0)
                    ,Seq::new(0.0, dims[1] as f64 - 1.0, 1.0)])
}

pub fn col(input: &Array, col_num: u64) -> Result<Array, AfError> {
    let dims_err = input.dims();
    let dims = match dims_err {
        Ok(dim) =>  dim.clone(),
        Err(e)  =>  panic!("Error unwrapping dims in row(): {}", e),
    };

    index(input, &[Seq::new(0.0, dims[0] as f64 - 1.0, 1.0)
                    ,Seq::new(col_num as f64, col_num as f64, 1.0)])
}

pub fn cols(input: &Array, first: u64, last: u64) -> Result<Array, AfError> {
    let dims_err = input.dims();
    let dims = match dims_err {
        Ok(dim) =>  dim.clone(),
        Err(e)  =>  panic!("Error unwrapping dims in row(): {}", e),
    };

    index(input, &[Seq::new(0.0, dims[0] as f64 - 1.0, 1.0)
                    ,Seq::new(first as f64, last as f64, 1.0)])
}

pub fn lookup(input: &Array, indices: &Array, seq_dim: i32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_lookup(&mut temp as MutAfArray, input.get() as AfArray,
                                indices.get() as AfArray, seq_dim as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

pub fn assign_seq(lhs: &Array, ndims: usize, seqs: &[Seq], rhs: &Array) -> Result<Array, AfError> {
    unsafe{
        let mut temp: i64 = 0;
        let err_val = af_assign_seq(&mut temp as MutAfArray, lhs.get() as AfArray,
                                    ndims as c_uint, seqs.as_ptr() as *const Seq,
                                    rhs.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

pub fn index_gen(input: &Array, indices: Indexer) -> Result<Array, AfError> {
    unsafe{
        let mut temp: i64 = 0;
        let err_val = af_index_gen(&mut temp as MutAfArray, input.get() as AfArray,
                                   indices.len() as DimT, indices.get() as *const IndexT);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

pub fn assign_gen(lhs: &Array, indices: &Indexer, rhs: &Array) -> Result<Array, AfError> {
    unsafe{
        let mut temp: i64 = 0;
        let err_val = af_assign_gen(&mut temp as MutAfArray, lhs.get() as AfArray,
                                    indices.len() as DimT, indices.get() as *const IndexT,
                                    rhs.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}
