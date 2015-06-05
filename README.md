# rust_arrayfire
Rust bindings for arrayfire

Currently this will most likely only work on OSX as the bindings were generated specifically for it.
To make it work for your system try building [bindgen](https://github.com/crabtw/rust-bindgen) and then run the following in the root directory:

```bash
bindgen  -I arrayfire/include -builtins -o src/arrayfire.rs arrayfire/include/arrayfire.h
```

After this you can simply:
```bash
cargo build
```

It does work for the first two examples in the hello_world arrayfire example:
```bash
jramapuram@gauss ~/p/rust_arrayfire> cargo run
     Running `target/debug/arrayfire`
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

## Automatic binding generation
I have commented out the auto binding generation as the api is rather terrible now.

It doesn't work because arrayfire.h includes af/*.h files.

To overcome this you need the `-I arrayfire/include` directive. 

I have yet to find how that works through the [bindgen API](https://github.com/crabtw/rust-bindgen/blob/master/src/lib.rs)
