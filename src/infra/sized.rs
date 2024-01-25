use std::mem::MaybeUninit;

pub trait SizedExt {
    unsafe fn uninit() -> Self;
}

impl<T> SizedExt for T {
    #[inline]
    unsafe fn uninit() -> Self {
        #[allow(clippy::uninit_assumed_init)]
        MaybeUninit::uninit().assume_init()
    }
}
