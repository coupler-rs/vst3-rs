#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl_display {
    _private: [u8; 0],
}
unsafe impl Send for wl_display {}
unsafe impl Sync for wl_display {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl_surface {
    _private: [u8; 0],
}
unsafe impl Send for wl_surface {}
unsafe impl Sync for wl_surface {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdg_surface {
    _private: [u8; 0],
}
unsafe impl Send for xdg_surface {}
unsafe impl Sync for xdg_surface {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdg_toplevel {
    _private: [u8; 0],
}
unsafe impl Send for xdg_toplevel {}
unsafe impl Sync for xdg_toplevel {}
pub mod Steinberg {
    #[allow(unused_imports)]
    use super::*;
    pub type CString = *const tchar;
    pub type CStringA = *const char8;
    pub type CStringW = *const char16;
    pub type ColorComponent = uint8;
    pub type ColorSpec = uint32;
    pub type Direction = crate::support::DefaultEnumType;
    pub mod Direction_ {
        #[allow(unused_imports)]
        use super::*;
        pub const kEast: Direction = 2;
        pub const kNoDirection: Direction = 8;
        pub const kNorth: Direction = 0;
        pub const kNorthEast: Direction = 1;
        pub const kNorthWest: Direction = 7;
        pub const kNumberOfDirections: Direction = 9;
        pub const kSouth: Direction = 4;
        pub const kSouthEast: Direction = 3;
        pub const kSouthWest: Direction = 5;
        pub const kWest: Direction = 6;
    }
    pub type FIDString = *const char8;
    pub type IAttrID = FIDString;
    pub type KeyModifier = crate::support::DefaultEnumType;
    pub mod KeyModifier_ {
        #[allow(unused_imports)]
        use super::*;
        pub const kAlternateKey: KeyModifier = 2;
        pub const kCommandKey: KeyModifier = 4;
        pub const kControlKey: KeyModifier = 8;
        pub const kShiftKey: KeyModifier = 1;
    }
    pub type Orientation = crate::support::DefaultEnumType;
    pub mod Orientation_ {
        #[allow(unused_imports)]
        use super::*;
        pub const kHorizontal: Orientation = 0;
        pub const kNumberOfOrientations: Orientation = 2;
        pub const kVertical: Orientation = 1;
    }
    pub type StandardColor = crate::support::DefaultEnumType;
    pub mod StandardColor_ {
        #[allow(unused_imports)]
        use super::*;
        pub const kBlack: StandardColor = 0;
        pub const kBlack50: StandardColor = 31;
        pub const kBlack70: StandardColor = 32;
        pub const kBlue: StandardColor = 18;
        pub const kDkBlue: StandardColor = 20;
        pub const kDkGray: StandardColor = 9;
        pub const kDkGreen: StandardColor = 17;
        pub const kDkMagenta: StandardColor = 23;
        pub const kDkOrange: StandardColor = 29;
        pub const kDkRed: StandardColor = 14;
        pub const kDkYellow: StandardColor = 26;
        pub const kGold: StandardColor = 30;
        pub const kGray: StandardColor = 7;
        pub const kGray10: StandardColor = 3;
        pub const kGray20: StandardColor = 4;
        pub const kGray30: StandardColor = 5;
        pub const kGray40: StandardColor = 6;
        pub const kGray5: StandardColor = 2;
        pub const kGray50: StandardColor = 7;
        pub const kGray60: StandardColor = 8;
        pub const kGray70: StandardColor = 9;
        pub const kGray80: StandardColor = 10;
        pub const kGray90: StandardColor = 11;
        pub const kGreen: StandardColor = 15;
        pub const kLtBlue: StandardColor = 19;
        pub const kLtGray: StandardColor = 4;
        pub const kLtGreen: StandardColor = 16;
        pub const kLtMagenta: StandardColor = 22;
        pub const kLtOrange: StandardColor = 28;
        pub const kLtRed: StandardColor = 13;
        pub const kLtYellow: StandardColor = 25;
        pub const kMagenta: StandardColor = 21;
        pub const kNumStandardColors: StandardColor = 33;
        pub const kOrange: StandardColor = 27;
        pub const kRed: StandardColor = 12;
        pub const kWhite: StandardColor = 1;
        pub const kYellow: StandardColor = 24;
    }
    pub type TBool = uint8;
    pub type TPtrInt = uint64;
    pub type TSize = int64;
    pub type TUID = [::std::ffi::c_char; 16];
    pub type UColorComponent = ColorComponent;
    pub type UColorSpec = ColorSpec;
    pub type UCoord = int32;
    pub type VirtualKeyCodes = crate::support::DefaultEnumType;
    pub mod VirtualKeyCodes_ {
        #[allow(unused_imports)]
        use super::*;
        pub const KEY_ADD: VirtualKeyCodes = 35;
        pub const KEY_ALT: VirtualKeyCodes = 56;
        pub const KEY_BACK: VirtualKeyCodes = 1;
        pub const KEY_CLEAR: VirtualKeyCodes = 3;
        pub const KEY_CONTEXTMENU: VirtualKeyCodes = 58;
        pub const KEY_CONTROL: VirtualKeyCodes = 55;
        pub const KEY_DECIMAL: VirtualKeyCodes = 38;
        pub const KEY_DELETE: VirtualKeyCodes = 22;
        pub const KEY_DIVIDE: VirtualKeyCodes = 39;
        pub const KEY_DOWN: VirtualKeyCodes = 14;
        pub const KEY_END: VirtualKeyCodes = 9;
        pub const KEY_ENTER: VirtualKeyCodes = 19;
        pub const KEY_EQUALS: VirtualKeyCodes = 57;
        pub const KEY_ESCAPE: VirtualKeyCodes = 6;
        pub const KEY_F1: VirtualKeyCodes = 40;
        pub const KEY_F10: VirtualKeyCodes = 49;
        pub const KEY_F11: VirtualKeyCodes = 50;
        pub const KEY_F12: VirtualKeyCodes = 51;
        pub const KEY_F13: VirtualKeyCodes = 65;
        pub const KEY_F14: VirtualKeyCodes = 66;
        pub const KEY_F15: VirtualKeyCodes = 67;
        pub const KEY_F16: VirtualKeyCodes = 68;
        pub const KEY_F17: VirtualKeyCodes = 69;
        pub const KEY_F18: VirtualKeyCodes = 70;
        pub const KEY_F19: VirtualKeyCodes = 71;
        pub const KEY_F2: VirtualKeyCodes = 41;
        pub const KEY_F20: VirtualKeyCodes = 72;
        pub const KEY_F21: VirtualKeyCodes = 73;
        pub const KEY_F22: VirtualKeyCodes = 74;
        pub const KEY_F23: VirtualKeyCodes = 75;
        pub const KEY_F24: VirtualKeyCodes = 76;
        pub const KEY_F3: VirtualKeyCodes = 42;
        pub const KEY_F4: VirtualKeyCodes = 43;
        pub const KEY_F5: VirtualKeyCodes = 44;
        pub const KEY_F6: VirtualKeyCodes = 45;
        pub const KEY_F7: VirtualKeyCodes = 46;
        pub const KEY_F8: VirtualKeyCodes = 47;
        pub const KEY_F9: VirtualKeyCodes = 48;
        pub const KEY_HELP: VirtualKeyCodes = 23;
        pub const KEY_HOME: VirtualKeyCodes = 10;
        pub const KEY_INSERT: VirtualKeyCodes = 21;
        pub const KEY_LEFT: VirtualKeyCodes = 11;
        pub const KEY_MEDIA_NEXT: VirtualKeyCodes = 62;
        pub const KEY_MEDIA_PLAY: VirtualKeyCodes = 59;
        pub const KEY_MEDIA_PREV: VirtualKeyCodes = 61;
        pub const KEY_MEDIA_STOP: VirtualKeyCodes = 60;
        pub const KEY_MULTIPLY: VirtualKeyCodes = 34;
        pub const KEY_NEXT: VirtualKeyCodes = 8;
        pub const KEY_NUMLOCK: VirtualKeyCodes = 52;
        pub const KEY_NUMPAD0: VirtualKeyCodes = 24;
        pub const KEY_NUMPAD1: VirtualKeyCodes = 25;
        pub const KEY_NUMPAD2: VirtualKeyCodes = 26;
        pub const KEY_NUMPAD3: VirtualKeyCodes = 27;
        pub const KEY_NUMPAD4: VirtualKeyCodes = 28;
        pub const KEY_NUMPAD5: VirtualKeyCodes = 29;
        pub const KEY_NUMPAD6: VirtualKeyCodes = 30;
        pub const KEY_NUMPAD7: VirtualKeyCodes = 31;
        pub const KEY_NUMPAD8: VirtualKeyCodes = 32;
        pub const KEY_NUMPAD9: VirtualKeyCodes = 33;
        pub const KEY_PAGEDOWN: VirtualKeyCodes = 16;
        pub const KEY_PAGEUP: VirtualKeyCodes = 15;
        pub const KEY_PAUSE: VirtualKeyCodes = 5;
        pub const KEY_PRINT: VirtualKeyCodes = 18;
        pub const KEY_RETURN: VirtualKeyCodes = 4;
        pub const KEY_RIGHT: VirtualKeyCodes = 13;
        pub const KEY_SCROLL: VirtualKeyCodes = 53;
        pub const KEY_SELECT: VirtualKeyCodes = 17;
        pub const KEY_SEPARATOR: VirtualKeyCodes = 36;
        pub const KEY_SHIFT: VirtualKeyCodes = 54;
        pub const KEY_SNAPSHOT: VirtualKeyCodes = 20;
        pub const KEY_SPACE: VirtualKeyCodes = 7;
        pub const KEY_SUBTRACT: VirtualKeyCodes = 37;
        pub const KEY_SUPER: VirtualKeyCodes = 77;
        pub const KEY_TAB: VirtualKeyCodes = 2;
        pub const KEY_UP: VirtualKeyCodes = 12;
        pub const KEY_VOLUME_DOWN: VirtualKeyCodes = 64;
        pub const KEY_VOLUME_UP: VirtualKeyCodes = 63;
        pub const VKEY_FIRST_ASCII: VirtualKeyCodes = 128;
        pub const VKEY_FIRST_CODE: VirtualKeyCodes = 1;
        pub const VKEY_LAST_CODE: VirtualKeyCodes = 77;
    }
    pub type char16 = u16;
    pub type char8 = ::std::ffi::c_char;
    pub type int16 = i16;
    pub type int32 = i32;
    pub type int64 = i64;
    pub type int8 = ::std::ffi::c_char;
    pub type tchar = char16;
    pub type tresult = int32;
    pub type uchar = ::std::ffi::c_uchar;
    pub type uint16 = u16;
    pub type uint32 = u32;
    pub type uint64 = u64;
    pub type uint8 = u8;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FUnknown {
        pub vtbl: *const FUnknownVtbl,
    }
    unsafe impl Send for FUnknown {}
    unsafe impl Sync for FUnknown {}
    impl ::com_scrape_types::Unknown for FUnknown {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for FUnknown {
        type Vtbl = FUnknownVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(FUnknown_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FUnknownVtbl {
        pub queryInterface: unsafe extern "system" fn(
            this: *mut FUnknown,
            _iid: *const TUID,
            obj: *mut *mut ::std::ffi::c_void,
        ) -> tresult,
        pub addRef: unsafe extern "system" fn(
            this: *mut FUnknown,
        ) -> uint32,
        pub release: unsafe extern "system" fn(
            this: *mut FUnknown,
        ) -> uint32,
    }
    mod __FVariant_wrapper {
        #[allow(unused_imports)]
        use super::*;
        #[allow(unused_imports)]
        use super::FVariant_::*;
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct FVariant {
            pub r#type: uint16,
            pub __field0: FVariant__type0,
        }
        unsafe impl Send for FVariant {}
        unsafe impl Sync for FVariant {}
    }
    pub use __FVariant_wrapper::*;
    pub mod FVariant_ {
        #[allow(unused_imports)]
        use super::*;
        pub const kEmpty: crate::support::DefaultEnumType = 0;
        pub const kFloat: crate::support::DefaultEnumType = 2;
        pub const kInteger: crate::support::DefaultEnumType = 1;
        pub const kObject: crate::support::DefaultEnumType = 8;
        pub const kOwner: crate::support::DefaultEnumType = 16;
        pub const kString16: crate::support::DefaultEnumType = 32;
        pub const kString8: crate::support::DefaultEnumType = 4;
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union FVariant__type0 {
        pub intValue: int64,
        pub floatValue: f64,
        pub string8: *const char8,
        pub string16: *const char16,
        pub object: *mut FUnknown,
    }
    unsafe impl Send for FVariant__type0 {}
    unsafe impl Sync for FVariant__type0 {}
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IAttributes {
        pub vtbl: *const IAttributesVtbl,
    }
    unsafe impl Send for IAttributes {}
    unsafe impl Sync for IAttributes {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IAttributes {}
    impl ::com_scrape_types::Unknown for IAttributes {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IAttributes {
        type Vtbl = IAttributesVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IAttributes_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IAttributesVtbl {
        pub base: FUnknownVtbl,
        pub set: unsafe extern "system" fn(
            this: *mut IAttributes,
            attrID: IAttrID,
            data: *const FVariant,
        ) -> tresult,
        pub queue: unsafe extern "system" fn(
            this: *mut IAttributes,
            listID: IAttrID,
            data: *const FVariant,
        ) -> tresult,
        pub setBinaryData: unsafe extern "system" fn(
            this: *mut IAttributes,
            attrID: IAttrID,
            data: *mut ::std::ffi::c_void,
            bytes: uint32,
            copyBytes: bool,
        ) -> tresult,
        pub get: unsafe extern "system" fn(
            this: *mut IAttributes,
            attrID: IAttrID,
            data: *mut FVariant,
        ) -> tresult,
        pub unqueue: unsafe extern "system" fn(
            this: *mut IAttributes,
            listID: IAttrID,
            data: *mut FVariant,
        ) -> tresult,
        pub getQueueItemCount: unsafe extern "system" fn(
            this: *mut IAttributes,
            attrId: IAttrID,
        ) -> int32,
        pub resetQueue: unsafe extern "system" fn(
            this: *mut IAttributes,
            attrID: IAttrID,
        ) -> tresult,
        pub resetAllQueues: unsafe extern "system" fn(
            this: *mut IAttributes,
        ) -> tresult,
        pub getBinaryData: unsafe extern "system" fn(
            this: *mut IAttributes,
            attrID: IAttrID,
            data: *mut ::std::ffi::c_void,
            bytes: uint32,
        ) -> tresult,
        pub getBinaryDataSize: unsafe extern "system" fn(
            this: *mut IAttributes,
            attrID: IAttrID,
        ) -> uint32,
    }
    pub trait IAttributesTrait {
        unsafe fn set(
            &self,
            attrID: IAttrID,
            data: *const FVariant,
        ) -> tresult;
        unsafe fn queue(
            &self,
            listID: IAttrID,
            data: *const FVariant,
        ) -> tresult;
        unsafe fn setBinaryData(
            &self,
            attrID: IAttrID,
            data: *mut ::std::ffi::c_void,
            bytes: uint32,
            copyBytes: bool,
        ) -> tresult;
        unsafe fn get(
            &self,
            attrID: IAttrID,
            data: *mut FVariant,
        ) -> tresult;
        unsafe fn unqueue(
            &self,
            listID: IAttrID,
            data: *mut FVariant,
        ) -> tresult;
        unsafe fn getQueueItemCount(
            &self,
            attrId: IAttrID,
        ) -> int32;
        unsafe fn resetQueue(
            &self,
            attrID: IAttrID,
        ) -> tresult;
        unsafe fn resetAllQueues(
            &self,
        ) -> tresult;
        unsafe fn getBinaryData(
            &self,
            attrID: IAttrID,
            data: *mut ::std::ffi::c_void,
            bytes: uint32,
        ) -> tresult;
        unsafe fn getBinaryDataSize(
            &self,
            attrID: IAttrID,
        ) -> uint32;
    }
    impl<P> IAttributesTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IAttributes>,
    {
        #[inline]
        unsafe fn set(
            &self,
            attrID: IAttrID,
            data: *const FVariant,
        ) -> tresult {
            let ptr = self.ptr() as *mut IAttributes;
            ((*(*ptr).vtbl).set)(
                ptr,
                attrID,
                data,
            )
        }
        #[inline]
        unsafe fn queue(
            &self,
            listID: IAttrID,
            data: *const FVariant,
        ) -> tresult {
            let ptr = self.ptr() as *mut IAttributes;
            ((*(*ptr).vtbl).queue)(
                ptr,
                listID,
                data,
            )
        }
        #[inline]
        unsafe fn setBinaryData(
            &self,
            attrID: IAttrID,
            data: *mut ::std::ffi::c_void,
            bytes: uint32,
            copyBytes: bool,
        ) -> tresult {
            let ptr = self.ptr() as *mut IAttributes;
            ((*(*ptr).vtbl).setBinaryData)(
                ptr,
                attrID,
                data,
                bytes,
                copyBytes,
            )
        }
        #[inline]
        unsafe fn get(
            &self,
            attrID: IAttrID,
            data: *mut FVariant,
        ) -> tresult {
            let ptr = self.ptr() as *mut IAttributes;
            ((*(*ptr).vtbl).get)(
                ptr,
                attrID,
                data,
            )
        }
        #[inline]
        unsafe fn unqueue(
            &self,
            listID: IAttrID,
            data: *mut FVariant,
        ) -> tresult {
            let ptr = self.ptr() as *mut IAttributes;
            ((*(*ptr).vtbl).unqueue)(
                ptr,
                listID,
                data,
            )
        }
        #[inline]
        unsafe fn getQueueItemCount(
            &self,
            attrId: IAttrID,
        ) -> int32 {
            let ptr = self.ptr() as *mut IAttributes;
            ((*(*ptr).vtbl).getQueueItemCount)(
                ptr,
                attrId,
            )
        }
        #[inline]
        unsafe fn resetQueue(
            &self,
            attrID: IAttrID,
        ) -> tresult {
            let ptr = self.ptr() as *mut IAttributes;
            ((*(*ptr).vtbl).resetQueue)(
                ptr,
                attrID,
            )
        }
        #[inline]
        unsafe fn resetAllQueues(
            &self,
        ) -> tresult {
            let ptr = self.ptr() as *mut IAttributes;
            ((*(*ptr).vtbl).resetAllQueues)(
                ptr,
            )
        }
        #[inline]
        unsafe fn getBinaryData(
            &self,
            attrID: IAttrID,
            data: *mut ::std::ffi::c_void,
            bytes: uint32,
        ) -> tresult {
            let ptr = self.ptr() as *mut IAttributes;
            ((*(*ptr).vtbl).getBinaryData)(
                ptr,
                attrID,
                data,
                bytes,
            )
        }
        #[inline]
        unsafe fn getBinaryDataSize(
            &self,
            attrID: IAttrID,
        ) -> uint32 {
            let ptr = self.ptr() as *mut IAttributes;
            ((*(*ptr).vtbl).getBinaryDataSize)(
                ptr,
                attrID,
            )
        }
    }
    impl IAttributes {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IAttributesVtbl
        where
            C: IAttributesTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn set<C, W, const OFFSET: isize>(
                this: *mut IAttributes,
                attrID: IAttrID,
                data: *const FVariant,
            ) -> tresult
            where
                C: IAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).set(
                    attrID,
                    data,
                )
            }
            unsafe extern "system" fn queue<C, W, const OFFSET: isize>(
                this: *mut IAttributes,
                listID: IAttrID,
                data: *const FVariant,
            ) -> tresult
            where
                C: IAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).queue(
                    listID,
                    data,
                )
            }
            unsafe extern "system" fn setBinaryData<C, W, const OFFSET: isize>(
                this: *mut IAttributes,
                attrID: IAttrID,
                data: *mut ::std::ffi::c_void,
                bytes: uint32,
                copyBytes: bool,
            ) -> tresult
            where
                C: IAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).setBinaryData(
                    attrID,
                    data,
                    bytes,
                    copyBytes,
                )
            }
            unsafe extern "system" fn get<C, W, const OFFSET: isize>(
                this: *mut IAttributes,
                attrID: IAttrID,
                data: *mut FVariant,
            ) -> tresult
            where
                C: IAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).get(
                    attrID,
                    data,
                )
            }
            unsafe extern "system" fn unqueue<C, W, const OFFSET: isize>(
                this: *mut IAttributes,
                listID: IAttrID,
                data: *mut FVariant,
            ) -> tresult
            where
                C: IAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).unqueue(
                    listID,
                    data,
                )
            }
            unsafe extern "system" fn getQueueItemCount<C, W, const OFFSET: isize>(
                this: *mut IAttributes,
                attrId: IAttrID,
            ) -> int32
            where
                C: IAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getQueueItemCount(
                    attrId,
                )
            }
            unsafe extern "system" fn resetQueue<C, W, const OFFSET: isize>(
                this: *mut IAttributes,
                attrID: IAttrID,
            ) -> tresult
            where
                C: IAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).resetQueue(
                    attrID,
                )
            }
            unsafe extern "system" fn resetAllQueues<C, W, const OFFSET: isize>(
                this: *mut IAttributes,
            ) -> tresult
            where
                C: IAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).resetAllQueues(
                )
            }
            unsafe extern "system" fn getBinaryData<C, W, const OFFSET: isize>(
                this: *mut IAttributes,
                attrID: IAttrID,
                data: *mut ::std::ffi::c_void,
                bytes: uint32,
            ) -> tresult
            where
                C: IAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getBinaryData(
                    attrID,
                    data,
                    bytes,
                )
            }
            unsafe extern "system" fn getBinaryDataSize<C, W, const OFFSET: isize>(
                this: *mut IAttributes,
                attrID: IAttrID,
            ) -> uint32
            where
                C: IAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getBinaryDataSize(
                    attrID,
                )
            }
            IAttributesVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                set: set::<C, W, OFFSET>,
                queue: queue::<C, W, OFFSET>,
                setBinaryData: setBinaryData::<C, W, OFFSET>,
                get: get::<C, W, OFFSET>,
                unqueue: unqueue::<C, W, OFFSET>,
                getQueueItemCount: getQueueItemCount::<C, W, OFFSET>,
                resetQueue: resetQueue::<C, W, OFFSET>,
                resetAllQueues: resetAllQueues::<C, W, OFFSET>,
                getBinaryData: getBinaryData::<C, W, OFFSET>,
                getBinaryDataSize: getBinaryDataSize::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IAttributes
    where
        C: IAttributesTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IAttributes {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IAttributes2 {
        pub vtbl: *const IAttributes2Vtbl,
    }
    unsafe impl Send for IAttributes2 {}
    unsafe impl Sync for IAttributes2 {}
    unsafe impl ::com_scrape_types::Inherits<IAttributes> for IAttributes2 {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IAttributes2 {}
    impl ::com_scrape_types::Unknown for IAttributes2 {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IAttributes2 {
        type Vtbl = IAttributes2Vtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IAttributes2_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || IAttributes::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IAttributes2Vtbl {
        pub base: IAttributesVtbl,
        pub countAttributes: unsafe extern "system" fn(
            this: *mut IAttributes2,
        ) -> int32,
        pub getAttributeID: unsafe extern "system" fn(
            this: *mut IAttributes2,
            index: int32,
        ) -> IAttrID,
    }
    pub trait IAttributes2Trait: IAttributesTrait {
        unsafe fn countAttributes(
            &self,
        ) -> int32;
        unsafe fn getAttributeID(
            &self,
            index: int32,
        ) -> IAttrID;
    }
    impl<P> IAttributes2Trait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IAttributes2>,
        P::Target: ::com_scrape_types::Inherits<IAttributes>,
    {
        #[inline]
        unsafe fn countAttributes(
            &self,
        ) -> int32 {
            let ptr = self.ptr() as *mut IAttributes2;
            ((*(*ptr).vtbl).countAttributes)(
                ptr,
            )
        }
        #[inline]
        unsafe fn getAttributeID(
            &self,
            index: int32,
        ) -> IAttrID {
            let ptr = self.ptr() as *mut IAttributes2;
            ((*(*ptr).vtbl).getAttributeID)(
                ptr,
                index,
            )
        }
    }
    impl IAttributes2 {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IAttributes2Vtbl
        where
            C: IAttributes2Trait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn countAttributes<C, W, const OFFSET: isize>(
                this: *mut IAttributes2,
            ) -> int32
            where
                C: IAttributes2Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).countAttributes(
                )
            }
            unsafe extern "system" fn getAttributeID<C, W, const OFFSET: isize>(
                this: *mut IAttributes2,
                index: int32,
            ) -> IAttrID
            where
                C: IAttributes2Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getAttributeID(
                    index,
                )
            }
            IAttributes2Vtbl {
                base: IAttributes::make_vtbl::<C, W, OFFSET>(),
                countAttributes: countAttributes::<C, W, OFFSET>,
                getAttributeID: getAttributeID::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IAttributes2
    where
        C: IAttributes2Trait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IAttributes2 {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    mod __IBStream_wrapper {
        #[allow(unused_imports)]
        use super::*;
        #[allow(unused_imports)]
        use super::IBStream_::*;
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IBStream {
            pub vtbl: *const IBStreamVtbl,
        }
        unsafe impl Send for IBStream {}
        unsafe impl Sync for IBStream {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IBStream {}
        impl ::com_scrape_types::Unknown for IBStream {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IBStream {
            type Vtbl = IBStreamVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IBStream_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IBStreamVtbl {
            pub base: FUnknownVtbl,
            pub read: unsafe extern "system" fn(
                this: *mut IBStream,
                buffer: *mut ::std::ffi::c_void,
                numBytes: int32,
                numBytesRead: *mut int32,
            ) -> tresult,
            pub write: unsafe extern "system" fn(
                this: *mut IBStream,
                buffer: *mut ::std::ffi::c_void,
                numBytes: int32,
                numBytesWritten: *mut int32,
            ) -> tresult,
            pub seek: unsafe extern "system" fn(
                this: *mut IBStream,
                pos: int64,
                mode: int32,
                result: *mut int64,
            ) -> tresult,
            pub tell: unsafe extern "system" fn(
                this: *mut IBStream,
                pos: *mut int64,
            ) -> tresult,
        }
        pub trait IBStreamTrait {
            unsafe fn read(
                &self,
                buffer: *mut ::std::ffi::c_void,
                numBytes: int32,
                numBytesRead: *mut int32,
            ) -> tresult;
            unsafe fn write(
                &self,
                buffer: *mut ::std::ffi::c_void,
                numBytes: int32,
                numBytesWritten: *mut int32,
            ) -> tresult;
            unsafe fn seek(
                &self,
                pos: int64,
                mode: int32,
                result: *mut int64,
            ) -> tresult;
            unsafe fn tell(
                &self,
                pos: *mut int64,
            ) -> tresult;
        }
        impl<P> IBStreamTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IBStream>,
        {
            #[inline]
            unsafe fn read(
                &self,
                buffer: *mut ::std::ffi::c_void,
                numBytes: int32,
                numBytesRead: *mut int32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IBStream;
                ((*(*ptr).vtbl).read)(
                    ptr,
                    buffer,
                    numBytes,
                    numBytesRead,
                )
            }
            #[inline]
            unsafe fn write(
                &self,
                buffer: *mut ::std::ffi::c_void,
                numBytes: int32,
                numBytesWritten: *mut int32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IBStream;
                ((*(*ptr).vtbl).write)(
                    ptr,
                    buffer,
                    numBytes,
                    numBytesWritten,
                )
            }
            #[inline]
            unsafe fn seek(
                &self,
                pos: int64,
                mode: int32,
                result: *mut int64,
            ) -> tresult {
                let ptr = self.ptr() as *mut IBStream;
                ((*(*ptr).vtbl).seek)(
                    ptr,
                    pos,
                    mode,
                    result,
                )
            }
            #[inline]
            unsafe fn tell(
                &self,
                pos: *mut int64,
            ) -> tresult {
                let ptr = self.ptr() as *mut IBStream;
                ((*(*ptr).vtbl).tell)(
                    ptr,
                    pos,
                )
            }
        }
        impl IBStream {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IBStreamVtbl
            where
                C: IBStreamTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn read<C, W, const OFFSET: isize>(
                    this: *mut IBStream,
                    buffer: *mut ::std::ffi::c_void,
                    numBytes: int32,
                    numBytesRead: *mut int32,
                ) -> tresult
                where
                    C: IBStreamTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).read(
                        buffer,
                        numBytes,
                        numBytesRead,
                    )
                }
                unsafe extern "system" fn write<C, W, const OFFSET: isize>(
                    this: *mut IBStream,
                    buffer: *mut ::std::ffi::c_void,
                    numBytes: int32,
                    numBytesWritten: *mut int32,
                ) -> tresult
                where
                    C: IBStreamTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).write(
                        buffer,
                        numBytes,
                        numBytesWritten,
                    )
                }
                unsafe extern "system" fn seek<C, W, const OFFSET: isize>(
                    this: *mut IBStream,
                    pos: int64,
                    mode: int32,
                    result: *mut int64,
                ) -> tresult
                where
                    C: IBStreamTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).seek(
                        pos,
                        mode,
                        result,
                    )
                }
                unsafe extern "system" fn tell<C, W, const OFFSET: isize>(
                    this: *mut IBStream,
                    pos: *mut int64,
                ) -> tresult
                where
                    C: IBStreamTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).tell(
                        pos,
                    )
                }
                IBStreamVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    read: read::<C, W, OFFSET>,
                    write: write::<C, W, OFFSET>,
                    seek: seek::<C, W, OFFSET>,
                    tell: tell::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IBStream
        where
            C: IBStreamTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IBStream {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
    }
    pub use __IBStream_wrapper::*;
    pub mod IBStream_ {
        #[allow(unused_imports)]
        use super::*;
        pub type IStreamSeekMode = crate::support::DefaultEnumType;
        pub mod IStreamSeekMode_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kIBSeekCur: IStreamSeekMode = 1;
            pub const kIBSeekEnd: IStreamSeekMode = 2;
            pub const kIBSeekSet: IStreamSeekMode = 0;
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ICloneable {
        pub vtbl: *const ICloneableVtbl,
    }
    unsafe impl Send for ICloneable {}
    unsafe impl Sync for ICloneable {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for ICloneable {}
    impl ::com_scrape_types::Unknown for ICloneable {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for ICloneable {
        type Vtbl = ICloneableVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(ICloneable_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ICloneableVtbl {
        pub base: FUnknownVtbl,
        pub clone: unsafe extern "system" fn(
            this: *mut ICloneable,
        ) -> *mut FUnknown,
    }
    pub trait ICloneableTrait {
        unsafe fn clone(
            &self,
        ) -> *mut FUnknown;
    }
    impl<P> ICloneableTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<ICloneable>,
    {
        #[inline]
        unsafe fn clone(
            &self,
        ) -> *mut FUnknown {
            let ptr = self.ptr() as *mut ICloneable;
            ((*(*ptr).vtbl).clone)(
                ptr,
            )
        }
    }
    impl ICloneable {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> ICloneableVtbl
        where
            C: ICloneableTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn clone<C, W, const OFFSET: isize>(
                this: *mut ICloneable,
            ) -> *mut FUnknown
            where
                C: ICloneableTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).clone(
                )
            }
            ICloneableVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                clone: clone::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for ICloneable
    where
        C: ICloneableTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = ICloneable {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    mod __IDependent_wrapper {
        #[allow(unused_imports)]
        use super::*;
        #[allow(unused_imports)]
        use super::IDependent_::*;
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IDependent {
            pub vtbl: *const IDependentVtbl,
        }
        unsafe impl Send for IDependent {}
        unsafe impl Sync for IDependent {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IDependent {}
        impl ::com_scrape_types::Unknown for IDependent {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IDependent {
            type Vtbl = IDependentVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IDependent_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IDependentVtbl {
            pub base: FUnknownVtbl,
            pub update: unsafe extern "system" fn(
                this: *mut IDependent,
                changedUnknown: *mut FUnknown,
                message: int32,
            ),
        }
        pub trait IDependentTrait {
            unsafe fn update(
                &self,
                changedUnknown: *mut FUnknown,
                message: int32,
            );
        }
        impl<P> IDependentTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IDependent>,
        {
            #[inline]
            unsafe fn update(
                &self,
                changedUnknown: *mut FUnknown,
                message: int32,
            ) {
                let ptr = self.ptr() as *mut IDependent;
                ((*(*ptr).vtbl).update)(
                    ptr,
                    changedUnknown,
                    message,
                )
            }
        }
        impl IDependent {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IDependentVtbl
            where
                C: IDependentTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn update<C, W, const OFFSET: isize>(
                    this: *mut IDependent,
                    changedUnknown: *mut FUnknown,
                    message: int32,
                )
                where
                    C: IDependentTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).update(
                        changedUnknown,
                        message,
                    )
                }
                IDependentVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    update: update::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IDependent
        where
            C: IDependentTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IDependent {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
    }
    pub use __IDependent_wrapper::*;
    pub mod IDependent_ {
        #[allow(unused_imports)]
        use super::*;
        pub type ChangeMessage = crate::support::DefaultEnumType;
        pub mod ChangeMessage_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kChanged: ChangeMessage = 1;
            pub const kDestroyed: ChangeMessage = 2;
            pub const kStdChangeMessageLast: ChangeMessage = 3;
            pub const kWillChange: ChangeMessage = 0;
            pub const kWillDestroy: ChangeMessage = 3;
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IErrorContext {
        pub vtbl: *const IErrorContextVtbl,
    }
    unsafe impl Send for IErrorContext {}
    unsafe impl Sync for IErrorContext {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IErrorContext {}
    impl ::com_scrape_types::Unknown for IErrorContext {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IErrorContext {
        type Vtbl = IErrorContextVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IErrorContext_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IErrorContextVtbl {
        pub base: FUnknownVtbl,
        pub disableErrorUI: unsafe extern "system" fn(
            this: *mut IErrorContext,
            state: bool,
        ),
        pub errorMessageShown: unsafe extern "system" fn(
            this: *mut IErrorContext,
        ) -> tresult,
        pub getErrorMessage: unsafe extern "system" fn(
            this: *mut IErrorContext,
            message: *mut IString,
        ) -> tresult,
    }
    pub trait IErrorContextTrait {
        unsafe fn disableErrorUI(
            &self,
            state: bool,
        );
        unsafe fn errorMessageShown(
            &self,
        ) -> tresult;
        unsafe fn getErrorMessage(
            &self,
            message: *mut IString,
        ) -> tresult;
    }
    impl<P> IErrorContextTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IErrorContext>,
    {
        #[inline]
        unsafe fn disableErrorUI(
            &self,
            state: bool,
        ) {
            let ptr = self.ptr() as *mut IErrorContext;
            ((*(*ptr).vtbl).disableErrorUI)(
                ptr,
                state,
            )
        }
        #[inline]
        unsafe fn errorMessageShown(
            &self,
        ) -> tresult {
            let ptr = self.ptr() as *mut IErrorContext;
            ((*(*ptr).vtbl).errorMessageShown)(
                ptr,
            )
        }
        #[inline]
        unsafe fn getErrorMessage(
            &self,
            message: *mut IString,
        ) -> tresult {
            let ptr = self.ptr() as *mut IErrorContext;
            ((*(*ptr).vtbl).getErrorMessage)(
                ptr,
                message,
            )
        }
    }
    impl IErrorContext {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IErrorContextVtbl
        where
            C: IErrorContextTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn disableErrorUI<C, W, const OFFSET: isize>(
                this: *mut IErrorContext,
                state: bool,
            )
            where
                C: IErrorContextTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).disableErrorUI(
                    state,
                )
            }
            unsafe extern "system" fn errorMessageShown<C, W, const OFFSET: isize>(
                this: *mut IErrorContext,
            ) -> tresult
            where
                C: IErrorContextTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).errorMessageShown(
                )
            }
            unsafe extern "system" fn getErrorMessage<C, W, const OFFSET: isize>(
                this: *mut IErrorContext,
                message: *mut IString,
            ) -> tresult
            where
                C: IErrorContextTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getErrorMessage(
                    message,
                )
            }
            IErrorContextVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                disableErrorUI: disableErrorUI::<C, W, OFFSET>,
                errorMessageShown: errorMessageShown::<C, W, OFFSET>,
                getErrorMessage: getErrorMessage::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IErrorContext
    where
        C: IErrorContextTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IErrorContext {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPersistent {
        pub vtbl: *const IPersistentVtbl,
    }
    unsafe impl Send for IPersistent {}
    unsafe impl Sync for IPersistent {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPersistent {}
    impl ::com_scrape_types::Unknown for IPersistent {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IPersistent {
        type Vtbl = IPersistentVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPersistent_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPersistentVtbl {
        pub base: FUnknownVtbl,
        pub getClassID: unsafe extern "system" fn(
            this: *mut IPersistent,
            uid: *mut char8,
        ) -> tresult,
        pub saveAttributes: unsafe extern "system" fn(
            this: *mut IPersistent,
            _0: *mut IAttributes,
        ) -> tresult,
        pub loadAttributes: unsafe extern "system" fn(
            this: *mut IPersistent,
            _0: *mut IAttributes,
        ) -> tresult,
    }
    pub trait IPersistentTrait {
        unsafe fn getClassID(
            &self,
            uid: *mut char8,
        ) -> tresult;
        unsafe fn saveAttributes(
            &self,
            _0: *mut IAttributes,
        ) -> tresult;
        unsafe fn loadAttributes(
            &self,
            _0: *mut IAttributes,
        ) -> tresult;
    }
    impl<P> IPersistentTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IPersistent>,
    {
        #[inline]
        unsafe fn getClassID(
            &self,
            uid: *mut char8,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPersistent;
            ((*(*ptr).vtbl).getClassID)(
                ptr,
                uid,
            )
        }
        #[inline]
        unsafe fn saveAttributes(
            &self,
            _0: *mut IAttributes,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPersistent;
            ((*(*ptr).vtbl).saveAttributes)(
                ptr,
                _0,
            )
        }
        #[inline]
        unsafe fn loadAttributes(
            &self,
            _0: *mut IAttributes,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPersistent;
            ((*(*ptr).vtbl).loadAttributes)(
                ptr,
                _0,
            )
        }
    }
    impl IPersistent {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IPersistentVtbl
        where
            C: IPersistentTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn getClassID<C, W, const OFFSET: isize>(
                this: *mut IPersistent,
                uid: *mut char8,
            ) -> tresult
            where
                C: IPersistentTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getClassID(
                    uid,
                )
            }
            unsafe extern "system" fn saveAttributes<C, W, const OFFSET: isize>(
                this: *mut IPersistent,
                _0: *mut IAttributes,
            ) -> tresult
            where
                C: IPersistentTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).saveAttributes(
                    _0,
                )
            }
            unsafe extern "system" fn loadAttributes<C, W, const OFFSET: isize>(
                this: *mut IPersistent,
                _0: *mut IAttributes,
            ) -> tresult
            where
                C: IPersistentTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).loadAttributes(
                    _0,
                )
            }
            IPersistentVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                getClassID: getClassID::<C, W, OFFSET>,
                saveAttributes: saveAttributes::<C, W, OFFSET>,
                loadAttributes: loadAttributes::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPersistent
    where
        C: IPersistentTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IPersistent {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPlugFrame {
        pub vtbl: *const IPlugFrameVtbl,
    }
    unsafe impl Send for IPlugFrame {}
    unsafe impl Sync for IPlugFrame {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPlugFrame {}
    impl ::com_scrape_types::Unknown for IPlugFrame {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IPlugFrame {
        type Vtbl = IPlugFrameVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPlugFrame_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPlugFrameVtbl {
        pub base: FUnknownVtbl,
        pub resizeView: unsafe extern "system" fn(
            this: *mut IPlugFrame,
            view: *mut IPlugView,
            newSize: *mut ViewRect,
        ) -> tresult,
    }
    pub trait IPlugFrameTrait {
        unsafe fn resizeView(
            &self,
            view: *mut IPlugView,
            newSize: *mut ViewRect,
        ) -> tresult;
    }
    impl<P> IPlugFrameTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IPlugFrame>,
    {
        #[inline]
        unsafe fn resizeView(
            &self,
            view: *mut IPlugView,
            newSize: *mut ViewRect,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugFrame;
            ((*(*ptr).vtbl).resizeView)(
                ptr,
                view,
                newSize,
            )
        }
    }
    impl IPlugFrame {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IPlugFrameVtbl
        where
            C: IPlugFrameTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn resizeView<C, W, const OFFSET: isize>(
                this: *mut IPlugFrame,
                view: *mut IPlugView,
                newSize: *mut ViewRect,
            ) -> tresult
            where
                C: IPlugFrameTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).resizeView(
                    view,
                    newSize,
                )
            }
            IPlugFrameVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                resizeView: resizeView::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPlugFrame
    where
        C: IPlugFrameTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IPlugFrame {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPlugView {
        pub vtbl: *const IPlugViewVtbl,
    }
    unsafe impl Send for IPlugView {}
    unsafe impl Sync for IPlugView {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPlugView {}
    impl ::com_scrape_types::Unknown for IPlugView {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IPlugView {
        type Vtbl = IPlugViewVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPlugView_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPlugViewVtbl {
        pub base: FUnknownVtbl,
        pub isPlatformTypeSupported: unsafe extern "system" fn(
            this: *mut IPlugView,
            r#type: FIDString,
        ) -> tresult,
        pub attached: unsafe extern "system" fn(
            this: *mut IPlugView,
            parent: *mut ::std::ffi::c_void,
            r#type: FIDString,
        ) -> tresult,
        pub removed: unsafe extern "system" fn(
            this: *mut IPlugView,
        ) -> tresult,
        pub onWheel: unsafe extern "system" fn(
            this: *mut IPlugView,
            distance: f32,
        ) -> tresult,
        pub onKeyDown: unsafe extern "system" fn(
            this: *mut IPlugView,
            key: char16,
            keyCode: int16,
            modifiers: int16,
        ) -> tresult,
        pub onKeyUp: unsafe extern "system" fn(
            this: *mut IPlugView,
            key: char16,
            keyCode: int16,
            modifiers: int16,
        ) -> tresult,
        pub getSize: unsafe extern "system" fn(
            this: *mut IPlugView,
            size: *mut ViewRect,
        ) -> tresult,
        pub onSize: unsafe extern "system" fn(
            this: *mut IPlugView,
            newSize: *mut ViewRect,
        ) -> tresult,
        pub onFocus: unsafe extern "system" fn(
            this: *mut IPlugView,
            state: TBool,
        ) -> tresult,
        pub setFrame: unsafe extern "system" fn(
            this: *mut IPlugView,
            frame: *mut IPlugFrame,
        ) -> tresult,
        pub canResize: unsafe extern "system" fn(
            this: *mut IPlugView,
        ) -> tresult,
        pub checkSizeConstraint: unsafe extern "system" fn(
            this: *mut IPlugView,
            rect: *mut ViewRect,
        ) -> tresult,
    }
    pub trait IPlugViewTrait {
        unsafe fn isPlatformTypeSupported(
            &self,
            r#type: FIDString,
        ) -> tresult;
        unsafe fn attached(
            &self,
            parent: *mut ::std::ffi::c_void,
            r#type: FIDString,
        ) -> tresult;
        unsafe fn removed(
            &self,
        ) -> tresult;
        unsafe fn onWheel(
            &self,
            distance: f32,
        ) -> tresult;
        unsafe fn onKeyDown(
            &self,
            key: char16,
            keyCode: int16,
            modifiers: int16,
        ) -> tresult;
        unsafe fn onKeyUp(
            &self,
            key: char16,
            keyCode: int16,
            modifiers: int16,
        ) -> tresult;
        unsafe fn getSize(
            &self,
            size: *mut ViewRect,
        ) -> tresult;
        unsafe fn onSize(
            &self,
            newSize: *mut ViewRect,
        ) -> tresult;
        unsafe fn onFocus(
            &self,
            state: TBool,
        ) -> tresult;
        unsafe fn setFrame(
            &self,
            frame: *mut IPlugFrame,
        ) -> tresult;
        unsafe fn canResize(
            &self,
        ) -> tresult;
        unsafe fn checkSizeConstraint(
            &self,
            rect: *mut ViewRect,
        ) -> tresult;
    }
    impl<P> IPlugViewTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IPlugView>,
    {
        #[inline]
        unsafe fn isPlatformTypeSupported(
            &self,
            r#type: FIDString,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).isPlatformTypeSupported)(
                ptr,
                r#type,
            )
        }
        #[inline]
        unsafe fn attached(
            &self,
            parent: *mut ::std::ffi::c_void,
            r#type: FIDString,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).attached)(
                ptr,
                parent,
                r#type,
            )
        }
        #[inline]
        unsafe fn removed(
            &self,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).removed)(
                ptr,
            )
        }
        #[inline]
        unsafe fn onWheel(
            &self,
            distance: f32,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).onWheel)(
                ptr,
                distance,
            )
        }
        #[inline]
        unsafe fn onKeyDown(
            &self,
            key: char16,
            keyCode: int16,
            modifiers: int16,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).onKeyDown)(
                ptr,
                key,
                keyCode,
                modifiers,
            )
        }
        #[inline]
        unsafe fn onKeyUp(
            &self,
            key: char16,
            keyCode: int16,
            modifiers: int16,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).onKeyUp)(
                ptr,
                key,
                keyCode,
                modifiers,
            )
        }
        #[inline]
        unsafe fn getSize(
            &self,
            size: *mut ViewRect,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).getSize)(
                ptr,
                size,
            )
        }
        #[inline]
        unsafe fn onSize(
            &self,
            newSize: *mut ViewRect,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).onSize)(
                ptr,
                newSize,
            )
        }
        #[inline]
        unsafe fn onFocus(
            &self,
            state: TBool,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).onFocus)(
                ptr,
                state,
            )
        }
        #[inline]
        unsafe fn setFrame(
            &self,
            frame: *mut IPlugFrame,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).setFrame)(
                ptr,
                frame,
            )
        }
        #[inline]
        unsafe fn canResize(
            &self,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).canResize)(
                ptr,
            )
        }
        #[inline]
        unsafe fn checkSizeConstraint(
            &self,
            rect: *mut ViewRect,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPlugView;
            ((*(*ptr).vtbl).checkSizeConstraint)(
                ptr,
                rect,
            )
        }
    }
    impl IPlugView {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IPlugViewVtbl
        where
            C: IPlugViewTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn isPlatformTypeSupported<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
                r#type: FIDString,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).isPlatformTypeSupported(
                    r#type,
                )
            }
            unsafe extern "system" fn attached<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
                parent: *mut ::std::ffi::c_void,
                r#type: FIDString,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).attached(
                    parent,
                    r#type,
                )
            }
            unsafe extern "system" fn removed<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).removed(
                )
            }
            unsafe extern "system" fn onWheel<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
                distance: f32,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).onWheel(
                    distance,
                )
            }
            unsafe extern "system" fn onKeyDown<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
                key: char16,
                keyCode: int16,
                modifiers: int16,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).onKeyDown(
                    key,
                    keyCode,
                    modifiers,
                )
            }
            unsafe extern "system" fn onKeyUp<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
                key: char16,
                keyCode: int16,
                modifiers: int16,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).onKeyUp(
                    key,
                    keyCode,
                    modifiers,
                )
            }
            unsafe extern "system" fn getSize<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
                size: *mut ViewRect,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getSize(
                    size,
                )
            }
            unsafe extern "system" fn onSize<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
                newSize: *mut ViewRect,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).onSize(
                    newSize,
                )
            }
            unsafe extern "system" fn onFocus<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
                state: TBool,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).onFocus(
                    state,
                )
            }
            unsafe extern "system" fn setFrame<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
                frame: *mut IPlugFrame,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).setFrame(
                    frame,
                )
            }
            unsafe extern "system" fn canResize<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).canResize(
                )
            }
            unsafe extern "system" fn checkSizeConstraint<C, W, const OFFSET: isize>(
                this: *mut IPlugView,
                rect: *mut ViewRect,
            ) -> tresult
            where
                C: IPlugViewTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).checkSizeConstraint(
                    rect,
                )
            }
            IPlugViewVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                isPlatformTypeSupported: isPlatformTypeSupported::<C, W, OFFSET>,
                attached: attached::<C, W, OFFSET>,
                removed: removed::<C, W, OFFSET>,
                onWheel: onWheel::<C, W, OFFSET>,
                onKeyDown: onKeyDown::<C, W, OFFSET>,
                onKeyUp: onKeyUp::<C, W, OFFSET>,
                getSize: getSize::<C, W, OFFSET>,
                onSize: onSize::<C, W, OFFSET>,
                onFocus: onFocus::<C, W, OFFSET>,
                setFrame: setFrame::<C, W, OFFSET>,
                canResize: canResize::<C, W, OFFSET>,
                checkSizeConstraint: checkSizeConstraint::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPlugView
    where
        C: IPlugViewTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IPlugView {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    mod __IPlugViewContentScaleSupport_wrapper {
        #[allow(unused_imports)]
        use super::*;
        #[allow(unused_imports)]
        use super::IPlugViewContentScaleSupport_::*;
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IPlugViewContentScaleSupport {
            pub vtbl: *const IPlugViewContentScaleSupportVtbl,
        }
        unsafe impl Send for IPlugViewContentScaleSupport {}
        unsafe impl Sync for IPlugViewContentScaleSupport {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPlugViewContentScaleSupport {}
        impl ::com_scrape_types::Unknown for IPlugViewContentScaleSupport {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IPlugViewContentScaleSupport {
            type Vtbl = IPlugViewContentScaleSupportVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPlugViewContentScaleSupport_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IPlugViewContentScaleSupportVtbl {
            pub base: FUnknownVtbl,
            pub setContentScaleFactor: unsafe extern "system" fn(
                this: *mut IPlugViewContentScaleSupport,
                factor: ScaleFactor,
            ) -> tresult,
        }
        pub trait IPlugViewContentScaleSupportTrait {
            unsafe fn setContentScaleFactor(
                &self,
                factor: ScaleFactor,
            ) -> tresult;
        }
        impl<P> IPlugViewContentScaleSupportTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IPlugViewContentScaleSupport>,
        {
            #[inline]
            unsafe fn setContentScaleFactor(
                &self,
                factor: ScaleFactor,
            ) -> tresult {
                let ptr = self.ptr() as *mut IPlugViewContentScaleSupport;
                ((*(*ptr).vtbl).setContentScaleFactor)(
                    ptr,
                    factor,
                )
            }
        }
        impl IPlugViewContentScaleSupport {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IPlugViewContentScaleSupportVtbl
            where
                C: IPlugViewContentScaleSupportTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn setContentScaleFactor<C, W, const OFFSET: isize>(
                    this: *mut IPlugViewContentScaleSupport,
                    factor: ScaleFactor,
                ) -> tresult
                where
                    C: IPlugViewContentScaleSupportTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setContentScaleFactor(
                        factor,
                    )
                }
                IPlugViewContentScaleSupportVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    setContentScaleFactor: setContentScaleFactor::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPlugViewContentScaleSupport
        where
            C: IPlugViewContentScaleSupportTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IPlugViewContentScaleSupport {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
    }
    pub use __IPlugViewContentScaleSupport_wrapper::*;
    pub mod IPlugViewContentScaleSupport_ {
        #[allow(unused_imports)]
        use super::*;
        pub type ScaleFactor = f32;
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPluginBase {
        pub vtbl: *const IPluginBaseVtbl,
    }
    unsafe impl Send for IPluginBase {}
    unsafe impl Sync for IPluginBase {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPluginBase {}
    impl ::com_scrape_types::Unknown for IPluginBase {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IPluginBase {
        type Vtbl = IPluginBaseVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPluginBase_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPluginBaseVtbl {
        pub base: FUnknownVtbl,
        pub initialize: unsafe extern "system" fn(
            this: *mut IPluginBase,
            context: *mut FUnknown,
        ) -> tresult,
        pub terminate: unsafe extern "system" fn(
            this: *mut IPluginBase,
        ) -> tresult,
    }
    pub trait IPluginBaseTrait {
        unsafe fn initialize(
            &self,
            context: *mut FUnknown,
        ) -> tresult;
        unsafe fn terminate(
            &self,
        ) -> tresult;
    }
    impl<P> IPluginBaseTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IPluginBase>,
    {
        #[inline]
        unsafe fn initialize(
            &self,
            context: *mut FUnknown,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPluginBase;
            ((*(*ptr).vtbl).initialize)(
                ptr,
                context,
            )
        }
        #[inline]
        unsafe fn terminate(
            &self,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPluginBase;
            ((*(*ptr).vtbl).terminate)(
                ptr,
            )
        }
    }
    impl IPluginBase {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IPluginBaseVtbl
        where
            C: IPluginBaseTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn initialize<C, W, const OFFSET: isize>(
                this: *mut IPluginBase,
                context: *mut FUnknown,
            ) -> tresult
            where
                C: IPluginBaseTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).initialize(
                    context,
                )
            }
            unsafe extern "system" fn terminate<C, W, const OFFSET: isize>(
                this: *mut IPluginBase,
            ) -> tresult
            where
                C: IPluginBaseTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).terminate(
                )
            }
            IPluginBaseVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                initialize: initialize::<C, W, OFFSET>,
                terminate: terminate::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPluginBase
    where
        C: IPluginBaseTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IPluginBase {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPluginCompatibility {
        pub vtbl: *const IPluginCompatibilityVtbl,
    }
    unsafe impl Send for IPluginCompatibility {}
    unsafe impl Sync for IPluginCompatibility {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPluginCompatibility {}
    impl ::com_scrape_types::Unknown for IPluginCompatibility {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IPluginCompatibility {
        type Vtbl = IPluginCompatibilityVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPluginCompatibility_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPluginCompatibilityVtbl {
        pub base: FUnknownVtbl,
        pub getCompatibilityJSON: unsafe extern "system" fn(
            this: *mut IPluginCompatibility,
            stream: *mut IBStream,
        ) -> tresult,
    }
    pub trait IPluginCompatibilityTrait {
        unsafe fn getCompatibilityJSON(
            &self,
            stream: *mut IBStream,
        ) -> tresult;
    }
    impl<P> IPluginCompatibilityTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IPluginCompatibility>,
    {
        #[inline]
        unsafe fn getCompatibilityJSON(
            &self,
            stream: *mut IBStream,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPluginCompatibility;
            ((*(*ptr).vtbl).getCompatibilityJSON)(
                ptr,
                stream,
            )
        }
    }
    impl IPluginCompatibility {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IPluginCompatibilityVtbl
        where
            C: IPluginCompatibilityTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn getCompatibilityJSON<C, W, const OFFSET: isize>(
                this: *mut IPluginCompatibility,
                stream: *mut IBStream,
            ) -> tresult
            where
                C: IPluginCompatibilityTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getCompatibilityJSON(
                    stream,
                )
            }
            IPluginCompatibilityVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                getCompatibilityJSON: getCompatibilityJSON::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPluginCompatibility
    where
        C: IPluginCompatibilityTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IPluginCompatibility {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPluginFactory {
        pub vtbl: *const IPluginFactoryVtbl,
    }
    unsafe impl Send for IPluginFactory {}
    unsafe impl Sync for IPluginFactory {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPluginFactory {}
    impl ::com_scrape_types::Unknown for IPluginFactory {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IPluginFactory {
        type Vtbl = IPluginFactoryVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPluginFactory_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPluginFactoryVtbl {
        pub base: FUnknownVtbl,
        pub getFactoryInfo: unsafe extern "system" fn(
            this: *mut IPluginFactory,
            info: *mut PFactoryInfo,
        ) -> tresult,
        pub countClasses: unsafe extern "system" fn(
            this: *mut IPluginFactory,
        ) -> int32,
        pub getClassInfo: unsafe extern "system" fn(
            this: *mut IPluginFactory,
            index: int32,
            info: *mut PClassInfo,
        ) -> tresult,
        pub createInstance: unsafe extern "system" fn(
            this: *mut IPluginFactory,
            cid: FIDString,
            _iid: FIDString,
            obj: *mut *mut ::std::ffi::c_void,
        ) -> tresult,
    }
    pub trait IPluginFactoryTrait {
        unsafe fn getFactoryInfo(
            &self,
            info: *mut PFactoryInfo,
        ) -> tresult;
        unsafe fn countClasses(
            &self,
        ) -> int32;
        unsafe fn getClassInfo(
            &self,
            index: int32,
            info: *mut PClassInfo,
        ) -> tresult;
        unsafe fn createInstance(
            &self,
            cid: FIDString,
            _iid: FIDString,
            obj: *mut *mut ::std::ffi::c_void,
        ) -> tresult;
    }
    impl<P> IPluginFactoryTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IPluginFactory>,
    {
        #[inline]
        unsafe fn getFactoryInfo(
            &self,
            info: *mut PFactoryInfo,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPluginFactory;
            ((*(*ptr).vtbl).getFactoryInfo)(
                ptr,
                info,
            )
        }
        #[inline]
        unsafe fn countClasses(
            &self,
        ) -> int32 {
            let ptr = self.ptr() as *mut IPluginFactory;
            ((*(*ptr).vtbl).countClasses)(
                ptr,
            )
        }
        #[inline]
        unsafe fn getClassInfo(
            &self,
            index: int32,
            info: *mut PClassInfo,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPluginFactory;
            ((*(*ptr).vtbl).getClassInfo)(
                ptr,
                index,
                info,
            )
        }
        #[inline]
        unsafe fn createInstance(
            &self,
            cid: FIDString,
            _iid: FIDString,
            obj: *mut *mut ::std::ffi::c_void,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPluginFactory;
            ((*(*ptr).vtbl).createInstance)(
                ptr,
                cid,
                _iid,
                obj,
            )
        }
    }
    impl IPluginFactory {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IPluginFactoryVtbl
        where
            C: IPluginFactoryTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn getFactoryInfo<C, W, const OFFSET: isize>(
                this: *mut IPluginFactory,
                info: *mut PFactoryInfo,
            ) -> tresult
            where
                C: IPluginFactoryTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getFactoryInfo(
                    info,
                )
            }
            unsafe extern "system" fn countClasses<C, W, const OFFSET: isize>(
                this: *mut IPluginFactory,
            ) -> int32
            where
                C: IPluginFactoryTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).countClasses(
                )
            }
            unsafe extern "system" fn getClassInfo<C, W, const OFFSET: isize>(
                this: *mut IPluginFactory,
                index: int32,
                info: *mut PClassInfo,
            ) -> tresult
            where
                C: IPluginFactoryTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getClassInfo(
                    index,
                    info,
                )
            }
            unsafe extern "system" fn createInstance<C, W, const OFFSET: isize>(
                this: *mut IPluginFactory,
                cid: FIDString,
                _iid: FIDString,
                obj: *mut *mut ::std::ffi::c_void,
            ) -> tresult
            where
                C: IPluginFactoryTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).createInstance(
                    cid,
                    _iid,
                    obj,
                )
            }
            IPluginFactoryVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                getFactoryInfo: getFactoryInfo::<C, W, OFFSET>,
                countClasses: countClasses::<C, W, OFFSET>,
                getClassInfo: getClassInfo::<C, W, OFFSET>,
                createInstance: createInstance::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPluginFactory
    where
        C: IPluginFactoryTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IPluginFactory {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPluginFactory2 {
        pub vtbl: *const IPluginFactory2Vtbl,
    }
    unsafe impl Send for IPluginFactory2 {}
    unsafe impl Sync for IPluginFactory2 {}
    unsafe impl ::com_scrape_types::Inherits<IPluginFactory> for IPluginFactory2 {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPluginFactory2 {}
    impl ::com_scrape_types::Unknown for IPluginFactory2 {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IPluginFactory2 {
        type Vtbl = IPluginFactory2Vtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPluginFactory2_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || IPluginFactory::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPluginFactory2Vtbl {
        pub base: IPluginFactoryVtbl,
        pub getClassInfo2: unsafe extern "system" fn(
            this: *mut IPluginFactory2,
            index: int32,
            info: *mut PClassInfo2,
        ) -> tresult,
    }
    pub trait IPluginFactory2Trait: IPluginFactoryTrait {
        unsafe fn getClassInfo2(
            &self,
            index: int32,
            info: *mut PClassInfo2,
        ) -> tresult;
    }
    impl<P> IPluginFactory2Trait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IPluginFactory2>,
        P::Target: ::com_scrape_types::Inherits<IPluginFactory>,
    {
        #[inline]
        unsafe fn getClassInfo2(
            &self,
            index: int32,
            info: *mut PClassInfo2,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPluginFactory2;
            ((*(*ptr).vtbl).getClassInfo2)(
                ptr,
                index,
                info,
            )
        }
    }
    impl IPluginFactory2 {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IPluginFactory2Vtbl
        where
            C: IPluginFactory2Trait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn getClassInfo2<C, W, const OFFSET: isize>(
                this: *mut IPluginFactory2,
                index: int32,
                info: *mut PClassInfo2,
            ) -> tresult
            where
                C: IPluginFactory2Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getClassInfo2(
                    index,
                    info,
                )
            }
            IPluginFactory2Vtbl {
                base: IPluginFactory::make_vtbl::<C, W, OFFSET>(),
                getClassInfo2: getClassInfo2::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPluginFactory2
    where
        C: IPluginFactory2Trait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IPluginFactory2 {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPluginFactory3 {
        pub vtbl: *const IPluginFactory3Vtbl,
    }
    unsafe impl Send for IPluginFactory3 {}
    unsafe impl Sync for IPluginFactory3 {}
    unsafe impl ::com_scrape_types::Inherits<IPluginFactory2> for IPluginFactory3 {}
    unsafe impl ::com_scrape_types::Inherits<IPluginFactory> for IPluginFactory3 {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPluginFactory3 {}
    impl ::com_scrape_types::Unknown for IPluginFactory3 {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IPluginFactory3 {
        type Vtbl = IPluginFactory3Vtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPluginFactory3_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || IPluginFactory2::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IPluginFactory3Vtbl {
        pub base: IPluginFactory2Vtbl,
        pub getClassInfoUnicode: unsafe extern "system" fn(
            this: *mut IPluginFactory3,
            index: int32,
            info: *mut PClassInfoW,
        ) -> tresult,
        pub setHostContext: unsafe extern "system" fn(
            this: *mut IPluginFactory3,
            context: *mut FUnknown,
        ) -> tresult,
    }
    pub trait IPluginFactory3Trait: IPluginFactory2Trait {
        unsafe fn getClassInfoUnicode(
            &self,
            index: int32,
            info: *mut PClassInfoW,
        ) -> tresult;
        unsafe fn setHostContext(
            &self,
            context: *mut FUnknown,
        ) -> tresult;
    }
    impl<P> IPluginFactory3Trait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IPluginFactory3>,
        P::Target: ::com_scrape_types::Inherits<IPluginFactory2>,
        P::Target: ::com_scrape_types::Inherits<IPluginFactory>,
    {
        #[inline]
        unsafe fn getClassInfoUnicode(
            &self,
            index: int32,
            info: *mut PClassInfoW,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPluginFactory3;
            ((*(*ptr).vtbl).getClassInfoUnicode)(
                ptr,
                index,
                info,
            )
        }
        #[inline]
        unsafe fn setHostContext(
            &self,
            context: *mut FUnknown,
        ) -> tresult {
            let ptr = self.ptr() as *mut IPluginFactory3;
            ((*(*ptr).vtbl).setHostContext)(
                ptr,
                context,
            )
        }
    }
    impl IPluginFactory3 {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IPluginFactory3Vtbl
        where
            C: IPluginFactory3Trait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn getClassInfoUnicode<C, W, const OFFSET: isize>(
                this: *mut IPluginFactory3,
                index: int32,
                info: *mut PClassInfoW,
            ) -> tresult
            where
                C: IPluginFactory3Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getClassInfoUnicode(
                    index,
                    info,
                )
            }
            unsafe extern "system" fn setHostContext<C, W, const OFFSET: isize>(
                this: *mut IPluginFactory3,
                context: *mut FUnknown,
            ) -> tresult
            where
                C: IPluginFactory3Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).setHostContext(
                    context,
                )
            }
            IPluginFactory3Vtbl {
                base: IPluginFactory2::make_vtbl::<C, W, OFFSET>(),
                getClassInfoUnicode: getClassInfoUnicode::<C, W, OFFSET>,
                setHostContext: setHostContext::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPluginFactory3
    where
        C: IPluginFactory3Trait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IPluginFactory3 {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ISizeableStream {
        pub vtbl: *const ISizeableStreamVtbl,
    }
    unsafe impl Send for ISizeableStream {}
    unsafe impl Sync for ISizeableStream {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for ISizeableStream {}
    impl ::com_scrape_types::Unknown for ISizeableStream {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for ISizeableStream {
        type Vtbl = ISizeableStreamVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(ISizeableStream_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ISizeableStreamVtbl {
        pub base: FUnknownVtbl,
        pub getStreamSize: unsafe extern "system" fn(
            this: *mut ISizeableStream,
            size: *mut int64,
        ) -> tresult,
        pub setStreamSize: unsafe extern "system" fn(
            this: *mut ISizeableStream,
            size: int64,
        ) -> tresult,
    }
    pub trait ISizeableStreamTrait {
        unsafe fn getStreamSize(
            &self,
            size: *mut int64,
        ) -> tresult;
        unsafe fn setStreamSize(
            &self,
            size: int64,
        ) -> tresult;
    }
    impl<P> ISizeableStreamTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<ISizeableStream>,
    {
        #[inline]
        unsafe fn getStreamSize(
            &self,
            size: *mut int64,
        ) -> tresult {
            let ptr = self.ptr() as *mut ISizeableStream;
            ((*(*ptr).vtbl).getStreamSize)(
                ptr,
                size,
            )
        }
        #[inline]
        unsafe fn setStreamSize(
            &self,
            size: int64,
        ) -> tresult {
            let ptr = self.ptr() as *mut ISizeableStream;
            ((*(*ptr).vtbl).setStreamSize)(
                ptr,
                size,
            )
        }
    }
    impl ISizeableStream {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> ISizeableStreamVtbl
        where
            C: ISizeableStreamTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn getStreamSize<C, W, const OFFSET: isize>(
                this: *mut ISizeableStream,
                size: *mut int64,
            ) -> tresult
            where
                C: ISizeableStreamTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getStreamSize(
                    size,
                )
            }
            unsafe extern "system" fn setStreamSize<C, W, const OFFSET: isize>(
                this: *mut ISizeableStream,
                size: int64,
            ) -> tresult
            where
                C: ISizeableStreamTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).setStreamSize(
                    size,
                )
            }
            ISizeableStreamVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                getStreamSize: getStreamSize::<C, W, OFFSET>,
                setStreamSize: setStreamSize::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for ISizeableStream
    where
        C: ISizeableStreamTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = ISizeableStream {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IString {
        pub vtbl: *const IStringVtbl,
    }
    unsafe impl Send for IString {}
    unsafe impl Sync for IString {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IString {}
    impl ::com_scrape_types::Unknown for IString {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IString {
        type Vtbl = IStringVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IString_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IStringVtbl {
        pub base: FUnknownVtbl,
        pub setText8: unsafe extern "system" fn(
            this: *mut IString,
            text: *const char8,
        ),
        pub setText16: unsafe extern "system" fn(
            this: *mut IString,
            text: *const char16,
        ),
        pub getText8: unsafe extern "system" fn(
            this: *mut IString,
        ) -> *const char8,
        pub getText16: unsafe extern "system" fn(
            this: *mut IString,
        ) -> *const char16,
        pub take: unsafe extern "system" fn(
            this: *mut IString,
            s: *mut ::std::ffi::c_void,
            isWide: bool,
        ),
        pub isWideString: unsafe extern "system" fn(
            this: *mut IString,
        ) -> bool,
    }
    pub trait IStringTrait {
        unsafe fn setText8(
            &self,
            text: *const char8,
        );
        unsafe fn setText16(
            &self,
            text: *const char16,
        );
        unsafe fn getText8(
            &self,
        ) -> *const char8;
        unsafe fn getText16(
            &self,
        ) -> *const char16;
        unsafe fn take(
            &self,
            s: *mut ::std::ffi::c_void,
            isWide: bool,
        );
        unsafe fn isWideString(
            &self,
        ) -> bool;
    }
    impl<P> IStringTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IString>,
    {
        #[inline]
        unsafe fn setText8(
            &self,
            text: *const char8,
        ) {
            let ptr = self.ptr() as *mut IString;
            ((*(*ptr).vtbl).setText8)(
                ptr,
                text,
            )
        }
        #[inline]
        unsafe fn setText16(
            &self,
            text: *const char16,
        ) {
            let ptr = self.ptr() as *mut IString;
            ((*(*ptr).vtbl).setText16)(
                ptr,
                text,
            )
        }
        #[inline]
        unsafe fn getText8(
            &self,
        ) -> *const char8 {
            let ptr = self.ptr() as *mut IString;
            ((*(*ptr).vtbl).getText8)(
                ptr,
            )
        }
        #[inline]
        unsafe fn getText16(
            &self,
        ) -> *const char16 {
            let ptr = self.ptr() as *mut IString;
            ((*(*ptr).vtbl).getText16)(
                ptr,
            )
        }
        #[inline]
        unsafe fn take(
            &self,
            s: *mut ::std::ffi::c_void,
            isWide: bool,
        ) {
            let ptr = self.ptr() as *mut IString;
            ((*(*ptr).vtbl).take)(
                ptr,
                s,
                isWide,
            )
        }
        #[inline]
        unsafe fn isWideString(
            &self,
        ) -> bool {
            let ptr = self.ptr() as *mut IString;
            ((*(*ptr).vtbl).isWideString)(
                ptr,
            )
        }
    }
    impl IString {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IStringVtbl
        where
            C: IStringTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn setText8<C, W, const OFFSET: isize>(
                this: *mut IString,
                text: *const char8,
            )
            where
                C: IStringTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).setText8(
                    text,
                )
            }
            unsafe extern "system" fn setText16<C, W, const OFFSET: isize>(
                this: *mut IString,
                text: *const char16,
            )
            where
                C: IStringTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).setText16(
                    text,
                )
            }
            unsafe extern "system" fn getText8<C, W, const OFFSET: isize>(
                this: *mut IString,
            ) -> *const char8
            where
                C: IStringTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getText8(
                )
            }
            unsafe extern "system" fn getText16<C, W, const OFFSET: isize>(
                this: *mut IString,
            ) -> *const char16
            where
                C: IStringTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getText16(
                )
            }
            unsafe extern "system" fn take<C, W, const OFFSET: isize>(
                this: *mut IString,
                s: *mut ::std::ffi::c_void,
                isWide: bool,
            )
            where
                C: IStringTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).take(
                    s,
                    isWide,
                )
            }
            unsafe extern "system" fn isWideString<C, W, const OFFSET: isize>(
                this: *mut IString,
            ) -> bool
            where
                C: IStringTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).isWideString(
                )
            }
            IStringVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                setText8: setText8::<C, W, OFFSET>,
                setText16: setText16::<C, W, OFFSET>,
                getText8: getText8::<C, W, OFFSET>,
                getText16: getText16::<C, W, OFFSET>,
                take: take::<C, W, OFFSET>,
                isWideString: isWideString::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IString
    where
        C: IStringTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IString {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IStringResult {
        pub vtbl: *const IStringResultVtbl,
    }
    unsafe impl Send for IStringResult {}
    unsafe impl Sync for IStringResult {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IStringResult {}
    impl ::com_scrape_types::Unknown for IStringResult {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IStringResult {
        type Vtbl = IStringResultVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IStringResult_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IStringResultVtbl {
        pub base: FUnknownVtbl,
        pub setText: unsafe extern "system" fn(
            this: *mut IStringResult,
            text: *const char8,
        ),
    }
    pub trait IStringResultTrait {
        unsafe fn setText(
            &self,
            text: *const char8,
        );
    }
    impl<P> IStringResultTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IStringResult>,
    {
        #[inline]
        unsafe fn setText(
            &self,
            text: *const char8,
        ) {
            let ptr = self.ptr() as *mut IStringResult;
            ((*(*ptr).vtbl).setText)(
                ptr,
                text,
            )
        }
    }
    impl IStringResult {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IStringResultVtbl
        where
            C: IStringResultTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn setText<C, W, const OFFSET: isize>(
                this: *mut IStringResult,
                text: *const char8,
            )
            where
                C: IStringResultTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).setText(
                    text,
                )
            }
            IStringResultVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                setText: setText::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IStringResult
    where
        C: IStringResultTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IStringResult {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IUpdateHandler {
        pub vtbl: *const IUpdateHandlerVtbl,
    }
    unsafe impl Send for IUpdateHandler {}
    unsafe impl Sync for IUpdateHandler {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IUpdateHandler {}
    impl ::com_scrape_types::Unknown for IUpdateHandler {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IUpdateHandler {
        type Vtbl = IUpdateHandlerVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IUpdateHandler_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IUpdateHandlerVtbl {
        pub base: FUnknownVtbl,
        pub addDependent: unsafe extern "system" fn(
            this: *mut IUpdateHandler,
            object: *mut FUnknown,
            dependent: *mut IDependent,
        ) -> tresult,
        pub removeDependent: unsafe extern "system" fn(
            this: *mut IUpdateHandler,
            object: *mut FUnknown,
            dependent: *mut IDependent,
        ) -> tresult,
        pub triggerUpdates: unsafe extern "system" fn(
            this: *mut IUpdateHandler,
            object: *mut FUnknown,
            message: int32,
        ) -> tresult,
        pub deferUpdates: unsafe extern "system" fn(
            this: *mut IUpdateHandler,
            object: *mut FUnknown,
            message: int32,
        ) -> tresult,
    }
    pub trait IUpdateHandlerTrait {
        unsafe fn addDependent(
            &self,
            object: *mut FUnknown,
            dependent: *mut IDependent,
        ) -> tresult;
        unsafe fn removeDependent(
            &self,
            object: *mut FUnknown,
            dependent: *mut IDependent,
        ) -> tresult;
        unsafe fn triggerUpdates(
            &self,
            object: *mut FUnknown,
            message: int32,
        ) -> tresult;
        unsafe fn deferUpdates(
            &self,
            object: *mut FUnknown,
            message: int32,
        ) -> tresult;
    }
    impl<P> IUpdateHandlerTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IUpdateHandler>,
    {
        #[inline]
        unsafe fn addDependent(
            &self,
            object: *mut FUnknown,
            dependent: *mut IDependent,
        ) -> tresult {
            let ptr = self.ptr() as *mut IUpdateHandler;
            ((*(*ptr).vtbl).addDependent)(
                ptr,
                object,
                dependent,
            )
        }
        #[inline]
        unsafe fn removeDependent(
            &self,
            object: *mut FUnknown,
            dependent: *mut IDependent,
        ) -> tresult {
            let ptr = self.ptr() as *mut IUpdateHandler;
            ((*(*ptr).vtbl).removeDependent)(
                ptr,
                object,
                dependent,
            )
        }
        #[inline]
        unsafe fn triggerUpdates(
            &self,
            object: *mut FUnknown,
            message: int32,
        ) -> tresult {
            let ptr = self.ptr() as *mut IUpdateHandler;
            ((*(*ptr).vtbl).triggerUpdates)(
                ptr,
                object,
                message,
            )
        }
        #[inline]
        unsafe fn deferUpdates(
            &self,
            object: *mut FUnknown,
            message: int32,
        ) -> tresult {
            let ptr = self.ptr() as *mut IUpdateHandler;
            ((*(*ptr).vtbl).deferUpdates)(
                ptr,
                object,
                message,
            )
        }
    }
    impl IUpdateHandler {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IUpdateHandlerVtbl
        where
            C: IUpdateHandlerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn addDependent<C, W, const OFFSET: isize>(
                this: *mut IUpdateHandler,
                object: *mut FUnknown,
                dependent: *mut IDependent,
            ) -> tresult
            where
                C: IUpdateHandlerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).addDependent(
                    object,
                    dependent,
                )
            }
            unsafe extern "system" fn removeDependent<C, W, const OFFSET: isize>(
                this: *mut IUpdateHandler,
                object: *mut FUnknown,
                dependent: *mut IDependent,
            ) -> tresult
            where
                C: IUpdateHandlerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).removeDependent(
                    object,
                    dependent,
                )
            }
            unsafe extern "system" fn triggerUpdates<C, W, const OFFSET: isize>(
                this: *mut IUpdateHandler,
                object: *mut FUnknown,
                message: int32,
            ) -> tresult
            where
                C: IUpdateHandlerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).triggerUpdates(
                    object,
                    message,
                )
            }
            unsafe extern "system" fn deferUpdates<C, W, const OFFSET: isize>(
                this: *mut IUpdateHandler,
                object: *mut FUnknown,
                message: int32,
            ) -> tresult
            where
                C: IUpdateHandlerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).deferUpdates(
                    object,
                    message,
                )
            }
            IUpdateHandlerVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                addDependent: addDependent::<C, W, OFFSET>,
                removeDependent: removeDependent::<C, W, OFFSET>,
                triggerUpdates: triggerUpdates::<C, W, OFFSET>,
                deferUpdates: deferUpdates::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IUpdateHandler
    where
        C: IUpdateHandlerTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IUpdateHandler {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IWaylandFrame {
        pub vtbl: *const IWaylandFrameVtbl,
    }
    unsafe impl Send for IWaylandFrame {}
    unsafe impl Sync for IWaylandFrame {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IWaylandFrame {}
    impl ::com_scrape_types::Unknown for IWaylandFrame {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IWaylandFrame {
        type Vtbl = IWaylandFrameVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IWaylandFrame_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IWaylandFrameVtbl {
        pub base: FUnknownVtbl,
        pub getWaylandSurface: unsafe extern "system" fn(
            this: *mut IWaylandFrame,
            display: *mut wl_display,
        ) -> *mut wl_surface,
        pub getParentSurface: unsafe extern "system" fn(
            this: *mut IWaylandFrame,
            parentSize: *mut ViewRect,
            display: *mut wl_display,
        ) -> *mut xdg_surface,
        pub getParentToplevel: unsafe extern "system" fn(
            this: *mut IWaylandFrame,
            display: *mut wl_display,
        ) -> *mut xdg_toplevel,
    }
    pub trait IWaylandFrameTrait {
        unsafe fn getWaylandSurface(
            &self,
            display: *mut wl_display,
        ) -> *mut wl_surface;
        unsafe fn getParentSurface(
            &self,
            parentSize: *mut ViewRect,
            display: *mut wl_display,
        ) -> *mut xdg_surface;
        unsafe fn getParentToplevel(
            &self,
            display: *mut wl_display,
        ) -> *mut xdg_toplevel;
    }
    impl<P> IWaylandFrameTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IWaylandFrame>,
    {
        #[inline]
        unsafe fn getWaylandSurface(
            &self,
            display: *mut wl_display,
        ) -> *mut wl_surface {
            let ptr = self.ptr() as *mut IWaylandFrame;
            ((*(*ptr).vtbl).getWaylandSurface)(
                ptr,
                display,
            )
        }
        #[inline]
        unsafe fn getParentSurface(
            &self,
            parentSize: *mut ViewRect,
            display: *mut wl_display,
        ) -> *mut xdg_surface {
            let ptr = self.ptr() as *mut IWaylandFrame;
            ((*(*ptr).vtbl).getParentSurface)(
                ptr,
                parentSize,
                display,
            )
        }
        #[inline]
        unsafe fn getParentToplevel(
            &self,
            display: *mut wl_display,
        ) -> *mut xdg_toplevel {
            let ptr = self.ptr() as *mut IWaylandFrame;
            ((*(*ptr).vtbl).getParentToplevel)(
                ptr,
                display,
            )
        }
    }
    impl IWaylandFrame {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IWaylandFrameVtbl
        where
            C: IWaylandFrameTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn getWaylandSurface<C, W, const OFFSET: isize>(
                this: *mut IWaylandFrame,
                display: *mut wl_display,
            ) -> *mut wl_surface
            where
                C: IWaylandFrameTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getWaylandSurface(
                    display,
                )
            }
            unsafe extern "system" fn getParentSurface<C, W, const OFFSET: isize>(
                this: *mut IWaylandFrame,
                parentSize: *mut ViewRect,
                display: *mut wl_display,
            ) -> *mut xdg_surface
            where
                C: IWaylandFrameTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getParentSurface(
                    parentSize,
                    display,
                )
            }
            unsafe extern "system" fn getParentToplevel<C, W, const OFFSET: isize>(
                this: *mut IWaylandFrame,
                display: *mut wl_display,
            ) -> *mut xdg_toplevel
            where
                C: IWaylandFrameTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).getParentToplevel(
                    display,
                )
            }
            IWaylandFrameVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                getWaylandSurface: getWaylandSurface::<C, W, OFFSET>,
                getParentSurface: getParentSurface::<C, W, OFFSET>,
                getParentToplevel: getParentToplevel::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IWaylandFrame
    where
        C: IWaylandFrameTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IWaylandFrame {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IWaylandHost {
        pub vtbl: *const IWaylandHostVtbl,
    }
    unsafe impl Send for IWaylandHost {}
    unsafe impl Sync for IWaylandHost {}
    unsafe impl ::com_scrape_types::Inherits<FUnknown> for IWaylandHost {}
    impl ::com_scrape_types::Unknown for IWaylandHost {
        #[inline]
        unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
            crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
        }
        #[inline]
        unsafe fn add_ref(this: *mut Self) -> usize {
            crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
        }
        #[inline]
        unsafe fn release(this: *mut Self) -> usize {
            crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
        }
    }
    unsafe impl ::com_scrape_types::Interface for IWaylandHost {
        type Vtbl = IWaylandHostVtbl;
        const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IWaylandHost_iid);
        #[inline]
        fn inherits(iid: &::com_scrape_types::Guid) -> bool {
            iid == &Self::IID || FUnknown::inherits(iid)
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IWaylandHostVtbl {
        pub base: FUnknownVtbl,
        pub openWaylandConnection: unsafe extern "system" fn(
            this: *mut IWaylandHost,
        ) -> *mut wl_display,
        pub closeWaylandConnection: unsafe extern "system" fn(
            this: *mut IWaylandHost,
            display: *mut wl_display,
        ) -> tresult,
    }
    pub trait IWaylandHostTrait {
        unsafe fn openWaylandConnection(
            &self,
        ) -> *mut wl_display;
        unsafe fn closeWaylandConnection(
            &self,
            display: *mut wl_display,
        ) -> tresult;
    }
    impl<P> IWaylandHostTrait for P
    where
        P: ::com_scrape_types::SmartPtr,
        P::Target: ::com_scrape_types::Inherits<IWaylandHost>,
    {
        #[inline]
        unsafe fn openWaylandConnection(
            &self,
        ) -> *mut wl_display {
            let ptr = self.ptr() as *mut IWaylandHost;
            ((*(*ptr).vtbl).openWaylandConnection)(
                ptr,
            )
        }
        #[inline]
        unsafe fn closeWaylandConnection(
            &self,
            display: *mut wl_display,
        ) -> tresult {
            let ptr = self.ptr() as *mut IWaylandHost;
            ((*(*ptr).vtbl).closeWaylandConnection)(
                ptr,
                display,
            )
        }
    }
    impl IWaylandHost {
        const fn make_vtbl<C, W, const OFFSET: isize>() -> IWaylandHostVtbl
        where
            C: IWaylandHostTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            unsafe extern "system" fn openWaylandConnection<C, W, const OFFSET: isize>(
                this: *mut IWaylandHost,
            ) -> *mut wl_display
            where
                C: IWaylandHostTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).openWaylandConnection(
                )
            }
            unsafe extern "system" fn closeWaylandConnection<C, W, const OFFSET: isize>(
                this: *mut IWaylandHost,
                display: *mut wl_display,
            ) -> tresult
            where
                C: IWaylandHostTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                (*ptr).closeWaylandConnection(
                    display,
                )
            }
            IWaylandHostVtbl {
                base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                openWaylandConnection: openWaylandConnection::<C, W, OFFSET>,
                closeWaylandConnection: closeWaylandConnection::<C, W, OFFSET>,
            }
        }
    }
    unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IWaylandHost
    where
        C: IWaylandHostTrait + ::com_scrape_types::Class,
        W: ::com_scrape_types::Wrapper<C>,
    {
        const OBJ: Self = IWaylandHost {
            vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
        };
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct KeyCode {
        pub character: tchar,
        pub virt: uint8,
        pub modifier: uint8,
    }
    unsafe impl Send for KeyCode {}
    unsafe impl Sync for KeyCode {}
    mod __PClassInfo_wrapper {
        #[allow(unused_imports)]
        use super::*;
        #[allow(unused_imports)]
        use super::PClassInfo_::*;
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct PClassInfo {
            pub cid: TUID,
            pub cardinality: int32,
            pub category: [char8; 32],
            pub name: [char8; 64],
        }
        unsafe impl Send for PClassInfo {}
        unsafe impl Sync for PClassInfo {}
    }
    pub use __PClassInfo_wrapper::*;
    pub mod PClassInfo_ {
        #[allow(unused_imports)]
        use super::*;
        pub type ClassCardinality = crate::support::DefaultEnumType;
        pub mod ClassCardinality_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kManyInstances: ClassCardinality = 2147483647;
        }
        pub const kCategorySize: crate::support::DefaultEnumType = 32;
        pub const kNameSize: crate::support::DefaultEnumType = 64;
    }
    mod __PClassInfo2_wrapper {
        #[allow(unused_imports)]
        use super::*;
        #[allow(unused_imports)]
        use super::PClassInfo2_::*;
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct PClassInfo2 {
            pub cid: TUID,
            pub cardinality: int32,
            pub category: [char8; 32],
            pub name: [char8; 64],
            pub classFlags: uint32,
            pub subCategories: [char8; 128],
            pub vendor: [char8; 64],
            pub version: [char8; 64],
            pub sdkVersion: [char8; 64],
        }
        unsafe impl Send for PClassInfo2 {}
        unsafe impl Sync for PClassInfo2 {}
    }
    pub use __PClassInfo2_wrapper::*;
    pub mod PClassInfo2_ {
        #[allow(unused_imports)]
        use super::*;
        pub const kSubCategoriesSize: crate::support::DefaultEnumType = 128;
        pub const kVendorSize: crate::support::DefaultEnumType = 64;
        pub const kVersionSize: crate::support::DefaultEnumType = 64;
    }
    mod __PClassInfoW_wrapper {
        #[allow(unused_imports)]
        use super::*;
        #[allow(unused_imports)]
        use super::PClassInfoW_::*;
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct PClassInfoW {
            pub cid: TUID,
            pub cardinality: int32,
            pub category: [char8; 32],
            pub name: [char16; 64],
            pub classFlags: uint32,
            pub subCategories: [char8; 128],
            pub vendor: [char16; 64],
            pub version: [char16; 64],
            pub sdkVersion: [char16; 64],
        }
        unsafe impl Send for PClassInfoW {}
        unsafe impl Sync for PClassInfoW {}
    }
    pub use __PClassInfoW_wrapper::*;
    pub mod PClassInfoW_ {
        #[allow(unused_imports)]
        use super::*;
        pub const kSubCategoriesSize: crate::support::DefaultEnumType = 128;
        pub const kVendorSize: crate::support::DefaultEnumType = 64;
        pub const kVersionSize: crate::support::DefaultEnumType = 64;
    }
    mod __PFactoryInfo_wrapper {
        #[allow(unused_imports)]
        use super::*;
        #[allow(unused_imports)]
        use super::PFactoryInfo_::*;
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct PFactoryInfo {
            pub vendor: [char8; 64],
            pub url: [char8; 256],
            pub email: [char8; 128],
            pub flags: int32,
        }
        unsafe impl Send for PFactoryInfo {}
        unsafe impl Sync for PFactoryInfo {}
    }
    pub use __PFactoryInfo_wrapper::*;
    pub mod PFactoryInfo_ {
        #[allow(unused_imports)]
        use super::*;
        pub type FactoryFlags = crate::support::DefaultEnumType;
        pub mod FactoryFlags_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kClassesDiscardable: FactoryFlags = 1;
            pub const kComponentNonDiscardable: FactoryFlags = 8;
            pub const kLicenseCheck: FactoryFlags = 2;
            pub const kNoFlags: FactoryFlags = 0;
            pub const kUnicode: FactoryFlags = 16;
        }
        pub const kEmailSize: crate::support::DefaultEnumType = 128;
        pub const kNameSize: crate::support::DefaultEnumType = 64;
        pub const kURLSize: crate::support::DefaultEnumType = 256;
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ViewRect {
        pub left: int32,
        pub top: int32,
        pub right: int32,
        pub bottom: int32,
    }
    unsafe impl Send for ViewRect {}
    unsafe impl Sync for ViewRect {}
    pub const FUnknown_iid: TUID = crate::support::uid(0x00000000, 0x00000000, 0xC0000000, 0x00000046);
    pub const IAttributes2_iid: TUID = crate::support::uid(0x1382126A, 0xFECA4871, 0x97D52A45, 0xB042AE99);
    pub const IAttributes_iid: TUID = crate::support::uid(0xFA1E32F9, 0xCA6D46F5, 0xA982F956, 0xB1191B58);
    pub const IBStream_iid: TUID = crate::support::uid(0xC3BF6EA2, 0x30994752, 0x9B6BF990, 0x1EE33E9B);
    pub const ICloneable_iid: TUID = crate::support::uid(0xD45406B9, 0x3A2D4443, 0x9DAD9BA9, 0x85A1454B);
    pub const IDependent_iid: TUID = crate::support::uid(0xF52B7AAE, 0xDE72416d, 0x8AF18ACE, 0x9DD7BD5E);
    pub const IErrorContext_iid: TUID = crate::support::uid(0x12BCD07B, 0x7C694336, 0xB7DA77C3, 0x444A0CD0);
    pub const IPersistent_iid: TUID = crate::support::uid(0xBA1A4637, 0x3C9F46D0, 0xA65DBA0E, 0xB85DA829);
    pub const IPlugFrame_iid: TUID = crate::support::uid(0x367FAF01, 0xAFA94693, 0x8D4DA2A0, 0xED0882A3);
    pub const IPlugViewContentScaleSupport_iid: TUID = crate::support::uid(0x65ED9690, 0x8AC44525, 0x8AADEF7A, 0x72EA703F);
    pub const IPlugView_iid: TUID = crate::support::uid(0x5BC32507, 0xD06049EA, 0xA6151B52, 0x2B755B29);
    pub const IPluginBase_iid: TUID = crate::support::uid(0x22888DDB, 0x156E45AE, 0x8358B348, 0x08190625);
    pub const IPluginCompatibility_iid: TUID = crate::support::uid(0x4AFD4B6A, 0x35D7C240, 0xA5C31414, 0xFB7D15E6);
    pub const IPluginFactory2_iid: TUID = crate::support::uid(0x0007B650, 0xF24B4C0B, 0xA464EDB9, 0xF00B2ABB);
    pub const IPluginFactory3_iid: TUID = crate::support::uid(0x4555A2AB, 0xC1234E57, 0x9B122910, 0x36878931);
    pub const IPluginFactory_iid: TUID = crate::support::uid(0x7A4D811C, 0x52114A1F, 0xAED9D2EE, 0x0B43BF9F);
    pub const ISizeableStream_iid: TUID = crate::support::uid(0x04F9549E, 0xE02F4E6E, 0x87E86A87, 0x47F4E17F);
    pub const IStringResult_iid: TUID = crate::support::uid(0x550798BC, 0x872049DB, 0x84920A15, 0x3B50B7A8);
    pub const IString_iid: TUID = crate::support::uid(0xF99DB7A3, 0x0FC14821, 0x800B0CF9, 0x8E348EDF);
    pub const IUpdateHandler_iid: TUID = crate::support::uid(0xF5246D56, 0x86544d60, 0xB026AFB5, 0x7B697B37);
    pub const IWaylandFrame_iid: TUID = crate::support::uid(0x809FAEC6, 0x231C4FFA, 0x98ED046C, 0x6E9E2003);
    pub const IWaylandHost_iid: TUID = crate::support::uid(0x5E9582EE, 0x86594652, 0xB213678E, 0x7F1A705E);
    pub const kInternalError: ::std::ffi::c_int = crate::support::kInternalError;
    pub const kInvalidArgument: ::std::ffi::c_int = crate::support::kInvalidArgument;
    pub const kMaxCoord: UCoord = 2147483647;
    pub const kMaxDouble: f64 = 1.7976931348623157e308;
    pub const kMaxFloat: f32 = 3.4028234663852886e38;
    pub const kMaxInt32: int32 = 2147483647;
    pub const kMaxInt32u: uint32 = 4294967295;
    pub const kMaxInt64: int64 = 9223372036854775807;
    pub const kMaxInt64u: uint64 = 18446744073709551615;
    pub const kMaxLong: int32 = 2147483647;
    pub const kMinCoord: UCoord = -2147483647;
    pub const kMinInt32: int32 = -2147483648;
    pub const kMinInt64: int64 = -9223372036854775808;
    pub const kMinLong: int32 = -2147483648;
    pub const kNoInterface: ::std::ffi::c_int = crate::support::kNoInterface;
    pub const kNotImplemented: ::std::ffi::c_int = crate::support::kNotImplemented;
    pub const kNotInitialized: ::std::ffi::c_int = crate::support::kNotInitialized;
    pub const kOutOfMemory: ::std::ffi::c_int = crate::support::kOutOfMemory;
    pub const kPlatformStringIOS: FIDString = b"IOS\0".as_ptr() as *const ::std::ffi::c_char;
    pub const kPlatformStringLinux: FIDString = b"Linux\0".as_ptr() as *const ::std::ffi::c_char;
    pub const kPlatformStringMac: FIDString = b"MAC\0".as_ptr() as *const ::std::ffi::c_char;
    pub const kPlatformStringWin: FIDString = b"WIN\0".as_ptr() as *const ::std::ffi::c_char;
    pub const kPlatformTypeHIView: FIDString = b"HIView\0".as_ptr() as *const ::std::ffi::c_char;
    pub const kPlatformTypeHWND: FIDString = b"HWND\0".as_ptr() as *const ::std::ffi::c_char;
    pub const kPlatformTypeNSView: FIDString = b"NSView\0".as_ptr() as *const ::std::ffi::c_char;
    pub const kPlatformTypeUIView: FIDString = b"UIView\0".as_ptr() as *const ::std::ffi::c_char;
    pub const kPlatformTypeWaylandSurfaceID: FIDString = b"WaylandSurfaceID\0".as_ptr() as *const ::std::ffi::c_char;
    pub const kPlatformTypeX11EmbedWindowID: FIDString = b"X11EmbedWindowID\0".as_ptr() as *const ::std::ffi::c_char;
    pub const kPrintfBufferSize: uint32 = 4096;
    pub const kResultFalse: ::std::ffi::c_int = crate::support::kResultFalse;
    pub const kResultOk: ::std::ffi::c_int = crate::support::kResultOk;
    pub const kResultTrue: ::std::ffi::c_int = crate::support::kResultTrue;
    pub mod Linux {
        #[allow(unused_imports)]
        use super::*;
        pub type FileDescriptor = ::std::ffi::c_int;
        pub type TimerInterval = uint64;
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IEventHandler {
            pub vtbl: *const IEventHandlerVtbl,
        }
        unsafe impl Send for IEventHandler {}
        unsafe impl Sync for IEventHandler {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IEventHandler {}
        impl ::com_scrape_types::Unknown for IEventHandler {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IEventHandler {
            type Vtbl = IEventHandlerVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IEventHandler_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IEventHandlerVtbl {
            pub base: FUnknownVtbl,
            pub onFDIsSet: unsafe extern "system" fn(
                this: *mut IEventHandler,
                fd: FileDescriptor,
            ),
        }
        pub trait IEventHandlerTrait {
            unsafe fn onFDIsSet(
                &self,
                fd: FileDescriptor,
            );
        }
        impl<P> IEventHandlerTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IEventHandler>,
        {
            #[inline]
            unsafe fn onFDIsSet(
                &self,
                fd: FileDescriptor,
            ) {
                let ptr = self.ptr() as *mut IEventHandler;
                ((*(*ptr).vtbl).onFDIsSet)(
                    ptr,
                    fd,
                )
            }
        }
        impl IEventHandler {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IEventHandlerVtbl
            where
                C: IEventHandlerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn onFDIsSet<C, W, const OFFSET: isize>(
                    this: *mut IEventHandler,
                    fd: FileDescriptor,
                )
                where
                    C: IEventHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).onFDIsSet(
                        fd,
                    )
                }
                IEventHandlerVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    onFDIsSet: onFDIsSet::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IEventHandler
        where
            C: IEventHandlerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IEventHandler {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IRunLoop {
            pub vtbl: *const IRunLoopVtbl,
        }
        unsafe impl Send for IRunLoop {}
        unsafe impl Sync for IRunLoop {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IRunLoop {}
        impl ::com_scrape_types::Unknown for IRunLoop {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IRunLoop {
            type Vtbl = IRunLoopVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IRunLoop_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IRunLoopVtbl {
            pub base: FUnknownVtbl,
            pub registerEventHandler: unsafe extern "system" fn(
                this: *mut IRunLoop,
                handler: *mut IEventHandler,
                fd: FileDescriptor,
            ) -> tresult,
            pub unregisterEventHandler: unsafe extern "system" fn(
                this: *mut IRunLoop,
                handler: *mut IEventHandler,
            ) -> tresult,
            pub registerTimer: unsafe extern "system" fn(
                this: *mut IRunLoop,
                handler: *mut ITimerHandler,
                milliseconds: TimerInterval,
            ) -> tresult,
            pub unregisterTimer: unsafe extern "system" fn(
                this: *mut IRunLoop,
                handler: *mut ITimerHandler,
            ) -> tresult,
        }
        pub trait IRunLoopTrait {
            unsafe fn registerEventHandler(
                &self,
                handler: *mut IEventHandler,
                fd: FileDescriptor,
            ) -> tresult;
            unsafe fn unregisterEventHandler(
                &self,
                handler: *mut IEventHandler,
            ) -> tresult;
            unsafe fn registerTimer(
                &self,
                handler: *mut ITimerHandler,
                milliseconds: TimerInterval,
            ) -> tresult;
            unsafe fn unregisterTimer(
                &self,
                handler: *mut ITimerHandler,
            ) -> tresult;
        }
        impl<P> IRunLoopTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IRunLoop>,
        {
            #[inline]
            unsafe fn registerEventHandler(
                &self,
                handler: *mut IEventHandler,
                fd: FileDescriptor,
            ) -> tresult {
                let ptr = self.ptr() as *mut IRunLoop;
                ((*(*ptr).vtbl).registerEventHandler)(
                    ptr,
                    handler,
                    fd,
                )
            }
            #[inline]
            unsafe fn unregisterEventHandler(
                &self,
                handler: *mut IEventHandler,
            ) -> tresult {
                let ptr = self.ptr() as *mut IRunLoop;
                ((*(*ptr).vtbl).unregisterEventHandler)(
                    ptr,
                    handler,
                )
            }
            #[inline]
            unsafe fn registerTimer(
                &self,
                handler: *mut ITimerHandler,
                milliseconds: TimerInterval,
            ) -> tresult {
                let ptr = self.ptr() as *mut IRunLoop;
                ((*(*ptr).vtbl).registerTimer)(
                    ptr,
                    handler,
                    milliseconds,
                )
            }
            #[inline]
            unsafe fn unregisterTimer(
                &self,
                handler: *mut ITimerHandler,
            ) -> tresult {
                let ptr = self.ptr() as *mut IRunLoop;
                ((*(*ptr).vtbl).unregisterTimer)(
                    ptr,
                    handler,
                )
            }
        }
        impl IRunLoop {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IRunLoopVtbl
            where
                C: IRunLoopTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn registerEventHandler<C, W, const OFFSET: isize>(
                    this: *mut IRunLoop,
                    handler: *mut IEventHandler,
                    fd: FileDescriptor,
                ) -> tresult
                where
                    C: IRunLoopTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).registerEventHandler(
                        handler,
                        fd,
                    )
                }
                unsafe extern "system" fn unregisterEventHandler<C, W, const OFFSET: isize>(
                    this: *mut IRunLoop,
                    handler: *mut IEventHandler,
                ) -> tresult
                where
                    C: IRunLoopTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).unregisterEventHandler(
                        handler,
                    )
                }
                unsafe extern "system" fn registerTimer<C, W, const OFFSET: isize>(
                    this: *mut IRunLoop,
                    handler: *mut ITimerHandler,
                    milliseconds: TimerInterval,
                ) -> tresult
                where
                    C: IRunLoopTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).registerTimer(
                        handler,
                        milliseconds,
                    )
                }
                unsafe extern "system" fn unregisterTimer<C, W, const OFFSET: isize>(
                    this: *mut IRunLoop,
                    handler: *mut ITimerHandler,
                ) -> tresult
                where
                    C: IRunLoopTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).unregisterTimer(
                        handler,
                    )
                }
                IRunLoopVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    registerEventHandler: registerEventHandler::<C, W, OFFSET>,
                    unregisterEventHandler: unregisterEventHandler::<C, W, OFFSET>,
                    registerTimer: registerTimer::<C, W, OFFSET>,
                    unregisterTimer: unregisterTimer::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IRunLoop
        where
            C: IRunLoopTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IRunLoop {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct ITimerHandler {
            pub vtbl: *const ITimerHandlerVtbl,
        }
        unsafe impl Send for ITimerHandler {}
        unsafe impl Sync for ITimerHandler {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for ITimerHandler {}
        impl ::com_scrape_types::Unknown for ITimerHandler {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for ITimerHandler {
            type Vtbl = ITimerHandlerVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(ITimerHandler_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct ITimerHandlerVtbl {
            pub base: FUnknownVtbl,
            pub onTimer: unsafe extern "system" fn(
                this: *mut ITimerHandler,
            ),
        }
        pub trait ITimerHandlerTrait {
            unsafe fn onTimer(
                &self,
            );
        }
        impl<P> ITimerHandlerTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<ITimerHandler>,
        {
            #[inline]
            unsafe fn onTimer(
                &self,
            ) {
                let ptr = self.ptr() as *mut ITimerHandler;
                ((*(*ptr).vtbl).onTimer)(
                    ptr,
                )
            }
        }
        impl ITimerHandler {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> ITimerHandlerVtbl
            where
                C: ITimerHandlerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn onTimer<C, W, const OFFSET: isize>(
                    this: *mut ITimerHandler,
                )
                where
                    C: ITimerHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).onTimer(
                    )
                }
                ITimerHandlerVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    onTimer: onTimer::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for ITimerHandler
        where
            C: ITimerHandlerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = ITimerHandler {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        pub const IEventHandler_iid: TUID = crate::support::uid(0x561E65C9, 0x13A0496F, 0x813A2C35, 0x654D7983);
        pub const IRunLoop_iid: TUID = crate::support::uid(0x18C35366, 0x97764F1A, 0x9C5B8385, 0x7A871389);
        pub const ITimerHandler_iid: TUID = crate::support::uid(0x10BDD94F, 0x41424774, 0x821FAD8F, 0xECA72CA9);
    }
    pub mod Vst {
        #[allow(unused_imports)]
        use super::*;
        pub type BusDirection = int32;
        pub type BusDirections = crate::support::DefaultEnumType;
        pub mod BusDirections_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kInput: BusDirections = 0;
            pub const kOutput: BusDirections = 1;
        }
        pub type BusIndex = int32;
        pub type BusType = int32;
        pub type BusTypes = crate::support::DefaultEnumType;
        pub mod BusTypes_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kAux: BusTypes = 1;
            pub const kMain: BusTypes = 0;
        }
        pub type CString = *const char8;
        pub type ColorSpec = uint32;
        pub type ComponentFlags = crate::support::DefaultEnumType;
        pub mod ComponentFlags_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kDistributable: ComponentFlags = 1;
            pub const kSimpleModeSupported: ComponentFlags = 2;
        }
        pub type ControllerNumbers = crate::support::DefaultEnumType;
        pub mod ControllerNumbers_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kAfterTouch: ControllerNumbers = 128;
            pub const kCountCtrlNumber: ControllerNumbers = 130;
            pub const kCtrlAllNotesOff: ControllerNumbers = 123;
            pub const kCtrlAllSoundsOff: ControllerNumbers = 120;
            pub const kCtrlAttackTime: ControllerNumbers = 73;
            pub const kCtrlBalance: ControllerNumbers = 8;
            pub const kCtrlBankSelectLSB: ControllerNumbers = 32;
            pub const kCtrlBankSelectMSB: ControllerNumbers = 0;
            pub const kCtrlBreath: ControllerNumbers = 2;
            pub const kCtrlDataDecrement: ControllerNumbers = 97;
            pub const kCtrlDataEntryLSB: ControllerNumbers = 38;
            pub const kCtrlDataEntryMSB: ControllerNumbers = 6;
            pub const kCtrlDataIncrement: ControllerNumbers = 96;
            pub const kCtrlDecayTime: ControllerNumbers = 75;
            pub const kCtrlEff1Depth: ControllerNumbers = 91;
            pub const kCtrlEff2Depth: ControllerNumbers = 92;
            pub const kCtrlEff3Depth: ControllerNumbers = 93;
            pub const kCtrlEff4Depth: ControllerNumbers = 94;
            pub const kCtrlEff5Depth: ControllerNumbers = 95;
            pub const kCtrlEffect1: ControllerNumbers = 12;
            pub const kCtrlEffect2: ControllerNumbers = 13;
            pub const kCtrlExpression: ControllerNumbers = 11;
            pub const kCtrlFilterCutoff: ControllerNumbers = 71;
            pub const kCtrlFilterResonance: ControllerNumbers = 74;
            pub const kCtrlFoot: ControllerNumbers = 4;
            pub const kCtrlGPC1: ControllerNumbers = 16;
            pub const kCtrlGPC2: ControllerNumbers = 17;
            pub const kCtrlGPC3: ControllerNumbers = 18;
            pub const kCtrlGPC4: ControllerNumbers = 19;
            pub const kCtrlGPC5: ControllerNumbers = 80;
            pub const kCtrlGPC6: ControllerNumbers = 81;
            pub const kCtrlGPC7: ControllerNumbers = 82;
            pub const kCtrlGPC8: ControllerNumbers = 83;
            pub const kCtrlHold2OnOff: ControllerNumbers = 69;
            pub const kCtrlLegatoFootSwOnOff: ControllerNumbers = 68;
            pub const kCtrlLocalCtrlOnOff: ControllerNumbers = 122;
            pub const kCtrlModWheel: ControllerNumbers = 1;
            pub const kCtrlNRPNSelectLSB: ControllerNumbers = 98;
            pub const kCtrlNRPNSelectMSB: ControllerNumbers = 99;
            pub const kCtrlOmniModeOff: ControllerNumbers = 124;
            pub const kCtrlOmniModeOn: ControllerNumbers = 125;
            pub const kCtrlPan: ControllerNumbers = 10;
            pub const kCtrlPolyModeOn: ControllerNumbers = 127;
            pub const kCtrlPolyModeOnOff: ControllerNumbers = 126;
            pub const kCtrlPolyPressure: ControllerNumbers = 131;
            pub const kCtrlPortaControl: ControllerNumbers = 84;
            pub const kCtrlPortaOnOff: ControllerNumbers = 65;
            pub const kCtrlPortaTime: ControllerNumbers = 5;
            pub const kCtrlProgramChange: ControllerNumbers = 130;
            pub const kCtrlQuarterFrame: ControllerNumbers = 132;
            pub const kCtrlRPNSelectLSB: ControllerNumbers = 100;
            pub const kCtrlRPNSelectMSB: ControllerNumbers = 101;
            pub const kCtrlReleaseTime: ControllerNumbers = 72;
            pub const kCtrlResetAllCtrlers: ControllerNumbers = 121;
            pub const kCtrlSoftPedalOnOff: ControllerNumbers = 67;
            pub const kCtrlSoundCtrler10: ControllerNumbers = 79;
            pub const kCtrlSoundVariation: ControllerNumbers = 70;
            pub const kCtrlSustainOnOff: ControllerNumbers = 64;
            pub const kCtrlSustenutoOnOff: ControllerNumbers = 66;
            pub const kCtrlVibratoDelay: ControllerNumbers = 78;
            pub const kCtrlVibratoDepth: ControllerNumbers = 77;
            pub const kCtrlVibratoRate: ControllerNumbers = 76;
            pub const kCtrlVolume: ControllerNumbers = 7;
            pub const kPitchBend: ControllerNumbers = 129;
            pub const kSystemActiveSensing: ControllerNumbers = 140;
            pub const kSystemCableSelect: ControllerNumbers = 135;
            pub const kSystemMidiClockContinue: ControllerNumbers = 138;
            pub const kSystemMidiClockStart: ControllerNumbers = 137;
            pub const kSystemMidiClockStop: ControllerNumbers = 139;
            pub const kSystemSongPointer: ControllerNumbers = 134;
            pub const kSystemSongSelect: ControllerNumbers = 133;
            pub const kSystemTuneRequest: ControllerNumbers = 136;
        }
        pub type CtrlNumber = int16;
        pub type DataExchangeBlockID = uint32;
        pub type DataExchangeQueueID = uint32;
        pub type DataExchangeUserContextID = uint32;
        pub type IoMode = int32;
        pub type IoModes = crate::support::DefaultEnumType;
        pub mod IoModes_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kAdvanced: IoModes = 1;
            pub const kOfflineProcessing: IoModes = 2;
            pub const kSimple: IoModes = 0;
        }
        pub type KeyswitchTypeID = uint32;
        pub type KeyswitchTypeIDs = uint32;
        pub mod KeyswitchTypeIDs_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kKeyRangeTypeID: KeyswitchTypeIDs = 3;
            pub const kNoteOnKeyswitchTypeID: KeyswitchTypeIDs = 0;
            pub const kOnReleaseKeyswitchTypeID: KeyswitchTypeIDs = 2;
            pub const kOnTheFlyKeyswitchTypeID: KeyswitchTypeIDs = 1;
        }
        pub type KnobMode = int32;
        pub type KnobModes = KnobMode;
        pub mod KnobModes_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kCircularMode: KnobModes = 0;
            pub const kLinearMode: KnobModes = 2;
            pub const kRelativCircularMode: KnobModes = 1;
        }
        pub type MediaType = int32;
        pub type MediaTypes = crate::support::DefaultEnumType;
        pub mod MediaTypes_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kAudio: MediaTypes = 0;
            pub const kEvent: MediaTypes = 1;
            pub const kNumMediaTypes: MediaTypes = 2;
        }
        pub type MidiChannel = uint8;
        pub type MidiGroup = uint8;
        pub type NoteExpressionTypeID = uint32;
        pub type NoteExpressionTypeIDs = uint32;
        pub mod NoteExpressionTypeIDs_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kBrightnessTypeID: NoteExpressionTypeIDs = 5;
            pub const kCustomEnd: NoteExpressionTypeIDs = 200000;
            pub const kCustomStart: NoteExpressionTypeIDs = 100000;
            pub const kExpressionTypeID: NoteExpressionTypeIDs = 4;
            pub const kInvalidTypeID: NoteExpressionTypeIDs = 4294967295;
            pub const kPanTypeID: NoteExpressionTypeIDs = 1;
            pub const kPhonemeTypeID: NoteExpressionTypeIDs = 7;
            pub const kTextTypeID: NoteExpressionTypeIDs = 6;
            pub const kTuningTypeID: NoteExpressionTypeIDs = 2;
            pub const kVibratoTypeID: NoteExpressionTypeIDs = 3;
            pub const kVolumeTypeID: NoteExpressionTypeIDs = 0;
        }
        pub type NoteExpressionValue = f64;
        pub type NoteIDUserRange = ::std::ffi::c_int;
        pub mod NoteIDUserRange_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kNoteIDUserRangeLowerBound: NoteIDUserRange = -10000;
            pub const kNoteIDUserRangeUpperBound: NoteIDUserRange = -1000;
        }
        pub type ParamID = uint32;
        pub type ParamValue = f64;
        pub type PhysicalUITypeID = uint32;
        pub type PhysicalUITypeIDs = crate::support::DefaultEnumType;
        pub mod PhysicalUITypeIDs_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kInvalidPUITypeID: PhysicalUITypeIDs = 0xFFFFFFFFu32 as crate::support::DefaultEnumType;
            pub const kPUIPressure: PhysicalUITypeIDs = 2;
            pub const kPUITypeCount: PhysicalUITypeIDs = 3;
            pub const kPUIXMovement: PhysicalUITypeIDs = 0;
            pub const kPUIYMovement: PhysicalUITypeIDs = 1;
        }
        pub type PrefetchableSupport = uint32;
        pub type ProcessModes = crate::support::DefaultEnumType;
        pub mod ProcessModes_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kOffline: ProcessModes = 2;
            pub const kPrefetch: ProcessModes = 1;
            pub const kRealtime: ProcessModes = 0;
        }
        pub type ProgramListID = int32;
        pub type RestartFlags = int32;
        pub mod RestartFlags_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kIoChanged: RestartFlags = 2;
            pub const kIoTitlesChanged: RestartFlags = 128;
            pub const kKeyswitchChanged: RestartFlags = 1024;
            pub const kLatencyChanged: RestartFlags = 8;
            pub const kMidiCCAssignmentChanged: RestartFlags = 32;
            pub const kNoteExpressionChanged: RestartFlags = 64;
            pub const kParamIDMappingChanged: RestartFlags = 2048;
            pub const kParamTitlesChanged: RestartFlags = 16;
            pub const kParamValuesChanged: RestartFlags = 4;
            pub const kPrefetchableSupportChanged: RestartFlags = 256;
            pub const kReloadComponent: RestartFlags = 1;
            pub const kRoutingInfoChanged: RestartFlags = 512;
        }
        pub type Sample32 = f32;
        pub type Sample64 = f64;
        pub type SampleRate = f64;
        pub type Speaker = uint64;
        pub type SpeakerArrangement = uint64;
        pub type String128 = [TChar; 128];
        pub type SymbolicSampleSizes = crate::support::DefaultEnumType;
        pub mod SymbolicSampleSizes_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kSample32: SymbolicSampleSizes = 0;
            pub const kSample64: SymbolicSampleSizes = 1;
        }
        pub type TChar = char16;
        pub type TQuarterNotes = f64;
        pub type TSamples = int64;
        pub type UnitID = int32;
        pub type ePrefetchableSupport = crate::support::DefaultEnumType;
        pub mod ePrefetchableSupport_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kIsNeverPrefetchable: ePrefetchableSupport = 0;
            pub const kIsNotYetPrefetchable: ePrefetchableSupport = 2;
            pub const kIsYetPrefetchable: ePrefetchableSupport = 1;
            pub const kNumPrefetchableSupport: ePrefetchableSupport = 3;
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct AudioBusBuffers {
            pub numChannels: int32,
            pub silenceFlags: uint64,
            pub __field0: AudioBusBuffers__type0,
        }
        unsafe impl Send for AudioBusBuffers {}
        unsafe impl Sync for AudioBusBuffers {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub union AudioBusBuffers__type0 {
            pub channelBuffers32: *mut *mut Sample32,
            pub channelBuffers64: *mut *mut Sample64,
        }
        unsafe impl Send for AudioBusBuffers__type0 {}
        unsafe impl Sync for AudioBusBuffers__type0 {}
        mod __BusInfo_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::BusInfo_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct BusInfo {
                pub mediaType: MediaType,
                pub direction: BusDirection,
                pub channelCount: int32,
                pub name: String128,
                pub busType: BusType,
                pub flags: uint32,
            }
            unsafe impl Send for BusInfo {}
            unsafe impl Sync for BusInfo {}
        }
        pub use __BusInfo_wrapper::*;
        pub mod BusInfo_ {
            #[allow(unused_imports)]
            use super::*;
            pub type BusFlags = crate::support::DefaultEnumType;
            pub mod BusFlags_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kDefaultActive: BusFlags = 1;
                pub const kIsControlVoltage: BusFlags = 2;
            }
        }
        mod __Chord_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::Chord_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct Chord {
                pub keyNote: uint8,
                pub rootNote: uint8,
                pub chordMask: int16,
            }
            unsafe impl Send for Chord {}
            unsafe impl Sync for Chord {}
        }
        pub use __Chord_wrapper::*;
        pub mod Chord_ {
            #[allow(unused_imports)]
            use super::*;
            pub type Masks = crate::support::DefaultEnumType;
            pub mod Masks_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kChordMask: Masks = 4095;
                pub const kReservedMask: Masks = 61440;
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct ChordEvent {
            pub root: int16,
            pub bassNote: int16,
            pub mask: int16,
            pub textLen: uint16,
            pub text: *const TChar,
        }
        unsafe impl Send for ChordEvent {}
        unsafe impl Sync for ChordEvent {}
        mod __DataEvent_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::DataEvent_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct DataEvent {
                pub size: uint32,
                pub r#type: uint32,
                pub bytes: *const uint8,
            }
            unsafe impl Send for DataEvent {}
            unsafe impl Sync for DataEvent {}
        }
        pub use __DataEvent_wrapper::*;
        pub mod DataEvent_ {
            #[allow(unused_imports)]
            use super::*;
            pub type DataTypes = crate::support::DefaultEnumType;
            pub mod DataTypes_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kMidiSysEx: DataTypes = 0;
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct DataExchangeBlock {
            pub data: *mut ::std::ffi::c_void,
            pub size: uint32,
            pub blockID: DataExchangeBlockID,
        }
        unsafe impl Send for DataExchangeBlock {}
        unsafe impl Sync for DataExchangeBlock {}
        mod __Event_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::Event_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct Event {
                pub busIndex: int32,
                pub sampleOffset: int32,
                pub ppqPosition: TQuarterNotes,
                pub flags: uint16,
                pub r#type: uint16,
                pub __field0: Event__type0,
            }
            unsafe impl Send for Event {}
            unsafe impl Sync for Event {}
        }
        pub use __Event_wrapper::*;
        pub mod Event_ {
            #[allow(unused_imports)]
            use super::*;
            pub type EventFlags = crate::support::DefaultEnumType;
            pub mod EventFlags_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kIsLive: EventFlags = 1;
                pub const kUserReserved1: EventFlags = 16384;
                pub const kUserReserved2: EventFlags = 32768;
            }
            pub type EventTypes = crate::support::DefaultEnumType;
            pub mod EventTypes_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kChordEvent: EventTypes = 6;
                pub const kDataEvent: EventTypes = 2;
                pub const kLegacyMIDICCOutEvent: EventTypes = 65535;
                pub const kNoteExpressionIntValueEvent: EventTypes = 8;
                pub const kNoteExpressionTextEvent: EventTypes = 5;
                pub const kNoteExpressionValueEvent: EventTypes = 4;
                pub const kNoteOffEvent: EventTypes = 1;
                pub const kNoteOnEvent: EventTypes = 0;
                pub const kPolyPressureEvent: EventTypes = 3;
                pub const kScaleEvent: EventTypes = 7;
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub union Event__type0 {
            pub noteOn: NoteOnEvent,
            pub noteOff: NoteOffEvent,
            pub data: DataEvent,
            pub polyPressure: PolyPressureEvent,
            pub noteExpressionValue: NoteExpressionValueEvent,
            pub noteExpressionText: NoteExpressionTextEvent,
            pub noteExpressionIntValue: NoteExpressionIntValueEvent,
            pub chord: ChordEvent,
            pub scale: ScaleEvent,
            pub midiCCOut: LegacyMIDICCOutEvent,
        }
        unsafe impl Send for Event__type0 {}
        unsafe impl Sync for Event__type0 {}
        mod __FrameRate_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::FrameRate_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct FrameRate {
                pub framesPerSecond: uint32,
                pub flags: uint32,
            }
            unsafe impl Send for FrameRate {}
            unsafe impl Sync for FrameRate {}
        }
        pub use __FrameRate_wrapper::*;
        pub mod FrameRate_ {
            #[allow(unused_imports)]
            use super::*;
            pub type FrameRateFlags = crate::support::DefaultEnumType;
            pub mod FrameRateFlags_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kDropRate: FrameRateFlags = 2;
                pub const kPullDownRate: FrameRateFlags = 1;
            }
        }
        mod __IAttributeList_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::IAttributeList_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IAttributeList {
                pub vtbl: *const IAttributeListVtbl,
            }
            unsafe impl Send for IAttributeList {}
            unsafe impl Sync for IAttributeList {}
            unsafe impl ::com_scrape_types::Inherits<FUnknown> for IAttributeList {}
            impl ::com_scrape_types::Unknown for IAttributeList {
                #[inline]
                unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                    crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
                }
                #[inline]
                unsafe fn add_ref(this: *mut Self) -> usize {
                    crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
                }
                #[inline]
                unsafe fn release(this: *mut Self) -> usize {
                    crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
                }
            }
            unsafe impl ::com_scrape_types::Interface for IAttributeList {
                type Vtbl = IAttributeListVtbl;
                const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IAttributeList_iid);
                #[inline]
                fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                    iid == &Self::IID || FUnknown::inherits(iid)
                }
            }
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IAttributeListVtbl {
                pub base: FUnknownVtbl,
                pub setInt: unsafe extern "system" fn(
                    this: *mut IAttributeList,
                    id: AttrID,
                    value: int64,
                ) -> tresult,
                pub getInt: unsafe extern "system" fn(
                    this: *mut IAttributeList,
                    id: AttrID,
                    value: *mut int64,
                ) -> tresult,
                pub setFloat: unsafe extern "system" fn(
                    this: *mut IAttributeList,
                    id: AttrID,
                    value: f64,
                ) -> tresult,
                pub getFloat: unsafe extern "system" fn(
                    this: *mut IAttributeList,
                    id: AttrID,
                    value: *mut f64,
                ) -> tresult,
                pub setString: unsafe extern "system" fn(
                    this: *mut IAttributeList,
                    id: AttrID,
                    string: *const TChar,
                ) -> tresult,
                pub getString: unsafe extern "system" fn(
                    this: *mut IAttributeList,
                    id: AttrID,
                    string: *mut TChar,
                    sizeInBytes: uint32,
                ) -> tresult,
                pub setBinary: unsafe extern "system" fn(
                    this: *mut IAttributeList,
                    id: AttrID,
                    data: *const ::std::ffi::c_void,
                    sizeInBytes: uint32,
                ) -> tresult,
                pub getBinary: unsafe extern "system" fn(
                    this: *mut IAttributeList,
                    id: AttrID,
                    data: *mut *const ::std::ffi::c_void,
                    sizeInBytes: *mut uint32,
                ) -> tresult,
            }
            pub trait IAttributeListTrait {
                unsafe fn setInt(
                    &self,
                    id: AttrID,
                    value: int64,
                ) -> tresult;
                unsafe fn getInt(
                    &self,
                    id: AttrID,
                    value: *mut int64,
                ) -> tresult;
                unsafe fn setFloat(
                    &self,
                    id: AttrID,
                    value: f64,
                ) -> tresult;
                unsafe fn getFloat(
                    &self,
                    id: AttrID,
                    value: *mut f64,
                ) -> tresult;
                unsafe fn setString(
                    &self,
                    id: AttrID,
                    string: *const TChar,
                ) -> tresult;
                unsafe fn getString(
                    &self,
                    id: AttrID,
                    string: *mut TChar,
                    sizeInBytes: uint32,
                ) -> tresult;
                unsafe fn setBinary(
                    &self,
                    id: AttrID,
                    data: *const ::std::ffi::c_void,
                    sizeInBytes: uint32,
                ) -> tresult;
                unsafe fn getBinary(
                    &self,
                    id: AttrID,
                    data: *mut *const ::std::ffi::c_void,
                    sizeInBytes: *mut uint32,
                ) -> tresult;
            }
            impl<P> IAttributeListTrait for P
            where
                P: ::com_scrape_types::SmartPtr,
                P::Target: ::com_scrape_types::Inherits<IAttributeList>,
            {
                #[inline]
                unsafe fn setInt(
                    &self,
                    id: AttrID,
                    value: int64,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IAttributeList;
                    ((*(*ptr).vtbl).setInt)(
                        ptr,
                        id,
                        value,
                    )
                }
                #[inline]
                unsafe fn getInt(
                    &self,
                    id: AttrID,
                    value: *mut int64,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IAttributeList;
                    ((*(*ptr).vtbl).getInt)(
                        ptr,
                        id,
                        value,
                    )
                }
                #[inline]
                unsafe fn setFloat(
                    &self,
                    id: AttrID,
                    value: f64,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IAttributeList;
                    ((*(*ptr).vtbl).setFloat)(
                        ptr,
                        id,
                        value,
                    )
                }
                #[inline]
                unsafe fn getFloat(
                    &self,
                    id: AttrID,
                    value: *mut f64,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IAttributeList;
                    ((*(*ptr).vtbl).getFloat)(
                        ptr,
                        id,
                        value,
                    )
                }
                #[inline]
                unsafe fn setString(
                    &self,
                    id: AttrID,
                    string: *const TChar,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IAttributeList;
                    ((*(*ptr).vtbl).setString)(
                        ptr,
                        id,
                        string,
                    )
                }
                #[inline]
                unsafe fn getString(
                    &self,
                    id: AttrID,
                    string: *mut TChar,
                    sizeInBytes: uint32,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IAttributeList;
                    ((*(*ptr).vtbl).getString)(
                        ptr,
                        id,
                        string,
                        sizeInBytes,
                    )
                }
                #[inline]
                unsafe fn setBinary(
                    &self,
                    id: AttrID,
                    data: *const ::std::ffi::c_void,
                    sizeInBytes: uint32,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IAttributeList;
                    ((*(*ptr).vtbl).setBinary)(
                        ptr,
                        id,
                        data,
                        sizeInBytes,
                    )
                }
                #[inline]
                unsafe fn getBinary(
                    &self,
                    id: AttrID,
                    data: *mut *const ::std::ffi::c_void,
                    sizeInBytes: *mut uint32,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IAttributeList;
                    ((*(*ptr).vtbl).getBinary)(
                        ptr,
                        id,
                        data,
                        sizeInBytes,
                    )
                }
            }
            impl IAttributeList {
                const fn make_vtbl<C, W, const OFFSET: isize>() -> IAttributeListVtbl
                where
                    C: IAttributeListTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    unsafe extern "system" fn setInt<C, W, const OFFSET: isize>(
                        this: *mut IAttributeList,
                        id: AttrID,
                        value: int64,
                    ) -> tresult
                    where
                        C: IAttributeListTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).setInt(
                            id,
                            value,
                        )
                    }
                    unsafe extern "system" fn getInt<C, W, const OFFSET: isize>(
                        this: *mut IAttributeList,
                        id: AttrID,
                        value: *mut int64,
                    ) -> tresult
                    where
                        C: IAttributeListTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).getInt(
                            id,
                            value,
                        )
                    }
                    unsafe extern "system" fn setFloat<C, W, const OFFSET: isize>(
                        this: *mut IAttributeList,
                        id: AttrID,
                        value: f64,
                    ) -> tresult
                    where
                        C: IAttributeListTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).setFloat(
                            id,
                            value,
                        )
                    }
                    unsafe extern "system" fn getFloat<C, W, const OFFSET: isize>(
                        this: *mut IAttributeList,
                        id: AttrID,
                        value: *mut f64,
                    ) -> tresult
                    where
                        C: IAttributeListTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).getFloat(
                            id,
                            value,
                        )
                    }
                    unsafe extern "system" fn setString<C, W, const OFFSET: isize>(
                        this: *mut IAttributeList,
                        id: AttrID,
                        string: *const TChar,
                    ) -> tresult
                    where
                        C: IAttributeListTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).setString(
                            id,
                            string,
                        )
                    }
                    unsafe extern "system" fn getString<C, W, const OFFSET: isize>(
                        this: *mut IAttributeList,
                        id: AttrID,
                        string: *mut TChar,
                        sizeInBytes: uint32,
                    ) -> tresult
                    where
                        C: IAttributeListTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).getString(
                            id,
                            string,
                            sizeInBytes,
                        )
                    }
                    unsafe extern "system" fn setBinary<C, W, const OFFSET: isize>(
                        this: *mut IAttributeList,
                        id: AttrID,
                        data: *const ::std::ffi::c_void,
                        sizeInBytes: uint32,
                    ) -> tresult
                    where
                        C: IAttributeListTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).setBinary(
                            id,
                            data,
                            sizeInBytes,
                        )
                    }
                    unsafe extern "system" fn getBinary<C, W, const OFFSET: isize>(
                        this: *mut IAttributeList,
                        id: AttrID,
                        data: *mut *const ::std::ffi::c_void,
                        sizeInBytes: *mut uint32,
                    ) -> tresult
                    where
                        C: IAttributeListTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).getBinary(
                            id,
                            data,
                            sizeInBytes,
                        )
                    }
                    IAttributeListVtbl {
                        base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                        setInt: setInt::<C, W, OFFSET>,
                        getInt: getInt::<C, W, OFFSET>,
                        setFloat: setFloat::<C, W, OFFSET>,
                        getFloat: getFloat::<C, W, OFFSET>,
                        setString: setString::<C, W, OFFSET>,
                        getString: getString::<C, W, OFFSET>,
                        setBinary: setBinary::<C, W, OFFSET>,
                        getBinary: getBinary::<C, W, OFFSET>,
                    }
                }
            }
            unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IAttributeList
            where
                C: IAttributeListTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                const OBJ: Self = IAttributeList {
                    vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
                };
            }
        }
        pub use __IAttributeList_wrapper::*;
        pub mod IAttributeList_ {
            #[allow(unused_imports)]
            use super::*;
            pub type AttrID = *const ::std::ffi::c_char;
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IAudioPresentationLatency {
            pub vtbl: *const IAudioPresentationLatencyVtbl,
        }
        unsafe impl Send for IAudioPresentationLatency {}
        unsafe impl Sync for IAudioPresentationLatency {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IAudioPresentationLatency {}
        impl ::com_scrape_types::Unknown for IAudioPresentationLatency {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IAudioPresentationLatency {
            type Vtbl = IAudioPresentationLatencyVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IAudioPresentationLatency_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IAudioPresentationLatencyVtbl {
            pub base: FUnknownVtbl,
            pub setAudioPresentationLatencySamples: unsafe extern "system" fn(
                this: *mut IAudioPresentationLatency,
                dir: BusDirection,
                busIndex: int32,
                latencyInSamples: uint32,
            ) -> tresult,
        }
        pub trait IAudioPresentationLatencyTrait {
            unsafe fn setAudioPresentationLatencySamples(
                &self,
                dir: BusDirection,
                busIndex: int32,
                latencyInSamples: uint32,
            ) -> tresult;
        }
        impl<P> IAudioPresentationLatencyTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IAudioPresentationLatency>,
        {
            #[inline]
            unsafe fn setAudioPresentationLatencySamples(
                &self,
                dir: BusDirection,
                busIndex: int32,
                latencyInSamples: uint32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IAudioPresentationLatency;
                ((*(*ptr).vtbl).setAudioPresentationLatencySamples)(
                    ptr,
                    dir,
                    busIndex,
                    latencyInSamples,
                )
            }
        }
        impl IAudioPresentationLatency {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IAudioPresentationLatencyVtbl
            where
                C: IAudioPresentationLatencyTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn setAudioPresentationLatencySamples<C, W, const OFFSET: isize>(
                    this: *mut IAudioPresentationLatency,
                    dir: BusDirection,
                    busIndex: int32,
                    latencyInSamples: uint32,
                ) -> tresult
                where
                    C: IAudioPresentationLatencyTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setAudioPresentationLatencySamples(
                        dir,
                        busIndex,
                        latencyInSamples,
                    )
                }
                IAudioPresentationLatencyVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    setAudioPresentationLatencySamples: setAudioPresentationLatencySamples::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IAudioPresentationLatency
        where
            C: IAudioPresentationLatencyTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IAudioPresentationLatency {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IAudioProcessor {
            pub vtbl: *const IAudioProcessorVtbl,
        }
        unsafe impl Send for IAudioProcessor {}
        unsafe impl Sync for IAudioProcessor {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IAudioProcessor {}
        impl ::com_scrape_types::Unknown for IAudioProcessor {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IAudioProcessor {
            type Vtbl = IAudioProcessorVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IAudioProcessor_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IAudioProcessorVtbl {
            pub base: FUnknownVtbl,
            pub setBusArrangements: unsafe extern "system" fn(
                this: *mut IAudioProcessor,
                inputs: *mut SpeakerArrangement,
                numIns: int32,
                outputs: *mut SpeakerArrangement,
                numOuts: int32,
            ) -> tresult,
            pub getBusArrangement: unsafe extern "system" fn(
                this: *mut IAudioProcessor,
                dir: BusDirection,
                index: int32,
                arr: *mut SpeakerArrangement,
            ) -> tresult,
            pub canProcessSampleSize: unsafe extern "system" fn(
                this: *mut IAudioProcessor,
                symbolicSampleSize: int32,
            ) -> tresult,
            pub getLatencySamples: unsafe extern "system" fn(
                this: *mut IAudioProcessor,
            ) -> uint32,
            pub setupProcessing: unsafe extern "system" fn(
                this: *mut IAudioProcessor,
                setup: *mut ProcessSetup,
            ) -> tresult,
            pub setProcessing: unsafe extern "system" fn(
                this: *mut IAudioProcessor,
                state: TBool,
            ) -> tresult,
            pub process: unsafe extern "system" fn(
                this: *mut IAudioProcessor,
                data: *mut ProcessData,
            ) -> tresult,
            pub getTailSamples: unsafe extern "system" fn(
                this: *mut IAudioProcessor,
            ) -> uint32,
        }
        pub trait IAudioProcessorTrait {
            unsafe fn setBusArrangements(
                &self,
                inputs: *mut SpeakerArrangement,
                numIns: int32,
                outputs: *mut SpeakerArrangement,
                numOuts: int32,
            ) -> tresult;
            unsafe fn getBusArrangement(
                &self,
                dir: BusDirection,
                index: int32,
                arr: *mut SpeakerArrangement,
            ) -> tresult;
            unsafe fn canProcessSampleSize(
                &self,
                symbolicSampleSize: int32,
            ) -> tresult;
            unsafe fn getLatencySamples(
                &self,
            ) -> uint32;
            unsafe fn setupProcessing(
                &self,
                setup: *mut ProcessSetup,
            ) -> tresult;
            unsafe fn setProcessing(
                &self,
                state: TBool,
            ) -> tresult;
            unsafe fn process(
                &self,
                data: *mut ProcessData,
            ) -> tresult;
            unsafe fn getTailSamples(
                &self,
            ) -> uint32;
        }
        impl<P> IAudioProcessorTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IAudioProcessor>,
        {
            #[inline]
            unsafe fn setBusArrangements(
                &self,
                inputs: *mut SpeakerArrangement,
                numIns: int32,
                outputs: *mut SpeakerArrangement,
                numOuts: int32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IAudioProcessor;
                ((*(*ptr).vtbl).setBusArrangements)(
                    ptr,
                    inputs,
                    numIns,
                    outputs,
                    numOuts,
                )
            }
            #[inline]
            unsafe fn getBusArrangement(
                &self,
                dir: BusDirection,
                index: int32,
                arr: *mut SpeakerArrangement,
            ) -> tresult {
                let ptr = self.ptr() as *mut IAudioProcessor;
                ((*(*ptr).vtbl).getBusArrangement)(
                    ptr,
                    dir,
                    index,
                    arr,
                )
            }
            #[inline]
            unsafe fn canProcessSampleSize(
                &self,
                symbolicSampleSize: int32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IAudioProcessor;
                ((*(*ptr).vtbl).canProcessSampleSize)(
                    ptr,
                    symbolicSampleSize,
                )
            }
            #[inline]
            unsafe fn getLatencySamples(
                &self,
            ) -> uint32 {
                let ptr = self.ptr() as *mut IAudioProcessor;
                ((*(*ptr).vtbl).getLatencySamples)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn setupProcessing(
                &self,
                setup: *mut ProcessSetup,
            ) -> tresult {
                let ptr = self.ptr() as *mut IAudioProcessor;
                ((*(*ptr).vtbl).setupProcessing)(
                    ptr,
                    setup,
                )
            }
            #[inline]
            unsafe fn setProcessing(
                &self,
                state: TBool,
            ) -> tresult {
                let ptr = self.ptr() as *mut IAudioProcessor;
                ((*(*ptr).vtbl).setProcessing)(
                    ptr,
                    state,
                )
            }
            #[inline]
            unsafe fn process(
                &self,
                data: *mut ProcessData,
            ) -> tresult {
                let ptr = self.ptr() as *mut IAudioProcessor;
                ((*(*ptr).vtbl).process)(
                    ptr,
                    data,
                )
            }
            #[inline]
            unsafe fn getTailSamples(
                &self,
            ) -> uint32 {
                let ptr = self.ptr() as *mut IAudioProcessor;
                ((*(*ptr).vtbl).getTailSamples)(
                    ptr,
                )
            }
        }
        impl IAudioProcessor {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IAudioProcessorVtbl
            where
                C: IAudioProcessorTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn setBusArrangements<C, W, const OFFSET: isize>(
                    this: *mut IAudioProcessor,
                    inputs: *mut SpeakerArrangement,
                    numIns: int32,
                    outputs: *mut SpeakerArrangement,
                    numOuts: int32,
                ) -> tresult
                where
                    C: IAudioProcessorTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setBusArrangements(
                        inputs,
                        numIns,
                        outputs,
                        numOuts,
                    )
                }
                unsafe extern "system" fn getBusArrangement<C, W, const OFFSET: isize>(
                    this: *mut IAudioProcessor,
                    dir: BusDirection,
                    index: int32,
                    arr: *mut SpeakerArrangement,
                ) -> tresult
                where
                    C: IAudioProcessorTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getBusArrangement(
                        dir,
                        index,
                        arr,
                    )
                }
                unsafe extern "system" fn canProcessSampleSize<C, W, const OFFSET: isize>(
                    this: *mut IAudioProcessor,
                    symbolicSampleSize: int32,
                ) -> tresult
                where
                    C: IAudioProcessorTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).canProcessSampleSize(
                        symbolicSampleSize,
                    )
                }
                unsafe extern "system" fn getLatencySamples<C, W, const OFFSET: isize>(
                    this: *mut IAudioProcessor,
                ) -> uint32
                where
                    C: IAudioProcessorTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getLatencySamples(
                    )
                }
                unsafe extern "system" fn setupProcessing<C, W, const OFFSET: isize>(
                    this: *mut IAudioProcessor,
                    setup: *mut ProcessSetup,
                ) -> tresult
                where
                    C: IAudioProcessorTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setupProcessing(
                        setup,
                    )
                }
                unsafe extern "system" fn setProcessing<C, W, const OFFSET: isize>(
                    this: *mut IAudioProcessor,
                    state: TBool,
                ) -> tresult
                where
                    C: IAudioProcessorTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setProcessing(
                        state,
                    )
                }
                unsafe extern "system" fn process<C, W, const OFFSET: isize>(
                    this: *mut IAudioProcessor,
                    data: *mut ProcessData,
                ) -> tresult
                where
                    C: IAudioProcessorTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).process(
                        data,
                    )
                }
                unsafe extern "system" fn getTailSamples<C, W, const OFFSET: isize>(
                    this: *mut IAudioProcessor,
                ) -> uint32
                where
                    C: IAudioProcessorTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getTailSamples(
                    )
                }
                IAudioProcessorVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    setBusArrangements: setBusArrangements::<C, W, OFFSET>,
                    getBusArrangement: getBusArrangement::<C, W, OFFSET>,
                    canProcessSampleSize: canProcessSampleSize::<C, W, OFFSET>,
                    getLatencySamples: getLatencySamples::<C, W, OFFSET>,
                    setupProcessing: setupProcessing::<C, W, OFFSET>,
                    setProcessing: setProcessing::<C, W, OFFSET>,
                    process: process::<C, W, OFFSET>,
                    getTailSamples: getTailSamples::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IAudioProcessor
        where
            C: IAudioProcessorTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IAudioProcessor {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        mod __IAutomationState_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::IAutomationState_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IAutomationState {
                pub vtbl: *const IAutomationStateVtbl,
            }
            unsafe impl Send for IAutomationState {}
            unsafe impl Sync for IAutomationState {}
            unsafe impl ::com_scrape_types::Inherits<FUnknown> for IAutomationState {}
            impl ::com_scrape_types::Unknown for IAutomationState {
                #[inline]
                unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                    crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
                }
                #[inline]
                unsafe fn add_ref(this: *mut Self) -> usize {
                    crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
                }
                #[inline]
                unsafe fn release(this: *mut Self) -> usize {
                    crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
                }
            }
            unsafe impl ::com_scrape_types::Interface for IAutomationState {
                type Vtbl = IAutomationStateVtbl;
                const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IAutomationState_iid);
                #[inline]
                fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                    iid == &Self::IID || FUnknown::inherits(iid)
                }
            }
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IAutomationStateVtbl {
                pub base: FUnknownVtbl,
                pub setAutomationState: unsafe extern "system" fn(
                    this: *mut IAutomationState,
                    state: int32,
                ) -> tresult,
            }
            pub trait IAutomationStateTrait {
                unsafe fn setAutomationState(
                    &self,
                    state: int32,
                ) -> tresult;
            }
            impl<P> IAutomationStateTrait for P
            where
                P: ::com_scrape_types::SmartPtr,
                P::Target: ::com_scrape_types::Inherits<IAutomationState>,
            {
                #[inline]
                unsafe fn setAutomationState(
                    &self,
                    state: int32,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IAutomationState;
                    ((*(*ptr).vtbl).setAutomationState)(
                        ptr,
                        state,
                    )
                }
            }
            impl IAutomationState {
                const fn make_vtbl<C, W, const OFFSET: isize>() -> IAutomationStateVtbl
                where
                    C: IAutomationStateTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    unsafe extern "system" fn setAutomationState<C, W, const OFFSET: isize>(
                        this: *mut IAutomationState,
                        state: int32,
                    ) -> tresult
                    where
                        C: IAutomationStateTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).setAutomationState(
                            state,
                        )
                    }
                    IAutomationStateVtbl {
                        base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                        setAutomationState: setAutomationState::<C, W, OFFSET>,
                    }
                }
            }
            unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IAutomationState
            where
                C: IAutomationStateTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                const OBJ: Self = IAutomationState {
                    vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
                };
            }
        }
        pub use __IAutomationState_wrapper::*;
        pub mod IAutomationState_ {
            #[allow(unused_imports)]
            use super::*;
            pub type AutomationStates = int32;
            pub mod AutomationStates_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kNoAutomation: AutomationStates = 0;
                pub const kReadState: AutomationStates = 1;
                pub const kReadWriteState: AutomationStates = 3;
                pub const kWriteState: AutomationStates = 2;
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponent {
            pub vtbl: *const IComponentVtbl,
        }
        unsafe impl Send for IComponent {}
        unsafe impl Sync for IComponent {}
        unsafe impl ::com_scrape_types::Inherits<IPluginBase> for IComponent {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IComponent {}
        impl ::com_scrape_types::Unknown for IComponent {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IComponent {
            type Vtbl = IComponentVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IComponent_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || IPluginBase::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentVtbl {
            pub base: IPluginBaseVtbl,
            pub getControllerClassId: unsafe extern "system" fn(
                this: *mut IComponent,
                classId: *mut TUID,
            ) -> tresult,
            pub setIoMode: unsafe extern "system" fn(
                this: *mut IComponent,
                mode: IoMode,
            ) -> tresult,
            pub getBusCount: unsafe extern "system" fn(
                this: *mut IComponent,
                r#type: MediaType,
                dir: BusDirection,
            ) -> int32,
            pub getBusInfo: unsafe extern "system" fn(
                this: *mut IComponent,
                r#type: MediaType,
                dir: BusDirection,
                index: int32,
                bus: *mut BusInfo,
            ) -> tresult,
            pub getRoutingInfo: unsafe extern "system" fn(
                this: *mut IComponent,
                inInfo: *mut RoutingInfo,
                outInfo: *mut RoutingInfo,
            ) -> tresult,
            pub activateBus: unsafe extern "system" fn(
                this: *mut IComponent,
                r#type: MediaType,
                dir: BusDirection,
                index: int32,
                state: TBool,
            ) -> tresult,
            pub setActive: unsafe extern "system" fn(
                this: *mut IComponent,
                state: TBool,
            ) -> tresult,
            pub setState: unsafe extern "system" fn(
                this: *mut IComponent,
                state: *mut IBStream,
            ) -> tresult,
            pub getState: unsafe extern "system" fn(
                this: *mut IComponent,
                state: *mut IBStream,
            ) -> tresult,
        }
        pub trait IComponentTrait: IPluginBaseTrait {
            unsafe fn getControllerClassId(
                &self,
                classId: *mut TUID,
            ) -> tresult;
            unsafe fn setIoMode(
                &self,
                mode: IoMode,
            ) -> tresult;
            unsafe fn getBusCount(
                &self,
                r#type: MediaType,
                dir: BusDirection,
            ) -> int32;
            unsafe fn getBusInfo(
                &self,
                r#type: MediaType,
                dir: BusDirection,
                index: int32,
                bus: *mut BusInfo,
            ) -> tresult;
            unsafe fn getRoutingInfo(
                &self,
                inInfo: *mut RoutingInfo,
                outInfo: *mut RoutingInfo,
            ) -> tresult;
            unsafe fn activateBus(
                &self,
                r#type: MediaType,
                dir: BusDirection,
                index: int32,
                state: TBool,
            ) -> tresult;
            unsafe fn setActive(
                &self,
                state: TBool,
            ) -> tresult;
            unsafe fn setState(
                &self,
                state: *mut IBStream,
            ) -> tresult;
            unsafe fn getState(
                &self,
                state: *mut IBStream,
            ) -> tresult;
        }
        impl<P> IComponentTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IComponent>,
            P::Target: ::com_scrape_types::Inherits<IPluginBase>,
        {
            #[inline]
            unsafe fn getControllerClassId(
                &self,
                classId: *mut TUID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponent;
                ((*(*ptr).vtbl).getControllerClassId)(
                    ptr,
                    classId,
                )
            }
            #[inline]
            unsafe fn setIoMode(
                &self,
                mode: IoMode,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponent;
                ((*(*ptr).vtbl).setIoMode)(
                    ptr,
                    mode,
                )
            }
            #[inline]
            unsafe fn getBusCount(
                &self,
                r#type: MediaType,
                dir: BusDirection,
            ) -> int32 {
                let ptr = self.ptr() as *mut IComponent;
                ((*(*ptr).vtbl).getBusCount)(
                    ptr,
                    r#type,
                    dir,
                )
            }
            #[inline]
            unsafe fn getBusInfo(
                &self,
                r#type: MediaType,
                dir: BusDirection,
                index: int32,
                bus: *mut BusInfo,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponent;
                ((*(*ptr).vtbl).getBusInfo)(
                    ptr,
                    r#type,
                    dir,
                    index,
                    bus,
                )
            }
            #[inline]
            unsafe fn getRoutingInfo(
                &self,
                inInfo: *mut RoutingInfo,
                outInfo: *mut RoutingInfo,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponent;
                ((*(*ptr).vtbl).getRoutingInfo)(
                    ptr,
                    inInfo,
                    outInfo,
                )
            }
            #[inline]
            unsafe fn activateBus(
                &self,
                r#type: MediaType,
                dir: BusDirection,
                index: int32,
                state: TBool,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponent;
                ((*(*ptr).vtbl).activateBus)(
                    ptr,
                    r#type,
                    dir,
                    index,
                    state,
                )
            }
            #[inline]
            unsafe fn setActive(
                &self,
                state: TBool,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponent;
                ((*(*ptr).vtbl).setActive)(
                    ptr,
                    state,
                )
            }
            #[inline]
            unsafe fn setState(
                &self,
                state: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponent;
                ((*(*ptr).vtbl).setState)(
                    ptr,
                    state,
                )
            }
            #[inline]
            unsafe fn getState(
                &self,
                state: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponent;
                ((*(*ptr).vtbl).getState)(
                    ptr,
                    state,
                )
            }
        }
        impl IComponent {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IComponentVtbl
            where
                C: IComponentTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getControllerClassId<C, W, const OFFSET: isize>(
                    this: *mut IComponent,
                    classId: *mut TUID,
                ) -> tresult
                where
                    C: IComponentTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getControllerClassId(
                        classId,
                    )
                }
                unsafe extern "system" fn setIoMode<C, W, const OFFSET: isize>(
                    this: *mut IComponent,
                    mode: IoMode,
                ) -> tresult
                where
                    C: IComponentTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setIoMode(
                        mode,
                    )
                }
                unsafe extern "system" fn getBusCount<C, W, const OFFSET: isize>(
                    this: *mut IComponent,
                    r#type: MediaType,
                    dir: BusDirection,
                ) -> int32
                where
                    C: IComponentTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getBusCount(
                        r#type,
                        dir,
                    )
                }
                unsafe extern "system" fn getBusInfo<C, W, const OFFSET: isize>(
                    this: *mut IComponent,
                    r#type: MediaType,
                    dir: BusDirection,
                    index: int32,
                    bus: *mut BusInfo,
                ) -> tresult
                where
                    C: IComponentTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getBusInfo(
                        r#type,
                        dir,
                        index,
                        bus,
                    )
                }
                unsafe extern "system" fn getRoutingInfo<C, W, const OFFSET: isize>(
                    this: *mut IComponent,
                    inInfo: *mut RoutingInfo,
                    outInfo: *mut RoutingInfo,
                ) -> tresult
                where
                    C: IComponentTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getRoutingInfo(
                        inInfo,
                        outInfo,
                    )
                }
                unsafe extern "system" fn activateBus<C, W, const OFFSET: isize>(
                    this: *mut IComponent,
                    r#type: MediaType,
                    dir: BusDirection,
                    index: int32,
                    state: TBool,
                ) -> tresult
                where
                    C: IComponentTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).activateBus(
                        r#type,
                        dir,
                        index,
                        state,
                    )
                }
                unsafe extern "system" fn setActive<C, W, const OFFSET: isize>(
                    this: *mut IComponent,
                    state: TBool,
                ) -> tresult
                where
                    C: IComponentTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setActive(
                        state,
                    )
                }
                unsafe extern "system" fn setState<C, W, const OFFSET: isize>(
                    this: *mut IComponent,
                    state: *mut IBStream,
                ) -> tresult
                where
                    C: IComponentTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setState(
                        state,
                    )
                }
                unsafe extern "system" fn getState<C, W, const OFFSET: isize>(
                    this: *mut IComponent,
                    state: *mut IBStream,
                ) -> tresult
                where
                    C: IComponentTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getState(
                        state,
                    )
                }
                IComponentVtbl {
                    base: IPluginBase::make_vtbl::<C, W, OFFSET>(),
                    getControllerClassId: getControllerClassId::<C, W, OFFSET>,
                    setIoMode: setIoMode::<C, W, OFFSET>,
                    getBusCount: getBusCount::<C, W, OFFSET>,
                    getBusInfo: getBusInfo::<C, W, OFFSET>,
                    getRoutingInfo: getRoutingInfo::<C, W, OFFSET>,
                    activateBus: activateBus::<C, W, OFFSET>,
                    setActive: setActive::<C, W, OFFSET>,
                    setState: setState::<C, W, OFFSET>,
                    getState: getState::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IComponent
        where
            C: IComponentTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IComponent {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentHandler {
            pub vtbl: *const IComponentHandlerVtbl,
        }
        unsafe impl Send for IComponentHandler {}
        unsafe impl Sync for IComponentHandler {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IComponentHandler {}
        impl ::com_scrape_types::Unknown for IComponentHandler {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IComponentHandler {
            type Vtbl = IComponentHandlerVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IComponentHandler_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentHandlerVtbl {
            pub base: FUnknownVtbl,
            pub beginEdit: unsafe extern "system" fn(
                this: *mut IComponentHandler,
                id: ParamID,
            ) -> tresult,
            pub performEdit: unsafe extern "system" fn(
                this: *mut IComponentHandler,
                id: ParamID,
                valueNormalized: ParamValue,
            ) -> tresult,
            pub endEdit: unsafe extern "system" fn(
                this: *mut IComponentHandler,
                id: ParamID,
            ) -> tresult,
            pub restartComponent: unsafe extern "system" fn(
                this: *mut IComponentHandler,
                flags: int32,
            ) -> tresult,
        }
        pub trait IComponentHandlerTrait {
            unsafe fn beginEdit(
                &self,
                id: ParamID,
            ) -> tresult;
            unsafe fn performEdit(
                &self,
                id: ParamID,
                valueNormalized: ParamValue,
            ) -> tresult;
            unsafe fn endEdit(
                &self,
                id: ParamID,
            ) -> tresult;
            unsafe fn restartComponent(
                &self,
                flags: int32,
            ) -> tresult;
        }
        impl<P> IComponentHandlerTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IComponentHandler>,
        {
            #[inline]
            unsafe fn beginEdit(
                &self,
                id: ParamID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponentHandler;
                ((*(*ptr).vtbl).beginEdit)(
                    ptr,
                    id,
                )
            }
            #[inline]
            unsafe fn performEdit(
                &self,
                id: ParamID,
                valueNormalized: ParamValue,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponentHandler;
                ((*(*ptr).vtbl).performEdit)(
                    ptr,
                    id,
                    valueNormalized,
                )
            }
            #[inline]
            unsafe fn endEdit(
                &self,
                id: ParamID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponentHandler;
                ((*(*ptr).vtbl).endEdit)(
                    ptr,
                    id,
                )
            }
            #[inline]
            unsafe fn restartComponent(
                &self,
                flags: int32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponentHandler;
                ((*(*ptr).vtbl).restartComponent)(
                    ptr,
                    flags,
                )
            }
        }
        impl IComponentHandler {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IComponentHandlerVtbl
            where
                C: IComponentHandlerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn beginEdit<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandler,
                    id: ParamID,
                ) -> tresult
                where
                    C: IComponentHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).beginEdit(
                        id,
                    )
                }
                unsafe extern "system" fn performEdit<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandler,
                    id: ParamID,
                    valueNormalized: ParamValue,
                ) -> tresult
                where
                    C: IComponentHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).performEdit(
                        id,
                        valueNormalized,
                    )
                }
                unsafe extern "system" fn endEdit<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandler,
                    id: ParamID,
                ) -> tresult
                where
                    C: IComponentHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).endEdit(
                        id,
                    )
                }
                unsafe extern "system" fn restartComponent<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandler,
                    flags: int32,
                ) -> tresult
                where
                    C: IComponentHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).restartComponent(
                        flags,
                    )
                }
                IComponentHandlerVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    beginEdit: beginEdit::<C, W, OFFSET>,
                    performEdit: performEdit::<C, W, OFFSET>,
                    endEdit: endEdit::<C, W, OFFSET>,
                    restartComponent: restartComponent::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IComponentHandler
        where
            C: IComponentHandlerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IComponentHandler {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentHandler2 {
            pub vtbl: *const IComponentHandler2Vtbl,
        }
        unsafe impl Send for IComponentHandler2 {}
        unsafe impl Sync for IComponentHandler2 {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IComponentHandler2 {}
        impl ::com_scrape_types::Unknown for IComponentHandler2 {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IComponentHandler2 {
            type Vtbl = IComponentHandler2Vtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IComponentHandler2_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentHandler2Vtbl {
            pub base: FUnknownVtbl,
            pub setDirty: unsafe extern "system" fn(
                this: *mut IComponentHandler2,
                state: TBool,
            ) -> tresult,
            pub requestOpenEditor: unsafe extern "system" fn(
                this: *mut IComponentHandler2,
                name: FIDString,
            ) -> tresult,
            pub startGroupEdit: unsafe extern "system" fn(
                this: *mut IComponentHandler2,
            ) -> tresult,
            pub finishGroupEdit: unsafe extern "system" fn(
                this: *mut IComponentHandler2,
            ) -> tresult,
        }
        pub trait IComponentHandler2Trait {
            unsafe fn setDirty(
                &self,
                state: TBool,
            ) -> tresult;
            unsafe fn requestOpenEditor(
                &self,
                name: FIDString,
            ) -> tresult;
            unsafe fn startGroupEdit(
                &self,
            ) -> tresult;
            unsafe fn finishGroupEdit(
                &self,
            ) -> tresult;
        }
        impl<P> IComponentHandler2Trait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IComponentHandler2>,
        {
            #[inline]
            unsafe fn setDirty(
                &self,
                state: TBool,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponentHandler2;
                ((*(*ptr).vtbl).setDirty)(
                    ptr,
                    state,
                )
            }
            #[inline]
            unsafe fn requestOpenEditor(
                &self,
                name: FIDString,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponentHandler2;
                ((*(*ptr).vtbl).requestOpenEditor)(
                    ptr,
                    name,
                )
            }
            #[inline]
            unsafe fn startGroupEdit(
                &self,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponentHandler2;
                ((*(*ptr).vtbl).startGroupEdit)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn finishGroupEdit(
                &self,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponentHandler2;
                ((*(*ptr).vtbl).finishGroupEdit)(
                    ptr,
                )
            }
        }
        impl IComponentHandler2 {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IComponentHandler2Vtbl
            where
                C: IComponentHandler2Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn setDirty<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandler2,
                    state: TBool,
                ) -> tresult
                where
                    C: IComponentHandler2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setDirty(
                        state,
                    )
                }
                unsafe extern "system" fn requestOpenEditor<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandler2,
                    name: FIDString,
                ) -> tresult
                where
                    C: IComponentHandler2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).requestOpenEditor(
                        name,
                    )
                }
                unsafe extern "system" fn startGroupEdit<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandler2,
                ) -> tresult
                where
                    C: IComponentHandler2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).startGroupEdit(
                    )
                }
                unsafe extern "system" fn finishGroupEdit<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandler2,
                ) -> tresult
                where
                    C: IComponentHandler2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).finishGroupEdit(
                    )
                }
                IComponentHandler2Vtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    setDirty: setDirty::<C, W, OFFSET>,
                    requestOpenEditor: requestOpenEditor::<C, W, OFFSET>,
                    startGroupEdit: startGroupEdit::<C, W, OFFSET>,
                    finishGroupEdit: finishGroupEdit::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IComponentHandler2
        where
            C: IComponentHandler2Trait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IComponentHandler2 {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentHandler3 {
            pub vtbl: *const IComponentHandler3Vtbl,
        }
        unsafe impl Send for IComponentHandler3 {}
        unsafe impl Sync for IComponentHandler3 {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IComponentHandler3 {}
        impl ::com_scrape_types::Unknown for IComponentHandler3 {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IComponentHandler3 {
            type Vtbl = IComponentHandler3Vtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IComponentHandler3_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentHandler3Vtbl {
            pub base: FUnknownVtbl,
            pub createContextMenu: unsafe extern "system" fn(
                this: *mut IComponentHandler3,
                plugView: *mut IPlugView,
                paramID: *const ParamID,
            ) -> *mut IContextMenu,
        }
        pub trait IComponentHandler3Trait {
            unsafe fn createContextMenu(
                &self,
                plugView: *mut IPlugView,
                paramID: *const ParamID,
            ) -> *mut IContextMenu;
        }
        impl<P> IComponentHandler3Trait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IComponentHandler3>,
        {
            #[inline]
            unsafe fn createContextMenu(
                &self,
                plugView: *mut IPlugView,
                paramID: *const ParamID,
            ) -> *mut IContextMenu {
                let ptr = self.ptr() as *mut IComponentHandler3;
                ((*(*ptr).vtbl).createContextMenu)(
                    ptr,
                    plugView,
                    paramID,
                )
            }
        }
        impl IComponentHandler3 {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IComponentHandler3Vtbl
            where
                C: IComponentHandler3Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn createContextMenu<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandler3,
                    plugView: *mut IPlugView,
                    paramID: *const ParamID,
                ) -> *mut IContextMenu
                where
                    C: IComponentHandler3Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).createContextMenu(
                        plugView,
                        paramID,
                    )
                }
                IComponentHandler3Vtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    createContextMenu: createContextMenu::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IComponentHandler3
        where
            C: IComponentHandler3Trait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IComponentHandler3 {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentHandlerBusActivation {
            pub vtbl: *const IComponentHandlerBusActivationVtbl,
        }
        unsafe impl Send for IComponentHandlerBusActivation {}
        unsafe impl Sync for IComponentHandlerBusActivation {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IComponentHandlerBusActivation {}
        impl ::com_scrape_types::Unknown for IComponentHandlerBusActivation {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IComponentHandlerBusActivation {
            type Vtbl = IComponentHandlerBusActivationVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IComponentHandlerBusActivation_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentHandlerBusActivationVtbl {
            pub base: FUnknownVtbl,
            pub requestBusActivation: unsafe extern "system" fn(
                this: *mut IComponentHandlerBusActivation,
                r#type: MediaType,
                dir: BusDirection,
                index: int32,
                state: TBool,
            ) -> tresult,
        }
        pub trait IComponentHandlerBusActivationTrait {
            unsafe fn requestBusActivation(
                &self,
                r#type: MediaType,
                dir: BusDirection,
                index: int32,
                state: TBool,
            ) -> tresult;
        }
        impl<P> IComponentHandlerBusActivationTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IComponentHandlerBusActivation>,
        {
            #[inline]
            unsafe fn requestBusActivation(
                &self,
                r#type: MediaType,
                dir: BusDirection,
                index: int32,
                state: TBool,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponentHandlerBusActivation;
                ((*(*ptr).vtbl).requestBusActivation)(
                    ptr,
                    r#type,
                    dir,
                    index,
                    state,
                )
            }
        }
        impl IComponentHandlerBusActivation {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IComponentHandlerBusActivationVtbl
            where
                C: IComponentHandlerBusActivationTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn requestBusActivation<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandlerBusActivation,
                    r#type: MediaType,
                    dir: BusDirection,
                    index: int32,
                    state: TBool,
                ) -> tresult
                where
                    C: IComponentHandlerBusActivationTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).requestBusActivation(
                        r#type,
                        dir,
                        index,
                        state,
                    )
                }
                IComponentHandlerBusActivationVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    requestBusActivation: requestBusActivation::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IComponentHandlerBusActivation
        where
            C: IComponentHandlerBusActivationTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IComponentHandlerBusActivation {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentHandlerSystemTime {
            pub vtbl: *const IComponentHandlerSystemTimeVtbl,
        }
        unsafe impl Send for IComponentHandlerSystemTime {}
        unsafe impl Sync for IComponentHandlerSystemTime {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IComponentHandlerSystemTime {}
        impl ::com_scrape_types::Unknown for IComponentHandlerSystemTime {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IComponentHandlerSystemTime {
            type Vtbl = IComponentHandlerSystemTimeVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IComponentHandlerSystemTime_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IComponentHandlerSystemTimeVtbl {
            pub base: FUnknownVtbl,
            pub getSystemTime: unsafe extern "system" fn(
                this: *mut IComponentHandlerSystemTime,
                systemTime: *mut int64,
            ) -> tresult,
        }
        pub trait IComponentHandlerSystemTimeTrait {
            unsafe fn getSystemTime(
                &self,
                systemTime: *mut int64,
            ) -> tresult;
        }
        impl<P> IComponentHandlerSystemTimeTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IComponentHandlerSystemTime>,
        {
            #[inline]
            unsafe fn getSystemTime(
                &self,
                systemTime: *mut int64,
            ) -> tresult {
                let ptr = self.ptr() as *mut IComponentHandlerSystemTime;
                ((*(*ptr).vtbl).getSystemTime)(
                    ptr,
                    systemTime,
                )
            }
        }
        impl IComponentHandlerSystemTime {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IComponentHandlerSystemTimeVtbl
            where
                C: IComponentHandlerSystemTimeTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getSystemTime<C, W, const OFFSET: isize>(
                    this: *mut IComponentHandlerSystemTime,
                    systemTime: *mut int64,
                ) -> tresult
                where
                    C: IComponentHandlerSystemTimeTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getSystemTime(
                        systemTime,
                    )
                }
                IComponentHandlerSystemTimeVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getSystemTime: getSystemTime::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IComponentHandlerSystemTime
        where
            C: IComponentHandlerSystemTimeTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IComponentHandlerSystemTime {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IConnectionPoint {
            pub vtbl: *const IConnectionPointVtbl,
        }
        unsafe impl Send for IConnectionPoint {}
        unsafe impl Sync for IConnectionPoint {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IConnectionPoint {}
        impl ::com_scrape_types::Unknown for IConnectionPoint {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IConnectionPoint {
            type Vtbl = IConnectionPointVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IConnectionPoint_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IConnectionPointVtbl {
            pub base: FUnknownVtbl,
            pub connect: unsafe extern "system" fn(
                this: *mut IConnectionPoint,
                other: *mut IConnectionPoint,
            ) -> tresult,
            pub disconnect: unsafe extern "system" fn(
                this: *mut IConnectionPoint,
                other: *mut IConnectionPoint,
            ) -> tresult,
            pub notify: unsafe extern "system" fn(
                this: *mut IConnectionPoint,
                message: *mut IMessage,
            ) -> tresult,
        }
        pub trait IConnectionPointTrait {
            unsafe fn connect(
                &self,
                other: *mut IConnectionPoint,
            ) -> tresult;
            unsafe fn disconnect(
                &self,
                other: *mut IConnectionPoint,
            ) -> tresult;
            unsafe fn notify(
                &self,
                message: *mut IMessage,
            ) -> tresult;
        }
        impl<P> IConnectionPointTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IConnectionPoint>,
        {
            #[inline]
            unsafe fn connect(
                &self,
                other: *mut IConnectionPoint,
            ) -> tresult {
                let ptr = self.ptr() as *mut IConnectionPoint;
                ((*(*ptr).vtbl).connect)(
                    ptr,
                    other,
                )
            }
            #[inline]
            unsafe fn disconnect(
                &self,
                other: *mut IConnectionPoint,
            ) -> tresult {
                let ptr = self.ptr() as *mut IConnectionPoint;
                ((*(*ptr).vtbl).disconnect)(
                    ptr,
                    other,
                )
            }
            #[inline]
            unsafe fn notify(
                &self,
                message: *mut IMessage,
            ) -> tresult {
                let ptr = self.ptr() as *mut IConnectionPoint;
                ((*(*ptr).vtbl).notify)(
                    ptr,
                    message,
                )
            }
        }
        impl IConnectionPoint {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IConnectionPointVtbl
            where
                C: IConnectionPointTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn connect<C, W, const OFFSET: isize>(
                    this: *mut IConnectionPoint,
                    other: *mut IConnectionPoint,
                ) -> tresult
                where
                    C: IConnectionPointTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).connect(
                        other,
                    )
                }
                unsafe extern "system" fn disconnect<C, W, const OFFSET: isize>(
                    this: *mut IConnectionPoint,
                    other: *mut IConnectionPoint,
                ) -> tresult
                where
                    C: IConnectionPointTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).disconnect(
                        other,
                    )
                }
                unsafe extern "system" fn notify<C, W, const OFFSET: isize>(
                    this: *mut IConnectionPoint,
                    message: *mut IMessage,
                ) -> tresult
                where
                    C: IConnectionPointTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).notify(
                        message,
                    )
                }
                IConnectionPointVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    connect: connect::<C, W, OFFSET>,
                    disconnect: disconnect::<C, W, OFFSET>,
                    notify: notify::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IConnectionPoint
        where
            C: IConnectionPointTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IConnectionPoint {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        mod __IContextMenu_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::IContextMenu_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IContextMenu {
                pub vtbl: *const IContextMenuVtbl,
            }
            unsafe impl Send for IContextMenu {}
            unsafe impl Sync for IContextMenu {}
            unsafe impl ::com_scrape_types::Inherits<FUnknown> for IContextMenu {}
            impl ::com_scrape_types::Unknown for IContextMenu {
                #[inline]
                unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                    crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
                }
                #[inline]
                unsafe fn add_ref(this: *mut Self) -> usize {
                    crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
                }
                #[inline]
                unsafe fn release(this: *mut Self) -> usize {
                    crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
                }
            }
            unsafe impl ::com_scrape_types::Interface for IContextMenu {
                type Vtbl = IContextMenuVtbl;
                const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IContextMenu_iid);
                #[inline]
                fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                    iid == &Self::IID || FUnknown::inherits(iid)
                }
            }
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IContextMenuVtbl {
                pub base: FUnknownVtbl,
                pub getItemCount: unsafe extern "system" fn(
                    this: *mut IContextMenu,
                ) -> int32,
                pub getItem: unsafe extern "system" fn(
                    this: *mut IContextMenu,
                    index: int32,
                    item: *mut Item,
                    target: *mut *mut IContextMenuTarget,
                ) -> tresult,
                pub addItem: unsafe extern "system" fn(
                    this: *mut IContextMenu,
                    item: *const Item,
                    target: *mut IContextMenuTarget,
                ) -> tresult,
                pub removeItem: unsafe extern "system" fn(
                    this: *mut IContextMenu,
                    item: *const Item,
                    target: *mut IContextMenuTarget,
                ) -> tresult,
                pub popup: unsafe extern "system" fn(
                    this: *mut IContextMenu,
                    x: UCoord,
                    y: UCoord,
                ) -> tresult,
            }
            pub trait IContextMenuTrait {
                unsafe fn getItemCount(
                    &self,
                ) -> int32;
                unsafe fn getItem(
                    &self,
                    index: int32,
                    item: *mut Item,
                    target: *mut *mut IContextMenuTarget,
                ) -> tresult;
                unsafe fn addItem(
                    &self,
                    item: *const Item,
                    target: *mut IContextMenuTarget,
                ) -> tresult;
                unsafe fn removeItem(
                    &self,
                    item: *const Item,
                    target: *mut IContextMenuTarget,
                ) -> tresult;
                unsafe fn popup(
                    &self,
                    x: UCoord,
                    y: UCoord,
                ) -> tresult;
            }
            impl<P> IContextMenuTrait for P
            where
                P: ::com_scrape_types::SmartPtr,
                P::Target: ::com_scrape_types::Inherits<IContextMenu>,
            {
                #[inline]
                unsafe fn getItemCount(
                    &self,
                ) -> int32 {
                    let ptr = self.ptr() as *mut IContextMenu;
                    ((*(*ptr).vtbl).getItemCount)(
                        ptr,
                    )
                }
                #[inline]
                unsafe fn getItem(
                    &self,
                    index: int32,
                    item: *mut Item,
                    target: *mut *mut IContextMenuTarget,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IContextMenu;
                    ((*(*ptr).vtbl).getItem)(
                        ptr,
                        index,
                        item,
                        target,
                    )
                }
                #[inline]
                unsafe fn addItem(
                    &self,
                    item: *const Item,
                    target: *mut IContextMenuTarget,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IContextMenu;
                    ((*(*ptr).vtbl).addItem)(
                        ptr,
                        item,
                        target,
                    )
                }
                #[inline]
                unsafe fn removeItem(
                    &self,
                    item: *const Item,
                    target: *mut IContextMenuTarget,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IContextMenu;
                    ((*(*ptr).vtbl).removeItem)(
                        ptr,
                        item,
                        target,
                    )
                }
                #[inline]
                unsafe fn popup(
                    &self,
                    x: UCoord,
                    y: UCoord,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IContextMenu;
                    ((*(*ptr).vtbl).popup)(
                        ptr,
                        x,
                        y,
                    )
                }
            }
            impl IContextMenu {
                const fn make_vtbl<C, W, const OFFSET: isize>() -> IContextMenuVtbl
                where
                    C: IContextMenuTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    unsafe extern "system" fn getItemCount<C, W, const OFFSET: isize>(
                        this: *mut IContextMenu,
                    ) -> int32
                    where
                        C: IContextMenuTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).getItemCount(
                        )
                    }
                    unsafe extern "system" fn getItem<C, W, const OFFSET: isize>(
                        this: *mut IContextMenu,
                        index: int32,
                        item: *mut Item,
                        target: *mut *mut IContextMenuTarget,
                    ) -> tresult
                    where
                        C: IContextMenuTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).getItem(
                            index,
                            item,
                            target,
                        )
                    }
                    unsafe extern "system" fn addItem<C, W, const OFFSET: isize>(
                        this: *mut IContextMenu,
                        item: *const Item,
                        target: *mut IContextMenuTarget,
                    ) -> tresult
                    where
                        C: IContextMenuTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).addItem(
                            item,
                            target,
                        )
                    }
                    unsafe extern "system" fn removeItem<C, W, const OFFSET: isize>(
                        this: *mut IContextMenu,
                        item: *const Item,
                        target: *mut IContextMenuTarget,
                    ) -> tresult
                    where
                        C: IContextMenuTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).removeItem(
                            item,
                            target,
                        )
                    }
                    unsafe extern "system" fn popup<C, W, const OFFSET: isize>(
                        this: *mut IContextMenu,
                        x: UCoord,
                        y: UCoord,
                    ) -> tresult
                    where
                        C: IContextMenuTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).popup(
                            x,
                            y,
                        )
                    }
                    IContextMenuVtbl {
                        base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                        getItemCount: getItemCount::<C, W, OFFSET>,
                        getItem: getItem::<C, W, OFFSET>,
                        addItem: addItem::<C, W, OFFSET>,
                        removeItem: removeItem::<C, W, OFFSET>,
                        popup: popup::<C, W, OFFSET>,
                    }
                }
            }
            unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IContextMenu
            where
                C: IContextMenuTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                const OBJ: Self = IContextMenu {
                    vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
                };
            }
        }
        pub use __IContextMenu_wrapper::*;
        pub mod IContextMenu_ {
            #[allow(unused_imports)]
            use super::*;
            pub type Item = IContextMenuItem;
        }
        mod __IContextMenuItem_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::IContextMenuItem_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IContextMenuItem {
                pub name: String128,
                pub tag: int32,
                pub flags: int32,
            }
            unsafe impl Send for IContextMenuItem {}
            unsafe impl Sync for IContextMenuItem {}
        }
        pub use __IContextMenuItem_wrapper::*;
        pub mod IContextMenuItem_ {
            #[allow(unused_imports)]
            use super::*;
            pub type Flags = crate::support::DefaultEnumType;
            pub mod Flags_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kIsChecked: Flags = 4;
                pub const kIsDisabled: Flags = 2;
                pub const kIsGroupEnd: Flags = 17;
                pub const kIsGroupStart: Flags = 10;
                pub const kIsSeparator: Flags = 1;
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IContextMenuTarget {
            pub vtbl: *const IContextMenuTargetVtbl,
        }
        unsafe impl Send for IContextMenuTarget {}
        unsafe impl Sync for IContextMenuTarget {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IContextMenuTarget {}
        impl ::com_scrape_types::Unknown for IContextMenuTarget {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IContextMenuTarget {
            type Vtbl = IContextMenuTargetVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IContextMenuTarget_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IContextMenuTargetVtbl {
            pub base: FUnknownVtbl,
            pub executeMenuItem: unsafe extern "system" fn(
                this: *mut IContextMenuTarget,
                tag: int32,
            ) -> tresult,
        }
        pub trait IContextMenuTargetTrait {
            unsafe fn executeMenuItem(
                &self,
                tag: int32,
            ) -> tresult;
        }
        impl<P> IContextMenuTargetTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IContextMenuTarget>,
        {
            #[inline]
            unsafe fn executeMenuItem(
                &self,
                tag: int32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IContextMenuTarget;
                ((*(*ptr).vtbl).executeMenuItem)(
                    ptr,
                    tag,
                )
            }
        }
        impl IContextMenuTarget {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IContextMenuTargetVtbl
            where
                C: IContextMenuTargetTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn executeMenuItem<C, W, const OFFSET: isize>(
                    this: *mut IContextMenuTarget,
                    tag: int32,
                ) -> tresult
                where
                    C: IContextMenuTargetTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).executeMenuItem(
                        tag,
                    )
                }
                IContextMenuTargetVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    executeMenuItem: executeMenuItem::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IContextMenuTarget
        where
            C: IContextMenuTargetTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IContextMenuTarget {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IDataExchangeHandler {
            pub vtbl: *const IDataExchangeHandlerVtbl,
        }
        unsafe impl Send for IDataExchangeHandler {}
        unsafe impl Sync for IDataExchangeHandler {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IDataExchangeHandler {}
        impl ::com_scrape_types::Unknown for IDataExchangeHandler {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IDataExchangeHandler {
            type Vtbl = IDataExchangeHandlerVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IDataExchangeHandler_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IDataExchangeHandlerVtbl {
            pub base: FUnknownVtbl,
            pub openQueue: unsafe extern "system" fn(
                this: *mut IDataExchangeHandler,
                processor: *mut IAudioProcessor,
                blockSize: uint32,
                numBlocks: uint32,
                alignment: uint32,
                userContextID: DataExchangeUserContextID,
                outID: *mut DataExchangeQueueID,
            ) -> tresult,
            pub closeQueue: unsafe extern "system" fn(
                this: *mut IDataExchangeHandler,
                queueID: DataExchangeQueueID,
            ) -> tresult,
            pub lockBlock: unsafe extern "system" fn(
                this: *mut IDataExchangeHandler,
                queueId: DataExchangeQueueID,
                block: *mut DataExchangeBlock,
            ) -> tresult,
            pub freeBlock: unsafe extern "system" fn(
                this: *mut IDataExchangeHandler,
                queueId: DataExchangeQueueID,
                blockID: DataExchangeBlockID,
                sendToController: TBool,
            ) -> tresult,
        }
        pub trait IDataExchangeHandlerTrait {
            unsafe fn openQueue(
                &self,
                processor: *mut IAudioProcessor,
                blockSize: uint32,
                numBlocks: uint32,
                alignment: uint32,
                userContextID: DataExchangeUserContextID,
                outID: *mut DataExchangeQueueID,
            ) -> tresult;
            unsafe fn closeQueue(
                &self,
                queueID: DataExchangeQueueID,
            ) -> tresult;
            unsafe fn lockBlock(
                &self,
                queueId: DataExchangeQueueID,
                block: *mut DataExchangeBlock,
            ) -> tresult;
            unsafe fn freeBlock(
                &self,
                queueId: DataExchangeQueueID,
                blockID: DataExchangeBlockID,
                sendToController: TBool,
            ) -> tresult;
        }
        impl<P> IDataExchangeHandlerTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IDataExchangeHandler>,
        {
            #[inline]
            unsafe fn openQueue(
                &self,
                processor: *mut IAudioProcessor,
                blockSize: uint32,
                numBlocks: uint32,
                alignment: uint32,
                userContextID: DataExchangeUserContextID,
                outID: *mut DataExchangeQueueID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IDataExchangeHandler;
                ((*(*ptr).vtbl).openQueue)(
                    ptr,
                    processor,
                    blockSize,
                    numBlocks,
                    alignment,
                    userContextID,
                    outID,
                )
            }
            #[inline]
            unsafe fn closeQueue(
                &self,
                queueID: DataExchangeQueueID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IDataExchangeHandler;
                ((*(*ptr).vtbl).closeQueue)(
                    ptr,
                    queueID,
                )
            }
            #[inline]
            unsafe fn lockBlock(
                &self,
                queueId: DataExchangeQueueID,
                block: *mut DataExchangeBlock,
            ) -> tresult {
                let ptr = self.ptr() as *mut IDataExchangeHandler;
                ((*(*ptr).vtbl).lockBlock)(
                    ptr,
                    queueId,
                    block,
                )
            }
            #[inline]
            unsafe fn freeBlock(
                &self,
                queueId: DataExchangeQueueID,
                blockID: DataExchangeBlockID,
                sendToController: TBool,
            ) -> tresult {
                let ptr = self.ptr() as *mut IDataExchangeHandler;
                ((*(*ptr).vtbl).freeBlock)(
                    ptr,
                    queueId,
                    blockID,
                    sendToController,
                )
            }
        }
        impl IDataExchangeHandler {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IDataExchangeHandlerVtbl
            where
                C: IDataExchangeHandlerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn openQueue<C, W, const OFFSET: isize>(
                    this: *mut IDataExchangeHandler,
                    processor: *mut IAudioProcessor,
                    blockSize: uint32,
                    numBlocks: uint32,
                    alignment: uint32,
                    userContextID: DataExchangeUserContextID,
                    outID: *mut DataExchangeQueueID,
                ) -> tresult
                where
                    C: IDataExchangeHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).openQueue(
                        processor,
                        blockSize,
                        numBlocks,
                        alignment,
                        userContextID,
                        outID,
                    )
                }
                unsafe extern "system" fn closeQueue<C, W, const OFFSET: isize>(
                    this: *mut IDataExchangeHandler,
                    queueID: DataExchangeQueueID,
                ) -> tresult
                where
                    C: IDataExchangeHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).closeQueue(
                        queueID,
                    )
                }
                unsafe extern "system" fn lockBlock<C, W, const OFFSET: isize>(
                    this: *mut IDataExchangeHandler,
                    queueId: DataExchangeQueueID,
                    block: *mut DataExchangeBlock,
                ) -> tresult
                where
                    C: IDataExchangeHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).lockBlock(
                        queueId,
                        block,
                    )
                }
                unsafe extern "system" fn freeBlock<C, W, const OFFSET: isize>(
                    this: *mut IDataExchangeHandler,
                    queueId: DataExchangeQueueID,
                    blockID: DataExchangeBlockID,
                    sendToController: TBool,
                ) -> tresult
                where
                    C: IDataExchangeHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).freeBlock(
                        queueId,
                        blockID,
                        sendToController,
                    )
                }
                IDataExchangeHandlerVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    openQueue: openQueue::<C, W, OFFSET>,
                    closeQueue: closeQueue::<C, W, OFFSET>,
                    lockBlock: lockBlock::<C, W, OFFSET>,
                    freeBlock: freeBlock::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IDataExchangeHandler
        where
            C: IDataExchangeHandlerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IDataExchangeHandler {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IDataExchangeReceiver {
            pub vtbl: *const IDataExchangeReceiverVtbl,
        }
        unsafe impl Send for IDataExchangeReceiver {}
        unsafe impl Sync for IDataExchangeReceiver {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IDataExchangeReceiver {}
        impl ::com_scrape_types::Unknown for IDataExchangeReceiver {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IDataExchangeReceiver {
            type Vtbl = IDataExchangeReceiverVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IDataExchangeReceiver_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IDataExchangeReceiverVtbl {
            pub base: FUnknownVtbl,
            pub queueOpened: unsafe extern "system" fn(
                this: *mut IDataExchangeReceiver,
                userContextID: DataExchangeUserContextID,
                blockSize: uint32,
                dispatchOnBackgroundThread: *mut TBool,
            ),
            pub queueClosed: unsafe extern "system" fn(
                this: *mut IDataExchangeReceiver,
                userContextID: DataExchangeUserContextID,
            ),
            pub onDataExchangeBlocksReceived: unsafe extern "system" fn(
                this: *mut IDataExchangeReceiver,
                userContextID: DataExchangeUserContextID,
                numBlocks: uint32,
                blocks: *mut DataExchangeBlock,
                onBackgroundThread: TBool,
            ),
        }
        pub trait IDataExchangeReceiverTrait {
            unsafe fn queueOpened(
                &self,
                userContextID: DataExchangeUserContextID,
                blockSize: uint32,
                dispatchOnBackgroundThread: *mut TBool,
            );
            unsafe fn queueClosed(
                &self,
                userContextID: DataExchangeUserContextID,
            );
            unsafe fn onDataExchangeBlocksReceived(
                &self,
                userContextID: DataExchangeUserContextID,
                numBlocks: uint32,
                blocks: *mut DataExchangeBlock,
                onBackgroundThread: TBool,
            );
        }
        impl<P> IDataExchangeReceiverTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IDataExchangeReceiver>,
        {
            #[inline]
            unsafe fn queueOpened(
                &self,
                userContextID: DataExchangeUserContextID,
                blockSize: uint32,
                dispatchOnBackgroundThread: *mut TBool,
            ) {
                let ptr = self.ptr() as *mut IDataExchangeReceiver;
                ((*(*ptr).vtbl).queueOpened)(
                    ptr,
                    userContextID,
                    blockSize,
                    dispatchOnBackgroundThread,
                )
            }
            #[inline]
            unsafe fn queueClosed(
                &self,
                userContextID: DataExchangeUserContextID,
            ) {
                let ptr = self.ptr() as *mut IDataExchangeReceiver;
                ((*(*ptr).vtbl).queueClosed)(
                    ptr,
                    userContextID,
                )
            }
            #[inline]
            unsafe fn onDataExchangeBlocksReceived(
                &self,
                userContextID: DataExchangeUserContextID,
                numBlocks: uint32,
                blocks: *mut DataExchangeBlock,
                onBackgroundThread: TBool,
            ) {
                let ptr = self.ptr() as *mut IDataExchangeReceiver;
                ((*(*ptr).vtbl).onDataExchangeBlocksReceived)(
                    ptr,
                    userContextID,
                    numBlocks,
                    blocks,
                    onBackgroundThread,
                )
            }
        }
        impl IDataExchangeReceiver {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IDataExchangeReceiverVtbl
            where
                C: IDataExchangeReceiverTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn queueOpened<C, W, const OFFSET: isize>(
                    this: *mut IDataExchangeReceiver,
                    userContextID: DataExchangeUserContextID,
                    blockSize: uint32,
                    dispatchOnBackgroundThread: *mut TBool,
                )
                where
                    C: IDataExchangeReceiverTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).queueOpened(
                        userContextID,
                        blockSize,
                        dispatchOnBackgroundThread,
                    )
                }
                unsafe extern "system" fn queueClosed<C, W, const OFFSET: isize>(
                    this: *mut IDataExchangeReceiver,
                    userContextID: DataExchangeUserContextID,
                )
                where
                    C: IDataExchangeReceiverTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).queueClosed(
                        userContextID,
                    )
                }
                unsafe extern "system" fn onDataExchangeBlocksReceived<C, W, const OFFSET: isize>(
                    this: *mut IDataExchangeReceiver,
                    userContextID: DataExchangeUserContextID,
                    numBlocks: uint32,
                    blocks: *mut DataExchangeBlock,
                    onBackgroundThread: TBool,
                )
                where
                    C: IDataExchangeReceiverTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).onDataExchangeBlocksReceived(
                        userContextID,
                        numBlocks,
                        blocks,
                        onBackgroundThread,
                    )
                }
                IDataExchangeReceiverVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    queueOpened: queueOpened::<C, W, OFFSET>,
                    queueClosed: queueClosed::<C, W, OFFSET>,
                    onDataExchangeBlocksReceived: onDataExchangeBlocksReceived::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IDataExchangeReceiver
        where
            C: IDataExchangeReceiverTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IDataExchangeReceiver {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IEditController {
            pub vtbl: *const IEditControllerVtbl,
        }
        unsafe impl Send for IEditController {}
        unsafe impl Sync for IEditController {}
        unsafe impl ::com_scrape_types::Inherits<IPluginBase> for IEditController {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IEditController {}
        impl ::com_scrape_types::Unknown for IEditController {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IEditController {
            type Vtbl = IEditControllerVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IEditController_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || IPluginBase::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IEditControllerVtbl {
            pub base: IPluginBaseVtbl,
            pub setComponentState: unsafe extern "system" fn(
                this: *mut IEditController,
                state: *mut IBStream,
            ) -> tresult,
            pub setState: unsafe extern "system" fn(
                this: *mut IEditController,
                state: *mut IBStream,
            ) -> tresult,
            pub getState: unsafe extern "system" fn(
                this: *mut IEditController,
                state: *mut IBStream,
            ) -> tresult,
            pub getParameterCount: unsafe extern "system" fn(
                this: *mut IEditController,
            ) -> int32,
            pub getParameterInfo: unsafe extern "system" fn(
                this: *mut IEditController,
                paramIndex: int32,
                info: *mut ParameterInfo,
            ) -> tresult,
            pub getParamStringByValue: unsafe extern "system" fn(
                this: *mut IEditController,
                id: ParamID,
                valueNormalized: ParamValue,
                string: *mut String128,
            ) -> tresult,
            pub getParamValueByString: unsafe extern "system" fn(
                this: *mut IEditController,
                id: ParamID,
                string: *mut TChar,
                valueNormalized: *mut ParamValue,
            ) -> tresult,
            pub normalizedParamToPlain: unsafe extern "system" fn(
                this: *mut IEditController,
                id: ParamID,
                valueNormalized: ParamValue,
            ) -> ParamValue,
            pub plainParamToNormalized: unsafe extern "system" fn(
                this: *mut IEditController,
                id: ParamID,
                plainValue: ParamValue,
            ) -> ParamValue,
            pub getParamNormalized: unsafe extern "system" fn(
                this: *mut IEditController,
                id: ParamID,
            ) -> ParamValue,
            pub setParamNormalized: unsafe extern "system" fn(
                this: *mut IEditController,
                id: ParamID,
                value: ParamValue,
            ) -> tresult,
            pub setComponentHandler: unsafe extern "system" fn(
                this: *mut IEditController,
                handler: *mut IComponentHandler,
            ) -> tresult,
            pub createView: unsafe extern "system" fn(
                this: *mut IEditController,
                name: FIDString,
            ) -> *mut IPlugView,
        }
        pub trait IEditControllerTrait: IPluginBaseTrait {
            unsafe fn setComponentState(
                &self,
                state: *mut IBStream,
            ) -> tresult;
            unsafe fn setState(
                &self,
                state: *mut IBStream,
            ) -> tresult;
            unsafe fn getState(
                &self,
                state: *mut IBStream,
            ) -> tresult;
            unsafe fn getParameterCount(
                &self,
            ) -> int32;
            unsafe fn getParameterInfo(
                &self,
                paramIndex: int32,
                info: *mut ParameterInfo,
            ) -> tresult;
            unsafe fn getParamStringByValue(
                &self,
                id: ParamID,
                valueNormalized: ParamValue,
                string: *mut String128,
            ) -> tresult;
            unsafe fn getParamValueByString(
                &self,
                id: ParamID,
                string: *mut TChar,
                valueNormalized: *mut ParamValue,
            ) -> tresult;
            unsafe fn normalizedParamToPlain(
                &self,
                id: ParamID,
                valueNormalized: ParamValue,
            ) -> ParamValue;
            unsafe fn plainParamToNormalized(
                &self,
                id: ParamID,
                plainValue: ParamValue,
            ) -> ParamValue;
            unsafe fn getParamNormalized(
                &self,
                id: ParamID,
            ) -> ParamValue;
            unsafe fn setParamNormalized(
                &self,
                id: ParamID,
                value: ParamValue,
            ) -> tresult;
            unsafe fn setComponentHandler(
                &self,
                handler: *mut IComponentHandler,
            ) -> tresult;
            unsafe fn createView(
                &self,
                name: FIDString,
            ) -> *mut IPlugView;
        }
        impl<P> IEditControllerTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IEditController>,
            P::Target: ::com_scrape_types::Inherits<IPluginBase>,
        {
            #[inline]
            unsafe fn setComponentState(
                &self,
                state: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).setComponentState)(
                    ptr,
                    state,
                )
            }
            #[inline]
            unsafe fn setState(
                &self,
                state: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).setState)(
                    ptr,
                    state,
                )
            }
            #[inline]
            unsafe fn getState(
                &self,
                state: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).getState)(
                    ptr,
                    state,
                )
            }
            #[inline]
            unsafe fn getParameterCount(
                &self,
            ) -> int32 {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).getParameterCount)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn getParameterInfo(
                &self,
                paramIndex: int32,
                info: *mut ParameterInfo,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).getParameterInfo)(
                    ptr,
                    paramIndex,
                    info,
                )
            }
            #[inline]
            unsafe fn getParamStringByValue(
                &self,
                id: ParamID,
                valueNormalized: ParamValue,
                string: *mut String128,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).getParamStringByValue)(
                    ptr,
                    id,
                    valueNormalized,
                    string,
                )
            }
            #[inline]
            unsafe fn getParamValueByString(
                &self,
                id: ParamID,
                string: *mut TChar,
                valueNormalized: *mut ParamValue,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).getParamValueByString)(
                    ptr,
                    id,
                    string,
                    valueNormalized,
                )
            }
            #[inline]
            unsafe fn normalizedParamToPlain(
                &self,
                id: ParamID,
                valueNormalized: ParamValue,
            ) -> ParamValue {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).normalizedParamToPlain)(
                    ptr,
                    id,
                    valueNormalized,
                )
            }
            #[inline]
            unsafe fn plainParamToNormalized(
                &self,
                id: ParamID,
                plainValue: ParamValue,
            ) -> ParamValue {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).plainParamToNormalized)(
                    ptr,
                    id,
                    plainValue,
                )
            }
            #[inline]
            unsafe fn getParamNormalized(
                &self,
                id: ParamID,
            ) -> ParamValue {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).getParamNormalized)(
                    ptr,
                    id,
                )
            }
            #[inline]
            unsafe fn setParamNormalized(
                &self,
                id: ParamID,
                value: ParamValue,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).setParamNormalized)(
                    ptr,
                    id,
                    value,
                )
            }
            #[inline]
            unsafe fn setComponentHandler(
                &self,
                handler: *mut IComponentHandler,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).setComponentHandler)(
                    ptr,
                    handler,
                )
            }
            #[inline]
            unsafe fn createView(
                &self,
                name: FIDString,
            ) -> *mut IPlugView {
                let ptr = self.ptr() as *mut IEditController;
                ((*(*ptr).vtbl).createView)(
                    ptr,
                    name,
                )
            }
        }
        impl IEditController {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IEditControllerVtbl
            where
                C: IEditControllerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn setComponentState<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    state: *mut IBStream,
                ) -> tresult
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setComponentState(
                        state,
                    )
                }
                unsafe extern "system" fn setState<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    state: *mut IBStream,
                ) -> tresult
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setState(
                        state,
                    )
                }
                unsafe extern "system" fn getState<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    state: *mut IBStream,
                ) -> tresult
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getState(
                        state,
                    )
                }
                unsafe extern "system" fn getParameterCount<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                ) -> int32
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getParameterCount(
                    )
                }
                unsafe extern "system" fn getParameterInfo<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    paramIndex: int32,
                    info: *mut ParameterInfo,
                ) -> tresult
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getParameterInfo(
                        paramIndex,
                        info,
                    )
                }
                unsafe extern "system" fn getParamStringByValue<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    id: ParamID,
                    valueNormalized: ParamValue,
                    string: *mut String128,
                ) -> tresult
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getParamStringByValue(
                        id,
                        valueNormalized,
                        string,
                    )
                }
                unsafe extern "system" fn getParamValueByString<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    id: ParamID,
                    string: *mut TChar,
                    valueNormalized: *mut ParamValue,
                ) -> tresult
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getParamValueByString(
                        id,
                        string,
                        valueNormalized,
                    )
                }
                unsafe extern "system" fn normalizedParamToPlain<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    id: ParamID,
                    valueNormalized: ParamValue,
                ) -> ParamValue
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).normalizedParamToPlain(
                        id,
                        valueNormalized,
                    )
                }
                unsafe extern "system" fn plainParamToNormalized<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    id: ParamID,
                    plainValue: ParamValue,
                ) -> ParamValue
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).plainParamToNormalized(
                        id,
                        plainValue,
                    )
                }
                unsafe extern "system" fn getParamNormalized<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    id: ParamID,
                ) -> ParamValue
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getParamNormalized(
                        id,
                    )
                }
                unsafe extern "system" fn setParamNormalized<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    id: ParamID,
                    value: ParamValue,
                ) -> tresult
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setParamNormalized(
                        id,
                        value,
                    )
                }
                unsafe extern "system" fn setComponentHandler<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    handler: *mut IComponentHandler,
                ) -> tresult
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setComponentHandler(
                        handler,
                    )
                }
                unsafe extern "system" fn createView<C, W, const OFFSET: isize>(
                    this: *mut IEditController,
                    name: FIDString,
                ) -> *mut IPlugView
                where
                    C: IEditControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).createView(
                        name,
                    )
                }
                IEditControllerVtbl {
                    base: IPluginBase::make_vtbl::<C, W, OFFSET>(),
                    setComponentState: setComponentState::<C, W, OFFSET>,
                    setState: setState::<C, W, OFFSET>,
                    getState: getState::<C, W, OFFSET>,
                    getParameterCount: getParameterCount::<C, W, OFFSET>,
                    getParameterInfo: getParameterInfo::<C, W, OFFSET>,
                    getParamStringByValue: getParamStringByValue::<C, W, OFFSET>,
                    getParamValueByString: getParamValueByString::<C, W, OFFSET>,
                    normalizedParamToPlain: normalizedParamToPlain::<C, W, OFFSET>,
                    plainParamToNormalized: plainParamToNormalized::<C, W, OFFSET>,
                    getParamNormalized: getParamNormalized::<C, W, OFFSET>,
                    setParamNormalized: setParamNormalized::<C, W, OFFSET>,
                    setComponentHandler: setComponentHandler::<C, W, OFFSET>,
                    createView: createView::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IEditController
        where
            C: IEditControllerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IEditController {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IEditController2 {
            pub vtbl: *const IEditController2Vtbl,
        }
        unsafe impl Send for IEditController2 {}
        unsafe impl Sync for IEditController2 {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IEditController2 {}
        impl ::com_scrape_types::Unknown for IEditController2 {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IEditController2 {
            type Vtbl = IEditController2Vtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IEditController2_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IEditController2Vtbl {
            pub base: FUnknownVtbl,
            pub setKnobMode: unsafe extern "system" fn(
                this: *mut IEditController2,
                mode: KnobMode,
            ) -> tresult,
            pub openHelp: unsafe extern "system" fn(
                this: *mut IEditController2,
                onlyCheck: TBool,
            ) -> tresult,
            pub openAboutBox: unsafe extern "system" fn(
                this: *mut IEditController2,
                onlyCheck: TBool,
            ) -> tresult,
        }
        pub trait IEditController2Trait {
            unsafe fn setKnobMode(
                &self,
                mode: KnobMode,
            ) -> tresult;
            unsafe fn openHelp(
                &self,
                onlyCheck: TBool,
            ) -> tresult;
            unsafe fn openAboutBox(
                &self,
                onlyCheck: TBool,
            ) -> tresult;
        }
        impl<P> IEditController2Trait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IEditController2>,
        {
            #[inline]
            unsafe fn setKnobMode(
                &self,
                mode: KnobMode,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController2;
                ((*(*ptr).vtbl).setKnobMode)(
                    ptr,
                    mode,
                )
            }
            #[inline]
            unsafe fn openHelp(
                &self,
                onlyCheck: TBool,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController2;
                ((*(*ptr).vtbl).openHelp)(
                    ptr,
                    onlyCheck,
                )
            }
            #[inline]
            unsafe fn openAboutBox(
                &self,
                onlyCheck: TBool,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditController2;
                ((*(*ptr).vtbl).openAboutBox)(
                    ptr,
                    onlyCheck,
                )
            }
        }
        impl IEditController2 {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IEditController2Vtbl
            where
                C: IEditController2Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn setKnobMode<C, W, const OFFSET: isize>(
                    this: *mut IEditController2,
                    mode: KnobMode,
                ) -> tresult
                where
                    C: IEditController2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setKnobMode(
                        mode,
                    )
                }
                unsafe extern "system" fn openHelp<C, W, const OFFSET: isize>(
                    this: *mut IEditController2,
                    onlyCheck: TBool,
                ) -> tresult
                where
                    C: IEditController2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).openHelp(
                        onlyCheck,
                    )
                }
                unsafe extern "system" fn openAboutBox<C, W, const OFFSET: isize>(
                    this: *mut IEditController2,
                    onlyCheck: TBool,
                ) -> tresult
                where
                    C: IEditController2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).openAboutBox(
                        onlyCheck,
                    )
                }
                IEditController2Vtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    setKnobMode: setKnobMode::<C, W, OFFSET>,
                    openHelp: openHelp::<C, W, OFFSET>,
                    openAboutBox: openAboutBox::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IEditController2
        where
            C: IEditController2Trait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IEditController2 {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IEditControllerHostEditing {
            pub vtbl: *const IEditControllerHostEditingVtbl,
        }
        unsafe impl Send for IEditControllerHostEditing {}
        unsafe impl Sync for IEditControllerHostEditing {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IEditControllerHostEditing {}
        impl ::com_scrape_types::Unknown for IEditControllerHostEditing {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IEditControllerHostEditing {
            type Vtbl = IEditControllerHostEditingVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IEditControllerHostEditing_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IEditControllerHostEditingVtbl {
            pub base: FUnknownVtbl,
            pub beginEditFromHost: unsafe extern "system" fn(
                this: *mut IEditControllerHostEditing,
                paramID: ParamID,
            ) -> tresult,
            pub endEditFromHost: unsafe extern "system" fn(
                this: *mut IEditControllerHostEditing,
                paramID: ParamID,
            ) -> tresult,
        }
        pub trait IEditControllerHostEditingTrait {
            unsafe fn beginEditFromHost(
                &self,
                paramID: ParamID,
            ) -> tresult;
            unsafe fn endEditFromHost(
                &self,
                paramID: ParamID,
            ) -> tresult;
        }
        impl<P> IEditControllerHostEditingTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IEditControllerHostEditing>,
        {
            #[inline]
            unsafe fn beginEditFromHost(
                &self,
                paramID: ParamID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditControllerHostEditing;
                ((*(*ptr).vtbl).beginEditFromHost)(
                    ptr,
                    paramID,
                )
            }
            #[inline]
            unsafe fn endEditFromHost(
                &self,
                paramID: ParamID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEditControllerHostEditing;
                ((*(*ptr).vtbl).endEditFromHost)(
                    ptr,
                    paramID,
                )
            }
        }
        impl IEditControllerHostEditing {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IEditControllerHostEditingVtbl
            where
                C: IEditControllerHostEditingTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn beginEditFromHost<C, W, const OFFSET: isize>(
                    this: *mut IEditControllerHostEditing,
                    paramID: ParamID,
                ) -> tresult
                where
                    C: IEditControllerHostEditingTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).beginEditFromHost(
                        paramID,
                    )
                }
                unsafe extern "system" fn endEditFromHost<C, W, const OFFSET: isize>(
                    this: *mut IEditControllerHostEditing,
                    paramID: ParamID,
                ) -> tresult
                where
                    C: IEditControllerHostEditingTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).endEditFromHost(
                        paramID,
                    )
                }
                IEditControllerHostEditingVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    beginEditFromHost: beginEditFromHost::<C, W, OFFSET>,
                    endEditFromHost: endEditFromHost::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IEditControllerHostEditing
        where
            C: IEditControllerHostEditingTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IEditControllerHostEditing {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IEventList {
            pub vtbl: *const IEventListVtbl,
        }
        unsafe impl Send for IEventList {}
        unsafe impl Sync for IEventList {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IEventList {}
        impl ::com_scrape_types::Unknown for IEventList {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IEventList {
            type Vtbl = IEventListVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IEventList_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IEventListVtbl {
            pub base: FUnknownVtbl,
            pub getEventCount: unsafe extern "system" fn(
                this: *mut IEventList,
            ) -> int32,
            pub getEvent: unsafe extern "system" fn(
                this: *mut IEventList,
                index: int32,
                e: *mut Event,
            ) -> tresult,
            pub addEvent: unsafe extern "system" fn(
                this: *mut IEventList,
                e: *mut Event,
            ) -> tresult,
        }
        pub trait IEventListTrait {
            unsafe fn getEventCount(
                &self,
            ) -> int32;
            unsafe fn getEvent(
                &self,
                index: int32,
                e: *mut Event,
            ) -> tresult;
            unsafe fn addEvent(
                &self,
                e: *mut Event,
            ) -> tresult;
        }
        impl<P> IEventListTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IEventList>,
        {
            #[inline]
            unsafe fn getEventCount(
                &self,
            ) -> int32 {
                let ptr = self.ptr() as *mut IEventList;
                ((*(*ptr).vtbl).getEventCount)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn getEvent(
                &self,
                index: int32,
                e: *mut Event,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEventList;
                ((*(*ptr).vtbl).getEvent)(
                    ptr,
                    index,
                    e,
                )
            }
            #[inline]
            unsafe fn addEvent(
                &self,
                e: *mut Event,
            ) -> tresult {
                let ptr = self.ptr() as *mut IEventList;
                ((*(*ptr).vtbl).addEvent)(
                    ptr,
                    e,
                )
            }
        }
        impl IEventList {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IEventListVtbl
            where
                C: IEventListTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getEventCount<C, W, const OFFSET: isize>(
                    this: *mut IEventList,
                ) -> int32
                where
                    C: IEventListTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getEventCount(
                    )
                }
                unsafe extern "system" fn getEvent<C, W, const OFFSET: isize>(
                    this: *mut IEventList,
                    index: int32,
                    e: *mut Event,
                ) -> tresult
                where
                    C: IEventListTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getEvent(
                        index,
                        e,
                    )
                }
                unsafe extern "system" fn addEvent<C, W, const OFFSET: isize>(
                    this: *mut IEventList,
                    e: *mut Event,
                ) -> tresult
                where
                    C: IEventListTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).addEvent(
                        e,
                    )
                }
                IEventListVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getEventCount: getEventCount::<C, W, OFFSET>,
                    getEvent: getEvent::<C, W, OFFSET>,
                    addEvent: addEvent::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IEventList
        where
            C: IEventListTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IEventList {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IHostApplication {
            pub vtbl: *const IHostApplicationVtbl,
        }
        unsafe impl Send for IHostApplication {}
        unsafe impl Sync for IHostApplication {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IHostApplication {}
        impl ::com_scrape_types::Unknown for IHostApplication {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IHostApplication {
            type Vtbl = IHostApplicationVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IHostApplication_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IHostApplicationVtbl {
            pub base: FUnknownVtbl,
            pub getName: unsafe extern "system" fn(
                this: *mut IHostApplication,
                name: *mut String128,
            ) -> tresult,
            pub createInstance: unsafe extern "system" fn(
                this: *mut IHostApplication,
                cid: *mut TUID,
                _iid: *mut TUID,
                obj: *mut *mut ::std::ffi::c_void,
            ) -> tresult,
        }
        pub trait IHostApplicationTrait {
            unsafe fn getName(
                &self,
                name: *mut String128,
            ) -> tresult;
            unsafe fn createInstance(
                &self,
                cid: *mut TUID,
                _iid: *mut TUID,
                obj: *mut *mut ::std::ffi::c_void,
            ) -> tresult;
        }
        impl<P> IHostApplicationTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IHostApplication>,
        {
            #[inline]
            unsafe fn getName(
                &self,
                name: *mut String128,
            ) -> tresult {
                let ptr = self.ptr() as *mut IHostApplication;
                ((*(*ptr).vtbl).getName)(
                    ptr,
                    name,
                )
            }
            #[inline]
            unsafe fn createInstance(
                &self,
                cid: *mut TUID,
                _iid: *mut TUID,
                obj: *mut *mut ::std::ffi::c_void,
            ) -> tresult {
                let ptr = self.ptr() as *mut IHostApplication;
                ((*(*ptr).vtbl).createInstance)(
                    ptr,
                    cid,
                    _iid,
                    obj,
                )
            }
        }
        impl IHostApplication {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IHostApplicationVtbl
            where
                C: IHostApplicationTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getName<C, W, const OFFSET: isize>(
                    this: *mut IHostApplication,
                    name: *mut String128,
                ) -> tresult
                where
                    C: IHostApplicationTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getName(
                        name,
                    )
                }
                unsafe extern "system" fn createInstance<C, W, const OFFSET: isize>(
                    this: *mut IHostApplication,
                    cid: *mut TUID,
                    _iid: *mut TUID,
                    obj: *mut *mut ::std::ffi::c_void,
                ) -> tresult
                where
                    C: IHostApplicationTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).createInstance(
                        cid,
                        _iid,
                        obj,
                    )
                }
                IHostApplicationVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getName: getName::<C, W, OFFSET>,
                    createInstance: createInstance::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IHostApplication
        where
            C: IHostApplicationTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IHostApplication {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IInterAppAudioConnectionNotification {
            pub vtbl: *const IInterAppAudioConnectionNotificationVtbl,
        }
        unsafe impl Send for IInterAppAudioConnectionNotification {}
        unsafe impl Sync for IInterAppAudioConnectionNotification {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IInterAppAudioConnectionNotification {}
        impl ::com_scrape_types::Unknown for IInterAppAudioConnectionNotification {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IInterAppAudioConnectionNotification {
            type Vtbl = IInterAppAudioConnectionNotificationVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IInterAppAudioConnectionNotification_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IInterAppAudioConnectionNotificationVtbl {
            pub base: FUnknownVtbl,
            pub onInterAppAudioConnectionStateChange: unsafe extern "system" fn(
                this: *mut IInterAppAudioConnectionNotification,
                newState: TBool,
            ),
        }
        pub trait IInterAppAudioConnectionNotificationTrait {
            unsafe fn onInterAppAudioConnectionStateChange(
                &self,
                newState: TBool,
            );
        }
        impl<P> IInterAppAudioConnectionNotificationTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IInterAppAudioConnectionNotification>,
        {
            #[inline]
            unsafe fn onInterAppAudioConnectionStateChange(
                &self,
                newState: TBool,
            ) {
                let ptr = self.ptr() as *mut IInterAppAudioConnectionNotification;
                ((*(*ptr).vtbl).onInterAppAudioConnectionStateChange)(
                    ptr,
                    newState,
                )
            }
        }
        impl IInterAppAudioConnectionNotification {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IInterAppAudioConnectionNotificationVtbl
            where
                C: IInterAppAudioConnectionNotificationTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn onInterAppAudioConnectionStateChange<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioConnectionNotification,
                    newState: TBool,
                )
                where
                    C: IInterAppAudioConnectionNotificationTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).onInterAppAudioConnectionStateChange(
                        newState,
                    )
                }
                IInterAppAudioConnectionNotificationVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    onInterAppAudioConnectionStateChange: onInterAppAudioConnectionStateChange::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IInterAppAudioConnectionNotification
        where
            C: IInterAppAudioConnectionNotificationTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IInterAppAudioConnectionNotification {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IInterAppAudioHost {
            pub vtbl: *const IInterAppAudioHostVtbl,
        }
        unsafe impl Send for IInterAppAudioHost {}
        unsafe impl Sync for IInterAppAudioHost {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IInterAppAudioHost {}
        impl ::com_scrape_types::Unknown for IInterAppAudioHost {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IInterAppAudioHost {
            type Vtbl = IInterAppAudioHostVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IInterAppAudioHost_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IInterAppAudioHostVtbl {
            pub base: FUnknownVtbl,
            pub getScreenSize: unsafe extern "system" fn(
                this: *mut IInterAppAudioHost,
                size: *mut ViewRect,
                scale: *mut f32,
            ) -> tresult,
            pub connectedToHost: unsafe extern "system" fn(
                this: *mut IInterAppAudioHost,
            ) -> tresult,
            pub switchToHost: unsafe extern "system" fn(
                this: *mut IInterAppAudioHost,
            ) -> tresult,
            pub sendRemoteControlEvent: unsafe extern "system" fn(
                this: *mut IInterAppAudioHost,
                event: uint32,
            ) -> tresult,
            pub getHostIcon: unsafe extern "system" fn(
                this: *mut IInterAppAudioHost,
                icon: *mut *mut ::std::ffi::c_void,
            ) -> tresult,
            pub scheduleEventFromUI: unsafe extern "system" fn(
                this: *mut IInterAppAudioHost,
                event: *mut Event,
            ) -> tresult,
            pub createPresetManager: unsafe extern "system" fn(
                this: *mut IInterAppAudioHost,
                cid: *const TUID,
            ) -> *mut IInterAppAudioPresetManager,
            pub showSettingsView: unsafe extern "system" fn(
                this: *mut IInterAppAudioHost,
            ) -> tresult,
        }
        pub trait IInterAppAudioHostTrait {
            unsafe fn getScreenSize(
                &self,
                size: *mut ViewRect,
                scale: *mut f32,
            ) -> tresult;
            unsafe fn connectedToHost(
                &self,
            ) -> tresult;
            unsafe fn switchToHost(
                &self,
            ) -> tresult;
            unsafe fn sendRemoteControlEvent(
                &self,
                event: uint32,
            ) -> tresult;
            unsafe fn getHostIcon(
                &self,
                icon: *mut *mut ::std::ffi::c_void,
            ) -> tresult;
            unsafe fn scheduleEventFromUI(
                &self,
                event: *mut Event,
            ) -> tresult;
            unsafe fn createPresetManager(
                &self,
                cid: *const TUID,
            ) -> *mut IInterAppAudioPresetManager;
            unsafe fn showSettingsView(
                &self,
            ) -> tresult;
        }
        impl<P> IInterAppAudioHostTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IInterAppAudioHost>,
        {
            #[inline]
            unsafe fn getScreenSize(
                &self,
                size: *mut ViewRect,
                scale: *mut f32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioHost;
                ((*(*ptr).vtbl).getScreenSize)(
                    ptr,
                    size,
                    scale,
                )
            }
            #[inline]
            unsafe fn connectedToHost(
                &self,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioHost;
                ((*(*ptr).vtbl).connectedToHost)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn switchToHost(
                &self,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioHost;
                ((*(*ptr).vtbl).switchToHost)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn sendRemoteControlEvent(
                &self,
                event: uint32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioHost;
                ((*(*ptr).vtbl).sendRemoteControlEvent)(
                    ptr,
                    event,
                )
            }
            #[inline]
            unsafe fn getHostIcon(
                &self,
                icon: *mut *mut ::std::ffi::c_void,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioHost;
                ((*(*ptr).vtbl).getHostIcon)(
                    ptr,
                    icon,
                )
            }
            #[inline]
            unsafe fn scheduleEventFromUI(
                &self,
                event: *mut Event,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioHost;
                ((*(*ptr).vtbl).scheduleEventFromUI)(
                    ptr,
                    event,
                )
            }
            #[inline]
            unsafe fn createPresetManager(
                &self,
                cid: *const TUID,
            ) -> *mut IInterAppAudioPresetManager {
                let ptr = self.ptr() as *mut IInterAppAudioHost;
                ((*(*ptr).vtbl).createPresetManager)(
                    ptr,
                    cid,
                )
            }
            #[inline]
            unsafe fn showSettingsView(
                &self,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioHost;
                ((*(*ptr).vtbl).showSettingsView)(
                    ptr,
                )
            }
        }
        impl IInterAppAudioHost {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IInterAppAudioHostVtbl
            where
                C: IInterAppAudioHostTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getScreenSize<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioHost,
                    size: *mut ViewRect,
                    scale: *mut f32,
                ) -> tresult
                where
                    C: IInterAppAudioHostTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getScreenSize(
                        size,
                        scale,
                    )
                }
                unsafe extern "system" fn connectedToHost<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioHost,
                ) -> tresult
                where
                    C: IInterAppAudioHostTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).connectedToHost(
                    )
                }
                unsafe extern "system" fn switchToHost<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioHost,
                ) -> tresult
                where
                    C: IInterAppAudioHostTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).switchToHost(
                    )
                }
                unsafe extern "system" fn sendRemoteControlEvent<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioHost,
                    event: uint32,
                ) -> tresult
                where
                    C: IInterAppAudioHostTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).sendRemoteControlEvent(
                        event,
                    )
                }
                unsafe extern "system" fn getHostIcon<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioHost,
                    icon: *mut *mut ::std::ffi::c_void,
                ) -> tresult
                where
                    C: IInterAppAudioHostTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getHostIcon(
                        icon,
                    )
                }
                unsafe extern "system" fn scheduleEventFromUI<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioHost,
                    event: *mut Event,
                ) -> tresult
                where
                    C: IInterAppAudioHostTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).scheduleEventFromUI(
                        event,
                    )
                }
                unsafe extern "system" fn createPresetManager<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioHost,
                    cid: *const TUID,
                ) -> *mut IInterAppAudioPresetManager
                where
                    C: IInterAppAudioHostTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).createPresetManager(
                        cid,
                    )
                }
                unsafe extern "system" fn showSettingsView<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioHost,
                ) -> tresult
                where
                    C: IInterAppAudioHostTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).showSettingsView(
                    )
                }
                IInterAppAudioHostVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getScreenSize: getScreenSize::<C, W, OFFSET>,
                    connectedToHost: connectedToHost::<C, W, OFFSET>,
                    switchToHost: switchToHost::<C, W, OFFSET>,
                    sendRemoteControlEvent: sendRemoteControlEvent::<C, W, OFFSET>,
                    getHostIcon: getHostIcon::<C, W, OFFSET>,
                    scheduleEventFromUI: scheduleEventFromUI::<C, W, OFFSET>,
                    createPresetManager: createPresetManager::<C, W, OFFSET>,
                    showSettingsView: showSettingsView::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IInterAppAudioHost
        where
            C: IInterAppAudioHostTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IInterAppAudioHost {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IInterAppAudioPresetManager {
            pub vtbl: *const IInterAppAudioPresetManagerVtbl,
        }
        unsafe impl Send for IInterAppAudioPresetManager {}
        unsafe impl Sync for IInterAppAudioPresetManager {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IInterAppAudioPresetManager {}
        impl ::com_scrape_types::Unknown for IInterAppAudioPresetManager {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IInterAppAudioPresetManager {
            type Vtbl = IInterAppAudioPresetManagerVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IInterAppAudioPresetManager_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IInterAppAudioPresetManagerVtbl {
            pub base: FUnknownVtbl,
            pub runLoadPresetBrowser: unsafe extern "system" fn(
                this: *mut IInterAppAudioPresetManager,
            ) -> tresult,
            pub runSavePresetBrowser: unsafe extern "system" fn(
                this: *mut IInterAppAudioPresetManager,
            ) -> tresult,
            pub loadNextPreset: unsafe extern "system" fn(
                this: *mut IInterAppAudioPresetManager,
            ) -> tresult,
            pub loadPreviousPreset: unsafe extern "system" fn(
                this: *mut IInterAppAudioPresetManager,
            ) -> tresult,
        }
        pub trait IInterAppAudioPresetManagerTrait {
            unsafe fn runLoadPresetBrowser(
                &self,
            ) -> tresult;
            unsafe fn runSavePresetBrowser(
                &self,
            ) -> tresult;
            unsafe fn loadNextPreset(
                &self,
            ) -> tresult;
            unsafe fn loadPreviousPreset(
                &self,
            ) -> tresult;
        }
        impl<P> IInterAppAudioPresetManagerTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IInterAppAudioPresetManager>,
        {
            #[inline]
            unsafe fn runLoadPresetBrowser(
                &self,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioPresetManager;
                ((*(*ptr).vtbl).runLoadPresetBrowser)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn runSavePresetBrowser(
                &self,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioPresetManager;
                ((*(*ptr).vtbl).runSavePresetBrowser)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn loadNextPreset(
                &self,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioPresetManager;
                ((*(*ptr).vtbl).loadNextPreset)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn loadPreviousPreset(
                &self,
            ) -> tresult {
                let ptr = self.ptr() as *mut IInterAppAudioPresetManager;
                ((*(*ptr).vtbl).loadPreviousPreset)(
                    ptr,
                )
            }
        }
        impl IInterAppAudioPresetManager {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IInterAppAudioPresetManagerVtbl
            where
                C: IInterAppAudioPresetManagerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn runLoadPresetBrowser<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioPresetManager,
                ) -> tresult
                where
                    C: IInterAppAudioPresetManagerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).runLoadPresetBrowser(
                    )
                }
                unsafe extern "system" fn runSavePresetBrowser<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioPresetManager,
                ) -> tresult
                where
                    C: IInterAppAudioPresetManagerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).runSavePresetBrowser(
                    )
                }
                unsafe extern "system" fn loadNextPreset<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioPresetManager,
                ) -> tresult
                where
                    C: IInterAppAudioPresetManagerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).loadNextPreset(
                    )
                }
                unsafe extern "system" fn loadPreviousPreset<C, W, const OFFSET: isize>(
                    this: *mut IInterAppAudioPresetManager,
                ) -> tresult
                where
                    C: IInterAppAudioPresetManagerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).loadPreviousPreset(
                    )
                }
                IInterAppAudioPresetManagerVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    runLoadPresetBrowser: runLoadPresetBrowser::<C, W, OFFSET>,
                    runSavePresetBrowser: runSavePresetBrowser::<C, W, OFFSET>,
                    loadNextPreset: loadNextPreset::<C, W, OFFSET>,
                    loadPreviousPreset: loadPreviousPreset::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IInterAppAudioPresetManager
        where
            C: IInterAppAudioPresetManagerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IInterAppAudioPresetManager {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IKeyswitchController {
            pub vtbl: *const IKeyswitchControllerVtbl,
        }
        unsafe impl Send for IKeyswitchController {}
        unsafe impl Sync for IKeyswitchController {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IKeyswitchController {}
        impl ::com_scrape_types::Unknown for IKeyswitchController {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IKeyswitchController {
            type Vtbl = IKeyswitchControllerVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IKeyswitchController_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IKeyswitchControllerVtbl {
            pub base: FUnknownVtbl,
            pub getKeyswitchCount: unsafe extern "system" fn(
                this: *mut IKeyswitchController,
                busIndex: int32,
                channel: int16,
            ) -> int32,
            pub getKeyswitchInfo: unsafe extern "system" fn(
                this: *mut IKeyswitchController,
                busIndex: int32,
                channel: int16,
                keySwitchIndex: int32,
                info: *mut KeyswitchInfo,
            ) -> tresult,
        }
        pub trait IKeyswitchControllerTrait {
            unsafe fn getKeyswitchCount(
                &self,
                busIndex: int32,
                channel: int16,
            ) -> int32;
            unsafe fn getKeyswitchInfo(
                &self,
                busIndex: int32,
                channel: int16,
                keySwitchIndex: int32,
                info: *mut KeyswitchInfo,
            ) -> tresult;
        }
        impl<P> IKeyswitchControllerTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IKeyswitchController>,
        {
            #[inline]
            unsafe fn getKeyswitchCount(
                &self,
                busIndex: int32,
                channel: int16,
            ) -> int32 {
                let ptr = self.ptr() as *mut IKeyswitchController;
                ((*(*ptr).vtbl).getKeyswitchCount)(
                    ptr,
                    busIndex,
                    channel,
                )
            }
            #[inline]
            unsafe fn getKeyswitchInfo(
                &self,
                busIndex: int32,
                channel: int16,
                keySwitchIndex: int32,
                info: *mut KeyswitchInfo,
            ) -> tresult {
                let ptr = self.ptr() as *mut IKeyswitchController;
                ((*(*ptr).vtbl).getKeyswitchInfo)(
                    ptr,
                    busIndex,
                    channel,
                    keySwitchIndex,
                    info,
                )
            }
        }
        impl IKeyswitchController {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IKeyswitchControllerVtbl
            where
                C: IKeyswitchControllerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getKeyswitchCount<C, W, const OFFSET: isize>(
                    this: *mut IKeyswitchController,
                    busIndex: int32,
                    channel: int16,
                ) -> int32
                where
                    C: IKeyswitchControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getKeyswitchCount(
                        busIndex,
                        channel,
                    )
                }
                unsafe extern "system" fn getKeyswitchInfo<C, W, const OFFSET: isize>(
                    this: *mut IKeyswitchController,
                    busIndex: int32,
                    channel: int16,
                    keySwitchIndex: int32,
                    info: *mut KeyswitchInfo,
                ) -> tresult
                where
                    C: IKeyswitchControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getKeyswitchInfo(
                        busIndex,
                        channel,
                        keySwitchIndex,
                        info,
                    )
                }
                IKeyswitchControllerVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getKeyswitchCount: getKeyswitchCount::<C, W, OFFSET>,
                    getKeyswitchInfo: getKeyswitchInfo::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IKeyswitchController
        where
            C: IKeyswitchControllerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IKeyswitchController {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IMessage {
            pub vtbl: *const IMessageVtbl,
        }
        unsafe impl Send for IMessage {}
        unsafe impl Sync for IMessage {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IMessage {}
        impl ::com_scrape_types::Unknown for IMessage {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IMessage {
            type Vtbl = IMessageVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IMessage_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IMessageVtbl {
            pub base: FUnknownVtbl,
            pub getMessageID: unsafe extern "system" fn(
                this: *mut IMessage,
            ) -> FIDString,
            pub setMessageID: unsafe extern "system" fn(
                this: *mut IMessage,
                id: FIDString,
            ),
            pub getAttributes: unsafe extern "system" fn(
                this: *mut IMessage,
            ) -> *mut IAttributeList,
        }
        pub trait IMessageTrait {
            unsafe fn getMessageID(
                &self,
            ) -> FIDString;
            unsafe fn setMessageID(
                &self,
                id: FIDString,
            );
            unsafe fn getAttributes(
                &self,
            ) -> *mut IAttributeList;
        }
        impl<P> IMessageTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IMessage>,
        {
            #[inline]
            unsafe fn getMessageID(
                &self,
            ) -> FIDString {
                let ptr = self.ptr() as *mut IMessage;
                ((*(*ptr).vtbl).getMessageID)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn setMessageID(
                &self,
                id: FIDString,
            ) {
                let ptr = self.ptr() as *mut IMessage;
                ((*(*ptr).vtbl).setMessageID)(
                    ptr,
                    id,
                )
            }
            #[inline]
            unsafe fn getAttributes(
                &self,
            ) -> *mut IAttributeList {
                let ptr = self.ptr() as *mut IMessage;
                ((*(*ptr).vtbl).getAttributes)(
                    ptr,
                )
            }
        }
        impl IMessage {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IMessageVtbl
            where
                C: IMessageTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getMessageID<C, W, const OFFSET: isize>(
                    this: *mut IMessage,
                ) -> FIDString
                where
                    C: IMessageTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getMessageID(
                    )
                }
                unsafe extern "system" fn setMessageID<C, W, const OFFSET: isize>(
                    this: *mut IMessage,
                    id: FIDString,
                )
                where
                    C: IMessageTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setMessageID(
                        id,
                    )
                }
                unsafe extern "system" fn getAttributes<C, W, const OFFSET: isize>(
                    this: *mut IMessage,
                ) -> *mut IAttributeList
                where
                    C: IMessageTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getAttributes(
                    )
                }
                IMessageVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getMessageID: getMessageID::<C, W, OFFSET>,
                    setMessageID: setMessageID::<C, W, OFFSET>,
                    getAttributes: getAttributes::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IMessage
        where
            C: IMessageTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IMessage {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IMidiLearn {
            pub vtbl: *const IMidiLearnVtbl,
        }
        unsafe impl Send for IMidiLearn {}
        unsafe impl Sync for IMidiLearn {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IMidiLearn {}
        impl ::com_scrape_types::Unknown for IMidiLearn {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IMidiLearn {
            type Vtbl = IMidiLearnVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IMidiLearn_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IMidiLearnVtbl {
            pub base: FUnknownVtbl,
            pub onLiveMIDIControllerInput: unsafe extern "system" fn(
                this: *mut IMidiLearn,
                busIndex: int32,
                channel: int16,
                midiCC: CtrlNumber,
            ) -> tresult,
        }
        pub trait IMidiLearnTrait {
            unsafe fn onLiveMIDIControllerInput(
                &self,
                busIndex: int32,
                channel: int16,
                midiCC: CtrlNumber,
            ) -> tresult;
        }
        impl<P> IMidiLearnTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IMidiLearn>,
        {
            #[inline]
            unsafe fn onLiveMIDIControllerInput(
                &self,
                busIndex: int32,
                channel: int16,
                midiCC: CtrlNumber,
            ) -> tresult {
                let ptr = self.ptr() as *mut IMidiLearn;
                ((*(*ptr).vtbl).onLiveMIDIControllerInput)(
                    ptr,
                    busIndex,
                    channel,
                    midiCC,
                )
            }
        }
        impl IMidiLearn {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IMidiLearnVtbl
            where
                C: IMidiLearnTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn onLiveMIDIControllerInput<C, W, const OFFSET: isize>(
                    this: *mut IMidiLearn,
                    busIndex: int32,
                    channel: int16,
                    midiCC: CtrlNumber,
                ) -> tresult
                where
                    C: IMidiLearnTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).onLiveMIDIControllerInput(
                        busIndex,
                        channel,
                        midiCC,
                    )
                }
                IMidiLearnVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    onLiveMIDIControllerInput: onLiveMIDIControllerInput::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IMidiLearn
        where
            C: IMidiLearnTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IMidiLearn {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IMidiLearn2 {
            pub vtbl: *const IMidiLearn2Vtbl,
        }
        unsafe impl Send for IMidiLearn2 {}
        unsafe impl Sync for IMidiLearn2 {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IMidiLearn2 {}
        impl ::com_scrape_types::Unknown for IMidiLearn2 {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IMidiLearn2 {
            type Vtbl = IMidiLearn2Vtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IMidiLearn2_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IMidiLearn2Vtbl {
            pub base: FUnknownVtbl,
            pub onLiveMidi2ControllerInput: unsafe extern "system" fn(
                this: *mut IMidiLearn2,
                index: BusIndex,
                channel: MidiChannel,
                midiCC: Midi2Controller,
            ) -> tresult,
            pub onLiveMidi1ControllerInput: unsafe extern "system" fn(
                this: *mut IMidiLearn2,
                index: BusIndex,
                channel: MidiChannel,
                midiCC: CtrlNumber,
            ) -> tresult,
        }
        pub trait IMidiLearn2Trait {
            unsafe fn onLiveMidi2ControllerInput(
                &self,
                index: BusIndex,
                channel: MidiChannel,
                midiCC: Midi2Controller,
            ) -> tresult;
            unsafe fn onLiveMidi1ControllerInput(
                &self,
                index: BusIndex,
                channel: MidiChannel,
                midiCC: CtrlNumber,
            ) -> tresult;
        }
        impl<P> IMidiLearn2Trait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IMidiLearn2>,
        {
            #[inline]
            unsafe fn onLiveMidi2ControllerInput(
                &self,
                index: BusIndex,
                channel: MidiChannel,
                midiCC: Midi2Controller,
            ) -> tresult {
                let ptr = self.ptr() as *mut IMidiLearn2;
                ((*(*ptr).vtbl).onLiveMidi2ControllerInput)(
                    ptr,
                    index,
                    channel,
                    midiCC,
                )
            }
            #[inline]
            unsafe fn onLiveMidi1ControllerInput(
                &self,
                index: BusIndex,
                channel: MidiChannel,
                midiCC: CtrlNumber,
            ) -> tresult {
                let ptr = self.ptr() as *mut IMidiLearn2;
                ((*(*ptr).vtbl).onLiveMidi1ControllerInput)(
                    ptr,
                    index,
                    channel,
                    midiCC,
                )
            }
        }
        impl IMidiLearn2 {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IMidiLearn2Vtbl
            where
                C: IMidiLearn2Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn onLiveMidi2ControllerInput<C, W, const OFFSET: isize>(
                    this: *mut IMidiLearn2,
                    index: BusIndex,
                    channel: MidiChannel,
                    midiCC: Midi2Controller,
                ) -> tresult
                where
                    C: IMidiLearn2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).onLiveMidi2ControllerInput(
                        index,
                        channel,
                        midiCC,
                    )
                }
                unsafe extern "system" fn onLiveMidi1ControllerInput<C, W, const OFFSET: isize>(
                    this: *mut IMidiLearn2,
                    index: BusIndex,
                    channel: MidiChannel,
                    midiCC: CtrlNumber,
                ) -> tresult
                where
                    C: IMidiLearn2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).onLiveMidi1ControllerInput(
                        index,
                        channel,
                        midiCC,
                    )
                }
                IMidiLearn2Vtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    onLiveMidi2ControllerInput: onLiveMidi2ControllerInput::<C, W, OFFSET>,
                    onLiveMidi1ControllerInput: onLiveMidi1ControllerInput::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IMidiLearn2
        where
            C: IMidiLearn2Trait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IMidiLearn2 {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IMidiMapping {
            pub vtbl: *const IMidiMappingVtbl,
        }
        unsafe impl Send for IMidiMapping {}
        unsafe impl Sync for IMidiMapping {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IMidiMapping {}
        impl ::com_scrape_types::Unknown for IMidiMapping {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IMidiMapping {
            type Vtbl = IMidiMappingVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IMidiMapping_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IMidiMappingVtbl {
            pub base: FUnknownVtbl,
            pub getMidiControllerAssignment: unsafe extern "system" fn(
                this: *mut IMidiMapping,
                busIndex: int32,
                channel: int16,
                midiControllerNumber: CtrlNumber,
                id: *mut ParamID,
            ) -> tresult,
        }
        pub trait IMidiMappingTrait {
            unsafe fn getMidiControllerAssignment(
                &self,
                busIndex: int32,
                channel: int16,
                midiControllerNumber: CtrlNumber,
                id: *mut ParamID,
            ) -> tresult;
        }
        impl<P> IMidiMappingTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IMidiMapping>,
        {
            #[inline]
            unsafe fn getMidiControllerAssignment(
                &self,
                busIndex: int32,
                channel: int16,
                midiControllerNumber: CtrlNumber,
                id: *mut ParamID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IMidiMapping;
                ((*(*ptr).vtbl).getMidiControllerAssignment)(
                    ptr,
                    busIndex,
                    channel,
                    midiControllerNumber,
                    id,
                )
            }
        }
        impl IMidiMapping {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IMidiMappingVtbl
            where
                C: IMidiMappingTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getMidiControllerAssignment<C, W, const OFFSET: isize>(
                    this: *mut IMidiMapping,
                    busIndex: int32,
                    channel: int16,
                    midiControllerNumber: CtrlNumber,
                    id: *mut ParamID,
                ) -> tresult
                where
                    C: IMidiMappingTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getMidiControllerAssignment(
                        busIndex,
                        channel,
                        midiControllerNumber,
                        id,
                    )
                }
                IMidiMappingVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getMidiControllerAssignment: getMidiControllerAssignment::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IMidiMapping
        where
            C: IMidiMappingTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IMidiMapping {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IMidiMapping2 {
            pub vtbl: *const IMidiMapping2Vtbl,
        }
        unsafe impl Send for IMidiMapping2 {}
        unsafe impl Sync for IMidiMapping2 {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IMidiMapping2 {}
        impl ::com_scrape_types::Unknown for IMidiMapping2 {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IMidiMapping2 {
            type Vtbl = IMidiMapping2Vtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IMidiMapping2_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IMidiMapping2Vtbl {
            pub base: FUnknownVtbl,
            pub getNumMidi2ControllerAssignments: unsafe extern "system" fn(
                this: *mut IMidiMapping2,
                direction: BusDirections,
            ) -> uint32,
            pub getMidi2ControllerAssignments: unsafe extern "system" fn(
                this: *mut IMidiMapping2,
                direction: BusDirections,
                list: *const Midi2ControllerParamIDAssignmentList,
            ) -> tresult,
            pub getNumMidi1ControllerAssignments: unsafe extern "system" fn(
                this: *mut IMidiMapping2,
                direction: BusDirections,
            ) -> uint32,
            pub getMidi1ControllerAssignments: unsafe extern "system" fn(
                this: *mut IMidiMapping2,
                direction: BusDirections,
                list: *const Midi1ControllerParamIDAssignmentList,
            ) -> tresult,
        }
        pub trait IMidiMapping2Trait {
            unsafe fn getNumMidi2ControllerAssignments(
                &self,
                direction: BusDirections,
            ) -> uint32;
            unsafe fn getMidi2ControllerAssignments(
                &self,
                direction: BusDirections,
                list: *const Midi2ControllerParamIDAssignmentList,
            ) -> tresult;
            unsafe fn getNumMidi1ControllerAssignments(
                &self,
                direction: BusDirections,
            ) -> uint32;
            unsafe fn getMidi1ControllerAssignments(
                &self,
                direction: BusDirections,
                list: *const Midi1ControllerParamIDAssignmentList,
            ) -> tresult;
        }
        impl<P> IMidiMapping2Trait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IMidiMapping2>,
        {
            #[inline]
            unsafe fn getNumMidi2ControllerAssignments(
                &self,
                direction: BusDirections,
            ) -> uint32 {
                let ptr = self.ptr() as *mut IMidiMapping2;
                ((*(*ptr).vtbl).getNumMidi2ControllerAssignments)(
                    ptr,
                    direction,
                )
            }
            #[inline]
            unsafe fn getMidi2ControllerAssignments(
                &self,
                direction: BusDirections,
                list: *const Midi2ControllerParamIDAssignmentList,
            ) -> tresult {
                let ptr = self.ptr() as *mut IMidiMapping2;
                ((*(*ptr).vtbl).getMidi2ControllerAssignments)(
                    ptr,
                    direction,
                    list,
                )
            }
            #[inline]
            unsafe fn getNumMidi1ControllerAssignments(
                &self,
                direction: BusDirections,
            ) -> uint32 {
                let ptr = self.ptr() as *mut IMidiMapping2;
                ((*(*ptr).vtbl).getNumMidi1ControllerAssignments)(
                    ptr,
                    direction,
                )
            }
            #[inline]
            unsafe fn getMidi1ControllerAssignments(
                &self,
                direction: BusDirections,
                list: *const Midi1ControllerParamIDAssignmentList,
            ) -> tresult {
                let ptr = self.ptr() as *mut IMidiMapping2;
                ((*(*ptr).vtbl).getMidi1ControllerAssignments)(
                    ptr,
                    direction,
                    list,
                )
            }
        }
        impl IMidiMapping2 {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IMidiMapping2Vtbl
            where
                C: IMidiMapping2Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getNumMidi2ControllerAssignments<C, W, const OFFSET: isize>(
                    this: *mut IMidiMapping2,
                    direction: BusDirections,
                ) -> uint32
                where
                    C: IMidiMapping2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getNumMidi2ControllerAssignments(
                        direction,
                    )
                }
                unsafe extern "system" fn getMidi2ControllerAssignments<C, W, const OFFSET: isize>(
                    this: *mut IMidiMapping2,
                    direction: BusDirections,
                    list: *const Midi2ControllerParamIDAssignmentList,
                ) -> tresult
                where
                    C: IMidiMapping2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getMidi2ControllerAssignments(
                        direction,
                        list,
                    )
                }
                unsafe extern "system" fn getNumMidi1ControllerAssignments<C, W, const OFFSET: isize>(
                    this: *mut IMidiMapping2,
                    direction: BusDirections,
                ) -> uint32
                where
                    C: IMidiMapping2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getNumMidi1ControllerAssignments(
                        direction,
                    )
                }
                unsafe extern "system" fn getMidi1ControllerAssignments<C, W, const OFFSET: isize>(
                    this: *mut IMidiMapping2,
                    direction: BusDirections,
                    list: *const Midi1ControllerParamIDAssignmentList,
                ) -> tresult
                where
                    C: IMidiMapping2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getMidi1ControllerAssignments(
                        direction,
                        list,
                    )
                }
                IMidiMapping2Vtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getNumMidi2ControllerAssignments: getNumMidi2ControllerAssignments::<C, W, OFFSET>,
                    getMidi2ControllerAssignments: getMidi2ControllerAssignments::<C, W, OFFSET>,
                    getNumMidi1ControllerAssignments: getNumMidi1ControllerAssignments::<C, W, OFFSET>,
                    getMidi1ControllerAssignments: getMidi1ControllerAssignments::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IMidiMapping2
        where
            C: IMidiMapping2Trait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IMidiMapping2 {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct INoteExpressionController {
            pub vtbl: *const INoteExpressionControllerVtbl,
        }
        unsafe impl Send for INoteExpressionController {}
        unsafe impl Sync for INoteExpressionController {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for INoteExpressionController {}
        impl ::com_scrape_types::Unknown for INoteExpressionController {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for INoteExpressionController {
            type Vtbl = INoteExpressionControllerVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(INoteExpressionController_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct INoteExpressionControllerVtbl {
            pub base: FUnknownVtbl,
            pub getNoteExpressionCount: unsafe extern "system" fn(
                this: *mut INoteExpressionController,
                busIndex: int32,
                channel: int16,
            ) -> int32,
            pub getNoteExpressionInfo: unsafe extern "system" fn(
                this: *mut INoteExpressionController,
                busIndex: int32,
                channel: int16,
                noteExpressionIndex: int32,
                info: *mut NoteExpressionTypeInfo,
            ) -> tresult,
            pub getNoteExpressionStringByValue: unsafe extern "system" fn(
                this: *mut INoteExpressionController,
                busIndex: int32,
                channel: int16,
                id: NoteExpressionTypeID,
                valueNormalized: NoteExpressionValue,
                string: *mut String128,
            ) -> tresult,
            pub getNoteExpressionValueByString: unsafe extern "system" fn(
                this: *mut INoteExpressionController,
                busIndex: int32,
                channel: int16,
                id: NoteExpressionTypeID,
                string: *const TChar,
                valueNormalized: *mut NoteExpressionValue,
            ) -> tresult,
        }
        pub trait INoteExpressionControllerTrait {
            unsafe fn getNoteExpressionCount(
                &self,
                busIndex: int32,
                channel: int16,
            ) -> int32;
            unsafe fn getNoteExpressionInfo(
                &self,
                busIndex: int32,
                channel: int16,
                noteExpressionIndex: int32,
                info: *mut NoteExpressionTypeInfo,
            ) -> tresult;
            unsafe fn getNoteExpressionStringByValue(
                &self,
                busIndex: int32,
                channel: int16,
                id: NoteExpressionTypeID,
                valueNormalized: NoteExpressionValue,
                string: *mut String128,
            ) -> tresult;
            unsafe fn getNoteExpressionValueByString(
                &self,
                busIndex: int32,
                channel: int16,
                id: NoteExpressionTypeID,
                string: *const TChar,
                valueNormalized: *mut NoteExpressionValue,
            ) -> tresult;
        }
        impl<P> INoteExpressionControllerTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<INoteExpressionController>,
        {
            #[inline]
            unsafe fn getNoteExpressionCount(
                &self,
                busIndex: int32,
                channel: int16,
            ) -> int32 {
                let ptr = self.ptr() as *mut INoteExpressionController;
                ((*(*ptr).vtbl).getNoteExpressionCount)(
                    ptr,
                    busIndex,
                    channel,
                )
            }
            #[inline]
            unsafe fn getNoteExpressionInfo(
                &self,
                busIndex: int32,
                channel: int16,
                noteExpressionIndex: int32,
                info: *mut NoteExpressionTypeInfo,
            ) -> tresult {
                let ptr = self.ptr() as *mut INoteExpressionController;
                ((*(*ptr).vtbl).getNoteExpressionInfo)(
                    ptr,
                    busIndex,
                    channel,
                    noteExpressionIndex,
                    info,
                )
            }
            #[inline]
            unsafe fn getNoteExpressionStringByValue(
                &self,
                busIndex: int32,
                channel: int16,
                id: NoteExpressionTypeID,
                valueNormalized: NoteExpressionValue,
                string: *mut String128,
            ) -> tresult {
                let ptr = self.ptr() as *mut INoteExpressionController;
                ((*(*ptr).vtbl).getNoteExpressionStringByValue)(
                    ptr,
                    busIndex,
                    channel,
                    id,
                    valueNormalized,
                    string,
                )
            }
            #[inline]
            unsafe fn getNoteExpressionValueByString(
                &self,
                busIndex: int32,
                channel: int16,
                id: NoteExpressionTypeID,
                string: *const TChar,
                valueNormalized: *mut NoteExpressionValue,
            ) -> tresult {
                let ptr = self.ptr() as *mut INoteExpressionController;
                ((*(*ptr).vtbl).getNoteExpressionValueByString)(
                    ptr,
                    busIndex,
                    channel,
                    id,
                    string,
                    valueNormalized,
                )
            }
        }
        impl INoteExpressionController {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> INoteExpressionControllerVtbl
            where
                C: INoteExpressionControllerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getNoteExpressionCount<C, W, const OFFSET: isize>(
                    this: *mut INoteExpressionController,
                    busIndex: int32,
                    channel: int16,
                ) -> int32
                where
                    C: INoteExpressionControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getNoteExpressionCount(
                        busIndex,
                        channel,
                    )
                }
                unsafe extern "system" fn getNoteExpressionInfo<C, W, const OFFSET: isize>(
                    this: *mut INoteExpressionController,
                    busIndex: int32,
                    channel: int16,
                    noteExpressionIndex: int32,
                    info: *mut NoteExpressionTypeInfo,
                ) -> tresult
                where
                    C: INoteExpressionControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getNoteExpressionInfo(
                        busIndex,
                        channel,
                        noteExpressionIndex,
                        info,
                    )
                }
                unsafe extern "system" fn getNoteExpressionStringByValue<C, W, const OFFSET: isize>(
                    this: *mut INoteExpressionController,
                    busIndex: int32,
                    channel: int16,
                    id: NoteExpressionTypeID,
                    valueNormalized: NoteExpressionValue,
                    string: *mut String128,
                ) -> tresult
                where
                    C: INoteExpressionControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getNoteExpressionStringByValue(
                        busIndex,
                        channel,
                        id,
                        valueNormalized,
                        string,
                    )
                }
                unsafe extern "system" fn getNoteExpressionValueByString<C, W, const OFFSET: isize>(
                    this: *mut INoteExpressionController,
                    busIndex: int32,
                    channel: int16,
                    id: NoteExpressionTypeID,
                    string: *const TChar,
                    valueNormalized: *mut NoteExpressionValue,
                ) -> tresult
                where
                    C: INoteExpressionControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getNoteExpressionValueByString(
                        busIndex,
                        channel,
                        id,
                        string,
                        valueNormalized,
                    )
                }
                INoteExpressionControllerVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getNoteExpressionCount: getNoteExpressionCount::<C, W, OFFSET>,
                    getNoteExpressionInfo: getNoteExpressionInfo::<C, W, OFFSET>,
                    getNoteExpressionStringByValue: getNoteExpressionStringByValue::<C, W, OFFSET>,
                    getNoteExpressionValueByString: getNoteExpressionValueByString::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for INoteExpressionController
        where
            C: INoteExpressionControllerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = INoteExpressionController {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct INoteExpressionPhysicalUIMapping {
            pub vtbl: *const INoteExpressionPhysicalUIMappingVtbl,
        }
        unsafe impl Send for INoteExpressionPhysicalUIMapping {}
        unsafe impl Sync for INoteExpressionPhysicalUIMapping {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for INoteExpressionPhysicalUIMapping {}
        impl ::com_scrape_types::Unknown for INoteExpressionPhysicalUIMapping {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for INoteExpressionPhysicalUIMapping {
            type Vtbl = INoteExpressionPhysicalUIMappingVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(INoteExpressionPhysicalUIMapping_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct INoteExpressionPhysicalUIMappingVtbl {
            pub base: FUnknownVtbl,
            pub getPhysicalUIMapping: unsafe extern "system" fn(
                this: *mut INoteExpressionPhysicalUIMapping,
                busIndex: int32,
                channel: int16,
                list: *mut PhysicalUIMapList,
            ) -> tresult,
        }
        pub trait INoteExpressionPhysicalUIMappingTrait {
            unsafe fn getPhysicalUIMapping(
                &self,
                busIndex: int32,
                channel: int16,
                list: *mut PhysicalUIMapList,
            ) -> tresult;
        }
        impl<P> INoteExpressionPhysicalUIMappingTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<INoteExpressionPhysicalUIMapping>,
        {
            #[inline]
            unsafe fn getPhysicalUIMapping(
                &self,
                busIndex: int32,
                channel: int16,
                list: *mut PhysicalUIMapList,
            ) -> tresult {
                let ptr = self.ptr() as *mut INoteExpressionPhysicalUIMapping;
                ((*(*ptr).vtbl).getPhysicalUIMapping)(
                    ptr,
                    busIndex,
                    channel,
                    list,
                )
            }
        }
        impl INoteExpressionPhysicalUIMapping {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> INoteExpressionPhysicalUIMappingVtbl
            where
                C: INoteExpressionPhysicalUIMappingTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getPhysicalUIMapping<C, W, const OFFSET: isize>(
                    this: *mut INoteExpressionPhysicalUIMapping,
                    busIndex: int32,
                    channel: int16,
                    list: *mut PhysicalUIMapList,
                ) -> tresult
                where
                    C: INoteExpressionPhysicalUIMappingTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getPhysicalUIMapping(
                        busIndex,
                        channel,
                        list,
                    )
                }
                INoteExpressionPhysicalUIMappingVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getPhysicalUIMapping: getPhysicalUIMapping::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for INoteExpressionPhysicalUIMapping
        where
            C: INoteExpressionPhysicalUIMappingTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = INoteExpressionPhysicalUIMapping {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IParamValueQueue {
            pub vtbl: *const IParamValueQueueVtbl,
        }
        unsafe impl Send for IParamValueQueue {}
        unsafe impl Sync for IParamValueQueue {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IParamValueQueue {}
        impl ::com_scrape_types::Unknown for IParamValueQueue {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IParamValueQueue {
            type Vtbl = IParamValueQueueVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IParamValueQueue_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IParamValueQueueVtbl {
            pub base: FUnknownVtbl,
            pub getParameterId: unsafe extern "system" fn(
                this: *mut IParamValueQueue,
            ) -> ParamID,
            pub getPointCount: unsafe extern "system" fn(
                this: *mut IParamValueQueue,
            ) -> int32,
            pub getPoint: unsafe extern "system" fn(
                this: *mut IParamValueQueue,
                index: int32,
                sampleOffset: *mut int32,
                value: *mut ParamValue,
            ) -> tresult,
            pub addPoint: unsafe extern "system" fn(
                this: *mut IParamValueQueue,
                sampleOffset: int32,
                value: ParamValue,
                index: *mut int32,
            ) -> tresult,
        }
        pub trait IParamValueQueueTrait {
            unsafe fn getParameterId(
                &self,
            ) -> ParamID;
            unsafe fn getPointCount(
                &self,
            ) -> int32;
            unsafe fn getPoint(
                &self,
                index: int32,
                sampleOffset: *mut int32,
                value: *mut ParamValue,
            ) -> tresult;
            unsafe fn addPoint(
                &self,
                sampleOffset: int32,
                value: ParamValue,
                index: *mut int32,
            ) -> tresult;
        }
        impl<P> IParamValueQueueTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IParamValueQueue>,
        {
            #[inline]
            unsafe fn getParameterId(
                &self,
            ) -> ParamID {
                let ptr = self.ptr() as *mut IParamValueQueue;
                ((*(*ptr).vtbl).getParameterId)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn getPointCount(
                &self,
            ) -> int32 {
                let ptr = self.ptr() as *mut IParamValueQueue;
                ((*(*ptr).vtbl).getPointCount)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn getPoint(
                &self,
                index: int32,
                sampleOffset: *mut int32,
                value: *mut ParamValue,
            ) -> tresult {
                let ptr = self.ptr() as *mut IParamValueQueue;
                ((*(*ptr).vtbl).getPoint)(
                    ptr,
                    index,
                    sampleOffset,
                    value,
                )
            }
            #[inline]
            unsafe fn addPoint(
                &self,
                sampleOffset: int32,
                value: ParamValue,
                index: *mut int32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IParamValueQueue;
                ((*(*ptr).vtbl).addPoint)(
                    ptr,
                    sampleOffset,
                    value,
                    index,
                )
            }
        }
        impl IParamValueQueue {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IParamValueQueueVtbl
            where
                C: IParamValueQueueTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getParameterId<C, W, const OFFSET: isize>(
                    this: *mut IParamValueQueue,
                ) -> ParamID
                where
                    C: IParamValueQueueTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getParameterId(
                    )
                }
                unsafe extern "system" fn getPointCount<C, W, const OFFSET: isize>(
                    this: *mut IParamValueQueue,
                ) -> int32
                where
                    C: IParamValueQueueTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getPointCount(
                    )
                }
                unsafe extern "system" fn getPoint<C, W, const OFFSET: isize>(
                    this: *mut IParamValueQueue,
                    index: int32,
                    sampleOffset: *mut int32,
                    value: *mut ParamValue,
                ) -> tresult
                where
                    C: IParamValueQueueTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getPoint(
                        index,
                        sampleOffset,
                        value,
                    )
                }
                unsafe extern "system" fn addPoint<C, W, const OFFSET: isize>(
                    this: *mut IParamValueQueue,
                    sampleOffset: int32,
                    value: ParamValue,
                    index: *mut int32,
                ) -> tresult
                where
                    C: IParamValueQueueTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).addPoint(
                        sampleOffset,
                        value,
                        index,
                    )
                }
                IParamValueQueueVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getParameterId: getParameterId::<C, W, OFFSET>,
                    getPointCount: getPointCount::<C, W, OFFSET>,
                    getPoint: getPoint::<C, W, OFFSET>,
                    addPoint: addPoint::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IParamValueQueue
        where
            C: IParamValueQueueTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IParamValueQueue {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IParameterChanges {
            pub vtbl: *const IParameterChangesVtbl,
        }
        unsafe impl Send for IParameterChanges {}
        unsafe impl Sync for IParameterChanges {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IParameterChanges {}
        impl ::com_scrape_types::Unknown for IParameterChanges {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IParameterChanges {
            type Vtbl = IParameterChangesVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IParameterChanges_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IParameterChangesVtbl {
            pub base: FUnknownVtbl,
            pub getParameterCount: unsafe extern "system" fn(
                this: *mut IParameterChanges,
            ) -> int32,
            pub getParameterData: unsafe extern "system" fn(
                this: *mut IParameterChanges,
                index: int32,
            ) -> *mut IParamValueQueue,
            pub addParameterData: unsafe extern "system" fn(
                this: *mut IParameterChanges,
                id: *const ParamID,
                index: *mut int32,
            ) -> *mut IParamValueQueue,
        }
        pub trait IParameterChangesTrait {
            unsafe fn getParameterCount(
                &self,
            ) -> int32;
            unsafe fn getParameterData(
                &self,
                index: int32,
            ) -> *mut IParamValueQueue;
            unsafe fn addParameterData(
                &self,
                id: *const ParamID,
                index: *mut int32,
            ) -> *mut IParamValueQueue;
        }
        impl<P> IParameterChangesTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IParameterChanges>,
        {
            #[inline]
            unsafe fn getParameterCount(
                &self,
            ) -> int32 {
                let ptr = self.ptr() as *mut IParameterChanges;
                ((*(*ptr).vtbl).getParameterCount)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn getParameterData(
                &self,
                index: int32,
            ) -> *mut IParamValueQueue {
                let ptr = self.ptr() as *mut IParameterChanges;
                ((*(*ptr).vtbl).getParameterData)(
                    ptr,
                    index,
                )
            }
            #[inline]
            unsafe fn addParameterData(
                &self,
                id: *const ParamID,
                index: *mut int32,
            ) -> *mut IParamValueQueue {
                let ptr = self.ptr() as *mut IParameterChanges;
                ((*(*ptr).vtbl).addParameterData)(
                    ptr,
                    id,
                    index,
                )
            }
        }
        impl IParameterChanges {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IParameterChangesVtbl
            where
                C: IParameterChangesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getParameterCount<C, W, const OFFSET: isize>(
                    this: *mut IParameterChanges,
                ) -> int32
                where
                    C: IParameterChangesTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getParameterCount(
                    )
                }
                unsafe extern "system" fn getParameterData<C, W, const OFFSET: isize>(
                    this: *mut IParameterChanges,
                    index: int32,
                ) -> *mut IParamValueQueue
                where
                    C: IParameterChangesTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getParameterData(
                        index,
                    )
                }
                unsafe extern "system" fn addParameterData<C, W, const OFFSET: isize>(
                    this: *mut IParameterChanges,
                    id: *const ParamID,
                    index: *mut int32,
                ) -> *mut IParamValueQueue
                where
                    C: IParameterChangesTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).addParameterData(
                        id,
                        index,
                    )
                }
                IParameterChangesVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getParameterCount: getParameterCount::<C, W, OFFSET>,
                    getParameterData: getParameterData::<C, W, OFFSET>,
                    addParameterData: addParameterData::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IParameterChanges
        where
            C: IParameterChangesTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IParameterChanges {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IParameterFinder {
            pub vtbl: *const IParameterFinderVtbl,
        }
        unsafe impl Send for IParameterFinder {}
        unsafe impl Sync for IParameterFinder {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IParameterFinder {}
        impl ::com_scrape_types::Unknown for IParameterFinder {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IParameterFinder {
            type Vtbl = IParameterFinderVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IParameterFinder_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IParameterFinderVtbl {
            pub base: FUnknownVtbl,
            pub findParameter: unsafe extern "system" fn(
                this: *mut IParameterFinder,
                xPos: int32,
                yPos: int32,
                resultTag: *mut ParamID,
            ) -> tresult,
        }
        pub trait IParameterFinderTrait {
            unsafe fn findParameter(
                &self,
                xPos: int32,
                yPos: int32,
                resultTag: *mut ParamID,
            ) -> tresult;
        }
        impl<P> IParameterFinderTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IParameterFinder>,
        {
            #[inline]
            unsafe fn findParameter(
                &self,
                xPos: int32,
                yPos: int32,
                resultTag: *mut ParamID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IParameterFinder;
                ((*(*ptr).vtbl).findParameter)(
                    ptr,
                    xPos,
                    yPos,
                    resultTag,
                )
            }
        }
        impl IParameterFinder {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IParameterFinderVtbl
            where
                C: IParameterFinderTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn findParameter<C, W, const OFFSET: isize>(
                    this: *mut IParameterFinder,
                    xPos: int32,
                    yPos: int32,
                    resultTag: *mut ParamID,
                ) -> tresult
                where
                    C: IParameterFinderTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).findParameter(
                        xPos,
                        yPos,
                        resultTag,
                    )
                }
                IParameterFinderVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    findParameter: findParameter::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IParameterFinder
        where
            C: IParameterFinderTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IParameterFinder {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IParameterFunctionName {
            pub vtbl: *const IParameterFunctionNameVtbl,
        }
        unsafe impl Send for IParameterFunctionName {}
        unsafe impl Sync for IParameterFunctionName {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IParameterFunctionName {}
        impl ::com_scrape_types::Unknown for IParameterFunctionName {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IParameterFunctionName {
            type Vtbl = IParameterFunctionNameVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IParameterFunctionName_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IParameterFunctionNameVtbl {
            pub base: FUnknownVtbl,
            pub getParameterIDFromFunctionName: unsafe extern "system" fn(
                this: *mut IParameterFunctionName,
                unitID: UnitID,
                functionName: FIDString,
                paramID: *mut ParamID,
            ) -> tresult,
        }
        pub trait IParameterFunctionNameTrait {
            unsafe fn getParameterIDFromFunctionName(
                &self,
                unitID: UnitID,
                functionName: FIDString,
                paramID: *mut ParamID,
            ) -> tresult;
        }
        impl<P> IParameterFunctionNameTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IParameterFunctionName>,
        {
            #[inline]
            unsafe fn getParameterIDFromFunctionName(
                &self,
                unitID: UnitID,
                functionName: FIDString,
                paramID: *mut ParamID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IParameterFunctionName;
                ((*(*ptr).vtbl).getParameterIDFromFunctionName)(
                    ptr,
                    unitID,
                    functionName,
                    paramID,
                )
            }
        }
        impl IParameterFunctionName {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IParameterFunctionNameVtbl
            where
                C: IParameterFunctionNameTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getParameterIDFromFunctionName<C, W, const OFFSET: isize>(
                    this: *mut IParameterFunctionName,
                    unitID: UnitID,
                    functionName: FIDString,
                    paramID: *mut ParamID,
                ) -> tresult
                where
                    C: IParameterFunctionNameTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getParameterIDFromFunctionName(
                        unitID,
                        functionName,
                        paramID,
                    )
                }
                IParameterFunctionNameVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getParameterIDFromFunctionName: getParameterIDFromFunctionName::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IParameterFunctionName
        where
            C: IParameterFunctionNameTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IParameterFunctionName {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IPlugInterfaceSupport {
            pub vtbl: *const IPlugInterfaceSupportVtbl,
        }
        unsafe impl Send for IPlugInterfaceSupport {}
        unsafe impl Sync for IPlugInterfaceSupport {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPlugInterfaceSupport {}
        impl ::com_scrape_types::Unknown for IPlugInterfaceSupport {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IPlugInterfaceSupport {
            type Vtbl = IPlugInterfaceSupportVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPlugInterfaceSupport_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IPlugInterfaceSupportVtbl {
            pub base: FUnknownVtbl,
            pub isPlugInterfaceSupported: unsafe extern "system" fn(
                this: *mut IPlugInterfaceSupport,
                _iid: *const TUID,
            ) -> tresult,
        }
        pub trait IPlugInterfaceSupportTrait {
            unsafe fn isPlugInterfaceSupported(
                &self,
                _iid: *const TUID,
            ) -> tresult;
        }
        impl<P> IPlugInterfaceSupportTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IPlugInterfaceSupport>,
        {
            #[inline]
            unsafe fn isPlugInterfaceSupported(
                &self,
                _iid: *const TUID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IPlugInterfaceSupport;
                ((*(*ptr).vtbl).isPlugInterfaceSupported)(
                    ptr,
                    _iid,
                )
            }
        }
        impl IPlugInterfaceSupport {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IPlugInterfaceSupportVtbl
            where
                C: IPlugInterfaceSupportTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn isPlugInterfaceSupported<C, W, const OFFSET: isize>(
                    this: *mut IPlugInterfaceSupport,
                    _iid: *const TUID,
                ) -> tresult
                where
                    C: IPlugInterfaceSupportTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).isPlugInterfaceSupported(
                        _iid,
                    )
                }
                IPlugInterfaceSupportVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    isPlugInterfaceSupported: isPlugInterfaceSupported::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPlugInterfaceSupport
        where
            C: IPlugInterfaceSupportTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IPlugInterfaceSupport {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IPrefetchableSupport {
            pub vtbl: *const IPrefetchableSupportVtbl,
        }
        unsafe impl Send for IPrefetchableSupport {}
        unsafe impl Sync for IPrefetchableSupport {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IPrefetchableSupport {}
        impl ::com_scrape_types::Unknown for IPrefetchableSupport {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IPrefetchableSupport {
            type Vtbl = IPrefetchableSupportVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IPrefetchableSupport_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IPrefetchableSupportVtbl {
            pub base: FUnknownVtbl,
            pub getPrefetchableSupport: unsafe extern "system" fn(
                this: *mut IPrefetchableSupport,
                prefetchable: *mut PrefetchableSupport,
            ) -> tresult,
        }
        pub trait IPrefetchableSupportTrait {
            unsafe fn getPrefetchableSupport(
                &self,
                prefetchable: *mut PrefetchableSupport,
            ) -> tresult;
        }
        impl<P> IPrefetchableSupportTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IPrefetchableSupport>,
        {
            #[inline]
            unsafe fn getPrefetchableSupport(
                &self,
                prefetchable: *mut PrefetchableSupport,
            ) -> tresult {
                let ptr = self.ptr() as *mut IPrefetchableSupport;
                ((*(*ptr).vtbl).getPrefetchableSupport)(
                    ptr,
                    prefetchable,
                )
            }
        }
        impl IPrefetchableSupport {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IPrefetchableSupportVtbl
            where
                C: IPrefetchableSupportTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getPrefetchableSupport<C, W, const OFFSET: isize>(
                    this: *mut IPrefetchableSupport,
                    prefetchable: *mut PrefetchableSupport,
                ) -> tresult
                where
                    C: IPrefetchableSupportTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getPrefetchableSupport(
                        prefetchable,
                    )
                }
                IPrefetchableSupportVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getPrefetchableSupport: getPrefetchableSupport::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IPrefetchableSupport
        where
            C: IPrefetchableSupportTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IPrefetchableSupport {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        mod __IProcessContextRequirements_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::IProcessContextRequirements_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IProcessContextRequirements {
                pub vtbl: *const IProcessContextRequirementsVtbl,
            }
            unsafe impl Send for IProcessContextRequirements {}
            unsafe impl Sync for IProcessContextRequirements {}
            unsafe impl ::com_scrape_types::Inherits<FUnknown> for IProcessContextRequirements {}
            impl ::com_scrape_types::Unknown for IProcessContextRequirements {
                #[inline]
                unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                    crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
                }
                #[inline]
                unsafe fn add_ref(this: *mut Self) -> usize {
                    crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
                }
                #[inline]
                unsafe fn release(this: *mut Self) -> usize {
                    crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
                }
            }
            unsafe impl ::com_scrape_types::Interface for IProcessContextRequirements {
                type Vtbl = IProcessContextRequirementsVtbl;
                const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IProcessContextRequirements_iid);
                #[inline]
                fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                    iid == &Self::IID || FUnknown::inherits(iid)
                }
            }
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IProcessContextRequirementsVtbl {
                pub base: FUnknownVtbl,
                pub getProcessContextRequirements: unsafe extern "system" fn(
                    this: *mut IProcessContextRequirements,
                ) -> uint32,
            }
            pub trait IProcessContextRequirementsTrait {
                unsafe fn getProcessContextRequirements(
                    &self,
                ) -> uint32;
            }
            impl<P> IProcessContextRequirementsTrait for P
            where
                P: ::com_scrape_types::SmartPtr,
                P::Target: ::com_scrape_types::Inherits<IProcessContextRequirements>,
            {
                #[inline]
                unsafe fn getProcessContextRequirements(
                    &self,
                ) -> uint32 {
                    let ptr = self.ptr() as *mut IProcessContextRequirements;
                    ((*(*ptr).vtbl).getProcessContextRequirements)(
                        ptr,
                    )
                }
            }
            impl IProcessContextRequirements {
                const fn make_vtbl<C, W, const OFFSET: isize>() -> IProcessContextRequirementsVtbl
                where
                    C: IProcessContextRequirementsTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    unsafe extern "system" fn getProcessContextRequirements<C, W, const OFFSET: isize>(
                        this: *mut IProcessContextRequirements,
                    ) -> uint32
                    where
                        C: IProcessContextRequirementsTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).getProcessContextRequirements(
                        )
                    }
                    IProcessContextRequirementsVtbl {
                        base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                        getProcessContextRequirements: getProcessContextRequirements::<C, W, OFFSET>,
                    }
                }
            }
            unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IProcessContextRequirements
            where
                C: IProcessContextRequirementsTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                const OBJ: Self = IProcessContextRequirements {
                    vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
                };
            }
        }
        pub use __IProcessContextRequirements_wrapper::*;
        pub mod IProcessContextRequirements_ {
            #[allow(unused_imports)]
            use super::*;
            pub type Flags = crate::support::DefaultEnumType;
            pub mod Flags_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kNeedBarPositionMusic: Flags = 8;
                pub const kNeedChord: Flags = 256;
                pub const kNeedContinousTimeSamples: Flags = 2;
                pub const kNeedCycleMusic: Flags = 16;
                pub const kNeedFrameRate: Flags = 512;
                pub const kNeedProjectTimeMusic: Flags = 4;
                pub const kNeedSamplesToNextClock: Flags = 32;
                pub const kNeedSystemTime: Flags = 1;
                pub const kNeedTempo: Flags = 64;
                pub const kNeedTimeSignature: Flags = 128;
                pub const kNeedTransportState: Flags = 1024;
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IProgramListData {
            pub vtbl: *const IProgramListDataVtbl,
        }
        unsafe impl Send for IProgramListData {}
        unsafe impl Sync for IProgramListData {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IProgramListData {}
        impl ::com_scrape_types::Unknown for IProgramListData {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IProgramListData {
            type Vtbl = IProgramListDataVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IProgramListData_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IProgramListDataVtbl {
            pub base: FUnknownVtbl,
            pub programDataSupported: unsafe extern "system" fn(
                this: *mut IProgramListData,
                listId: ProgramListID,
            ) -> tresult,
            pub getProgramData: unsafe extern "system" fn(
                this: *mut IProgramListData,
                listId: ProgramListID,
                programIndex: int32,
                data: *mut IBStream,
            ) -> tresult,
            pub setProgramData: unsafe extern "system" fn(
                this: *mut IProgramListData,
                listId: ProgramListID,
                programIndex: int32,
                data: *mut IBStream,
            ) -> tresult,
        }
        pub trait IProgramListDataTrait {
            unsafe fn programDataSupported(
                &self,
                listId: ProgramListID,
            ) -> tresult;
            unsafe fn getProgramData(
                &self,
                listId: ProgramListID,
                programIndex: int32,
                data: *mut IBStream,
            ) -> tresult;
            unsafe fn setProgramData(
                &self,
                listId: ProgramListID,
                programIndex: int32,
                data: *mut IBStream,
            ) -> tresult;
        }
        impl<P> IProgramListDataTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IProgramListData>,
        {
            #[inline]
            unsafe fn programDataSupported(
                &self,
                listId: ProgramListID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IProgramListData;
                ((*(*ptr).vtbl).programDataSupported)(
                    ptr,
                    listId,
                )
            }
            #[inline]
            unsafe fn getProgramData(
                &self,
                listId: ProgramListID,
                programIndex: int32,
                data: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IProgramListData;
                ((*(*ptr).vtbl).getProgramData)(
                    ptr,
                    listId,
                    programIndex,
                    data,
                )
            }
            #[inline]
            unsafe fn setProgramData(
                &self,
                listId: ProgramListID,
                programIndex: int32,
                data: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IProgramListData;
                ((*(*ptr).vtbl).setProgramData)(
                    ptr,
                    listId,
                    programIndex,
                    data,
                )
            }
        }
        impl IProgramListData {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IProgramListDataVtbl
            where
                C: IProgramListDataTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn programDataSupported<C, W, const OFFSET: isize>(
                    this: *mut IProgramListData,
                    listId: ProgramListID,
                ) -> tresult
                where
                    C: IProgramListDataTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).programDataSupported(
                        listId,
                    )
                }
                unsafe extern "system" fn getProgramData<C, W, const OFFSET: isize>(
                    this: *mut IProgramListData,
                    listId: ProgramListID,
                    programIndex: int32,
                    data: *mut IBStream,
                ) -> tresult
                where
                    C: IProgramListDataTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getProgramData(
                        listId,
                        programIndex,
                        data,
                    )
                }
                unsafe extern "system" fn setProgramData<C, W, const OFFSET: isize>(
                    this: *mut IProgramListData,
                    listId: ProgramListID,
                    programIndex: int32,
                    data: *mut IBStream,
                ) -> tresult
                where
                    C: IProgramListDataTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setProgramData(
                        listId,
                        programIndex,
                        data,
                    )
                }
                IProgramListDataVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    programDataSupported: programDataSupported::<C, W, OFFSET>,
                    getProgramData: getProgramData::<C, W, OFFSET>,
                    setProgramData: setProgramData::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IProgramListData
        where
            C: IProgramListDataTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IProgramListData {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        mod __IProgress_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::IProgress_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IProgress {
                pub vtbl: *const IProgressVtbl,
            }
            unsafe impl Send for IProgress {}
            unsafe impl Sync for IProgress {}
            unsafe impl ::com_scrape_types::Inherits<FUnknown> for IProgress {}
            impl ::com_scrape_types::Unknown for IProgress {
                #[inline]
                unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                    crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
                }
                #[inline]
                unsafe fn add_ref(this: *mut Self) -> usize {
                    crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
                }
                #[inline]
                unsafe fn release(this: *mut Self) -> usize {
                    crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
                }
            }
            unsafe impl ::com_scrape_types::Interface for IProgress {
                type Vtbl = IProgressVtbl;
                const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IProgress_iid);
                #[inline]
                fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                    iid == &Self::IID || FUnknown::inherits(iid)
                }
            }
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IProgressVtbl {
                pub base: FUnknownVtbl,
                pub start: unsafe extern "system" fn(
                    this: *mut IProgress,
                    r#type: ProgressType,
                    optionalDescription: *const tchar,
                    outID: *mut ID,
                ) -> tresult,
                pub update: unsafe extern "system" fn(
                    this: *mut IProgress,
                    id: ID,
                    normValue: ParamValue,
                ) -> tresult,
                pub finish: unsafe extern "system" fn(
                    this: *mut IProgress,
                    id: ID,
                ) -> tresult,
            }
            pub trait IProgressTrait {
                unsafe fn start(
                    &self,
                    r#type: ProgressType,
                    optionalDescription: *const tchar,
                    outID: *mut ID,
                ) -> tresult;
                unsafe fn update(
                    &self,
                    id: ID,
                    normValue: ParamValue,
                ) -> tresult;
                unsafe fn finish(
                    &self,
                    id: ID,
                ) -> tresult;
            }
            impl<P> IProgressTrait for P
            where
                P: ::com_scrape_types::SmartPtr,
                P::Target: ::com_scrape_types::Inherits<IProgress>,
            {
                #[inline]
                unsafe fn start(
                    &self,
                    r#type: ProgressType,
                    optionalDescription: *const tchar,
                    outID: *mut ID,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IProgress;
                    ((*(*ptr).vtbl).start)(
                        ptr,
                        r#type,
                        optionalDescription,
                        outID,
                    )
                }
                #[inline]
                unsafe fn update(
                    &self,
                    id: ID,
                    normValue: ParamValue,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IProgress;
                    ((*(*ptr).vtbl).update)(
                        ptr,
                        id,
                        normValue,
                    )
                }
                #[inline]
                unsafe fn finish(
                    &self,
                    id: ID,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IProgress;
                    ((*(*ptr).vtbl).finish)(
                        ptr,
                        id,
                    )
                }
            }
            impl IProgress {
                const fn make_vtbl<C, W, const OFFSET: isize>() -> IProgressVtbl
                where
                    C: IProgressTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    unsafe extern "system" fn start<C, W, const OFFSET: isize>(
                        this: *mut IProgress,
                        r#type: ProgressType,
                        optionalDescription: *const tchar,
                        outID: *mut ID,
                    ) -> tresult
                    where
                        C: IProgressTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).start(
                            r#type,
                            optionalDescription,
                            outID,
                        )
                    }
                    unsafe extern "system" fn update<C, W, const OFFSET: isize>(
                        this: *mut IProgress,
                        id: ID,
                        normValue: ParamValue,
                    ) -> tresult
                    where
                        C: IProgressTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).update(
                            id,
                            normValue,
                        )
                    }
                    unsafe extern "system" fn finish<C, W, const OFFSET: isize>(
                        this: *mut IProgress,
                        id: ID,
                    ) -> tresult
                    where
                        C: IProgressTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).finish(
                            id,
                        )
                    }
                    IProgressVtbl {
                        base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                        start: start::<C, W, OFFSET>,
                        update: update::<C, W, OFFSET>,
                        finish: finish::<C, W, OFFSET>,
                    }
                }
            }
            unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IProgress
            where
                C: IProgressTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                const OBJ: Self = IProgress {
                    vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
                };
            }
        }
        pub use __IProgress_wrapper::*;
        pub mod IProgress_ {
            #[allow(unused_imports)]
            use super::*;
            pub type ID = uint64;
            pub type ProgressType = uint32;
            pub mod ProgressType_ {
                #[allow(unused_imports)]
                use super::*;
                pub const AsyncStateRestoration: ProgressType = 0;
                pub const UIBackgroundTask: ProgressType = 1;
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IRemapParamID {
            pub vtbl: *const IRemapParamIDVtbl,
        }
        unsafe impl Send for IRemapParamID {}
        unsafe impl Sync for IRemapParamID {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IRemapParamID {}
        impl ::com_scrape_types::Unknown for IRemapParamID {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IRemapParamID {
            type Vtbl = IRemapParamIDVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IRemapParamID_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IRemapParamIDVtbl {
            pub base: FUnknownVtbl,
            pub getCompatibleParamID: unsafe extern "system" fn(
                this: *mut IRemapParamID,
                pluginToReplaceUID: *const TUID,
                oldParamID: ParamID,
                newParamID: *mut ParamID,
            ) -> tresult,
        }
        pub trait IRemapParamIDTrait {
            unsafe fn getCompatibleParamID(
                &self,
                pluginToReplaceUID: *const TUID,
                oldParamID: ParamID,
                newParamID: *mut ParamID,
            ) -> tresult;
        }
        impl<P> IRemapParamIDTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IRemapParamID>,
        {
            #[inline]
            unsafe fn getCompatibleParamID(
                &self,
                pluginToReplaceUID: *const TUID,
                oldParamID: ParamID,
                newParamID: *mut ParamID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IRemapParamID;
                ((*(*ptr).vtbl).getCompatibleParamID)(
                    ptr,
                    pluginToReplaceUID,
                    oldParamID,
                    newParamID,
                )
            }
        }
        impl IRemapParamID {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IRemapParamIDVtbl
            where
                C: IRemapParamIDTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getCompatibleParamID<C, W, const OFFSET: isize>(
                    this: *mut IRemapParamID,
                    pluginToReplaceUID: *const TUID,
                    oldParamID: ParamID,
                    newParamID: *mut ParamID,
                ) -> tresult
                where
                    C: IRemapParamIDTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getCompatibleParamID(
                        pluginToReplaceUID,
                        oldParamID,
                        newParamID,
                    )
                }
                IRemapParamIDVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getCompatibleParamID: getCompatibleParamID::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IRemapParamID
        where
            C: IRemapParamIDTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IRemapParamID {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IStreamAttributes {
            pub vtbl: *const IStreamAttributesVtbl,
        }
        unsafe impl Send for IStreamAttributes {}
        unsafe impl Sync for IStreamAttributes {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IStreamAttributes {}
        impl ::com_scrape_types::Unknown for IStreamAttributes {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IStreamAttributes {
            type Vtbl = IStreamAttributesVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IStreamAttributes_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IStreamAttributesVtbl {
            pub base: FUnknownVtbl,
            pub getFileName: unsafe extern "system" fn(
                this: *mut IStreamAttributes,
                name: *mut String128,
            ) -> tresult,
            pub getAttributes: unsafe extern "system" fn(
                this: *mut IStreamAttributes,
            ) -> *mut IAttributeList,
        }
        pub trait IStreamAttributesTrait {
            unsafe fn getFileName(
                &self,
                name: *mut String128,
            ) -> tresult;
            unsafe fn getAttributes(
                &self,
            ) -> *mut IAttributeList;
        }
        impl<P> IStreamAttributesTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IStreamAttributes>,
        {
            #[inline]
            unsafe fn getFileName(
                &self,
                name: *mut String128,
            ) -> tresult {
                let ptr = self.ptr() as *mut IStreamAttributes;
                ((*(*ptr).vtbl).getFileName)(
                    ptr,
                    name,
                )
            }
            #[inline]
            unsafe fn getAttributes(
                &self,
            ) -> *mut IAttributeList {
                let ptr = self.ptr() as *mut IStreamAttributes;
                ((*(*ptr).vtbl).getAttributes)(
                    ptr,
                )
            }
        }
        impl IStreamAttributes {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IStreamAttributesVtbl
            where
                C: IStreamAttributesTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getFileName<C, W, const OFFSET: isize>(
                    this: *mut IStreamAttributes,
                    name: *mut String128,
                ) -> tresult
                where
                    C: IStreamAttributesTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getFileName(
                        name,
                    )
                }
                unsafe extern "system" fn getAttributes<C, W, const OFFSET: isize>(
                    this: *mut IStreamAttributes,
                ) -> *mut IAttributeList
                where
                    C: IStreamAttributesTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getAttributes(
                    )
                }
                IStreamAttributesVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getFileName: getFileName::<C, W, OFFSET>,
                    getAttributes: getAttributes::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IStreamAttributes
        where
            C: IStreamAttributesTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IStreamAttributes {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IUnitData {
            pub vtbl: *const IUnitDataVtbl,
        }
        unsafe impl Send for IUnitData {}
        unsafe impl Sync for IUnitData {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IUnitData {}
        impl ::com_scrape_types::Unknown for IUnitData {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IUnitData {
            type Vtbl = IUnitDataVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IUnitData_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IUnitDataVtbl {
            pub base: FUnknownVtbl,
            pub unitDataSupported: unsafe extern "system" fn(
                this: *mut IUnitData,
                unitID: UnitID,
            ) -> tresult,
            pub getUnitData: unsafe extern "system" fn(
                this: *mut IUnitData,
                unitId: UnitID,
                data: *mut IBStream,
            ) -> tresult,
            pub setUnitData: unsafe extern "system" fn(
                this: *mut IUnitData,
                unitId: UnitID,
                data: *mut IBStream,
            ) -> tresult,
        }
        pub trait IUnitDataTrait {
            unsafe fn unitDataSupported(
                &self,
                unitID: UnitID,
            ) -> tresult;
            unsafe fn getUnitData(
                &self,
                unitId: UnitID,
                data: *mut IBStream,
            ) -> tresult;
            unsafe fn setUnitData(
                &self,
                unitId: UnitID,
                data: *mut IBStream,
            ) -> tresult;
        }
        impl<P> IUnitDataTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IUnitData>,
        {
            #[inline]
            unsafe fn unitDataSupported(
                &self,
                unitID: UnitID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitData;
                ((*(*ptr).vtbl).unitDataSupported)(
                    ptr,
                    unitID,
                )
            }
            #[inline]
            unsafe fn getUnitData(
                &self,
                unitId: UnitID,
                data: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitData;
                ((*(*ptr).vtbl).getUnitData)(
                    ptr,
                    unitId,
                    data,
                )
            }
            #[inline]
            unsafe fn setUnitData(
                &self,
                unitId: UnitID,
                data: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitData;
                ((*(*ptr).vtbl).setUnitData)(
                    ptr,
                    unitId,
                    data,
                )
            }
        }
        impl IUnitData {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IUnitDataVtbl
            where
                C: IUnitDataTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn unitDataSupported<C, W, const OFFSET: isize>(
                    this: *mut IUnitData,
                    unitID: UnitID,
                ) -> tresult
                where
                    C: IUnitDataTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).unitDataSupported(
                        unitID,
                    )
                }
                unsafe extern "system" fn getUnitData<C, W, const OFFSET: isize>(
                    this: *mut IUnitData,
                    unitId: UnitID,
                    data: *mut IBStream,
                ) -> tresult
                where
                    C: IUnitDataTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getUnitData(
                        unitId,
                        data,
                    )
                }
                unsafe extern "system" fn setUnitData<C, W, const OFFSET: isize>(
                    this: *mut IUnitData,
                    unitId: UnitID,
                    data: *mut IBStream,
                ) -> tresult
                where
                    C: IUnitDataTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setUnitData(
                        unitId,
                        data,
                    )
                }
                IUnitDataVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    unitDataSupported: unitDataSupported::<C, W, OFFSET>,
                    getUnitData: getUnitData::<C, W, OFFSET>,
                    setUnitData: setUnitData::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IUnitData
        where
            C: IUnitDataTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IUnitData {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IUnitHandler {
            pub vtbl: *const IUnitHandlerVtbl,
        }
        unsafe impl Send for IUnitHandler {}
        unsafe impl Sync for IUnitHandler {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IUnitHandler {}
        impl ::com_scrape_types::Unknown for IUnitHandler {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IUnitHandler {
            type Vtbl = IUnitHandlerVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IUnitHandler_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IUnitHandlerVtbl {
            pub base: FUnknownVtbl,
            pub notifyUnitSelection: unsafe extern "system" fn(
                this: *mut IUnitHandler,
                unitId: UnitID,
            ) -> tresult,
            pub notifyProgramListChange: unsafe extern "system" fn(
                this: *mut IUnitHandler,
                listId: ProgramListID,
                programIndex: int32,
            ) -> tresult,
        }
        pub trait IUnitHandlerTrait {
            unsafe fn notifyUnitSelection(
                &self,
                unitId: UnitID,
            ) -> tresult;
            unsafe fn notifyProgramListChange(
                &self,
                listId: ProgramListID,
                programIndex: int32,
            ) -> tresult;
        }
        impl<P> IUnitHandlerTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IUnitHandler>,
        {
            #[inline]
            unsafe fn notifyUnitSelection(
                &self,
                unitId: UnitID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitHandler;
                ((*(*ptr).vtbl).notifyUnitSelection)(
                    ptr,
                    unitId,
                )
            }
            #[inline]
            unsafe fn notifyProgramListChange(
                &self,
                listId: ProgramListID,
                programIndex: int32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitHandler;
                ((*(*ptr).vtbl).notifyProgramListChange)(
                    ptr,
                    listId,
                    programIndex,
                )
            }
        }
        impl IUnitHandler {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IUnitHandlerVtbl
            where
                C: IUnitHandlerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn notifyUnitSelection<C, W, const OFFSET: isize>(
                    this: *mut IUnitHandler,
                    unitId: UnitID,
                ) -> tresult
                where
                    C: IUnitHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).notifyUnitSelection(
                        unitId,
                    )
                }
                unsafe extern "system" fn notifyProgramListChange<C, W, const OFFSET: isize>(
                    this: *mut IUnitHandler,
                    listId: ProgramListID,
                    programIndex: int32,
                ) -> tresult
                where
                    C: IUnitHandlerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).notifyProgramListChange(
                        listId,
                        programIndex,
                    )
                }
                IUnitHandlerVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    notifyUnitSelection: notifyUnitSelection::<C, W, OFFSET>,
                    notifyProgramListChange: notifyProgramListChange::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IUnitHandler
        where
            C: IUnitHandlerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IUnitHandler {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IUnitHandler2 {
            pub vtbl: *const IUnitHandler2Vtbl,
        }
        unsafe impl Send for IUnitHandler2 {}
        unsafe impl Sync for IUnitHandler2 {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IUnitHandler2 {}
        impl ::com_scrape_types::Unknown for IUnitHandler2 {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IUnitHandler2 {
            type Vtbl = IUnitHandler2Vtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IUnitHandler2_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IUnitHandler2Vtbl {
            pub base: FUnknownVtbl,
            pub notifyUnitByBusChange: unsafe extern "system" fn(
                this: *mut IUnitHandler2,
            ) -> tresult,
        }
        pub trait IUnitHandler2Trait {
            unsafe fn notifyUnitByBusChange(
                &self,
            ) -> tresult;
        }
        impl<P> IUnitHandler2Trait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IUnitHandler2>,
        {
            #[inline]
            unsafe fn notifyUnitByBusChange(
                &self,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitHandler2;
                ((*(*ptr).vtbl).notifyUnitByBusChange)(
                    ptr,
                )
            }
        }
        impl IUnitHandler2 {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IUnitHandler2Vtbl
            where
                C: IUnitHandler2Trait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn notifyUnitByBusChange<C, W, const OFFSET: isize>(
                    this: *mut IUnitHandler2,
                ) -> tresult
                where
                    C: IUnitHandler2Trait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).notifyUnitByBusChange(
                    )
                }
                IUnitHandler2Vtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    notifyUnitByBusChange: notifyUnitByBusChange::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IUnitHandler2
        where
            C: IUnitHandler2Trait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IUnitHandler2 {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IUnitInfo {
            pub vtbl: *const IUnitInfoVtbl,
        }
        unsafe impl Send for IUnitInfo {}
        unsafe impl Sync for IUnitInfo {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IUnitInfo {}
        impl ::com_scrape_types::Unknown for IUnitInfo {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IUnitInfo {
            type Vtbl = IUnitInfoVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IUnitInfo_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IUnitInfoVtbl {
            pub base: FUnknownVtbl,
            pub getUnitCount: unsafe extern "system" fn(
                this: *mut IUnitInfo,
            ) -> int32,
            pub getUnitInfo: unsafe extern "system" fn(
                this: *mut IUnitInfo,
                unitIndex: int32,
                info: *mut UnitInfo,
            ) -> tresult,
            pub getProgramListCount: unsafe extern "system" fn(
                this: *mut IUnitInfo,
            ) -> int32,
            pub getProgramListInfo: unsafe extern "system" fn(
                this: *mut IUnitInfo,
                listIndex: int32,
                info: *mut ProgramListInfo,
            ) -> tresult,
            pub getProgramName: unsafe extern "system" fn(
                this: *mut IUnitInfo,
                listId: ProgramListID,
                programIndex: int32,
                name: *mut String128,
            ) -> tresult,
            pub getProgramInfo: unsafe extern "system" fn(
                this: *mut IUnitInfo,
                listId: ProgramListID,
                programIndex: int32,
                attributeId: CString,
                attributeValue: *mut String128,
            ) -> tresult,
            pub hasProgramPitchNames: unsafe extern "system" fn(
                this: *mut IUnitInfo,
                listId: ProgramListID,
                programIndex: int32,
            ) -> tresult,
            pub getProgramPitchName: unsafe extern "system" fn(
                this: *mut IUnitInfo,
                listId: ProgramListID,
                programIndex: int32,
                midiPitch: int16,
                name: *mut String128,
            ) -> tresult,
            pub getSelectedUnit: unsafe extern "system" fn(
                this: *mut IUnitInfo,
            ) -> UnitID,
            pub selectUnit: unsafe extern "system" fn(
                this: *mut IUnitInfo,
                unitId: UnitID,
            ) -> tresult,
            pub getUnitByBus: unsafe extern "system" fn(
                this: *mut IUnitInfo,
                r#type: MediaType,
                dir: BusDirection,
                busIndex: int32,
                channel: int32,
                unitId: *mut UnitID,
            ) -> tresult,
            pub setUnitProgramData: unsafe extern "system" fn(
                this: *mut IUnitInfo,
                listOrUnitId: int32,
                programIndex: int32,
                data: *mut IBStream,
            ) -> tresult,
        }
        pub trait IUnitInfoTrait {
            unsafe fn getUnitCount(
                &self,
            ) -> int32;
            unsafe fn getUnitInfo(
                &self,
                unitIndex: int32,
                info: *mut UnitInfo,
            ) -> tresult;
            unsafe fn getProgramListCount(
                &self,
            ) -> int32;
            unsafe fn getProgramListInfo(
                &self,
                listIndex: int32,
                info: *mut ProgramListInfo,
            ) -> tresult;
            unsafe fn getProgramName(
                &self,
                listId: ProgramListID,
                programIndex: int32,
                name: *mut String128,
            ) -> tresult;
            unsafe fn getProgramInfo(
                &self,
                listId: ProgramListID,
                programIndex: int32,
                attributeId: CString,
                attributeValue: *mut String128,
            ) -> tresult;
            unsafe fn hasProgramPitchNames(
                &self,
                listId: ProgramListID,
                programIndex: int32,
            ) -> tresult;
            unsafe fn getProgramPitchName(
                &self,
                listId: ProgramListID,
                programIndex: int32,
                midiPitch: int16,
                name: *mut String128,
            ) -> tresult;
            unsafe fn getSelectedUnit(
                &self,
            ) -> UnitID;
            unsafe fn selectUnit(
                &self,
                unitId: UnitID,
            ) -> tresult;
            unsafe fn getUnitByBus(
                &self,
                r#type: MediaType,
                dir: BusDirection,
                busIndex: int32,
                channel: int32,
                unitId: *mut UnitID,
            ) -> tresult;
            unsafe fn setUnitProgramData(
                &self,
                listOrUnitId: int32,
                programIndex: int32,
                data: *mut IBStream,
            ) -> tresult;
        }
        impl<P> IUnitInfoTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IUnitInfo>,
        {
            #[inline]
            unsafe fn getUnitCount(
                &self,
            ) -> int32 {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).getUnitCount)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn getUnitInfo(
                &self,
                unitIndex: int32,
                info: *mut UnitInfo,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).getUnitInfo)(
                    ptr,
                    unitIndex,
                    info,
                )
            }
            #[inline]
            unsafe fn getProgramListCount(
                &self,
            ) -> int32 {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).getProgramListCount)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn getProgramListInfo(
                &self,
                listIndex: int32,
                info: *mut ProgramListInfo,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).getProgramListInfo)(
                    ptr,
                    listIndex,
                    info,
                )
            }
            #[inline]
            unsafe fn getProgramName(
                &self,
                listId: ProgramListID,
                programIndex: int32,
                name: *mut String128,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).getProgramName)(
                    ptr,
                    listId,
                    programIndex,
                    name,
                )
            }
            #[inline]
            unsafe fn getProgramInfo(
                &self,
                listId: ProgramListID,
                programIndex: int32,
                attributeId: CString,
                attributeValue: *mut String128,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).getProgramInfo)(
                    ptr,
                    listId,
                    programIndex,
                    attributeId,
                    attributeValue,
                )
            }
            #[inline]
            unsafe fn hasProgramPitchNames(
                &self,
                listId: ProgramListID,
                programIndex: int32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).hasProgramPitchNames)(
                    ptr,
                    listId,
                    programIndex,
                )
            }
            #[inline]
            unsafe fn getProgramPitchName(
                &self,
                listId: ProgramListID,
                programIndex: int32,
                midiPitch: int16,
                name: *mut String128,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).getProgramPitchName)(
                    ptr,
                    listId,
                    programIndex,
                    midiPitch,
                    name,
                )
            }
            #[inline]
            unsafe fn getSelectedUnit(
                &self,
            ) -> UnitID {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).getSelectedUnit)(
                    ptr,
                )
            }
            #[inline]
            unsafe fn selectUnit(
                &self,
                unitId: UnitID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).selectUnit)(
                    ptr,
                    unitId,
                )
            }
            #[inline]
            unsafe fn getUnitByBus(
                &self,
                r#type: MediaType,
                dir: BusDirection,
                busIndex: int32,
                channel: int32,
                unitId: *mut UnitID,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).getUnitByBus)(
                    ptr,
                    r#type,
                    dir,
                    busIndex,
                    channel,
                    unitId,
                )
            }
            #[inline]
            unsafe fn setUnitProgramData(
                &self,
                listOrUnitId: int32,
                programIndex: int32,
                data: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IUnitInfo;
                ((*(*ptr).vtbl).setUnitProgramData)(
                    ptr,
                    listOrUnitId,
                    programIndex,
                    data,
                )
            }
        }
        impl IUnitInfo {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IUnitInfoVtbl
            where
                C: IUnitInfoTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getUnitCount<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                ) -> int32
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getUnitCount(
                    )
                }
                unsafe extern "system" fn getUnitInfo<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                    unitIndex: int32,
                    info: *mut UnitInfo,
                ) -> tresult
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getUnitInfo(
                        unitIndex,
                        info,
                    )
                }
                unsafe extern "system" fn getProgramListCount<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                ) -> int32
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getProgramListCount(
                    )
                }
                unsafe extern "system" fn getProgramListInfo<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                    listIndex: int32,
                    info: *mut ProgramListInfo,
                ) -> tresult
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getProgramListInfo(
                        listIndex,
                        info,
                    )
                }
                unsafe extern "system" fn getProgramName<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                    listId: ProgramListID,
                    programIndex: int32,
                    name: *mut String128,
                ) -> tresult
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getProgramName(
                        listId,
                        programIndex,
                        name,
                    )
                }
                unsafe extern "system" fn getProgramInfo<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                    listId: ProgramListID,
                    programIndex: int32,
                    attributeId: CString,
                    attributeValue: *mut String128,
                ) -> tresult
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getProgramInfo(
                        listId,
                        programIndex,
                        attributeId,
                        attributeValue,
                    )
                }
                unsafe extern "system" fn hasProgramPitchNames<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                    listId: ProgramListID,
                    programIndex: int32,
                ) -> tresult
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).hasProgramPitchNames(
                        listId,
                        programIndex,
                    )
                }
                unsafe extern "system" fn getProgramPitchName<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                    listId: ProgramListID,
                    programIndex: int32,
                    midiPitch: int16,
                    name: *mut String128,
                ) -> tresult
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getProgramPitchName(
                        listId,
                        programIndex,
                        midiPitch,
                        name,
                    )
                }
                unsafe extern "system" fn getSelectedUnit<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                ) -> UnitID
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getSelectedUnit(
                    )
                }
                unsafe extern "system" fn selectUnit<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                    unitId: UnitID,
                ) -> tresult
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).selectUnit(
                        unitId,
                    )
                }
                unsafe extern "system" fn getUnitByBus<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                    r#type: MediaType,
                    dir: BusDirection,
                    busIndex: int32,
                    channel: int32,
                    unitId: *mut UnitID,
                ) -> tresult
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getUnitByBus(
                        r#type,
                        dir,
                        busIndex,
                        channel,
                        unitId,
                    )
                }
                unsafe extern "system" fn setUnitProgramData<C, W, const OFFSET: isize>(
                    this: *mut IUnitInfo,
                    listOrUnitId: int32,
                    programIndex: int32,
                    data: *mut IBStream,
                ) -> tresult
                where
                    C: IUnitInfoTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setUnitProgramData(
                        listOrUnitId,
                        programIndex,
                        data,
                    )
                }
                IUnitInfoVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getUnitCount: getUnitCount::<C, W, OFFSET>,
                    getUnitInfo: getUnitInfo::<C, W, OFFSET>,
                    getProgramListCount: getProgramListCount::<C, W, OFFSET>,
                    getProgramListInfo: getProgramListInfo::<C, W, OFFSET>,
                    getProgramName: getProgramName::<C, W, OFFSET>,
                    getProgramInfo: getProgramInfo::<C, W, OFFSET>,
                    hasProgramPitchNames: hasProgramPitchNames::<C, W, OFFSET>,
                    getProgramPitchName: getProgramPitchName::<C, W, OFFSET>,
                    getSelectedUnit: getSelectedUnit::<C, W, OFFSET>,
                    selectUnit: selectUnit::<C, W, OFFSET>,
                    getUnitByBus: getUnitByBus::<C, W, OFFSET>,
                    setUnitProgramData: setUnitProgramData::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IUnitInfo
        where
            C: IUnitInfoTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IUnitInfo {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IVst3ToAAXWrapper {
        }
        unsafe impl Send for IVst3ToAAXWrapper {}
        unsafe impl Sync for IVst3ToAAXWrapper {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IVst3ToAUWrapper {
        }
        unsafe impl Send for IVst3ToAUWrapper {}
        unsafe impl Sync for IVst3ToAUWrapper {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IVst3ToVst2Wrapper {
        }
        unsafe impl Send for IVst3ToVst2Wrapper {}
        unsafe impl Sync for IVst3ToVst2Wrapper {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IVst3WrapperMPESupport {
            pub vtbl: *const IVst3WrapperMPESupportVtbl,
        }
        unsafe impl Send for IVst3WrapperMPESupport {}
        unsafe impl Sync for IVst3WrapperMPESupport {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IVst3WrapperMPESupport {}
        impl ::com_scrape_types::Unknown for IVst3WrapperMPESupport {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IVst3WrapperMPESupport {
            type Vtbl = IVst3WrapperMPESupportVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IVst3WrapperMPESupport_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IVst3WrapperMPESupportVtbl {
            pub base: FUnknownVtbl,
            pub enableMPEInputProcessing: unsafe extern "system" fn(
                this: *mut IVst3WrapperMPESupport,
                state: TBool,
            ) -> tresult,
            pub setMPEInputDeviceSettings: unsafe extern "system" fn(
                this: *mut IVst3WrapperMPESupport,
                masterChannel: int32,
                memberBeginChannel: int32,
                memberEndChannel: int32,
            ) -> tresult,
        }
        pub trait IVst3WrapperMPESupportTrait {
            unsafe fn enableMPEInputProcessing(
                &self,
                state: TBool,
            ) -> tresult;
            unsafe fn setMPEInputDeviceSettings(
                &self,
                masterChannel: int32,
                memberBeginChannel: int32,
                memberEndChannel: int32,
            ) -> tresult;
        }
        impl<P> IVst3WrapperMPESupportTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IVst3WrapperMPESupport>,
        {
            #[inline]
            unsafe fn enableMPEInputProcessing(
                &self,
                state: TBool,
            ) -> tresult {
                let ptr = self.ptr() as *mut IVst3WrapperMPESupport;
                ((*(*ptr).vtbl).enableMPEInputProcessing)(
                    ptr,
                    state,
                )
            }
            #[inline]
            unsafe fn setMPEInputDeviceSettings(
                &self,
                masterChannel: int32,
                memberBeginChannel: int32,
                memberEndChannel: int32,
            ) -> tresult {
                let ptr = self.ptr() as *mut IVst3WrapperMPESupport;
                ((*(*ptr).vtbl).setMPEInputDeviceSettings)(
                    ptr,
                    masterChannel,
                    memberBeginChannel,
                    memberEndChannel,
                )
            }
        }
        impl IVst3WrapperMPESupport {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IVst3WrapperMPESupportVtbl
            where
                C: IVst3WrapperMPESupportTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn enableMPEInputProcessing<C, W, const OFFSET: isize>(
                    this: *mut IVst3WrapperMPESupport,
                    state: TBool,
                ) -> tresult
                where
                    C: IVst3WrapperMPESupportTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).enableMPEInputProcessing(
                        state,
                    )
                }
                unsafe extern "system" fn setMPEInputDeviceSettings<C, W, const OFFSET: isize>(
                    this: *mut IVst3WrapperMPESupport,
                    masterChannel: int32,
                    memberBeginChannel: int32,
                    memberEndChannel: int32,
                ) -> tresult
                where
                    C: IVst3WrapperMPESupportTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).setMPEInputDeviceSettings(
                        masterChannel,
                        memberBeginChannel,
                        memberEndChannel,
                    )
                }
                IVst3WrapperMPESupportVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    enableMPEInputProcessing: enableMPEInputProcessing::<C, W, OFFSET>,
                    setMPEInputDeviceSettings: setMPEInputDeviceSettings::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IVst3WrapperMPESupport
        where
            C: IVst3WrapperMPESupportTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IVst3WrapperMPESupport {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IXmlRepresentationController {
            pub vtbl: *const IXmlRepresentationControllerVtbl,
        }
        unsafe impl Send for IXmlRepresentationController {}
        unsafe impl Sync for IXmlRepresentationController {}
        unsafe impl ::com_scrape_types::Inherits<FUnknown> for IXmlRepresentationController {}
        impl ::com_scrape_types::Unknown for IXmlRepresentationController {
            #[inline]
            unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
            }
            #[inline]
            unsafe fn add_ref(this: *mut Self) -> usize {
                crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
            }
            #[inline]
            unsafe fn release(this: *mut Self) -> usize {
                crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
            }
        }
        unsafe impl ::com_scrape_types::Interface for IXmlRepresentationController {
            type Vtbl = IXmlRepresentationControllerVtbl;
            const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IXmlRepresentationController_iid);
            #[inline]
            fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                iid == &Self::IID || FUnknown::inherits(iid)
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct IXmlRepresentationControllerVtbl {
            pub base: FUnknownVtbl,
            pub getXmlRepresentationStream: unsafe extern "system" fn(
                this: *mut IXmlRepresentationController,
                info: *mut RepresentationInfo,
                stream: *mut IBStream,
            ) -> tresult,
        }
        pub trait IXmlRepresentationControllerTrait {
            unsafe fn getXmlRepresentationStream(
                &self,
                info: *mut RepresentationInfo,
                stream: *mut IBStream,
            ) -> tresult;
        }
        impl<P> IXmlRepresentationControllerTrait for P
        where
            P: ::com_scrape_types::SmartPtr,
            P::Target: ::com_scrape_types::Inherits<IXmlRepresentationController>,
        {
            #[inline]
            unsafe fn getXmlRepresentationStream(
                &self,
                info: *mut RepresentationInfo,
                stream: *mut IBStream,
            ) -> tresult {
                let ptr = self.ptr() as *mut IXmlRepresentationController;
                ((*(*ptr).vtbl).getXmlRepresentationStream)(
                    ptr,
                    info,
                    stream,
                )
            }
        }
        impl IXmlRepresentationController {
            const fn make_vtbl<C, W, const OFFSET: isize>() -> IXmlRepresentationControllerVtbl
            where
                C: IXmlRepresentationControllerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                unsafe extern "system" fn getXmlRepresentationStream<C, W, const OFFSET: isize>(
                    this: *mut IXmlRepresentationController,
                    info: *mut RepresentationInfo,
                    stream: *mut IBStream,
                ) -> tresult
                where
                    C: IXmlRepresentationControllerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                    let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                    (*ptr).getXmlRepresentationStream(
                        info,
                        stream,
                    )
                }
                IXmlRepresentationControllerVtbl {
                    base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                    getXmlRepresentationStream: getXmlRepresentationStream::<C, W, OFFSET>,
                }
            }
        }
        unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IXmlRepresentationController
        where
            C: IXmlRepresentationControllerTrait + ::com_scrape_types::Class,
            W: ::com_scrape_types::Wrapper<C>,
        {
            const OBJ: Self = IXmlRepresentationController {
                vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
            };
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct KeyswitchInfo {
            pub typeId: KeyswitchTypeID,
            pub title: String128,
            pub shortTitle: String128,
            pub keyswitchMin: int32,
            pub keyswitchMax: int32,
            pub keyRemapped: int32,
            pub unitId: int32,
            pub flags: int32,
        }
        unsafe impl Send for KeyswitchInfo {}
        unsafe impl Sync for KeyswitchInfo {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct LegacyMIDICCOutEvent {
            pub controlNumber: uint8,
            pub channel: int8,
            pub value: int8,
            pub value2: int8,
        }
        unsafe impl Send for LegacyMIDICCOutEvent {}
        unsafe impl Sync for LegacyMIDICCOutEvent {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct Midi1ControllerParamIDAssignment {
            pub pId: ParamID,
            pub busIndex: BusIndex,
            pub channel: MidiChannel,
            pub controller: CtrlNumber,
        }
        unsafe impl Send for Midi1ControllerParamIDAssignment {}
        unsafe impl Sync for Midi1ControllerParamIDAssignment {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct Midi1ControllerParamIDAssignmentList {
            pub count: uint32,
            pub map: *mut Midi1ControllerParamIDAssignment,
        }
        unsafe impl Send for Midi1ControllerParamIDAssignmentList {}
        unsafe impl Sync for Midi1ControllerParamIDAssignmentList {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct Midi2Controller {
            pub bank: uint8,
            pub registered: TBool,
            pub index: uint8,
            pub reserved: TBool,
        }
        unsafe impl Send for Midi2Controller {}
        unsafe impl Sync for Midi2Controller {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct Midi2ControllerParamIDAssignment {
            pub pId: ParamID,
            pub busIndex: BusIndex,
            pub channel: MidiChannel,
            pub controller: Midi2Controller,
        }
        unsafe impl Send for Midi2ControllerParamIDAssignment {}
        unsafe impl Sync for Midi2ControllerParamIDAssignment {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct Midi2ControllerParamIDAssignmentList {
            pub count: uint32,
            pub map: *mut Midi2ControllerParamIDAssignment,
        }
        unsafe impl Send for Midi2ControllerParamIDAssignmentList {}
        unsafe impl Sync for Midi2ControllerParamIDAssignmentList {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct NoteExpressionIntValueEvent {
            pub typeId: NoteExpressionTypeID,
            pub noteId: int32,
            pub value: uint64,
        }
        unsafe impl Send for NoteExpressionIntValueEvent {}
        unsafe impl Sync for NoteExpressionIntValueEvent {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct NoteExpressionTextEvent {
            pub typeId: NoteExpressionTypeID,
            pub noteId: int32,
            pub textLen: uint32,
            pub text: *const TChar,
        }
        unsafe impl Send for NoteExpressionTextEvent {}
        unsafe impl Sync for NoteExpressionTextEvent {}
        mod __NoteExpressionTypeInfo_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::NoteExpressionTypeInfo_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct NoteExpressionTypeInfo {
                pub typeId: NoteExpressionTypeID,
                pub title: String128,
                pub shortTitle: String128,
                pub units: String128,
                pub unitId: int32,
                pub valueDesc: NoteExpressionValueDescription,
                pub associatedParameterId: ParamID,
                pub flags: int32,
            }
            unsafe impl Send for NoteExpressionTypeInfo {}
            unsafe impl Sync for NoteExpressionTypeInfo {}
        }
        pub use __NoteExpressionTypeInfo_wrapper::*;
        pub mod NoteExpressionTypeInfo_ {
            #[allow(unused_imports)]
            use super::*;
            pub type NoteExpressionTypeFlags = crate::support::DefaultEnumType;
            pub mod NoteExpressionTypeFlags_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kAssociatedParameterIDValid: NoteExpressionTypeFlags = 8;
                pub const kIsAbsolute: NoteExpressionTypeFlags = 4;
                pub const kIsBipolar: NoteExpressionTypeFlags = 1;
                pub const kIsOneShot: NoteExpressionTypeFlags = 2;
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct NoteExpressionValueDescription {
            pub defaultValue: NoteExpressionValue,
            pub minimum: NoteExpressionValue,
            pub maximum: NoteExpressionValue,
            pub stepCount: int32,
        }
        unsafe impl Send for NoteExpressionValueDescription {}
        unsafe impl Sync for NoteExpressionValueDescription {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct NoteExpressionValueEvent {
            pub typeId: NoteExpressionTypeID,
            pub noteId: int32,
            pub value: NoteExpressionValue,
        }
        unsafe impl Send for NoteExpressionValueEvent {}
        unsafe impl Sync for NoteExpressionValueEvent {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct NoteOffEvent {
            pub channel: int16,
            pub pitch: int16,
            pub velocity: f32,
            pub noteId: int32,
            pub tuning: f32,
        }
        unsafe impl Send for NoteOffEvent {}
        unsafe impl Sync for NoteOffEvent {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct NoteOnEvent {
            pub channel: int16,
            pub pitch: int16,
            pub tuning: f32,
            pub velocity: f32,
            pub length: int32,
            pub noteId: int32,
        }
        unsafe impl Send for NoteOnEvent {}
        unsafe impl Sync for NoteOnEvent {}
        mod __ParameterInfo_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::ParameterInfo_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct ParameterInfo {
                pub id: ParamID,
                pub title: String128,
                pub shortTitle: String128,
                pub units: String128,
                pub stepCount: int32,
                pub defaultNormalizedValue: ParamValue,
                pub unitId: UnitID,
                pub flags: int32,
            }
            unsafe impl Send for ParameterInfo {}
            unsafe impl Sync for ParameterInfo {}
        }
        pub use __ParameterInfo_wrapper::*;
        pub mod ParameterInfo_ {
            #[allow(unused_imports)]
            use super::*;
            pub type ParameterFlags = int32;
            pub mod ParameterFlags_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kCanAutomate: ParameterFlags = 1;
                pub const kIsBypass: ParameterFlags = 65536;
                pub const kIsHidden: ParameterFlags = 16;
                pub const kIsList: ParameterFlags = 8;
                pub const kIsProgramChange: ParameterFlags = 32768;
                pub const kIsReadOnly: ParameterFlags = 2;
                pub const kIsWrapAround: ParameterFlags = 4;
                pub const kNoFlags: ParameterFlags = 0;
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct PhysicalUIMap {
            pub physicalUITypeID: PhysicalUITypeID,
            pub noteExpressionTypeID: NoteExpressionTypeID,
        }
        unsafe impl Send for PhysicalUIMap {}
        unsafe impl Sync for PhysicalUIMap {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct PhysicalUIMapList {
            pub count: uint32,
            pub map: *mut PhysicalUIMap,
        }
        unsafe impl Send for PhysicalUIMapList {}
        unsafe impl Sync for PhysicalUIMapList {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct PolyPressureEvent {
            pub channel: int16,
            pub pitch: int16,
            pub pressure: f32,
            pub noteId: int32,
        }
        unsafe impl Send for PolyPressureEvent {}
        unsafe impl Sync for PolyPressureEvent {}
        mod __ProcessContext_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::ProcessContext_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct ProcessContext {
                pub state: uint32,
                pub sampleRate: f64,
                pub projectTimeSamples: TSamples,
                pub systemTime: int64,
                pub continousTimeSamples: TSamples,
                pub projectTimeMusic: TQuarterNotes,
                pub barPositionMusic: TQuarterNotes,
                pub cycleStartMusic: TQuarterNotes,
                pub cycleEndMusic: TQuarterNotes,
                pub tempo: f64,
                pub timeSigNumerator: int32,
                pub timeSigDenominator: int32,
                pub chord: Chord,
                pub smpteOffsetSubframes: int32,
                pub frameRate: FrameRate,
                pub samplesToNextClock: int32,
            }
            unsafe impl Send for ProcessContext {}
            unsafe impl Sync for ProcessContext {}
        }
        pub use __ProcessContext_wrapper::*;
        pub mod ProcessContext_ {
            #[allow(unused_imports)]
            use super::*;
            pub type StatesAndFlags = crate::support::DefaultEnumType;
            pub mod StatesAndFlags_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kBarPositionValid: StatesAndFlags = 2048;
                pub const kChordValid: StatesAndFlags = 262144;
                pub const kClockValid: StatesAndFlags = 32768;
                pub const kContTimeValid: StatesAndFlags = 131072;
                pub const kCycleActive: StatesAndFlags = 4;
                pub const kCycleValid: StatesAndFlags = 4096;
                pub const kPlaying: StatesAndFlags = 2;
                pub const kProjectTimeMusicValid: StatesAndFlags = 512;
                pub const kRecording: StatesAndFlags = 8;
                pub const kSmpteValid: StatesAndFlags = 16384;
                pub const kSystemTimeValid: StatesAndFlags = 256;
                pub const kTempoValid: StatesAndFlags = 1024;
                pub const kTimeSigValid: StatesAndFlags = 8192;
            }
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct ProcessData {
            pub processMode: int32,
            pub symbolicSampleSize: int32,
            pub numSamples: int32,
            pub numInputs: int32,
            pub numOutputs: int32,
            pub inputs: *mut AudioBusBuffers,
            pub outputs: *mut AudioBusBuffers,
            pub inputParameterChanges: *mut IParameterChanges,
            pub outputParameterChanges: *mut IParameterChanges,
            pub inputEvents: *mut IEventList,
            pub outputEvents: *mut IEventList,
            pub processContext: *mut ProcessContext,
        }
        unsafe impl Send for ProcessData {}
        unsafe impl Sync for ProcessData {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct ProcessSetup {
            pub processMode: int32,
            pub symbolicSampleSize: int32,
            pub maxSamplesPerBlock: int32,
            pub sampleRate: SampleRate,
        }
        unsafe impl Send for ProcessSetup {}
        unsafe impl Sync for ProcessSetup {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct ProgramListInfo {
            pub id: ProgramListID,
            pub name: String128,
            pub programCount: int32,
        }
        unsafe impl Send for ProgramListInfo {}
        unsafe impl Sync for ProgramListInfo {}
        mod __RepresentationInfo_wrapper {
            #[allow(unused_imports)]
            use super::*;
            #[allow(unused_imports)]
            use super::RepresentationInfo_::*;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct RepresentationInfo {
                pub vendor: [char8; 64],
                pub name: [char8; 64],
                pub version: [char8; 64],
                pub host: [char8; 64],
            }
            unsafe impl Send for RepresentationInfo {}
            unsafe impl Sync for RepresentationInfo {}
        }
        pub use __RepresentationInfo_wrapper::*;
        pub mod RepresentationInfo_ {
            #[allow(unused_imports)]
            use super::*;
            pub const kNameSize: crate::support::DefaultEnumType = 64;
        }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct RoutingInfo {
            pub mediaType: MediaType,
            pub busIndex: int32,
            pub channel: int32,
        }
        unsafe impl Send for RoutingInfo {}
        unsafe impl Sync for RoutingInfo {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct ScaleEvent {
            pub root: int16,
            pub mask: int16,
            pub textLen: uint16,
            pub text: *const TChar,
        }
        unsafe impl Send for ScaleEvent {}
        unsafe impl Sync for ScaleEvent {}
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct UnitInfo {
            pub id: UnitID,
            pub parentUnitId: UnitID,
            pub name: String128,
            pub programListId: ProgramListID,
        }
        unsafe impl Send for UnitInfo {}
        unsafe impl Sync for UnitInfo {}
        pub const IAttributeList_iid: TUID = crate::support::uid(0x1E5F0AEB, 0xCC7F4533, 0xA2544011, 0x38AD5EE4);
        pub const IAudioPresentationLatency_iid: TUID = crate::support::uid(0x309ECE78, 0xEB7D4fae, 0x8B2225D9, 0x09FD08B6);
        pub const IAudioProcessor_iid: TUID = crate::support::uid(0x42043F99, 0xB7DA453C, 0xA569E79D, 0x9AAEC33D);
        pub const IAutomationState_iid: TUID = crate::support::uid(0xB4E8287F, 0x1BB346AA, 0x83A46667, 0x68937BAB);
        pub const IComponentHandler2_iid: TUID = crate::support::uid(0xF040B4B3, 0xA36045EC, 0xABCDC045, 0xB4D5A2CC);
        pub const IComponentHandler3_iid: TUID = crate::support::uid(0x69F11617, 0xD26B400D, 0xA4B6B964, 0x7B6EBBAB);
        pub const IComponentHandlerBusActivation_iid: TUID = crate::support::uid(0x067D02C1, 0x5B4E274D, 0xA92D90FD, 0x6EAF7240);
        pub const IComponentHandlerSystemTime_iid: TUID = crate::support::uid(0xF9E53056, 0xD1554CD5, 0xB7695E1B, 0x7B0F7745);
        pub const IComponentHandler_iid: TUID = crate::support::uid(0x93A0BEA3, 0x0BD045DB, 0x8E890B0C, 0xC1E46AC6);
        pub const IComponent_iid: TUID = crate::support::uid(0xE831FF31, 0xF2D54301, 0x928EBBEE, 0x25697802);
        pub const IConnectionPoint_iid: TUID = crate::support::uid(0x70A4156F, 0x6E6E4026, 0x989148BF, 0xAA60D8D1);
        pub const IContextMenuTarget_iid: TUID = crate::support::uid(0x3CDF2E75, 0x85D34144, 0xBF86D36B, 0xD7C4894D);
        pub const IContextMenu_iid: TUID = crate::support::uid(0x2E93C863, 0x0C9C4588, 0x97DBECF5, 0xAD17817D);
        pub const IDataExchangeHandler_iid: TUID = crate::support::uid(0x36D551BD, 0x6FF54F08, 0xB48E830D, 0x8BD5A03B);
        pub const IDataExchangeReceiver_iid: TUID = crate::support::uid(0x45A759DC, 0x84FA4907, 0xABCB6175, 0x2FC786B6);
        pub const IEditController2_iid: TUID = crate::support::uid(0x7F4EFE59, 0xF3204967, 0xAC27A3AE, 0xAFB63038);
        pub const IEditControllerHostEditing_iid: TUID = crate::support::uid(0xC1271208, 0x70594098, 0xB9DD34B3, 0x6BB0195E);
        pub const IEditController_iid: TUID = crate::support::uid(0xDCD7BBE3, 0x7742448D, 0xA874AACC, 0x979C759E);
        pub const IEventList_iid: TUID = crate::support::uid(0x3A2C4214, 0x346349FE, 0xB2C4F397, 0xB9695A44);
        pub const IHostApplication_iid: TUID = crate::support::uid(0x58E595CC, 0xDB2D4969, 0x8B6AAF8C, 0x36A664E5);
        pub const IInterAppAudioConnectionNotification_iid: TUID = crate::support::uid(0x6020C72D, 0x5FC24AA1, 0xB0950DB5, 0xD7D6D5CF);
        pub const IInterAppAudioHost_iid: TUID = crate::support::uid(0x0CE5743D, 0x68DF415E, 0xAE285BD4, 0xE2CDC8FD);
        pub const IInterAppAudioPresetManager_iid: TUID = crate::support::uid(0xADE6FCC4, 0x46C94E1D, 0xB3B49A80, 0xC93FEFDD);
        pub const IKeyswitchController_iid: TUID = crate::support::uid(0x1F2F76D3, 0xBFFB4B96, 0xB99527A5, 0x5EBCCEF4);
        pub const IMessage_iid: TUID = crate::support::uid(0x936F033B, 0xC6C047DB, 0xBB0882F8, 0x13C1E613);
        pub const IMidiLearn2_iid: TUID = crate::support::uid(0xF07E498A, 0x78864327, 0x8B431CED, 0xA3C553FC);
        pub const IMidiLearn_iid: TUID = crate::support::uid(0x6B2449CC, 0x419740B5, 0xAB3C79DA, 0xC5FE5C86);
        pub const IMidiMapping2_iid: TUID = crate::support::uid(0x6DE14B88, 0x03F94F09, 0xA2552F0F, 0x9326593E);
        pub const IMidiMapping_iid: TUID = crate::support::uid(0xDF0FF9F7, 0x49B74669, 0xB63AB732, 0x7ADBF5E5);
        pub const INoteExpressionController_iid: TUID = crate::support::uid(0xB7F8F859, 0x41234872, 0x91169581, 0x4F3721A3);
        pub const INoteExpressionPhysicalUIMapping_iid: TUID = crate::support::uid(0xB03078FF, 0x94D24AC8, 0x90CCD303, 0xD4133324);
        pub const IParamValueQueue_iid: TUID = crate::support::uid(0x01263A18, 0xED074F6F, 0x98C9D356, 0x4686F9BA);
        pub const IParameterChanges_iid: TUID = crate::support::uid(0xA4779663, 0x0BB64A56, 0xB44384A8, 0x466FEB9D);
        pub const IParameterFinder_iid: TUID = crate::support::uid(0x0F618302, 0x215D4587, 0xA512073C, 0x77B9D383);
        pub const IParameterFunctionName_iid: TUID = crate::support::uid(0x6D21E1DC, 0x91199D4B, 0xA2A02FEF, 0x6C1AE55C);
        pub const IPlugInterfaceSupport_iid: TUID = crate::support::uid(0x4FB58B9E, 0x9EAA4E0F, 0xAB361C1C, 0xCCB56FEA);
        pub const IPrefetchableSupport_iid: TUID = crate::support::uid(0x8AE54FDA, 0xE93046B9, 0xA28555BC, 0xDC98E21E);
        pub const IProcessContextRequirements_iid: TUID = crate::support::uid(0x2A654303, 0xEF764E3D, 0x95B5FE83, 0x730EF6D0);
        pub const IProgramListData_iid: TUID = crate::support::uid(0x8683B01F, 0x7B354F70, 0xA2651DEC, 0x353AF4FF);
        pub const IProgress_iid: TUID = crate::support::uid(0x00C9DC5B, 0x9D904254, 0x91A388C8, 0xB4E91B69);
        pub const IRemapParamID_iid: TUID = crate::support::uid(0x2B88021E, 0x6286B646, 0xB49DF76A, 0x5663061C);
        pub const IStreamAttributes_iid: TUID = crate::support::uid(0xD6CE2FFC, 0xEFAF4B8C, 0x9E74F1BB, 0x12DA44B4);
        pub const IUnitData_iid: TUID = crate::support::uid(0x6C389611, 0xD391455D, 0xB870B833, 0x94A0EFDD);
        pub const IUnitHandler2_iid: TUID = crate::support::uid(0xF89F8CDF, 0x699E4BA5, 0x96AAC9A4, 0x81452B01);
        pub const IUnitHandler_iid: TUID = crate::support::uid(0x4B5147F8, 0x4654486B, 0x8DAB30BA, 0x163A3C56);
        pub const IUnitInfo_iid: TUID = crate::support::uid(0x3D4BD6B5, 0x913A4FD2, 0xA886E768, 0xA5EB92C1);
        pub const IVst3ToAAXWrapper_iid: TUID = crate::support::uid(0x6D319DC6, 0x60C56242, 0xB32C951B, 0x93BEF4C6);
        pub const IVst3ToAUWrapper_iid: TUID = crate::support::uid(0xA3B8C6C5, 0xC0954688, 0xB0916F0B, 0xB697AA44);
        pub const IVst3ToVst2Wrapper_iid: TUID = crate::support::uid(0x29633AEC, 0x1D1C47E2, 0xBB85B97B, 0xD36EAC61);
        pub const IVst3WrapperMPESupport_iid: TUID = crate::support::uid(0x44149067, 0x42CF4BF9, 0x8800B750, 0xF7359FE3);
        pub const IXmlRepresentationController_iid: TUID = crate::support::uid(0xA81A0471, 0x48C34DC4, 0xAC30C9E1, 0x3C8393D5);
        pub const InvalidDataExchangeBlockID: DataExchangeBlockID = 2147483647;
        pub const InvalidDataExchangeQueueID: DataExchangeQueueID = 2147483647;
        pub const SDKVersion: uint32 = 198656;
        pub const SDKVersionMajor: uint32 = 3;
        pub const SDKVersionMinor: uint32 = 8;
        pub const SDKVersionString: FIDString = b"VST 3.8.0\0".as_ptr() as *const ::std::ffi::c_char;
        pub const SDKVersionSub: uint32 = 0;
        pub const SDKVersion_3_0_0: uint32 = 196608;
        pub const SDKVersion_3_1_0: uint32 = 196864;
        pub const SDKVersion_3_5_0: uint32 = 197888;
        pub const SDKVersion_3_6_0: uint32 = 198144;
        pub const SDKVersion_3_6_10: uint32 = 198154;
        pub const SDKVersion_3_6_11: uint32 = 198155;
        pub const SDKVersion_3_6_12: uint32 = 198156;
        pub const SDKVersion_3_6_13: uint32 = 198157;
        pub const SDKVersion_3_6_14: uint32 = 198158;
        pub const SDKVersion_3_6_5: uint32 = 198149;
        pub const SDKVersion_3_6_6: uint32 = 198150;
        pub const SDKVersion_3_6_7: uint32 = 198151;
        pub const SDKVersion_3_6_8: uint32 = 198152;
        pub const SDKVersion_3_6_9: uint32 = 198153;
        pub const SDKVersion_3_7_0: uint32 = 198400;
        pub const SDKVersion_3_7_1: uint32 = 198401;
        pub const SDKVersion_3_7_10: uint32 = 198410;
        pub const SDKVersion_3_7_11: uint32 = 198411;
        pub const SDKVersion_3_7_12: uint32 = 198412;
        pub const SDKVersion_3_7_13: uint32 = 198413;
        pub const SDKVersion_3_7_14: uint32 = 198414;
        pub const SDKVersion_3_7_2: uint32 = 198402;
        pub const SDKVersion_3_7_3: uint32 = 198403;
        pub const SDKVersion_3_7_4: uint32 = 198404;
        pub const SDKVersion_3_7_5: uint32 = 198405;
        pub const SDKVersion_3_7_6: uint32 = 198406;
        pub const SDKVersion_3_7_7: uint32 = 198407;
        pub const SDKVersion_3_7_8: uint32 = 198408;
        pub const SDKVersion_3_7_9: uint32 = 198409;
        pub const SDKVersion_3_8_0: uint32 = 198656;
        pub const kAllProgramInvalid: int32 = -1;
        pub const kDefaultFactoryFlags: int32 = 16;
        pub const kInfiniteTail: uint32 = 4294967295;
        pub const kMaxParamId: ParamID = 2147483647;
        pub const kMinParamId: ParamID = 0;
        pub const kNoParamId: ParamID = 4294967295;
        pub const kNoParentUnitId: UnitID = -1;
        pub const kNoProgramListId: ProgramListID = -1;
        pub const kNoTail: uint32 = 0;
        pub const kRootUnitId: UnitID = 0;
        pub const kSpeakerACN0: Speaker = 1048576;
        pub const kSpeakerACN1: Speaker = 2097152;
        pub const kSpeakerACN10: Speaker = 17592186044416;
        pub const kSpeakerACN11: Speaker = 35184372088832;
        pub const kSpeakerACN12: Speaker = 70368744177664;
        pub const kSpeakerACN13: Speaker = 140737488355328;
        pub const kSpeakerACN14: Speaker = 281474976710656;
        pub const kSpeakerACN15: Speaker = 562949953421312;
        pub const kSpeakerACN16: Speaker = 1125899906842624;
        pub const kSpeakerACN17: Speaker = 2251799813685248;
        pub const kSpeakerACN18: Speaker = 4503599627370496;
        pub const kSpeakerACN19: Speaker = 9007199254740992;
        pub const kSpeakerACN2: Speaker = 4194304;
        pub const kSpeakerACN20: Speaker = 18014398509481984;
        pub const kSpeakerACN21: Speaker = 36028797018963968;
        pub const kSpeakerACN22: Speaker = 72057594037927936;
        pub const kSpeakerACN23: Speaker = 144115188075855872;
        pub const kSpeakerACN24: Speaker = 288230376151711744;
        pub const kSpeakerACN3: Speaker = 8388608;
        pub const kSpeakerACN4: Speaker = 274877906944;
        pub const kSpeakerACN5: Speaker = 549755813888;
        pub const kSpeakerACN6: Speaker = 1099511627776;
        pub const kSpeakerACN7: Speaker = 2199023255552;
        pub const kSpeakerACN8: Speaker = 4398046511104;
        pub const kSpeakerACN9: Speaker = 8796093022208;
        pub const kSpeakerBfc: Speaker = 536870912;
        pub const kSpeakerBfl: Speaker = 268435456;
        pub const kSpeakerBfr: Speaker = 1073741824;
        pub const kSpeakerBrc: Speaker = 68719476736;
        pub const kSpeakerBrl: Speaker = 34359738368;
        pub const kSpeakerBrr: Speaker = 137438953472;
        pub const kSpeakerBsl: Speaker = 8589934592;
        pub const kSpeakerBsr: Speaker = 17179869184;
        pub const kSpeakerC: Speaker = 4;
        pub const kSpeakerCs: Speaker = 256;
        pub const kSpeakerL: Speaker = 1;
        pub const kSpeakerLc: Speaker = 64;
        pub const kSpeakerLcs: Speaker = 67108864;
        pub const kSpeakerLfe: Speaker = 8;
        pub const kSpeakerLfe2: Speaker = 262144;
        pub const kSpeakerLs: Speaker = 16;
        pub const kSpeakerLw: Speaker = 576460752303423488;
        pub const kSpeakerM: Speaker = 524288;
        pub const kSpeakerPl: Speaker = 2147483648;
        pub const kSpeakerPr: Speaker = 4294967296;
        pub const kSpeakerR: Speaker = 2;
        pub const kSpeakerRc: Speaker = 128;
        pub const kSpeakerRcs: Speaker = 134217728;
        pub const kSpeakerRs: Speaker = 32;
        pub const kSpeakerRw: Speaker = 1152921504606846976;
        pub const kSpeakerS: Speaker = 256;
        pub const kSpeakerSl: Speaker = 512;
        pub const kSpeakerSr: Speaker = 1024;
        pub const kSpeakerTc: Speaker = 2048;
        pub const kSpeakerTfc: Speaker = 8192;
        pub const kSpeakerTfl: Speaker = 4096;
        pub const kSpeakerTfr: Speaker = 16384;
        pub const kSpeakerTrc: Speaker = 65536;
        pub const kSpeakerTrl: Speaker = 32768;
        pub const kSpeakerTrr: Speaker = 131072;
        pub const kSpeakerTsl: Speaker = 16777216;
        pub const kSpeakerTsr: Speaker = 33554432;
        pub mod Attributes {
            #[allow(unused_imports)]
            use super::*;
            pub const kFlags: CString = b"flags\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFunction: CString = b"function\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kKnobTurnsPerFullRange: CString = b"turnsPerFullRange\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLEDStyle: CString = b"ledStyle\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStyle: CString = b"style\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSwitchStyle: CString = b"switchStyle\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod AttributesFlags {
            #[allow(unused_imports)]
            use super::*;
            pub const kHideableFlag: CString = b"hideable\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod AttributesFunction {
            #[allow(unused_imports)]
            use super::*;
            pub const kGainReductionFunc: CString = b"GainReduction\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMuteFunc: CString = b"Mute\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanLawFunc: CString = b"PanLaw\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanLfeGainFunc: CString = b"PanLfeGain\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanMirrorModeFunc: CString = b"PanMirrorMode\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanPosCenterXFunc: CString = b"PanPosCenterX\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanPosCenterYFunc: CString = b"PanPosCenterY\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanPosFrontLeftXFunc: CString = b"PanPosFrontLeftX\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanPosFrontLeftYFunc: CString = b"PanPosFrontLeftY\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanPosFrontRightXFunc: CString = b"PanPosFrontRightX\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanPosFrontRightYFunc: CString = b"PanPosFrontRightY\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanRotationFunc: CString = b"PanRotation\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSoloFunc: CString = b"Solo\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kVolumeFunc: CString = b"Volume\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod AttributesStyle {
            #[allow(unused_imports)]
            use super::*;
            pub const kInverseStyle: CString = b"inverse\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLEDBoostCutStyle: CString = b"boostCut\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLEDSingleDotStyle: CString = b"singleDot\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLEDSpreadStyle: CString = b"spread\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLEDWrapLeftStyle: CString = b"wrapLeft\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLEDWrapRightStyle: CString = b"wrapRight\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSwitchLatchStyle: CString = b"latch\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSwitchPushDecLoopedStyle: CString = b"pushDecLooped\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSwitchPushDecStyle: CString = b"pushDec\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSwitchPushIncLoopedStyle: CString = b"pushIncLooped\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSwitchPushIncStyle: CString = b"pushInc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSwitchPushStyle: CString = b"push\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod ChannelContext {
            #[allow(unused_imports)]
            use super::*;
            pub type ChannelPluginLocation = crate::support::DefaultEnumType;
            pub mod ChannelPluginLocation_ {
                #[allow(unused_imports)]
                use super::*;
                pub const kPostVolumeFader: ChannelPluginLocation = 1;
                pub const kPreVolumeFader: ChannelPluginLocation = 0;
                pub const kUsedAsPanner: ChannelPluginLocation = 2;
            }
            pub type ColorComponent = uint8;
            pub type ColorSpec = uint32;
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IInfoListener {
                pub vtbl: *const IInfoListenerVtbl,
            }
            unsafe impl Send for IInfoListener {}
            unsafe impl Sync for IInfoListener {}
            unsafe impl ::com_scrape_types::Inherits<FUnknown> for IInfoListener {}
            impl ::com_scrape_types::Unknown for IInfoListener {
                #[inline]
                unsafe fn query_interface(this: *mut Self, iid: &::com_scrape_types::Guid) -> Option<*mut ::std::ffi::c_void> {
                    crate::support::FUnknown_query_interface(this as *mut ::std::ffi::c_void, iid)
                }
                #[inline]
                unsafe fn add_ref(this: *mut Self) -> usize {
                    crate::support::FUnknown_add_ref(this as *mut ::std::ffi::c_void)
                }
                #[inline]
                unsafe fn release(this: *mut Self) -> usize {
                    crate::support::FUnknown_release(this as *mut ::std::ffi::c_void)
                }
            }
            unsafe impl ::com_scrape_types::Interface for IInfoListener {
                type Vtbl = IInfoListenerVtbl;
                const IID: ::com_scrape_types::Guid = crate::support::tuid_as_guid(IInfoListener_iid);
                #[inline]
                fn inherits(iid: &::com_scrape_types::Guid) -> bool {
                    iid == &Self::IID || FUnknown::inherits(iid)
                }
            }
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct IInfoListenerVtbl {
                pub base: FUnknownVtbl,
                pub setChannelContextInfos: unsafe extern "system" fn(
                    this: *mut IInfoListener,
                    list: *mut IAttributeList,
                ) -> tresult,
            }
            pub trait IInfoListenerTrait {
                unsafe fn setChannelContextInfos(
                    &self,
                    list: *mut IAttributeList,
                ) -> tresult;
            }
            impl<P> IInfoListenerTrait for P
            where
                P: ::com_scrape_types::SmartPtr,
                P::Target: ::com_scrape_types::Inherits<IInfoListener>,
            {
                #[inline]
                unsafe fn setChannelContextInfos(
                    &self,
                    list: *mut IAttributeList,
                ) -> tresult {
                    let ptr = self.ptr() as *mut IInfoListener;
                    ((*(*ptr).vtbl).setChannelContextInfos)(
                        ptr,
                        list,
                    )
                }
            }
            impl IInfoListener {
                const fn make_vtbl<C, W, const OFFSET: isize>() -> IInfoListenerVtbl
                where
                    C: IInfoListenerTrait + ::com_scrape_types::Class,
                    W: ::com_scrape_types::Wrapper<C>,
                {
                    unsafe extern "system" fn setChannelContextInfos<C, W, const OFFSET: isize>(
                        this: *mut IInfoListener,
                        list: *mut IAttributeList,
                    ) -> tresult
                    where
                        C: IInfoListenerTrait + ::com_scrape_types::Class,
                        W: ::com_scrape_types::Wrapper<C>,
                    {
                        let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut ::com_scrape_types::Header<C>;
                        let ptr = <W as ::com_scrape_types::Wrapper<C>>::data_from_header(header_ptr);
                        (*ptr).setChannelContextInfos(
                            list,
                        )
                    }
                    IInfoListenerVtbl {
                        base: FUnknown::make_vtbl::<C, W, OFFSET>(),
                        setChannelContextInfos: setChannelContextInfos::<C, W, OFFSET>,
                    }
                }
            }
            unsafe impl<C, W, const OFFSET: isize> ::com_scrape_types::Construct<C, W, OFFSET> for IInfoListener
            where
                C: IInfoListenerTrait + ::com_scrape_types::Class,
                W: ::com_scrape_types::Wrapper<C>,
            {
                const OBJ: Self = IInfoListener {
                    vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
                };
            }
            pub const IInfoListener_iid: TUID = crate::support::uid(0x0F194781, 0x8D984ADA, 0xBBA0C1EF, 0xC011D8D0);
            pub const kChannelColorKey: CString = b"channel color\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelImageKey: CString = b"channel image\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelIndexKey: CString = b"channel index\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelIndexNamespaceKey: CString = b"channel index namespace\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelIndexNamespaceLengthKey: CString = b"channel index namespace length\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelIndexNamespaceOrderKey: CString = b"channel index namespace order\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelNameKey: CString = b"channel name\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelNameLengthKey: CString = b"channel name length\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelPluginLocationKey: CString = b"channel plugin location\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelRuntimeIDKey: CString = b"channel runtime id\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelUIDKey: CString = b"channel uid\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChannelUIDLengthKey: CString = b"channel uid length\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod CurveType {
            #[allow(unused_imports)]
            use super::*;
            pub const kSegment: CString = b"segment\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kValueList: CString = b"valueList\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod FunctionNameType {
            #[allow(unused_imports)]
            use super::*;
            pub const kCompGainReduction: CString = b"Comp:GainReduction\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCompGainReductionMax: CString = b"Comp:GainReductionMax\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCompGainReductionPeakHold: CString = b"Comp:GainReductionPeakHold\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCompResetGainReductionMax: CString = b"Comp:ResetGainReductionMax\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDryWetMix: CString = b"DryWetMix\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLowLatencyMode: CString = b"LowLatencyMode\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanPosCenterX: CString = b"PanPosCenterX\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanPosCenterY: CString = b"PanPosCenterY\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPanPosCenterZ: CString = b"PanPosCenterZ\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRandomize: CString = b"Randomize\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod LayerType {
            #[allow(unused_imports)]
            use super::*;
            pub const kDisplay: crate::support::DefaultEnumType = 6;
            pub const kEndOfLayerType: crate::support::DefaultEnumType = 8;
            pub const kFader: crate::support::DefaultEnumType = 7;
            pub const kKnob: crate::support::DefaultEnumType = 0;
            pub const kLED: crate::support::DefaultEnumType = 4;
            pub const kLink: crate::support::DefaultEnumType = 5;
            pub const kPressedKnob: crate::support::DefaultEnumType = 1;
            pub const kSwitch: crate::support::DefaultEnumType = 3;
            pub const kSwitchKnob: crate::support::DefaultEnumType = 2;
        }
        pub mod MusicalCharacter {
            #[allow(unused_imports)]
            use super::*;
            pub const kAcoustic: CString = b"Acoustic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAnalog: CString = b"Analog\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAttack: CString = b"Attack\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBright: CString = b"Bright\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClean: CString = b"Clean\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClear: CString = b"Clear\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCold: CString = b"Cold\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDark: CString = b"Dark\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDecay: CString = b"Decay\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDigital: CString = b"Digital\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDissonant: CString = b"Dissonant\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDistorted: CString = b"Distorted\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDry: CString = b"Dry\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectric: CString = b"Electric\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kEnsemble: CString = b"Ensemble\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFast: CString = b"Fast\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFastAttack: CString = b"Fast Attack\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kGlass: CString = b"Glass\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kGlide: CString = b"Glide\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kGlissando: CString = b"Glissando\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kHarmonic: CString = b"Harmonic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLayer: CString = b"Layer\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLong: CString = b"Long\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLongRelease: CString = b"Long Release\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kLoop: CString = b"Loop\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMajor: CString = b"Major\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMetallic: CString = b"Metallic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMinor: CString = b"Minor\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kModern: CString = b"Modern\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMono: CString = b"Mono\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMoving: CString = b"Moving\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kNew: CString = b"New\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kNoisy: CString = b"Noisy\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kOld: CString = b"Old\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kOneShot: CString = b"One Shot\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPercussive: CString = b"Percussive\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPlastic: CString = b"Plastic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPoly: CString = b"Poly\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kProcessed: CString = b"Processed\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRelease: CString = b"Release\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRich: CString = b"Rich\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kShort: CString = b"Short\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kShortRelease: CString = b"Short Release\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSingle: CString = b"Single\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSlow: CString = b"Slow\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSlowAttack: CString = b"Slow Attack\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSoft: CString = b"Soft\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSplit: CString = b"Split\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStatic: CString = b"Static\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSustain: CString = b"Sustain\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kThin: CString = b"Thin\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kVintage: CString = b"Vintage\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWarm: CString = b"Warm\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWooden: CString = b"Wooden\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod MusicalInstrument {
            #[allow(unused_imports)]
            use super::*;
            pub const kAccordion: CString = b"Accordion\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAccordionAccordion: CString = b"Accordion|Accordion\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAccordionHarmonica: CString = b"Accordion|Harmonica\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAccordionOther: CString = b"Accordion|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBass: CString = b"Bass\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBassABass: CString = b"Bass|A. Bass\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBassEBass: CString = b"Bass|E. Bass\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBassOther: CString = b"Bass|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBassSynthBass: CString = b"Bass|Synth Bass\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBrass: CString = b"Brass\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBrassFrenchHorn: CString = b"Brass|French Horn\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBrassOther: CString = b"Brass|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBrassSection: CString = b"Brass|Section\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBrassSynth: CString = b"Brass|Synth\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBrassTrombone: CString = b"Brass|Trombone\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBrassTrumpet: CString = b"Brass|Trumpet\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBrassTuba: CString = b"Brass|Tuba\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChromaticPerc: CString = b"Chromatic Perc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChromaticPercBell: CString = b"Chromatic Perc|Bell\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChromaticPercMallett: CString = b"Chromatic Perc|Mallett\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChromaticPercOther: CString = b"Chromatic Perc|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChromaticPercPercussion: CString = b"Chromatic Perc|Percussion\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChromaticPercTimpani: CString = b"Chromatic Perc|Timpani\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kChromaticPercWood: CString = b"Chromatic Perc|Wood\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPerc: CString = b"Drum&Perc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercBeats: CString = b"Drum&Perc|Beats\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercCymbals: CString = b"Drum&Perc|Cymbals\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercDrumMenues: CString = b"Drum&Perc|Drum Menues\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercDrumset: CString = b"Drum&Perc|Drumset\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercDrumsetGM: CString = b"Drum&Perc|Drumset GM\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercHiHats: CString = b"Drum&Perc|HiHats\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercKickDrum: CString = b"Drum&Perc|Kick Drum\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercOther: CString = b"Drum&Perc|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercPercussion: CString = b"Drum&Perc|Percussion\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercSnareDrum: CString = b"Drum&Perc|Snare Drum\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kDrumPercToms: CString = b"Drum&Perc|Toms\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kEthnic: CString = b"Ethnic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kEthnicAfrican: CString = b"Ethnic|African\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kEthnicAlien: CString = b"Ethnic|Alien\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kEthnicAmerican: CString = b"Ethnic|American\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kEthnicAsian: CString = b"Ethnic|Asian\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kEthnicEuropean: CString = b"Ethnic|European\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kEthnicLatin: CString = b"Ethnic|Latin\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kEthnicOther: CString = b"Ethnic|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kGuitar: CString = b"Guitar/Plucked\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kGuitarAGuitar: CString = b"Guitar/Plucked|A. Guitar\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kGuitarEGuitar: CString = b"Guitar/Plucked|E. Guitar\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kGuitarEthnic: CString = b"Guitar/Plucked|Ethnic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kGuitarHarp: CString = b"Guitar/Plucked|Harp\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kGuitarOther: CString = b"Guitar/Plucked|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kKeyboard: CString = b"Keyboard\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kKeyboardClavi: CString = b"Keyboard|Clavi\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kKeyboardEPiano: CString = b"Keyboard|E. Piano\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kKeyboardHarpsichord: CString = b"Keyboard|Harpsichord\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kKeyboardOther: CString = b"Keyboard|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMusicalFX: CString = b"Musical FX\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMusicalFXBeepsBlips: CString = b"Musical FX|Beeps&Blips\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMusicalFXHitsStabs: CString = b"Musical FX|Hits&Stabs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMusicalFXMotion: CString = b"Musical FX|Motion\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMusicalFXOther: CString = b"Musical FX|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMusicalFXScratches: CString = b"Musical FX|Scratches\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMusicalFXSweeps: CString = b"Musical FX|Sweeps\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kOrgan: CString = b"Organ\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kOrganElectric: CString = b"Organ|Electric\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kOrganOther: CString = b"Organ|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kOrganPipe: CString = b"Organ|Pipe\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPiano: CString = b"Piano\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPianoAPiano: CString = b"Piano|A. Piano\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPianoEGrand: CString = b"Piano|E. Grand\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPianoOther: CString = b"Piano|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSoundFX: CString = b"Sound FX\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSoundFXMechanical: CString = b"Sound FX|Mechanical\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSoundFXNature: CString = b"Sound FX|Nature\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSoundFXOther: CString = b"Sound FX|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSoundFXSynthetic: CString = b"Sound FX|Synthetic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStrings: CString = b"Strings\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringsBass: CString = b"Strings|Bass\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringsCello: CString = b"Strings|Cello\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringsOther: CString = b"Strings|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringsSection: CString = b"Strings|Section\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringsSynth: CString = b"Strings|Synth\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringsViola: CString = b"Strings|Viola\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringsViolin: CString = b"Strings|Violin\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthComp: CString = b"Synth Comp\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthCompAnalog: CString = b"Synth Comp|Analog\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthCompDigital: CString = b"Synth Comp|Digital\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthCompOther: CString = b"Synth Comp|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthLead: CString = b"Synth Lead\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthLeadAnalog: CString = b"Synth Lead|Analog\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthLeadArpeggio: CString = b"Synth Lead|Arpeggio\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthLeadDigital: CString = b"Synth Lead|Digital\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthLeadOther: CString = b"Synth Lead|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthPad: CString = b"Synth Pad\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthPadAnalog: CString = b"Synth Pad|Analog\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthPadDigital: CString = b"Synth Pad|Digital\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthPadMotion: CString = b"Synth Pad|Motion\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthPadOther: CString = b"Synth Pad|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSynthPadSynthChoir: CString = b"Synth Pad|Synth Choir\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kVocal: CString = b"Vocal\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kVocalAdlibs: CString = b"Vocal|Adlibs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kVocalChoir: CString = b"Vocal|Choir\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kVocalFX: CString = b"Vocal|FX\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kVocalLeadVocal: CString = b"Vocal|Lead Vocal\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kVocalOther: CString = b"Vocal|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kVocalSolo: CString = b"Vocal|Solo\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kVocalSpoken: CString = b"Vocal|Spoken\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWoodwinds: CString = b"Woodwinds\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWoodwindsBassoon: CString = b"Woodwinds|Bassoon\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWoodwindsClarinet: CString = b"Woodwinds|Clarinet\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWoodwindsEnglHorn: CString = b"Woodwinds|Engl. Horn\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWoodwindsEthnic: CString = b"Woodwinds|Ethnic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWoodwindsFlute: CString = b"Woodwinds|Flute\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWoodwindsOboe: CString = b"Woodwinds|Oboe\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWoodwindsOther: CString = b"Woodwinds|Other\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWoodwindsSaxophone: CString = b"Woodwinds|Saxophone\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod MusicalStyle {
            #[allow(unused_imports)]
            use super::*;
            pub const kAlternativeIndie: CString = b"Alternative/Indie\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAlternativeIndieCollegeRock: CString = b"Alternative/Indie|College Rock\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAlternativeIndieDarkWave: CString = b"Alternative/Indie|Dark Wave\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAlternativeIndieGothRock: CString = b"Alternative/Indie|Goth Rock\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAlternativeIndieGrunge: CString = b"Alternative/Indie|Grunge\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAlternativeIndieHardcore: CString = b"Alternative/Indie|Hardcore\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAlternativeIndieNewWave: CString = b"Alternative/Indie|New Wave\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAlternativeIndiePunk: CString = b"Alternative/Indie|Punk\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAmbientChillOut: CString = b"Ambient/ChillOut\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAmbientChillOutDarkAmbient: CString = b"Ambient/ChillOut|Dark Ambient\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAmbientChillOutDowntempo: CString = b"Ambient/ChillOut|Downtempo\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAmbientChillOutLounge: CString = b"Ambient/ChillOut|Lounge\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAmbientChillOutNewAgeMeditation: CString = b"Ambient/ChillOut|New Age/Meditation\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBlues: CString = b"Blues\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBluesAcousticBlues: CString = b"Blues|Acoustic Blues\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBluesChicagoBlues: CString = b"Blues|Chicago Blues\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBluesCountryBlues: CString = b"Blues|Country Blues\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kBluesElectricBlues: CString = b"Blues|Electric Blues\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassical: CString = b"Classical\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassicalBaroque: CString = b"Classical|Baroque\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassicalChamberMusic: CString = b"Classical|Chamber Music\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassicalClassic: CString = b"Classical|Classic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassicalGregorian: CString = b"Classical|Gregorian\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassicalMedieval: CString = b"Classical|Medieval\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassicalModernComposition: CString = b"Classical|Modern Composition\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassicalOpera: CString = b"Classical|Opera\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassicalRenaissance: CString = b"Classical|Renaissance\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassicalRomantic: CString = b"Classical|Romantic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kClassicalSoundtrack: CString = b"Classical|Soundtrack\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCountry: CString = b"Country\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCountryAmericana: CString = b"Country|Americana\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCountryBluegrass: CString = b"Country|Bluegrass\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCountryCountryWestern: CString = b"Country|Country/Western\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCountryHonkyTonk: CString = b"Country|Honky Tonk\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCountryNorthAmericanFolk: CString = b"Country|North American Folk\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCountrySquaredance: CString = b"Country|Squaredance\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kCountryUrbanCowboy: CString = b"Country|Urban Cowboy\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDance: CString = b"Electronica/Dance\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceBigBeats: CString = b"Electronica/Dance|Big Beats\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceClassicHouse: CString = b"Electronica/Dance|Classic House\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceDrumNBassJungle: CString = b"Electronica/Dance|Drum'n'Bass/Jungle\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceDub: CString = b"Electronica/Dance|Dub\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceElectronicBodyMusic: CString = b"Electronica/Dance|Electronic Body Music\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceElektro: CString = b"Electronica/Dance|Elektro\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceElektroHouse: CString = b"Electronica/Dance|Elektro House\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceFunkyHouse: CString = b"Electronica/Dance|Funky House\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceIndustrial: CString = b"Electronica/Dance|Industrial\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceMinimal: CString = b"Electronica/Dance|Minimal\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceTechno: CString = b"Electronica/Dance|Techno\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceTrance: CString = b"Electronica/Dance|Trance\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kElectronicaDanceTripHop: CString = b"Electronica/Dance|Trip Hop\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kExperimental: CString = b"Experimental\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kExperimentalElectronicArtMusic: CString = b"Experimental|Electronic Art Music\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kExperimentalFreeImprovisation: CString = b"Experimental|Free Improvisation\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kExperimentalNewMusic: CString = b"Experimental|New Music\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kExperimentalNoise: CString = b"Experimental|Noise\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kJazz: CString = b"Jazz\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kJazzAvantgarde: CString = b"Jazz|Avantgarde\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kJazzFreeJazz: CString = b"Jazz|Free Jazz\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kJazzFusion: CString = b"Jazz|Fusion\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kJazzLatinJazz: CString = b"Jazz|Latin Jazz\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kJazzNewOrleansJazz: CString = b"Jazz|New Orleans Jazz\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kJazzOldtimeJazzDixiland: CString = b"Jazz|Oldtime Jazz/Dixiland\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kJazzRagtime: CString = b"Jazz|Ragtime\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kJazzTraditionalJazz: CString = b"Jazz|Traditional Jazz\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPop: CString = b"Pop\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPop80sPop: CString = b"Pop|80's Pop\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPopBritpop: CString = b"Pop|Britpop\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPopChartDance: CString = b"Pop|Chart Dance\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPopDancehall: CString = b"Pop|Dancehall\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPopDisco: CString = b"Pop|Disco\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPopRock: CString = b"Pop|Pop/Rock\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPopTeenPop: CString = b"Pop|Teen Pop\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetal: CString = b"Rock/Metal\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalAlternativeRock: CString = b"Rock/Metal|Alternative Rock\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalBallad: CString = b"Rock/Metal|Ballad\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalBluesRock: CString = b"Rock/Metal|Blues Rock\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalClassicRock: CString = b"Rock/Metal|Classic Rock\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalDeathBlackMetal: CString = b"Rock/Metal|Death/Black Metal\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalHardRock: CString = b"Rock/Metal|Hard Rock\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalHeavyMetal: CString = b"Rock/Metal|Heavy Metal\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalNuMetal: CString = b"Rock/Metal|NuMetal\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalProgressiveRock: CString = b"Rock/Metal|Progressive Rock\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalReggae: CString = b"Rock/Metal|Reggae\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalRockRoll: CString = b"Rock/Metal|Rock &amp; Roll\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalRockabilly: CString = b"Rock/Metal|Rockabilly\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalSingerSongwriter: CString = b"Rock/Metal|Singer/Songwriter\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kRockMetalThrashMetal: CString = b"Rock/Metal|Thrash Metal\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kUrbanHipHopRB: CString = b"Urban (Hip-Hop / R&B)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kUrbanHipHopRBClassic: CString = b"Urban (Hip-Hop / R&B)|Classic R&B\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kUrbanHipHopRBEastCoastHipHop: CString = b"Urban (Hip-Hop / R&B)|EastCoast Hip-Hop\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kUrbanHipHopRBFunk: CString = b"Urban (Hip-Hop / R&B)|Funk\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kUrbanHipHopRBModern: CString = b"Urban (Hip-Hop / R&B)|Modern R&B\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kUrbanHipHopRBPop: CString = b"Urban (Hip-Hop / R&B)|R&B Pop\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kUrbanHipHopRBRapHipHop: CString = b"Urban (Hip-Hop / R&B)|Rap/Hip Hop\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kUrbanHipHopRBSoul: CString = b"Urban (Hip-Hop / R&B)|Soul\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kUrbanHipHopRBWestCoastHipHop: CString = b"Urban (Hip-Hop / R&B)|WestCoast Hip-Hop\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnic: CString = b"World/Ethnic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicAfrica: CString = b"World/Ethnic|Africa\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicAsia: CString = b"World/Ethnic|Asia\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicAustralia: CString = b"World/Ethnic|Australia\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicCeltic: CString = b"World/Ethnic|Celtic\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicEasternEurope: CString = b"World/Ethnic|Eastern Europe\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicEurope: CString = b"World/Ethnic|Europe\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicIndiaOriental: CString = b"World/Ethnic|India/Oriental\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicKlezmer: CString = b"World/Ethnic|Klezmer\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicNorthAmerica: CString = b"World/Ethnic|North America\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicScandinavia: CString = b"World/Ethnic|Scandinavia\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kWorldEthnicSouthAmerica: CString = b"World/Ethnic|South America\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod PlugType {
            #[allow(unused_imports)]
            use super::*;
            pub const kAmbisonics: CString = b"Ambisonics\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kAnalyzer: CString = b"Analyzer\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFx: CString = b"Fx\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxAnalyzer: CString = b"Fx|Analyzer\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxBass: CString = b"Fx|Bass\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxChannelStrip: CString = b"Fx|Channel Strip\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxDelay: CString = b"Fx|Delay\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxDistortion: CString = b"Fx|Distortion\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxDrums: CString = b"Fx|Drums\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxDynamics: CString = b"Fx|Dynamics\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxEQ: CString = b"Fx|EQ\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxFilter: CString = b"Fx|Filter\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxGenerator: CString = b"Fx|Generator\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxGuitar: CString = b"Fx|Guitar\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxInstrument: CString = b"Fx|Instrument\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxInstrumentExternal: CString = b"Fx|Instrument|External\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxMastering: CString = b"Fx|Mastering\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxMicrophone: CString = b"Fx|Microphone\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxModulation: CString = b"Fx|Modulation\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxNetwork: CString = b"Fx|Network\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxPitchShift: CString = b"Fx|Pitch Shift\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxRestoration: CString = b"Fx|Restoration\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxReverb: CString = b"Fx|Reverb\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxSpatial: CString = b"Fx|Spatial\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxSurround: CString = b"Fx|Surround\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxTools: CString = b"Fx|Tools\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFxVocals: CString = b"Fx|Vocals\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kInstrument: CString = b"Instrument\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kInstrumentDrum: CString = b"Instrument|Drum\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kInstrumentExternal: CString = b"Instrument|External\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kInstrumentPiano: CString = b"Instrument|Piano\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kInstrumentSampler: CString = b"Instrument|Sampler\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kInstrumentSynth: CString = b"Instrument|Synth\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kInstrumentSynthSampler: CString = b"Instrument|Synth|Sampler\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kMono: CString = b"Mono\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kNoOfflineProcess: CString = b"NoOfflineProcess\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kOnlyARA: CString = b"OnlyARA\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kOnlyOfflineProcess: CString = b"OnlyOfflineProcess\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kOnlyRealTime: CString = b"OnlyRT\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSpatial: CString = b"Spatial\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSpatialFx: CString = b"Spatial|Fx\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStereo: CString = b"Stereo\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kSurround: CString = b"Surround\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kUpDownMix: CString = b"Up-Downmix\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod PresetAttributes {
            #[allow(unused_imports)]
            use super::*;
            pub const kCharacter: CString = b"MusicalCharacter\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFileName: CString = b"FileName\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kFilePathStringType: CString = b"FilePathString\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kInstrument: CString = b"MusicalInstrument\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kName: CString = b"Name\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPlugInCategory: CString = b"PlugInCategory\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kPlugInName: CString = b"PlugInName\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStateType: CString = b"StateType\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStyle: CString = b"MusicalStyle\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod SpeakerArr {
            #[allow(unused_imports)]
            use super::*;
            pub const k100: SpeakerArrangement = 186423;
            pub const k100Cine: SpeakerArrangement = 2039;
            pub const k100_9_3: SpeakerArrangement = 1929641975;
            pub const k101: SpeakerArrangement = 186431;
            pub const k101Cine: SpeakerArrangement = 2047;
            pub const k101MPEG3D: SpeakerArrangement = 186431;
            pub const k102: SpeakerArrangement = 454719;
            pub const k102_9_3: SpeakerArrangement = 1929904127;
            pub const k110: SpeakerArrangement = 194615;
            pub const k111: SpeakerArrangement = 194623;
            pub const k111MPEG3D: SpeakerArrangement = 185919;
            pub const k122: SpeakerArrangement = 454911;
            pub const k130: SpeakerArrangement = 196151;
            pub const k131: SpeakerArrangement = 196159;
            pub const k140: SpeakerArrangement = 173141055027;
            pub const k220: SpeakerArrangement = 1929641975;
            pub const k222: SpeakerArrangement = 1929904127;
            pub const k30Cine: SpeakerArrangement = 7;
            pub const k30Music: SpeakerArrangement = 259;
            pub const k30_5_2: SpeakerArrangement = 1342369799;
            pub const k31Cine: SpeakerArrangement = 15;
            pub const k31Music: SpeakerArrangement = 267;
            pub const k40Cine: SpeakerArrangement = 263;
            pub const k40Music: SpeakerArrangement = 51;
            pub const k40_2_2: SpeakerArrangement = 25820137220;
            pub const k40_4: SpeakerArrangement = 184371;
            pub const k40_4_2: SpeakerArrangement = 1342361651;
            pub const k40_4_4: SpeakerArrangement = 173141053491;
            pub const k41Cine: SpeakerArrangement = 271;
            pub const k41Music: SpeakerArrangement = 59;
            pub const k41_4_1: SpeakerArrangement = 536899643;
            pub const k50: SpeakerArrangement = 55;
            pub const k50_2: SpeakerArrangement = 20535;
            pub const k50_2_2: SpeakerArrangement = 1392508983;
            pub const k50_2_TS: SpeakerArrangement = 50331703;
            pub const k50_3_2: SpeakerArrangement = 1342206007;
            pub const k50_4: SpeakerArrangement = 184375;
            pub const k50_4_1: SpeakerArrangement = 537055287;
            pub const k50_4_2: SpeakerArrangement = 1342361655;
            pub const k50_4_4: SpeakerArrangement = 173141053495;
            pub const k50_5: SpeakerArrangement = 186423;
            pub const k50_5_3: SpeakerArrangement = 1879240759;
            pub const k50_5_Sony: SpeakerArrangement = 192567;
            pub const k50_6: SpeakerArrangement = 194615;
            pub const k51: SpeakerArrangement = 63;
            pub const k51_2: SpeakerArrangement = 20543;
            pub const k51_2_TS: SpeakerArrangement = 50331711;
            pub const k51_4: SpeakerArrangement = 184383;
            pub const k51_4_1: SpeakerArrangement = 537055295;
            pub const k51_5: SpeakerArrangement = 186431;
            pub const k51_5_3: SpeakerArrangement = 1879240767;
            pub const k51_6: SpeakerArrangement = 194623;
            pub const k52_5: SpeakerArrangement = 454719;
            pub const k60Cine: SpeakerArrangement = 311;
            pub const k60Music: SpeakerArrangement = 1587;
            pub const k60_4_4: SpeakerArrangement = 173141055027;
            pub const k61Cine: SpeakerArrangement = 319;
            pub const k61Music: SpeakerArrangement = 1595;
            pub const k70Cine: SpeakerArrangement = 247;
            pub const k70CineFrontHigh: SpeakerArrangement = 20535;
            pub const k70CineSideHigh: SpeakerArrangement = 50331703;
            pub const k70MPEG3D: SpeakerArrangement = 20535;
            pub const k70Music: SpeakerArrangement = 1591;
            pub const k70_2: SpeakerArrangement = 50333239;
            pub const k70_2_TF: SpeakerArrangement = 22071;
            pub const k70_3: SpeakerArrangement = 87607;
            pub const k70_4: SpeakerArrangement = 185911;
            pub const k70_4_2: SpeakerArrangement = 1342363191;
            pub const k70_6: SpeakerArrangement = 50517559;
            pub const k71Cine: SpeakerArrangement = 255;
            pub const k71CineCenterHigh: SpeakerArrangement = 8511;
            pub const k71CineFrontHigh: SpeakerArrangement = 20543;
            pub const k71CineFullFront: SpeakerArrangement = 255;
            pub const k71CineFullRear: SpeakerArrangement = 201326655;
            pub const k71CineSideFill: SpeakerArrangement = 1599;
            pub const k71CineSideHigh: SpeakerArrangement = 50331711;
            pub const k71CineTopCenter: SpeakerArrangement = 2367;
            pub const k71MPEG3D: SpeakerArrangement = 20543;
            pub const k71Music: SpeakerArrangement = 1599;
            pub const k71Proximity: SpeakerArrangement = 6442451007;
            pub const k71_2: SpeakerArrangement = 50333247;
            pub const k71_2_TF: SpeakerArrangement = 22079;
            pub const k71_4: SpeakerArrangement = 185919;
            pub const k71_6: SpeakerArrangement = 50517567;
            pub const k72_3: SpeakerArrangement = 349759;
            pub const k72_5: SpeakerArrangement = 454911;
            pub const k80Cine: SpeakerArrangement = 503;
            pub const k80Cube: SpeakerArrangement = 184371;
            pub const k80Music: SpeakerArrangement = 1847;
            pub const k81Cine: SpeakerArrangement = 511;
            pub const k81MPEG3D: SpeakerArrangement = 536899643;
            pub const k81Music: SpeakerArrangement = 1855;
            pub const k90: SpeakerArrangement = 184375;
            pub const k90Cine: SpeakerArrangement = 1783;
            pub const k90_4: SpeakerArrangement = 186103;
            pub const k90_4_W: SpeakerArrangement = 1729382256910456375;
            pub const k90_6: SpeakerArrangement = 50517751;
            pub const k90_6_W: SpeakerArrangement = 1729382256960788023;
            pub const k91: SpeakerArrangement = 184383;
            pub const k91Atmos: SpeakerArrangement = 50333247;
            pub const k91Cine: SpeakerArrangement = 1791;
            pub const k91_4: SpeakerArrangement = 186111;
            pub const k91_4_W: SpeakerArrangement = 1729382256910456383;
            pub const k91_6: SpeakerArrangement = 50517759;
            pub const k91_6_W: SpeakerArrangement = 1729382256960788031;
            pub const kAmbi1stOrderACN: SpeakerArrangement = 15728640;
            pub const kAmbi2cdOrderACN: SpeakerArrangement = 8521230843904;
            pub const kAmbi3rdOrderACN: SpeakerArrangement = 1125625044664320;
            pub const kAmbi4thOrderACN: SpeakerArrangement = 576460477441245184;
            pub const kAmbi5thOrderACN: SpeakerArrangement = 68719476735;
            pub const kAmbi6thOrderACN: SpeakerArrangement = 562949953421311;
            pub const kAmbi7thOrderACN: SpeakerArrangement = 18446744073709551615;
            pub const kCineFront: SpeakerArrangement = 199;
            pub const kEmpty: SpeakerArrangement = 0;
            pub const kMono: SpeakerArrangement = 524288;
            pub const kStereo: SpeakerArrangement = 3;
            pub const kStereoBF: SpeakerArrangement = 1342177280;
            pub const kStereoCLfe: SpeakerArrangement = 12;
            pub const kStereoCenter: SpeakerArrangement = 192;
            pub const kStereoSide: SpeakerArrangement = 1536;
            pub const kStereoSurround: SpeakerArrangement = 48;
            pub const kStereoTF: SpeakerArrangement = 20480;
            pub const kStereoTR: SpeakerArrangement = 163840;
            pub const kStereoTS: SpeakerArrangement = 50331648;
            pub const kStereoWide: SpeakerArrangement = 1729382256910270464;
            pub const kString100Cine: CString = b"10.0 Cine\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString100CineS: CString = b"L R C Ls Rs Lc Rc Cs Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString101Cine: CString = b"10.1 Cine\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString101CineS: CString = b"L R C LFE Ls Rs Lc Rc Cs Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString130: CString = b"13.0 Auro-3D\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString130S: CString = b"L R C Ls Rs Sl Sr Tc Tfl Tfc Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString131: CString = b"13.1 Auro-3D\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString131S: CString = b"L R C LFE Ls Rs Sl Sr Tc Tfl Tfc Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString220: CString = b"22.0\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString220S: CString = b"L R C Ls Rs Lc Rc Cs Sl Sr Tc Tfl Tfc Tfr Trl Trc Trr Tsl Tsr Bfl Bfc Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString222: CString = b"22.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString222S: CString = b"L R C LFE Ls Rs Lc Rc Cs Sl Sr Tc Tfl Tfc Tfr Trl Trc Trr LFE2 Tsl Tsr Bfl Bfc Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString30Cine: CString = b"LRC\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString30CineS: CString = b"L R C\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString30Music: CString = b"LRS\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString30MusicS: CString = b"L R S\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString30_5_2: CString = b"3.0.5.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString30_5_2S: CString = b"L R C Tfl Tfc Tfr Trl Trr Bfl Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString31Cine: CString = b"LRC+LFE\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString31CineS: CString = b"L R C LFE\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString31Music: CString = b"LRS+LFE\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString31MusicS: CString = b"L R LFE S\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40Cine: CString = b"LRCS\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40CineS: CString = b"L R C S\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40Music: CString = b"Quadro\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40MusicS: CString = b"L R Ls Rs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40_2_2: CString = b"4.0.3.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40_2_2S: CString = b"C Sl Sr Cs Tfc Tsl Tsr Trc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40_4: CString = b"8.0 Cube\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40_4S: CString = b"L R Ls Rs Tfl Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40_4_2: CString = b"4.0.4.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40_4_2S: CString = b"L R Ls Rs Tfl Tfr Trl Trr Bfl Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40_4_4: CString = b"4.0.4.4\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString40_4_4S: CString = b"L R Ls Rs Tfl Tfr Trl Trr Bfl Bfr Brl Brr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString41Cine: CString = b"LRCS+LFE\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString41CineS: CString = b"L R C LFE S\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString41Music: CString = b"Quadro+LFE\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString41MusicS: CString = b"L R LFE Ls Rs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString41_4_1: CString = b"8.1 MPEG\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString41_4_1S: CString = b"L R LFE Ls Rs Tfl Tfc Tfr Bfc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50: CString = b"5.0\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50S: CString = b"L R C Ls Rs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_2: CString = b"5.0.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_2S: CString = b"L R C Ls Rs Tfl Tfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_2TopSide: CString = b"5.0.2 Top Side\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_2TopSideS: CString = b"L R C Ls Rs Tsl Tsr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_2_2: CString = b"5.0.2.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_2_2S: CString = b"L R C Ls Rs Tsl Tsr Bfl Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_3_2: CString = b"5.0.3.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_3_2S: CString = b"L R C Ls Rs Tfl Tfc Tfr Bfl Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_4: CString = b"5.0.4\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_4S: CString = b"L R C Ls Rs Tfl Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_4_1: CString = b"5.0.4.1\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_4_1S: CString = b"L R C Ls Rs Tfl Tfr Trl Trr Bfc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_4_2: CString = b"5.0.4.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_4_2S: CString = b"L R C Ls Rs Tfl Tfr Trl Trr Bfl Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_4_4: CString = b"5.0.4.4\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_4_4S: CString = b"L R C Ls Rs Tfl Tfr Trl Trr Bfl Bfr Brl Brr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_5: CString = b"10.0 Auro-3D\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_5S: CString = b"L R C Ls Rs Tc Tfl Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_5_3: CString = b"5.0.5.3\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_5_3S: CString = b"L R C Ls Rs Tfl Tfc Tfr Trl Trr Bfl Bfc Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_5_Sony: CString = b"5.0.5 Sony\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_5_SonyS: CString = b"L R C Ls Rs Tfl Tfc Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_6: CString = b"11.0 Auro-3D\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString50_6S: CString = b"L R C Ls Rs Tc Tfl Tfc Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51: CString = b"5.1\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51S: CString = b"L R C LFE Ls Rs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_2: CString = b"5.1.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_2S: CString = b"L R C LFE Ls Rs Tfl Tfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_2TopSide: CString = b"5.1.2 Top Side\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_2TopSideS: CString = b"L R C LFE Ls Rs Tsl Tsr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_4: CString = b"5.1.4\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_4S: CString = b"L R C LFE Ls Rs Tfl Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_4_1: CString = b"5.1.4.1\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_4_1S: CString = b"L R C LFE Ls Rs Tfl Tfr Trl Trr Bfc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_5: CString = b"10.1 Auro-3D\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_5S: CString = b"L R C LFE Ls Rs Tc Tfl Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_5_3: CString = b"5.1.5.3\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_5_3S: CString = b"L R C LFE Ls Rs Tfl Tfc Tfr Trl Trr Bfl Bfc Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_6: CString = b"11.1 Auro-3D\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString51_6S: CString = b"L R C LFE Ls Rs Tc Tfl Tfc Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString52_5: CString = b"5.2.5\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString52_5S: CString = b"L R C LFE Ls Rs Tfl Tfc Tfr Trl Trr LFE2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString60Cine: CString = b"6.0 Cine\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString60CineS: CString = b"L R C Ls Rs Cs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString60Music: CString = b"6.0 Music\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString60MusicS: CString = b"L R Ls Rs Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString60_4_4: CString = b"14.0\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString60_4_4S: CString = b"L R Ls Rs Sl Sr Tfl Tfr Trl Trr Bfl Bfr Brl Brr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString61Cine: CString = b"6.1 Cine\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString61CineS: CString = b"L R C LFE Ls Rs Cs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString61Music: CString = b"6.1 Music\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString61MusicS: CString = b"L R LFE Ls Rs Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70Cine: CString = b"7.0 SDDS\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70CineOld: CString = b"7.0 Cine (SDDS)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70CineS: CString = b"L R C Ls Rs Lc Rc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70Music: CString = b"7.0\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70MusicOld: CString = b"7.0 Music (Dolby)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70MusicS: CString = b"L R C Ls Rs Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_2: CString = b"7.0.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_2S: CString = b"L R C Ls Rs Sl Sr Tsl Tsr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_2_TF: CString = b"7.0.2 Top Front\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_2_TFS: CString = b"L R C Ls Rs Sl Sr Tfl Tfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_3: CString = b"7.0.3\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_3S: CString = b"L R C Ls Rs Sl Sr Tfl Tfr Trc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_4: CString = b"7.0.4\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_4S: CString = b"L R C Ls Rs Sl Sr Tfl Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_4_2: CString = b"7.0.4.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_4_2S: CString = b"L R C Ls Rs Sl Sr Tfl Tfr Trl Trr Bfl Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_6: CString = b"7.0.6\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString70_6S: CString = b"L R C Ls Rs Sl Sr Tfl Tfr Trl Trr Tsl Tsr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71Cine: CString = b"7.1 SDDS\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71CineCenterHigh: CString = b"7.1 Cine Center High\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71CineCenterHighS: CString = b"L R C LFE Ls Rs Cs Tfc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71CineFullRear: CString = b"7.1 Cine Full Rear\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71CineFullRearS: CString = b"L R C LFE Ls Rs Lcs Rcs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71CineOld: CString = b"7.1 Cine (SDDS)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71CineS: CString = b"L R C LFE Ls Rs Lc Rc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71CineTopCenter: CString = b"7.1 Cine Top Center\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71CineTopCenterS: CString = b"L R C LFE Ls Rs Cs Tc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71Music: CString = b"7.1\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71MusicOld: CString = b"7.1 Music (Dolby)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71MusicS: CString = b"L R C LFE Ls Rs Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71Proximity: CString = b"7.1 Proximity\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71ProximityS: CString = b"L R C LFE Ls Rs Pl Pr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71_2: CString = b"7.1.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71_2S: CString = b"L R C LFE Ls Rs Sl Sr Tsl Tsr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71_2_TF: CString = b"7.1.2 Top Front\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71_2_TFS: CString = b"L R C LFE Ls Rs Sl Sr Tfl Tfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71_4: CString = b"7.1.4\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71_4S: CString = b"L R C LFE Ls Rs Sl Sr Tfl Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71_6: CString = b"7.1.6\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString71_6S: CString = b"L R C LFE Ls Rs Sl Sr Tfl Tfr Trl Trr Tsl Tsr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString72_3: CString = b"7.2.3\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString72_3S: CString = b"L R C LFE Ls Rs Sl Sr Tfl Tfr Trc LFE2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString72_5: CString = b"12.2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString72_5S: CString = b"L R C LFE Ls Rs Lc Rc Tfl Tfc Tfr Trl Trr LFE2\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString80Cine: CString = b"8.0 Cine\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString80CineS: CString = b"L R C Ls Rs Lc Rc Cs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString80Music: CString = b"8.0 Music\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString80MusicS: CString = b"L R C Ls Rs Cs Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString81Cine: CString = b"8.1 Cine\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString81CineS: CString = b"L R C LFE Ls Rs Lc Rc Cs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString81Music: CString = b"8.1 Music\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString81MusicS: CString = b"L R C LFE Ls Rs Cs Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString90Cine: CString = b"9.0 Cine\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString90CineS: CString = b"L R C Ls Rs Lc Rc Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString90_4: CString = b"9.0.4 ITU\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString90_4S: CString = b"L R C Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString90_4_W: CString = b"9.0.4\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString90_4_WS: CString = b"L R C Ls Rs Sl Sr Tfl Tfr Trl Trr Lw Rw\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString90_6: CString = b"9.0.6 ITU\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString90_6S: CString = b"L R C Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr Tsl Tsr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString90_6_W: CString = b"9.0.6\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString90_6_WS: CString = b"L R C Ls Rs Sl Sr Tfl Tfr Trl Trr Tsl Tsr Lw Rw\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString91Cine: CString = b"9.1 Cine\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString91CineS: CString = b"L R C LFE Ls Rs Lc Rc Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString91_4: CString = b"9.1.4 ITU\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString91_4S: CString = b"L R C LFE Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString91_4_W: CString = b"9.1.4\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString91_4_WS: CString = b"L R C LFE Ls Rs Sl Sr Tfl Tfr Trl Trr Lw Rw\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString91_6: CString = b"9.1.6 ITU\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString91_6S: CString = b"L R C LFE Ls Rs Lc Rc Sl Sr Tfl Tfr Trl Trr Tsl Tsr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString91_6_W: CString = b"9.1.6\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kString91_6_WS: CString = b"L R C LFE Ls Rs Sl Sr Tfl Tfr Trl Trr Tsl Tsr Lw Rw\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi1stOrder: CString = b"1OA\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi1stOrderS: CString = b"0 1 2 3\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi2cdOrder: CString = b"2OA\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi2cdOrderS: CString = b"0 1 2 3 4 5 6 7 8\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi3rdOrder: CString = b"3OA\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi3rdOrderS: CString = b"0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi4thOrder: CString = b"4OA\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi4thOrderS: CString = b"0..24\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi5thOrder: CString = b"5OA\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi5thOrderS: CString = b"0..35\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi6thOrder: CString = b"6OA\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi6thOrderS: CString = b"0..48\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi7thOrder: CString = b"7OA\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringAmbi7thOrderS: CString = b"0..63\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringCineFront: CString = b"Cine Front\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringCineFrontS: CString = b"L R C Lc Rc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringEmpty: CString = b"\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringMono: CString = b"Mono\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringMonoS: CString = b"M\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereo: CString = b"Stereo\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoBF: CString = b"Stereo (Bfl Bfr)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoBFS: CString = b"Bfl Bfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoC: CString = b"Stereo (Lc Rc)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoCLfe: CString = b"Stereo (C LFE)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoCLfeS: CString = b"C LFE\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoCS: CString = b"Lc Rc\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoR: CString = b"Stereo (Ls Rs)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoRS: CString = b"Ls Rs\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoS: CString = b"L R\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoSS: CString = b"Sl Sr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoSide: CString = b"Stereo (Sl Sr)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoTF: CString = b"Stereo (Tfl Tfr)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoTFS: CString = b"Tfl Tfr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoTR: CString = b"Stereo (Trl Trr)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoTRS: CString = b"Trl Trr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoTS: CString = b"Stereo (Tsl Tsr)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoTSS: CString = b"Tsl Tsr\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoWide: CString = b"Stereo (Lw Rw)\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kStringStereoWideS: CString = b"Lw Rw\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod StateType {
            #[allow(unused_imports)]
            use super::*;
            pub const kDefault: CString = b"Default\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kProject: CString = b"Project\0".as_ptr() as *const ::std::ffi::c_char;
            pub const kTrackPreset: CString = b"TrackPreset\0".as_ptr() as *const ::std::ffi::c_char;
        }
        pub mod ViewType {
            #[allow(unused_imports)]
            use super::*;
            pub const kEditor: CString = b"editor\0".as_ptr() as *const ::std::ffi::c_char;
        }
    }
}
