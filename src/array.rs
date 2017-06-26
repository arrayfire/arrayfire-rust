extern crate libc;

use dim4::Dim4;
use defines::{AfError, DType, Backend};
use error::HANDLE_ERROR;
use util::{AfArray, DimT, HasAfEnum, MutAfArray, MutVoidPtr};
use self::libc::{uint8_t, c_void, c_int, c_uint, c_longlong, c_ulong, c_char};
use std::ffi::CString;

// Some unused functions from array.h in C-API of ArrayFire
// af_copy_array
// af_write_array
// af_get_data_ref_count

#[allow(dead_code)]
extern {
    fn af_create_array(out: MutAfArray, data: *const c_void,
                       ndims: c_uint, dims: *const DimT, aftype: uint8_t) -> c_int;

    fn af_create_handle(out: MutAfArray, ndims: c_uint, dims: *const DimT, aftype: uint8_t) -> c_int;

    fn af_get_elements(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_get_type(out: *mut c_int, arr: AfArray) -> c_int;

    fn af_get_dims(dim0: *mut c_longlong, dim1: *mut c_longlong, dim2: *mut c_longlong,
                   dim3: *mut c_longlong, arr: AfArray) -> c_int;

    fn af_get_numdims(result: *mut c_uint, arr: AfArray) -> c_int;

    fn af_is_empty(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_scalar(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_row(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_column(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_vector(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_complex(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_real(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_double(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_single(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_realfloating(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_floating(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_integer(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_bool(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_get_data_ptr(data: *mut c_void, arr: AfArray) -> c_int;

    fn af_eval(arr: AfArray) -> c_int;

    fn af_eval_multiple(num: c_int, arrays: *const AfArray) -> c_int;

    fn af_set_manual_eval_flag(flag: c_int) -> c_int;

    fn af_get_manual_eval_flag(flag: *mut c_int) -> c_int;

    fn af_retain_array(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_copy_array(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_release_array(arr: AfArray) -> c_int;

    fn af_print_array(arr: AfArray) -> c_int;

    fn af_print_array_gen(exp: *const c_char, arr: AfArray, precision: c_int) -> c_int;

    fn af_cast(out: MutAfArray, arr: AfArray, aftype: uint8_t) -> c_int;

    fn af_get_backend_id(backend: *mut c_int, input: AfArray) -> c_int;

    fn af_get_device_id(device: *mut c_int, input: AfArray) -> c_int;

    fn af_create_strided_array(arr: MutAfArray, data: *const c_void, offset: DimT,
                               ndims: c_uint, dims: *const DimT, strides: *const DimT,
                               aftype: uint8_t, stype: uint8_t) -> c_int;

    fn af_get_strides(s0: *mut DimT, s1: *mut DimT, s2: *mut DimT, s3: *mut DimT,
                      arr: AfArray) -> c_int;

    fn af_get_offset(offset: *mut DimT, arr: AfArray) -> c_int;

    fn af_is_linear(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_owner(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_is_sparse(result: *mut c_int, arr: AfArray) -> c_int;

    fn af_lock_array(arr: AfArray) -> c_int;

    fn af_unlock_array(arr: AfArray) -> c_int;

    fn af_get_device_ptr(ptr: MutVoidPtr, arr: AfArray) -> c_int;

    fn af_get_allocated_bytes(result: *mut c_ulong, arr: AfArray) -> c_int;
}

/// A multidimensional data container
///
/// Currently, Array objects can store only data until four dimensions
///
/// ### NOTE
///
/// All operators(traits) from std::ops module implemented for Array object
/// carry out element wise operations. For example, `*` does multiplication of
/// elements at corresponding locations in two different Arrays.
pub struct Array {
    handle: i64,
}

macro_rules! is_func {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => (
        #[doc=$doc_str]
        pub fn $fn_name(&self) -> bool {
            unsafe {
                let mut ret_val: i32 = 0;
                let err_val = $ffi_fn(&mut ret_val as *mut c_int, self.handle as AfArray);
                HANDLE_ERROR(AfError::from(err_val));
                ret_val>0
            }
        }
    )
}

impl Array {
    /// Constructs a new Array object
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arrayfire::{Array, Dim4, print};
    /// let values: [f32; 3] = [1.0, 2.0, 3.0];
    /// let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
    /// print(&indices);
    /// ```
    #[allow(unused_mut)]
    pub fn new<T: HasAfEnum>(slice: &[T], dims: Dim4) -> Array {
        unsafe {
            let aftype = T::get_af_dtype();
            let mut temp: i64 = 0;
            let err_val = af_create_array(&mut temp as MutAfArray,
                                          slice.as_ptr() as *const c_void,
                                          dims.ndims() as c_uint,
                                          dims.get().as_ptr() as * const c_longlong,
                                          aftype as uint8_t);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    }

    /// Constructs a new Array object from strided data
    ///
    /// The data pointed by the slice passed to this function can possibily be offseted using an additional `offset` parameter.
    #[allow(unused_mut)]
    pub fn new_strided<T: HasAfEnum>(slice: &[T], offset: i64,
                                     dims: Dim4, strides: Dim4) -> Array {
        unsafe {
            let aftype = T::get_af_dtype();
            let mut temp: i64 = 0;
            let err_val = af_create_strided_array(&mut temp as MutAfArray,
                                                  slice.as_ptr() as *const c_void,
                                                  offset as DimT,
                                                  dims.ndims() as c_uint,
                                                  dims.get().as_ptr() as * const c_longlong,
                                                  strides.get().as_ptr() as * const c_longlong,
                                                  aftype as uint8_t, 1);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    }

    /// Constructs a new Array object of specified dimensions and type
    ///
    /// # Examples
    ///
    /// ```rust
    /// use arrayfire::{Array, Dim4, DType};
    /// let garbageVals = Array::new_empty(Dim4::new(&[3, 1, 1, 1]), DType::F32);
    /// ```
    #[allow(unused_mut)]
    pub fn new_empty(dims: Dim4, aftype: DType) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_create_handle(&mut temp as MutAfArray,
                                           dims.ndims() as c_uint,
                                           dims.get().as_ptr() as * const c_longlong,
                                           aftype as uint8_t);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
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
            let mut ret_val: i32 = 0;
            let err_val = af_get_backend_id(&mut ret_val as *mut c_int, self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            match (err_val, ret_val) {
                (0, 1) => Backend::CPU,
                (0, 2) => Backend::CUDA,
                (0, 3) => Backend::OPENCL,
                _      => Backend::DEFAULT,
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
            let err_val = af_get_device_id(&mut ret_val as *mut c_int, self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            ret_val
        }
    }

    /// Returns the number of elements in the Array
    pub fn elements(&self) -> usize {
        unsafe {
            let mut ret_val: i64 = 0;
            let err_val = af_get_elements(&mut ret_val as MutAfArray, self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            ret_val as usize
        }
    }

    /// Returns the Array data type
    pub fn get_type(&self) -> DType {
        unsafe {
            let mut ret_val: i32 = 0;
            let err_val = af_get_type(&mut ret_val as *mut c_int, self.handle as AfArray);
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
            let err_val = af_get_dims(&mut ret0 as *mut DimT, &mut ret1 as *mut DimT,
                                      &mut ret2 as *mut DimT, &mut ret3 as *mut DimT,
                                      self.handle as AfArray);
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
            let err_val = af_get_strides(&mut ret0 as *mut DimT, &mut ret1 as *mut DimT,
                                         &mut ret2 as *mut DimT, &mut ret3 as *mut DimT,
                                         self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            Dim4::new(&[ret0 as u64, ret1 as u64, ret2 as u64, ret3 as u64])
        }
    }

    /// Returns the number of dimensions of the Array
    pub fn numdims(&self) -> u32 {
        unsafe {
            let mut ret_val: u32 = 0;
            let err_val = af_get_numdims(&mut ret_val as *mut c_uint, self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            ret_val
        }
    }

    /// Returns the offset to the pointer from where data begins
    pub fn offset(&self) -> i64 {
        unsafe {
            let mut ret_val: i64 = 0;
            let err_val = af_get_offset(&mut ret_val as *mut DimT, self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            ret_val
        }
    }

    /// Returns the native FFI handle for Rust object `Array`
    pub fn get(&self) -> i64 {
        self.handle
    }

    /// Copies the data from the Array to the mutable slice `data`
    pub fn host<T: HasAfEnum>(&self, data: &mut [T]) {
        if data.len() != self.elements() {
            HANDLE_ERROR(AfError::ERR_SIZE);
        }
        unsafe {
            let err_val = af_get_data_ptr(data.as_mut_ptr() as *mut c_void, self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Evaluates any pending lazy expressions that represent the data in the Array object
    pub fn eval(&self) {
        unsafe {
            let err_val = af_eval(self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Makes an copy of the Array
    ///
    /// This does a deep copy of the data into a new Array
    pub fn copy(&self) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_copy_array(&mut temp as MutAfArray, self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    }

    is_func!("Check if Array is empty", is_empty, af_is_empty);
    is_func!("Check if Array is scalar", is_scalar, af_is_scalar);
    is_func!("Check if Array is a row", is_row, af_is_row);
    is_func!("Check if Array is a column", is_column, af_is_column);
    is_func!("Check if Array is a vector", is_vector, af_is_vector);
    is_func!("Check if Array is of complex type", is_complex, af_is_complex);
    is_func!("Check if Array's numerical type is of double precision", is_double, af_is_double);
    is_func!("Check if Array's numerical type is of single precision", is_single, af_is_single);
    is_func!("Check if Array is of real type", is_real, af_is_real);
    is_func!("Check if Array is of single precision", is_floating, af_is_floating);
    is_func!("Check if Array is of integral type", is_integer, af_is_integer);
    is_func!("Check if Array is of boolean type", is_bool, af_is_bool);
    is_func!("Check if Array's memory layout is continuous and one dimensional", is_linear, af_is_linear);
    is_func!("Check if Array's memory is owned by it and not a view of another Array", is_owner, af_is_owner);

    /// Cast the Array data type to `target_type`
    pub fn cast<T: HasAfEnum>(&self) -> Array {
        unsafe {
            let trgt_type = T::get_af_dtype();
            let mut temp: i64 = 0;
            let err_val = af_cast(&mut temp as MutAfArray, self.handle as AfArray, trgt_type as uint8_t);
            HANDLE_ERROR(AfError::from(err_val));
            Array::from(temp)
        }
    }

    /// Find if the current array is sparse
    pub fn is_sparse(&self) -> bool {
        unsafe {
            let mut temp: i32 = 0;
            let err_val = af_is_sparse(&mut temp as *mut c_int, self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            temp > 0
        }
    }

    /// Lock the device buffer in the memory manager
    ///
    /// Locked buffers are not freed by memory manager until unlock is called.
    pub fn lock(&self) {
        unsafe {
            let err_val = af_lock_array(self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Unlock the device buffer in the memory manager
    ///
    /// This function will give back the control over the device pointer to the
    /// memory manager.
    pub fn unlock(&self) {
        unsafe {
            let err_val = af_unlock_array(self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Get the device pointer and lock the buffer in memory manager
    ///
    /// The device pointer is not freed by memory manager until unlock is called.
    pub fn device_ptr(&self) -> u64 {
        unsafe {
            let mut temp: u64 = 0;
            let err_val = af_get_device_ptr(&mut temp as MutVoidPtr, self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            temp
        }
    }

    /// Get the size of physical allocated bytes.
    ///
    /// This function will return the size of the parent/owner if the current Array object is an
    /// indexed Array.
    pub fn get_allocated_bytes(&self) -> u64 {
        unsafe {
            let mut temp: u64 = 0;
            let err_val = af_get_allocated_bytes(&mut temp as *mut c_ulong, self.handle as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
            temp
        }
    }
}

/// Used for creating Array object from native resource id
impl From<i64> for Array {
    fn from(t: i64) -> Array {
        Array {handle: t}
    }
}

/// Returns a new Array object after incrementing the reference count of native resource
///
/// Cloning an Array does not do a deep copy of the underlying array data. It increments the
/// reference count of native resource and returns you the new reference in the form a new Array
/// object.
///
/// To create a deep copy use
/// [copy()](http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.copy)
impl Clone for Array {
    fn clone(&self) -> Array {
        unsafe {
            let mut temp: i64 = 0;
            let ret_val = af_retain_array(&mut temp as MutAfArray, self.handle as AfArray);
            match ret_val {
                0 => Array {handle: temp},
                _ => panic!("Weak copy of Array failed with error code: {}", ret_val),
            }
        }
    }
}

/// To free resources when Array goes out of scope
impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            let ret_val = af_release_array(self.handle);
            match ret_val {
                0 => (),
                _ => panic!("Weak copy of Array failed with error code: {}", ret_val),
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
pub fn print(input: &Array) {
    let emptystring = CString::new("").unwrap();
    unsafe {
        let err_val = af_print_array_gen(emptystring.to_bytes_with_nul().as_ptr() as *const c_char,
                                         input.get() as AfArray, 4);
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
pub fn print_gen(msg: String, input: &Array, precision: Option<i32>) {
    let emptystring = CString::new(msg.as_bytes()).unwrap();
    unsafe {
        let err_val = af_print_array_gen(emptystring.to_bytes_with_nul().as_ptr() as *const c_char,
                                         input.get() as AfArray,
                                         match precision {Some(p)=>p, None=>4} as c_int);
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
pub fn eval_multiple(inputs: Vec<&Array>) {
    unsafe {
        let mut v = Vec::new();
        for i in inputs {
            v.push(i.get());
        }

        let err_val = af_eval_multiple(v.len() as c_int, v.as_ptr() as *const AfArray);
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
