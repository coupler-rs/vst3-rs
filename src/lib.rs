#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;
use std::ptr;

use Steinberg::{int8, kResultOk, FUnknown, TUID};

pub use com_scrape_types::*;

macro_rules! impl_interface {
    ($name:ident, $iid:ident) => {
        impl ::com_scrape_types::Interface for $name {
            const IID: ::com_scrape_types::Guid = crate::tuid_as_guid($iid);

            #[inline]
            unsafe fn query_interface(
                this: *mut Self,
                iid: &Guid,
            ) -> Option<*mut ::std::ffi::c_void> {
                crate::FUnknown_query_interface(this as *mut crate::Steinberg::FUnknown, iid)
            }

            #[inline]
            unsafe fn add_ref(this: *mut Self) {
                crate::FUnknown_add_ref(this as *mut crate::Steinberg::FUnknown)
            }

            #[inline]
            unsafe fn release(this: *mut Self) {
                crate::FUnknown_release(this as *mut crate::Steinberg::FUnknown)
            }
        }
    };
}

#[inline]
unsafe fn FUnknown_query_interface(this: *mut FUnknown, iid: &Guid) -> Option<*mut c_void> {
    let ptr = this as *mut FUnknown;
    let mut obj = ptr::null_mut();
    let result = ((*(*ptr).vtbl).queryInterface)(ptr, iid.as_ptr() as *const TUID, &mut obj);

    if result == kResultOk {
        Some(obj as *mut c_void)
    } else {
        None
    }
}

#[inline]
unsafe fn FUnknown_add_ref(this: *mut FUnknown) {
    let ptr = this as *mut FUnknown;
    ((*(*ptr).vtbl).addRef)(ptr);
}

#[inline]
unsafe fn FUnknown_release(this: *mut FUnknown) {
    let ptr = this as *mut FUnknown;
    ((*(*ptr).vtbl).release)(ptr);
}

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

#[cfg(target_os = "windows")]
pub const fn uid(a: u32, b: u32, c: u32, d: u32) -> TUID {
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
pub const fn uid(a: u32, b: u32, c: u32, d: u32) -> TUID {
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
