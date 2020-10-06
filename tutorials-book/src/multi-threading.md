# ArrayFire in Threaded Applications

In this chapter, we will looking at how to use ArrayFire in multi-threaded programs. We shall
go over the details in the following order.

- [Move an Array to thread](#move-an-array-to-thread)
- [Read Array from Multiple threads](#read-array-from-multiple-threads)
- [Write to Array from Multiple threads](#write-to-array-from-multiple-threads)
- [Write to single Array using Channel](#write-to-single-array-using-channel)

## Move an Array to thread

In this section, we are going to create an Array on main thread and move it to a child thread,
modify it and then print it from the child thread.

```rust,noplaypen
{{#include ../../src/core/array.rs:move_array_to_thread}}
```

## Read Array from Multiple threads

Now, let's expand the earlier example to do a bunch of arithmetic operations in parallel on
multiple threads using the same Array objects.

```rust,noplaypen
{{#include ../../src/core/array.rs:read_from_multiple_threads}}
```

Given below is the definition of the enum `Op` we used in the example for illustration simplicity.
```rust,noplaypen
{{#include ../../src/core/array.rs:multiple_threads_enum_def}}
```

## Write to Array from Multiple threads

Let us further expand the earlier example by accumulating the results of the arithmetic operations
into a single Array object.

The code will differ from earlier section in couple of locations:

- In the main thread, we wrap the accumulating Array in a read-write lock (`std::sync::RwLock`)
  which is in turn wrapped in an atomically reference counted counter a.k.a `std::sync::Arc`.
- In the children threads, we use the guarded objects returned by RwLock's write method to access
  the accumulator Array.

```rust,noplaypen
{{#include ../../src/core/array.rs:access_using_rwlock}}
```

## Write to single Array using Channel

In this section, we shall modify the example to use channel instead of data sharing.

```rust,noplaypen
{{#include ../../src/core/array.rs:accum_using_channel}}
```
