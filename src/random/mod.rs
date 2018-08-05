extern crate libc;

use array::Array;
use dim4::Dim4;
use defines::{AfError, RandomEngineType};
use error::HANDLE_ERROR;
use self::libc::{uint8_t, c_int, c_uint};
use util::{FloatingPoint, HasAfEnum};
use util::{DimT, MutAfArray, MutRandEngine, RandEngine, Uintl};

#[allow(dead_code)]
extern {
    fn af_set_seed(seed: Uintl) -> c_int;
    fn af_get_seed(seed: *mut Uintl) -> c_int;

    fn af_randu(out: MutAfArray, ndims: c_uint, dims: *const DimT, afdtype: uint8_t) -> c_int;
    fn af_randn(out: MutAfArray, ndims: c_uint, dims: *const DimT, afdtype: uint8_t) -> c_int;

    fn af_create_random_engine(engine: MutRandEngine, rtype: uint8_t, seed: Uintl) -> c_int;
    fn af_retain_random_engine(engine: MutRandEngine, inputEngine: RandEngine) -> c_int;
    fn af_random_engine_set_type(engine: MutRandEngine, rtpye: uint8_t) -> c_int;
    fn af_random_engine_get_type(rtype: *mut uint8_t, engine: RandEngine) -> c_int;
    fn af_random_engine_set_seed(engine: MutRandEngine, seed: Uintl) -> c_int;
    fn af_random_engine_get_seed(seed: *mut Uintl, engine: RandEngine) -> c_int;
    fn af_release_random_engine(engine: RandEngine) -> c_int;

    fn af_get_default_random_engine(engine: MutRandEngine) -> c_int;
    fn af_set_default_random_engine_type(rtype: uint8_t) -> c_int;

    fn af_random_uniform(out: MutAfArray, ndims: c_uint, dims: *const DimT,
                         aftype: uint8_t, engine: RandEngine) -> c_int;
    fn af_random_normal(out: MutAfArray, ndims: c_uint, dims: *const DimT,
                        aftype: uint8_t, engine: RandEngine) -> c_int;
}

/// Set seed for random number generation
pub fn set_seed(seed: u64) {
    unsafe {
        let err_val = af_set_seed(seed as Uintl);
        HANDLE_ERROR(AfError::from(err_val));
    }
}

/// Get the seed of random number generator
#[allow(unused_mut)]
pub fn get_seed() -> u64 {
    let mut ret_val: u64 = 0;
    unsafe {
        let err_val = af_get_seed(&mut ret_val as *mut Uintl);
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
        #[allow(unused_mut)]
        pub fn $fn_name<T>(dims: Dim4) -> Array<T>
        where $( T: $type_trait, )* {
            let aftype = T::get_af_dtype();
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                        dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                                        aftype as uint8_t);
                HANDLE_ERROR(AfError::from(err_val));
            }
            temp.into()
        }
    )
}

data_gen_def!("Create random numbers from uniform distribution",
              randu, af_randu, HasAfEnum);
data_gen_def!("Create random numbers from normal distribution",
              randn, af_randn, HasAfEnum, FloatingPoint);

/// Random number generator engine
///
/// This is a wrapper for ArrayFire's native random number generator engine.
pub struct RandomEngine {
    handle: i64,
}

/// Used for creating RandomEngine object from native resource id
impl From<i64> for RandomEngine {
    fn from(t: i64) -> RandomEngine {
        RandomEngine {handle: t}
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
    pub fn new(rengine: RandomEngineType, seed: Option<u64>) -> RandomEngine {
        let mut temp: i64 = 0;
        unsafe {
            let err_val = af_create_random_engine(&mut temp as MutRandEngine, rengine as uint8_t,
                                                  match seed {Some(s) => s, None => 0} as Uintl);
            HANDLE_ERROR(AfError::from(err_val));
        }
        RandomEngine::from(temp)
    }

    /// Get random engine type
    pub fn get_type(&self) -> RandomEngineType {
        let mut temp: u8 = 0;
        unsafe {
            let err_val = af_random_engine_get_type(&mut temp as *mut uint8_t,
                                                    self.handle as RandEngine);
            HANDLE_ERROR(AfError::from(err_val));
        }
        RandomEngineType::from(temp as i32)
    }

    /// Get random engine type
    pub fn set_type(&mut self, engine_type: RandomEngineType) {
        unsafe {
            let err_val = af_random_engine_set_type(&mut self.handle as MutRandEngine,
                                                    engine_type as uint8_t);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Set seed for random engine
    pub fn set_seed(&mut self, seed: u64) {
        unsafe {
            let err_val = af_random_engine_set_seed(&mut self.handle as MutRandEngine,
                                                    seed as Uintl);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Get seed of the random engine
    pub fn get_seed(&self) -> u64 {
        let mut seed: u64 = 0;
        unsafe {
            let err_val = af_random_engine_get_seed(&mut seed as *mut Uintl, self.handle as RandEngine);
            HANDLE_ERROR(AfError::from(err_val));
        }
        seed
    }

    /// Returns the native FFI handle for Rust object `RandomEngine`
    pub fn get(&self) -> i64 {
        self.handle
    }
}

/// Increment reference count of RandomEngine's native resource
impl Clone for RandomEngine {
    fn clone(&self) -> RandomEngine {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_retain_random_engine(&mut temp as MutRandEngine, self.handle as RandEngine);
            HANDLE_ERROR(AfError::from(err_val));
            RandomEngine::from(temp)
        }
    }
}

/// Free RandomEngine's native resource
impl Drop for RandomEngine {
    fn drop(&mut self) {
        unsafe {
            let err_val = af_release_random_engine(self.handle as RandEngine);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

/// Get default random engine
pub fn get_default_random_engine() -> RandomEngine {
    let mut handle : i64 = 0;
    unsafe {
        let mut temp : i64 = 0;
        let mut err_val = af_get_default_random_engine(&mut temp as MutRandEngine);
        HANDLE_ERROR(AfError::from(err_val));
        err_val = af_retain_random_engine(&mut handle as MutRandEngine, temp as RandEngine);
        HANDLE_ERROR(AfError::from(err_val));
    }
    RandomEngine::from(handle)
}

/// Set the random engine type for default random number generator
///
/// # Parameters
///
/// - `rtype` can take one of the values of enum [RandomEngineType](./enum.RandomEngineType.html)
pub fn set_default_random_engine_type(rtype: RandomEngineType) {
    unsafe {
        let err_val = af_set_default_random_engine_type(rtype as uint8_t);
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
where T: HasAfEnum {
    let aftype = T::get_af_dtype();
    let mut temp : i64 = 0;
    unsafe {
        let err_val = af_random_uniform(&mut temp as MutAfArray,
                                        dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                                        aftype as uint8_t, engine.get() as RandEngine);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
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
    where T: HasAfEnum + FloatingPoint
{
    let aftype = T::get_af_dtype();
    let mut temp : i64 = 0;
    unsafe {
        let err_val = af_random_normal(&mut temp as MutAfArray,
                                       dims.ndims() as c_uint, dims.get().as_ptr() as *const DimT,
                                       aftype as uint8_t, engine.get() as RandEngine);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}
