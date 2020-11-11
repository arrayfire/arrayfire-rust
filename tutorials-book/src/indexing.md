# Indexing

Indexing in ArrayFire is a powerful but easy to abuse feature. This feature allows you to reference
or copy subsections of a larger array and perform operations on only a subset of elements.

This chapter is split into the following sections:
- [Index an Array using Seq Objects](#using-seq-objects)
    - [Create a view of an existing Array](#create-a-view-of-an-existing-array)
    - [Modify a sub region of an existing Array](#modify-a-sub-region-of-an-existing-array)
- [Using Array and Seq combination](#using-array-and-seq-combination)
    - [Create a view of an existing Array](#create-a-view-of-an-existing-array)
    - [Modify a sub region of an existing Array](#modify-a-sub-region-of-an-existing-array)
- [Extract or Set rows/columns of an Array](#extract-or-set-rowscolumns-of-an-array)
- [Negative Indices](#negative-indices)

[Indexer][1] structure is the key element used in Rust wrapper of ArrayFire for creating references
to existing Arrays. The above sections illustrate how it can be used in conjunction with `Seq`
and/or `Array`. Apart from that, each section also showcases a macro based equivalent
code(if one exists) that is more terse in syntax but offers the same functionality.

## Using Seq objects

### Create a view of an existing Array

We will Sequences and the function [index][3] in this approach. 

```rust,noplaypen
{{#include ../../src/core/index.rs:non_macro_seq_index}}
```
However, the same above code can be condensed into a much terse syntax with the help of [view][11]
macro. Take a look at the following two approaches using view macro.
```rust, noplaypen
{{#include ../../src/core/index.rs:seq_index}}
```
<div style="text-align: center"> OR </div>

```rust, noplaypen
{{#include ../../src/core/macros.rs:seq_view2}}
```

### Modify a sub region of an existing Array

Let us take a look at an example where a portion of an existing Array will be set to with another
Array. We will an constant value Array and the function [assign\_seq][4] in the below example.

```rust,noplaypen
{{#include ../../src/core/index.rs:non_macro_seq_assign}}
```

A much terser way of doing the same using macro is shown below
```rust,noplaypen
{{#include ../../src/core/macros.rs:macro_seq_assign}}
```

> **NOTE** Normally you want to avoid accessing individual elements of the array like this for performance reasons.

## Using Array and Seq combination

### Create a view of an existing Array

To use a combination of Array and Seq objects to index an existing Array, we will need a more
generalized function [index\_gen][12].

```rust,noplaypen
{{#include ../../src/core/index.rs:non_macro_seq_array_index}}
```
Similar to how [view][11] macro helped with abreviating the syntax when indexing with just
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
<div style="text-align: center"> OR </div>

 ```rust,noplaypen
{{#include ../../src/core/macros.rs:macro_seq_array_assign}}
 ```

## Extract or Set rows/columns of an Array

Extract a specific set of rows/coloumns from an existing Array.

```rust,noplaypen
{{#include ../../src/core/index.rs:setrow}}
```

You can also use [rows][5] & [cols][6] to retrieve a subset of rows or coloumns respectively.

Similarly, [set\_row][7] & [set\_rows][9] can be used to change the values in a particular set of
rows using another Array. [set\_col][8] & [set\_cols][10] has same functionality, except that it is
for coloumns.

## Negative Indices

Negative indices can also be used to refer elements from the end of a given axis. Negative value for
a row/column/slice will fetch corresponding row/column/slice in reverse order. Given below are some
examples that showcase getting row(s)/col(s) from an existing Array.

```rust,noplaypen
{{#include ../../src/core/index.rs:get_row}}
```

```rust,noplaypen
{{#include ../../src/core/index.rs:get_rows}}
```

[1]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Indexer.html
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
