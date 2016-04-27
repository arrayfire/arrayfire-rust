extern crate libc;

use array::Array;
use defines::AfError;
use error::HANDLE_ERROR;
use seq::Seq;
use self::libc::{c_double, c_int, c_uint};

type MutAfIndex    = *mut self::libc::c_longlong;
type MutAfArray    = *mut self::libc::c_longlong;
type AfArray       = self::libc::c_longlong;
type DimT          = self::libc::c_longlong;
type IndexT        = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_create_indexers(indexers: MutAfIndex) -> c_int;
    fn af_set_array_indexer(indexer: MutAfIndex, idx: AfArray, dim: DimT) -> c_int;
    fn af_set_seq_indexer(indexer: MutAfIndex, idx: *const SeqInternal, dim: DimT, is_batch: c_int) -> c_int;
    fn af_release_indexers(indexers: MutAfIndex) -> c_int;

    fn af_index(out: MutAfArray, input: AfArray, ndims: c_uint, index: *const SeqInternal) -> c_int;
    fn af_lookup(out: MutAfArray, arr: AfArray, indices: AfArray, dim: c_uint) -> c_int;
    fn af_assign_seq(out: MutAfArray, lhs: AfArray, ndims: c_uint, indices: *const SeqInternal, rhs: AfArray) -> c_int;
    fn af_index_gen(out: MutAfArray, input: AfArray, ndims: DimT, indices: *const IndexT) -> c_int;
    fn af_assign_gen(out: MutAfArray, lhs: AfArray, ndims: DimT, indices: *const IndexT, rhs: AfArray) -> c_int;
}

/// Struct to manage an array of resources of type `af_indexer_t`(ArrayFire C struct)
pub struct Indexer {
    handle: i64,
    count: u32,
}

// Trait that indicates that object can be used for indexing
//
// Any object to be able to be passed on to [./struct.Indexer.html#method.set_index] method
// should implement this trait with appropriate implementation
pub trait Indexable {
    fn set(&self, idxr: &Indexer, dim: u32, is_batch: Option<bool>);
}

/// Enables [Array](./struct.Array.html) to be used to index another Array
///
/// This is used in functions [index_gen](./fn.index_gen.html) and
/// [assign_gen](./fn.assign_gen.html)
impl Indexable for Array {
    #[allow(unused_variables)]
    fn set(&self, idxr: &Indexer, dim: u32, is_batch: Option<bool>) {
        unsafe {
            let err_val = af_set_array_indexer(idxr.clone().get() as MutAfIndex,
                                             self.get() as AfArray,
                                             dim as DimT);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

/// Enables [Seq](./struct.Seq.html) to be used to index another Array
///
/// This is used in functions [index_gen](./fn.index_gen.html) and
/// [assign_gen](./fn.assign_gen.html)
impl<T: Copy> Indexable for Seq<T> where c_double: From<T> {
    fn set(&self, idxr: &Indexer, dim: u32, is_batch: Option<bool>) {
        unsafe {
            let err_val = af_set_seq_indexer(idxr.clone().get() as MutAfIndex,
                                             &SeqInternal::from_seq(self) as *const SeqInternal,
                                             dim as DimT, is_batch.unwrap() as c_int);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

impl Indexer {
    #[allow(unused_mut)]
    pub fn new() -> Indexer {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_create_indexers(&mut temp as MutAfIndex);
            HANDLE_ERROR(AfError::from(err_val));
            Indexer{handle: temp, count: 0}
        }
    }

    /// Set either [Array](./struct.Array.html) or [Seq](./struct.Seq.html) to index an Array along `idx` dimension
    pub fn set_index<T: Indexable>(&mut self, idx: &T, dim: u32, is_batch: Option<bool>) {
        self.count = self.count + 1;
        idx.set(self, dim, is_batch)
    }

    /// Get native(ArrayFire) resource handle
    pub fn get(&self) -> i64 {
        self.handle
    }

    /// Get number of indexers
    ///
    /// This can be a maximum of four since currently ArrayFire supports maximum of four dimensions
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

/// Indexes the `input` Array using `seqs` Sequences
///
/// # Examples
///
/// ```
/// use arrayfire::{Dim4, Seq, index, randu, print};
/// let dims = Dim4::new(&[5, 5, 1, 1]);
/// let a = randu::<f32>(dims);
/// let seqs = &[Seq::new(1.0, 3.0, 1.0), Seq::default()];
/// let sub  = index(&a, seqs);
/// println!("a(seq(1, 3, 1), span)");
/// print(&sub);
/// ```
pub fn index<T: Copy>(input: &Array, seqs: &[Seq<T>]) -> Array
    where c_double: From<T>
{
    unsafe {
        let mut temp: i64 = 0;
        // TODO: allocating a whole new array on the heap just for this is BAD
        let seqs: Vec<SeqInternal> = seqs.iter().map(|s| SeqInternal::from_seq(s)).collect();
        let err_val = af_index(&mut temp as MutAfArray
                               , input.get() as AfArray, seqs.len() as u32
                               , seqs.as_ptr() as *const SeqInternal);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Extract `row_num` row from `input` Array
///
/// # Examples
///
/// ```
/// use arrayfire::{Dim4, randu, row, print};
/// let dims = Dim4::new(&[5, 5, 1, 1]);
/// let a = randu::<f32>(dims);
/// println!("Grab last row of the random matrix");
/// print(&a);
/// print(&row(&a, 4));
/// ```
#[allow(dead_code)]
pub fn row(input: &Array, row_num: u64) -> Array {
    index(input, &[Seq::new(row_num as f64, row_num as f64, 1.0),
                   Seq::default()])
}

#[allow(dead_code)]
/// Set row `row_num` in `input` Array to a new Array `new_row`
pub fn set_row(input: &Array, new_row: &Array, row_num: u64) -> Array {
    assign_seq(input,
               &[Seq::new(row_num as f64, row_num as f64, 1.0), Seq::default()],
               new_row)
}

#[allow(dead_code)]
/// Get all rows from `first` to `last` in the `input` Array
pub fn rows(input: &Array, first: u64, last: u64) -> Array {
    index(input, &[Seq::new(first as f64, last as f64, 1.0), Seq::default()])
}

#[allow(dead_code)]
/// Set rows from `first` to `last` in `input` Array with rows from Array `new_rows`
pub fn set_rows(input: &Array, new_rows: &Array, first: u64, last: u64) -> Array {
    assign_seq(input, &[Seq::new(first as f64, last as f64, 1.0), Seq::default()], new_rows)
}

/// Extract `col_num` col from `input` Array
///
/// # Examples
///
/// ```
/// use arrayfire::{Dim4, randu, col, print};
/// let dims = Dim4::new(&[5, 5, 1, 1]);
/// let a = randu::<f32>(dims);
/// println!("Grab last col of the random matrix");
/// print(&a);
/// print(&col(&a, 4));
/// ```
#[allow(dead_code)]
pub fn col(input: &Array, col_num: u64) -> Array {
    index(input, &[Seq::default(), Seq::new(col_num as f64, col_num as f64, 1.0)])
}

#[allow(dead_code)]
/// Set col `col_num` in `input` Array to a new Array `new_col`
pub fn set_col(input: &Array, new_col: &Array, col_num: u64) -> Array {
    assign_seq(input,
               &[Seq::default(), Seq::new(col_num as f64, col_num as f64, 1.0)],
               new_col)
}

#[allow(dead_code)]
/// Get all cols from `first` to `last` in the `input` Array
pub fn cols(input: &Array, first: u64, last: u64) -> Array {
    index(input, &[Seq::default(), Seq::new(first as f64, last as f64, 1.0)])
}

#[allow(dead_code)]
/// Set cols from `first` to `last` in `input` Array with cols from Array `new_cols`
pub fn set_cols(input: &Array, new_cols: &Array, first: u64, last: u64) -> Array {
    assign_seq(input, &[Seq::default(), Seq::new(first as f64, last as f64, 1.0)], new_cols)
}

#[allow(dead_code)]
/// Get slice `slice_num` from `input` Array
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn slice(input: &Array, slice_num: u64) -> Array {
    index(input, 
          &[Seq::default(), Seq::default(), Seq::new(slice_num as f64, slice_num as f64, 1.0)])
}

#[allow(dead_code)]
/// Set slice `slice_num` in `input` Array to a new Array `new_slice`
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn set_slice(input: &Array, new_slice: &Array, slice_num: u64) -> Array {
    assign_seq(input,
               &[Seq::default(), Seq::default(), Seq::new(slice_num as f64, slice_num as f64, 1.0)],
               new_slice)
}

#[allow(dead_code)]
/// Get slices from `first` to `last` in `input` Array
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn slices(input: &Array, first: u64, last: u64) -> Array {
    index(input,
          &[Seq::default(), Seq::default(), Seq::new(first as f64, last as f64, 1.0)])
}

#[allow(dead_code)]
/// Set `first` to `last` slices of `input` Array to a new Array `new_slices`
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn set_slices(input: &Array, new_slices: &Array, first: u64, last: u64) -> Array {
    assign_seq(input,
               &[Seq::default() , Seq::default(), Seq::new(first as f64, last as f64, 1.0)],
               new_slices)
}

/// Lookup(hash) an Array using another Array
///
/// Given a dimension `seq_dim`, `indices` are lookedup in `input` and returned as a new
/// Array if found
pub fn lookup(input: &Array, indices: &Array, seq_dim: i32) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_lookup(&mut temp as MutAfArray, input.get() as AfArray,
                                indices.get() as AfArray, seq_dim as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Assign(copy) content of an Array to another Array indexed by Sequences
///
/// Assign `rhs` to `lhs` after indexing `lhs`
///
/// # Examples
///
/// ```
/// use arrayfire::{constant, Dim4, Seq, assign_seq, print};
/// let a    = constant(2.0 as f32, Dim4::new(&[5, 3, 1, 1]));
/// let b    = constant(1.0 as f32, Dim4::new(&[3, 3, 1, 1]));
/// let seqs = &[Seq::new(1.0, 3.0, 1.0), Seq::default()];
/// let sub  = assign_seq(&a, seqs, &b);
/// print(&a);
/// // 2.0 2.0 2.0
/// // 2.0 2.0 2.0
/// // 2.0 2.0 2.0
/// // 2.0 2.0 2.0
/// // 2.0 2.0 2.0
///
/// print(&sub);
/// // 2.0 2.0 2.0
/// // 1.0 1.0 1.0
/// // 1.0 1.0 1.0
/// // 1.0 1.0 1.0
/// // 2.0 2.0 2.0
/// ```
pub fn assign_seq<T: Copy>(lhs: &Array, seqs: &[Seq<T>], rhs: &Array) -> Array
                           where c_double: From<T>
{
    unsafe{
        let mut temp: i64 = 0;
        // TODO: allocating a whole new array on the heap just for this is BAD
        let seqs: Vec<SeqInternal> = seqs.iter().map(|s| SeqInternal::from_seq(s)).collect();
        let err_val = af_assign_seq(&mut temp as MutAfArray, lhs.get() as AfArray,
                                    seqs.len() as c_uint, seqs.as_ptr() as *const SeqInternal,
                                    rhs.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Index an Array using any combination of Array's and Sequence's
///
/// # Examples
///
/// ```
/// use arrayfire::{Array, Dim4, Seq, print, randu, index_gen, Indexer};
/// let values: &[f32] = &[1.0, 2.0, 3.0];
/// let indices = Array::new(values, Dim4::new(&[3, 1, 1, 1]));
/// let seq4gen = Seq::new(0.0, 2.0, 1.0);
/// let a = randu::<f32>(Dim4::new(&[5, 3, 1, 1]));
/// // [5 3 1 1]
/// //     0.0000     0.2190     0.3835
/// //     0.1315     0.0470     0.5194
/// //     0.7556     0.6789     0.8310
/// //     0.4587     0.6793     0.0346
/// //     0.5328     0.9347     0.0535
///
///
/// let mut idxrs = Indexer::new();
/// idxrs.set_index(&indices, 0, None); // 2nd parameter is indexing dimension
/// idxrs.set_index(&seq4gen, 1, Some(false)); // 3rd parameter indicates batch operation
///
/// let sub2 = index_gen(&a, idxrs);
/// println!("a(indices, seq(0, 2, 1))"); print(&sub2);
/// // [3 3 1 1]
/// //     0.1315     0.0470     0.5194
/// //     0.7556     0.6789     0.8310
/// //     0.4587     0.6793     0.0346
/// ```
pub fn index_gen(input: &Array, indices: Indexer) -> Array {
    unsafe{
        let mut temp: i64 = 0;
        let err_val = af_index_gen(&mut temp as MutAfArray, input.get() as AfArray,
                                   indices.len() as DimT, indices.get() as *const IndexT);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

/// Assign an Array to another after indexing it using any combination of Array's and Sequence's
///
/// # Examples
///
/// ```
/// use arrayfire::{Array, Dim4, Seq, print, randu, constant, Indexer, assign_gen};
/// let values: &[f32] = &[1.0, 2.0, 3.0];
/// let indices = Array::new(values, Dim4::new(&[3, 1, 1, 1]));
/// let seq4gen = Seq::new(0.0, 2.0, 1.0);
/// let a = randu::<f32>(Dim4::new(&[5, 3, 1, 1]));
/// // [5 3 1 1]
/// //     0.0000     0.2190     0.3835
/// //     0.1315     0.0470     0.5194
/// //     0.7556     0.6789     0.8310
/// //     0.4587     0.6793     0.0346
/// //     0.5328     0.9347     0.0535
///
/// let b    = constant(2.0 as f32, Dim4::new(&[3, 3, 1, 1]));
///
/// let mut idxrs = Indexer::new();
/// idxrs.set_index(&indices, 0, None); // 2nd parameter is indexing dimension
/// idxrs.set_index(&seq4gen, 1, Some(false)); // 3rd parameter indicates batch operation
///
/// let sub2 = assign_gen(&a, &idxrs, &b);
/// println!("a(indices, seq(0, 2, 1))"); print(&sub2);
/// // [5 3 1 1]
/// //     0.0000     0.2190     0.3835
/// //     2.0000     2.0000     2.0000
/// //     2.0000     2.0000     2.0000
/// //     2.0000     2.0000     2.0000
/// //     0.5328     0.9347     0.0535
/// ```
pub fn assign_gen(lhs: &Array, indices: &Indexer, rhs: &Array) -> Array {
    unsafe{
        let mut temp: i64 = 0;
        let err_val = af_assign_gen(&mut temp as MutAfArray, lhs.get() as AfArray,
                                    indices.len() as DimT, indices.get() as *const IndexT,
                                    rhs.get() as AfArray);
        HANDLE_ERROR(AfError::from(err_val));
        Array::from(temp)
    }
}

#[repr(C)]
struct SeqInternal {
    begin: c_double,
    end: c_double,
    step: c_double,
}

impl SeqInternal {
    fn from_seq<T: Copy>(s: &Seq<T>) -> Self where c_double: From<T> {
        SeqInternal {
            begin: From::from(s.begin()),
            end: From::from(s.end()),
            step: From::from(s.step()),
        }
    }
}