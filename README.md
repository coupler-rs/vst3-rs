# vst3-bindgen

[![Docs](https://github.com/coupler-rs/vst3-bindgen/actions/workflows/docs.yml/badge.svg?branch=master)](https://coupler.rs/vst3-bindgen/)

`vst3-bindgen` provides Rust bindings for the VST 3 API, generated from the original C++ headers. Abstractions are provided for manipulating COM objects and implementing COM interfaces from Rust. Beyond that, however, these bindings are unsafe, and no attempt is made to abstract over the VST 3 API itself.

For licensing reasons, `vst3-bindgen` does not contain a copy of the VST 3 SDK itself. Instead, users of this crate must separately obtain a copy of the SDK and specify its path via the `VST3_SDK_DIR` environment variable.

## Usage

First, add `vst3-bindgen` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
vst3-bindgen = "0.1.0"
```

Then, download the VST 3 SDK:

```console
git clone --recursive https://github.com/steinbergmedia/vst3sdk.git
```

Alternatively, you can download the SDK [here](https://www.steinberg.net/developers/) and unzip it.

Finally, set the `VST3_SDK_DIR` environment variable to the path of the SDK directory (the directory just above `pluginterfaces`).

It can be convenient to include the VST 3 interface headers in your project repository. If the `pluginterfaces` directory is located at `vst3sdk/pluginterfaces/` relative to your Cargo workspace root, you can include the following in a `.cargo/config.toml` file (also relative to your workspace root):

```toml
[env]
VST3_SDK_DIR = { value = "vst3sdk", relative = true }
```

This crate also depends on `libclang` for parsing the C++ header files in the SDK. For information on how to install `libclang` for various platforms, see the [`bindgen` user guide](https://rust-lang.github.io/rust-bindgen/requirements.html#clang); for information on controlling how `vst3-bindgen` searches for `libclang`, see the [`clang-sys` documentation](https://github.com/KyleMayes/clang-sys#readme). `libclang` version 6.0 or later is required.

## License

`vst3-bindgen` is distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache license, version 2.0](LICENSE-APACHE). Contributions are accepted under the same terms.
