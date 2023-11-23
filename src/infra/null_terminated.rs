use std::marker::PhantomData;
use std::slice;

#[derive(Debug)]
#[repr(C)]
pub struct NullTerminated<T> {
    pd: PhantomData<T>,
}

impl<T> NullTerminated<T> {
    pub unsafe fn from_ptr<P>(ptr: &P) -> &NullTerminated<T> {
        let ptr = ptr as *const _ as *const NullTerminated<T>;
        &*ptr
    }

    pub fn as_slice(&self) -> &[T] {
        let mut ptr = self as *const _ as *const u8;
        let mut bytes_len = 0_usize;
        while unsafe { *ptr } != 0 {
            bytes_len += 1;
            unsafe {
                ptr = ptr.offset(1);
            }
        }
        let len = bytes_len / std::mem::size_of::<T>();
        let ptr = self as *const _ as *const T;
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}
