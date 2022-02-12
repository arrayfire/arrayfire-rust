use super::defines::{AfError, Backend, DType};
use super::dim4::Dim4;
use super::error::HANDLE_ERROR;
use super::util::{af_array, dim_t, free_host, void_ptr, HasAfEnum};

use libc::{c_char, c_int, c_longlong, c_uint, c_void};
use std::ffi::{CStr, CString};
use std::fmt;
use std::marker::PhantomData;

// Some unused functions from array.h in C-API of ArrayFire
// af_copy_array
// af_write_array
// af_get_data_ref_count

extern "C" {
    fn af_create_array(
        out: *mut af_array,
        data: *const c_void,
        ndims: c_uint,
        dims: *const dim_t,
        aftype: c_uint,
    ) -> c_int;

    fn af_create_handle(
        out: *mut af_array,
        ndims: c_uint,
        dims: *const dim_t,
        aftype: c_uint,
    ) -> c_int;

    fn af_device_array(
        out: *mut af_array,
        data: *mut c_void,
        ndims: c_uint,
        dims: *const dim_t,
        aftype: c_uint,
    ) -> c_int;

    fn af_get_elements(out: *mut dim_t, arr: af_array) -> c_int;

    fn af_get_type(out: *mut c_uint, arr: af_array) -> c_int;

    fn af_get_dims(
        dim0: *mut c_longlong,
        dim1: *mut c_longlong,
        dim2: *mut c_longlong,
        dim3: *mut c_longlong,
        arr: af_array,
    ) -> c_int;

    fn af_get_numdims(result: *mut c_uint, arr: af_array) -> c_int;

    fn af_is_empty(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_scalar(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_row(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_column(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_vector(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_complex(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_real(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_double(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_single(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_half(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_integer(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_bool(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_realfloating(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_floating(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_linear(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_owner(result: *mut bool, arr: af_array) -> c_int;

    fn af_is_sparse(result: *mut bool, arr: af_array) -> c_int;

    fn af_get_data_ptr(data: *mut c_void, arr: af_array) -> c_int;

    fn af_eval(arr: af_array) -> c_int;

    fn af_eval_multiple(num: c_int, arrays: *const af_array) -> c_int;

    fn af_set_manual_eval_flag(flag: c_int) -> c_int;

    fn af_get_manual_eval_flag(flag: *mut c_int) -> c_int;

    fn af_retain_array(out: *mut af_array, arr: af_array) -> c_int;

    fn af_copy_array(out: *mut af_array, arr: af_array) -> c_int;

    fn af_release_array(arr: af_array) -> c_int;

    //fn af_print_array(arr: af_array) -> c_int;

    fn af_print_array_gen(exp: *const c_char, arr: af_array, precision: c_int) -> c_int;

    fn af_cast(out: *mut af_array, arr: af_array, aftype: c_uint) -> c_int;

    fn af_get_backend_id(backend: *mut c_uint, input: af_array) -> c_int;

    fn af_get_device_id(device: *mut c_int, input: af_array) -> c_int;

    fn af_create_strided_array(
        arr: *mut af_array,
        data: *const c_void,
        offset: dim_t,
        ndims: c_uint,
        dims: *const dim_t,
        strides: *const dim_t,
        aftype: c_uint,
        stype: c_uint,
    ) -> c_int;

    fn af_get_strides(
        s0: *mut dim_t,
        s1: *mut dim_t,
        s2: *mut dim_t,
        s3: *mut dim_t,
        arr: af_array,
    ) -> c_int;

    fn af_get_offset(offset: *mut dim_t, arr: af_array) -> c_int;

    fn af_lock_array(arr: af_array) -> c_int;

    fn af_unlock_array(arr: af_array) -> c_int;

    fn af_get_device_ptr(ptr: *mut void_ptr, arr: af_array) -> c_int;

    fn af_get_allocated_bytes(result: *mut usize, arr: af_array) -> c_int;

    fn af_array_to_string(
        ostr: *mut *mut c_char,
        exp: *const c_char,
        arr: af_array,
        precision: c_int,
        transpose: bool,
    ) -> c_int;
}

/// A multidimensional data container
///
/// Currently, `Array<T>` objects support data up to four dimensions.
///
/// All operations on arrays (including creation) require that `T: HasAfEnum`,
/// meaning that `T` must be one of the numerical datatypes supported by Arrayfire.
///
/// ## Sharing Across Threads
///
/// While sharing an Array with other threads, there is no need to wrap
/// this in an Arc object unless only one such object is required to exist.
/// The reason being that ArrayFire's internal Array is appropriately reference
/// counted in thread safe manner. However, if you need to modify Array object,
/// then please do wrap the object using a Mutex or Read-Write lock.
///
/// Examples on how to share Array across threads is illustrated in our
/// [book](http://arrayfire.org/arrayfire-rust/book/multi-threading.html)
///
/// ### NOTE
///
/// All operators(traits) from std::ops module implemented for Array object
/// carry out element wise operations. For example, `*` does multiplication of
/// elements at corresponding locations in two different Arrays.
pub struct Array<T> {
    handle: af_array,
    /// The phantom marker denotes the
    /// allocation of data on compute device
    _marker: PhantomData<T>,
}

/// Enable safely moving Array objects across threads
unsafe impl<T: HasAfEnum> Send for Array<T> {}

unsafe impl<T: HasAfEnum> Sync for Array<T> {}

macro_rules! is_func {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => {
        #[doc=$doc_str]
        pub fn $fn_name(&self) -> bool {
            unsafe {
                let mut ret_val: bool = false;
                let err_val = $ffi_fn(&mut ret_val as *mut bool, self.handle);
                HANDLE_ERROR(AfError::from(err_val));
                ret_val
            }
        }
    };
}

impl<T> Array<T>
where
    T: HasAfEnum,
{
    /// Constructs a new Array object
    ///
    /// # Examples
    ///
    /// An example of creating an Array from f32 array
    ///
    /// ```rust
    /// use arrayfire::{Array, Dim4, print};
    /// let values: [f32; 3] = [1.0, 2.0, 3.0];
    /// let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
    /// print(&indices);
    /// ```
    /// An example of creating an Array from half::f16 array
    ///
    /// ```rust
    /// use arrayfire::{Array, Dim4, is_half_available, print};
    /// use half::f16;
    ///
    /// let values: [f32; 3] = [1.0, 2.0, 3.0];
    ///
    /// if is_half_available(0) { // Default device is 0, hence the argument
    ///     let half_values = values.iter().map(|&x| f16::from_f32(x)).collect::<Vec<_>>();
    ///
    ///     let hvals = Array::new(&half_values, Dim4::new(&[3, 1, 1, 1]));
    ///
    ///     print(&hvals);
    /// } else {
    ///     println!("Half support isn't available on this device");
    /// }
    /// ```
    ///
    pub fn new(slice: &[T], dims: Dim4) -> Self {
        let aftype = T::get_af_dtype();
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_create_array(
                &mut temp as *mut af_array,
                slice.as_ptr() as *const c_void,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as *const c_longlong,
                aftype as c_uint,
            );
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }

    /// Constructs a new Array object from strided data
    ///
    /// The data pointed by the slice passed to this function can possibily be offseted using an additional `offset` parameter.
    pub fn new_strided(slice: &[T], offset: i64, dims: Dim4, strides: Dim4) -> Self {
        let aftype = T::get_af_dtype();
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_create_strided_array(
                &mut temp as *mut af_array,
                slice.as_ptr() as *const c_void,
                offset as dim_t,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as *const c_longlong,
                strides.get().as_ptr() as *const c_longlong,
                aftype as c_uint,
                1_u32,
            );
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }

    /// Constructs a new Array object of specified dimensions and type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arrayfire::{Array, Dim4};
    /// let garbage_vals = Array::<f32>::new_empty(Dim4::new(&[3, 1, 1, 1]));
    /// ```
    pub fn new_empty(dims: Dim4) -> Self {
        let aftype = T::get_af_dtype();
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_create_handle(
                &mut temp as *mut af_array,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as *const c_longlong,
                aftype as c_uint,
            );
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }

    /// Constructs a new Array object from device pointer
    ///
    /// The example show cases the usage using CUDA API, but usage of this function will
    /// be similar in CPU and OpenCL backends also. In the case of OpenCL backend, the pointer
    /// would be cl_mem. A short example of how to create an Array from device pointer is
    /// shown below but for detailed set of examples, please check out the tutorial book
    /// pages:
    ///  - [Interoperability with CUDA][1]
    ///  - [Interoperability with OpenCL][2]
    ///
    ///  [1]: http://arrayfire.org/arrayfire-rust/book/cuda-interop.html
    ///  [2]: http://arrayfire.org/arrayfire-rust/book/opencl-interop.html
    ///
    /// # Examples
    ///
    /// An example of creating an Array device pointer using
    /// [rustacuda](https://github.com/bheisler/RustaCUDA) crate. The
    /// example has to be copied to a `bin` crate with following contents in Cargo.toml
    /// to run successfully. Note that, all required setup for rustacuda and arrayfire crate
    /// have to completed first.
    /// ```text
    /// [package]
    /// ....
    /// [dependencies]
    /// rustacuda = "0.1"
    /// rustacuda_derive = "0.1"
    /// rustacuda_core = "0.1"
    /// arrayfire = "3.7.*"
    /// ```
    ///
    /// ```rust,ignore
    ///use arrayfire::*;
    ///use rustacuda::*;
    ///use rustacuda::prelude::*;
    ///
    ///fn main() {
    ///    let v: Vec<_> = (0u8 .. 100).map(f32::from).collect();
    ///
    ///    rustacuda::init(CudaFlags::empty());
    ///    let device = Device::get_device(0).unwrap();
    ///    let context = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO,
    ///                                           device).unwrap();
    ///    // Approach 1
    ///    {
    ///        let mut buffer = memory::DeviceBuffer::from_slice(&v).unwrap();
    ///
    ///        let array_dptr = Array::new_from_device_ptr(
    ///            buffer.as_device_ptr().as_raw_mut(), dim4!(10, 10));
    ///
    ///        af_print!("array_dptr", &array_dptr);
    ///
    ///        array_dptr.lock(); // Needed to avoid free as arrayfire takes ownership
    ///    }
    ///
    ///    // Approach 2
    ///    {
    ///        let mut dptr: *mut f32 = std::ptr::null_mut();
    ///        unsafe {
    ///            dptr = memory::cuda_malloc::<f32>(10*10).unwrap().as_raw_mut();
    ///        }
    ///        let array_dptr = Array::new_from_device_ptr(dptr, dim4!(10, 10));
    ///        // note that values might be garbage in the memory pointed out by dptr
    ///        // in this example as it is allocated but not initialized prior to passing
    ///        // along to arrayfire::Array::new*
    ///
    ///        // After ArrayFire takes over ownership of the pointer, you can use other
    ///        // arrayfire functions as usual.
    ///        af_print!("array_dptr", &array_dptr);
    ///    }
    ///}
    /// ```
    pub fn new_from_device_ptr(dev_ptr: *mut T, dims: Dim4) -> Self {
        let aftype = T::get_af_dtype();
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_device_array(
                &mut temp as *mut af_array,
                dev_ptr as *mut c_void,
                dims.ndims() as c_uint,
                dims.get().as_ptr() as *const dim_t,
                aftype as c_uint,
            );
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }

    /// Returns the backend of the Array
    ///
    /// # Return Values
    ///
    /// Returns an value of type `Backend` which indicates which backend
    /// was active when Array was created.
    pub fn get_backend(&self) -> Backend {
        unsafe {
            let mut ret_val: u32 = 0;
            let err_val = af_get_backend_id(&mut ret_val as *mut c_uint, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
            match (err_val, ret_val) {
                (0, 1) => Backend::CPU,
                (0, 2) => Backend::CUDA,
                (0, 3) => Backend::OPENCL,
                _ => Backend::DEFAULT,
            }
        }
    }

    /// Returns the device identifier(integer) on which the Array was created
    ///
    /// # Return Values
    ///
    /// Return the device id on which Array was created.
    pub fn get_device_id(&self) -> i32 {
        unsafe {
            let mut ret_val: i32 = 0;
            let err_val = af_get_device_id(&mut ret_val as *mut c_int, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
            ret_val
        }
    }

    /// Returns the number of elements in the Array
    pub fn elements(&self) -> usize {
        unsafe {
            let mut ret_val: dim_t = 0;
            let err_val = af_get_elements(&mut ret_val as *mut dim_t, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
            ret_val as usize
        }
    }

    /// Returns the Array data type
    pub fn get_type(&self) -> DType {
        unsafe {
            let mut ret_val: u32 = 0;
            let err_val = af_get_type(&mut ret_val as *mut c_uint, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
            DType::from(ret_val)
        }
    }

    /// Returns the dimensions of the Array
    pub fn dims(&self) -> Dim4 {
        unsafe {
            let mut ret0: i64 = 0;
            let mut ret1: i64 = 0;
            let mut ret2: i64 = 0;
            let mut ret3: i64 = 0;
            let err_val = af_get_dims(
                &mut ret0 as *mut dim_t,
                &mut ret1 as *mut dim_t,
                &mut ret2 as *mut dim_t,
                &mut ret3 as *mut dim_t,
                self.handle,
            );
            HANDLE_ERROR(AfError::from(err_val));
            Dim4::new(&[ret0 as u64, ret1 as u64, ret2 as u64, ret3 as u64])
        }
    }

    /// Returns the strides of the Array
    pub fn strides(&self) -> Dim4 {
        unsafe {
            let mut ret0: i64 = 0;
            let mut ret1: i64 = 0;
            let mut ret2: i64 = 0;
            let mut ret3: i64 = 0;
            let err_val = af_get_strides(
                &mut ret0 as *mut dim_t,
                &mut ret1 as *mut dim_t,
                &mut ret2 as *mut dim_t,
                &mut ret3 as *mut dim_t,
                self.handle,
            );
            HANDLE_ERROR(AfError::from(err_val));
            Dim4::new(&[ret0 as u64, ret1 as u64, ret2 as u64, ret3 as u64])
        }
    }

    /// Returns the number of dimensions of the Array
    pub fn numdims(&self) -> u32 {
        unsafe {
            let mut ret_val: u32 = 0;
            let err_val = af_get_numdims(&mut ret_val as *mut c_uint, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
            ret_val
        }
    }

    /// Returns the offset to the pointer from where data begins
    pub fn offset(&self) -> i64 {
        unsafe {
            let mut ret_val: i64 = 0;
            let err_val = af_get_offset(&mut ret_val as *mut dim_t, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
            ret_val
        }
    }

    /// Returns the native FFI handle for Rust object `Array`
    pub(crate) unsafe fn get(&self) -> af_array {
        self.handle
    }

    /// Set the native FFI handle for Rust object `Array`
    pub fn set(&mut self, handle: af_array) {
        self.handle = handle;
    }

    /// Copies the data from the Array to the mutable slice `data`
    ///
    /// # Examples
    ///
    /// Basic case
    /// ```
    /// # use arrayfire::{Array,Dim4,HasAfEnum};
    /// let a:Vec<u8> = vec![0,1,2,3,4,5,6,7,8];
    /// let b = Array::<u8>::new(&a,Dim4::new(&[3,3,1,1]));
    /// let mut c = vec!(u8::default();b.elements());
    /// b.host(&mut c);
    /// assert_eq!(c,a);
    /// ```
    /// Generic case
    /// ```
    /// # use arrayfire::{Array,Dim4,HasAfEnum};
    /// fn to_vec<T:HasAfEnum+Default+Clone>(array:&Array<T>) -> Vec<T> {
    ///     let mut vec = vec!(T::default();array.elements());
    ///     array.host(&mut vec);
    ///     return vec;
    /// }
    ///
    /// let a = Array::<u8>::new(&[0,1,2,3,4,5,6,7,8],Dim4::new(&[3,3,1,1]));
    /// let b:Vec<u8> = vec![0,1,2,3,4,5,6,7,8];
    /// assert_eq!(to_vec(&a),b);
    /// ```
    pub fn host<O: HasAfEnum>(&self, data: &mut [O]) {
        if data.len() != self.elements() {
            HANDLE_ERROR(AfError::ERR_SIZE);
        }
        unsafe {
            let err_val = af_get_data_ptr(data.as_mut_ptr() as *mut c_void, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Evaluates any pending lazy expressions that represent the data in the Array object
    pub fn eval(&self) {
        unsafe {
            let err_val = af_eval(self.handle);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Makes an copy of the Array
    ///
    /// This does a deep copy of the data into a new Array
    pub fn copy(&self) -> Self {
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_copy_array(&mut temp as *mut af_array, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }

    is_func!("Check if Array is empty", is_empty, af_is_empty);
    is_func!("Check if Array is scalar", is_scalar, af_is_scalar);
    is_func!("Check if Array is a row", is_row, af_is_row);
    is_func!("Check if Array is a column", is_column, af_is_column);
    is_func!("Check if Array is a vector", is_vector, af_is_vector);

    is_func!(
        "Check if Array is of real (not complex) type",
        is_real,
        af_is_real
    );
    is_func!(
        "Check if Array is of complex type",
        is_complex,
        af_is_complex
    );

    is_func!(
        "Check if Array's numerical type is of double precision",
        is_double,
        af_is_double
    );
    is_func!(
        "Check if Array's numerical type is of single precision",
        is_single,
        af_is_single
    );
    is_func!(
        "Check if Array's numerical type is of half precision",
        is_half,
        af_is_half
    );
    is_func!(
        "Check if Array is of integral type",
        is_integer,
        af_is_integer
    );
    is_func!("Check if Array is of boolean type", is_bool, af_is_bool);

    is_func!(
        "Check if Array is floating point real(not complex) data type",
        is_realfloating,
        af_is_realfloating
    );
    is_func!(
        "Check if Array is floating point type, either real or complex data",
        is_floating,
        af_is_floating
    );

    is_func!(
        "Check if Array's memory layout is continuous and one dimensional",
        is_linear,
        af_is_linear
    );
    is_func!("Check if Array is a sparse matrix", is_sparse, af_is_sparse);
    is_func!(
        "Check if Array's memory is owned by it and not a view of another Array",
        is_owner,
        af_is_owner
    );

    /// Cast the Array data type to `target_type`
    pub fn cast<O: HasAfEnum>(&self) -> Array<O> {
        let trgt_type = O::get_af_dtype();
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let err_val = af_cast(&mut temp as *mut af_array, self.handle, trgt_type as c_uint);
            HANDLE_ERROR(AfError::from(err_val));
            temp.into()
        }
    }

    /// Lock the device buffer in the memory manager
    ///
    /// Locked buffers are not freed by memory manager until unlock is called.
    pub fn lock(&self) {
        unsafe {
            let err_val = af_lock_array(self.handle);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Unlock the device buffer in the memory manager
    ///
    /// This function will give back the control over the device pointer to the
    /// memory manager.
    pub fn unlock(&self) {
        unsafe {
            let err_val = af_unlock_array(self.handle);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Get the device pointer and lock the buffer in memory manager
    ///
    /// The device pointer is not freed by memory manager until unlock is called.
    ///
    /// # Safety
    ///
    /// Using the function returns a pointer(CPU)/GPU-memory-pointer(CUDA)/cl_mem(OpenCL).
    /// Use this function only when you know what to do further with returned object.
    pub unsafe fn device_ptr(&self) -> void_ptr {
        let mut temp: void_ptr = std::ptr::null_mut();
        let err_val = af_get_device_ptr(&mut temp as *mut void_ptr, self.handle);
        HANDLE_ERROR(AfError::from(err_val));
        temp
    }

    /// Get the size of physical allocated bytes.
    ///
    /// This function will return the size of the parent/owner if the current Array object is an
    /// indexed Array.
    pub fn get_allocated_bytes(&self) -> usize {
        unsafe {
            let mut temp: usize = 0;
            let err_val = af_get_allocated_bytes(&mut temp as *mut usize, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
            temp
        }
    }

    /// Fetch Array as String
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let result: String;
        unsafe {
            let cname = CString::new("test").unwrap();
            let mut tmp: *mut c_char = ::std::ptr::null_mut();
            let err_val = af_array_to_string(
                &mut tmp,
                cname.to_bytes_with_nul().as_ptr() as *const c_char,
                self.get(),
                4,
                true,
            );
            HANDLE_ERROR(AfError::from(err_val));
            result = CStr::from_ptr(tmp).to_string_lossy().into_owned();
            free_host(tmp);
        }
        result
    }
}

/// Used for creating Array object from native
/// resource id, an 64 bit integer
#[allow(clippy::from_over_into)]
impl<T: HasAfEnum> Into<Array<T>> for af_array {
    fn into(self) -> Array<T> {
        Array {
            handle: self,
            _marker: PhantomData,
        }
    }
}

/// Returns a new Array object after incrementing the reference count of native resource
///
/// Cloning an Array does not do a deep copy of the underlying array data. It increments the
/// reference count of native resource and returns you the new reference in the form a new Array
/// object.
///
/// To create a deep copy use
/// [copy()](./struct.Array.html#method.copy)
impl<T> Clone for Array<T>
where
    T: HasAfEnum,
{
    fn clone(&self) -> Self {
        unsafe {
            let mut temp: af_array = std::ptr::null_mut();
            let ret_val = af_retain_array(&mut temp as *mut af_array, self.handle);
            match ret_val {
                0 => temp.into(),
                _ => panic!("Weak copy of Array failed with error code: {}", ret_val),
            }
        }
    }
}

/// To free resources when Array goes out of scope
impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        unsafe {
            let ret_val = af_release_array(self.handle);
            match ret_val {
                0 => (),
                _ => panic!("Array<T> drop failed with error code: {}", ret_val),
            }
        }
    }
}

/// Print data in the Array
///
/// # Parameters
///
/// - `input` is the Array to be printed
///
/// # Examples
///
/// ```rust
/// use arrayfire::{Dim4, print, randu};
/// println!("Create a 5-by-3 matrix of random floats on the GPU");
/// let dims = Dim4::new(&[5, 3, 1, 1]);
/// let a = randu::<f32>(dims);
/// print(&a);
/// ```
///
/// The sample output will look like below:
///
/// ```text
/// [5 3 1 1]
///     0.7402     0.4464     0.7762
///     0.9210     0.6673     0.2948
///     0.0390     0.1099     0.7140
///     0.9690     0.4702     0.3585
///     0.9251     0.5132     0.6814
/// ```
pub fn print<T: HasAfEnum>(input: &Array<T>) {
    let emptystring = CString::new("").unwrap();
    unsafe {
        let err_val = af_print_array_gen(
            emptystring.to_bytes_with_nul().as_ptr() as *const c_char,
            input.get(),
            4,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Generalized Array print function
///
/// Use this function to print Array data with arbitrary preicsion
///
/// # Parameters
///
/// - `msg` is message to be printed before printing the Array data
/// - `input` is the Array to be printed
/// - `precision` is data precision with which Array has to be printed
///
/// # Examples
///
/// ```rust
/// use arrayfire::{Dim4, print_gen, randu};
/// println!("Create a 5-by-3 matrix of random floats on the GPU");
/// let dims = Dim4::new(&[5, 3, 1, 1]);
/// let a = randu::<f32>(dims);
/// print_gen(String::from("Random Array"), &a, Some(6));
/// ```
///
/// The sample output will look like below:
///
/// ```text
/// Random Array
///
/// [5 3 1 1]
///     0.740276     0.446440     0.776202
///     0.921094     0.667321     0.294810
///     0.039014     0.109939     0.714090
///     0.969058     0.470269     0.358590
///     0.925181     0.513225     0.681451
/// ```
pub fn print_gen<T: HasAfEnum>(msg: String, input: &Array<T>, precision: Option<i32>) {
    let emptystring = CString::new(msg.as_bytes()).unwrap();
    unsafe {
        let err_val = af_print_array_gen(
            emptystring.to_bytes_with_nul().as_ptr() as *const c_char,
            input.get(),
            precision.unwrap_or(4),
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// evaluate multiple arrays
///
/// Use this function to evaluate multiple arrays in single call
///
/// # Parameters
///
/// - `inputs` are the list of arrays to be evaluated
pub fn eval_multiple<T: HasAfEnum>(inputs: Vec<&Array<T>>) {
    unsafe {
        let mut v = Vec::new();
        for i in inputs {
            v.push(i.get());
        }

        let err_val = af_eval_multiple(v.len() as c_int, v.as_ptr() as *const af_array);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Set eval flag value
///
/// This function can be used to toggle on/off the manual evaluation of arrays.
///
/// # Parameters
///
/// - `flag` is a boolean value indicating manual evaluation setting
pub fn set_manual_eval(flag: bool) {
    unsafe {
        let err_val = af_set_manual_eval_flag(flag as c_int);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Get eval flag value
///
/// This function can be used to find out if manual evaluation of arrays is
/// turned on or off.
///
/// # Return Values
///
/// A boolean indicating manual evaluation setting.
pub fn is_eval_manual() -> bool {
    unsafe {
        let mut ret_val: i32 = 0;
        let err_val = af_get_manual_eval_flag(&mut ret_val as *mut c_int);
        HANDLE_ERROR(AfError::from(err_val));
        ret_val > 0
    }
}

/// Prints data type, shape and data of a given Array in programming friendly context
///
/// Used via println macro or formatter
impl<T> fmt::Debug for Array<T>
where
    T: HasAfEnum,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let mut vec = vec![T::default(); self.elements()];
            self.host(&mut vec);
            f.debug_struct("Array")
                .field("dtype", &self.get_type())
                .field("shape", &self.dims())
                .field("strides", &self.strides())
                .field("offset", &self.offset())
                .field("device_id", &self.get_device_id())
                .field("data", &vec)
                .finish()
        } else {
            f.debug_struct("Array")
                .field("dtype", &self.get_type())
                .field("shape", &self.dims())
                .field("af_array", unsafe { &self.get() })
                .finish()
        }
    }
}

#[cfg(feature = "afserde")]
mod afserde {
    // Reimport required from super scope
    use super::{Array, DType, Dim4, HasAfEnum};

    use serde::de::{Deserializer, Error, Unexpected};
    use serde::ser::Serializer;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct ArrayOnHost<T: HasAfEnum + std::fmt::Debug> {
        dtype: DType,
        shape: Dim4,
        data: Vec<T>,
    }

    /// Serialize Implementation of Array
    impl<T> Serialize for Array<T>
    where
        T: Serialize + HasAfEnum,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut vec = vec![T::default(); self.elements()];
            self.host(&mut vec);
            let arr_on_host = ArrayOnHost {
                dtype: self.get_type(),
                shape: self.dims().clone(),
                data: vec,
            };
            arr_on_host.serialize(serializer)
        }
    }

    /// Deserialize Implementation of Array
    impl<'de, T> Deserialize<'de> for Array<T>
    where
        T: Deserialize<'de> + HasAfEnum,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            match ArrayOnHost::<T>::deserialize(deserializer) {
                Ok(arr_on_host) => {
                    let read_dtype = arr_on_host.dtype;
                    let expected_dtype = T::get_af_dtype();
                    if expected_dtype != read_dtype {
                        let error_msg = format!(
                            "data type is {:?}, deserialized type is {:?}",
                            expected_dtype, read_dtype
                        );
                        return Err(Error::invalid_value(Unexpected::Enum, &error_msg.as_str()));
                    }
                    Ok(Array::<T>::new(
                        &arr_on_host.data,
                        arr_on_host.shape.clone(),
                    ))
                }
                Err(err) => Err(err),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::array::print;
    use super::super::data::constant;
    use super::super::device::{info, set_device, sync};
    use crate::dim4;
    use std::sync::{mpsc, Arc, RwLock};
    use std::thread;

    #[test]
    fn thread_move_array() {
        // ANCHOR: move_array_to_thread
        set_device(0);
        info();
        let mut a = constant(1, dim4!(3, 3));

        let handle = thread::spawn(move || {
            //set_device to appropriate device id is required in each thread
            set_device(0);

            println!("\nFrom thread {:?}", thread::current().id());

            a += constant(2, dim4!(3, 3));
            print(&a);
        });

        //Need to join other threads as main thread holds arrayfire context
        handle.join().unwrap();
        // ANCHOR_END: move_array_to_thread
    }

    #[test]
    fn thread_borrow_array() {
        set_device(0);
        info();
        let a = constant(1i32, dim4!(3, 3));

        let handle = thread::spawn(move || {
            set_device(0); //set_device to appropriate device id is required in each thread
            println!("\nFrom thread {:?}", thread::current().id());
            print(&a);
        });
        //Need to join other threads as main thread holds arrayfire context
        handle.join().unwrap();
    }

    // ANCHOR: multiple_threads_enum_def
    #[derive(Debug, Copy, Clone)]
    enum Op {
        Add,
        Sub,
        Div,
        Mul,
    }
    // ANCHOR_END: multiple_threads_enum_def

    #[test]
    fn read_from_multiple_threads() {
        // ANCHOR: read_from_multiple_threads
        let ops: Vec<_> = vec![Op::Add, Op::Sub, Op::Div, Op::Mul, Op::Add, Op::Div];

        // Set active GPU/device on main thread on which
        // subsequent Array objects are created
        set_device(0);

        // ArrayFire Array's are internally maintained via atomic reference counting
        // Thus, they need no Arc wrapping while moving to another thread.
        // Just call clone method on the object and share the resulting clone object
        let a = constant(1.0f32, dim4!(3, 3));
        let b = constant(2.0f32, dim4!(3, 3));

        let threads: Vec<_> = ops
            .into_iter()
            .map(|op| {
                let x = a.clone();
                let y = b.clone();
                thread::spawn(move || {
                    set_device(0); //Both of objects are created on device 0 earlier
                    match op {
                        Op::Add => {
                            let _c = x + y;
                        }
                        Op::Sub => {
                            let _c = x - y;
                        }
                        Op::Div => {
                            let _c = x / y;
                        }
                        Op::Mul => {
                            let _c = x * y;
                        }
                    }
                    sync(0);
                    thread::sleep(std::time::Duration::new(1, 0));
                })
            })
            .collect();
        for child in threads {
            let _ = child.join();
        }
        // ANCHOR_END: read_from_multiple_threads
    }

    #[test]
    fn access_using_rwlock() {
        // ANCHOR: access_using_rwlock
        let ops: Vec<_> = vec![Op::Add, Op::Sub, Op::Div, Op::Mul, Op::Add, Op::Div];

        // Set active GPU/device on main thread on which
        // subsequent Array objects are created
        set_device(0);

        let c = constant(0.0f32, dim4!(3, 3));
        let a = constant(1.0f32, dim4!(3, 3));
        let b = constant(2.0f32, dim4!(3, 3));

        // Move ownership to RwLock and wrap in Arc since same object is to be modified
        let c_lock = Arc::new(RwLock::new(c));

        // a and b are internally reference counted by ArrayFire. Unless there
        // is prior known need that they may be modified, you can simply clone
        // the objects pass them to threads

        let threads: Vec<_> = ops
            .into_iter()
            .map(|op| {
                let x = a.clone();
                let y = b.clone();

                let wlock = c_lock.clone();
                thread::spawn(move || {
                    //Both of objects are created on device 0 in main thread
                    //Every thread needs to set the device that it is going to
                    //work on. Note that all Array objects must have been created
                    //on same device as of date this is written on.
                    set_device(0);
                    if let Ok(mut c_guard) = wlock.write() {
                        match op {
                            Op::Add => {
                                *c_guard += x + y;
                            }
                            Op::Sub => {
                                *c_guard += x - y;
                            }
                            Op::Div => {
                                *c_guard += x / y;
                            }
                            Op::Mul => {
                                *c_guard += x * y;
                            }
                        }
                    }
                })
            })
            .collect();

        for child in threads {
            let _ = child.join();
        }

        //let read_guard = c_lock.read().unwrap();
        //af_print!("C after threads joined", *read_guard);
        //C after threads joined
        //[3 3 1 1]
        //    8.0000     8.0000     8.0000
        //    8.0000     8.0000     8.0000
        //    8.0000     8.0000     8.0000
        // ANCHOR_END: access_using_rwlock
    }

    #[test]
    fn accum_using_channel() {
        // ANCHOR: accum_using_channel
        let ops: Vec<_> = vec![Op::Add, Op::Sub, Op::Div, Op::Mul, Op::Add, Op::Div];
        let ops_len: usize = ops.len();

        // Set active GPU/device on main thread on which
        // subsequent Array objects are created
        set_device(0);

        let mut c = constant(0.0f32, dim4!(3, 3));
        let a = constant(1.0f32, dim4!(3, 3));
        let b = constant(2.0f32, dim4!(3, 3));

        let (tx, rx) = mpsc::channel();

        let threads: Vec<_> = ops
            .into_iter()
            .map(|op| {
                // a and b are internally reference counted by ArrayFire. Unless there
                // is prior known need that they may be modified, you can simply clone
                // the objects pass them to threads
                let x = a.clone();
                let y = b.clone();

                let tx_clone = tx.clone();

                thread::spawn(move || {
                    //Both of objects are created on device 0 in main thread
                    //Every thread needs to set the device that it is going to
                    //work on. Note that all Array objects must have been created
                    //on same device as of date this is written on.
                    set_device(0);

                    let c = match op {
                        Op::Add => x + y,
                        Op::Sub => x - y,
                        Op::Div => x / y,
                        Op::Mul => x * y,
                    };
                    tx_clone.send(c).unwrap();
                })
            })
            .collect();

        for _i in 0..ops_len {
            c += rx.recv().unwrap();
        }

        //Need to join other threads as main thread holds arrayfire context
        for child in threads {
            let _ = child.join();
        }

        //af_print!("C after accumulating results", &c);
        //[3 3 1 1]
        //    8.0000     8.0000     8.0000
        //    8.0000     8.0000     8.0000
        //    8.0000     8.0000     8.0000
        // ANCHOR_END: accum_using_channel
    }

    #[cfg(feature = "afserde")]
    mod serde_tests {
        use super::super::Array;
        use crate::algorithm::sum_all;
        use crate::randu;

        #[test]
        fn array_serde_json() {
            // ANCHOR: array_json_serde_snippet
            let input = randu!(u8; 2, 2);
            let serd = match serde_json::to_string(&input) {
                Ok(serialized_str) => serialized_str,
                Err(e) => e.to_string(),
            };

            let deserd: Array<u8> = serde_json::from_str(&serd).unwrap();
            // ANCHOR_END: array_json_serde_snippet

            assert_eq!(sum_all(&(input - deserd)), (0u32, 0u32));
        }

        #[test]
        fn array_serde_bincode() {
            // ANCHOR: array_bincode_serde_snippet
            let input = randu!(u8; 2, 2);
            let encoded = match bincode::serialize(&input) {
                Ok(encoded) => encoded,
                Err(_) => vec![],
            };

            let decoded: Array<u8> = bincode::deserialize(&encoded).unwrap();
            // ANCHOR_END: array_bincode_serde_snippet

            assert_eq!(sum_all(&(input - decoded)), (0u32, 0u32));
        }
    }
}
