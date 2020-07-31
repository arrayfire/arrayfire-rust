# Vectorization

Programmers and Data Scientists want to take advantage of fast and parallel computational devices.
Writing vectorized code is necessary to get the best performance out of the current generation
parallel hardware and scientific computing software. However, writing vectorized code may not be
immediately intuitive. ArrayFire provides many ways to vectorize a given code segment. In this
chapter, we present several methods to vectorize code using ArrayFire and discuss the benefits and
drawbacks associated with each method.

## Generic/Default vectorization
By its very nature, ArrayFire is a vectorized library. Most functions operate on Arrays as a whole
i.e. on all elements in parallel. For example consider the following code:

```rust,noplaypen
let mut a = af::range(Dim::new(&[10, 1, 1, 1]));  // [0,  9]
a = a + 1;                                        // [1, 10]
```

This code will result in a single kernel that operates on all 10 elements of `a` in  parallel.

A small subset of such vectorized ArrayFire functions are given below for quick reference:

|  Operator Category               | Functions                                                  |
|----------------------------------|------------------------------------------------------------|
|  Arithmetic operations           | +, -, \*, /, %, >>, <<                                     |
|  Logical operations              | &&, \|\|, <, >, ==, != etc.                                |
|  Numeric functions               | [abs][1], [floor][2], [round][3], [min][4], [max][5], etc. |
|  Complex operations              | [real][6], [imag][7], [conjg][8], etc.                     |
|  Exponential and logarithmic fns | [exp][9], [log][10], [expm1][11], [log1p][12], etc.        |
|  Trigonometric functions         | [sin][13], [cos][14], [tan][15], etc.                      |
|  Hyperbolic functions            | [sinh][16], [cosh][17], [tanh][18], etc.                   |

In addition to element-wise operations, many other functions are also vectorized in ArrayFire.

Notice that even functions that perform some form of aggregation (e.g. [sum][19] or [min][14]),
signal processing (like [convolve][20]), and image processing functions (i.e. [rotate][21] etc.)
- all support vectorization on  different columns or images.

For example, if we have `NUM` images of size `WIDTH`x`HEIGHT`, one could convolve each image in a
vector fashion as follows:

```rust,noplaypen
let g_coef: [f32, 9] = { 1, 2, 1, 2, 4, 2, 1, 2, 1 };

let f = Array::new(g_coef, Dim4::new(&[3, 3, 1, 1]));
let filter = f * 1.0f32/16;

let signal = randu(WIDTH, HEIGHT, NUM);
let conv   = convolve2(signal, filter, ConvMode::DEFAULT, ConvDomain::AUTO);
```

Similarly, one can rotate 100 images by 45 degrees in a single call using code like the following:

```rust,noplaypen
// Construct an array of 100 WIDTH x HEIGHT images of random numbers
let imgs = randu(WIDTH, HEIGHT, 100);

// Rotate all of the images in a single command
let rot_imgs = rotate(imgs, 45.0, False, InterpType::LINEAR);
```

Although *most* functions in ArrayFire do support vectorization, some do not. Most notably, all
linear algebra functions. Even though they are not vectorized linear algebra operations, they still
execute in parallel on your hardware.

Using the built in vectorized operations should be the first and preferred method of vectorizing any
code written with ArrayFire.

## GFOR

This construct is similar to gfor loop from C++ API of ArrayFire. It has not been implemented in
rust wrapper. This section will be updated once the feature has been added to the crate.

## batch\_func

This another pending feature that is similar to our C++ API of [batchFunc()][22]

[1]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.abs.html
[2]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.floor.html
[3]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.round.html
[4]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.min.html
[5]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.max.html
[6]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.real.html
[7]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.imag.html
[8]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.conjg.html
[9]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.exp.html
[10]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.log.html
[11]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.expm1.html
[12]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.log1p.html
[13]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.sin.html
[14]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.cos.html
[15]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.tan.html
[16]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.sinh.html
[17]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.cosh.html
[18]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.tanh.html
[19]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.sum.html
[20]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.convolve1.html
[21]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.rotate.html
[22]: http://arrayfire.org/docs/namespaceaf.htm#aa0eb9e160f5be4b95234543e5c47934b
