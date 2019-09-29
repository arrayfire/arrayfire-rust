# Vectorization

<!--
Programmers and Data Scientists want to take advantage of fast and parallel
computational devices. Writing vectorized code is necessary to get
therust,noplayperust,noplaypen performance out of the current generation parallel hardware and
scientific computing software. However, writing vectorized code may not be
immediately intuitive. ArrayFire provides many ways to vectorize a given code
segment. In this tutorial, we present several methods to vectorize code
using ArrayFire and discuss the benefits and drawbacks associated with each method.

### Generic/Default vectorization
-->
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
|  Arithmetic operations                    | +, -, *, /, %, >>, << |
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
linear algebra operations still execute in parallel on your hardware.

Using the built in vectorized operations should be the first
and preferred method of vectorizing any code written with ArrayFire.

<!--
# Batching

The batchFunc() function allows the broad application of existing ArrayFire
functions to multiple sets of data. Effectively, batchFunc() allows ArrayFire
functions to execute in "batch processing" mode. In this mode, functions will
find a dimension which contains "batches" of data to be processed and will
parallelize the procedure.

Consider the following example. Here we create a filter which we would like
to apply to each of the weight vectors. The naive solution would be using a
for-loop as we have seen previously:

```rust,noplaypen
// Create the filter and the weight vectors
af::array filter = randn(1, 5);
af::array weights = randu(5, 5);

// Apply the filter using a for-loop
af::array filtered_weights = constant(0, 5, 5);
for(int i=0; i<weights.dims(1); ++i){
    filtered_weights.col(i) = filter * weights.col(i);
}
```

However, as we have discussed above, this solution will be very inefficient.
One may be tempted to implement a vectorized solution as follows:

```rust,noplaypen
// Create the filter and the weight vectors
af::array filter = randn(1, 5);
af::array weights = randu(5, 5);

af::array filtered_weights = filter * weights; // fails due to dimension mismatch
```

However, the dimensions of `filter` and `weights` do not match, thus ArrayFire
will generate a runtime error.

`batchfunc()` was created to solve this specific problem.
The signature of the function is as follows:

```
array batchFunc(const array &lhs, const array &rhs, batchFunc_t func);
```

where `__batchFunc_t__` is a function pointer of the form:

```
typedef array (*batchFunc_t) (const array &lhs, const array &rhs);
```

So, to use batchFunc(), we need to provide the function we wish to apply as a
batch operation. For illustration's sake, let's "implement" a multiplication
function following the format.

```
af::array my_mult (const af::array &lhs, const af::array &rhs){
    return lhs * rhs;
}
```

Our final batch call is not much more difficult than the ideal
syntax we imagined.

```
// Create the filter and the weight vectors
af::array filter = randn(1, 5);
af::array weights = randu(5, 5);

// Apply the batch function
af::array filtered_weights = batchFunc( filter, weights, my_mult );
```

The batch function will work with many previously mentioned vectorized ArrayFire
functions. It can even work with a combination of those functions if they are
wrapped inside a helper function matching the `__batchFunc_t__` signature.
One limitation of `batchfunc()` is that it cannot be used from within a
`gfor()` loop at the present time.

# Advanced Vectorization

We have seen the different methods ArrayFire provides to vectorize our code. Tying
them all together is a slightly more involved process that needs to consider data
dimensionality and layout, memory usage, nesting order, etc. An excellent example
and discussion of these factors can be found on our blog:

http://arrayfire.com/how-to-write-vectorized-code/

It's worth noting that the content discussed in the blog has since been transformed
into a convenient af::nearestNeighbour() function. Before writing something from
scratch, check that ArrayFire doesn't already have an implementation. The default
vectorized nature of ArrayFire and an extensive collection of functions will
speed things up in addition to replacing dozens of lines of code!
-->
