pub trait ConstPtrExt {
    unsafe fn align_as_ptr<T>(self) -> *const T;
}

impl<A> ConstPtrExt for *const A {
    #[allow(clippy::ptr_offset_with_cast)]
    unsafe fn align_as_ptr<T>(self) -> *const T {
        let u8_ptr = self as *const u8;
        let offset = u8_ptr.align_offset(std::mem::align_of::<T>());
        (unsafe { u8_ptr.add(offset) }) as *const T
    }
}

pub trait MutPtrExt {
    unsafe fn align_as_ptr<T>(self) -> *mut T;
}

impl<A> MutPtrExt for *mut A {
    #[allow(clippy::ptr_offset_with_cast)]
    unsafe fn align_as_ptr<T>(self) -> *mut T {
        (self as *const A).align_as_ptr::<T>() as *mut T
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::ConstPtrExt;

    #[test]
    fn test_align_as_ptr_const_1() {
        let buf = [1, 2, 3, 4, 5, 6, 7, 8_u8];
        let ptr_0 = &buf[0] as *const u8;

        let ptr = unsafe { ptr_0.align_as_ptr::<u32>() } as *const u8;
        assert_eq!(ptr, ptr_0);
    }

    #[test]
    fn test_align_as_ptr_const_2() {
        let buf = [1, 2, 3, 4, 5, 6, 7, 8_u8];
        let ptr_1 = &buf[1] as *const u8;
        let ptr_4 = &buf[4] as *const u8;

        let ptr = unsafe { ptr_1.align_as_ptr::<u32>() } as *const u8;
        assert_eq!(ptr, ptr_4);
    }

    #[test]
    fn test_align_as_ptr_const_3() {
        let ptr_1 = 0x00007fa1a4000f6c as *const u32;
        let ptr_2 = 0x00007fa1a4000f70 as *const u32;

        let ptr = unsafe { ptr_1.align_as_ptr::<u64>() } as *const u32;
        assert_eq!(ptr, ptr_2);
    }
}
