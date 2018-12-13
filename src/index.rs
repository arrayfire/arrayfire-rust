extern crate libc;

use self::libc::{c_double, c_int, c_uint};
use crate::array::Array;
use crate::defines::AfError;
use crate::error::HANDLE_ERROR;
use crate::seq::Seq;
use crate::util::{AfArray, AfIndex, DimT, HasAfEnum, MutAfArray, MutAfIndex};

use std::marker::PhantomData;

#[allow(dead_code)]
extern "C" {
    fn af_create_indexers(indexers: MutAfIndex) -> c_int;
    fn af_set_array_indexer(indexer: AfIndex, idx: AfArray, dim: DimT) -> c_int;
    fn af_set_seq_indexer(
        indexer: AfIndex,
        idx: *const SeqInternal,
        dim: DimT,
        is_batch: c_int,
    ) -> c_int;
    fn af_release_indexers(indexers: AfIndex) -> c_int;

    fn af_index(out: MutAfArray, input: AfArray, ndims: c_uint, index: *const SeqInternal)
        -> c_int;
    fn af_lookup(out: MutAfArray, arr: AfArray, indices: AfArray, dim: c_uint) -> c_int;
    fn af_assign_seq(
        out: MutAfArray,
        lhs: AfArray,
        ndims: c_uint,
        indices: *const SeqInternal,
        rhs: AfArray,
    ) -> c_int;
    fn af_index_gen(out: MutAfArray, input: AfArray, ndims: DimT, indices: AfIndex) -> c_int;
    fn af_assign_gen(
        out: MutAfArray,
        lhs: AfArray,
        ndims: DimT,
        indices: AfIndex,
        rhs: AfArray,
    ) -> c_int;
}

/// Struct to manage an array of resources of type `af_indexer_t`(ArrayFire C struct)
///
/// # Examples
///
/// Given below are examples illustrating correct and incorrect usage of Indexer struct.
///
/// <h3> Correct Usage </h3>
///
/// ```rust
/// use arrayfire::{Array, Dim4, randu, index_gen, Indexer};
///
/// // Always be aware of the fact that, the `Seq` or `Array` objects
/// // that we intend to use for indexing via `Indexer` have to outlive
/// // the `Indexer` object created in this context.
///
/// let dims    = Dim4::new(&[1, 3, 1, 1]);
/// let indices = [1u8, 0, 1];
/// let idx     = Array::new(&indices, dims);
/// let values  = [2.0f32, 5.0, 6.0];
/// let arr     = Array::new(&values, dims);
///
/// let mut idxr = Indexer::new();
///
/// // `idx` is created much before idxr, thus will
/// // stay in scope at least as long as idxr
/// idxr.set_index(&idx, 0, None);
///
/// index_gen(&arr, idxr);
/// ```
///
/// <h3> Incorrect Usage </h3>
///
/// ```rust,ignore
/// // Say, you create an Array on the fly and try
/// // to call set_index, it will throw the given below
/// // error or something similar to that
/// idxr.set_index(&Array::new(&[1, 0, 1], dims), 0, None);
/// ```
///
/// ```text
/// error: borrowed value does not live long enough
///   --> <anon>:16:55
///   |
///16 | idxr.set_index(&Array::new(&[1, 0, 1], dims), 0, None);
///   |                 ----------------------------          ^ temporary value dropped here while still borrowed
///   |                 |
///   |                 temporary value created here
///...
///19 | }
///   | - temporary value needs to live until here
///   |
///   = note: consider using a `let` binding to increase its lifetime
/// ```
pub struct Indexer<'object> {
    handle: i64,
    count: usize,
    marker: PhantomData<&'object ()>,
}

/// Trait bound indicating indexability
///
/// Any object to be able to be passed on to [Indexer::set_index()](./struct.Indexer.html#method.set_index) method  should implement this trait with appropriate implementation of `set` method.
pub trait Indexable {
    /// Set indexing object for a given dimension
    ///
    /// # Parameters
    ///
    /// - `idxr` is mutable reference to [Indexer](./struct.Indexer.html) object which will
    ///   be modified to set `self` indexable along `dim` dimension.
    /// - `dim` is the dimension along which `self` indexable will be used for indexing.
    /// - `is_batch` is only used if `self` is [Seq](./struct.Seq.html) to indicate if indexing
    ///   along `dim` is a batched operation.
    fn set(&self, idxr: &mut Indexer, dim: u32, is_batch: Option<bool>);
}

/// Enables [Array](./struct.Array.html) to be used to index another Array
///
/// This is used in functions [index_gen](./fn.index_gen.html) and
/// [assign_gen](./fn.assign_gen.html)
impl<T: HasAfEnum> Indexable for Array<T> {
    #[allow(unused_variables)]
    fn set(&self, idxr: &mut Indexer, dim: u32, is_batch: Option<bool>) {
        unsafe {
            let err_val =
                af_set_array_indexer(idxr.get() as AfIndex, self.get() as AfArray, dim as DimT);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

/// Enables [Seq](./struct.Seq.html) to be used to index another Array
///
/// This is used in functions [index_gen](./fn.index_gen.html) and
/// [assign_gen](./fn.assign_gen.html)
impl<T: Copy> Indexable for Seq<T>
where
    c_double: From<T>,
{
    fn set(&self, idxr: &mut Indexer, dim: u32, is_batch: Option<bool>) {
        unsafe {
            let err_val = af_set_seq_indexer(
                idxr.get() as AfIndex,
                &SeqInternal::from_seq(self) as *const SeqInternal,
                dim as DimT,
                is_batch.unwrap() as c_int,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

impl<'object> Indexer<'object> {
    #[allow(unused_mut)]
    /// Create a new Indexer object and set the dimension specific index objects later
    pub fn new() -> Indexer<'object> {
        let mut temp: i64 = 0;
        unsafe {
            let err_val = af_create_indexers(&mut temp as MutAfIndex);
            HANDLE_ERROR(AfError::from(err_val));
        }
        Indexer {
            handle: temp,
            count: 0,
            marker: PhantomData,
        }
    }

    /// Set either [Array](./struct.Array.html) or [Seq](./struct.Seq.html) to index an Array along `idx` dimension
    pub fn set_index<'s, T>(&'s mut self, idx: &'object T, dim: u32, is_batch: Option<bool>)
    where
        T: Indexable + 'object,
    {
        idx.set(self, dim, is_batch);
        self.count = self.count + 1;
    }

    /// Get number of indexing objects set
    pub fn len(&self) -> usize {
        self.count
    }

    /// Get native(ArrayFire) resource handle
    pub fn get(&self) -> i64 {
        self.handle
    }
}

impl<'object> Drop for Indexer<'object> {
    fn drop(&mut self) {
        unsafe {
            let ret_val = af_release_indexers(self.handle as AfIndex);
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
/// ```rust
/// use arrayfire::{Dim4, Seq, index, randu, print};
/// let dims = Dim4::new(&[5, 5, 1, 1]);
/// let a = randu::<f32>(dims);
/// let seqs = &[Seq::new(1.0, 3.0, 1.0), Seq::default()];
/// let sub  = index(&a, seqs);
/// println!("a(seq(1, 3, 1), span)");
/// print(&sub);
/// ```
pub fn index<IO, T: Copy>(input: &Array<IO>, seqs: &[Seq<T>]) -> Array<IO>
where
    c_double: From<T>,
    IO: HasAfEnum,
{
    let mut temp: i64 = 0;
    unsafe {
        // TODO: allocating a whole new array on the heap just for this is BAD
        let seqs: Vec<SeqInternal> = seqs.iter().map(|s| SeqInternal::from_seq(s)).collect();
        let err_val = af_index(
            &mut temp as MutAfArray,
            input.get() as AfArray,
            seqs.len() as u32,
            seqs.as_ptr() as *const SeqInternal,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Extract `row_num` row from `input` Array
///
/// # Examples
///
/// ```rust
/// use arrayfire::{Dim4, randu, row, print};
/// let dims = Dim4::new(&[5, 5, 1, 1]);
/// let a = randu::<f32>(dims);
/// println!("Grab last row of the random matrix");
/// print(&a);
/// print(&row(&a, 4));
/// ```
#[allow(dead_code)]
pub fn row<T>(input: &Array<T>, row_num: u64) -> Array<T>
where
    T: HasAfEnum,
{
    index(
        input,
        &[
            Seq::new(row_num as f64, row_num as f64, 1.0),
            Seq::default(),
        ],
    )
}

#[allow(dead_code)]
/// Set `row_num`^th row in `input` Array to a new Array `new_row`
pub fn set_row<T>(input: &Array<T>, new_row: &Array<T>, row_num: u64) -> Array<T>
where
    T: HasAfEnum,
{
    let seqs = [
        Seq::new(row_num as f64, row_num as f64, 1.0),
        Seq::default(),
    ];
    assign_seq(input, &seqs, new_row)
}

#[allow(dead_code)]
/// Get an Array with all rows from `first` to `last` in the `input` Array
pub fn rows<T>(input: &Array<T>, first: u64, last: u64) -> Array<T>
where
    T: HasAfEnum,
{
    index(
        input,
        &[Seq::new(first as f64, last as f64, 1.0), Seq::default()],
    )
}

#[allow(dead_code)]
/// Set rows from `first` to `last` in `input` Array with rows from Array `new_rows`
pub fn set_rows<T>(input: &Array<T>, new_rows: &Array<T>, first: u64, last: u64) -> Array<T>
where
    T: HasAfEnum,
{
    let seqs = [Seq::new(first as f64, last as f64, 1.0), Seq::default()];
    assign_seq(input, &seqs, new_rows)
}

/// Extract `col_num` col from `input` Array
///
/// # Examples
///
/// ```rust
/// use arrayfire::{Dim4, randu, col, print};
/// let dims = Dim4::new(&[5, 5, 1, 1]);
/// let a = randu::<f32>(dims);
/// print(&a);
/// println!("Grab last col of the random matrix");
/// print(&col(&a, 4));
/// ```
#[allow(dead_code)]
pub fn col<T>(input: &Array<T>, col_num: u64) -> Array<T>
where
    T: HasAfEnum,
{
    index(
        input,
        &[
            Seq::default(),
            Seq::new(col_num as f64, col_num as f64, 1.0),
        ],
    )
}

#[allow(dead_code)]
/// Set `col_num`^th col in `input` Array to a new Array `new_col`
pub fn set_col<T>(input: &Array<T>, new_col: &Array<T>, col_num: u64) -> Array<T>
where
    T: HasAfEnum,
{
    let seqs = [
        Seq::default(),
        Seq::new(col_num as f64, col_num as f64, 1.0),
    ];
    assign_seq(input, &seqs, new_col)
}

#[allow(dead_code)]
/// Get all cols from `first` to `last` in the `input` Array
pub fn cols<T>(input: &Array<T>, first: u64, last: u64) -> Array<T>
where
    T: HasAfEnum,
{
    index(
        input,
        &[Seq::default(), Seq::new(first as f64, last as f64, 1.0)],
    )
}

#[allow(dead_code)]
/// Set cols from `first` to `last` in `input` Array with cols from Array `new_cols`
pub fn set_cols<T>(input: &Array<T>, new_cols: &Array<T>, first: u64, last: u64) -> Array<T>
where
    T: HasAfEnum,
{
    let seqs = [Seq::default(), Seq::new(first as f64, last as f64, 1.0)];
    assign_seq(input, &seqs, new_cols)
}

#[allow(dead_code)]
/// Get `slice_num`^th slice from `input` Array
///
/// Note. Slices indicate that the indexing is along 3rd dimension
pub fn slice<T>(input: &Array<T>, slice_num: u64) -> Array<T>
where
    T: HasAfEnum,
{
    let seqs = [
        Seq::default(),
        Seq::default(),
        Seq::new(slice_num as f64, slice_num as f64, 1.0),
    ];
    index(input, &seqs)
}

#[allow(dead_code)]
/// Set slice `slice_num` in `input` Array to a new Array `new_slice`
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn set_slice<T>(input: &Array<T>, new_slice: &Array<T>, slice_num: u64) -> Array<T>
where
    T: HasAfEnum,
{
    let seqs = [
        Seq::default(),
        Seq::default(),
        Seq::new(slice_num as f64, slice_num as f64, 1.0),
    ];
    assign_seq(input, &seqs, new_slice)
}

#[allow(dead_code)]
/// Get slices from `first` to `last` in `input` Array
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn slices<T>(input: &Array<T>, first: u64, last: u64) -> Array<T>
where
    T: HasAfEnum,
{
    let seqs = [
        Seq::default(),
        Seq::default(),
        Seq::new(first as f64, last as f64, 1.0),
    ];
    index(input, &seqs)
}

#[allow(dead_code)]
/// Set `first` to `last` slices of `input` Array to a new Array `new_slices`
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn set_slices<T>(input: &Array<T>, new_slices: &Array<T>, first: u64, last: u64) -> Array<T>
where
    T: HasAfEnum,
{
    let seqs = [
        Seq::default(),
        Seq::default(),
        Seq::new(first as f64, last as f64, 1.0),
    ];
    assign_seq(input, &seqs, new_slices)
}

/// Lookup(hash) an Array using another Array
///
/// Given a dimension `seq_dim`, `indices` are lookedup in `input` and returned as a new
/// Array if found
pub fn lookup<T, I>(input: &Array<T>, indices: &Array<I>, seq_dim: i32) -> Array<T>
where
    T: HasAfEnum,
    I: HasAfEnum,
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_lookup(
            &mut temp as MutAfArray,
            input.get() as AfArray,
            indices.get() as AfArray,
            seq_dim as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Assign(copy) content of an Array to another Array indexed by Sequences
///
/// Assign `rhs` to `lhs` after indexing `lhs`
///
/// # Examples
///
/// ```rust
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
pub fn assign_seq<T: Copy, I>(lhs: &Array<I>, seqs: &[Seq<T>], rhs: &Array<I>) -> Array<I>
where
    c_double: From<T>,
    I: HasAfEnum,
{
    let mut temp: i64 = 0;
    // TODO: allocating a whole new array on the heap just for this is BAD
    let seqs: Vec<SeqInternal> = seqs.iter().map(|s| SeqInternal::from_seq(s)).collect();
    unsafe {
        let err_val = af_assign_seq(
            &mut temp as MutAfArray,
            lhs.get() as AfArray,
            seqs.len() as c_uint,
            seqs.as_ptr() as *const SeqInternal,
            rhs.get() as AfArray,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Index an Array using any combination of Array's and Sequence's
///
/// # Examples
///
/// ```rust
/// use arrayfire::{Array, Dim4, Seq, print, randu, index_gen, Indexer};
/// let values: [f32; 3] = [1.0, 2.0, 3.0];
/// let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
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
pub fn index_gen<T>(input: &Array<T>, indices: Indexer) -> Array<T>
where
    T: HasAfEnum,
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_index_gen(
            &mut temp as MutAfArray,
            input.get() as AfArray,
            indices.len() as DimT,
            indices.get() as AfIndex,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Assign an Array to another after indexing it using any combination of Array's and Sequence's
///
/// # Examples
///
/// ```rust
/// use arrayfire::{Array, Dim4, Seq, print, randu, constant, Indexer, assign_gen};
/// let values: [f32; 3] = [1.0, 2.0, 3.0];
/// let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
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
pub fn assign_gen<T>(lhs: &Array<T>, indices: &Indexer, rhs: &Array<T>) -> Array<T>
where
    T: HasAfEnum,
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_assign_gen(
            &mut temp as MutAfArray,
            lhs.get() as AfArray,
            indices.len() as DimT,
            indices.get() as AfIndex,
            rhs.get() as AfArray,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

#[repr(C)]
struct SeqInternal {
    begin: c_double,
    end: c_double,
    step: c_double,
}

impl SeqInternal {
    fn from_seq<T: Copy>(s: &Seq<T>) -> Self
    where
        c_double: From<T>,
    {
        SeqInternal {
            begin: From::from(s.begin()),
            end: From::from(s.end()),
            step: From::from(s.step()),
        }
    }
}
