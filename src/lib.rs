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

use std::ffi::c_void;

use Steinberg::{int8, kNoInterface, kResultOk, tresult, uint32, FUnknown, FUnknownVtbl, TUID};

use com_scrape_types::{Construct, Guid, Header, InterfaceList, Wrapper};

pub use com_scrape_types;
pub use com_scrape_types::{Class, ComPtr, ComRef, ComWrapper, Interface};

const fn tuid_as_guid(tuid: TUID) -> Guid {
    [
        tuid[0] as u8,
        tuid[1] as u8,
        tuid[2] as u8,
        tuid[3] as u8,
        tuid[4] as u8,
        tuid[5] as u8,
        tuid[6] as u8,
        tuid[7] as u8,
        tuid[8] as u8,
        tuid[9] as u8,
        tuid[10] as u8,
        tuid[11] as u8,
        tuid[12] as u8,
        tuid[13] as u8,
        tuid[14] as u8,
        tuid[15] as u8,
    ]
}

#[inline]
unsafe fn FUnknown_query_interface(this: *mut c_void, iid: &Guid) -> Option<*mut c_void> {
    let ptr = this as *mut FUnknown;
    let mut obj = std::ptr::null_mut();
    let result = ((*(*ptr).vtbl).queryInterface)(ptr, iid.as_ptr() as *const TUID, &mut obj);

    if result == kResultOk {
        Some(obj)
    } else {
        None
    }
}

#[inline]
unsafe fn FUnknown_add_ref(this: *mut c_void) -> usize {
    let ptr = this as *mut FUnknown;
    ((*(*ptr).vtbl).addRef)(ptr) as usize
}

#[inline]
unsafe fn FUnknown_release(this: *mut c_void) -> usize {
    let ptr = this as *mut FUnknown;
    ((*(*ptr).vtbl).release)(ptr) as usize
}

impl FUnknown {
    const fn make_vtbl<C, W, const OFFSET: isize>() -> FUnknownVtbl
    where
        C: Class,
        W: Wrapper<C>,
    {
        unsafe extern "system" fn queryInterface<C, W, const OFFSET: isize>(
            this: *mut FUnknown,
            _iid: *const TUID,
            obj: *mut *mut c_void,
        ) -> tresult
        where
            C: Class,
            W: Wrapper<C>,
        {
            let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut Header<C>;
            if let Some(result) = C::Interfaces::query(&*(_iid as *const Guid)) {
                let ptr = W::data_from_header(header_ptr);
                W::add_ref(ptr);

                *obj = (header_ptr as *mut u8).offset(result) as *mut c_void;

                kResultOk
            } else {
                kNoInterface
            }
        }

        unsafe extern "system" fn addRef<C, W, const OFFSET: isize>(this: *mut FUnknown) -> uint32
        where
            C: Class,
            W: Wrapper<C>,
        {
            let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut Header<C>;
            let ptr = W::data_from_header(header_ptr);
            W::add_ref(ptr) as uint32
        }

        unsafe extern "system" fn release<C, W, const OFFSET: isize>(this: *mut FUnknown) -> uint32
        where
            C: Class,
            W: Wrapper<C>,
        {
            let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut Header<C>;
            let ptr = W::data_from_header(header_ptr);
            W::release(ptr) as uint32
        }

        FUnknownVtbl {
            queryInterface: queryInterface::<C, W, OFFSET>,
            addRef: addRef::<C, W, OFFSET>,
            release: release::<C, W, OFFSET>,
        }
    }
}

unsafe impl<C, W, const OFFSET: isize> Construct<C, W, OFFSET> for FUnknown
where
    C: Class,
    W: Wrapper<C>,
{
    const OBJ: FUnknown = FUnknown {
        vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
    };
}

/// Constructs a 16-byte [`TUID`] value from four 32-bit integers.
///
/// Note that the byte order of the resulting value will differ between Windows and other
/// platforms.
pub const fn uid(a: u32, b: u32, c: u32, d: u32) -> TUID {
    uid_impl(a, b, c, d)
}

#[cfg(target_os = "windows")]
const fn uid_impl(a: u32, b: u32, c: u32, d: u32) -> TUID {
    [
        ((a & 0x000000FF) >> 0) as int8,
        ((a & 0x0000FF00) >> 8) as int8,
        ((a & 0x00FF0000) >> 16) as int8,
        ((a & 0xFF000000) >> 24) as int8,
        ((b & 0x00FF0000) >> 16) as int8,
        ((b & 0xFF000000) >> 24) as int8,
        ((b & 0x000000FF) >> 0) as int8,
        ((b & 0x0000FF00) >> 8) as int8,
        ((c & 0xFF000000) >> 24) as int8,
        ((c & 0x00FF0000) >> 16) as int8,
        ((c & 0x0000FF00) >> 8) as int8,
        ((c & 0x000000FF) >> 0) as int8,
        ((d & 0xFF000000) >> 24) as int8,
        ((d & 0x00FF0000) >> 16) as int8,
        ((d & 0x0000FF00) >> 8) as int8,
        ((d & 0x000000FF) >> 0) as int8,
    ]
}

#[cfg(not(target_os = "windows"))]
const fn uid_impl(a: u32, b: u32, c: u32, d: u32) -> TUID {
    [
        ((a & 0xFF000000) >> 24) as int8,
        ((a & 0x00FF0000) >> 16) as int8,
        ((a & 0x0000FF00) >> 8) as int8,
        ((a & 0x000000FF) >> 0) as int8,
        ((b & 0xFF000000) >> 24) as int8,
        ((b & 0x00FF0000) >> 16) as int8,
        ((b & 0x0000FF00) >> 8) as int8,
        ((b & 0x000000FF) >> 0) as int8,
        ((c & 0xFF000000) >> 24) as int8,
        ((c & 0x00FF0000) >> 16) as int8,
        ((c & 0x0000FF00) >> 8) as int8,
        ((c & 0x000000FF) >> 0) as int8,
        ((d & 0xFF000000) >> 24) as int8,
        ((d & 0x00FF0000) >> 16) as int8,
        ((d & 0x0000FF00) >> 8) as int8,
        ((d & 0x000000FF) >> 0) as int8,
    ]
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
