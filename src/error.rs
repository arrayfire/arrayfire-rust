use std::ops::{Deref, DerefMut};
use defines::AfError;
use std::error::Error;
use std::marker::{Send, Sync};
use std::sync::RwLock;


pub type ErrorCallback = Fn(AfError);


/// Wrap ErrorCallback function pointer inside a structure
/// to enable implementing Send, Sync traits on it.
pub struct Callback<'cblifetime> {
    pub cb: &'cblifetime ErrorCallback,
}


// Implement Send, Sync traits for Callback structure to
// enable the user of Callback function pointer in conjunction
// with threads using a mutex.
unsafe impl<'cblifetime> Send for Callback<'cblifetime> {}
unsafe impl<'cblifetime> Sync for Callback<'cblifetime> {}


pub const DEFAULT_HANDLE_ERROR: Callback<'static> = Callback{cb: &handle_error_general};


lazy_static! {
    static ref ERROR_HANDLER_LOCK: RwLock< Callback<'static> > =
        RwLock::new(DEFAULT_HANDLE_ERROR);
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
/// pub const ERR_HANDLE: Callback<'static> = Callback{ cb: &handleError};
///
/// fn main() {
///     register_error_handler(ERR_HANDLE);
///
///     info();
/// }
/// ```
#[allow(unused_must_use)]
pub fn register_error_handler(cb_value: Callback<'static>) {
    let mut gaurd = match ERROR_HANDLER_LOCK.write() {
        Ok(g) => g,
        Err(_)=> panic!("Failed to acquire lock to register error handler"),
    };

    *gaurd.deref_mut() = cb_value;
}

pub fn handle_error_general(error_code: AfError) {
    match error_code {
        AfError::SUCCESS => {}, /* No-op */
        _ => panic!("Error message: {}", error_code.description()),
    }
}

#[allow(non_snake_case)]
pub fn HANDLE_ERROR(error_code: AfError) {
    let gaurd = match ERROR_HANDLER_LOCK.read() {
        Ok(g) => g,
        Err(_)=> panic!("Failed to acquire lock while handling FFI return value"),
    };

    let func = gaurd.deref().cb;

    func(error_code);
}
