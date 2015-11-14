extern crate libc;

use dim4::Dim4;
use defines::{AfError, Aftype, Backend};
use self::libc::{uint8_t, c_void, c_int, c_uint, c_longlong};

type MutAfArray = *mut self::libc::c_longlong;
type MutDouble  = *mut self::libc::c_double;
type MutUint    = *mut self::libc::c_uint;
type AfArray    = self::libc::c_longlong;
type DimT       = self::libc::c_longlong;

// Some unused functions from array.h in C-API of ArrayFire
// af_create_handle
// af_copy_array
// af_write_array
// af_get_data_ptr
// af_get_data_ref_count

#[allow(dead_code)]
extern {
    fn af_create_array(out: MutAfArray, data: *const c_void,
                       ndims: c_uint, dims: *const DimT, aftype: uint8_t) -> c_int;

    fn af_get_elements(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_get_type(out: *mut uint8_t, arr: AfArray) -> c_int;

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

    fn af_retain_array(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_copy_array(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_release_array(arr: AfArray) -> c_int;

    fn af_print_array(arr: AfArray) -> c_int;

    fn af_cast(out: MutAfArray, arr: AfArray, aftype: uint8_t) -> c_int;

    fn af_get_backend_id(backend: *mut c_int, input: AfArray) -> c_int;
}

/// A multidimensional data container
///
/// Currently, Array objects can store only data until four dimensions
pub struct Array {
    handle: i64,
}

macro_rules! is_func {
    ($fn_name: ident, $ffi_fn: ident) => (
        /// Checks if the Array is of specific format/data type.
        pub fn $fn_name(&self) -> Result<bool, AfError> {
            unsafe {
                let mut ret_val: i32 = 0;
                let err_val = $ffi_fn(&mut ret_val as *mut c_int, self.handle as AfArray);
                match err_val {
                    0 => Ok(ret_val>0),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

impl Array {
    /// Constructs a new Array object
    ///
    /// # Examples
    ///
    /// ```
    /// let values: &[f32] = &[1.0, 2.0, 3.0];
    /// let indices = Array::new(Dim4::new(&[3, 1, 1, 1]), values, Aftype::F32).unwrap();
    /// ```
    #[allow(unused_mut)]
    pub fn new<T>(dims: Dim4, slice: &[T], aftype: Aftype) -> Result<Array, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_create_array(&mut temp as MutAfArray,
                                          slice.as_ptr() as *const c_void,
                                          dims.ndims() as c_uint,
                                          dims.get().as_ptr() as * const c_longlong,
                                          aftype as uint8_t);
            match err_val {
                0 => Ok(Array {handle: temp}),
                _ => Err(AfError::from(err_val)),
            }
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
            af_get_backend_id(&mut ret_val as *mut c_int, self.handle as AfArray);
            match ret_val {
                1 => Backend::AF_BACKEND_CPU,
                2 => Backend::AF_BACKEND_CUDA,
                3 => Backend::AF_BACKEND_OPENCL,
                _ => Backend::AF_BACKEND_DEFAULT,
            }
        }
    }

    /// Returns the number of elements in the Array
    pub fn elements(&self) -> Result<i64, AfError> {
        unsafe {
            let mut ret_val: i64 = 0;
            let err_val = af_get_elements(&mut ret_val as MutAfArray, self.handle as AfArray);
            match err_val {
                0 => Ok(ret_val),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    /// Returns the Array data type
    pub fn get_type(&self) -> Result<Aftype, AfError> {
        unsafe {
            let mut ret_val: u8 = 0;
            let err_val = af_get_type(&mut ret_val as *mut uint8_t, self.handle as AfArray);
            match err_val {
                0 => Ok(Aftype::from(ret_val)),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    /// Returns the dimensions of the Array
    pub fn dims(&self) -> Result<Dim4, AfError> {
        unsafe {
            let mut ret0: i64 = 0;
            let mut ret1: i64 = 0;
            let mut ret2: i64 = 0;
            let mut ret3: i64 = 0;
            let err_val = af_get_dims(&mut ret0 as *mut c_longlong, &mut ret1 as *mut c_longlong,
                                      &mut ret2 as *mut c_longlong, &mut ret3 as *mut c_longlong,
                                      self.handle as AfArray);
            match err_val {
                0 => Ok(Dim4::new(&[ret0 as u64, ret1 as u64, ret2 as u64, ret3 as u64])),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    /// Returns the number of dimensions of the Array
    pub fn numdims(&self) -> Result<u32, AfError> {
        unsafe {
            let mut ret_val: u32 = 0;
            let err_val = af_get_numdims(&mut ret_val as *mut c_uint, self.handle as AfArray);
            match err_val {
                0 => Ok(ret_val),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    /// Returns the native FFI handle for Rust object `Array`
    pub fn get(&self) -> i64 {
        self.handle
    }

    /// Copies the data from the Array to the mutable slice `data`
    pub fn host<T>(&self, data: &mut [T]) -> Result<(), AfError> {
        unsafe {
            let ret_val = af_get_data_ptr(data.as_mut_ptr() as *mut c_void, self.handle as AfArray);
            match ret_val {
                0 => Ok(()),
                _ => Err(AfError::from(ret_val)),
            }
        }
    }

    /// Evaluates any pending lazy expressions that represent the data in the Array object
    pub fn eval(&self) -> Result<(), AfError> {
        unsafe {
            let ret_val = af_eval(self.handle as AfArray);
            match ret_val {
                0 => Ok(()),
                _ => Err(AfError::from(ret_val)),
            }
        }
    }

    /// Makes an copy of the Array
    ///
    /// Internally, this is handled by reference counting
    pub fn copy(&self) -> Result<Array, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_copy_array(&mut temp as MutAfArray, self.handle as AfArray);
            match err_val {
                0 => Ok(Array::from(temp)),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    is_func!(is_empty, af_is_empty);
    is_func!(is_scalar, af_is_scalar);
    is_func!(is_row, af_is_row);
    is_func!(is_column, af_is_column);
    is_func!(is_vector, af_is_vector);
    is_func!(is_complex, af_is_complex);
    is_func!(is_double, af_is_double);
    is_func!(is_single, af_is_single);
    is_func!(is_real, af_is_real);
    is_func!(is_floating, af_is_floating);
    is_func!(is_integer, af_is_integer);
    is_func!(is_bool, af_is_bool);

    /// Cast the Array data type to `target_type`
    pub fn cast(&self, target_type: Aftype) -> Result<Array, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_cast(&mut temp as MutAfArray, self.handle as AfArray, target_type as uint8_t);
            match err_val {
                0 => Ok(Array::from(temp)),
                _ => Err(AfError::from(err_val)),
            }
        }
    }
}

/// Used for creating Array object from native resource id
impl From<i64> for Array {
    fn from(t: i64) -> Array {
        Array {handle: t}
    }
}

/// Used for incrementing the reference count of Array's native resource
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
/// # Examples
///
///  ```
/// println!("Create a 5-by-3 matrix of random floats on the GPU");
/// let a = match randu(dims, Aftype::F32) {
///     Ok(value) => value,
///     Err(error) => panic!("{}", error),
/// };
/// print(&a);
///  ```
///
///  The sample output will look like below:
///
///  ```
///  [5 3 1 1]
///      0.7402     0.4464     0.7762
///      0.9210     0.6673     0.2948
///      0.0390     0.1099     0.7140
///      0.9690     0.4702     0.3585
///      0.9251     0.5132     0.6814
///  ```
pub fn print(input: &Array) -> Result<(), AfError> {
    unsafe {
        let ret_val = af_print_array(input.get() as AfArray);
        match ret_val {
            0 => Ok(()),
            _ => Err(AfError::from(ret_val)),
        }
    }
}
