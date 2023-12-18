pub trait SliceExt<T> {
    unsafe fn follow_mem_ptr(&self) -> *const T;
}

impl<T> SliceExt<T> for &[T] {
    #[inline]
    unsafe fn follow_mem_ptr(&self) -> *const T {
        self.as_ptr().add(self.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::SliceExt;

    #[test]
    fn test_follow_mem_ptr_1() {
        let buf = [0, 1, 2, 3_u8];
        let slice = &buf[0..buf.len() - 1];
        let ptr = unsafe { slice.follow_mem_ptr() };
        assert_eq!(ptr, &buf[buf.len() - 1] as *const u8);
    }

    #[test]
    fn test_follow_mem_ptr_2() {
        let buf = [0, 1, 2, 3_u8];
        let slice = &buf[0..0];
        let ptr = unsafe { slice.follow_mem_ptr() };
        assert_eq!(ptr, &buf[0] as *const u8);
    }
}
