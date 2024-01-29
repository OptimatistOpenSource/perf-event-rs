use std::alloc::{alloc, Layout};
use std::ptr;

pub trait WrapBox<T> {
    #[inline]
    fn wrap_box(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<T> WrapBox<T> for T {}

pub trait BoxSliceExt {
    unsafe fn uninit(len: usize) -> Self;
}

impl<T> BoxSliceExt for Box<[T]> {
    unsafe fn uninit(len: usize) -> Self {
        let layout = Layout::array::<u8>(len).unwrap();
        let ptr = unsafe { alloc(layout) };
        let slice = ptr::slice_from_raw_parts(ptr, len);
        unsafe { Self::from_raw(std::mem::transmute(slice)) }
    }
}
