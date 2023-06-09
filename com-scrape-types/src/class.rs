use std::ops::Deref;
use std::ptr::addr_of;
use std::sync::Arc;

use super::{ComPtr, ComRef, Guid, Interface};

macro_rules! offset_of {
    ($struct:ty, $field:tt) => {{
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
    unsafe fn data_from_header(ptr: *mut Header<C>) -> *mut C;
    unsafe fn header_from_data(ptr: *mut C) -> *mut Header<C>;
    unsafe fn add_ref(ptr: *mut C) -> usize;
    unsafe fn release(ptr: *mut C) -> usize;
}

pub trait Construct<C, W, const OFFSET: isize> {
    const OBJ: Self;
}

pub unsafe trait InterfaceList {
    type Header;

    fn query(iid: &Guid) -> Option<isize>;
}

pub trait MakeHeader<C, W>: InterfaceList
where
    C: Class,
    W: Wrapper<C>,
{
    fn header() -> Self::Header;
}

pub trait Class {
    type Interfaces: InterfaceList;
}

pub type Header<C> = <<C as Class>::Interfaces as InterfaceList>::Header;

pub unsafe trait Implements<I> {
    const OFFSET: isize;
}

macro_rules! interface_list {
    ($header:ident, $($interface:ident $index:tt),*) => {
        #[repr(C)]
        pub struct $header<$($interface),*>($($interface),*);

        unsafe impl<$($interface: Interface),*> InterfaceList for ($($interface,)*) {
            type Header = $header<$($interface),*>;

            fn query(iid: &Guid) -> Option<isize> {
                $(
                    if $interface::inherits(iid) {
                        return Some(unsafe { offset_of!(Self::Header, $index) });
                    }
                )*

                None
            }
        }

        impl<C, W $(, $interface)*> MakeHeader<C, W> for ($($interface,)*)
        where
            C: Class,
            W: Wrapper<C>,
            $($interface: Interface + Construct<C, W, { $index * std::mem::size_of::<*mut ()>() as isize }>,)*
        {
            fn header() -> Self::Header {
                $header($($interface::OBJ),*)
            }
        }
    }
}

interface_list!(Header1, I0 0);
interface_list!(Header2, I0 0, I1 1);
interface_list!(Header3, I0 0, I1 1, I2 2);
interface_list!(Header4, I0 0, I1 1, I2 2, I3 3);
interface_list!(Header5, I0 0, I1 1, I2 2, I3 3, I4 4);
interface_list!(Header6, I0 0, I1 1, I2 2, I3 3, I4 4, I5 5);
interface_list!(Header7, I0 0, I1 1, I2 2, I3 3, I4 4, I5 5, I6 6);
interface_list!(Header8, I0 0, I1 1, I2 2, I3 3, I4 4, I5 5, I6 6, I7 7);
interface_list!(Header9, I0 0, I1 1, I2 2, I3 3, I4 4, I5 5, I6 6, I7 7, I8 8);
interface_list!(Header10, I0 0, I1 1, I2 2, I3 3, I4 4, I5 5, I6 6, I7 7, I8 8, I9 9);
interface_list!(Header11, I0 0, I1 1, I2 2, I3 3, I4 4, I5 5, I6 6, I7 7, I8 8, I9 9, I10 10);
interface_list!(Header12, I0 0, I1 1, I2 2, I3 3, I4 4, I5 5, I6 6, I7 7, I8 8, I9 9, I10 10, I11 11);
interface_list!(Header13, I0 0, I1 1, I2 2, I3 3, I4 4, I5 5, I6 6, I7 7, I8 8, I9 9, I10 10, I11 11, I12 12);
interface_list!(Header14, I0 0, I1 1, I2 2, I3 3, I4 4, I5 5, I6 6, I7 7, I8 8, I9 9, I10 10, I11 11, I12 12, I13 13);
interface_list!(Header15, I0 0, I1 1, I2 2, I3 3, I4 4, I5 5, I6 6, I7 7, I8 8, I9 9, I10 10, I11 11, I12 12, I13 13, I14 14);

#[repr(C)]
struct ComWrapperInner<C: Class> {
    header: Header<C>,
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
    unsafe fn data_from_header(ptr: *mut Header<C>) -> *mut C {
        (ptr as *mut u8)
            .offset(-offset_of!(ComWrapperInner<C>, header))
            .offset(offset_of!(ComWrapperInner<C>, data)) as *mut C
    }

    #[inline]
    unsafe fn header_from_data(ptr: *mut C) -> *mut Header<C> {
        (ptr as *mut u8)
            .offset(-offset_of!(ComWrapperInner<C>, data))
            .offset(offset_of!(ComWrapperInner<C>, header)) as *mut Header<C>
    }

    #[inline]
    unsafe fn add_ref(ptr: *mut C) -> usize {
        let wrapper_ptr = (ptr as *mut u8).offset(-offset_of!(ComWrapperInner<C>, data))
            as *mut ComWrapperInner<C>;

        let arc = Arc::from_raw(wrapper_ptr);
        let result = Arc::strong_count(&arc) + 1;
        let _ = Arc::into_raw(arc);

        Arc::increment_strong_count(wrapper_ptr);

        result
    }

    #[inline]
    unsafe fn release(ptr: *mut C) -> usize {
        let wrapper_ptr = (ptr as *mut u8).offset(-offset_of!(ComWrapperInner<C>, data))
            as *mut ComWrapperInner<C>;

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
        C::Interfaces: MakeHeader<C, Self>,
    {
        ComWrapper {
            inner: Arc::new(ComWrapperInner {
                header: C::Interfaces::header(),
                data,
            }),
        }
    }

    #[inline]
    pub fn as_com_ref<'a, I: Interface>(&'a self) -> Option<ComRef<'a, I>> {
        if let Some(offset) = C::Interfaces::query(&I::IID) {
            unsafe {
                let wrapper_ptr = Arc::as_ptr(&self.inner) as *mut ComWrapperInner<C>;
                let interface_ptr = (wrapper_ptr as *mut u8)
                    .offset(offset_of!(ComWrapperInner<C>, header))
                    .offset(offset) as *mut I;
                Some(ComRef::from_raw_unchecked(interface_ptr))
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn to_com_ptr<I: Interface>(&self) -> Option<ComPtr<I>> {
        if let Some(offset) = C::Interfaces::query(&I::IID) {
            unsafe {
                let wrapper_ptr = Arc::into_raw(self.inner.clone()) as *mut ComWrapperInner<C>;
                let interface_ptr = (wrapper_ptr as *mut u8)
                    .offset(offset_of!(ComWrapperInner<C>, header))
                    .offset(offset) as *mut I;
                Some(ComPtr::from_raw_unchecked(interface_ptr))
            }
        } else {
            None
        }
    }
}
