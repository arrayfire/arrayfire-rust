# Vectorization

Programmers and Data Scientists want to take advantage of fast and parallel
computational devices. Writing vectorized code is necessary to get
the best performance out of the current generation parallel hardware and
scientific computing software. However, writing vectorized code may not be
immediately intuitive. ArrayFire provides many ways to vectorize a given code
segment. In this chapter, we present several methods to vectorize code
using ArrayFire and discuss the benefits and drawbacks associated with each method.

## Generic/Default vectorization
By its very nature, ArrayFire is a vectorized library. Most functions operate on
Arrays as a whole -- on all elements in parallel. For example consider the following code:

```rust,noplaypen
let mut a = af::range(Dim::new(&[10, 1, 1, 1]));  // [0,  9]
a = a + 1;                                        // [1, 10]
```

This code will result in a single backend kernel that operates on all 10 elements
of `a` in  parallel.

A small subset of such vectorized ArrayFire functions are given below for quick reference:

|  Operator Category                                           | Functions                  |
|--------------------------------------------------------------|----------------------------|
|  Arithmetic operations                    | +, -, \*, /, %, >>, << |
|  Logical operations                       | &&, \|\|, <, >, ==, != etc. |
|  Numeric functions                        | [abs](../fn.abs.html), [floor](../fn.floor.html), [round](../fn.round.html), [min](../fn.min.html), [max](../fn.max.html), etc. |
|  Complex operations                       | [real](../fn.real.html), [imag](../fn.imag.html), [conjg](../fn.conjg.html), etc. |
|  Exponential and logarithmic functions    | [exp](../fn.exp.html), [log](../fn.log.html), [expm1](../fn.expm1.html), [log1p](../fn.log1p.html), etc. |
|  Trigonometric functions                  | [sin](../fn.sin.html), [cos](../fn.cos.html), [tan](../fn.tan.html), etc. |
|  Hyperbolic functions                     | [sinh](../fn.sinh.html), [cosh](../fn.cosh.html), [tanh](../fn.tanh.html), etc. |

In addition to element-wise operations, many other functions are also
vectorized in ArrayFire.

Notice that even functions that perform some form of aggregation (e.g.
[sum](../fn.sum.html) or [min](../fn.min.html)), signal processing (like
[convolve](../fn.convolve1.html)), and image processing functions
(i.e. [rotate](../fn.rotate.html) etc.) - all support vectorization on
 different columns or images.

For example, if we have `NUM` images of size `WIDTH`x`HEIGHT`, one could
convolve each image in a vector fashion as follows:

```rust,noplaypen
let g_coef: [f32, 9] = { 1, 2, 1, 2, 4, 2, 1, 2, 1 };

let f = Array::new(g_coef, Dim4::new(&[3, 3, 1, 1]));
let filter = f * 1.0f32/16;

let signal = randu(WIDTH, HEIGHT, NUM);
let conv   = convolve2(signal, filter, ConvMode::DEFAULT, ConvDomain::AUTO);
```

Similarly, one can rotate 100 images by 45 degrees in a single call using
code like the following:

```rust,noplaypen
// Construct an array of 100 WIDTH x HEIGHT images of random numbers
let imgs = randu(WIDTH, HEIGHT, 100);

// Rotate all of the images in a single command
let rot_imgs = rotate(imgs, 45.0, False, InterpType::LINEAR);
```

Although *most* functions in ArrayFire do support vectorization, some do not.
Most notably, all linear algebra functions. Even though they are not vectorized
linear algebra operations, they still execute in parallel on your hardware.

Using the built in vectorized operations should be the first
and preferred method of vectorizing any code written with ArrayFire.

## GFOR

This construct is similar to gfor loop from C++ API of ArrayFire. It has not
been implemented in rust wrapper. This section will be updated once the feature
has been added to the crate.

## batch\_func

This another pending feature that is similar to our C++ API of
[batchFunc()](http://arrayfire.org/docs/namespaceaf.htm#aa0eb9e160f5be4b95234543e5c47934b)
