/// Macro to print the current stats of ArrayFire's memory manager.
///
/// `mem_info!` print 4 values:
///
///  Name                    | Description
/// -------------------------|-------------------------
///  Allocated Bytes         | Total number of bytes allocated by the memory manager
///  Allocated Buffers       | Total number of buffers allocated
///  Locked (In Use) Bytes   | Number of bytes that are in use by active arrays
///  Locked (In Use) Buffers | Number of buffers that are in use by active arrays
///
///  The `Allocated Bytes` is always a multiple of the memory step size. The
///  default step size is 1024 bytes. This means when a buffer is to be
///  allocated, the size is always rounded up to a multiple of the step size.
///  You can use [get_mem_step_size](./fn.get_mem_step_size.html) to check the
///  current step size and [set_mem_step_size](./fn.set_mem_step_size.html) to
///  set a custom resolution size.
///
///  The `Allocated Buffers` is the number of buffers that use up the allocated
///  bytes. This includes buffers currently in scope, as well as buffers marked
///  as free, ie, from arrays gone out of scope. The free buffers are available
///  for use by new arrays that might be created.
///
///  The `Locked Bytes` is the number of bytes in use that cannot be
///  reallocated at the moment. The difference of Allocated Bytes and Locked
///  Bytes is the total bytes available for reallocation.
///
///  The `Locked Buffers` is the number of buffer in use that cannot be
///  reallocated at the moment. The difference of Allocated Buffers and Locked
///  Buffers is the number of buffers available for reallocation.
///
/// # Parameters
///
/// - `msg` is the message that is printed to screen before printing stats
///
/// # Examples
///
/// ```rust
/// use arrayfire::{Dim4, device_mem_info, print, randu, mem_info};
///
/// let dims = Dim4::new(&[5, 5, 1, 1]);
/// let a = randu::<f32>(dims);
/// print(&a);
/// mem_info!("Hello!");
/// ```
///
/// Sample Output:
///
/// ```text
/// AF Memory: Here
/// Allocated [ Bytes | Buffers ] = [ 4096 | 4 ]
/// In Use    [ Bytes | Buffers ] = [ 2048 | 2 ]
/// ```
#[macro_export]
macro_rules! mem_info {
    [$msg: expr] => {
        {
            let (abytes, abuffs, lbytes, lbuffs) = $crate::device_mem_info();
            println!("AF Memory: {:?}", $msg);
            println!("Allocated [Bytes | Buffers] = [ {} | {} ]", abytes, abuffs);
            println!("In Use    [Bytes | Buffers] = [ {} | {} ]", lbytes, lbuffs);
        }
    };
}

/// Join multiple Arrays along a given dimension
///
/// All the Arrays provided to this macro should be of type `&Array`
///
/// # Examples
///
/// ```rust
/// use arrayfire::{Dim4, join_many, print, randu};
///
/// let a = &randu::<f32>(Dim4::new(&[5, 3, 1, 1]));
/// let b = &randu::<f32>(Dim4::new(&[5, 3, 1, 1]));
/// let c = &randu::<f32>(Dim4::new(&[5, 3, 1, 1]));
/// let d = join_many![2; a, b, c];
/// print(&d);
/// ```
///
/// # Panics
///
/// This macro just calls [join_many](./fn.join_many.html) function after collecting all
/// the input arrays into a vector.
// Using macro to implement join many wrapper
#[macro_export]
macro_rules! join_many {
    [$dim: expr; $($x:expr),+] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
             )*
            $crate::join_many($dim, temp_vec)
        }
    };
}

/// Print given message before printing out the Array to standard output
///
/// # Examples
///
/// ```rust
/// use arrayfire::{Dim4, print_gen, randu, af_print};
/// let dims = Dim4::new(&[3, 1, 1, 1]);
/// let a = randu::<f32>(dims);
/// af_print!("Create a 5-by-3 matrix of random floats on the GPU", a);
/// ```
///
#[macro_export]
macro_rules! af_print {
    [$msg: expr, $x: expr] => {
        {
            $crate::print_gen(String::from($msg), &$x, Some(4));
        }
    };
}

/// Evaluate arbitrary number of arrays
#[macro_export]
macro_rules! eval {
    [$($x:expr),+] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
             )*
            eval_multiple(temp_vec)
        }
    };
}

/// Create a dim4 object from provided dimensions
///
/// The user can pass 1 or more sizes and the left over values will default to 1.
#[macro_export]
macro_rules! dim4 {
    ($dim0:expr) => {
        $crate::Dim4::new(&[$dim0, 1, 1, 1])
    };
    ($dim0:expr, $dim1:expr) => {
        $crate::Dim4::new(&[$dim0, $dim1, 1, 1])
    };
    ($dim0:expr, $dim1:expr, $dim2:expr) => {
        $crate::Dim4::new(&[$dim0, $dim1, $dim2, 1])
    };
    ($dim0:expr, $dim1:expr, $dim2:expr, $dim3:expr) => {
        $crate::Dim4::new(&[$dim0, $dim1, $dim2, $dim3])
    };
}

/// Create a sequence object
///
/// If type is not provided, then the Seq will default to i32 type
#[macro_export]
macro_rules! seq {
    () => {
        $crate::Seq::<i32>::default()
    };
    ($sty:ty; $start:literal : $end:literal : $step:literal) => {
        $crate::Seq::<$sty>::new($start, $end, $step)
    };
    ($start:literal : $end:literal : $step:literal) => {
        $crate::Seq::<i32>::new($start, $end, $step)
    };
    ($sty:ty; $start:expr , $end:expr , $step:expr) => {
        $crate::Seq::<$sty>::new($start, $end, $step)
    };
    ($start:expr , $end:expr , $step:expr) => {
        $crate::Seq::<i32>::new($start, $end, $step)
    };
}

/// Indexing into an existing Array
///
/// This macro call with return an Array that has a view of another Array. The Array returned due to
/// the indexing operation will follow copy-on-write semantics. The Array identifier taken by this
/// macro is passed to the relevant internal functions as a borrowed reference. Thus, this identifier
/// will be still available for futher use after the macro call.
///
/// The following types of inputs are matched by this macro.
///
/// - A simple Array identifier.
/// - An Array with slicing info for indexing.
/// - An Array with slicing info and other arrays used for indexing.
#[macro_export]
macro_rules! view {
    (@af_max_dims) => {
        4
    };
    ( $array_ident:ident ) => {
        $array_ident.clone()
    };
    ( $array_ident:ident [ $($start:literal : $end:literal : $step:literal),+ ] ) => {
        {
            #[allow(non_snake_case)]
            let AF_MAX_DIMS: usize = view!(@af_max_dims);
            let mut seq_vec = Vec::<$crate::Seq<i32>>::with_capacity(AF_MAX_DIMS);
            $(
                seq_vec.push($crate::seq!($start:$end:$step));
             )*
             for _span_place_holder in seq_vec.len()..AF_MAX_DIMS {
                 seq_vec.push($crate::seq!());
             }
            $crate::index(&$array_ident, &seq_vec)
        }
    };
    (@set_indexer $idim:expr, $idxr:ident, $lterm:expr) => {
        {
            $idxr.set_index(&$lterm, $idim, None);
        }
    };
    (@set_indexer $idim:expr, $idxr:ident, $hterm:expr, $($tterm:expr),*) => {
        {
            $idxr.set_index(&$hterm, $idim, None);
            view!(@set_indexer $idim + 1, $idxr, $($tterm),*);
        }
    };
    ($array_ident:ident [ $($_e:expr),+ ]) => {
        {
            #[allow(non_snake_case)]
            let AF_MAX_DIMS: u32 = view!(@af_max_dims);
            let span = $crate::seq!();
            let mut idxrs = $crate::Indexer::default();

            view!(@set_indexer 0, idxrs, $($_e),*);

            let mut dim_ix = idxrs.len() as u32;
            while dim_ix < AF_MAX_DIMS {
                idxrs.set_index(&span, dim_ix, None);
                dim_ix += 1;
            }
            $crate::index_gen(&$array_ident, idxrs)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::super::array::Array;
    use super::super::index::index;
    use super::super::random::randu;

    #[test]
    fn dim4_construction() {
        let dim1d = dim4!(2);
        let dim2d = dim4!(2, 3);
        let dim3d = dim4!(2, 3, 4);
        let dim4d = dim4!(2, 3, 4, 2);
        let _dimn = dim4!(dim1d[0], dim2d[1], dim3d[2], dim4d[3]);
    }

    #[test]
    fn seq_construction() {
        let default_seq = seq!();
        let _range_1_to_10_step_1 = seq!(0:9:1);
        let _range_1_to_10_step_1_2 = seq!(f32; 0.0:9.0:1.5);
        let _range_from_exprs = seq!(default_seq.begin(), default_seq.end(), default_seq.step());
        let _range_from_exprs2 = seq!(f32; default_seq.begin() as f32,
                 default_seq.end() as f32, default_seq.step() as f32);
    }

    #[test]
    fn seq_view() {
        let mut dim4d = dim4!(5, 3, 2, 1);
        dim4d[2] = 1;

        let a = randu::<f32>(dim4d);
        let seqs = &[seq!(1:3:1), seq!()];
        let sub = index(&a, seqs);
        af_print!("A", a);
        af_print!("Indexed A", sub);
    }

    #[test]
    fn view_macro() {
        let dims = dim4!(5, 5, 2, 1);
        let a = randu::<f32>(dims);
        let b = a.clone();
        let c = a.clone();
        let d = a.clone();
        let e = a.clone();

        let v = view!(a);
        af_print!("v = a[None]", v);

        let m = view!(c[1:3:1, 1:3:2]);
        af_print!("m = c[:, :]", m);

        let x = seq!(1:3:1);
        let y = seq!(1:3:2);
        let u = view!(b[x, y]);
        af_print!("u = b[seq(), seq()]", u);

        let values: [u32; 3] = [1, 2, 3];
        let indices = Array::new(&values, dim4!(3, 1, 1, 1));
        let indices2 = Array::new(&values, dim4!(3, 1, 1, 1));

        let w = view!(d[indices, indices2]);
        af_print!("w = d[Array, Array]", w);

        let z = view!(e[indices, y]);
        af_print!("z = e[Array, Seq]", z);
    }
}
