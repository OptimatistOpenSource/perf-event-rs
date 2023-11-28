pub trait ConstPtrExt {
    unsafe fn align_as_ptr<T>(self) -> *const T;
}

impl<A> ConstPtrExt for *const A {
    unsafe fn align_as_ptr<T>(self) -> *const T {
        let offset = self.align_offset(std::mem::align_of::<T>());
        (unsafe { self.offset(offset as isize) }) as *const T
    }
}

pub trait MutPtrExt {
    unsafe fn align_as_ptr<T>(self) -> *mut T;
}

impl<A> MutPtrExt for *const A {
    unsafe fn align_as_ptr<T>(self) -> *mut T {
        let offset = self.align_offset(std::mem::align_of::<T>());
        (unsafe { self.offset(offset as isize) }) as *mut T
    }
}
