use std::ops::Deref;
use std::ptr::addr_of;
use std::sync::Arc;

use super::{ComPtr, ComRef, Guid, Interface};

#[doc(hidden)]
#[macro_export]
macro_rules! impl_class_inner {
    ($class:ident: $($interface:ident),* $(,)?) => {
        #[allow(non_snake_case)]
        const _: () = {
            struct __Header {
                $($interface: $interface,)*
            }

            unsafe impl $crate::Class for $class {
                type Header = __Header;

                #[inline]
                fn header<W: $crate::Wrapper<Self>>() -> Self::Header {
                    __Header {
                        $(
                            $interface: <$interface as $crate::Construct<$class, W, { <Self as $crate::Implements<$interface>>::OFFSET }>>::OBJ,
                        )*
                    }
                }

                #[inline]
                fn query_interface(iid: &$crate::Guid) -> Option<isize> {
                    $(
                        if <$interface as $crate::Interface>::inherits(iid) {
                            return Some(<Self as $crate::Implements<$interface>>::OFFSET);
                        }
                    )*

                    None
                }
            }

            $(
                unsafe impl $crate::Implements<$interface> for $class {
                    const OFFSET: isize = unsafe { $crate::offset_of!(__Header, $interface) };
                }
            )*
        };
    }
}

#[macro_export]
macro_rules! impl_class {
    ($class:ident: $interface:ident $(+ $interfaces:ident)* $(+)?) => {
        $crate::impl_class_inner!($class: $interface, $($interfaces),*);
    }
}

#[macro_export]
macro_rules! offset_of {
    ($struct:ty, $field:ident) => {{
        use ::std::ffi::c_void;
        use ::std::mem::MaybeUninit;
        use ::std::ptr::addr_of;

        let dummy = MaybeUninit::<$struct>::uninit();
        let base = dummy.as_ptr();
        let field = addr_of!((*base).$field);

        (field as *const c_void).offset_from(base as *const c_void)
    }};
}

pub trait Wrapper<C: Class + ?Sized> {
    unsafe fn data_from_header(ptr: *mut C::Header) -> *mut C;
    unsafe fn header_from_data(ptr: *mut C) -> *mut C::Header;
    unsafe fn add_ref(ptr: *mut C) -> usize;
    unsafe fn release(ptr: *mut C) -> usize;
}

pub trait Construct<C, W, const OFFSET: isize> {
    const OBJ: Self;
}

pub unsafe trait Class {
    type Header;

    fn header<W: Wrapper<Self>>() -> Self::Header;
    fn query_interface(iid: &Guid) -> Option<isize>;
}

pub unsafe trait Implements<I> {
    const OFFSET: isize;
}

#[repr(C)]
struct ComWrapperInner<C: Class> {
    header: C::Header,
    data: C,
}

pub struct ComWrapper<C: Class> {
    inner: Arc<ComWrapperInner<C>>,
}

impl<C: Class> Clone for ComWrapper<C> {
    fn clone(&self) -> ComWrapper<C> {
        ComWrapper {
            inner: self.inner.clone(),
        }
    }
}

unsafe impl<C: Class> Send for ComWrapper<C> where C: Send + Sync {}
unsafe impl<C: Class> Sync for ComWrapper<C> where C: Send + Sync {}

impl<C: Class> Deref for ComWrapper<C> {
    type Target = C;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner.data
    }
}

impl<C: Class> Wrapper<C> for ComWrapper<C> {
    #[inline]
    unsafe fn data_from_header(ptr: *mut C::Header) -> *mut C {
        (ptr as *mut u8)
            .offset(-offset_of!(ComWrapperInner<C>, header))
            .offset(offset_of!(ComWrapperInner<C>, data)) as *mut C
    }

    #[inline]
    unsafe fn header_from_data(ptr: *mut C) -> *mut C::Header {
        (ptr as *mut u8)
            .offset(-offset_of!(ComWrapperInner<C>, data))
            .offset(offset_of!(ComWrapperInner<C>, header)) as *mut C::Header
    }

    #[inline]
    unsafe fn add_ref(ptr: *mut C) -> usize {
        let wrapper_ptr = Self::wrapper_from_data(ptr);

        let arc = Arc::from_raw(wrapper_ptr);
        let result = Arc::strong_count(&arc) + 1;
        let _ = Arc::into_raw(arc);

        Arc::increment_strong_count(wrapper_ptr);

        result
    }

    #[inline]
    unsafe fn release(ptr: *mut C) -> usize {
        let wrapper_ptr = Self::wrapper_from_data(ptr);

        let arc = Arc::from_raw(wrapper_ptr);
        let result = Arc::strong_count(&arc) - 1;
        let _ = Arc::into_raw(arc);

        Arc::decrement_strong_count(wrapper_ptr);

        result
    }
}

impl<C: Class> ComWrapper<C> {
    #[inline]
    pub fn new(data: C) -> ComWrapper<C>
    where
        C: 'static,
    {
        ComWrapper {
            inner: Arc::new(ComWrapperInner {
                header: C::header::<Self>(),
                data,
            }),
        }
    }

    #[inline]
    unsafe fn interface_from_wrapper<I>(ptr: *mut ComWrapperInner<C>) -> *mut I
    where
        I: Interface,
        C: Implements<I>,
    {
        (ptr as *mut u8)
            .offset(offset_of!(ComWrapperInner<C>, header))
            .offset(<C as Implements<I>>::OFFSET) as *mut I
    }

    #[inline]
    pub fn as_com_ref<'a, I>(&'a self) -> ComRef<'a, I>
    where
        I: Interface,
        C: Implements<I>,
    {
        unsafe {
            let wrapper_ptr = Arc::as_ptr(&self.inner) as *mut ComWrapperInner<C>;
            let interface_ptr = Self::interface_from_wrapper::<I>(wrapper_ptr);
            ComRef::from_raw_unchecked(interface_ptr)
        }
    }

    #[inline]
    pub fn to_com_ptr<I>(&self) -> ComPtr<I>
    where
        I: Interface,
        C: Implements<I>,
    {
        unsafe {
            let wrapper_ptr = Arc::into_raw(self.inner.clone()) as *mut ComWrapperInner<C>;
            let interface_ptr = Self::interface_from_wrapper::<I>(wrapper_ptr);
            ComPtr::from_raw_unchecked(interface_ptr)
        }
    }

    #[inline]
    pub fn into_com_ptr<I>(self) -> ComPtr<I>
    where
        I: Interface,
        C: Implements<I>,
    {
        unsafe {
            let wrapper_ptr = Arc::into_raw(self.inner) as *mut ComWrapperInner<C>;
            let interface_ptr = Self::interface_from_wrapper::<I>(wrapper_ptr);
            ComPtr::from_raw_unchecked(interface_ptr)
        }
    }

    #[inline]
    unsafe fn wrapper_from_data(ptr: *mut C) -> *mut ComWrapperInner<C> {
        (ptr as *mut u8).offset(-offset_of!(ComWrapperInner<C>, data)) as *mut ComWrapperInner<C>
    }
}
