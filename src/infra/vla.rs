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

use std::marker::PhantomData;
use std::slice;

#[repr(C)]
pub struct Vla<L, T> {
    len: L,
    pd: PhantomData<T>,
}

impl<L, T> Vla<L, T> {
    #[inline]
    pub const unsafe fn from_ptr<'t>(ptr: *const L) -> &'t Self {
        &*(ptr as *const Self)
    }
}

impl<T> Vla<u8, T> {
    #[inline]
    pub const fn len(&self) -> u8 {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u8;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u16, T> {
    #[inline]
    pub const fn len(&self) -> u16 {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u16;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u32, T> {
    #[inline]
    pub const fn len(&self) -> u32 {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u32;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u64, T> {
    #[inline]
    pub const fn len(&self) -> u64 {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u64;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<u128, T> {
    #[inline]
    pub const fn len(&self) -> u128 {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const u128;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

impl<T> Vla<usize, T> {
    #[inline]
    pub const fn len(&self) -> usize {
        self.len as _
    }

    pub const fn as_slice(&self) -> &[T] {
        let len_ptr = self as *const _ as *const usize;
        let head_ptr = unsafe { len_ptr.add(1) } as *const T;
        unsafe { slice::from_raw_parts(head_ptr, self.len as _) }
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::Vla;

    #[test]
    fn test_vla_u8_u8() {
        let buf = [2, 1, 2, 3, 4, 5_u8];
        let len_ptr = &buf[0] as *const u8;
        let vla: &Vla<u8, u8> = unsafe { &*Vla::from_ptr(len_ptr) };

        assert_eq!(vla.as_slice(), &buf[1..3])
    }

    #[test]
    fn test_vla_u32_u8() {
        #[repr(align(32))]
        struct Wrapper<const T: usize>([u8; T]);

        let buf = Wrapper([2, 0, 0, 0, 1, 2, 3, 4, 5_u8]);
        let len_ptr = &buf.0[0] as *const _ as *const u32;
        let vla: &Vla<u32, u8> = unsafe { &*Vla::from_ptr(len_ptr) };

        assert_eq!(vla.as_slice(), &buf.0[4..6])
    }
}
