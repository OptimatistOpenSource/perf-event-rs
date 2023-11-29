pub trait SliceExt<T> {
    unsafe fn follow_mem_ptr(&self) -> *const T;
}

impl<T> SliceExt<T> for &[T] {
    unsafe fn follow_mem_ptr(&self) -> *const T {
        unsafe { self.as_ptr().add(self.len()) }
    }
}
