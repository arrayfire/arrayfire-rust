use std::ops::{Deref, DerefMut};
use defines::AfError;
use std::error::Error;
use std::sync::RwLock;

/// Signature of error handling callback function
pub type ErrorCallback = fn(AfError);

/// Structure holding handle to callback function
pub struct Callback {
    cb: ErrorCallback,
}

impl Callback {
    /// Associated function to create a new Callback object
    pub fn new(callback: ErrorCallback) -> Self {
        Callback {cb: callback}
    }

    /// call invokes the error callback with `error_code`.
    pub fn call(&self, error_code: AfError) {
        (self.cb)(error_code)
    }
}

/// Default error handling callback provided by ArrayFire crate
pub fn handle_error_general(error_code: AfError) {
    match error_code {
        AfError::SUCCESS => {}, /* No-op */
        _ => panic!("Error message: {}", error_code.description()),
    }
}

lazy_static! {
    static ref ERROR_HANDLER_LOCK: RwLock< Callback > =
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
/// fn handleError(error_code: AfError) {
///     match error_code {
///         AfError::SUCCESS => {}, /* No-op */
///         _ => panic!("Error message: {}", error_code.description()),
///     }
/// }
///
/// fn main() {
///     //Registering the error handler should be the first call
///     //before any other functions are called if your version
///     //of error is to be used for subsequent function calls
///     register_error_handler(Callback::new(handleError));
///
///     info();
/// }
/// ```
#[allow(unused_must_use)]
pub fn register_error_handler(cb_value: Callback) {
    let mut gaurd = match ERROR_HANDLER_LOCK.write() {
        Ok(g) => g,
        Err(_)=> panic!("Failed to acquire lock to register error handler"),
    };

    *gaurd.deref_mut() = cb_value;
}

#[allow(non_snake_case)]
pub fn HANDLE_ERROR(error_code: AfError) {
    let gaurd = match ERROR_HANDLER_LOCK.read() {
        Ok(g) => g,
        Err(_)=> panic!("Failed to acquire lock while handling FFI return value"),
    };

    (*gaurd.deref()).call(error_code);
}
