# Indexing

Indexing in ArrayFire is a powerful but easy to abuse feature. This feature allows you to reference
or copy subsections of a larger array and perform operations on only a subset of elements.

[Indexer][1] structure is the key element used in Rust wrapper of ArrayFire for creating references
to existing Arrays. Given below are few of such functions and their corresponding use cases. Use
[Indexer::new][2] to create an Indexer object and set either a `Seq` object or `Array` as indexing
object for a given dimension.

## Using Seq objects

### Create a view of an existing Array

We will Sequences and the function [index][3] in this approach. 

```rust,noplaypen
{{#include ../../src/core/index.rs:non_macro_seq_index}}
```
However, the same above code can be condensed into a much terse syntax with the help of [view][11]
macro. Take a look at the above code modified to use view macro.
```rust, noplaypen
{{#include ../../src/core/index.rs:seq_index}}
```

### Modify a sub region of an existing Array

Let us take a look at an example where a portion of an existing Array will be set to with another
Array. We will an constant value Array and the function [assign\_seq][4] in the below example.

```rust,noplaypen
{{#include ../../src/core/index.rs:non_macro_seq_assign}}
```

> **NOTE** Normally you want to avoid accessing individual elements of the array like this for performance reasons.

## Using Array and Seq combination

### Create a view of an existing Array

To use a combination of Array and Seq objects to index an existing Array, we will need a more
generalized function [index\_gen][12].

```rust,noplaypen
{{#include ../../src/core/index.rs:non_macro_seq_array_index}}
```
Similar to how, [view][11] macro helped with abreviating the syntax when indexing with just
sequences, it can also help when using a combination of Seq and Array.
```rust, noplaypen
{{#include ../../src/core/index.rs:seq_array_index}}
```

### Modify a sub region of an existing Array

Set a portion of an existing Array with another Array using a combination of `Seq` and `Array`.
We will use [assign\_gen][13] function to do it.

 ```rust,noplaypen
{{#include ../../src/core/index.rs:non_macro_seq_array_assign}}
 ```

## Extract or Set rows/coloumns of an Array

Extract a specific set of rows/coloumns from an existing Array.

```rust,noplaypen
{{#include ../../src/core/index.rs:setrow}}
```

You can also use [rows][5] & [cols][6] to retrieve a subset of rows or coloumns respectively.

Similarly, [set\_row][7] & [set\_rows][9] can be used to change the values in a particular set of
rows using another Array. [set\_col][8] & [set\_cols][10] has same functionality, except that it is
for coloumns.

[1]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Indexer.html
[2]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Indexer.html#method.new
[3]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.index.html
[4]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.assign_seq.html
[5]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.rows.html
[6]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.cols.html
[7]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.set_row.html
[8]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.set_col.html
[9]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.set_rows.html
[10]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.set_cols.html
[11]: http://arrayfire.org/arrayfire-rust/arrayfire/macro.view.html
[12]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.index_gen.html
[13]: http://arrayfire.org/arrayfire-rust/arrayfire/fn.assign_gen.html
