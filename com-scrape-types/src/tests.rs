use std::cell::Cell;
use std::ffi::{c_long, c_ulong, c_void};
use std::ptr;
use std::rc::Rc;

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

impl IUnknown {
    pub const fn make_vtbl<C, W, const OFFSET: isize>() -> IUnknownVtbl
    where
        C: Class,
        W: Wrapper<C>,
    {
        unsafe extern "system" fn query_interface<C, W, const OFFSET: isize>(
            this: *mut IUnknown,
            _iid: *const Guid,
            obj: *mut *mut c_void,
        ) -> c_long
        where
            C: Class,
            W: Wrapper<C>,
        {
            let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut C::Header;
            if let Some(result) = C::query_interface(&*(_iid as *const Guid)) {
                let ptr = W::data_from_header(header_ptr);
                W::add_ref(ptr);

                *obj = (header_ptr as *mut u8).offset(result) as *mut c_void;

                0
            } else {
                1
            }
        }

        unsafe extern "system" fn add_ref<C, W, const OFFSET: isize>(this: *mut IUnknown) -> c_ulong
        where
            C: Class,
            W: Wrapper<C>,
        {
            let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut C::Header;
            let ptr = W::data_from_header(header_ptr);
            W::add_ref(ptr) as c_ulong
        }

        unsafe extern "system" fn release<C, W, const OFFSET: isize>(this: *mut IUnknown) -> c_ulong
        where
            C: Class,
            W: Wrapper<C>,
        {
            let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut C::Header;
            let ptr = W::data_from_header(header_ptr);
            W::release(ptr) as c_ulong
        }

        IUnknownVtbl {
            query_interface: query_interface::<C, W, OFFSET>,
            add_ref: add_ref::<C, W, OFFSET>,
            release: release::<C, W, OFFSET>,
        }
    }
}

impl<C, W, const OFFSET: isize> Construct<C, W, OFFSET> for IUnknown
where
    C: Class,
    W: Wrapper<C>,
{
    const OBJ: IUnknown = IUnknown {
        vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
    };
}

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

impl IMyInterface {
    pub const fn make_vtbl<C, W, const OFFSET: isize>() -> IMyInterfaceVtbl
    where
        C: IMyInterfaceTrait + Class,
        W: Wrapper<C>,
    {
        unsafe extern "system" fn my_method<C, W, const OFFSET: isize>(
            this: *mut IMyInterface,
        ) -> u32
        where
            C: IMyInterfaceTrait + Class,
            W: Wrapper<C>,
        {
            let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut C::Header;
            let ptr = W::data_from_header(header_ptr);
            (*ptr).my_method()
        }

        IMyInterfaceVtbl {
            base: IUnknown::make_vtbl::<C, W, OFFSET>(),
            my_method: my_method::<C, W, OFFSET>,
        }
    }
}

impl<C, W, const OFFSET: isize> Construct<C, W, OFFSET> for IMyInterface
where
    C: IMyInterfaceTrait + Class,
    W: Wrapper<C>,
{
    const OBJ: IMyInterface = IMyInterface {
        vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
    };
}

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

impl IOtherInterface {
    pub const fn make_vtbl<C, W, const OFFSET: isize>() -> IOtherInterfaceVtbl
    where
        C: IOtherInterfaceTrait + Class,
        W: Wrapper<C>,
    {
        unsafe extern "system" fn other_method<C, W, const OFFSET: isize>(
            this: *mut IOtherInterface,
        ) -> u32
        where
            C: IOtherInterfaceTrait + Class,
            W: Wrapper<C>,
        {
            let header_ptr = (this as *mut u8).offset(-OFFSET) as *mut C::Header;
            let ptr = W::data_from_header(header_ptr);
            (*ptr).other_method()
        }

        IOtherInterfaceVtbl {
            base: IUnknown::make_vtbl::<C, W, OFFSET>(),
            other_method: other_method::<C, W, OFFSET>,
        }
    }
}

impl<C, W, const OFFSET: isize> Construct<C, W, OFFSET> for IOtherInterface
where
    C: IOtherInterfaceTrait + Class,
    W: Wrapper<C>,
{
    const OBJ: IOtherInterface = IOtherInterface {
        vtbl: &Self::make_vtbl::<C, W, OFFSET>(),
    };
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

struct MyClass2 {
    x: u32,
    y: u32,
    dropped: Rc<Cell<bool>>,
}

impl Drop for MyClass2 {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

impl IMyInterfaceTrait for MyClass2 {
    fn my_method(&self) -> u32 {
        self.x
    }
}

impl IOtherInterfaceTrait for MyClass2 {
    fn other_method(&self) -> u32 {
        self.y
    }
}

struct MyClass2Header {
    my_interface: IMyInterface,
    other_interface: IOtherInterface,
}

unsafe impl Class for MyClass2 {
    type Header = MyClass2Header;

    fn header<W: Wrapper<Self>>() -> Self::Header {
        MyClass2Header {
            my_interface: <IMyInterface as Construct<
                Self,
                W,
                { unsafe { offset_of!(MyClass2Header, my_interface) } },
            >>::OBJ,
            other_interface: <IOtherInterface as Construct<
                Self,
                W,
                { unsafe { offset_of!(MyClass2Header, other_interface) } },
            >>::OBJ,
        }
    }

    #[inline]
    fn query_interface(iid: &Guid) -> Option<isize> {
        if IMyInterface::inherits(iid) {
            return Some(unsafe { offset_of!(MyClass2Header, my_interface) });
        }

        if IOtherInterface::inherits(iid) {
            return Some(unsafe { offset_of!(MyClass2Header, other_interface) });
        }

        None
    }
}

#[test]
fn com_wrapper() {
    let dropped = Rc::new(Cell::new(false));
    let obj = ComWrapper::new(MyClass2 {
        x: 1,
        y: 2,
        dropped: dropped.clone(),
    });

    let com_ref_1 = obj.as_com_ref::<IMyInterface>().unwrap();
    assert_eq!(com_ref_1.my_method(), 1);

    let com_ref_2 = obj.as_com_ref::<IOtherInterface>().unwrap();
    assert_eq!(com_ref_2.other_method(), 2);

    let com_ptr_1 = com_ref_2
        .upcast::<IUnknown>()
        .cast::<IMyInterface>()
        .unwrap();
    assert_eq!(com_ptr_1.my_method(), 1);

    let com_ptr_2 = com_ref_1
        .upcast::<IUnknown>()
        .cast::<IOtherInterface>()
        .unwrap();
    assert_eq!(com_ptr_2.other_method(), 2);

    assert_eq!(dropped.get(), false);

    let com_ptr_3 = obj.to_com_ptr::<IMyInterface>().unwrap();
    assert_eq!(com_ptr_3.my_method(), 1);

    let com_ptr_4 = obj.to_com_ptr::<IOtherInterface>().unwrap();
    assert_eq!(com_ptr_4.other_method(), 2);

    drop(obj);
    drop(com_ptr_1);
    drop(com_ptr_2);
    drop(com_ptr_3);
    assert_eq!(dropped.get(), false);

    drop(com_ptr_4);
    assert_eq!(dropped.get(), true);
}

struct MyClass3 {
    x: u32,
    y: u32,
    dropped: Rc<Cell<bool>>,
}

impl_class!(MyClass3: IMyInterface + IOtherInterface);

impl Drop for MyClass3 {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

impl IMyInterfaceTrait for MyClass3 {
    fn my_method(&self) -> u32 {
        self.x
    }
}

impl IOtherInterfaceTrait for MyClass3 {
    fn other_method(&self) -> u32 {
        self.y
    }
}

#[test]
fn impl_class_macro() {
    let dropped = Rc::new(Cell::new(false));
    let obj = ComWrapper::new(MyClass3 {
        x: 1,
        y: 2,
        dropped: dropped.clone(),
    });

    let com_ref_1 = obj.as_com_ref::<IMyInterface>().unwrap();
    assert_eq!(com_ref_1.my_method(), 1);

    let com_ref_2 = obj.as_com_ref::<IOtherInterface>().unwrap();
    assert_eq!(com_ref_2.other_method(), 2);

    let com_ptr_1 = com_ref_2
        .upcast::<IUnknown>()
        .cast::<IMyInterface>()
        .unwrap();
    assert_eq!(com_ptr_1.my_method(), 1);

    let com_ptr_2 = com_ref_1
        .upcast::<IUnknown>()
        .cast::<IOtherInterface>()
        .unwrap();
    assert_eq!(com_ptr_2.other_method(), 2);

    assert_eq!(dropped.get(), false);

    let com_ptr_3 = obj.to_com_ptr::<IMyInterface>().unwrap();
    assert_eq!(com_ptr_3.my_method(), 1);

    let com_ptr_4 = obj.to_com_ptr::<IOtherInterface>().unwrap();
    assert_eq!(com_ptr_4.other_method(), 2);

    drop(obj);
    drop(com_ptr_1);
    drop(com_ptr_2);
    drop(com_ptr_3);
    assert_eq!(dropped.get(), false);

    drop(com_ptr_4);
    assert_eq!(dropped.get(), true);
}
