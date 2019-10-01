# Arrayfire Rust Bindings

[ArrayFire](https://github.com/arrayfire/arrayfire) is a high performance
library for parallel computing with an easy-to-use API. It enables users
to write scientific computing code that is portable across CUDA, OpenCL
and CPU devices. This project provides Rust bindings for the ArrayFire
library. Given below table shows the rust bindings compatability with
ArrayFire.  If you find any bugs, please report them
[here](https://github.com/arrayfire/arrayfire-rust/issues).

| arrayfire-rust | ArrayFire |
|:--------------:|:---------:|
|         M.m.p1 |    M.m.p2 |

Only, Major(M) & Minor(m) version numbers need to match. *p1* and *p2*
are patch/fix updates for `arrayfire-rust` & `ArrayFire` respectively,
and they don't need to match.

## Documentation

You can find the most recent updated documentation
[here](http://arrayfire.github.io/arrayfire-rust/arrayfire/index.html).

## Communication

* [Slack Chat](https://join.slack.com/t/arrayfire-org/shared_invite/MjI4MjIzMDMzMTczLTE1MDI5ODg4NzYtN2QwNGE3ODA5OQ)
* [Google Groups](https://groups.google.com/forum/#!forum/arrayfire-users)

## Supported platforms

Linux, Windows and OSX. Rust 1.15.1 or higher is required.

## Use from Crates.io [![](http://meritbadge.herokuapp.com/arrayfire)](https://crates.io/crates/arrayfire) [![](https://docs.rs/arrayfire/badge.svg)](https://docs.rs/arrayfire)

To use the rust bindings for ArrayFire from crates.io, the following
requirements are to be met first.

1. [Download and install ArrayFire binaries](https://arrayfire.com/download)
   based on your operating system.
2. Set the evironment variable `AF_PATH` to point to ArrayFire installation
   root folder.
3. Make sure to add the path to lib files to your path environment variables.
    - On Linux: do `export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$AF_PATH/lib64`
    - On OSX: do `export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:$AF_PATH/lib64`
    - On Windows: Add `%AF_PATH%\lib` to your PATH environment variable.
4. Add `arrayfire = "3.6"` to the dependencies section of your project's
   Cargo.toml file. Make sure to change the version to latest available.

Once step (4) is over, you should be able to use ArrayFire in your Rust
project. If you find any bugs, please report them
[here](https://github.com/arrayfire/arrayfire-rust/issues).

## Build from Source

Edit [build.conf](build.conf) to modify the build flags. The structure is a
simple JSON blob. Currently Rust does not allow key:value pairs to be passed
from the CLI. To use an existing ArrayFire installation modify the first three
JSON values. You can install ArrayFire using one of the following two ways.

- [Download and install binaries](https://arrayfire.com/download)
- [Build and install from source](https://github.com/arrayfire/arrayfire)

To build arrayfire submodule available in the rust wrapper, you have to do
the following.

```bash
git submodule update --init --recursive
cargo build
```
This is recommended way to build Rust wrapper since the submodule points to
the most compatible version of ArrayFire the Rust wrapper has been tested with.
You can find the ArrayFire dependencies below.

- [Linux](https://github.com/arrayfire/arrayfire/wiki/Build-Instructions-for-Linux)
- [OSX](https://github.com/arrayfire/arrayfire/wiki/Build-Instructions-for-OSX)
- [Windows](https://github.com/arrayfire/arrayfire/wiki/Build-Instructions-for-Windows)

## Example

```rust
let num_rows: u64 = 5;
let num_cols: u64 = 3;
let dims = Dim4::new(&[num_rows, num_cols, 1, 1]);
let a = randu::<f32>(dims);
af_print!("Create a 5-by-3 matrix of random floats on the GPU", a);
```

### Sample output

```bash
~/p/arrayfire_rust> cargo run --example helloworld
...
Create a 5-by-3 matrix of random floats on the GPU
[5 3 1 1]
    0.7402     0.4464     0.7762
    0.9210     0.6673     0.2948
    0.0390     0.1099     0.7140
    0.9690     0.4702     0.3585
    0.9251     0.5132     0.6814
...
```

## Acknowledgements

The ArrayFire library is written by developers at
[ArrayFire](http://arrayfire.com) LLC with
[contributions from several individuals](https://github.com/arrayfire/arrayfire_rust/graphs/contributors).

The developers at ArrayFire LLC have received partial financial support
from several grants and institutions. Those that wish to receive public
acknowledgement are listed below:

<!--
The following section contains acknowledgements for grant funding. In most
circumstances, the specific phrasing of the text is mandated by the grant
provider. Thus these acknowledgements must remain intact without modification.
-->

### Grants

This material is based upon work supported by the DARPA SBIR Program Office
under Contract Numbers W31P4Q-14-C-0012 and W31P4Q-15-C-0008.
Any opinions, findings and conclusions or recommendations expressed in this
material are those of the author(s) and do not necessarily reflect the views of
the DARPA SBIR Program Office.
