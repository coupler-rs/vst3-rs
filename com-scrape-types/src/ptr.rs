use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

use super::{Inherits, Interface};

/// Trait for types that represent a smart pointer to a COM object.
///
/// Implemented by [`ComPtr`] and [`ComRef`].
///
/// The purpose of this trait is to enable blanket implementations of interface traits for all
/// smart pointers with a compatible target for that interface. For each interface `IInterface`,
/// `com-scrape` will generate a blanket implementation of `IInterfaceTrait` as follows:
///
/// ```ignore
/// impl<P> IInterfaceTrait for P
/// where
///     P: SmartPtr,
///     P::Target: Inherits<IInterface>,
/// {
///     /* ... */
/// }
/// ```
///
/// This makes it possible to call the methods of `IInterface` directly on a `ComPtr<IInterface>`,
/// or on any `ComPtr<IOtherInterface>` where `IOtherInterface` is derived from from `IInterface`.
pub trait SmartPtr {
    /// The interface type pointed to by this smart pointer.
    type Target;

    /// Gets the raw pointer held by this smart pointer.
    fn ptr(&self) -> *mut Self::Target;
}

/// A non-owning smart pointer to a COM object.
///
/// A `ComRef<'a, I>` represents a borrowed reference to a COM object implementing interface `I`.
/// Like [`ComPtr`], `ComRef` can be used to call interface methods on the referenced object.
/// Unlike [`ComPtr`], `ComRef` does not manage the object's reference count, i.e. it will *not*
/// call the release method of the object it points to when going out of scope. See the
/// [crate-level documentation](crate#reference-counting) for more information.
///
/// A `ComRef` can be created safely from a [`ComPtr`] via [`ComPtr::as_com_ref`], or from a
/// [`ComWrapper`][crate::ComWrapper] via [`ComWrapper::as_com_ref`][crate::ComWrapper::as_com_ref].
/// It can also be created unsafely via [`ComRef::from_raw`].
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

unsafe impl<'a, I: Interface> Send for ComRef<'a, I> where I: Sync + Send {}

unsafe impl<'a, I: Interface> Sync for ComRef<'a, I> where I: Sync + Send {}

impl<'a, I: Interface> ComRef<'a, I> {
    /// Gets the wrapped interface pointer.
    ///
    /// Does not perform any reference counting operations.
    #[inline]
    pub fn as_ptr(&self) -> *mut I {
        self.ptr.as_ptr() as *mut I
    }

    /// Creates a `ComRef` from a raw interface pointer if the pointer is non-null.
    ///
    /// Does not perform any reference counting operations.
    ///
    /// `from_raw` will check if `ptr` is null (and return `None` if so), but this method is still
    /// unsafe, as the caller must still ensure that `ptr` is a valid interface pointer (see below)
    /// if it is non-null.
    ///
    /// # Safety
    ///
    /// If `ptr` is non-null, it must be a valid pointer to a value of type `I`, and if `*ptr` is
    /// reinterpreted as `*const I::Vtbl` (see the [safety documentation](Interface#safety) for
    /// [`Interface`]), it must in turn be a valid pointer to `I::Vtbl`.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut I) -> Option<ComRef<'a, I>> {
        NonNull::new(ptr).map(|ptr| ComRef {
            ptr,
            _marker: PhantomData,
        })
    }

    /// Creates a `ComRef` from a raw interface pointer.
    ///
    /// Does not perform any reference counting operations.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid pointer to a value of type `I`, and if `*ptr` is reinterpreted as
    /// `*const I::Vtbl` (see the [safety documentation](Interface#safety) for [`Interface`]), it
    /// must in turn be a valid pointer to `I::Vtbl`.
    #[inline]
    pub unsafe fn from_raw_unchecked(ptr: *mut I) -> ComRef<'a, I> {
        ComRef {
            ptr: NonNull::new_unchecked(ptr),
            _marker: PhantomData,
        }
    }

    /// Upgrades the `ComRef` to a [`ComPtr`].
    ///
    /// Increments the reference count of the object that the `ComRef` points to.
    #[inline]
    pub fn to_com_ptr(&self) -> ComPtr<I> {
        unsafe {
            let ptr = self.ptr.as_ptr();

            I::add_ref(ptr);

            ComPtr::from_raw_unchecked(ptr)
        }
    }

    /// Casts the `ComRef` from a derived interface to a base interface.
    ///
    /// Does not perform any reference counting operations.
    #[inline]
    pub fn upcast<J: Interface>(self) -> ComRef<'a, J>
    where
        I: Inherits<J>,
    {
        unsafe { ComRef::from_raw_unchecked(self.as_ptr() as *mut J) }
    }

    /// Attempts to cast from one interface to another, returning a [`ComPtr`] if successful.
    ///
    /// If the cast is successful, increments the reference count of the object that the `ComRef`
    /// points to.
    #[inline]
    pub fn cast<J: Interface>(&self) -> Option<ComPtr<J>> {
        unsafe {
            if let Some(ptr) = I::query_interface(self.ptr.as_ptr(), &J::IID) {
                ComPtr::from_raw(ptr as *mut J)
            } else {
                None
            }
        }
    }
}

/// An owning smart pointer to a COM object.
///
/// A `ComPtr<I>` represents an owning reference to a COM object implementing interface `I`. Like
/// [`ComRef`], `ComPtr` can be used to call interface methods on the referenced object. Unlike
/// [`ComRef`], `ComPtr` manages the object's reference count, i.e. it *will* call the release
/// method of the object it points to when going out of scope. See the
/// [crate-level documentation](crate#reference-counting) for more information.
///
/// A `ComPtr` can be created safely from a [`ComRef`] via [`ComRef::to_com_ptr`], or from a
/// [`ComWrapper`][crate::ComWrapper] via [`ComWrapper::to_com_ptr`][crate::ComWrapper::to_com_ptr].
/// It can also be created unsafely via [`ComPtr::from_raw`].
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
            I::add_ref(self.ptr.as_ptr());
        }

        ComPtr { ptr: self.ptr }
    }
}

unsafe impl<I: Interface> Send for ComPtr<I> where I: Sync + Send {}

unsafe impl<I: Interface> Sync for ComPtr<I> where I: Sync + Send {}

impl<I: Interface> Drop for ComPtr<I> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            I::release(self.ptr.as_ptr());
        }
    }
}

impl<I: Interface> ComPtr<I> {
    /// Gets the wrapped interface pointer.
    ///
    /// Does not perform any reference counting operations.
    #[inline]
    pub fn as_ptr(&self) -> *mut I {
        self.ptr.as_ptr() as *mut I
    }

    /// Creates a `ComPtr` from a raw interface pointer if the pointer is non-null.
    ///
    /// When the resulting `ComPtr` is dropped, the reference count of the object it points to will
    /// be decremented. Thus, using this method can be thought of as "taking ownership" of a
    /// pointer to a COM object.
    ///
    /// `from_raw` will check if `ptr` is null (and return `None` if so), but this method is still
    /// unsafe, as the caller must still ensure that `ptr` is a valid interface pointer (see below)
    /// if it is non-null.
    ///
    /// # Safety
    ///
    /// If `ptr` is non-null, it must be a valid pointer to a value of type `I`, and if `*ptr` is
    /// reinterpreted as `*const I::Vtbl` (see the [safety documentation](Interface#safety) for
    /// [`Interface`]), it must in turn be a valid pointer to `I::Vtbl`.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut I) -> Option<ComPtr<I>> {
        NonNull::new(ptr).map(|ptr| ComPtr { ptr })
    }

    /// Creates a `ComPtr` from a raw interface pointer.
    ///
    /// When the resulting `ComPtr` is dropped, the reference count of the object it points to will
    /// be decremented. Thus, using this method can be thought of as "taking ownership" of a
    /// pointer to a COM object.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid pointer to a value of type `I`, and if `*ptr` is reinterpreted as
    /// `*const I::Vtbl` (see the [safety documentation](Interface#safety) for [`Interface`]), it
    /// must in turn be a valid pointer to `I::Vtbl`.
    #[inline]
    pub unsafe fn from_raw_unchecked(ptr: *mut I) -> ComPtr<I> {
        ComPtr {
            ptr: NonNull::new_unchecked(ptr),
        }
    }

    /// Consumes the `ComPtr`, returning the wrapped interface pointer.
    ///
    /// Since this method consumes the `ComPtr`, it prevents the `ComPtr` from decrementing the
    /// reference count of the object it points to. Thus, using this method can be thought of as
    /// "relinquishing ownership" of a pointer to a COM object.
    #[inline]
    pub fn into_raw(self) -> *mut I {
        let ptr = self.ptr.as_ptr();
        mem::forget(self);
        ptr
    }

    /// Returns a [`ComRef`] pointing to the same object as this `ComPtr`. Can be thought of as a
    /// borrow.
    ///
    /// Does not perform any reference counting operations.
    #[inline]
    pub fn as_com_ref<'a>(&'a self) -> ComRef<'a, I> {
        unsafe { ComRef::from_raw_unchecked(self.ptr.as_ptr()) }
    }

    /// Does not perform any reference counting operations.
    #[inline]
    pub fn upcast<J: Interface>(self) -> ComPtr<J>
    where
        I: Inherits<J>,
    {
        unsafe { ComPtr::from_raw_unchecked(self.into_raw() as *mut J) }
    }

    /// Attempts to cast from one interface to another, returning another [`ComPtr`] if successful.
    ///
    /// If the cast is successful, increments the reference count of the object that the `ComPtr`
    /// points to.
    #[inline]
    pub fn cast<J: Interface>(&self) -> Option<ComPtr<J>> {
        unsafe {
            if let Some(ptr) = I::query_interface(self.ptr.as_ptr(), &J::IID) {
                ComPtr::from_raw(ptr as *mut J)
            } else {
                None
            }
        }
    }
}
