use std::thread;

use ::arrayfire::*;

macro_rules! implement_handler {
    ($fn_name:ident) => {
        pub fn $fn_name(error_code: AfError) {
            match error_code {
                AfError::SUCCESS => {} /* No-op */
                _ => panic!("Error message: {}", error_code),
            }
        }
    };
}

implement_handler!(handler_sample1);
implement_handler!(handler_sample2);
implement_handler!(handler_sample3);
implement_handler!(handler_sample4);

#[allow(unused_must_use)]
#[test]
fn check_error_handler_mutation() {
    let children = (0..4)
        .map(|i| {
            thread::Builder::new()
                .name(format!("child {}", i + 1))
                .spawn(move || {
                    let target_device = i % arrayfire::device_count();
                    println!(
                        "Thread {:?} 's target device is {}",
                        thread::current(),
                        target_device
                    );
                    match i {
                        0 => register_error_handler(Callback::new(handler_sample1)),
                        1 => register_error_handler(Callback::new(handler_sample2)),
                        2 => register_error_handler(Callback::new(handler_sample3)),
                        3 => register_error_handler(Callback::new(handler_sample4)),
                        _ => panic!("Impossible scenario"),
                    }
                })
                .expect("Failed to launch a thread")
        })
        .collect::<Vec<_>>();

    for c in children {
        c.join();
    }
}
