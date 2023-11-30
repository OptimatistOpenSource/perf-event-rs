use std::marker::PhantomData;
use std::ops::Not;
use std::slice;

#[derive(Debug)]
#[repr(C)]
pub struct ZeroTerminated<T> {
    pd: PhantomData<T>,
}

impl<T> ZeroTerminated<T> {
    pub unsafe fn from_ref(r: &T) -> &Self {
        let ptr = r as *const _ as *const Self;
        ptr.as_ref().unwrap()
    }

    pub fn as_slice(&self) -> &[T] {
        let mut ptr = self as *const _ as *const T;
        let mut len = 0_usize;

        fn is_all_zero<T>(ptr: *const T) -> bool {
            let ptr = ptr as *const u8;
            for i in 0..std::mem::size_of::<T>() {
                if unsafe { *ptr.add(i) } != 0 {
                    return false;
                }
            }
            true
        }

        while is_all_zero(ptr).not() {
            len += 1;
            unsafe {
                ptr = ptr.add(1);
            }
        }
        let ptr = self as *const _ as *const T;
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::ZeroTerminated;

    #[test]
    fn test_u8_1() {
        let buf = [1, 2, 3, 4, 5, 6, 0_u8];
        let zt = unsafe { ZeroTerminated::from_ref(&buf[0]) };
        let slice = zt.as_slice();
        assert_eq!(slice.len(), 6);
        assert_eq!(slice, &buf[0..6]);
    }

    #[test]
    fn test_u8_2() {
        let buf = [1, 2, 3, 0, 5, 6, 0_u8];
        let zt = unsafe { ZeroTerminated::from_ref(&buf[0]) };
        let slice = zt.as_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice, &buf[0..3]);
    }

    #[test]
    fn test_u64_1() {
        let buf = [1, 2, 3, 4, 5, 6, 0_u64];
        let zt = unsafe { ZeroTerminated::from_ref(&buf[0]) };
        let slice = zt.as_slice();
        assert_eq!(slice.len(), 6);
        assert_eq!(slice, &buf[0..6]);
    }

    #[test]
    fn test_u64_2() {
        let buf = [1, 2, 3, 0, 5, 6, 0_u64];
        let zt = unsafe { ZeroTerminated::from_ref(&buf[0]) };
        let slice = zt.as_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice, &buf[0..3]);
    }
}
