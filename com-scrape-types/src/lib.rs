use std::ops::Deref;

pub trait Interface {
    type Ptr: InterfacePtr<Self>;
}

pub trait InterfacePtr<I: ?Sized> {
    fn from_raw(ptr: *mut I) -> Self;
    fn into_raw(self) -> *mut I;
}

pub struct ComPtr<I: Interface> {
    ptr: I::Ptr,
}

impl<I: Interface> ComPtr<I> {
    pub fn from_raw(ptr: *mut I) -> ComPtr<I> {
        ComPtr {
            ptr: I::Ptr::from_raw(ptr),
        }
    }
}

impl<I: Interface> Deref for ComPtr<I> {
    type Target = I::Ptr;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}
