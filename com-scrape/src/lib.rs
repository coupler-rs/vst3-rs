//! `com-scrape` is a tool for automatically generating Rust bindings for COM interfaces defined in
//! C++. `com-scrape` is developed specifically for use in the `vst3-bindgen` crate, and as such,
//! robustness for arbitrary C++ inputs is a non-goal.

mod clang;
mod generator;
mod parse;
mod print;

pub use generator::Generator;
