# vst3-rs

[![Cargo](https://img.shields.io/crates/v/vst3.svg)](https://crates.io/crates/vst3)
[![Docs](https://docs.rs/vst3/badge.svg)](https://docs.rs/vst3)

The `vst3` crate provides Rust bindings for the VST 3 API, generated from the original C++ headers. Abstractions are provided for manipulating COM objects and implementing COM interfaces from Rust. Beyond that, however, these bindings are unsafe, and no attempt is made to abstract over the VST 3 API itself.

## Usage

Add `vst3` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
vst3 = "0.3.0"
```

## License

This project is distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache license, version 2.0](LICENSE-APACHE). Contributions are accepted under the same terms.
