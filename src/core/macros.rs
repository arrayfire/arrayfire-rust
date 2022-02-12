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
///
/// Examples on how to use this macro are provided in the [tutorials book][1]
///
/// [1]: http://arrayfire.org/arrayfire-rust/book/indexing.html
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
             for _d in seq_vec.len()..$array_ident.dims().ndims() {
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
            let mut idxrs = $crate::Indexer::default();
            view!(@set_indexer 0, idxrs, $($_e),*);
            $crate::index_gen(&$array_ident, idxrs)
        }
    };
}

/// Macro to evaluate individual Arrays or assignment operations
///
/// - Evaluate on one or more Array identifiers: essentially calls [Array::eval][4] on each of those
///   Array objects individually.
///
///   ```rust
///   use arrayfire::{dim4, eval, randu};
///   let dims = dim4!(5, 5);
///   let a = randu::<f32>(dims);
///   let b = a.clone();
///   let c = a.clone();
///   let d = a.clone();
///   let x = a - b;
///   let y = c * d;
///   eval!(&x, &y);
///   ```
///
/// - Evaluate assignment operations: This is essentially syntactic sugar for modifying portions of
///   Array with another Array using a combination of [Sequences][1] and/or [Array][2] objects.
///   Full examples for this use case are provided in the [tutorials book][3]
///
/// [1]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Seq.html
/// [2]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html
/// [3]: http://arrayfire.org/arrayfire-rust/book/indexing.html
/// [4]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.eval
#[macro_export]
macro_rules! eval {
    ( $l:ident [ $($lb:literal : $le:literal : $ls:literal),+ ] =
      $r:ident [ $($rb:literal : $re:literal : $rs:literal),+ ]) => {
        {
            #[allow(non_snake_case)]
            let AF_MAX_DIMS: usize = view!(@af_max_dims);
            let mut seq_vec = Vec::<$crate::Seq<i32>>::with_capacity(AF_MAX_DIMS);
            $(
                seq_vec.push($crate::seq!($lb:$le:$ls));
             )*
            let mut idxrs = $crate::Indexer::default();
            for i in 0..seq_vec.len() {
                idxrs.set_index(&seq_vec[i], i as u32, None);
            }
            let eq_rterm = $crate::view!($r[ $($rb:$re:$rs),+ ]);
            $crate::assign_gen(&mut $l, &idxrs, &eq_rterm);
        }
    };
    ( $l:ident [ $($lb:literal : $le:literal : $ls:literal),+ ] = $r:expr ) => {
        {
            #[allow(non_snake_case)]
            let AF_MAX_DIMS: usize = view!(@af_max_dims);
            let mut seq_vec = Vec::<$crate::Seq<i32>>::with_capacity(AF_MAX_DIMS);
            $(
                seq_vec.push($crate::seq!($lb:$le:$ls));
             )*
            let mut idxrs = $crate::Indexer::default();
            for i in 0..seq_vec.len() {
                idxrs.set_index(&seq_vec[i], i as u32, None);
            }
            $crate::assign_gen(&mut $l, &idxrs, &$r);
        }
    };
    ($lhs:ident [ $($lhs_e:expr),+ ] = $rhs:ident [ $($rhs_e:expr),+ ]) => {
        {
            let eq_rterm = $crate::view!($rhs[ $($rhs_e),+ ]);
            let mut idxrs = $crate::Indexer::default();
            view!(@set_indexer 0, idxrs, $($lhs_e),*);
            $crate::assign_gen(&mut $lhs, &idxrs, &eq_rterm);
        }
    };
    ($lhs:ident [ $($lhs_e:expr),+ ] = $rhs:expr) => {
        {
            let mut idxrs = $crate::Indexer::default();
            view!(@set_indexer 0, idxrs, $($lhs_e),*);
            $crate::assign_gen(&mut $lhs, &idxrs, &$rhs);
        }
    };
    [$($x:expr),+] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
             )*
            $crate::eval_multiple(temp_vec)
        }
    };
}

/// Create an array of given shape filled with a single value a.k.a constant array
///
/// # Examples
///
/// ```rust
/// # use arrayfire::{constant};
/// let _zeros_1d = constant!(0.0f32; 10);
/// let _ones_3d = constant!(1u32; 3, 3, 3);
///
/// let dim = 10;
/// let mix_shape = constant!(42.0f32; dim, 10);
/// ```
#[macro_export]
macro_rules! constant {
    ($value:expr; $($dim:expr),+) => {
        $crate::constant($value, $crate::dim4!($($dim),*))
    };
}

/// Create an array of given shape sampled from uniform distribution
///
/// If no type argument is specified, the data type defaults to 32 bit floats.
///
/// # Examples
///
/// ```rust
/// # use arrayfire::{randu};
/// let mat10x10 = randu!(10, 10);
/// ```
#[macro_export]
macro_rules! randu {
    ($($dim:expr),+) => { $crate::randu::<f32>($crate::dim4!($($dim),*)) };
    ($type:ty; $($dim:expr),+) => { $crate::randu::<$type>($crate::dim4!($($dim),*)) };
}

/// Create an array of given shape sampled from normal distribution
///
/// If no type argument is specified, the data type defaults to 32 bit floats.
///
/// # Examples
///
/// ```rust
/// # use arrayfire::{randn};
/// let mat10x10 = randn!(10, 10);
/// ```
#[macro_export]
macro_rules! randn {
    ($($dim:expr),+) => { $crate::randn::<f32>($crate::dim4!($($dim),*)) };
    ($type:ty; $($dim:expr),+) => { $crate::randn::<$type>($crate::dim4!($($dim),*)) };
}

#[cfg(test)]
mod tests {
    use super::super::array::Array;
    use super::super::data::constant;
    use super::super::device::set_device;
    use super::super::index::{index, rows, set_rows};
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
        set_device(0);
        let mut dim4d = dim4!(5, 3, 2, 1);
        dim4d[2] = 1;

        let a = randu::<f32>(dim4d);
        let seqs = &[seq!(1:3:1), seq!()];
        let _sub = index(&a, seqs);
    }

    #[test]
    fn seq_view2() {
        set_device(0);
        // ANCHOR: seq_view2
        let a = randu::<f32>(dim4!(5, 5));
        let _sub = view!(a[1:3:1, 1:1:0]); // 1:1:0 means all elements along axis

        // ANCHOR_END: seq_view2
    }

    #[test]
    fn view_macro() {
        set_device(0);
        let dims = dim4!(5, 5, 2, 1);
        let a = randu::<f32>(dims);
        let b = a.clone();
        let c = a.clone();
        let d = a.clone();
        let e = a.clone();

        let _v = view!(a);

        let _m = view!(c[1:3:1, 1:3:2]);

        let x = seq!(1:3:1);
        let y = seq!(1:3:2);
        let _u = view!(b[x, y]);

        let values: [u32; 3] = [1, 2, 3];
        let indices = Array::new(&values, dim4!(3, 1, 1, 1));
        let indices2 = Array::new(&values, dim4!(3, 1, 1, 1));

        let _w = view!(d[indices, indices2]);

        let _z = view!(e[indices, y]);
    }

    #[test]
    fn eval_assign_seq_indexed_array() {
        set_device(0);
        let dims = dim4!(5, 5);
        let mut a = randu::<f32>(dims);
        //print(&a);
        //[5 5 1 1]
        //    0.6010     0.5497     0.1583     0.3636     0.6755
        //    0.0278     0.2864     0.3712     0.4165     0.6105
        //    0.9806     0.3410     0.3543     0.5814     0.5232
        //    0.2126     0.7509     0.6450     0.8962     0.5567
        //    0.0655     0.4105     0.9675     0.3712     0.7896

        let b = randu::<f32>(dims);
        //print(&b);
        //[5 5 1 1]
        //    0.8966     0.5143     0.0123     0.7917     0.2522
        //    0.0536     0.3670     0.3988     0.1654     0.9644
        //    0.5775     0.3336     0.9787     0.8657     0.4711
        //    0.2908     0.0363     0.2308     0.3766     0.3637
        //    0.9941     0.5349     0.6244     0.7331     0.9643

        let d0 = seq!(1:2:1);
        let d1 = seq!(1:2:1);
        let s0 = seq!(1:2:1);
        let s1 = seq!(1:2:1);
        eval!(a[d0, d1] = b[s0, s1]);
        //print(&a);
        //[5 5 1 1]
        //    0.6010     0.5497     0.1583     0.3636     0.6755
        //    0.0278     0.3670     0.3988     0.4165     0.6105
        //    0.9806     0.3336     0.9787     0.5814     0.5232
        //    0.2126     0.7509     0.6450     0.8962     0.5567
        //    0.0655     0.4105     0.9675     0.3712     0.7896
    }

    #[test]
    fn eval_assign_array_to_seqd_array() {
        set_device(0);
        // ANCHOR: macro_seq_assign
        let mut a = randu::<f32>(dim4!(5, 5));
        let b = randu::<f32>(dim4!(2, 2));
        eval!(a[1:2:1, 1:2:1] = b);
        // ANCHOR_END: macro_seq_assign
    }

    #[test]
    fn macro_seq_array_assign() {
        set_device(0);
        // ANCHOR: macro_seq_array_assign
        let values: [f32; 3] = [1.0, 2.0, 3.0];
        let indices = Array::new(&values, dim4!(3));
        let seq4gen = seq!(0:2:1);
        let mut a = randu::<f32>(dim4!(5, 3));

        let b = constant(2.0_f32, dim4!(3, 3));

        eval!(a[indices, seq4gen] = b);
        // ANCHOR_END: macro_seq_array_assign
    }

    #[test]
    fn constant_macro() {
        set_device(0);
        let _zeros_1d = constant!(0.0f32; 10);
        let _zeros_2d = constant!(0.0f64; 5, 5);
        let _ones_3d = constant!(1u32; 3, 3, 3);
        let _twos_4d = constant!(2u16; 2, 2, 2, 2);

        let dim = 10;
        let _mix_shape = constant!(42.0f32; dim, 10);
    }

    #[test]
    fn rand_macro() {
        set_device(0);
        let _ru5x5 = randu!(5, 5);
        let _rn5x5 = randn!(5, 5);
        let _ruu32_5x5 = randu!(u32; 5, 5);
        let _ruu8_5x5 = randu!(u8; 5, 5);
    }

    #[test]
    fn match_eval_macro_with_set_rows() {
        set_device(0);

        let inpt = vec![true, true, true, true, true, true, true, true, true, true];
        let gold = vec![
            true, true, false, false, true, true, true, false, false, true,
        ];

        let mut orig_arr = Array::new(&inpt, dim4!(5, 2));
        let mut orig_cln = orig_arr.clone();

        let new_vals = vec![false, false, false, false];
        let new_arr = Array::new(&new_vals, dim4!(2, 2));

        eval!( orig_arr[2:3:1,1:1:0] = new_arr );
        let mut res1 = vec![true; orig_arr.elements()];
        orig_arr.host(&mut res1);

        set_rows(&mut orig_cln, &new_arr, 2, 3);
        let mut res2 = vec![true; orig_cln.elements()];
        orig_cln.host(&mut res2);

        assert_eq!(gold, res1);
        assert_eq!(res1, res2);
    }

    #[test]
    fn match_view_macro_with_get_rows() {
        set_device(0);

        let inpt: Vec<i32> = (0..10).collect();
        let gold: Vec<i32> = vec![2, 3, 7, 8];

        println!("input {:?}", inpt);
        println!("gold {:?}", gold);

        let orig_arr = Array::new(&inpt, dim4!(5, 2));

        let view_out = view!( orig_arr[2:3:1] );
        let mut res1 = vec![0i32; view_out.elements()];
        view_out.host(&mut res1);

        let rows_out = rows(&orig_arr, 2, 3);
        let mut res2 = vec![0i32; rows_out.elements()];
        rows_out.host(&mut res2);

        assert_eq!(gold, res1);
        assert_eq!(res1, res2);
    }
}
