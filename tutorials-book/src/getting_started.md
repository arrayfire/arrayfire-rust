ArrayFire is a high performance software library for parallel computing with an easy-to-use API.
ArrayFire abstracts away much of the details of programming parallel architectures by providing
a high-level container object, the [Array][1], that represents data stored on a CPU, GPU, FPGA,
or other type of accelerator. This abstraction permits developers to write massively parallel
applications in a high-level language where they need not be concerned about low-level optimizations
that are frequently required to achieve high throughput on most parallel architectures.

## Supported data types

ArrayFire provides one generic container object, the [Array][1] on which functions and mathematical
operations are performed. The `Array` can represent one of many different [basic data types][2]:

* [F32][2] real single-precision (`float`)
* [C32][2] complex single-precision (`cfloat`)
* [F64][2] real double-precision (`double`)
* [C64][2] complex double-precision (`cdouble`)
* [B8 ][2] 8-bit boolean values (`bool`)
* [S32][2] 32-bit signed integer (`int`)
* [U32][2] 32-bit unsigned integer (`unsigned`)
* [U8 ][2] 8-bit unsigned values (`unsigned char`)
* [S64][2] 64-bit signed integer (`intl`)
* [U64][2] 64-bit unsigned integer (`uintl`)
* [S16][2] 16-bit signed integer (`short`)
* [U16][2] 16-bit unsigned integer (`unsigned short`)
* [F16][2] 16-bit floating point number ([`half::f16`][3])

Most of these data types are supported on all modern GPUs; however, some older devices may lack
support for double precision arrays. In this case, a runtime error will be generated when the array
is constructed. 

If not specified, `Array`s are created as single precision floating point numbers ([F32][2]).

## Creating and populating an Array

ArrayFire [Array][1]'s represent memory stored on the device. As such, creation and population of
an array will consume memory on the device which cannot freed until the `array` object goes out of
scope. As device memory allocation can be expensive, ArrayFire also includes a memory manager which
will re-use device memory whenever possible.

Arrays can be created using one of the [array constructors][4]. Below we show how to create 1D, 2D,
and 3D arrays with uninitialized values:

```rust,noplaypen
let garbageVals = Array::new_empty(Dim4::new(&[3, 1, 1, 1]), DType::F32);
```

However, uninitialized memory is likely not useful in your application. ArrayFire provides several
convenient functions for creating arrays that contain pre-populated values including constants,
uniform random numbers, uniform normally distributed numbers, and the identity matrix:

```rust,noplaypen
// Create an array filled with constant value of 2.0 of type floating point
// The type of Array is infered from the type of the constant argument
let cnst = constant(2.0f32, Dim4::new(&[5, 5, 1, 1]));
print(&cnst);
```
```rust,noplaypen
println!("Create a 5-by-3 matrix of random floats on the GPU");
let dims = Dim4::new(&[5, 3, 1, 1]);
let a = randu::<f32>(dims);
print(&a);
```

As stated above, the default data type for arrays is [F32][2](32-bit floating point number) unless
specified otherwise.

ArrayFire `Array`s may also be populated from data found on the host. For example:

```rust,noplaypen
let values: [u32; 3] = [1u32, 2, 3];
let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
print(&indices);
```

## Properties of an Array

ArrayFire provides several functions to determine various aspects of arrays. This includes
functions to print the contents, query dimensions, and determine various other aspects of arrays.

The [print][5] function can be used to print arrays that have already been generated or any
expression involving arrays:

```rust,noplaypen
let values: [f32; 3] = [1.0, 2.0, 3.0];
let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
print(&indices);
```

The dimensions of an array may be determined using either a [Dim4][6] object or by accessing the
dimensions directly using the [Dim4::get][7] and [Dim4::numdims][8] functions:

```rust,noplaypen
let values: [f32; 3] = [1.0, 2.0, 3.0];
let dims: Dim4 = Dim4::new(&[3, 1, 1, 1]);
let indices = Array::new(&values, dims);
println!("Dims {:?} with dimensions {}", dims.get(), dims.ndims());
```

In addition to dimensions, arrays also carry several properties including methods to determine the
underlying type and size (in bytes). You can even determine whether the array is empty, real/complex,
a row/column, or a scalar or a vector. For further information on these capabilities, we suggest you
consult the full documentation on the [Array][1].

## Writing math expressions using ArrayFire

ArrayFire features an intelligent Just-In-Time (JIT) compilation engine that converts expressions
using arrays into the smallest number of CUDA/OpenCL kernels. For most operations on Arrays,
ArrayFire functions like a vector library. That means that an element-wise operation, like
`c[i] = a[i] + b[i]` in C, would be written more concisely without indexing, like `c = a + b`. When
there are multiple expressions involving arrays, ArrayFire's JIT engine will merge them together.
his "kernel fusion" technology not only decreases the number of kernel calls, but, more importantly, avoids extraneous global memory operations.

Our JIT functionality extends across C API boundary and only ends when a non-JIT function is
encountered or a synchronization operation is explicitly called by the code.

ArrayFire provides hundreds of functions for element-wise operations. All of the standard operators
(e.g. +,-,\*,/) are supported as are most transcendental functions (sin, cos, log, sqrt, etc.). Here are a few examples:

```rust,noplaypen
let num_rows: u64 = 5;
let num_cols: u64 = 3;
let dims = Dim4::new(&[num_rows, num_cols, 1, 1]);
let a = randu::<f32>(dims);
let b = randu::<f32>(dims);
print(&a);
print(&b);
let c = a + b;
print(&c);

//Example of *Assign traits
let mut d = randu::<f32>(dims);
let e     = constant(1f32, dims);
d += e;
print(&d);
```

## Indexing

Like all functions in ArrayFire, indexing is also executed in parallel on the OpenCL/CUDA device. To
index `Array`s you may use one or a combination of the following functions:

* [Seq][9] representing a linear sequence
* [Seq::Default()][9] representing the entire dimension
* [row(&Array, i)][10] or [col(&Array, i)][11] specifying a single row/column
* [rows(&Array, first,last)][12] or [cols(&Array, first,last)][13] specifying a span of rows or columns

Please see the [indexing page](./indexing.md) for several examples of how to use these functions.

## Access to Array memory on the host

Memory in `af::Array`s may be accessed using the [Array::host()][14] method. The `host` function
*copies* the data from the device and makes it available in a standard slice or similar container on
the host. As such, it is up to the developer to manage any memory returned by `host`.

<!--
Lastly, if you want only the first value from an `af::array` you can use
get it using the [scalar()](\ref af::array::scalar) function:
```rust,noplaypen
```
-->

## Bitwise operators

In addition to supporting standard mathematical functions, Arrays that contain integer data types
also support bitwise operators including and, or, and shift etc. Operator traits for Array as well
as separate functions are also defined to support various use cases.

```rust,noplaypen
let dims = Dim4::new(&[5, 3, 1, 1]);
let a = randu::<bool>(dims);
let b = randu::<bool>(dims);

print(&a);
print(&b);

let c = &a | &b; //Borrowing to avoid move of a and b, a | b is also valid
let d = bitand(&a, &b, false);

print(&c);
print(&d);
```

## Where to go for help?

* [Google Groups][15]
* ArrayFire Services: [Consulting][16] | [Support][17] | [Training][18]
* [ArrayFire Blogs][19]
* Email: <mailto:technical@arrayfire.com>

[1]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html
[2]: http://arrayfire.org/arrayfire-rust/arrayfire/enum.DType.html
[3]: https://crates.io/crates/half
[4]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html#method.new_empty
[5]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.print.html
[6]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Dim4.html
[7]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Dim4.html#method.get
[8]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Dim4.html#method.ndims
[9]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Seq.html
[10]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.row.html
[11]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.col.html
[12]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.rows.html
[13]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.cols.html
[14]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Dim4.html#method.host
[15]: https://groups.google.com/forum/#!forum/arrayfire-users
[16]: http://arrayfire.com/consulting/
[17]: http://arrayfire.com/support/
[18]: http://arrayfire.com/training/
[19]: http://arrayfire.com/blog/
