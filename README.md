# vst3-bindgen

[![Docs](https://github.com/coupler-rs/vst3-bindgen/actions/workflows/docs.yml/badge.svg?branch=master)](https://coupler.rs/vst3-bindgen/)

`vst3-bindgen` generates Rust bindings for the VST 3 API from the original C++ headers.

## Usage

This crate does not contain a copy of the VST 3 SDK itself. Instead, `vst3-bindgen` will look for a user-supplied copy of the SDK at the path specified in the `VST3_SDK_DIR` environment variable. The VST 3 SDK can be acquired [here](https://github.com/steinbergmedia/vst3sdk).

This crate also depends on `libclang` for parsing the C++ header files in the SDK. For information on how to install `libclang` for various platforms, see the [`bindgen` user guide](https://rust-lang.github.io/rust-bindgen/requirements.html#clang); for information on controlling how `vst3-bindgen` searches for `libclang`, see the [`clang-sys` documentation](https://github.com/KyleMayes/clang-sys#readme). `libclang` version 6.0 or later is required.

## License

`vst3-bindgen` is distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache license, version 2.0](LICENSE-APACHE). Contributions are accepted under the same terms.
