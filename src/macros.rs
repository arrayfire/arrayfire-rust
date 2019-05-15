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
            let (abytes, abuffs, lbytes, lbuffs) = device_mem_info();
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
            join_many($dim, temp_vec)
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
            print_gen(String::from($msg), &$x, Some(4));
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
