use std::cell::Cell;
use std::ffi::{c_long, c_ulong, c_void};
use std::ptr;

use crate::*;

#[repr(C)]
struct IUnknown {
    vtbl: *const IUnknownVtbl,
}

#[repr(C)]
struct IUnknownVtbl {
    query_interface: unsafe extern "system" fn(
        this: *mut IUnknown,
        iid: *const Guid,
        obj: *mut *mut c_void,
    ) -> c_long,
    add_ref: unsafe extern "system" fn(this: *mut IUnknown) -> c_ulong,
    release: unsafe extern "system" fn(this: *mut IUnknown) -> c_ulong,
}

impl Unknown for IUnknown {
    unsafe fn query_interface(this: *mut Self, iid: &Guid) -> Option<*mut c_void> {
        let mut obj = ptr::null_mut();
        let result = ((*(*this).vtbl).query_interface)(this, iid, &mut obj);

        if result == 0 {
            Some(obj)
        } else {
            None
        }
    }

    unsafe fn add_ref(this: *mut Self) -> usize {
        ((*(*this).vtbl).add_ref)(this) as usize
    }

    unsafe fn release(this: *mut Self) -> usize {
        ((*(*this).vtbl).release)(this) as usize
    }
}

unsafe impl Interface for IUnknown {
    const IID: Guid = *b"aaaaaaaaaaaaaaaa";

    fn inherits(iid: &Guid) -> bool {
        iid == &Self::IID
    }
}

unsafe impl Inherits<IUnknown> for IUnknown {}

#[repr(C)]
struct IMyInterface {
    vtbl: *const IMyInterfaceVtbl,
}

#[repr(C)]
struct IMyInterfaceVtbl {
    base: IUnknownVtbl,
    my_method: unsafe extern "system" fn(this: *mut IMyInterface) -> u32,
}

trait IMyInterfaceTrait {
    fn my_method(&self) -> u32;
}

impl<P> IMyInterfaceTrait for P
where
    P: SmartPtr,
    P::Target: Inherits<IMyInterface>,
{
    fn my_method(&self) -> u32 {
        unsafe {
            let ptr = self.ptr() as *mut IMyInterface;
            ((*(*ptr).vtbl).my_method)(ptr)
        }
    }
}

impl Unknown for IMyInterface {
    unsafe fn query_interface(this: *mut Self, iid: &Guid) -> Option<*mut c_void> {
        IUnknown::query_interface(this as *mut IUnknown, iid)
    }

    unsafe fn add_ref(this: *mut Self) -> usize {
        IUnknown::add_ref(this as *mut IUnknown) as usize
    }

    unsafe fn release(this: *mut Self) -> usize {
        IUnknown::release(this as *mut IUnknown) as usize
    }
}

unsafe impl Interface for IMyInterface {
    const IID: Guid = *b"bbbbbbbbbbbbbbbb";

    fn inherits(iid: &Guid) -> bool {
        iid == &Self::IID || IUnknown::inherits(iid)
    }
}

unsafe impl Inherits<IUnknown> for IMyInterface {}
unsafe impl Inherits<IMyInterface> for IMyInterface {}

#[repr(C)]
struct IOtherInterface {
    vtbl: *const IOtherInterfaceVtbl,
}

#[repr(C)]
struct IOtherInterfaceVtbl {
    base: IUnknownVtbl,
    other_method: unsafe extern "system" fn(this: *mut IOtherInterface) -> u32,
}

trait IOtherInterfaceTrait {
    fn other_method(&self) -> u32;
}

impl<P> IOtherInterfaceTrait for P
where
    P: SmartPtr,
    P::Target: Inherits<IOtherInterface>,
{
    fn other_method(&self) -> u32 {
        unsafe {
            let ptr = self.ptr() as *mut IOtherInterface;
            ((*(*ptr).vtbl).other_method)(ptr)
        }
    }
}

impl Unknown for IOtherInterface {
    unsafe fn query_interface(this: *mut Self, iid: &Guid) -> Option<*mut c_void> {
        IUnknown::query_interface(this as *mut IUnknown, iid)
    }

    unsafe fn add_ref(this: *mut Self) -> usize {
        IUnknown::add_ref(this as *mut IUnknown) as usize
    }

    unsafe fn release(this: *mut Self) -> usize {
        IUnknown::release(this as *mut IUnknown) as usize
    }
}

unsafe impl Interface for IOtherInterface {
    const IID: Guid = *b"cccccccccccccccc";

    fn inherits(iid: &Guid) -> bool {
        iid == &Self::IID
    }
}

unsafe impl Inherits<IUnknown> for IOtherInterface {}
unsafe impl Inherits<IOtherInterface> for IOtherInterface {}

#[repr(C)]
struct MyClass {
    my_interface: IMyInterface,
    count: Cell<c_ulong>,
}

impl MyClass {
    fn new() -> MyClass {
        MyClass {
            my_interface: IMyInterface {
                vtbl: &IMyInterfaceVtbl {
                    base: IUnknownVtbl {
                        query_interface: Self::query_interface,
                        add_ref: Self::add_ref,
                        release: Self::release,
                    },
                    my_method: Self::my_method,
                },
            },
            count: Cell::new(1),
        }
    }

    unsafe extern "system" fn query_interface(
        this: *mut IUnknown,
        iid: *const Guid,
        obj: *mut *mut c_void,
    ) -> c_long {
        if IMyInterface::inherits(&*iid) {
            Self::add_ref(this);
            *obj = this as *mut c_void;
            0
        } else {
            1
        }
    }

    unsafe extern "system" fn add_ref(this: *mut IUnknown) -> c_ulong {
        let obj = &*(this as *mut MyClass);
        obj.count.set(obj.count.get() + 1);
        obj.count.get()
    }

    unsafe extern "system" fn release(this: *mut IUnknown) -> c_ulong {
        let obj = &*(this as *mut MyClass);
        obj.count.set(obj.count.get() - 1);
        obj.count.get()
    }

    unsafe extern "system" fn my_method(_this: *mut IMyInterface) -> u32 {
        0
    }
}

#[test]
fn reference_counting() {
    let obj = MyClass::new();

    let com_ptr_1 = unsafe { ComPtr::from_raw(&obj as *const MyClass as *mut IUnknown) }.unwrap();
    assert_eq!(obj.count.get(), 1);

    let com_ptr_2 = com_ptr_1.clone();
    assert_eq!(obj.count.get(), 2);

    let com_ref_1 = unsafe { ComRef::from_raw(&obj as *const MyClass as *mut IUnknown) }.unwrap();
    assert_eq!(obj.count.get(), 2);

    let com_ptr_3 = com_ref_1.to_com_ptr();
    assert_eq!(obj.count.get(), 3);

    let _com_ref_2 = com_ptr_3.as_com_ref();
    assert_eq!(obj.count.get(), 3);

    drop(com_ptr_1);
    assert_eq!(obj.count.get(), 2);

    drop(com_ptr_2);
    assert_eq!(obj.count.get(), 1);

    drop(com_ptr_3);
    assert_eq!(obj.count.get(), 0);
}

#[test]
fn cast_com_ref() {
    let obj = MyClass::new();

    let com_ref = unsafe { ComRef::from_raw(&obj as *const MyClass as *mut IUnknown) }.unwrap();

    let unknown = com_ref.cast::<IUnknown>();
    assert!(unknown.is_some());
    assert_eq!(obj.count.get(), 2);

    let my_interface = com_ref.cast::<IMyInterface>();
    assert!(my_interface.is_some());
    assert_eq!(obj.count.get(), 3);

    let other_interface = com_ref.cast::<IOtherInterface>();
    assert!(other_interface.is_none());
    assert_eq!(obj.count.get(), 3);
}

#[test]
fn cast_com_ptr() {
    let obj = MyClass::new();

    let com_ptr = unsafe { ComPtr::from_raw(&obj as *const MyClass as *mut IUnknown) }.unwrap();

    let unknown = com_ptr.cast::<IUnknown>();
    assert!(unknown.is_some());
    assert_eq!(obj.count.get(), 2);

    let my_interface = com_ptr.cast::<IMyInterface>();
    assert!(my_interface.is_some());
    assert_eq!(obj.count.get(), 3);

    let other_interface = com_ptr.cast::<IOtherInterface>();
    assert!(other_interface.is_none());
    assert_eq!(obj.count.get(), 3);
}
