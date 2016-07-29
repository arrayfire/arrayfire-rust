#[macro_use]
extern crate arrayfire as af;

use std::error::Error;
use std::thread;
use std::time::Duration;
use af::*;

macro_rules! implement_handler {
    ($fn_name:ident, $msg: expr) => (

        pub fn $fn_name(error_code: AfError) {
            println!("{:?}", $msg);
            match error_code {
                AfError::SUCCESS => {}, /* No-op */
                _ => panic!("Error message: {}", error_code.description()),
            }
        }

    )
}

implement_handler!(handler_sample1, "Error Handler Sample1");
implement_handler!(handler_sample2, "Error Handler Sample2");
implement_handler!(handler_sample3, "Error Handler Sample3");
implement_handler!(handler_sample4, "Error Handler Sample4");

pub const HANDLE1: Callback<'static> = Callback{ cb: &handler_sample1};
pub const HANDLE2: Callback<'static> = Callback{ cb: &handler_sample2};
pub const HANDLE3: Callback<'static> = Callback{ cb: &handler_sample3};
pub const HANDLE4: Callback<'static> = Callback{ cb: &handler_sample4};

#[allow(unused_must_use)]
#[test]
fn check_error_handler_mutation() {

    for i in 0..4 {
        thread::Builder::new().name(format!("child {}",i+1).to_string()).spawn(move || {
            println!("{:?}", thread::current());
            match i {
                0 => register_error_handler(HANDLE1),
                1 => register_error_handler(HANDLE2),
                2 => register_error_handler(HANDLE3),
                3 => register_error_handler(HANDLE4),
                _ => panic!("Impossible scenario"),
            }
        });
    }

    af::info();
    thread::sleep(Duration::from_millis(50));

}
