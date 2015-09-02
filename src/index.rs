extern crate libc;

use array::Array;
use defines::AfError;
use dim4::Dim4;
use seq::Seq;
use defines::Aftype;
use self::libc::{c_int, c_uint, c_longlong};

type MutAfArray    = *mut self::libc::c_longlong;
type MutMutAfIndex = *mut *mut self::libc::c_longlong;
type MutAfIndex    = *mut self::libc::c_longlong;
type AfArray       = self::libc::c_longlong;
type AfIndex       = self::libc::c_longlong;
type DimT          = self::libc::c_longlong;
type IndexT        = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_create_seq_index(result: MutMutAfIndex, input: *const Seq, is_batch: c_int) -> c_int;
    fn af_create_array_index(result: MutMutAfIndex, input: AfArray) -> c_int;
    fn af_release_index(indexer: MutAfIndex) -> c_int;
    fn af_index(out: MutAfArray, input: AfArray, ndims: c_uint, index: *const Seq) -> c_int;
    fn af_lookup(out: MutAfArray, arr: AfArray, indices: AfArray, dim: c_uint) -> c_int;
    fn af_assign_seq(out: MutAfArray, lhs: AfArray, ndims: c_uint, indices: *const IndexT, rhs: AfArray) -> c_int;
    fn af_index_gen(out: MutAfArray, input: AfArray, ndims: DimT, indices: *const IndexT) -> c_int;
    fn af_assign_gen(out: MutAfArray, lhs: AfArray, ndims: DimT, indices: *const IndexT, rhs: AfArray) -> c_int;
}

pub struct Index {
    handle: i64,
    is_batch: bool,
    is_seq: bool,
}

impl Index {
    #[allow(unused_mut)]
    pub fn new(arr: Option<Array>, seq: Option<Seq>, is_batch: bool) -> Result<Index, AfError> {
        unsafe {
            let mut err_val: c_int = 0;
            let mut temp: i64 = 0;
            let mut is_seq = false;
            err_val = match arr {
                //c_func(&mut (x as *mut libc::c_void)); --> (&mut x) as *mut _ as *mut *mut libc::c_void.
                //&mut temp as MutMutAfIndex, x),
                Some(mut x) => { is_seq = false; af_create_array_index((&mut temp) as *mut _ as MutMutAfIndex, x.get() as AfArray) },
                None    => 0,
            };

            err_val = match seq {
                Some(mut x) => { is_seq = true; af_create_seq_index((&mut temp) as *mut _ as MutMutAfIndex, &mut x, is_batch as c_int) },
                None    => AfError::ERR_UNKNOWN as c_int,
            };

            match err_val {
                0 => Ok(Index {handle: temp, is_batch: is_batch, is_seq: is_seq}),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    pub fn get(&self) -> i64{
        self.handle
    }

    pub fn is_seq(&self) -> bool{
        self.is_seq
    }

    pub fn is_batch(&self) -> bool{
        self.is_batch
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            let ret_val = af_release_index(self.handle as MutAfIndex);
            match ret_val {
                0 => (),
                _ => panic!("Failed to destruct Index: {}", ret_val),
            }
        }
    }
}

pub fn index(input: &Array, seqs: &[Seq]) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        println!("size is: {}", seqs.len() as u32);
        let err_val = af_index(&mut temp as MutAfArray
                                , input.get() as AfArray
                                , seqs.len() as u32
                                , seqs.clone().as_ptr() as *const Seq);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

pub fn row(input: &Array, row_num: u64) -> Result<Array, AfError> {
    let dims_err = input.dims();
    let mut dims = Dim4::default();
    match dims_err {
        Ok(dim) =>  dims = dim.clone(),
        Err(e)  =>  panic!("Error unwrapping dims in row(): {}", e),
    }
    
    index(input, &[Seq::new(row_num as f64, row_num as f64, 1.0)
                    ,Seq::new(0.0, dims[1] as f64 - 1.0, 1.0)])
}

pub fn col(input: &Array, col_num: u64) -> Result<Array, AfError> {
    let dims_err = input.dims();
    let mut dims = Dim4::default();
    match dims_err {
        Ok(dim) =>  dims = dim.clone(),
        Err(e)  =>  panic!("Error unwrapping dims in row(): {}", e),
    }
    
    index(input, &[Seq::new(0.0, dims[0] as f64 - 1.0, 1.0)
                    ,Seq::new(col_num as f64, col_num as f64, 1.0)])
}

pub fn lookup(input: &Array, indices: &Array, seq_dim: i32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_lookup(&mut temp as MutAfArray
                                , input.get() as AfArray
                                , indices.get() as AfArray
                                , seq_dim as c_uint);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

pub fn assign_seq(lhs: &Array, ndims: usize, seqs: &[Seq], rhs: &Array) -> Result<Array, AfError> {
    unsafe{
        let mut temp: i64 = 0;
        let err_val = af_assign_seq(&mut temp as MutAfArray
                                    , lhs.get() as AfArray
                                    , ndims as c_uint, seqs.as_ptr() as *const IndexT
                                    , rhs.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

pub fn index_gen(input: &Array, ndims: Dim4, indices: &mut [Index]) -> Result<Array, AfError> {
    unsafe{
        let mut temp: i64 = 0;
        let mut index_ptrs = Vec::new();
        for index_struct in &mut indices[..] {
            index_ptrs.push(index_struct.get());
        }

        let err_val = af_index_gen(&mut temp as MutAfArray
                                  , input.get() as AfArray
                                  , ndims.get().as_ptr() as DimT
                                  , index_ptrs.as_ptr() as *const IndexT);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

pub fn assign_gen(lhs: &Array, ndims: Dim4, indices: &[Index], rhs: &Array) -> Result<Array, AfError> {
    unsafe{
        let mut temp: i64 = 0;
        let err_val = af_assign_gen(&mut temp as MutAfArray
                                    , lhs.get() as AfArray
                                    , ndims.get().as_ptr() as DimT
                                    , indices.as_ptr() as *const IndexT
                                    , rhs.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}