use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

pub type Guid = [u8; 16];

pub unsafe trait Inherits<I> {}

pub trait SmartPtr {
    type Target;

    fn ptr(&self) -> *mut Self::Target;
}

pub unsafe trait Interface {
    const IID: Guid;

    unsafe fn query_interface(this: *mut Self, iid: &Guid) -> Option<*mut c_void>;
    unsafe fn add_ref(this: *mut Self);
    unsafe fn release(this: *mut Self);
}

pub struct ComRef<'a, I: Interface> {
    ptr: NonNull<I>,
    _marker: PhantomData<&'a I>,
}

impl<'a, I: Interface> SmartPtr for ComRef<'a, I> {
    type Target = I;

    #[inline]
    fn ptr(&self) -> *mut I {
        self.ptr.as_ptr()
    }
}

impl<'a, I: Interface> Copy for ComRef<'a, I> {}

impl<'a, I: Interface> Clone for ComRef<'a, I> {
    #[inline]
    fn clone(&self) -> ComRef<'a, I> {
        ComRef {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

impl<'a, I: Interface> ComRef<'a, I> {
    #[inline]
    pub fn as_ptr(&self) -> *const I {
        self.ptr.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut I {
        self.as_ptr() as *mut I
    }

    #[inline]
    pub unsafe fn from_raw(ptr: *mut I) -> Option<ComRef<'a, I>> {
        NonNull::new(ptr).map(|ptr| ComRef {
            ptr,
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn into_raw(self) -> *mut I {
        self.as_mut_ptr()
    }

    #[inline]
    pub unsafe fn from_raw_unchecked(ptr: *mut I) -> ComRef<'a, I> {
        ComRef {
            ptr: NonNull::new_unchecked(ptr),
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn to_com_ptr(&self) -> ComPtr<I> {
        unsafe {
            I::add_ref(self.as_mut_ptr());

            ComPtr::from_raw_unchecked(self.as_mut_ptr())
        }
    }

    #[inline]
    pub fn cast<J: Interface>(&self) -> Option<ComPtr<J>> {
        unsafe {
            I::query_interface(self.as_mut_ptr(), &J::IID)
                .and_then(|ptr| ComPtr::from_raw(ptr as *mut J))
        }
    }
}

pub struct ComPtr<I: Interface> {
    ptr: NonNull<I>,
}

impl<I: Interface> SmartPtr for ComPtr<I> {
    type Target = I;

    #[inline]
    fn ptr(&self) -> *mut I {
        self.ptr.as_ptr()
    }
}

impl<I: Interface> Clone for ComPtr<I> {
    #[inline]
    fn clone(&self) -> ComPtr<I> {
        unsafe {
            I::add_ref(self.as_mut_ptr());
        }

        ComPtr { ptr: self.ptr }
    }
}

impl<I: Interface> Drop for ComPtr<I> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            I::release(self.ptr.as_ptr());
        }
    }
}

impl<I: Interface> ComPtr<I> {
    #[inline]
    pub fn as_ptr(&self) -> *const I {
        self.ptr.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut I {
        self.as_ptr() as *mut I
    }

    #[inline]
    pub unsafe fn from_raw(ptr: *mut I) -> Option<ComPtr<I>> {
        NonNull::new(ptr).map(|ptr| ComPtr { ptr })
    }

    #[inline]
    pub fn into_raw(self) -> *mut I {
        let ptr = self.ptr.as_ptr();
        mem::forget(self);
        ptr
    }

    #[inline]
    pub unsafe fn from_raw_unchecked(ptr: *mut I) -> ComPtr<I> {
        ComPtr {
            ptr: NonNull::new_unchecked(ptr),
        }
    }

    #[inline]
    pub fn as_com_ref<'a>(&'a self) -> ComRef<'a, I> {
        unsafe { ComRef::from_raw_unchecked(self.as_mut_ptr()) }
    }

    #[inline]
    pub fn cast<J: Interface>(&self) -> Option<ComPtr<J>> {
        unsafe {
            I::query_interface(self.as_mut_ptr(), &J::IID)
                .and_then(|ptr| ComPtr::from_raw(ptr as *mut J))
        }
    }
}

#[cfg(test)]
mod tests {
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

        unsafe fn query_interface(this: *mut Self, iid: &Guid) -> Option<*mut c_void> {
            let ptr = this as *mut IUnknown;
            let mut obj = ::std::ptr::null_mut();
            let result = ((*(*ptr).vtbl).query_interface)(ptr, iid, &mut obj);

            if result == 0 {
                Some(obj as *mut c_void)
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

        unsafe fn query_interface(this: *mut Self, iid: &Guid) -> Option<*mut c_void> {
            IUnknown::query_interface(this as *mut IUnknown, iid)
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
        unknown: IUnknown,
        count: Cell<c_ulong>,
    }

    impl MyClass {
        fn new() -> MyClass {
            MyClass {
                unknown: IUnknown {
                    vtbl: &IUnknownVtbl {
                        query_interface: Self::query_interface,
                        add_ref: Self::add_ref,
                        release: Self::release,
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

        unsafe fn query_interface(_this: *mut Self, _iid: &Guid) -> Option<*mut c_void> {
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

        let com_ptr_1 =
            unsafe { ComPtr::from_raw(&obj as *const MyClass as *mut IUnknown) }.unwrap();
        assert_eq!(obj.count.get(), 1);

        let com_ptr_2 = com_ptr_1.clone();
        assert_eq!(obj.count.get(), 2);

        let com_ref_1 =
            unsafe { ComRef::from_raw(&obj as *const MyClass as *mut IUnknown) }.unwrap();
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
}
