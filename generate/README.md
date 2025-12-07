# VST 3 binding generator

Binding generator script for the VST 3 API. Generates bindings from the headers in the `/vst3sdk` directory and writes them to `/src/bindings.rs`.

## Usage

To generate updated bindings:

```console
cargo run --release -p generate
```

This generator depends on `libclang` for parsing the C++ header files in the SDK. For information on how to install `libclang` for various platforms, see the [`bindgen` user guide](https://rust-lang.github.io/rust-bindgen/requirements.html#clang); for information on controlling how the generator searches for `libclang`, see the [`clang-sys` documentation](https://github.com/KyleMayes/clang-sys#readme). `libclang` version 6.0 or later is required.
