// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of perf-event-rs.
//
// Perf-event-rs is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License 
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// 
// Perf-event-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not, 
// see <https://www.gnu.org/licenses/>.

use crate::infra::SizedExt;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Not;
use std::slice;

#[repr(C)]
pub struct ZeroTerminated<T> {
    pd: PhantomData<T>,
}

impl<T> ZeroTerminated<T> {
    #[inline]
    pub const unsafe fn from_ptr<'t>(ptr: *const T) -> &'t Self {
        &*(ptr as *const Self)
    }

    pub fn as_slice(&self) -> &[T] {
        let mut ptr = self as *const _ as *const T;
        let mut len = 0_usize;

        fn is_all_zero<T>(ptr: *const T) -> bool {
            let ptr = ptr as *const u8;
            for i in 0..size_of::<T>() {
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
        let zt = unsafe { ZeroTerminated::from_ptr(&buf[0]) };
        let slice = zt.as_slice();
        assert_eq!(slice.len(), 6);
        assert_eq!(slice, &buf[0..6]);
    }

    #[test]
    fn test_u8_2() {
        let buf = [1, 2, 3, 0, 5, 6, 0_u8];
        let zt = unsafe { ZeroTerminated::from_ptr(&buf[0]) };
        let slice = zt.as_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice, &buf[0..3]);
    }

    #[test]
    fn test_u64_1() {
        let buf = [1, 2, 3, 4, 5, 6, 0_u64];
        let zt = unsafe { ZeroTerminated::from_ptr(&buf[0]) };
        let slice = zt.as_slice();
        assert_eq!(slice.len(), 6);
        assert_eq!(slice, &buf[0..6]);
    }

    #[test]
    fn test_u64_2() {
        let buf = [1, 2, 3, 0, 5, 6, 0_u64];
        let zt = unsafe { ZeroTerminated::from_ptr(&buf[0]) };
        let slice = zt.as_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice, &buf[0..3]);
    }
}
