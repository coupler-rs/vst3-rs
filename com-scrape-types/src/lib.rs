mod ptr;

#[cfg(test)]
mod tests;

pub use ptr::{ComPtr, ComRef, SmartPtr};

pub type Guid = [u8; 16];

pub unsafe trait Inherits<I> {}

pub unsafe trait Interface {
    const IID: Guid;

    unsafe fn query_interface<I: Interface>(this: *mut Self) -> Option<*mut I>;
    unsafe fn add_ref(this: *mut Self);
    unsafe fn release(this: *mut Self);
}
