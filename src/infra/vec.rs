pub trait VecExt<T> {
    unsafe fn with_len_uninit(len: usize) -> Vec<T>;
}

impl<T> VecExt<T> for Vec<T> {
    #[inline]
    unsafe fn with_len_uninit(len: usize) -> Self {
        let mut vec = Self::with_capacity(len);
        #[allow(clippy::uninit_vec)]
        vec.set_len(len);
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::VecExt;

    #[test]
    fn test_with_len_uninit() {
        let vec = unsafe { Vec::<u8>::with_len_uninit(114514) };
        assert_eq!(vec.len(), 114514);
    }
}
