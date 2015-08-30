extern crate libc;

use array::Array;
use defines::AfError;
use dim4::Dim4;
use seq::Seq;
use defines::Aftype;
use self::libc::{uint8_t, c_void, c_int, c_uint, c_longlong};

type MutAfArray    = *mut self::libc::c_longlong;
type MutAfSeq      = *mut Seq;
type MutMutAfIndex = *mut *mut self::libc::c_longlong;
type MutAfIndex    = *mut self::libc::c_longlong;
type MutDouble     = *mut self::libc::c_double;
type MutUint       = *mut self::libc::c_uint;
type AfArray       = self::libc::c_longlong;
type AfIndex       = self::libc::c_longlong;
type DimT          = self::libc::c_longlong;
type SeqT          = self::libc::c_longlong;
type IndexT        = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_create_seq_index(result: MutMutAfIndex, input: *const Seq, is_batch: c_int) -> c_int;
    fn af_create_array_index(result: MutMutAfIndex, input: AfArray) -> c_int;
    fn af_release_index(indexer: MutAfIndex) -> c_int;
    fn af_index(out: MutAfArray, input: AfArray, ndims: c_uint, index: *const IndexT) -> c_int;
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
    pub fn new<T>(arr: Option<Array>, seq: Option<Seq>, is_batch: bool) -> Result<Index, AfError> {
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

    pub fn index(&self, input: Array, seqs: &[Seq]) -> Result<Array, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_index(&mut temp as MutAfArray
                                    , input.get() as AfArray, seqs.len() as u32
                                    , seqs.as_ptr() as *const SeqT);
            match err_val {
                0 => Ok(Array::from(temp)),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    pub fn lookup(&self, input: Array, indices: Array, seq_dim: i32) -> Result<Array, AfError> {
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

    pub fn assign_seq(&self, lhs: Array, ndims: usize, seq_dim: i32, seqs: &[Seq], rhs: Array) -> Result<Array, AfError> {
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

    pub fn index_gen(&self, input: Array, ndims: Dim4, indices: &[Index]) -> Result<Array, AfError> {
        unsafe{
            let mut temp: i64 = 0;
            let err_val = af_index_gen(&mut temp as MutAfArray
                                      , input.get() as AfArray
                                      , ndims.get().as_ptr() as DimT
                                      , indices.as_ptr() as *const IndexT);
            match err_val {
                0 => Ok(Array::from(temp)),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    pub fn assign_gen(&self, lhs: Array, ndims: Dim4, indices: &[Index], rhs: Array) -> Result<Array, AfError> {
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

    pub fn get(&self) -> i64 {
        self.handle
    }

    pub fn is_seq(&self) -> bool{
        self.is_seq
    }

    pub fn is_batch(&self) -> bool{
        self.is_seq
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