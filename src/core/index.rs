use super::array::Array;
use super::defines::AfError;
use super::error::HANDLE_ERROR;
use super::seq::Seq;
use super::util::{af_array, af_index_t, dim_t, HasAfEnum, IndexableType};

use libc::{c_double, c_int, c_uint};
use std::default::Default;
use std::marker::PhantomData;
use std::mem;

extern "C" {
    fn af_create_indexers(indexers: *mut af_index_t) -> c_int;
    fn af_set_array_indexer(indexer: af_index_t, idx: af_array, dim: dim_t) -> c_int;
    fn af_set_seq_indexer(
        indexer: af_index_t,
        idx: *const SeqInternal,
        dim: dim_t,
        is_batch: bool,
    ) -> c_int;
    fn af_release_indexers(indexers: af_index_t) -> c_int;

    fn af_index(
        out: *mut af_array,
        input: af_array,
        ndims: c_uint,
        index: *const SeqInternal,
    ) -> c_int;
    fn af_lookup(out: *mut af_array, arr: af_array, indices: af_array, dim: c_uint) -> c_int;
    fn af_assign_seq(
        out: *mut af_array,
        lhs: af_array,
        ndims: c_uint,
        indices: *const SeqInternal,
        rhs: af_array,
    ) -> c_int;
    fn af_index_gen(
        out: *mut af_array,
        input: af_array,
        ndims: dim_t,
        indices: af_index_t,
    ) -> c_int;
    fn af_assign_gen(
        out: *mut af_array,
        lhs: af_array,
        ndims: dim_t,
        indices: af_index_t,
        rhs: af_array,
    ) -> c_int;
}

/// Struct to manage an array of resources of type `af_indexer_t`(ArrayFire C struct)
///
/// ## Sharing Across Threads
///
/// While sharing an Indexer object with other threads, just move it across threads. At the
/// moment, one cannot share borrowed references across threads.
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
/// let mut idxr = Indexer::default();
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
    handle: af_index_t,
    count: usize,
    marker: PhantomData<&'object ()>,
}

unsafe impl<'object> Send for Indexer<'object> {}

/// Trait bound indicating indexability
///
/// Any object to be able to be passed on to [Indexer::set_index()](./struct.Indexer.html#method.set_index) method  should implement this trait with appropriate implementation of `set` method.
pub trait Indexable {
    /// Set indexing object for a given dimension
    ///
    /// `is_batch` parameter is not used in most cases as it has been provided in
    /// ArrayFire C-API to enable GFOR construct in ArrayFire C++ API. This type
    /// of construct/idea is not exposed in rust wrapper yet. So, the user would
    /// just need to pass `None` to this parameter while calling this function.
    /// Since we can't have default default values and we wanted to keep this
    /// parameter for future use cases, we just made it an `std::Option`.
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
impl<T> Indexable for Array<T>
where
    T: HasAfEnum + IndexableType,
{
    fn set(&self, idxr: &mut Indexer, dim: u32, _is_batch: Option<bool>) {
        unsafe {
            let err_val = af_set_array_indexer(idxr.get(), self.get(), dim as dim_t);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

/// Enables [Seq](./struct.Seq.html) to be used to index another Array
///
/// This is used in functions [index_gen](./fn.index_gen.html) and
/// [assign_gen](./fn.assign_gen.html)
impl<T> Indexable for Seq<T>
where
    c_double: From<T>,
    T: Copy + IndexableType,
{
    fn set(&self, idxr: &mut Indexer, dim: u32, is_batch: Option<bool>) {
        unsafe {
            let err_val = af_set_seq_indexer(
                idxr.get(),
                &SeqInternal::from_seq(self) as *const SeqInternal,
                dim as dim_t,
                is_batch.unwrap_or(false),
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

impl<'object> Default for Indexer<'object> {
    fn default() -> Self {
        unsafe {
            let mut temp: af_index_t = std::ptr::null_mut();
            let err_val = af_create_indexers(&mut temp as *mut af_index_t);
            HANDLE_ERROR(AfError::from(err_val));
            Self {
                handle: temp,
                count: 0,
                marker: PhantomData,
            }
        }
    }
}

impl<'object> Indexer<'object> {
    /// Create a new Indexer object and set the dimension specific index objects later
    #[deprecated(since = "3.7.0", note = "Use Indexer::default() instead")]
    pub fn new() -> Self {
        unsafe {
            let mut temp: af_index_t = std::ptr::null_mut();
            let err_val = af_create_indexers(&mut temp as *mut af_index_t);
            HANDLE_ERROR(AfError::from(err_val));
            Self {
                handle: temp,
                count: 0,
                marker: PhantomData,
            }
        }
    }

    /// Set either [Array](./struct.Array.html) or [Seq](./struct.Seq.html) to index an Array along `idx` dimension
    pub fn set_index<'s, T>(&'s mut self, idx: &'object T, dim: u32, is_batch: Option<bool>)
    where
        T: Indexable + 'object,
    {
        idx.set(self, dim, is_batch);
        self.count += 1;
    }

    /// Get number of indexing objects set
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if any indexing objects are set
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Get native(ArrayFire) resource handle
    unsafe fn get(&self) -> af_index_t {
        self.handle
    }
}

impl<'object> Drop for Indexer<'object> {
    fn drop(&mut self) {
        unsafe {
            let ret_val = af_release_indexers(self.handle as af_index_t);
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
pub fn index<IO, T>(input: &Array<IO>, seqs: &[Seq<T>]) -> Array<IO>
where
    c_double: From<T>,
    IO: HasAfEnum,
    T: Copy + HasAfEnum + IndexableType,
{
    let seqs: Vec<SeqInternal> = seqs.iter().map(|s| SeqInternal::from_seq(s)).collect();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_index(
            &mut temp as *mut af_array,
            input.get(),
            seqs.len() as u32,
            seqs.as_ptr() as *const SeqInternal,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
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
pub fn row<T>(input: &Array<T>, row_num: i64) -> Array<T>
where
    T: HasAfEnum,
{
    let mut seqs = vec![Seq::new(row_num as f64, row_num as f64, 1.0)];
    for _d in 1..input.dims().ndims() {
        seqs.push(Seq::default());
    }
    index(input, &seqs)
}

/// Set `row_num`^th row in `inout` Array to a new Array `new_row`
pub fn set_row<T>(inout: &mut Array<T>, new_row: &Array<T>, row_num: i64)
where
    T: HasAfEnum,
{
    let mut seqs = vec![Seq::new(row_num as f64, row_num as f64, 1.0)];
    for _d in 1..inout.dims().ndims() {
        seqs.push(Seq::default());
    }
    assign_seq(inout, &seqs, new_row)
}

/// Get an Array with all rows from `first` to `last` in the `input` Array
pub fn rows<T>(input: &Array<T>, first: i64, last: i64) -> Array<T>
where
    T: HasAfEnum,
{
    let step: f64 = if first > last && last < 0 { -1.0 } else { 1.0 };
    let mut seqs = vec![Seq::new(first as f64, last as f64, step)];
    for _d in 1..input.dims().ndims() {
        seqs.push(Seq::default());
    }
    index(input, &seqs)
}

/// Set rows from `first` to `last` in `inout` Array with rows from Array `new_rows`
pub fn set_rows<T>(inout: &mut Array<T>, new_rows: &Array<T>, first: i64, last: i64)
where
    T: HasAfEnum,
{
    let step: f64 = if first > last && last < 0 { -1.0 } else { 1.0 };
    let mut seqs = vec![Seq::new(first as f64, last as f64, step)];
    for _d in 1..inout.dims().ndims() {
        seqs.push(Seq::default());
    }
    assign_seq(inout, &seqs, new_rows)
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
pub fn col<T>(input: &Array<T>, col_num: i64) -> Array<T>
where
    T: HasAfEnum,
{
    let mut seqs = vec![
        Seq::default(),
        Seq::new(col_num as f64, col_num as f64, 1.0),
    ];
    for _d in 2..input.dims().ndims() {
        seqs.push(Seq::default());
    }
    index(input, &seqs)
}

/// Set `col_num`^th col in `inout` Array to a new Array `new_col`
pub fn set_col<T>(inout: &mut Array<T>, new_col: &Array<T>, col_num: i64)
where
    T: HasAfEnum,
{
    let mut seqs = vec![
        Seq::default(),
        Seq::new(col_num as f64, col_num as f64, 1.0),
    ];
    for _d in 2..inout.dims().ndims() {
        seqs.push(Seq::default());
    }
    assign_seq(inout, &seqs, new_col)
}

/// Get all cols from `first` to `last` in the `input` Array
pub fn cols<T>(input: &Array<T>, first: i64, last: i64) -> Array<T>
where
    T: HasAfEnum,
{
    let step: f64 = if first > last && last < 0 { -1.0 } else { 1.0 };
    let mut seqs = vec![Seq::default(), Seq::new(first as f64, last as f64, step)];
    for _d in 2..input.dims().ndims() {
        seqs.push(Seq::default());
    }
    index(input, &seqs)
}

/// Set cols from `first` to `last` in `inout` Array with cols from Array `new_cols`
pub fn set_cols<T>(inout: &mut Array<T>, new_cols: &Array<T>, first: i64, last: i64)
where
    T: HasAfEnum,
{
    let step: f64 = if first > last && last < 0 { -1.0 } else { 1.0 };
    let mut seqs = vec![Seq::default(), Seq::new(first as f64, last as f64, step)];
    for _d in 2..inout.dims().ndims() {
        seqs.push(Seq::default());
    }
    assign_seq(inout, &seqs, new_cols)
}

/// Get `slice_num`^th slice from `input` Array
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn slice<T>(input: &Array<T>, slice_num: i64) -> Array<T>
where
    T: HasAfEnum,
{
    let mut seqs = vec![
        Seq::default(),
        Seq::default(),
        Seq::new(slice_num as f64, slice_num as f64, 1.0),
    ];
    for _d in 3..input.dims().ndims() {
        seqs.push(Seq::default());
    }
    index(input, &seqs)
}

/// Set slice `slice_num` in `inout` Array to a new Array `new_slice`
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn set_slice<T>(inout: &mut Array<T>, new_slice: &Array<T>, slice_num: i64)
where
    T: HasAfEnum,
{
    let mut seqs = vec![
        Seq::default(),
        Seq::default(),
        Seq::new(slice_num as f64, slice_num as f64, 1.0),
    ];
    for _d in 3..inout.dims().ndims() {
        seqs.push(Seq::default());
    }
    assign_seq(inout, &seqs, new_slice)
}

/// Get slices from `first` to `last` in `input` Array
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn slices<T>(input: &Array<T>, first: i64, last: i64) -> Array<T>
where
    T: HasAfEnum,
{
    let step: f64 = if first > last && last < 0 { -1.0 } else { 1.0 };
    let mut seqs = vec![
        Seq::default(),
        Seq::default(),
        Seq::new(first as f64, last as f64, step),
    ];
    for _d in 3..input.dims().ndims() {
        seqs.push(Seq::default());
    }
    index(input, &seqs)
}

/// Set `first` to `last` slices of `inout` Array to a new Array `new_slices`
///
/// Slices indicate that the indexing is along 3rd dimension
pub fn set_slices<T>(inout: &mut Array<T>, new_slices: &Array<T>, first: i64, last: i64)
where
    T: HasAfEnum,
{
    let step: f64 = if first > last && last < 0 { -1.0 } else { 1.0 };
    let mut seqs = vec![
        Seq::default(),
        Seq::default(),
        Seq::new(first as f64, last as f64, step),
    ];
    for _d in 3..inout.dims().ndims() {
        seqs.push(Seq::default());
    }
    assign_seq(inout, &seqs, new_slices)
}

/// Lookup(hash) an Array using another Array
///
/// Given a dimension `seq_dim`, `indices` are lookedup in `input` and returned as a new
/// Array if found
pub fn lookup<T, I>(input: &Array<T>, indices: &Array<I>, seq_dim: i32) -> Array<T>
where
    T: HasAfEnum,
    I: HasAfEnum + IndexableType,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_lookup(
            &mut temp as *mut af_array,
            input.get() as af_array,
            indices.get() as af_array,
            seq_dim as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Assign(copy) content of an Array to another Array indexed by Sequences
///
/// Assign `rhs` to `lhs` after indexing `lhs`
///
/// # Examples
///
/// ```rust
/// use arrayfire::{constant, Dim4, Seq, assign_seq, print};
/// let mut a = constant(2.0 as f32, Dim4::new(&[5, 3, 1, 1]));
/// print(&a);
/// // 2.0 2.0 2.0
/// // 2.0 2.0 2.0
/// // 2.0 2.0 2.0
/// // 2.0 2.0 2.0
/// // 2.0 2.0 2.0
///
/// let b    = constant(1.0 as f32, Dim4::new(&[3, 3, 1, 1]));
/// let seqs = &[Seq::new(1.0, 3.0, 1.0), Seq::default()];
/// assign_seq(&mut a, seqs, &b);
///
/// print(&a);
/// // 2.0 2.0 2.0
/// // 1.0 1.0 1.0
/// // 1.0 1.0 1.0
/// // 1.0 1.0 1.0
/// // 2.0 2.0 2.0
/// ```
pub fn assign_seq<T, I>(lhs: &mut Array<I>, seqs: &[Seq<T>], rhs: &Array<I>)
where
    c_double: From<T>,
    I: HasAfEnum,
    T: Copy + IndexableType,
{
    let seqs: Vec<SeqInternal> = seqs.iter().map(|s| SeqInternal::from_seq(s)).collect();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_assign_seq(
            &mut temp as *mut af_array,
            lhs.get() as af_array,
            seqs.len() as c_uint,
            seqs.as_ptr() as *const SeqInternal,
            rhs.get() as af_array,
        );
        HANDLE_ERROR(AfError::from(err_val));

        let modified = temp.into();
        let _old_arr = mem::replace(lhs, modified);
    }
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
/// let mut idxrs = Indexer::default();
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
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_index_gen(
            &mut temp as *mut af_array,
            input.get() as af_array,
            indices.len() as dim_t,
            indices.get() as af_index_t,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
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
/// let mut a = randu::<f32>(Dim4::new(&[5, 3, 1, 1]));
/// // [5 3 1 1]
/// //     0.0000     0.2190     0.3835
/// //     0.1315     0.0470     0.5194
/// //     0.7556     0.6789     0.8310
/// //     0.4587     0.6793     0.0346
/// //     0.5328     0.9347     0.0535
///
/// let b    = constant(2.0 as f32, Dim4::new(&[3, 3, 1, 1]));
///
/// let mut idxrs = Indexer::default();
/// idxrs.set_index(&indices, 0, None); // 2nd parameter is indexing dimension
/// idxrs.set_index(&seq4gen, 1, Some(false)); // 3rd parameter indicates batch operation
///
/// assign_gen(&mut a, &idxrs, &b);
/// println!("a(indices, seq(0, 2, 1))"); print(&a);
/// // [5 3 1 1]
/// //     0.0000     0.2190     0.3835
/// //     2.0000     2.0000     2.0000
/// //     2.0000     2.0000     2.0000
/// //     2.0000     2.0000     2.0000
/// //     0.5328     0.9347     0.0535
/// ```
pub fn assign_gen<T>(lhs: &mut Array<T>, indices: &Indexer, rhs: &Array<T>)
where
    T: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_assign_gen(
            &mut temp as *mut af_array,
            lhs.get() as af_array,
            indices.len() as dim_t,
            indices.get() as af_index_t,
            rhs.get() as af_array,
        );
        HANDLE_ERROR(AfError::from(err_val));

        let modified = temp.into();
        let _old_arr = mem::replace(lhs, modified);
    }
}

#[repr(C)]
struct SeqInternal {
    begin: c_double,
    end: c_double,
    step: c_double,
}

impl SeqInternal {
    fn from_seq<T>(s: &Seq<T>) -> Self
    where
        c_double: From<T>,
        T: Copy + IndexableType,
    {
        Self {
            begin: From::from(s.begin()),
            end: From::from(s.end()),
            step: From::from(s.step()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::array::Array;
    use super::super::data::constant;
    use super::super::device::set_device;
    use super::super::dim4::Dim4;
    use super::super::index::{assign_gen, assign_seq, col, index, index_gen, row, Indexer};
    use super::super::index::{cols, rows, set_row, set_rows};
    use super::super::random::randu;
    use super::super::seq::Seq;

    use crate::{dim4, seq, view};

    #[test]
    fn non_macro_seq_index() {
        set_device(0);
        // ANCHOR: non_macro_seq_index
        let dims = Dim4::new(&[5, 5, 1, 1]);
        let a = randu::<f32>(dims);
        //af_print!("a", a);
        //a
        //[5 5 1 1]
        //    0.3990     0.5160     0.8831     0.9107     0.6688
        //    0.6720     0.3932     0.0621     0.9159     0.8434
        //    0.5339     0.2706     0.7089     0.0231     0.1328
        //    0.1386     0.9455     0.9434     0.2330     0.2657
        //    0.7353     0.1587     0.1227     0.2220     0.2299

        // Index array using sequences
        let seqs = &[Seq::new(1u32, 3, 1), Seq::default()];
        let _sub = index(&a, seqs);
        //af_print!("a(seq(1,3,1), span)", sub);
        // [3 5 1 1]
        //     0.6720     0.3932     0.0621     0.9159     0.8434
        //     0.5339     0.2706     0.7089     0.0231     0.1328
        //     0.1386     0.9455     0.9434     0.2330     0.2657
        // ANCHOR_END: non_macro_seq_index
    }

    #[test]
    fn seq_index() {
        set_device(0);
        // ANCHOR: seq_index
        let dims = dim4!(5, 5, 1, 1);
        let a = randu::<f32>(dims);
        let first3 = seq!(1:3:1);
        let allindim2 = seq!();
        let _sub = view!(a[first3, allindim2]);
        // ANCHOR_END: seq_index
    }

    #[test]
    fn non_macro_seq_assign() {
        set_device(0);
        // ANCHOR: non_macro_seq_assign
        let mut a = constant(2.0_f32, dim4!(5, 3));
        //print(&a);
        // 2.0 2.0 2.0
        // 2.0 2.0 2.0
        // 2.0 2.0 2.0
        // 2.0 2.0 2.0
        // 2.0 2.0 2.0

        let b = constant(1.0_f32, dim4!(3, 3));
        let seqs = [seq!(1:3:1), seq!()];
        assign_seq(&mut a, &seqs, &b);
        //print(&a);
        // 2.0 2.0 2.0
        // 1.0 1.0 1.0
        // 1.0 1.0 1.0
        // 1.0 1.0 1.0
        // 2.0 2.0 2.0
        // ANCHOR_END: non_macro_seq_assign
    }

    #[test]
    fn non_macro_seq_array_index() {
        set_device(0);
        // ANCHOR: non_macro_seq_array_index
        let values: [f32; 3] = [1.0, 2.0, 3.0];
        let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
        let seq4gen = Seq::new(0.0, 2.0, 1.0);
        let a = randu::<f32>(Dim4::new(&[5, 3, 1, 1]));
        // [5 3 1 1]
        //     0.0000     0.2190     0.3835
        //     0.1315     0.0470     0.5194
        //     0.7556     0.6789     0.8310
        //     0.4587     0.6793     0.0346
        //     0.5328     0.9347     0.0535

        let mut idxrs = Indexer::default();
        idxrs.set_index(&indices, 0, None); // 2nd arg is indexing dimension
        idxrs.set_index(&seq4gen, 1, Some(false)); // 3rd arg indicates batch operation

        let _sub2 = index_gen(&a, idxrs);
        //println!("a(indices, seq(0, 2, 1))"); print(&sub2);
        // [3 3 1 1]
        //     0.1315     0.0470     0.5194
        //     0.7556     0.6789     0.8310
        //     0.4587     0.6793     0.0346
        // ANCHOR_END: non_macro_seq_array_index
    }

    #[test]
    fn seq_array_index() {
        set_device(0);
        // ANCHOR: seq_array_index
        let values: [f32; 3] = [1.0, 2.0, 3.0];
        let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
        let seq4gen = seq!(0:2:1);
        let a = randu::<f32>(Dim4::new(&[5, 3, 1, 1]));
        let _sub2 = view!(a[indices, seq4gen]);
        // ANCHOR_END: seq_array_index
    }

    #[test]
    fn non_macro_seq_array_assign() {
        set_device(0);
        // ANCHOR: non_macro_seq_array_assign
        let values: [f32; 3] = [1.0, 2.0, 3.0];
        let indices = Array::new(&values, dim4!(3, 1, 1, 1));
        let seq4gen = seq!(0:2:1);
        let mut a = randu::<f32>(dim4!(5, 3, 1, 1));
        // [5 3 1 1]
        //     0.0000     0.2190     0.3835
        //     0.1315     0.0470     0.5194
        //     0.7556     0.6789     0.8310
        //     0.4587     0.6793     0.0346
        //     0.5328     0.9347     0.0535

        let b = constant(2.0_f32, dim4!(3, 3, 1, 1));

        let mut idxrs = Indexer::default();
        idxrs.set_index(&indices, 0, None); // 2nd arg is indexing dimension
        idxrs.set_index(&seq4gen, 1, Some(false)); // 3rd arg indicates batch operation

        let _sub2 = assign_gen(&mut a, &idxrs, &b);
        //println!("a(indices, seq(0, 2, 1))"); print(&sub2);
        // [5 3 1 1]
        //     0.0000     0.2190     0.3835
        //     2.0000     2.0000     2.0000
        //     2.0000     2.0000     2.0000
        //     2.0000     2.0000     2.0000
        //     0.5328     0.9347     0.0535
        // ANCHOR_END: non_macro_seq_array_assign
    }

    #[test]
    fn setrow() {
        set_device(0);
        // ANCHOR: setrow
        let a = randu::<f32>(dim4!(5, 5, 1, 1));
        //print(&a);
        // [5 5 1 1]
        //     0.6010     0.5497     0.1583     0.3636     0.6755
        //     0.0278     0.2864     0.3712     0.4165     0.6105
        //     0.9806     0.3410     0.3543     0.5814     0.5232
        //     0.2126     0.7509     0.6450     0.8962     0.5567
        //     0.0655     0.4105     0.9675     0.3712     0.7896
        let _r = row(&a, 4);
        // [1 5 1 1]
        //     0.0655     0.4105     0.9675     0.3712     0.7896
        let _c = col(&a, 4);
        // [5 1 1 1]
        //     0.6755
        //     0.6105
        //     0.5232
        //     0.5567
        //     0.7896
        // ANCHOR_END: setrow
    }

    #[test]
    fn get_row() {
        set_device(0);
        // ANCHOR: get_row
        let a = randu::<f32>(dim4!(5, 5));
        // [5 5 1 1]
        //     0.6010     0.5497     0.1583     0.3636     0.6755
        //     0.0278     0.2864     0.3712     0.4165     0.6105
        //     0.9806     0.3410     0.3543     0.5814     0.5232
        //     0.2126     0.7509     0.6450     0.8962     0.5567
        //     0.0655     0.4105     0.9675     0.3712     0.7896
        let _r = row(&a, -1);
        // [1 5 1 1]
        //     0.0655     0.4105     0.9675     0.3712     0.7896
        let _c = col(&a, -1);
        // [5 1 1 1]
        //     0.6755
        //     0.6105
        //     0.5232
        //     0.5567
        //     0.7896
        // ANCHOR_END: get_row
    }

    #[test]
    fn get_rows() {
        set_device(0);
        // ANCHOR: get_rows
        let a = randu::<f32>(dim4!(5, 5));
        // [5 5 1 1]
        //     0.6010     0.5497     0.1583     0.3636     0.6755
        //     0.0278     0.2864     0.3712     0.4165     0.6105
        //     0.9806     0.3410     0.3543     0.5814     0.5232
        //     0.2126     0.7509     0.6450     0.8962     0.5567
        //     0.0655     0.4105     0.9675     0.3712     0.7896
        let _r = rows(&a, -1, -2);
        // [2 5 1 1]
        //     0.2126     0.7509     0.6450     0.8962     0.5567
        //     0.0655     0.4105     0.9675     0.3712     0.7896
        let _c = cols(&a, -1, -3);
        // [5 3 1 1]
        //     0.1583     0.3636     0.6755
        //     0.3712     0.4165     0.6105
        //     0.3543     0.5814     0.5232
        //     0.6450     0.8962     0.5567
        //     0.9675     0.3712     0.7896
        // ANCHOR_END: get_rows
    }

    #[test]
    fn change_row() {
        set_device(0);

        let v0: Vec<bool> = vec![true, true, true, true, true, true];
        let mut a0 = Array::new(&v0, dim4!(v0.len() as u64));

        let v1: Vec<bool> = vec![false];
        let a1 = Array::new(&v1, dim4!(v1.len() as u64));

        set_row(&mut a0, &a1, 2);

        let mut res = vec![true; a0.elements()];
        a0.host(&mut res);

        let gold = vec![true, true, false, true, true, true];

        assert_eq!(gold, res);
    }

    #[test]
    fn change_rows() {
        set_device(0);

        let v0: Vec<bool> = vec![true, true, true, true, true, true];
        let mut a0 = Array::new(&v0, dim4!(v0.len() as u64));

        let v1: Vec<bool> = vec![false, false];
        let a1 = Array::new(&v1, dim4!(v1.len() as u64));

        set_rows(&mut a0, &a1, 2, 3);

        let mut res = vec![true; a0.elements()];
        a0.host(&mut res);

        let gold = vec![true, true, false, false, true, true];

        assert_eq!(gold, res);
    }
}
