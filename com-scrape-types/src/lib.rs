use std::ptr::NonNull;

pub unsafe trait Inherits<I> {}

pub trait SmartPtr {
    type Target;

    fn ptr(&self) -> *mut Self::Target;
}

pub trait Interface {}

pub struct ComPtr<I> {
    ptr: NonNull<I>,
}

impl<I> SmartPtr for ComPtr<I> {
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
