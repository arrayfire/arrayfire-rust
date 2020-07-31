[![ci][19]][16] [![][18]][3] [![][17]][4]

# ArrayFire CUDA Interop

This crate is an addition on top of [ArrayFire][1] crate to enable users to mix RAW CUDA code in rust
and [ArrayFire][1].

## Supported platforms

Supported on all platforms [arrayfire-rust][1] is supported.

## Usage

Command to build the crate
```
cargo build -p af_cuda_interop
```

Use the following command to run an example
```
cargo run -p af_cuda_interop --example custom_kernel
```

This crate can be used by directly using the packages on crates.io or building them on your own.
The process for this can be found on [arrayfire-rust#readme][2]

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

[1]: https://github.com/arrayfire/arrayfire-rust
[2]: https://github.com/arrayfire/arrayfire-rust/blob/master/README.md
[3]: http://arrayfire.github.io/arrayfire-rust/af_cuda_interop/index.html
[4]: https://join.slack.com/t/arrayfire-org/shared_invite/MjI4MjIzMDMzMTczLTE1MDI5ODg4NzYtN2QwNGE3ODA5OQ
[14]: https://arrayfire.com/
[15]: https://github.com/arrayfire/arrayfire_rust/graphs/contributors
[16]: https://github.com/arrayfire/arrayfire-rust/actions?workflow=CI
[17]: https://img.shields.io/badge/af_cuda_interop-community-e69138?logo=slack
[18]: https://img.shields.io/badge/af_cuda_interop-Docs-blue?logo=readthedocs
[19]: https://github.com/arrayfire/arrayfire-rust/workflows/ci/badge.svg?event=push
