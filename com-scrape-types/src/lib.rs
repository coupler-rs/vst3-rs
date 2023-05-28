use std::ffi::c_void;
use std::ptr::NonNull;

pub type Guid = [u8; 16];

pub unsafe trait Inherits<I> {}

pub trait SmartPtr {
    type Target;

    fn ptr(&self) -> *mut Self::Target;
}

pub trait Interface {
    unsafe fn query_interface(this: *mut Self, iid: &Guid) -> Option<*mut c_void>;
    unsafe fn add_ref(this: *mut Self);
    unsafe fn release(this: *mut Self);
}

pub struct ComPtr<I: Interface> {
    ptr: NonNull<I>,
}

impl<I: Interface> SmartPtr for ComPtr<I> {
    type Target = I;

    fn ptr(&self) -> *mut I {
        self.ptr.as_ptr()
    }
}

impl<I: Interface> ComPtr<I> {
    pub unsafe fn from_raw(ptr: *mut I) -> Option<ComPtr<I>> {
        NonNull::new(ptr).map(|ptr| ComPtr { ptr })
    }

    pub unsafe fn from_raw_unchecked(ptr: *mut I) -> ComPtr<I> {
        ComPtr {
            ptr: NonNull::new_unchecked(ptr),
        }
    }
}
