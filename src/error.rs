
use defines::AfError;
use std::error::Error;
use std::sync::Mutex;

pub type ErrorCallback = Fn(AfError);

pub static mut HANDLE_ERROR: &'static ErrorCallback = &handle_error_general; 

lazy_static! {
    static ref HANDLE_ERROR_LOCK: Mutex<i32> = Mutex::new(0);
}

#[allow(unused_must_use)]
pub fn register_error_handler(callback: &'static ErrorCallback) {
    HANDLE_ERROR_LOCK.lock().unwrap();
    unsafe {
        HANDLE_ERROR = callback;
    }
}

pub fn handle_error_general(error_code: AfError) {
    match error_code {
        AfError::SUCCESS => {}, /* No-op */
        _ => panic!("Error message: {}", error_code.description()),
    }
}
