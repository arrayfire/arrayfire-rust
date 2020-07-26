use super::defines::AfError;
use super::util::{dim_t, free_host};

use libc::c_char;
use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
use std::sync::RwLock;

extern "C" {
    fn af_get_last_error(str: *mut *mut c_char, len: *mut dim_t);
}

/// Signature of error handling callback function
pub type ErrorCallback = fn(AfError);

/// Structure holding handle to callback function
pub struct Callback {
    cb: ErrorCallback,
}

impl Callback {
    /// Associated function to create a new Callback object
    pub fn new(callback: ErrorCallback) -> Self {
        Self { cb: callback }
    }

    /// call invokes the error callback with `error_code`.
    pub fn call(&self, error_code: AfError) {
        (self.cb)(error_code)
    }
}

/// Default error handling callback provided by ArrayFire crate
pub fn handle_error_general(error_code: AfError) {
    match error_code {
        AfError::SUCCESS => {} /* No-op */
        _ => panic!(
            "Error message: {}\nLast error: {}",
            error_code,
            get_last_error()
        ),
    }
}

lazy_static! {
    static ref ERROR_HANDLER_LOCK: RwLock<Callback> =
        RwLock::new(Callback::new(handle_error_general));
}

/// Register user provided error handler
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate arrayfire;
///
/// use arrayfire::{AfError, Callback, info, register_error_handler};
/// use std::error::Error;
///
/// fn handle_error(error_code: AfError) {
///     match error_code {
///         AfError::SUCCESS => {}, /* No-op */
///         _ => panic!("Error message: {}", error_code),
///     }
/// }
///
/// fn main() {
///     //Registering the error handler should be the first call
///     //before any other functions are called if your version
///     //of error is to be used for subsequent function calls
///     register_error_handler(Callback::new(handle_error));
///
///     info();
/// }
/// ```
#[allow(clippy::match_wild_err_arm)]
pub fn register_error_handler(cb_value: Callback) {
    let mut gaurd = match ERROR_HANDLER_LOCK.write() {
        Ok(g) => g,
        Err(_) => panic!("Failed to acquire lock to register error handler"),
    };

    *gaurd.deref_mut() = cb_value;
}

/// Default error handler for error code returned by ArrayFire FFI calls
#[allow(non_snake_case)]
#[allow(clippy::match_wild_err_arm)]
pub fn HANDLE_ERROR(error_code: AfError) {
    let gaurd = match ERROR_HANDLER_LOCK.read() {
        Ok(g) => g,
        Err(_) => panic!("Failed to acquire lock while handling FFI return value"),
    };

    (*gaurd.deref()).call(error_code);
}

/// Fetch last error description as String
pub fn get_last_error() -> String {
    let mut result: String = String::from("No Last Error");
    let mut tmp: *mut c_char = ::std::ptr::null_mut();
    let mut len: dim_t = 0;
    unsafe {
        af_get_last_error(&mut tmp, &mut len as *mut dim_t);
        if len > 0 {
            result = CStr::from_ptr(tmp).to_string_lossy().into_owned();
            free_host(tmp);
        }
    }
    result
}
