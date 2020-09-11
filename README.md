# bitarray

[![Discord][dci]][dcl] [![Crates.io][ci]][cl] ![MIT/Apache][li] [![docs.rs][di]][dl] ![LoC][lo] ![Tests][btl] ![Lints][bll] ![no_std][bnl]

[ci]: https://img.shields.io/crates/v/bitarray.svg
[cl]: https://crates.io/crates/bitarray/

[li]: https://img.shields.io/crates/l/specs.svg?maxAge=2592000

[di]: https://docs.rs/bitarray/badge.svg
[dl]: https://docs.rs/bitarray/

[lo]: https://tokei.rs/b1/github/rust-cv/bitarray?category=code

[dci]: https://img.shields.io/discord/550706294311485440.svg?logo=discord&colorB=7289DA
[dcl]: https://discord.gg/d32jaam

[btl]: https://github.com/rust-cv/bitarray/workflows/unit%20tests/badge.svg
[bll]: https://github.com/rust-cv/bitarray/workflows/lints/badge.svg
[bnl]: https://github.com/rust-cv/bitarray/workflows/no-std/badge.svg

A compile time sized array of bits that uses const generics and intrinsics.

This library was created to maximize the speed of hamming weight and hamming distance computation.
It could be used for many other things. Contributions are welcome!

## Features

Enable the `unstable-512-bit-simd` feature if you would like to use 512-bit SIMD instructions to speed up the library. This feature does not compile on all machines for some currently unknown reason, as an LLVM intrinsics error is reported, even with the same compiler version and host tripple.

## Questions

Please visit [![Discord][dci]][dcl] if you have any questions or want to contribute. Also feel free to file an issue on GitHub.
