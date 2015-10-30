# Arrayfire Rust Bindings

The wrapper is currently compliant with ArrayFire 3.0 API. You can find the documentation [here](http://arrayfire.github.io/arrayfire-rust/arrayfire/index.html). If you find any bugs, please report them [here](https://github.com/arrayfire/arrayfire-rust/issues).

## Building & Running

Edit [build.conf](build.conf) to modify the build flags. The structure is a simple JSON blob.
Currently Rust does not allow key:value pairs to be passed from the CLI.
To use an existing arrayfire installation modify the first three JSON values.

To build arrayfire:

```bash
git submodule update --init --recursive
cargo build
```

To run hello world example:

```bash
~/p/arrayfire_rust> cargo run --example helloworld
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
sin(a) + 1.5 =>
[5 3 1 1]
    2.1744     1.9317     2.2006
    2.2962     2.1189     1.7905
    1.5390     1.6097     2.1549
    2.3243     1.9531     1.8509
    2.2987     1.9910     2.1299

sin(a) + cos(a) =>
[5 3 1 1]
    1.4128     1.3337     1.4142
    1.4012     1.4044     1.2474
    1.0382     1.1037     1.4106
    1.3905     1.3446     1.2873
    1.4004     1.3621     1.4066

!a =>
[5 3 1 1]
         1          1          1
         1          1          1
         1          1          1
         1          1          1
         1          1          1

a + b
[5 3 1 1]
    2.9147     2.3780     2.9767
    3.2172     2.7862     2.0853
    1.5780     1.7196     2.8689
    3.2933     2.4233     2.2094
    3.2238     2.5042     2.8113

Fourier transform the result
[5 3 1 1]
         (10.6327,0.0000)         (9.6043,0.0000)          (10.1267,0.0000)
         (0.4689,0.4640)          (0.3193,0.0802)          (0.1713,0.1441)
         (-0.3491,-0.7454)        (-0.2923,-0.4018)        (0.2667,0.4886)
         (-0.3491,0.7454)         (-0.2923,0.4018)         (0.2667,-0.4886)
         (0.4689,-0.4640)         (0.3193,-0.0802)         (0.1713,-0.1441)

Create 2-by-3 matrix from host data
[2 3 1 1]
         1          3          5
         2          4          6

Sort A and print sorted array and corresponding indices
[5 3 1 1]
    0.0390     0.1099     0.2948
    0.7402     0.4464     0.3585
    0.9210     0.4702     0.6814
    0.9251     0.5132     0.7140
    0.9690     0.6673     0.7762

[5 3 1 1]
         2          2          1
         0          0          3
         1          3          4
         4          4          2
         3          1          0

u8 constant array
[5 3 1 1]
         1          1          1
         1          1          1
         1          1          1
         1          1          1
         1          1          1

Is u8_cnst array float precision type ? false
```

## Issues

You might see something along the lines of :

```bash
dyld: Library not loaded: @rpath/libafopencl.3.dylib
```

You need to add the location of libaf.{dylib, so, dll} to your LD_LIBRARY_PATH.
