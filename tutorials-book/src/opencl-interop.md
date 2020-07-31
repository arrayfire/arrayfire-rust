# Interoperability with OpenCL

{{#include interop_excerpts.md:1:4}}

ArrayFire manages its own context, queue, memory, and creates custom IDs for devices. As such, most
of the interoperability functions focus on reducing potential synchronization conflicts between
ArrayFire and OpenCL.

## Basics

{{#include interop_excerpts.md:6:7}}

| Function                           | Purpose                                                   |
|------------------------------------|-----------------------------------------------------------|
| [Array::new\_from\_device\_ptr][1] | Construct an ArrayFire Array from cl\_mem                 |
| [Array::device\_ptr][2]            | Obtain a pointer to the device memory (implies `lock`)    |
| [Array::lock][3]                   | Removes ArrayFire's control of a device memory pointer    |
| [Array::unlock][4]                 | Restores ArrayFire's control over a device memory pointer |
| [get\_platform][7]                 | Get ArrayFire's current cl\_platform                      |
| [get\_device][5]                   | Gets the current ArrayFire device ID                      |
| [get\_device\_id][8]               | Get ArrayFire's current cl\_device\_id                    |
| [set\_device\_id][9]               | Set ArrayFire's device from a cl\_device\_id              |
| [set\_device][6]                   | Switches ArrayFire to the specified device                |
| [get\_context][10]                 | Get ArrayFire's current cl\_context                       |
| [get\_queue][11]                   | Get ArrayFire's current cl\_command\_queue                |
| [get\_device\_type][12]            | Get the current [DeviceType][16]                          |

Note that the pointer returned by [Array::device\_ptr][2] should be cast to `cl_mem` before using
it with OpenCL opaque types. The pointer is a `cl_mem` internally that is force casted to pointer
type by ArrayFire before returning the value to caller.

Additionally, the OpenCL backend permits the programmer to add and remove custom devices from the
ArrayFire device manager. These permit you to attach ArrayFire directly to the OpenCL queue used by
other portions of your application.

| Function                      | Purpose                                                  |
|-------------------------------|----------------------------------------------------------|
| [add\_device\_context][13]    | Add a new device to ArrayFire's device manager           |
| [set\_device\_context][15]    | Set ArrayFire's device from cl\_device\_id & cl\_context |
| [delete\_device\_context][14] | Remove a device from ArrayFire's device manager          |

Below we provide two worked examples on how ArrayFire can be integrated
into new and existing projects.

## Adding custom OpenCL kernels to an existing ArrayFire application

By default, ArrayFire manages its own context, queue, memory, and creates custom IDs for devices.
Thus there is some bookkeeping that needs to be done to integrate your custom OpenCL kernel.

If your kernels can share operate in the same queue as ArrayFire, you should:

1. Obtain the OpenCL context, device, and queue used by ArrayFire
2. Obtain cl\_mem references to [Array][18] objects
3. Load, build, and use your kernels
4. Return control of [Array][18] memory to ArrayFire

Note, ArrayFire uses an in-order queue, thus when ArrayFire and your kernels are operating in the
same queue, there is no need to perform any synchronization operations.

This process is best illustrated with a fully worked example:

```rust
{{#include ../../opencl-interop/examples/custom_kernel.rs}}
```

If your kernels needs to operate in their own OpenCL queue, the process is essentially identical,
except you need to instruct ArrayFire to complete its computations using the [sync][17] function
prior to launching your own kernel and ensure your kernels are complete using `clFinish`
(or similar) commands prior to returning control of the memory to ArrayFire:

1. Obtain the OpenCL context, device, and queue used by ArrayFire
2. Obtain cl\_mem references to [Array][18] objects
3. Instruct ArrayFire to finish operations using [sync][17]
4. Load, build, and use your kernels
5. Instruct OpenCL to finish operations using clFinish() or similar commands.
6. Return control of [Array][18] memory to ArrayFire

## Adding ArrayFire to an existing OpenCL application

{{#include interop_excerpts.md:9:15}}

1. Instruct OpenCL to complete its operations using clFinish (or similar)
2. Instruct ArrayFire to use the user-created OpenCL Context
3. Create ArrayFire arrays from OpenCL memory objects
4. Perform ArrayFire operations on the [Array][18]s
5. Instruct ArrayFire to finish operations using [sync][17]
6. Obtain cl\_mem references for important memory
7. Continue your OpenCL application

<!--
To create the [Array][18] object, you should use the following approach:

```rust
```
-->

> ArrayFire's memory manager automatically assumes responsibility for any memory provided to
it. If you are creating an array from another RAII style object, you should retain it to ensure
your memory is not deallocated if your RAII object were to go out of scope.

> If you do not wish for ArrayFire to manage your memory, you may call the [Array::unlock][4]
function and manage the memory yourself; however, if you do so, please be cautious not to call
`clReleaseMemObj` on a `cl_mem`  when ArrayFire might be using it!

Given below is a fully working example:

```rust
{{#include ../../opencl-interop/examples/ocl_af_app.rs}}
```

[1]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.new_from_device_ptr
[2]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.device_ptr
[3]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.lock
[4]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.unlock
[5]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.get_device.html
[6]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.set_device.html
[7]: http://arrayfire.org/arrayfire-rust/af_opencl_interop/fn.get_platform.html
[8]: http://arrayfire.org/arrayfire-rust/af_opencl_interop/fn.get_device_id.html
[9]: http://arrayfire.org/arrayfire-rust/af_opencl_interop/fn.set_device_id.html
[10]: http://arrayfire.org/arrayfire-rust/af_opencl_interop/fn.get_context.html
[11]: http://arrayfire.org/arrayfire-rust/af_opencl_interop/fn.get_queue.html
[12]: http://arrayfire.org/arrayfire-rust/af_opencl_interop/fn.get_device_type.html
[13]: http://arrayfire.org/arrayfire-rust/af_opencl_interop/fn.add_device_context.html
[14]: http://arrayfire.org/arrayfire-rust/af_opencl_interop/fn.delete_device_context.html
[15]: http://arrayfire.org/arrayfire-rust/af_opencl_interop/fn.set_device_context.html
[16]: http://arrayfire.org/arrayfire-rust/af_opencl_interop/enum.DeviceType.html
[17]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.sync.html
[18]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html
