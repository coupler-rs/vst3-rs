use std::cell::Cell;
use std::ffi::{c_long, c_ulong, c_void};

use crate::{ComPtr, ComRef, Guid, Inherits, Interface, SmartPtr};

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

trait IUnknownTrait {
    unsafe fn query_interface(&self, iid: *const Guid, obj: *mut *mut c_void) -> c_long;
    unsafe fn add_ref(&self) -> c_ulong;
    unsafe fn release(&self) -> c_ulong;
}

unsafe impl Interface for IUnknown {
    const IID: Guid = *b"aaaaaaaaaaaaaaaa";

    unsafe fn query_interface<I: Interface>(this: *mut Self) -> Option<*mut I> {
        let ptr = this as *mut IUnknown;
        let mut obj = ::std::ptr::null_mut();
        let result = ((*(*ptr).vtbl).query_interface)(ptr, &I::IID, &mut obj);

        if result == 0 {
            Some(obj as *mut I)
        } else {
            None
        }
    }

    unsafe fn add_ref(this: *mut Self) {
        let ptr = this as *mut IUnknown;
        ((*(*ptr).vtbl).add_ref)(ptr);
    }

    unsafe fn release(this: *mut Self) {
        let ptr = this as *mut IUnknown;
        ((*(*ptr).vtbl).release)(ptr);
    }
}

unsafe impl Inherits<IUnknown> for IUnknown {}

impl<P> IUnknownTrait for P
where
    P: SmartPtr,
    P::Target: Inherits<IUnknown>,
{
    unsafe fn query_interface(&self, iid: *const Guid, obj: *mut *mut c_void) -> c_long {
        let ptr = self.ptr() as *mut IUnknown;
        ((*(*ptr).vtbl).query_interface)(ptr, iid, obj)
    }

    unsafe fn add_ref(&self) -> c_ulong {
        let ptr = self.ptr() as *mut IUnknown;
        ((*(*ptr).vtbl).add_ref)(ptr)
    }

    unsafe fn release(&self) -> c_ulong {
        let ptr = self.ptr() as *mut IUnknown;
        ((*(*ptr).vtbl).release)(ptr)
    }
}

#[repr(C)]
struct IMyInterface {
    vtbl: *const IMyInterfaceVtbl,
}

#[repr(C)]
struct IMyInterfaceVtbl {
    base: IUnknownVtbl,
    my_method: unsafe extern "system" fn(this: *mut IMyInterface),
}

trait IMyInterfaceTrait {
    unsafe fn my_method(&self);
}

unsafe impl Interface for IMyInterface {
    const IID: Guid = *b"bbbbbbbbbbbbbbbb";

    unsafe fn query_interface<I: Interface>(this: *mut Self) -> Option<*mut I> {
        IUnknown::query_interface::<I>(this as *mut IUnknown)
    }

    unsafe fn add_ref(this: *mut Self) {
        IUnknown::add_ref(this as *mut IUnknown)
    }

    unsafe fn release(this: *mut Self) {
        IUnknown::release(this as *mut IUnknown)
    }
}

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
        if let IUnknown::IID | IMyInterface::IID = *iid {
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

    unsafe extern "system" fn my_method(_this: *mut IMyInterface) {}
}

#[repr(C)]
struct IOtherInterface {
    vtbl: *const IOtherInterfaceVtbl,
}

#[repr(C)]
struct IOtherInterfaceVtbl {
    base: IUnknownVtbl,
}

trait IOtherInterfaceTrait {}

unsafe impl Interface for IOtherInterface {
    const IID: Guid = *b"cccccccccccccccc";

    unsafe fn query_interface<I: Interface>(_this: *mut Self) -> Option<*mut I> {
        unimplemented!()
    }

    unsafe fn add_ref(_this: *mut Self) {
        unimplemented!()
    }

    unsafe fn release(_this: *mut Self) {
        unimplemented!()
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
