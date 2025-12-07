# Changelog

## 0.3.0

- Generated bindings are now identical on Windows, macOS, and Linux:
  - Fixed-width integer types are now mapped to the same Rust integer types on all platforms (e.g., `uint8_t` to `u8`)
  - Definitions are now sorted by name so that they are in the same order on all platforms
  - Result value constants (`kNoInterface`, etc.) have different values on different plaforms. This is now accomplished using `#[cfg]` on the Rust side rather than relying on `#if`s on the C++ side
  - Enum definitions without an explicit base type have different underlying integer types on different platforms. This is now also accomplished using `#[cfg]` on the Rust side rather than relying on `libclang`'s output
- Generated bindings are now published as part of the source of the `vst3` crate rather than being generated at build time. It is no longer necessary for users to supply a copy of the VST 3 SDK when building the crate.

## 0.2.0

- Upgrade `vst3-bindgen` dependency to 0.3.0. Fixes a build failure with version 3.8.0 of the VST 3 SDK.

## 0.1.2

- Upgrade `vst3-bindgen` dependency to 0.2.2.
- Upgrade `com-scrape-types` dependency to 0.1.1.

## 0.1.1

- Upgrade `vst3-bindgen` dependency to 0.2.1.

## 0.1.0

- Split up `vst3-bindgen` crate. `vst3-bindgen` is now a library which `vst3` uses from `build.rs`.
