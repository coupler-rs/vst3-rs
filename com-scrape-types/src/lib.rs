pub unsafe trait Inherits<I> {}

pub trait SmartPtr {
    type Target;

    fn ptr(&self) -> *mut Self::Target;
}

pub trait Interface {}

pub struct ComPtr<I> {
    ptr: *mut I,
}

impl<I> SmartPtr for ComPtr<I> {
    type Target = I;

    fn ptr(&self) -> *mut I {
        self.ptr
    }
}

impl<I: Interface> ComPtr<I> {
    pub fn from_raw(ptr: *mut I) -> ComPtr<I> {
        ComPtr { ptr }
    }
}
