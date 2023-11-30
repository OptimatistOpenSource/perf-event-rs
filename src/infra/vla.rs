use std::marker::PhantomData;
use std::slice;

#[repr(C)]
pub struct Vla<L, T> {
    len: L,
    pd: PhantomData<T>,
}

impl<L, T> Vla<L, T> {
    pub unsafe fn from_ptr(ptr: *const L) -> *const Self {
        let ptr = ptr as *const Self;
        ptr.as_ref().unwrap()
    }
    pub unsafe fn from_ref<X>(r: &X) -> &Self {
        let ptr = r as *const _ as *const Self;
        ptr.as_ref().unwrap()
    }
}

impl<T> Vla<u8, T> {
    pub const fn len(&self) -> usize {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u8;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u16, T> {
    pub const fn len(&self) -> usize {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u16;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u32, T> {
    pub const fn len(&self) -> usize {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u32;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u64, T> {
    pub const fn len(&self) -> usize {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u64;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u128, T> {
    pub const fn len(&self) -> usize {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u128;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<usize, T> {
    pub const fn len(&self) -> usize {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const usize;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}
