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
            let AF_MAX_DIMS: usize = view!(@af_max_dims);
            let mut seq_vec = Vec::<$crate::Seq<i32>>::with_capacity(AF_MAX_DIMS);
            $(
                seq_vec.push($crate::seq!($start:$end:$step));
             )*
             for span_place_holder in seq_vec.len()..AF_MAX_DIMS {
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
            let AF_MAX_DIMS: u32 = view!(@af_max_dims);
            let span = $crate::seq!();
            let mut idxrs = $crate::Indexer::default();

            view!(@set_indexer 0, idxrs, $($_e),*);

            let mut dimIx = idxrs.len() as u32;
            while dimIx < AF_MAX_DIMS {
                idxrs.set_index(&span, dimIx, None);
                dimIx += 1;
            }
            $crate::index_gen(&$array_ident, idxrs)
        }
    };
}
