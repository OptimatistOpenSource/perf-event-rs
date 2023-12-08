pub trait SizedExt {
    fn size() -> usize;
}

impl<T> SizedExt for T {
    #[inline]
    fn size() -> usize {
        std::mem::size_of::<T>()
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::SizedExt;

    #[test]
    fn test_u8() {
        assert_eq!(u8::size(), 1);
    }

    #[test]
    fn test_u32() {
        assert_eq!(u32::size(), 4);
    }

    #[test]
    fn test_u64() {
        assert_eq!(u64::size(), 8);
    }
}
