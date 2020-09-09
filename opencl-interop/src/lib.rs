//! af-opencl-interop package is to used only when the application intends to mix
//! arrayfire code with raw OpenCL code.
//!
//! Functions from this crate return OpenCL C API opaque pointers typedefs
//!
//! - [cl_device_id](https://docs.rs/cl-sys/0.4.2/cl_sys/type.cl_device_id.html)
//! - [cl_context](https://docs.rs/cl-sys/0.4.2/cl_sys/type.cl_context.html)
//! - [cl_command_queue](https://docs.rs/cl-sys/0.4.2/cl_sys/type.cl_command_queue.html)

use arrayfire::{handle_error_general, AfError};
use cl_sys::{
    cl_command_queue, cl_context, cl_device_id, CL_DEVICE_TYPE_ACCELERATOR, CL_DEVICE_TYPE_ALL,
    CL_DEVICE_TYPE_CPU, CL_DEVICE_TYPE_DEFAULT, CL_DEVICE_TYPE_GPU,
};
use libc::c_int;
use std::mem;

/// OpenCL Vendor Platform
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VendorPlatform {
    AMD = 0,
    APPLE = 1,
    INTEL = 2,
    NVIDIA = 3,
    BEIGNET = 4,
    POCL = 5,
    UNKNOWN = -1,
}

/// OpenCL Device Type
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DeviceType {
    DEFAULT = CL_DEVICE_TYPE_DEFAULT,
    CPU = CL_DEVICE_TYPE_CPU,
    GPU = CL_DEVICE_TYPE_GPU,
    ACCEL = CL_DEVICE_TYPE_ACCELERATOR,
    ALL = CL_DEVICE_TYPE_ALL,
}

extern "C" {
    fn afcl_get_context(ctx: *mut cl_context, retain: bool) -> c_int;
    fn afcl_get_queue(queue: *mut cl_command_queue, retain: bool) -> c_int;
    fn afcl_get_device_id(out: *mut cl_device_id) -> c_int;
    fn afcl_set_device_id(id: cl_device_id) -> c_int;

    fn afcl_add_device_context(
        dev_id: cl_device_id,
        ctx: cl_context,
        queue: cl_command_queue,
    ) -> c_int;
    fn afcl_set_device_context(dev_id: cl_device_id, ctx: cl_context) -> c_int;
    fn afcl_delete_device_context(dev_id: cl_device_id, ctx: cl_context) -> c_int;

    fn afcl_get_device_type(dtype: *mut c_int) -> c_int;
    fn afcl_get_platform(ptype: *mut c_int) -> c_int;
}

/// Get the handle to active ArrayFire OpenCL context
pub fn get_context(retain: bool) -> cl_context {
    unsafe {
        let mut out: cl_context = std::ptr::null_mut();
        let err_val = afcl_get_context(&mut out as *mut cl_context, retain);
        handle_error_general(AfError::from(err_val));
        out
    }
}

/// Get the handle to active ArrayFire OpenCL command queue
pub fn get_queue(retain: bool) -> cl_command_queue {
    unsafe {
        let mut out: cl_command_queue = std::ptr::null_mut();
        let err_val = afcl_get_queue(&mut out as *mut cl_command_queue, retain);
        handle_error_general(AfError::from(err_val));
        out
    }
}

/// Get the handle to active ArrayFire OpenCL device
pub fn get_device_id() -> cl_device_id {
    unsafe {
        let mut out: cl_device_id = std::ptr::null_mut();
        let err_val = afcl_get_device_id(&mut out as *mut cl_device_id);
        handle_error_general(AfError::from(err_val));
        out
    }
}

/// Set the cl_device_id as the active ArrayFire OpenCL device
pub fn set_device_id(dev_id: cl_device_id) {
    unsafe {
        let err_val = afcl_set_device_id(dev_id);
        handle_error_general(AfError::from(err_val));
    }
}

/// Push user provided device, context and queue tuple to ArrayFire device mamanger
pub fn add_device_context(dev_id: cl_device_id, ctx: cl_context, queue: cl_command_queue) {
    unsafe {
        let err_val = afcl_add_device_context(dev_id, ctx, queue);
        handle_error_general(AfError::from(err_val));
    }
}

/// Set the device identified by device & context pair as the active device for ArrayFire
pub fn set_device_context(dev_id: cl_device_id, ctx: cl_context) {
    unsafe {
        let err_val = afcl_set_device_context(dev_id, ctx);
        handle_error_general(AfError::from(err_val));
    }
}

/// Remove the user provided device, context pair from ArrayFire device mamanger
pub fn delete_device_context(dev_id: cl_device_id, ctx: cl_context) {
    unsafe {
        let err_val = afcl_delete_device_context(dev_id, ctx);
        handle_error_general(AfError::from(err_val));
    }
}

///// Fetch Active ArrayFire device's type i.e. CPU/GPU/Accelerator etc.
pub fn get_device_type() -> DeviceType {
    unsafe {
        let mut out: i32 = 0;
        let err_val = afcl_get_device_type(&mut out as *mut c_int);
        handle_error_general(AfError::from(err_val));
        match out {
            -1 => mem::transmute(out as u64),
            _ => DeviceType::ALL,
        }
    }
}

/// Fetch Active ArrayFire device's vendor platform
pub fn get_platform() -> VendorPlatform {
    unsafe {
        let mut out: i32 = 0;
        let err_val = afcl_get_platform(&mut out as *mut c_int);
        handle_error_general(AfError::from(err_val));
        mem::transmute(out)
    }
}
