use std::ops::Deref;
use std::ptr::addr_of;
use std::sync::Arc;

use super::{ComPtr, ComRef, Interface, Unknown};

#[doc(hidden)]
#[macro_export]
macro_rules! impl_class_inner {
    ($class:ident: $($interface:ident),* $(,)?) => {
        const _: () = {
            enum __Offsets { $($interface,)* __Len }

            impl $crate::Unknown for $class {
                #[inline]
                unsafe fn query_interface(this: *mut Self, iid: &$crate::Guid) -> Option<*mut ::std::ffi::c_void> {
                    $(
                        if <$interface as $crate::Interface>::inherits(iid) {
                            $crate::Unknown::add_ref(this);
                            let ptr = $crate::ComWrapper::<$class>::interface_from_data::<$interface>(this);
                            return Some(ptr as *mut ::std::ffi::c_void);
                        }
                    )*

                    None
                }

                #[inline]
                unsafe fn add_ref(this: *mut Self) -> usize {
                    $crate::ComWrapper::add_ref(this)
                }

                #[inline]
                unsafe fn release(this: *mut Self) -> usize {
                    $crate::ComWrapper::release(this)
                }
            }

            unsafe impl $crate::Class for $class {
                type Header = [*mut (); __Offsets::__Len as usize];

                const HEADER: Self::Header = [
                    $(
                        &$interface::make_vtbl::<$class, $interface>() as *const _ as *mut (),
                    )*
                ];
            }

            $(
                unsafe impl $crate::Implements<$interface> for $class {
                    const OFFSET: isize = __Offsets::$interface as isize * ::std::mem::size_of::<*mut ()>() as isize;
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

pub unsafe trait Class: Unknown {
    type Header;

    const HEADER: Self::Header;
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

impl<C: Class> ComWrapper<C> {
    #[inline]
    pub fn new(data: C) -> ComWrapper<C>
    where
        C: 'static,
    {
        ComWrapper {
            inner: Arc::new(ComWrapperInner {
                header: C::HEADER,
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

    #[inline]
    pub unsafe fn add_ref(ptr: *mut C) -> usize {
        let wrapper_ptr = Self::wrapper_from_data(ptr);

        let arc = Arc::from_raw(wrapper_ptr);
        let result = Arc::strong_count(&arc) + 1;
        let _ = Arc::into_raw(arc);

        Arc::increment_strong_count(wrapper_ptr);

        result
    }

    #[inline]
    pub unsafe fn release(ptr: *mut C) -> usize {
        let wrapper_ptr = Self::wrapper_from_data(ptr);

        let arc = Arc::from_raw(wrapper_ptr);
        let result = Arc::strong_count(&arc) - 1;
        let _ = Arc::into_raw(arc);

        Arc::decrement_strong_count(wrapper_ptr);

        result
    }

    #[inline]
    pub unsafe fn data_from_interface<I>(ptr: *mut I) -> *mut C
    where
        I: Interface,
        C: Implements<I>,
    {
        (ptr as *mut u8)
            .offset(-<C as Implements<I>>::OFFSET)
            .offset(-offset_of!(ComWrapperInner<C>, header))
            .offset(offset_of!(ComWrapperInner<C>, data)) as *mut C
    }

    #[inline]
    pub unsafe fn interface_from_data<I>(ptr: *mut C) -> *mut I
    where
        I: Interface,
        C: Implements<I>,
    {
        (ptr as *mut u8)
            .offset(-offset_of!(ComWrapperInner<C>, data))
            .offset(offset_of!(ComWrapperInner<C>, header))
            .offset(<C as Implements<I>>::OFFSET) as *mut I
    }
}
