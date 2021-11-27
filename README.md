[![ci][19]][16] [![docs][18]][3] [![book][22]][21] [![slack][17]][4] [![github-discussions][20]][5]

# Arrayfire Rust Bindings

[ArrayFire][1] is a high performance library for parallel computing with an easy-to-use API. It
enables users to write scientific computing code that is portable across CUDA, OpenCL and CPU
devices. This project provides Rust bindings for the ArrayFire library. Given below table shows
the rust bindings compatability with ArrayFire.  If you find any bugs, please report them [here][2].

| arrayfire-rust | ArrayFire |
|:--------------:|:---------:|
|         M.m.p1 |    M.m.p2 |

Only, Major(M) & Minor(m) version numbers need to match. *p1* and *p2* are patch/fix updates for
`arrayfire-rust` & `ArrayFire` respectively, and they don't need to match.

## Supported platforms

Linux, Windows and OSX. Rust 1.31 or newer is required.

## Use from Crates.io [![][6]][7] [![][8]][9]

To use the rust bindings for ArrayFire from crates.io, the following requirements are to be met first.

1. [Download and install ArrayFire binaries][10] based on your operating system. Depending on the
   method of your installation for Linux, steps (2) & (3) may not be required. If that is the case,
   proceed to step (4) directly.
2. Set the evironment variable `AF_PATH` to point to ArrayFire installation root folder.
3. Make sure to add the path to lib files to your path environment variables.
    - On Linux: do `export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$AF_PATH/lib64`
    - On OSX: do `export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:$AF_PATH/lib`
    - On Windows: Add `%AF_PATH%\lib` to your PATH environment variable.
4. Add `arrayfire = "3.8"` to the dependencies section of your project's Cargo.toml file.
   Make sure to change the version to latest available.

Once step (4) is over, you should be able to use ArrayFire in your Rust project. If you find any
bugs, please report them [here][2].

## Build from Source

Edit [build.conf](build.conf) to modify the build flags. The structure is a simple JSON blob.
Currently Rust does not allow key:value pairs to be passed from the CLI. To use an existing
ArrayFire installation modify the first three JSON values. You can install ArrayFire using
one of the following two ways.

- [Download and install binaries][10]
- [Build and install from source][1]

To build arrayfire submodule available in the rust wrapper repository, you have to do the following.

```bash
git submodule update --init --recursive
cargo build // use --all to build all crates in the workspace
```
This is recommended way to build Rust wrapper since the submodule points to the most compatible
version of ArrayFire the Rust wrapper has been tested with. You can find the ArrayFire dependencies below.

- [Linux][11]
- [OSX][12]
- [Windows][13]

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

### Troubleshooting

If the build command fails with undefined references errors even after taking care of environment
variables, we recommend doing a `cargo clean` and re-running `cargo build` or `cargo test`.

You can also use some environment variables mentioned in our [book][23], such as `AF_PRINT_ERRORS`
to print more elaborate error messages to console.

## Acknowledgements

The ArrayFire library is written by developers at [ArrayFire][14] LLC with [contributions][15]
from several individuals. The developers at ArrayFire LLC have received partial financial support
from several grants and institutions. Those that wish to receive public acknowledgement are listed
below:

### Grants

This material is based upon work supported by the DARPA SBIR Program Office under Contract Numbers
W31P4Q-14-C-0012 and W31P4Q-15-C-0008. Any opinions, findings and conclusions or recommendations
expressed in this material are those of the author(s) and do not necessarily reflect the views of
the DARPA SBIR Program Office.

[1]: https://github.com/arrayfire/arrayfire
[2]: https://github.com/arrayfire/arrayfire-rust/issues
[3]: http://arrayfire.github.io/arrayfire-rust/arrayfire/index.html
[4]: https://join.slack.com/t/arrayfire-org/shared_invite/MjI4MjIzMDMzMTczLTE1MDI5ODg4NzYtN2QwNGE3ODA5OQ
[5]: https://github.com/arrayfire/arrayfire-rust/discussions
[6]: http://meritbadge.herokuapp.com/arrayfire
[7]: https://crates.io/crates/arrayfire
[8]: https://docs.rs/arrayfire/badge.svg
[9]: https://docs.rs/arrayfire
[10]: https://arrayfire.com/download
[11]: https://github.com/arrayfire/arrayfire/wiki/Build-Instructions-for-Linux
[12]: https://github.com/arrayfire/arrayfire/wiki/Build-Instructions-for-OSX
[13]: https://github.com/arrayfire/arrayfire/wiki/Build-Instructions-for-Windows
[14]: https://arrayfire.com/
[15]: https://github.com/arrayfire/arrayfire_rust/graphs/contributors
[16]: https://github.com/arrayfire/arrayfire-rust/actions?workflow=CI
[17]: https://img.shields.io/badge/arrayfire-community-e69138?logo=slack
[18]: https://img.shields.io/badge/arrayfire-Docs-blue?logo=readthedocs
[19]: https://github.com/arrayfire/arrayfire-rust/workflows/ci/badge.svg?event=push
[20]: https://img.shields.io/badge/GitHub-Discussions-orange
[21]: http://arrayfire.org/arrayfire-rust/book/index.html
[22]: https://img.shields.io/badge/arrayfire-mdbook-073763?logo=readthedocs
[23]: http://arrayfire.org/arrayfire-rust/book/configuring_arrayfire_environment.html
