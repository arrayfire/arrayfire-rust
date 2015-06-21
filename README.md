# Arrayfire Rust Bindings

This project uses [bindgen](https://github.com/crabtw/rust-bindgen) to build an arrayfire binding that is usable in Rust. This project is currently in it's infancy and there will be quite a few issues to work through.

Currently the example creates an arrayfire array structure and runs the first two examples listed in hello_world.

## Automatic binding generation

Bindgen automatically iterates from the root arrayfire.h header & pulls in all the required C includes. This is currently blobbed into one huge arrayfire.rs file. 

In the future we will try to provide a nicer Rust-wrapped version of this header so that the code does not look too verbose.

## Building & Running

Edit [build.conf](build.conf) to modify the build flags. The structure is a simple JSON blob.
Currently Rust does not allow key:value pairs to be passed from the CLI.
To use an existing arrayfire installation modify the first three JSON values.

To build:

```bash
git submodule update --init --recursive
cargo build
```

To test:

```bash
~/p/arrayfire_rust> cargo test
...
     running 1 test
ArrayFire v3.0.0 (CUDA, 64-bit Mac OSX, build d8d4b38)
Platform: CUDA Toolkit 7, Driver: CUDA Driver Version: 7000
[0] GeForce GT 750M, 2048 MB, CUDA Compute 3.0
Create a 5-by-3 matrix of random floats on the GPU
[5 3 1 1]
    0.7402     0.4464     0.7762
    0.9210     0.6673     0.2948
    0.0390     0.1099     0.7140
    0.9690     0.4702     0.3585
    0.9251     0.5132     0.6814

Element-wise arithmetic
[5 3 1 1]
    0.6744     0.4317     0.7006
    0.7962     0.6189     0.2905
    0.0390     0.1097     0.6549
    0.8243     0.4531     0.3509
    0.7987     0.4910     0.6299
```

## Issues

You might see something along the lines of :

```bash
dyld: Library not loaded: @rpath/libafopencl.3.dylib
```

This is related to this (Rust issue)[https://github.com/rust-lang/rust/issues/25185]
A workaround for now is to add the location of libaf*.dylib to your LD_LIBRARY_PATH.
