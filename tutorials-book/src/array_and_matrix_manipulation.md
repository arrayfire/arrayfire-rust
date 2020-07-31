# Array and Matrix Manipulation

ArrayFire provides several different methods for manipulating arrays and matrices. The functionality
includes:

* [moddims()](#moddims) - change the dimensions of an array without changing the data
* [flat()](#flat) - flatten an array to one dimension
* [flip()](#flip) - flip an array along a dimension
* [join()](#join) - join up to 4 arrays
* [reorder()](#reorder) - changes the dimension order within the array
* [shift()](#shift) - shifts data along a dimension
* [tile()](#tile) - repeats an array along a dimension
* [transpose()](#transpose) - performs a matrix transpose

Below we provide several examples of these functions and their use.

### moddims()

The [moddims][1] function changes the dimensions of an array without changing its data or order.
Note that this function modifies only the _metadata_ associated with the array. It does not modify
the content of the array. Here is an example of moddims() converting an 8x1 array into a 2x4 and
then back to a 8x1:

```rust,noplaypen
a [8 1 1 1]
    1.0000
    2.0000
    1.0000
    2.0000
    1.0000
    2.0000
    1.0000
    2.0000

let new_dims = Dim4::new(&[2, 4, 1, 1]);
moddims(&a, new_dims)
[2 4 1 1]
    1.0000     1.0000     1.0000     1.0000
    2.0000     2.0000     2.0000     2.0000

let out = moddims(&a, a.elements(), 1, 1, 1);
[8 1 1 1]
    1.0000
    2.0000
    1.0000
    2.0000
    1.0000
    2.0000
    1.0000
    2.0000
```

### flat()

The [flat][2] function flattens an array to one dimension:

```
a [3 3 1 1]
    1.0000     4.0000     7.0000
    2.0000     5.0000     8.0000
    3.0000     6.0000     9.0000

flat(&a)
[9 1 1 1]
    1.0000
    2.0000
    3.0000
    4.0000
    5.0000
    6.0000
    7.0000
    8.0000
    9.0000
```

### flip()

The [flip][3] function flips the contents of an array along a chosen dimension. In the example
below, we show the 5x2 array flipped along the zeroth (i.e. within a column) and first (e.g.
across rows) axes:

```rust,noplaypen
a [5 2 1 1]
    1.0000     6.0000
    2.0000     7.0000
    3.0000     8.0000
    4.0000     9.0000
    5.0000    10.0000

flip(a, 0) [5 2 1 1]
    5.0000    10.0000
    4.0000     9.0000
    3.0000     8.0000
    2.0000     7.0000
    1.0000     6.0000

flip(a, 1) [5 2 1 1]
    6.0000     1.0000
    7.0000     2.0000
    8.0000     3.0000
    9.0000     4.0000
   10.0000     5.0000
```

### join()

The [join][4], [join\_many][5] functions can be used to join arrays along a specific dimension.

Here is an example of how to use join an array to itself:

```rust,noplaypen
a [5 1 1 1]
    1.0000
    2.0000
    3.0000
    4.0000
    5.0000

join(0, a, a) [10 1 1 1]
    1.0000
    2.0000
    3.0000
    4.0000
    5.0000
    1.0000
    2.0000
    3.0000
    4.0000
    5.0000

join(1, a, a) [5 2 1 1]
    1.0000     1.0000
    2.0000     2.0000
    3.0000     3.0000
    4.0000     4.0000
    5.0000     5.0000
```

### reorder()

The [reorder][6] function modifies the order of data within an array by exchanging data according to
the change in dimensionality. The linear ordering of data within the array is preserved.

```rust,noplaypen
a [2 2 3 1]
    1.0000     3.0000
    2.0000     4.0000

    1.0000     3.0000
    2.0000     4.0000

    1.0000     3.0000
    2.0000     4.0000


reorder(&a, 1, 0, 2)
[2 2 3 1]  //equivalent to a transpose
    1.0000     2.0000
    3.0000     4.0000

    1.0000     2.0000
    3.0000     4.0000

    1.0000     2.0000
    3.0000     4.0000


reorder(&a, 2, 0, 1)
[3 2 2 1]
    1.0000     2.0000
    1.0000     2.0000
    1.0000     2.0000

    3.0000     4.0000
    3.0000     4.0000
    3.0000     4.0000
```

### shift()

The [shift][7] function shifts data in a circular buffer fashion along a chosen dimension. Consider
the following example:

```rust,noplaypen
a [3 5 1 1]
    0.0000     0.0000     0.0000     0.0000     0.0000
    3.0000     4.0000     5.0000     1.0000     2.0000
    3.0000     4.0000     5.0000     1.0000     2.0000

shift(&a, 0, 2 )
[3 5 1 1]
    0.0000     0.0000     0.0000     0.0000     0.0000
    1.0000     2.0000     3.0000     4.0000     5.0000
    1.0000     2.0000     3.0000     4.0000     5.0000

shift(&a, -1, 2 )
[3 5 1 1]
    1.0000     2.0000     3.0000     4.0000     5.0000
    1.0000     2.0000     3.0000     4.0000     5.0000
    0.0000     0.0000     0.0000     0.0000     0.0000
```

### tile()

The [tile][8] function repeats an array along the specified dimension. For example below we show how
to tile an array along the zeroth and first dimensions of an array:

```rust,noplaypen
a [3 1 1 1]
    1.0000
    2.0000
    3.0000

// Repeat array a twice in the zeroth dimension
tile(&a, 2)
[6 1 1 1]
    1.0000
    2.0000
    3.0000
    1.0000
    2.0000
    3.0000

// Repeat array a twice along both the zeroth and first dimensions
tile(&a, 2, 2)
[6 2 1 1]
    1.0000     1.0000
    2.0000     2.0000
    3.0000     3.0000
    1.0000     1.0000
    2.0000     2.0000
    3.0000     3.0000

// Repeat array a twice along the first and three times along the second
// dimension.
let tile_dims = Dim4::new(&[1, 2, 3, 1]);
tile(a, tile_dims) [3 2 3 1]
    1.0000     1.0000
    2.0000     2.0000
    3.0000     3.0000

    1.0000     1.0000
    2.0000     2.0000
    3.0000     3.0000

    1.0000     1.0000
    2.0000     2.0000
    3.0000     3.0000
```

### transpose()

The [transpose][9] function performs a standard matrix transpose. The input array must have the
dimensions of a 2D-matrix.

```rust,noplaypen
a [3 3 1 1]
    1.0000     3.0000     3.0000
    2.0000     1.0000     3.0000
    2.0000     2.0000     1.0000

transpose(&a, False) //Second parameter to be used for conjugate transpose
[3 3 1 1]
    1.0000     2.0000     2.0000
    3.0000     1.0000     2.0000
    3.0000     3.0000     1.0000
```

### Combining functions to enumerate grid coordinates

By using a combination of the functions, one can quickly code complex manipulation patterns with
a few lines of code. For example, consider generating (x,y) coordinates for a grid where each axis
goes from 1 to n. Instead of using several loops to populate our arrays we can just use a small
combination of the above functions.

```rust,noplaypen
let a      = iota::<u32>(Dim4::new(&[3, 1, 1, 1]),
                         Dim4::new(&[1, 3, 1, 1]));
let b      = transpose(&a, false);
let coords = join(1, &flat(&a), &flat(&b));
print(&coords);
```

The output for a `[3 3 1 1]` matrix will be the following.
```rust,noplaypen
[9 2 1 1]
         0          0
         1          0
         2          0
         0          1
         1          1
         2          1
         0          2
         1          2
         2          2
```

[1]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.moddims.html
[2]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.flat.html
[3]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.flip.html
[4]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.join.html
[5]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.join_many.html
[6]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.reorder.html
[7]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.shift.html
[8]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.tile.html
[9]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.transpose.html
