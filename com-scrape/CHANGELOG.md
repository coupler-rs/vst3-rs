# Changelog

## 0.3.0

- `constant_parser` callback now takes `&[&str]` instead of `&[String]` and is expected to output only an expression rather than a full constant definition (i.e., for `const X: T = E;`, the callback should return only `"E"`).
- For non-anonymous enum declarations, the generated enum constants now use the enum type rather than the underlying integer type (i.e., for `enum E : int32 { kC };`, we generate `const kC: E` instead of `const kC: int32`).
- Fixed-width integer types are now mapped directly to Rust integer types rather than using the underlying definitions in system `<cstdint>` headers (e.g., `uint8_t` is mapped directly to `u8`).
- Definitions in generator output are now sorted by name.
- `skip_types` and `skip_interface_traits` methods now expect full namespace paths for types (e.g., `Steinberg::FUnknown` rather than `FUnknown`).
- Add `override_constant_values`, `override_typedef_types`, and `override_constant_types` options to `Generator`.

## 0.2.0

- Add support for forward declarations of extern types.
- Map `char16_t` type to `u16` (was previously mapped to `i16`).

## 0.1.3

- Add a `target` option to the `Generator` API for explicitly specifying a target triple.
- Loosen generic bounds on `Generator::skip_types`.

## 0.1.2

- Fix cross-compilation build failure introduced in 0.1.1 ([#9](https://github.com/coupler-rs/vst3-rs/pull/9)).

## 0.1.1

- Check for and report Clang errors when parsing C++ headers.
- Fix generation failures when using Clang from Xcode command line tools.

## 0.1.0

- Initial release.
