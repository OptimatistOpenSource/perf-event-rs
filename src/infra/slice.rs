pub trait SliceExt<T> {
    fn follow_mem_ptr(&self) -> *const T;
}

impl<T> SliceExt<T> for &[T] {
    fn follow_mem_ptr(&self) -> *const T {
        let offset = self.len() as isize;
        unsafe { self.as_ptr().offset(offset) }
    }
}
