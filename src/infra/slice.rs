pub trait SliceExt {
    fn follow_mem_ptr<P>(&self) -> *const P;
}

impl<T> SliceExt for &[T] {
    fn follow_mem_ptr<P>(&self) -> *const P {
        let offset = self.len() as isize;
        let ptr = unsafe { self.as_ptr().offset(offset) };
        ptr as *const P
    }
}
