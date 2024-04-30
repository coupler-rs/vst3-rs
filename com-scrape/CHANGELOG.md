# Changelog

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
