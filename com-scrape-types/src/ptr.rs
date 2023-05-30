use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

use super::Interface;

pub trait SmartPtr {
    type Target;

    fn ptr(&self) -> *mut Self::Target;
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
        unsafe { I::query_interface::<J>(self.as_mut_ptr()).and_then(|ptr| ComPtr::from_raw(ptr)) }
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
        unsafe { I::query_interface::<J>(self.as_mut_ptr()).and_then(|ptr| ComPtr::from_raw(ptr)) }
    }
}
