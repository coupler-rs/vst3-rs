#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;

use Steinberg::{int8, kNoInterface, kResultOk, tresult, uint32, FUnknown, FUnknownVtbl, TUID};

pub use com_scrape_types::*;

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
    pub const fn make_vtbl<C, I>() -> FUnknownVtbl
    where
        I: Interface,
        C: Class + Implements<I>,
    {
        unsafe extern "system" fn queryInterface<C, I>(
            this: *mut FUnknown,
            _iid: *const TUID,
            obj: *mut *mut c_void,
        ) -> tresult
        where
            I: Interface,
            C: Class + Implements<I>,
        {
            let ptr = ComWrapper::<C>::data_from_interface::<I>(this as *mut I);
            if let Some(result) = C::query_interface(ptr, &*(_iid as *const Guid)) {
                *obj = result;
                kResultOk
            } else {
                kNoInterface
            }
        }

        unsafe extern "system" fn addRef<C, I>(this: *mut FUnknown) -> uint32
        where
            I: Interface,
            C: Class + Implements<I>,
        {
            let ptr = ComWrapper::<C>::data_from_interface::<I>(this as *mut I);
            C::add_ref(ptr) as uint32
        }

        unsafe extern "system" fn release<C, I>(this: *mut FUnknown) -> uint32
        where
            I: Interface,
            C: Class + Implements<I>,
        {
            let ptr = ComWrapper::<C>::data_from_interface::<I>(this as *mut I);
            C::release(ptr) as uint32
        }

        FUnknownVtbl {
            queryInterface: queryInterface::<C, I>,
            addRef: addRef::<C, I>,
            release: release::<C, I>,
        }
    }
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
