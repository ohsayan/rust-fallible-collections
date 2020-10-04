# Fallible access trait for Rust's collections

> ### **Status**: Experimental

## Introduction

This crate provides the `FallibleSequence` trait, which can be used for standardization of the methods provided by sequential data structures like `Vec` , `VecDeque` and `String` .

## Motivation

While I was working on implementing snapshots in [TerrabaseDB](https://github.com/terrabasedb/terrabasedb), I needed to implement a simple queue data structure, which required a 'non-panicky' implementation of `Vec::remove(idx)` . This prompted me to create [this PR](https://github.com/rust-lang/rust/pull/77480). However, from the discussion in the linked PR, it was clear that there was a lack of standardization in the collections when it came to `get()` , `remove()` , `insert()` , etc. This crate exists as a [_testbed_](https://en.wikipedia.org/wiki/Testbed) for attempts on standardizing these methods.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally by you, shall be dual-licensed under Apache-2.0 and MIT, without any additional terms or conditions.

## License

This project is dual-licensed under the [Apache-2.0](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses.
