ArrayFire is a high performance software library for parallel computing with
an easy-to-use API. ArrayFire abstracts away much of the details of
programming parallel architectures by providing a high-level container object,
the [Array](../struct.Array.html), that represents data stored on a CPU, GPU, FPGA,
or other type of accelerator. This abstraction permits developers to write
massively parallel applications in a high-level language where they need
not be concerned about low-level optimizations that are frequently required to
achieve high throughput on most parallel architectures.

## Supported data types

ArrayFire provides one generic container object, the [Array](../struct.Array.html)
on which functions and mathematical operations are performed. The `Array`
can represent one of many different [basic data types](../enum.DType.html):

* [F32](../enum.DType.html) real single-precision (`float`)
* [C32](../enum.DType.html) complex single-precision (`cfloat`)
* [F64](../enum.DType.html) real double-precision (`double`)
* [C64](../enum.DType.html) complex double-precision (`cdouble`)
* [B8 ](../enum.DType.html) 8-bit boolean values (`bool`)
* [S32](../enum.DType.html) 32-bit signed integer (`int`)
* [U32](../enum.DType.html) 32-bit unsigned integer (`unsigned`)
* [U8 ](../enum.DType.html) 8-bit unsigned values (`unsigned char`)
* [S64](../enum.DType.html) 64-bit signed integer (`intl`)
* [U64](../enum.DType.html) 64-bit unsigned integer (`uintl`)
* [S16](../enum.DType.html) 16-bit signed integer (`short`)
* [U16](../enum.DType.html) 16-bit unsigned integer (`unsigned short`)

Most of these data types are supported on all modern GPUs; however, some
older devices may lack support for double precision arrays. In this case,
a runtime error will be generated when the array is constructed.

If not specified otherwise, `Array`s are created as single precision floating
point numbers ([F32](../enum.DType.html)).

## Creating and populating an Array

ArrayFire [Array](../struct.Array.html)s represent memory stored on the device.
As such, creation and population of an array will consume memory on the device
which cannot freed until the `array` object goes out of scope. As device memory
allocation can be expensive, ArrayFire also includes a memory manager which
will re-use device memory whenever possible.

Arrays can be created using one of the [array constructors](../struct.Array.html#method.new_empty).
Below we show how to create 1D, 2D, and 3D arrays with uninitialized values:

```rust,noplaypen
let garbageVals = Array::new_empty(Dim4::new(&[3, 1, 1, 1]), DType::F32);
```

However, uninitialized memory is likely not useful in your application.
ArrayFire provides several convenient functions for creating arrays that contain
pre-populated values including constants, uniform random numbers, uniform
normally distributed numbers, and the identity matrix:

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

As stated above, the default data type for arrays is [F32](../enum.DType.html) (a
32-bit floating point number) unless specified otherwise.

ArrayFire `Array`s may also be populated from data found on the host.
For example:

```rust,noplaypen
let values: [u32; 3] = [1u32, 2, 3];
let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
print(&indices);
```

<!--
ArrayFire also supports array initialization from memory already on the GPU.
For example, with CUDA one can populate an `array` directly using a call
to `cudaMemcpy`:

\snippet test/getting_started.cpp ex_getting_started_dev_ptr

Similar functionality exists for OpenCL too. If you wish to intermingle
ArrayFire with CUDA or OpenCL code, we suggest you consult the
[CUDA interoperability](\ref interop_cuda) or
[OpenCL interoperability](\ref interop_opencl) pages for detailed instructions.
-->

## Properties of an Array

ArrayFire provides several functions to determine various aspects of arrays.
This includes functions to print the contents, query the dimensions, and
determine various other aspects of arrays.

The [print](../fn.print.html) function can be used to print arrays that
have already been generated or any expression involving arrays:

```rust,noplaypen
let values: [f32; 3] = [1.0, 2.0, 3.0];
let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
print(&indices);
```

The dimensions of an array may be determined using either a [Dim4](../struct.Dim4.html) object or by accessing the dimensions directly using the [Dim4::get](../struct.Dim4.html#method.get) and [Dim4::numdims](../struct.Dim4.html#method.ndims) functions:

```rust,noplaypen
let values: [f32; 3] = [1.0, 2.0, 3.0];
let dims: Dim4 = Dim4::new(&[3, 1, 1, 1]);
let indices = Array::new(&values, dims);
println!("Dims {:?} with dimensions {}", dims.get(), dims.ndims());
```

In addition to dimensions, arrays also carry several properties including
methods to determine the underlying type and size (in bytes). You can even
determine whether the array is empty, real/complex, a row/column, or a scalar
or a vector. For further information on these capabilities, we suggest you consult the
full documentation on the [Array](../struct.Array.html).

## Writing math expressions using ArrayFire

ArrayFire features an intelligent Just-In-Time (JIT) compilation engine that
converts expressions using arrays into the smallest number of CUDA/OpenCL
kernels. For most operations on Arrays, ArrayFire functions like a vector library.
That means that an element-wise operation, like `c[i] = a[i] + b[i]` in C,
would be written more concisely without indexing, like `c = a + b`.
When there are multiple expressions involving arrays, ArrayFire's JIT engine
will merge them together. This "kernel fusion" technology not only decreases
the number of kernel calls, but, more importantly, avoids extraneous global
memory operations.

Our JIT functionality extends across C API boundary and only ends
when a non-JIT function is encountered or a synchronization operation is
explicitly called by the code.

ArrayFire provides hundreds of functions for element-wise
operations. All of the standard operators (e.g. +,-,\*,/) are supported
as are most transcendental functions (sin, cos, log, sqrt, etc.).
Here are a few examples:

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

<!--To see the complete list of functions please refer to the documentation on
[mathematical](\ref mathfunc_mat), [linear algebra](\ref linalg_mat),
[signal processing](\ref signal_mat), and [statistics](\ref stats_mat).
-->

## Indexing

Like all functions in ArrayFire, indexing is also executed in parallel on
the OpenCL/CUDA device. To index `Array`s you may use one or a combination of the following functions:

* [Seq](../struct.Seq.html) representing a linear sequence
* [Seq::Default()](../struct.Seq.html) representing the entire dimension
* [row(&Array, i)](../fn.row.html) or [col(&Array, i)](../fn.col.html) specifying a single row/column
* [rows(&Array, first,last)](../fn.rows.html) or [cols(&Array, first,last)](../fn.cols.html)
 specifying a span of rows or columns

Please see the [indexing page](./indexing.md) for several examples of how to
use these functions.

## Access to Array memory on the host

Memory in `af::Array`s may be accessed using the [Array::host()](../struct.Array.html#method.host) method.
The `host` function *copies* the data from the device and makes it available
in a standard slice or similar container on the host. As such, it is up to the developer to manage
any memory returned by `host`.


<!--
# Getting access to ArrayFire array memory on the host and device

Memory in `af::array`s may be accessed using the [host()](\ref af::array::host)
and [device()](\ref af::array::device) functions.
The `host` function *copies* the data from the device and makes it available
in a C-style array on the host. As such, it is up to the developer to manage
any memory returned by `host`.
The `device` function returns a pointer/reference to device memory for
interoperability with external CUDA/OpenCL kernels. As this memory belongs to
ArrayFire, the programmer should not attempt to free/deallocate the pointer.
For example, here is how we can interact with both OpenCL and CUDA:

```rust,noplaypen
```

ArrayFire also provides several helper functions for creating `af::array`s from
OpenCL `cl_mem` references and `cl::Buffer` objects. See the `include/af/opencl.h`
file for further information.

Lastly, if you want only the first value from an `af::array` you can use
get it using the [scalar()](\ref af::array::scalar) function:

```rust,noplaypen
```
-->

## Bitwise operators

In addition to supporting standard mathematical functions, Arrays
that contain integer data types also support bitwise operators including
and, or, and shift etc. Operator traits for Array as well as separate functions
are also defined to support various use cases.

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

* [Google Groups](https://groups.google.com/forum/#!forum/arrayfire-users)
* ArrayFire Services:  [Consulting](http://arrayfire.com/consulting/)  |  [Support](http://arrayfire.com/support/)   |  [Training](http://arrayfire.com/training/)
* [ArrayFire Blogs](http://arrayfire.com/blog/)
* Email: <mailto:technical@arrayfire.com>
