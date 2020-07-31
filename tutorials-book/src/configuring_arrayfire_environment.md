# Configuring Arrayfire Environment

Following are the list of environment and runtime configurations that will help enhance your
experience with ArrayFire.

## AF\_PATH

This is the path with ArrayFire gets installed, ie. the includes and libs are present in this
directory. You can use this variable to add include paths and libraries to your projects.

## AF\_PRINT\_ERRORS

When `AF\_PRINT\_ERRORS` is set to 1, the exceptions thrown are more verbose and detailed. This
helps in locating the exact failure.

```
AF_PRINT_ERRORS=1 ./myprogram
```

## AF\_CUDA\_DEFAULT\_DEVICE

Use this variable to set the default CUDA device. Valid values for this variable are the device
identifiers shown when [info][1] is run.

```
AF_CUDA_DEFAULT_DEVICE=1 ./myprogram
```

Note: [set\_device][2] call in the source code will take precedence over this variable.

## AF\_OPENCL\_DEFAULT\_DEVICE

Use this variable to set the default OpenCL device. Valid values for this variable are the device
identifiers shown when [info][1] is run.

```
AF_OPENCL_DEFAULT_DEVICE=1 ./myprogram
```

Note: [set\_device][2] call in the source code will take precedence over this variable.

## AF\_OPENCL\_DEFAULT\_DEVICE\_TYPE

Use this variable to set the default OpenCL device type. Valid values for this variable are: CPU,
GPU, ACC (Accelerators). When set, the first device of the specified type is chosen as default device.

```
AF_OPENCL_DEFAULT_DEVICE_TYPE=CPU ./myprogram
```

Note: `AF_OPENCL_DEFAULT_DEVICE` and [set\_device][2] takes precedence over this variable.

## AF\_OPENCL\_DEVICE\_TYPE

Use this variable to only choose OpenCL devices of specified type. Valid values for this variable are:

- ALL: All OpenCL devices. (Default behavior).
- CPU: CPU devices only.
- GPU: GPU devices only.
- ACC: Accelerator devices only.

When set, the remaining OpenCL device types are ignored by the OpenCL backend.

```
AF_OPENCL_DEVICE_TYPE=CPU ./myprogram
```

## AF\_OPENCL\_CPU\_OFFLOAD

When ArrayFire runs on devices with unified memory with the host (ie. `CL_DEVICE_HOST_UNIFIED_MENORY`
is true for the device) then certain functions are offloaded to run on the CPU using mapped buffers.

ArrayFire takes advantage of fast libraries such as MKL while spending no time copying memory from
device to host. The device memory is mapped to a host pointer which can be used in the offloaded
functions.

This functionality can be disabled by using the environment variable `AF_OPENCL_CPU_OFFLOAD=0`.

The default bevaior of this has changed in version 3.4. Prior to v3.4, CPU Offload functionality was
used only when the user set `AF_OPENCL_CPU_OFFLOAD=1` and disabled otherwise. From v3.4 onwards, CPU
Offload is enabled by default and is disabled only when `AF_OPENCL_CPU_OFFLOAD=0` is set.

## AF\_OPENCL\_SHOW\_BUILD\_INFO

This variable is useful when debuggin OpenCL kernel compilation failures. When this variable is set
to 1, and an error occurs during a OpenCL kernel compilation, then the log and kernel are printed to screen.

## AF\_DISABLE\_GRAPHICS

Setting this variable to 1 will disable window creation when graphics functions are being called.
Disabling window creation will disable all other graphics calls at runtime. This is a useful
when running code on servers and systems without displays. When graphics calls are run on such
machines, they will print warning about window creation failing. To suppress those calls, set this
variable.

## AF\_SYNCHRONOUS\_CALLS

When this environment variable is set to 1, ArrayFire will execute all functions synchronously.

## AF\_SHOW\_LOAD\_PATH

When using the Unified backend, if this variable is set to 1, it will show the path where the ArrayFire
backend libraries are loaded from.

If the libraries are loaded from system paths, such as PATH or LD\_LIBRARY\_PATH etc, then it will
print "system path". If the libraries are loaded from other paths, then those paths are shown in full.

## AF\_MEM\_DEBUG

When AF\_MEM\_DEBUG is set to 1 (or anything not equal to 0), the caching mechanism in the memory manager
is disabled. The device buffers are allocated using native functions as needed and freed when going out
of scope. When the environment variable is not set, it is treated to be non zero.

```
AF_MEM_DEBUG=1 ./myprogram
```

## AF\_MAX\_BUFFERS

When AF\_MAX\_BUFFERS is set, this environment variable specifies the maximum number of buffers
allocated before garbage collection kicks in. Please note that the total number of buffers that
can exist simultaneously can be higher than this number. This variable tells the garbage collector
that it should free any available buffers immediately if the treshold is reached. When not set,
the default value is 1000.

## AF\_OPENCL\_MAX\_JIT\_LEN

When set, this environment variable specifies the maximum height of the OpenCL JIT tree after
which evaluation is forced. The default value, as of v3.4, is 50 on OSX, 100 everywhere else.
This value was 20 for older versions.

## AF\_CUDA\_MAX\_JIT\_LEN

When set, this environment variable specifies the maximum height of the CUDA JIT tree after
which evaluation is forced. The default value, as of v3.4, 100. This value was 20 for older versions.

## AF\_CPU\_MAX\_JIT\_LEN

When set, this environment variable specifies the maximum length of the CPU JIT tree after
which evaluation is forced. The default value, as of v3.4, 100. This value was 20 for older versions.

[1]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.info.html
[2]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.set_device.html
