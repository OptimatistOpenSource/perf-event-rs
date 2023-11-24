use std::fmt::Debug;
use std::marker::PhantomData;
use std::slice;

#[derive(Debug)]
#[repr(C)]
pub struct Vla<L, T> {
    len: L,
    pd: PhantomData<T>,
}

impl<L, T> Vla<L, T> {
    pub unsafe fn from_ptr<X>(ptr: *const X) -> *const Vla<L, T> {
        let ptr = ptr as *const Vla<L, T>;
        &*ptr
    }
    pub unsafe fn from_brw<X>(brw: &X) -> &Vla<L, T> {
        let ptr = brw as *const _ as *const Vla<L, T>;
        &*ptr
    }
}

impl<T> Vla<u8, T> {
    pub fn len(&self) -> usize {
        self.len as _
    }

    pub fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u8;
        let head_ptr = unsafe { len_ptr.offset(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u16, T> {
    pub fn len(&self) -> usize {
        self.len as _
    }

    pub fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u16;
        let head_ptr = unsafe { len_ptr.offset(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u32, T> {
    pub fn len(&self) -> usize {
        self.len as _
    }

    pub fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u32;
        let head_ptr = unsafe { len_ptr.offset(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u64, T> {
    pub fn len(&self) -> usize {
        self.len as _
    }

    pub fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u64;
        let head_ptr = unsafe { len_ptr.offset(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u128, T> {
    pub fn len(&self) -> usize {
        self.len as _
    }

    pub fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u128;
        let head_ptr = unsafe { len_ptr.offset(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<usize, T> {
    pub fn len(&self) -> usize {
        self.len as _
    }

    pub fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const usize;
        let head_ptr = unsafe { len_ptr.offset(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}