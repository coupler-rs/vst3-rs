//! The `vst3` crate provides Rust bindings for the VST 3 API, generated from the original C++
//! headers. Abstractions are provided for manipulating COM objects and implementing COM interfaces
//! from Rust. Beyond that, however, these bindings are unsafe, and no attempt is made to abstract
//! over the VST 3 API itself.
//!
//! # Bindings
//!
//! Generated bindings are located in the [`Steinberg`] module. In addition to the COM interfaces,
//! bindings include struct definitions, type aliases, constants, and enums. The module structure
//! of the bindings mirrors the namespace structure of the original headers, with minor differences
//! where necessary (e.g., definitions which are nested inside a C++ type `SomeType` will be found
//! inside a `SomeType_` module in the generated bindings).
//!
//! For each COM interface `IInterface` in the C++ headers, the bindings include a corresponding
//! Rust type `IInterface`, a virtual table struct `IInterfaceVtbl`, and a trait `IInterfaceTrait`
//! (excluding `FUnknown`, for which no trait is generated). Each `IInterface` type also implements
//! the [`Interface`] trait, which holds an associated constant [`Interface::IID`] specifying the
//! GUID corresponding to that interface.
//!
//! # Interacting with COM objects
//!
//! The [`ComPtr`] and [`ComRef`] smart pointers are provided for interacting with COM objects.
//! These types make it safer and more convenient to call methods, cast between interfaces, and
//! manage reference counts.
//!
//! For an overview of how to properly manage ownership and reference counts using [`ComPtr`] and
//! [`ComRef`], see the [`com-scrape-types` documentation](com_scrape_types#reference-counting).
//!
//! # Implementing COM interfaces from Rust
//!
//! COM classes can be defined in Rust using the [`Class`] trait and the interface traits generated
//! from the VST 3 headers, and objects of these classes can be instantiated using the
//! [`ComWrapper`] smart pointer:
//!
//! ```
//! # use vst3::{*, Steinberg::*};
//! struct MyClass;
//!
//! impl Class for MyClass {
//!     type Interfaces = (IPluginBase,);
//! }
//!
//! impl IPluginBaseTrait for MyClass {
//!     unsafe fn initialize(&self, context: *mut FUnknown) -> tresult {
//!         kResultOk
//!     }
//!
//!     unsafe fn terminate(&self) -> tresult {
//!         kResultOk
//!     }
//! }
//!
//! let my_obj = ComWrapper::new(MyClass);
//! let ptr = my_obj.to_com_ptr::<IPluginBase>().unwrap();
//! ```
//!
//! For more detail on implementing COM interfaces from rust, see the
//! [`com-scrape-types` documentation](com_scrape_types#implementing-com-interfaces-from-rust).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use com_scrape_types;
pub use com_scrape_types::{Class, ComPtr, ComRef, ComWrapper, Interface};

mod support;

pub use support::uid;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

pub mod Steinberg {
    pub use crate::bindings::Steinberg::*;

    #[cfg(target_os = "windows")]
    mod result_values {
        #![allow(overflowing_literals)]

        use std::ffi::c_int;

        pub const kNoInterface: c_int = 0x80004002;
        pub const kResultOk: c_int = 0x00000000;
        pub const kResultTrue: c_int = kResultOk;
        pub const kResultFalse: c_int = 0x00000001;
        pub const kInvalidArgument: c_int = 0x80070057;
        pub const kNotImplemented: c_int = 0x80004001;
        pub const kInternalError: c_int = 0x80004005;
        pub const kNotInitialized: c_int = 0x8000FFFF;
        pub const kOutOfMemory: c_int = 0x8007000E;
    }

    #[cfg(not(target_os = "windows"))]
    mod result_values {
        use std::ffi::c_int;

        pub const kNoInterface: c_int = -1;
        pub const kResultOk: c_int = 0;
        pub const kResultTrue: c_int = kResultOk;
        pub const kResultFalse: c_int = 1;
        pub const kInvalidArgument: c_int = 2;
        pub const kNotImplemented: c_int = 3;
        pub const kInternalError: c_int = 4;
        pub const kNotInitialized: c_int = 5;
        pub const kOutOfMemory: c_int = 6;
    }

    pub use result_values::*;
}
