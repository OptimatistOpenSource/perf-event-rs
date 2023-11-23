use std::fmt::Debug;
use std::marker::PhantomData;
use std::slice;

#[derive(Debug)]
#[repr(C)]
pub struct Vla<T> {
    len: u64,
    pd: PhantomData<T>,
}

impl<T> Vla<T> {
    pub unsafe fn from_ptr<P>(ptr: &P) -> &Vla<T> {
        let ptr = ptr as *const _ as *const Vla<T>;
        &*ptr
    }

    pub fn len(&self) -> usize {
        self.len as _
    }

    pub fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const usize;
        let head_ptr = unsafe { len_ptr.offset(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}
