use std::mem::MaybeUninit;

pub trait SizedExt {
    fn size() -> usize;

    unsafe fn uninit() -> Self;
}

impl<T> SizedExt for T {
    #[inline]
    fn size() -> usize {
        std::mem::size_of::<T>()
    }

    unsafe fn uninit() -> Self {
        #[allow(clippy::uninit_assumed_init)]
        MaybeUninit::uninit().assume_init()
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
