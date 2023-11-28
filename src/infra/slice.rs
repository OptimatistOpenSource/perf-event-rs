pub trait SliceExt<T> {
    unsafe fn follow_mem_ptr(&self) -> *const T;
}

impl<T> SliceExt<T> for &[T] {
    unsafe fn follow_mem_ptr(&self) -> *const T {
        let offset = self.len() as isize;
        unsafe { self.as_ptr().offset(offset) }
    }
}
