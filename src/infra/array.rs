use std::mem::MaybeUninit;

pub trait ArrayExt<T, const N: usize> {
    unsafe fn uninit() -> [T; N];
}

impl<T, const N: usize> ArrayExt<T, N> for [T; N] {
    unsafe fn uninit() -> [T; N] {
        #[allow(clippy::uninit_assumed_init)]
        MaybeUninit::uninit().assume_init()
    }
}
