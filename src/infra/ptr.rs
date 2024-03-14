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

pub trait ConstPtrExt {
    unsafe fn align_as_ptr<T>(self) -> *const T;
}

impl<A> ConstPtrExt for *const A {
    #[allow(clippy::ptr_offset_with_cast)]
    #[inline]
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
    #[inline]
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
