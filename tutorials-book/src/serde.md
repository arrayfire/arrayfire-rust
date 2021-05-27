# Serialization & Deserialization of ArrayFire Objects

To save [Array][1] contents, shape in JSON format, it just takes couple of lines of code as shown below:
```rust,noplaypen
{{#include ../../src/core/array.rs:array_json_serde_snippet}}
```
Saving [Array][1] in different formats is as simple as changing the object qualifier of methods `serialize` and `deserialize`. For example, if user wants to store [Array][1] in `bincode` format instead of JSON, the above code only needs to be change in couple of lines.
```rust,noplaypen
{{#include ../../src/core/array.rs:array_bincode_serde_snippet}}
```

In similar fashion, we can serialize and deserialize [Dim4][2], [RandomEngine][3], [Seq][4] and other Enums. Examples of [Dim4][2], [RandomEngine][3] and [Seq][4] are given below.

```rust,noplaypen
{{#include ../../src/core/dim4.rs:dim4_json_serde_snippet}}
```

```rust,noplaypen
{{#include ../../src/core/random.rs:rng_bincode_serde_snippet}}
```

```rust,noplaypen
{{#include ../../src/core/seq.rs:seq_json_serde_snippet}}
```

[1]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Array.html
[2]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Dim4.html
[3]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.RandomEngine.html
[4]: http://arrayfire.org/arrayfire-rust/arrayfire/struct.Seq.html
