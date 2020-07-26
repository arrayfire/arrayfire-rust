use super::array::Array;
use super::defines::{AfError, RandomEngineType};
use super::dim4::Dim4;
use super::error::HANDLE_ERROR;
use super::util::{af_array, af_random_engine, dim_t, u64_t, FloatingPoint, HasAfEnum};

use libc::{c_int, c_uint};

extern "C" {
    fn af_set_seed(seed: u64_t) -> c_int;
    fn af_get_seed(seed: *mut u64_t) -> c_int;

    fn af_randu(out: *mut af_array, ndims: c_uint, dims: *const dim_t, afdtype: c_uint) -> c_int;
    fn af_randn(out: *mut af_array, ndims: c_uint, dims: *const dim_t, afdtype: c_uint) -> c_int;

    fn af_create_random_engine(engine: *mut af_random_engine, rtype: c_uint, seed: u64_t) -> c_int;
    fn af_retain_random_engine(
        engine: *mut af_random_engine,
        inputEngine: af_random_engine,
    ) -> c_int;
    fn af_random_engine_set_type(engine: *mut af_random_engine, rtpye: c_uint) -> c_int;
    fn af_random_engine_get_type(rtype: *mut c_uint, engine: af_random_engine) -> c_int;
    fn af_random_engine_set_seed(engine: *mut af_random_engine, seed: u64_t) -> c_int;
    fn af_random_engine_get_seed(seed: *mut u64_t, engine: af_random_engine) -> c_int;
    fn af_release_random_engine(engine: af_random_engine) -> c_int;

    fn af_get_default_random_engine(engine: *mut af_random_engine) -> c_int;
    fn af_set_default_random_engine_type(rtype: c_uint) -> c_int;

    fn af_random_uniform(
        out: *mut af_array,
        ndims: c_uint,
        dims: *const dim_t,
        aftype: c_uint,
        engine: af_random_engine,
    ) -> c_int;
    fn af_random_normal(
        out: *mut af_array,
        ndims: c_uint,
        dims: *const dim_t,
        aftype: c_uint,
        engine: af_random_engine,
    ) -> c_int;
}

/// Set seed for random number generation
pub fn set_seed(seed: u64) {
    unsafe {
        let err_val = af_set_seed(seed as u64_t);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Get the seed of random number generator
pub fn get_seed() -> u64 {
    let mut ret_val: u64 = 0;
    unsafe {
        let err_val = af_get_seed(&mut ret_val as *mut u64_t);
        HANDLE_ERROR(AfError::from(err_val));
    }
    ret_val
}

macro_rules! data_gen_def {
    [$doc_str: expr, $fn_name:ident, $ffi_name: ident, $($type_trait: ident),+] => (
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `dims` is the output dimensions
        ///
        ///# Return Values
        ///
        /// An Array with random values.
        pub fn $fn_name<T>(dims: Dim4) -> Array<T>
        where $( T: $type_trait, )* {
            let aftype = T::get_af_dtype();
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(&mut temp as *mut af_array,
                                        dims.ndims() as c_uint, dims.get().as_ptr() as *const dim_t,
                                        aftype as c_uint);
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    )
}

data_gen_def!(
    "Create random numbers from uniform distribution",
    randu,
    af_randu,
    HasAfEnum
);
data_gen_def!(
    "Create random numbers from normal distribution",
    randn,
    af_randn,
    HasAfEnum,
    FloatingPoint
);

/// Random number generator engine
///
/// This is a wrapper for ArrayFire's native random number generator engine.
pub struct RandomEngine {
    handle: af_random_engine,
}

/// Used for creating RandomEngine object from native resource id
impl From<af_random_engine> for RandomEngine {
    fn from(t: af_random_engine) -> Self {
        Self { handle: t }
    }
}

impl RandomEngine {
    /// Create a new random engine object
    ///
    /// # Parameters
    ///
    /// - `rengine` can be value of [RandomEngineType](./enum.RandomEngineType.html) enum.
    /// - `seed` is the initial seed value
    ///
    /// # Return Values
    ///
    /// A object of type RandomEngine
    pub fn new(rengine: RandomEngineType, seed: Option<u64>) -> Self {
        unsafe {
            let mut temp: af_random_engine = std::ptr::null_mut();
            let err_val = af_create_random_engine(
                &mut temp as *mut af_random_engine,
                rengine as c_uint,
                match seed {
                    Some(s) => s,
                    None => 0,
                } as u64_t,
            );
            HANDLE_ERROR(AfError::from(err_val));
            RandomEngine { handle: temp }
        }
    }

    /// Get random engine type
    pub fn get_type(&self) -> RandomEngineType {
        let mut temp: u32 = 0;
        unsafe {
            let err_val = af_random_engine_get_type(&mut temp as *mut c_uint, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
        }
        RandomEngineType::from(temp)
    }

    /// Get random engine type
    pub fn set_type(&mut self, engine_type: RandomEngineType) {
        unsafe {
            let err_val = af_random_engine_set_type(
                &mut self.handle as *mut af_random_engine,
                engine_type as c_uint,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Set seed for random engine
    pub fn set_seed(&mut self, seed: u64) {
        unsafe {
            let err_val =
                af_random_engine_set_seed(&mut self.handle as *mut af_random_engine, seed as u64_t);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Get seed of the random engine
    pub fn get_seed(&self) -> u64 {
        let mut seed: u64 = 0;
        unsafe {
            let err_val = af_random_engine_get_seed(&mut seed as *mut u64_t, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
        }
        seed
    }

    /// Returns the native FFI handle for Rust object `RandomEngine`
    pub fn get(&self) -> af_random_engine {
        self.handle
    }
}

/// Increment reference count of RandomEngine's native resource
impl Clone for RandomEngine {
    fn clone(&self) -> Self {
        unsafe {
            let mut temp: af_random_engine = std::ptr::null_mut();
            let err_val = af_retain_random_engine(&mut temp as *mut af_random_engine, self.handle);
            HANDLE_ERROR(AfError::from(err_val));
            RandomEngine::from(temp)
        }
    }
}

/// Free RandomEngine's native resource
impl Drop for RandomEngine {
    fn drop(&mut self) {
        unsafe {
            let err_val = af_release_random_engine(self.handle);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

/// Get default random engine
pub fn get_default_random_engine() -> RandomEngine {
    unsafe {
        let mut temp: af_random_engine = std::ptr::null_mut();
        let mut err_val = af_get_default_random_engine(&mut temp as *mut af_random_engine);
        HANDLE_ERROR(AfError::from(err_val));
        let mut handle: af_random_engine = std::ptr::null_mut();
        err_val = af_retain_random_engine(&mut handle as *mut af_random_engine, temp);
        HANDLE_ERROR(AfError::from(err_val));
        RandomEngine { handle: handle } //    ::from(handle)
    }
}

/// Set the random engine type for default random number generator
///
/// # Parameters
///
/// - `rtype` can take one of the values of enum [RandomEngineType](./enum.RandomEngineType.html)
pub fn set_default_random_engine_type(rtype: RandomEngineType) {
    unsafe {
        let err_val = af_set_default_random_engine_type(rtype as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Generate array of uniform numbers using a random engine
///
/// # Parameters
///
/// - `dims` is output array dimensions
/// - `engine` is an object of type [RandomEngine](./struct.RandomEngine.html)
///
/// # Return Values
///
/// An Array with uniform numbers generated using random engine
pub fn random_uniform<T>(dims: Dim4, engine: &RandomEngine) -> Array<T>
where
    T: HasAfEnum,
{
    let aftype = T::get_af_dtype();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_random_uniform(
            &mut temp as *mut af_array,
            dims.ndims() as c_uint,
            dims.get().as_ptr() as *const dim_t,
            aftype as c_uint,
            engine.get(),
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Generate array of normal numbers using a random engine
///
/// # Parameters
///
/// - `dims` is output array dimensions
/// - `engine` is an object of type [RandomEngine](./struct.RandomEngine.html)
///
/// # Return Values
///
/// An Array with normal numbers generated using random engine
pub fn random_normal<T>(dims: Dim4, engine: &RandomEngine) -> Array<T>
where
    T: HasAfEnum + FloatingPoint,
{
    let aftype = T::get_af_dtype();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_random_normal(
            &mut temp as *mut af_array,
            dims.ndims() as c_uint,
            dims.get().as_ptr() as *const dim_t,
            aftype as c_uint,
            engine.get(),
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}
