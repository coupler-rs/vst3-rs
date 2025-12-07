# Changelog

## 2025-12-06

- Rename `vst3-bindgen` crate to `generate` and add `publish = false`, as it is no longer used from the `vst3` build script and is only used to update the static set of generated bindings in the repository.

## 0.3.0

- Update `com-scrape` dependency to 0.2.0. Fixes a build failure with version 3.8.0 of the VST 3 SDK.

## 0.2.2

- Update `com-scrape` dependency to 0.1.3.
- Add an argument to `generate` for explicitly specifying a target triple.

## 0.2.1

- Update `com-scrape` dependency to 0.1.2.

## 0.2.0

- Split up `vst3-bindgen` crate. `vst3-bindgen` is now a library which `vst3` uses from `build.rs`.

## 0.1.1

- Update `com-scrape` version to 0.1.1.

## 0.1.0

- Initial release.
