# Interoperability with CUDA

{{#include interop_excerpts.md:1:4}}

ArrayFire manages its own memory, runs within its own CUDA stream, and creates custom IDs for
devices. As such, most of the interoperability functions focus on reducing potential synchronization
conflicts between ArrayFire and CUDA.

## Basics

{{#include interop_excerpts.md:6:7}}

| Function                           | Purpose                                                   |
|------------------------------------|-----------------------------------------------------------|
| [Array::new\_from\_device\_ptr][1] | Construct an ArrayFire [Array][14] from device memory     |
| [Array::device\_ptr][2]            | Obtain a pointer to the device memory (implies `lock()`)  |
| [Array::lock][3]                   | Removes ArrayFire's control of a device memory pointer    |
| [Array::unlock][4]                 | Restores ArrayFire's control over a device memory pointer |
| [get\_device][5]                   | Gets the current ArrayFire device ID                      |
| [set\_device][6]                   | Switches ArrayFire to the specified device                |
| [get\_device\_native\_id][7]       | Fetches CUDA deviceID for a given ArrayFire device ID     |
| [set\_device\_native\_id][8]       | Switches active device to the specified CUDA device ID    |
| [get\_stream][9]                   | Get the current CUDA stream used by ArrayFire             |

## Using custom CUDA kernels in existing ArrayFire application

By default, ArrayFire manages its own memory and operates in its own CUDA stream. Thus there is a
slight amount of bookkeeping that needs to be done in order to integrate your custom CUDA kernel.

Ideally, we recommend using ArrayFire's CUDA stream to launch your custom kernels. However, this
is currently not possible due to limitation on [RustaCUDA][10] not being to able to wrap an
existing cudaStream\_t/CUstream\_t objects. The current work around is to create a stream of your
own and launch the kernel on it.

Notice that since ArrayFire and your kernels are not sharing the same CUDA stream, there is a need
to perform explicit synchronization before launching kernel on your stream that depends on the
computation carried out by ArrayFire earlier. This extra step is unnecessary once the above stated
limiation of RustaCUDA's stream is eliminated.

This process is best illustrated with a fully worked example:
```rust
{{#include ../../cuda-interop/examples/custom_kernel.rs}}
```

## Adding ArrayFire to existing CUDA Application

{{#include interop_excerpts.md:9:15}}

 1. Finish any pending CUDA operations (e.g. cudaDeviceSynchronize() or similar stream functions)
 2. Create ArrayFire arrays from existing CUDA pointers
 3. Perform operations on ArrayFire arrays
 4. Instruct ArrayFire to finish operations using [eval][11] and [sync][12]
 5. Obtain pointers to important memory
 6. Continue your CUDA application.
 7. Free non-managed memory

To create the [Array][14] fom device pointer, you should use one of the following approaches:

Using DeviceBuffer from [RustaCUDA][10], or a Wrapper Object for CUDA device memory
```rust
let mut buffer = memory::DeviceBuffer::from_slice(&v).unwrap();

let array_dptr = Array::new_from_device_ptr(
    buffer.as_device_ptr().as_raw_mut(), dim4!(10, 10));

array_dptr.lock(); // Needed to avoid free as arrayfire takes ownership
```

Using raw pointer returned from cuda\_malloc interface exposed by [RustaCUDA][10]
```rust
let mut dptr: *mut f32 = std::ptr::null_mut();
unsafe {
    dptr = memory::cuda_malloc::<f32>(10*10).unwrap().as_raw_mut();
}

let array_dptr = Array::new_from_device_ptr(dptr, dim4!(10, 10));
// After ArrayFire takes over ownership of the pointer, you can use other
// arrayfire functions as usual.
```

ArrayFire's memory manager automatically assumes responsibility for any memory provided to it.
Thus ArrayFire could free or reuse the memory at any later time. If this behavior is not desired,
you may call [Array::unlock][13] and manage the memory yourself. However, if you do so, please be
cautious not to free memory when ArrayFire might be using it!

The seven steps above are best illustrated using a fully-worked example:

```rust
{{#include ../../cuda-interop/examples/cuda_af_app.rs}}
```

[1]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.new_from_device_ptr
[2]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.device_ptr
[3]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.lock
[4]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.unlock
[5]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.get_device.html
[6]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.set_device.html
[7]: http://arrayfire.org/arrayfire-rust/af_cuda_interop/fn.get_device_native_id.html
[8]: http://arrayfire.org/arrayfire-rust/af_cuda_interop/fn.set_device_native_id.html
[9]: http://arrayfire.org/arrayfire-rust/af_cuda_interop/fn.get_stream.html
[10]: https://github.com/bheisler/RustaCUDA
[11]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.eval
[12]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.sync.html
[13]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.unlock
[14]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html
