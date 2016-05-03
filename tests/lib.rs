extern crate arrayfire as af;

use std::error::Error;
use std::thread;
use std::time::Duration;
use af::*;

pub fn handler_sample1(error_code: AfError) {
    println!("Error handler sample1");
    match error_code {
        AfError::SUCCESS => {}, /* No-op */
        _ => panic!("Error message: {}", error_code.description()),
    }
}

pub fn handler_sample2(error_code: AfError) {
    println!("Error handler sample2");
    match error_code {
        AfError::SUCCESS => {}, /* No-op */
        _ => panic!("Error message: {}", error_code.description()),
    }
}

pub static HANDLE1: &'static ErrorCallback = &handler_sample1;
pub static HANDLE2: &'static ErrorCallback = &handler_sample2;

#[test]
fn check_error_handler_mutation() {

    for i in 0..4 {
        thread::spawn(move || {
            if i%2==0 {
                register_error_handler(HANDLE2);
            } else {
                register_error_handler(HANDLE1);
            }
        });
    }

    thread::sleep(Duration::from_millis(50));

}
