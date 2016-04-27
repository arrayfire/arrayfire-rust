use defines::AfError;

pub type ErrorCallback = Fn(AfError);

pub static mut HANDLE_ERROR: &'static ErrorCallback = &handle_error_general; 

pub fn register_error_handler(callback: &'static ErrorCallback) {
    unsafe {
        HANDLE_ERROR = callback;
    }
}

pub fn handle_error_general(error_code: AfError) {
    match error_code {
        AfError::SUCCESS            => {}, /* No-op */
        AfError::ERR_NO_MEM         => panic!("The system or device ran out of memory"),
        AfError::ERR_DRIVER         => panic!("There was an error in the device driver"),
        AfError::ERR_RUNTIME        => panic!("There was an error with the runtime environment"),
        AfError::ERR_INVALID_ARRAY  => panic!("The input array is not a valid Array object"),
        AfError::ERR_ARG            => panic!("One of the function arguments is incorrect"),
        AfError::ERR_SIZE           => panic!("The size is incorrect"),
        AfError::ERR_TYPE           => panic!("The type is not suppported by this function"),
        AfError::ERR_DIFF_TYPE      => panic!("The type of the input arrays are not compatible"),
        AfError::ERR_BATCH          => panic!("Function does not support GFOR / batch mode"),
        AfError::ERR_DEVICE         => panic!("Input does not belong to the current device"),
        AfError::ERR_NOT_SUPPORTED  => panic!("Unsupported operation/parameter option"),
        AfError::ERR_NOT_CONFIGURED => panic!("This build of ArrayFire does not support this feature"),
        AfError::ERR_NO_DBL         => panic!("This device does not support double"),
        AfError::ERR_NO_GFX         => panic!("This build of ArrayFire was not built with graphics or this device does not support graphics"),
        AfError::ERR_INTERNAL       => panic!("There was an internal error either in ArrayFire or in a project upstream"),
        AfError::ERR_UNKNOWN        => panic!("Unknown Error"),
    }
}